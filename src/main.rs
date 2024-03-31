use clap::Parser;
use regex::Regex;
use reqwest;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
struct CliOpts {
    filename: PathBuf,
}

#[derive(Default)]
struct EmojiData {
    pub url: String,
    pub name: String,
}

impl EmojiData {
    fn download_path(&self) -> PathBuf {
        // get the file extension from the url
        let ext = self
            .url
            .split('.')
            .last()
            .expect("Failed to get file extension");

        PathBuf::from_str(&format!("downloads/{}.{}", self.name, ext))
            .expect("Failed to create path")
    }
}

fn parse_line(input: &str) -> Option<EmojiData> {
    // example ![foobird](https://emoji.slack-edge.com/ASDFG134/bird_run/88fb09386799c639.gif)

    let parser =
        Regex::new(r#"\!\[(?<name>[^\]]+)\]\((?<url>https:\/\/emoji\.slack-edge\.com[^\)]+)"#)
            .expect("Failed to create regex");

    let caps = parser.captures(input)?;

    Some(EmojiData {
        url: caps
            .name("url")
            .map(|m| m.as_str().to_string())
            .expect("Didn't get url"),
        name: caps
            .name("name")
            .map(|m| m.as_str().to_string())
            .expect("Didn't get name"),
    })
}

fn main() {
    let opts = CliOpts::parse();

    if !opts.filename.exists() {
        eprintln!("File not found: {:?}", opts.filename);
        std::process::exit(1);
    }

    for line in std::fs::read_to_string(&opts.filename)
        .expect("Failed to read file")
        .lines()
    {
        if let Some(emoji) = parse_line(line) {
            let download_path = emoji.download_path();
            if download_path.exists() {
                // println!("Already downloaded: {}", emoji.name);
                continue;
            }
            let mut file = std::fs::File::create(&download_path).expect("Failed to create file");

            let mut response = reqwest::blocking::get(emoji.url).expect("Failed to download emoji");

            response
                .copy_to(&mut file)
                .expect("Failed to write to file");

            println!("Downloaded: {} to {}", emoji.name, download_path.display());
        }
    }
}
