use std::{fs, io::Cursor, num::NonZeroU64};

fn find(haystack: &[u8], needle: &[u8]) -> usize {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
        .expect("fixture stream marker")
}

fn compare(name: &str, expected: &[u8], actual: &[u8]) {
    let first_diff = expected
        .iter()
        .zip(actual)
        .position(|(left, right)| left != right)
        .unwrap_or(expected.len().min(actual.len()));
    println!(
        "{name}\tlength={}\theader={:02x?}\tfirst_diff={}\texpected={:02x?}\tactual={:02x?}",
        actual.len(),
        &actual[..actual.len().min(8)],
        first_diff,
        &expected[first_diff.saturating_sub(4)..(first_diff + 5).min(expected.len())],
        &actual[first_diff.saturating_sub(4)..(first_diff + 5).min(actual.len())],
    );
}

fn common_prefix(expected: &[u8], actual: &[u8]) -> usize {
    expected
        .iter()
        .zip(actual)
        .take_while(|(left, right)| left == right)
        .count()
}

fn main() {
    let fixture = fs::read("temp/📄️bachelor-thesis.pdf").expect("fixture");
    let marker = b"/Length 3362\n/Filter /FlateDecode\n>>\nstream\n";
    let start = find(&fixture, marker) + marker.len();
    let expected = &fixture[start..start + 3362];
    let decoded = miniz_oxide::inflate::decompress_to_vec_zlib(expected).expect("inflate");
    println!(
        "fixture\tcompressed={}\tdecoded={}\theader={:02x?}\ttail={:02x?}",
        expected.len(),
        decoded.len(),
        &expected[..8],
        &expected[expected.len() - 8..],
    );
    for level in 0..=10 {
        let actual = miniz_oxide::deflate::compress_to_vec_zlib(&decoded, level);
        compare(&format!("miniz-{level}"), expected, &actual);
    }
    let mut zlib_rs_best = (0, String::new(), Vec::new());
    let strategies = [
        ("default", zlib_rs::Strategy::Default),
        ("filtered", zlib_rs::Strategy::Filtered),
        ("huffman", zlib_rs::Strategy::HuffmanOnly),
        ("rle", zlib_rs::Strategy::Rle),
        ("fixed", zlib_rs::Strategy::Fixed),
    ];
    for window_bits in 9..=15 {
        for level in 0..=9 {
            for mem_level in 1..=9 {
                for (strategy_name, strategy) in strategies {
                    let config = zlib_rs::DeflateConfig {
                        level,
                        method: zlib_rs::Method::Deflated,
                        window_bits,
                        mem_level,
                        strategy,
                    };
                    let mut buffer = vec![0; zlib_rs::compress_bound(decoded.len())];
                    let (output, status) = zlib_rs::compress_slice(&mut buffer, &decoded, config);
                    assert_eq!(status, zlib_rs::ReturnCode::Ok);
                    let prefix = common_prefix(expected, output);
                    if prefix > zlib_rs_best.0 {
                        zlib_rs_best = (
                            prefix,
                            format!(
                                "zlib-rs-w{window_bits}-l{level}-m{mem_level}-{strategy_name}"
                            ),
                            output.to_vec(),
                        );
                    }
                }
            }
        }
    }
    compare(&zlib_rs_best.1, expected, &zlib_rs_best.2);
    for iterations in [1_u64, 5, 15] {
        let options = zopfli::Options {
            iteration_count: NonZeroU64::new(iterations).unwrap(),
            iterations_without_improvement: NonZeroU64::new(u64::MAX).unwrap(),
            maximum_block_splits: 15,
        };
        let mut actual = Vec::new();
        zopfli::compress(
            options,
            zopfli::Format::Zlib,
            Cursor::new(&decoded),
            &mut actual,
        )
        .expect("zopfli");
        compare(&format!("zopfli-{iterations}"), expected, &actual);
    }
}
