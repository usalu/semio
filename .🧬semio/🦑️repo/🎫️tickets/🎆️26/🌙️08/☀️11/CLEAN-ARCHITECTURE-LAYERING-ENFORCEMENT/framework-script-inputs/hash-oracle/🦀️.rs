/// 🧮️ Emits independent dev-only BLAKE3 reference vectors for the owned hash contract.
fn main() {
    use std::fmt::Write;
    let lengths: [usize; 21] = [0, 1, 2, 3, 63, 64, 65, 1023, 1024, 1025, 2048, 2049, 3072, 3073, 4096, 4097, 8192, 8193, 16384, 102400, 1048577];
    print!("{{\"schema\":\"semio.hash.blake3-vectors/v1\",\"source\":\"blake3 crate dev oracle; input byte i = i % 251\",\"vectors\":[");
    for (index, length) in lengths.into_iter().enumerate() {
        let input: Vec<u8> = (0..length).map(|offset| (offset % 251) as u8).collect();
        let hash = blake3::hash(&input).to_hex();
        let mut hasher = blake3::Hasher::new();
        hasher.update(&input);
        let mut extended = [0u8; 131];
        hasher.finalize_xof().fill(&mut extended);
        let mut xof = String::with_capacity(262);
        for byte in extended {
            write!(&mut xof, "{byte:02x}").unwrap();
        }
        if index > 0 { print!(","); }
        print!("{{\"length\":{length},\"hash\":\"{hash}\",\"xof131\":\"{xof}\"}}");
        eprintln!("[DEBUG] blake3-vector length={length}");
    }
    println!("]}}");
}
