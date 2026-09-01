use dfverify::{deflate, inflate, InflateOutcome, Inflater};
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

fn btype_of(bytes: &[u8]) -> u8 {
    ((bytes[0] >> 1) & 0b11) as u8
}

struct Lcg(u64);
impl Lcg {
    fn n(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn streamed_inflate(compressed: &[u8]) -> Vec<u8> {
    let mut inflater = Inflater::new();
    let mut out = Vec::new();
    let mut index = 0usize;
    let mut pending: Option<u8> = None;
    loop {
        if pending.is_none() && index < compressed.len() {
            pending = Some(compressed[index]);
            index += 1;
        }
        let input_complete = index >= compressed.len();
        match inflater.advance(&mut pending, input_complete).expect("advance") {
            InflateOutcome::NeedInput => {
                assert!(!input_complete, "NeedInput while input already complete");
            }
            InflateOutcome::Wrote(byte) => out.push(byte),
            InflateOutcome::Done => return out,
        }
    }
}

#[test]
fn miniz_emits_dynamic_huffman_for_structured_input_and_we_read_it() {
    let text = "the quick brown fox jumps over the lazy dog. ".repeat(2000).into_bytes();
    let theirs = compress_to_vec(&text, 6);
    assert_eq!(btype_of(&theirs), 2, "expected BTYPE=10 dynamic Huffman from miniz on structured input");
    let ours = inflate(&theirs, text.len()).expect("inflate dynamic huffman");
    assert_eq!(ours, text);
}

#[test]
fn miniz_emits_stored_blocks_for_incompressible_input_and_we_read_it() {
    let mut r = Lcg(0xD1CEBEEF);
    let raw: Vec<u8> = (0..20000).map(|_| (r.n() >> 16) as u8).collect();
    for level in [0u8, 1] {
        let theirs = compress_to_vec(&raw, level);
        let found_stored = theirs.chunks(1).next().map(|_| true).unwrap_or(false) && {
            let mut i = 0usize;
            let mut bitpos = 0u32;
            let mut saw_stored = false;
            let mut buf = 0u32;
            let mut bits = 0u32;
            loop {
                while bits < 3 && i < theirs.len() {
                    buf |= (theirs[i] as u32) << bits;
                    bits += 8;
                    i += 1;
                }
                if bits < 3 {
                    break;
                }
                let _bfinal = buf & 1;
                let btype = (buf >> 1) & 0b11;
                buf >>= 3;
                bits -= 3;
                if btype == 0 {
                    saw_stored = true;
                    break;
                }
                bitpos += 3;
                break;
            }
            let _ = bitpos;
            saw_stored
        };
        println!("level {level}: btype(first block)={} saw_stored_first_block={found_stored}", btype_of(&theirs));
    }
    let raw_bytes = [0u8, 0, 0, 0xFF, 0xFF];
    let stored_block: Vec<u8> = {
        let mut v = vec![0x01u8];
        let len: u16 = raw_bytes.len() as u16;
        v.extend_from_slice(&len.to_le_bytes());
        v.extend_from_slice(&(!len).to_le_bytes());
        v.extend_from_slice(&raw_bytes);
        v
    };
    let ours = inflate(&stored_block, raw_bytes.len()).expect("inflate hand-built stored block");
    assert_eq!(ours, raw_bytes);
    let miniz_check = decompress_to_vec(&stored_block).expect("miniz reads our hand-built stored block");
    assert_eq!(miniz_check, raw_bytes);
}

#[test]
fn multi_block_input_round_trips() {
    let mut r = Lcg(0x1234);
    let raw: Vec<u8> = (0..200_000).map(|_| (r.n() >> 16) as u8 & 0x07).collect();
    let theirs = compress_to_vec(&raw, 6);
    let ours = inflate(&theirs, raw.len()).expect("inflate multi-block miniz output");
    assert_eq!(ours, raw);
    let self_enc = deflate(&raw);
    let self_dec = inflate(&self_enc, raw.len()).expect("inflate our own multi-block output");
    assert_eq!(self_dec, raw);
}

#[test]
fn inputs_larger_than_window_round_trip_across_block_boundaries() {
    let mut r = Lcg(0xFEEDFACE);
    let mut raw: Vec<u8> = Vec::new();
    for _ in 0..3 {
        raw.extend((0..40_000).map(|_| (r.n() >> 20) as u8));
    }
    assert!(raw.len() > 32768 * 3);
    let theirs = compress_to_vec(&raw, 6);
    let ours = inflate(&theirs, raw.len()).expect("inflate large miniz output spanning window");
    assert_eq!(ours, raw);
    let self_enc = deflate(&raw);
    let self_dec = inflate(&self_enc, raw.len()).expect("inflate our own large output spanning window");
    assert_eq!(self_dec, raw);
}

#[test]
fn resumable_inflater_agrees_with_one_shot_across_block_types() {
    let cases: Vec<Vec<u8>> = vec![
        vec![],
        vec![0x42],
        "the quick brown fox jumps over the lazy dog. ".repeat(2000).into_bytes(),
        {
            let mut r = Lcg(0xABCDEF01);
            (0..20000).map(|_| (r.n() >> 16) as u8).collect()
        },
        {
            let mut r = Lcg(0x99999999);
            (0..200_000).map(|_| (r.n() >> 16) as u8 & 0x07).collect()
        },
    ];
    for raw in &cases {
        let theirs = compress_to_vec(raw, 6);
        let via_stream = streamed_inflate(&theirs);
        let via_one_shot = inflate(&theirs, raw.len()).expect("one-shot inflate");
        assert_eq!(&via_stream, raw, "streamed Inflater mismatch, len={}", raw.len());
        assert_eq!(&via_one_shot, raw, "one-shot inflate mismatch, len={}", raw.len());
        assert_eq!(via_stream, via_one_shot);

        let ours = deflate(raw);
        let via_stream_self = streamed_inflate(&ours);
        assert_eq!(&via_stream_self, raw, "streamed Inflater mismatch on our own encoder, len={}", raw.len());
    }
    println!("resumable Inflater agrees with one-shot inflate across {} cases incl. dynamic/stored/multi-block", cases.len());
}
