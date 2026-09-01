use dfverify::{deflate, inflate};

#[test]
fn truncated_input_still_errors() {
    let sample = "the quick brown fox jumps over the lazy dog. ".repeat(500).into_bytes();
    let compressed = deflate(&sample);
    assert!(compressed.len() > 10);
    for cut in [1usize, compressed.len() / 2, compressed.len() - 1] {
        let truncated = &compressed[..cut];
        let result = inflate(truncated, sample.len());
        assert!(result.is_err(), "truncated input at cut={cut} must not succeed, got {result:?}");
    }
    println!("truncated inputs correctly rejected");
}
