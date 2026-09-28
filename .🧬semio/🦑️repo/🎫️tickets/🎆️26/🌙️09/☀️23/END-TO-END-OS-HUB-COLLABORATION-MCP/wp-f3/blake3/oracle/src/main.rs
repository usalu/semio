//! 🧪️ F3 — BLAKE3 test vectors from the third-party `blake3` crate (the BLAKE3 team's implementation): the official
//! input construction (byte i = i % 251) at the chunk/tree edge lengths, 32-byte hash and 131-byte XOF output, as JSON.
fn main() {
    let lengths: [usize; 26] = [0, 1, 63, 64, 65, 1023, 1024, 1025, 2048, 2049, 3072, 3073, 4096, 4097, 5120, 5121, 6144, 6145, 7168, 7169, 8192, 8193, 16384, 31744, 102400, 1048577];
    let rows: Vec<String> = lengths
        .iter()
        .map(|&length| {
            let input: Vec<u8> = (0..length).map(|index| (index % 251) as u8).collect();
            let mut xof = [0u8; 131];
            blake3::Hasher::new().update(&input).finalize_xof().fill(&mut xof);
            let hex: String = xof.iter().map(|byte| format!("{byte:02x}")).collect();
            format!("  {{\"length\": {length}, \"hash\": \"{}\", \"xof131\": \"{hex}\"}}", blake3::hash(&input).to_hex())
        })
        .collect();
    println!("{{\n \"schema\": \"semio.hash.blake3-vectors/v1\",\n \"source\": \"blake3 crate 1.8.7 (BLAKE3-team/BLAKE3), input byte i = i % 251\",\n \"vectors\": [\n{}\n ]\n}}", rows.join(",\n"));
}
