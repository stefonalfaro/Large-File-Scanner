use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Count total lines
    let total_lines = reader.lines().count();
    println!("Total lines in the file: {}", total_lines);

    // Reopen the file to reset the reader
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    loop {
        println!("Enter a command:");
        println!("  'l <number>' to view lines around a specific line number");
        println!("  's <keyword>' to search for a keyword");
        println!("  'q' to quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "q" {
            break;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() != 2 {
            println!("Invalid input. Please use 'l <number>', 's <keyword>', or 'q'.");
            continue;
        }

        match parts[0] {
            "l" => view_lines(&mut reader, parts[1], total_lines)?,
            "s" => search_keyword(&mut reader, parts[1])?,
            _ => println!("Invalid command. Use 'l', 's', or 'q'."),
        }
    }

    Ok(())
}

fn view_lines(reader: &mut BufReader<File>, line_number_str: &str, total_lines: usize) -> io::Result<()> {
    if let Ok(line_number) = line_number_str.parse::<usize>() {
        if line_number > total_lines {
            println!("Line number exceeds total lines in the file.");
            return Ok(());
        }

        let start_line = line_number.saturating_sub(5);
        let end_line = (line_number + 5).min(total_lines);

        let approx_position = (start_line as u64).saturating_sub(1) * 100;
        reader.seek(SeekFrom::Start(approx_position))?;

        let mut current_line = 0;
        for line in reader.lines() {
            current_line += 1;
            if current_line >= start_line {
                if current_line > end_line {
                    break;
                }
                println!("{}: {}", current_line, line?);
            }
        }
    } else {
        println!("Invalid line number.");
    }
    Ok(())
}

fn search_keyword(reader: &mut BufReader<File>, keyword: &str) -> io::Result<()> {
    reader.seek(SeekFrom::Start(0))?;
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(keyword) {
            println!("{}: {}", line_number + 1, line);
        }
    }
    Ok(())
}