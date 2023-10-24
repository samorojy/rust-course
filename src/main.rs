use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("No Path Specified!");
        return;
    }

    let files = match fs::read_dir(&args[1]) {
        Ok(files) => files,
        Err(err) => {
            eprintln!("Error Reading Directory: {}", err);
            return;
        }
    };

    for entry in files {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                println!("{}", file_name);
            }
        }
    }
}
