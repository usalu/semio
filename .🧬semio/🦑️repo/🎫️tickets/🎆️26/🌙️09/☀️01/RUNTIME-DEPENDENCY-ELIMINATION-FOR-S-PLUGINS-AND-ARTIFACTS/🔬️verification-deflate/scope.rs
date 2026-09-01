use dfverify::{deflate, inflate};
use miniz_oxide::inflate::decompress_to_vec;

#[test]
fn characterise_inflate_failures() {
    let mut fail_self = Vec::new();
    let mut fail_lens = Vec::new();
    let mut ok = 0;
    for len in 0usize..=512 {
        let raw: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
        let enc = deflate(&raw);
        let miniz_ok = decompress_to_vec(&enc).map(|v| v == raw).unwrap_or(false);
        match inflate(&enc, raw.len()) {
            Ok(v) if v == raw => ok += 1,
            Ok(_) => { fail_self.push((len, "wrong bytes".to_string())); fail_lens.push(len) }
            Err(e) => { fail_self.push((len, format!("{e:?}"))); fail_lens.push(len) }
        }
        if !miniz_ok { println!("  NOTE: miniz also failed at len {len}"); }
    }
    println!("lens 0..=512: our inflate OK on {ok}, FAILED on {}", fail_lens.len());
    if !fail_lens.is_empty() {
        println!("first 20 failing lengths: {:?}", &fail_lens[..fail_lens.len().min(20)]);
        println!("sample errors: {:?}", &fail_self[..fail_self.len().min(5)]);
        let contiguous = fail_lens.windows(2).all(|w| w[1] == w[0] + 1);
        println!("failing set contiguous from {}? {}", fail_lens[0], contiguous);
    }
}
