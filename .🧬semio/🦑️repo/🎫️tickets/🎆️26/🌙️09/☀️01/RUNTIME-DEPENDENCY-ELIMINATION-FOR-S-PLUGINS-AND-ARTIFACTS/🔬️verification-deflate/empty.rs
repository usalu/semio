use dfverify::{deflate, inflate};
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

#[test]
fn empty_input_diagnosis() {
    let raw: Vec<u8> = vec![];
    let ours = deflate(&raw);
    println!("our deflate(empty) = {ours:02x?} ({} bytes)", ours.len());
    println!("miniz inflate of ours: {:?}", decompress_to_vec(&ours));
    for lvl in [1u8,6,9] {
        let theirs = compress_to_vec(&raw, lvl);
        println!("miniz deflate(empty, lvl {lvl}) = {theirs:02x?}");
        println!("   our inflate(theirs, 0) = {:?}", inflate(&theirs, 0));
    }
    println!("our inflate(ours, 0) = {:?}", inflate(&ours, 0));
    println!("our inflate(ours, 1) = {:?}", inflate(&ours, 1));
}

#[test]
fn nonempty_only_parity_still_holds() {
    let mut s: u64 = 0x243F6A8885A308D3;
    let mut n = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s };
    let mut cases: Vec<Vec<u8>> = vec![vec![0x42], vec![0u8;10_000], vec![0xFFu8;4096],
        (0..65536).map(|i| (i % 256) as u8).collect(),
        "the quick brown fox. ".repeat(500).into_bytes()];
    for len in [1usize,2,3,15,16,17,255,256,257,1023,1024,1025,5000,33000] {
        cases.push((0..len).map(|_| (n() >> 24) as u8).collect());
        cases.push((0..len).map(|_| ((n() >> 24) as u8) & 0x03).collect());
    }
    let mut ok = 0;
    for raw in &cases {
        assert_eq!(&inflate(&deflate(raw), raw.len()).expect("self"), raw);
        for lvl in [1u8,6,9] {
            assert_eq!(&inflate(&compress_to_vec(raw, lvl), raw.len()).expect("miniz->ours"), raw);
        }
        assert_eq!(&decompress_to_vec(&deflate(raw)).expect("ours->miniz"), raw);
        ok += 1;
    }
    println!("non-empty parity OK across {ok} cases (self, miniz->ours x3 levels, ours->miniz)");
}
