use dfverify::{deflate, inflate};
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

struct Lcg(u64);
impl Lcg {
    fn n(&mut self) -> u64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0 }
}

fn corpus() -> Vec<(String, Vec<u8>)> {
    let mut r = Lcg(0x243F6A8885A308D3);
    let mut v: Vec<(String, Vec<u8>)> = vec![
        ("empty".into(), vec![]),
        ("one byte".into(), vec![0x42]),
        ("all zeros 10k".into(), vec![0u8; 10_000]),
        ("all 0xFF 4k".into(), vec![0xFFu8; 4096]),
        ("ascending 64k".into(), (0..65536).map(|i| (i % 256) as u8).collect()),
        ("text repeated".into(), "the quick brown fox jumps over the lazy dog. ".repeat(500).into_bytes()),
    ];
    for len in [1usize, 2, 3, 15, 16, 17, 255, 256, 257, 1023, 1024, 1025, 5000, 33000] {
        v.push((format!("incompressible {len}"), (0..len).map(|_| (r.n() >> 24) as u8).collect()));
        v.push((format!("low entropy {len}"), (0..len).map(|_| ((r.n() >> 24) as u8) & 0x03).collect()));
    }
    v
}

#[test]
fn our_deflate_roundtrips_through_miniz_oxide() {
    for (name, raw) in corpus() {
        let ours = deflate(&raw);
        let back = decompress_to_vec(&ours).unwrap_or_else(|e| panic!("miniz could not inflate OUR output for {name}: {e:?}"));
        assert_eq!(back, raw, "roundtrip mismatch (ours->miniz) for {name}");
    }
    println!("our deflate -> miniz inflate: OK across {} cases", corpus().len());
}

#[test]
fn our_inflate_reads_miniz_oxide_output() {
    for (name, raw) in corpus() {
        for level in [1u8, 6, 9] {
            let theirs = compress_to_vec(&raw, level);
            let back = inflate(&theirs, raw.len()).unwrap_or_else(|e| panic!("OUR inflate failed on miniz level-{level} output for {name}: {e:?}"));
            assert_eq!(back, raw, "roundtrip mismatch (miniz->ours) for {name} at level {level}");
        }
    }
    println!("miniz deflate -> our inflate: OK across {} cases x 3 levels", corpus().len());
}

#[test]
fn our_own_roundtrip_is_exact() {
    for (name, raw) in corpus() {
        let back = inflate(&deflate(&raw), raw.len()).unwrap_or_else(|e| panic!("self-roundtrip failed for {name}: {e:?}"));
        assert_eq!(back, raw, "self-roundtrip mismatch for {name}");
    }
    println!("our deflate -> our inflate: OK across {} cases", corpus().len());
}
