use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let f = File::open("/usr/share/dict/words")?;
    let mut buf_reader = BufReader::new(f);
    let mut words = String::new();
    buf_reader.read_to_string(&mut words)?;

    'lines: for line in words.lines() {
        for byte in line.bytes() {
            match byte {
                b'A' | b'B' | b'C' | b'D' | b'E' | b'F' |
                b'a' | b'b' | b'c' | b'd' | b'e' | b'f' => continue,
                _ => continue 'lines,
            }
        }

        if line.len() > 2 {
            println!("{}", line);
        }
    };

    Ok(())
}
