use b3verify::{hash, Hasher};

fn pattern(len: usize) -> Vec<u8> { (0..len).map(|i| (i % 251) as u8).collect() }

#[test]
fn official_vector_lengths_match_reference() {
    let lens = [0usize,1,2,3,4,5,6,7,8,63,64,65,127,128,129,1023,1024,1025,2048,2049,3072,3073,4096,4097,5120,10240,102400,1000000];
    for &n in &lens {
        let input = pattern(n);
        let ours = hash(&input);
        let theirs = blake3::hash(&input);
        assert_eq!(ours.as_bytes(), theirs.as_bytes(), "MISMATCH at len {n}");
    }
    println!("one-shot parity OK across {} official vector lengths (max 1000000)", lens.len());
}

#[test]
fn incremental_update_matches_reference() {
    let mut state: u64 = 0x2545F4914F6CDD1D;
    let mut next = || { state ^= state << 13; state ^= state >> 7; state ^= state << 17; state };
    for case in 0..300u32 {
        let total = (next() % 9000) as usize + 1;
        let input = pattern(total);
        let mut h = Hasher::new();
        let mut off = 0;
        while off < total {
            let step = ((next() % 700) as usize + 1).min(total - off);
            h.update(&input[off..off + step]);
            off += step;
        }
        assert_eq!(h.finalize().as_bytes(), blake3::hash(&input).as_bytes(), "MISMATCH case {case} len {total}");
    }
    println!("incremental parity OK across 300 randomly-chunked inputs");
}
