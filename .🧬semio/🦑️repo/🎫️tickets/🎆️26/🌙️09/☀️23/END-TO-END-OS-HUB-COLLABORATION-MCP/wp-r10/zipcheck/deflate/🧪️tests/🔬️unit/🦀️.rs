
use super::*;

fn lcg_bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut state = seed | 1;
    (0..len)
        .map(|_| {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (state >> 33) as u8
        })
        .collect()
}

#[test]
fn round_trips_empty_and_small_inputs() {
    for sample in [b"".as_slice(), b"a", b"ab", b"hello, world!"] {
        let compressed = deflate(sample);
        let decompressed = inflate(&compressed, sample.len()).expect("inflate ours");
        assert_eq!(decompressed, sample);
    }
}

#[test]
fn round_trips_repetitive_input_that_forces_long_matches() {
    let sample: Vec<u8> = std::iter::repeat(b'x').take(5000).collect();
    let compressed = deflate(&sample);
    assert!(compressed.len() < sample.len(), "a real LZ77 matcher must shrink a run of one byte");
    let decompressed = inflate(&compressed, sample.len()).expect("inflate ours");
    assert_eq!(decompressed, sample);
}

#[test]
fn round_trips_pseudo_random_lcg_input_across_sizes() {
    for (seed, len) in [(1u64, 0usize), (2, 1), (3, 63), (4, 64), (5, 1023), (6, 4097), (7, 40000)] {
        let sample = lcg_bytes(seed, len);
        let compressed = deflate(&sample);
        let decompressed = inflate(&compressed, sample.len()).expect("inflate ours");
        assert_eq!(decompressed, sample, "mismatch at seed={seed} len={len}");
    }
}

/// 🌊️ Drives `Inflater::advance` one admitted byte at a time — the exact granularity
/// `DeflateRetainedCursor` uses in production — and returns everything it wrote.
fn streamed_inflate_result(compressed: &[u8]) -> Result<Vec<u8>, DeflateError> {
    let mut inflater = Inflater::new();
    let mut output = Vec::new();
    let mut index = 0usize;
    let mut pending: Option<u8> = None;
    loop {
        if pending.is_none() && index < compressed.len() {
            pending = Some(compressed[index]);
            index += 1;
        }
        let input_complete = index >= compressed.len();
        match inflater.advance(&mut pending, input_complete)? {
            InflateOutcome::NeedInput if input_complete => return Err(DeflateError::UnexpectedEnd),
            InflateOutcome::NeedInput => {}
            InflateOutcome::Wrote(byte) => output.push(byte),
            InflateOutcome::Done => return Ok(output),
        }
    }
}

fn streamed_inflate(compressed: &[u8]) -> Vec<u8> {
    streamed_inflate_result(compressed).expect("advance")
}

fn btype_of(compressed: &[u8]) -> u8 {
    (compressed[0] >> 1) & 0b11
}

#[test]
fn stream_produces_the_same_bytes_as_one_shot_inflate() {
    let sample = lcg_bytes(9, 5000);
    let compressed = deflate(&sample);
    assert_eq!(streamed_inflate(&compressed), sample);
    assert_eq!(inflate(&compressed, sample.len()).expect("inflate ours"), sample);
}

#[test]
fn accepts_a_short_final_huffman_tail() {
    let fixture = include_str!("../../🧫️fixtures/🏁️deflate-tail-cases.json");
    let file: TailCorpusFile = serde_json::from_str(fixture).expect("valid DEFLATE tail fixture");
    assert_eq!(file.cases.len(), 1, "expected one fixed-EOB tail vector");
    for case in file.cases {
        assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&case.stored, 1).expect("miniz inflates fixed EOB"), case.raw, "miniz mismatch for {}", case.name);
        assert_eq!(inflate(&case.stored, 0).expect("inflate fixed EOB"), case.raw, "one-shot mismatch for {}", case.name);
        assert_eq!(streamed_inflate_result(&case.stored).expect("streamed fixed EOB"), case.raw, "streamed mismatch for {}", case.name);
    }
}

#[test]
fn rejects_truncation_before_the_final_huffman_symbol() {
    let sample = "the quick brown fox jumps over the lazy dog. ".repeat(500).into_bytes();
    let compressed = deflate(&sample);
    let truncated = &compressed[..1];
    assert_eq!(inflate(truncated, sample.len()), Err(DeflateError::UnexpectedEnd));
    assert_eq!(streamed_inflate_result(truncated), Err(DeflateError::UnexpectedEnd));
}

#[test]
fn reads_miniz_oxide_dynamic_huffman_blocks() {
    let sample = "the quick brown fox jumps over the lazy dog. ".repeat(2000).into_bytes();
    let theirs = miniz_oxide::deflate::compress_to_vec(&sample, 6);
    assert_eq!(btype_of(&theirs), 2, "expected miniz_oxide to pick BTYPE=10 dynamic Huffman here");
    assert_eq!(inflate(&theirs, sample.len()).expect("inflate dynamic huffman"), sample);
    assert_eq!(streamed_inflate(&theirs), sample);
}

#[test]
fn reads_a_stored_block() {
    let raw: [u8; 5] = [0, 0, 0, 0xff, 0xff];
    let mut stored = vec![0b001u8];
    let len = raw.len() as u16;
    stored.extend_from_slice(&len.to_le_bytes());
    stored.extend_from_slice(&(!len).to_le_bytes());
    stored.extend_from_slice(&raw);
    assert_eq!(btype_of(&stored), 0, "expected BTYPE=00 stored block");
    assert_eq!(inflate(&stored, raw.len()).expect("inflate stored block"), raw);
    assert_eq!(streamed_inflate(&stored), raw);
}

#[test]
fn round_trips_multi_block_input_spanning_the_window() {
    let sample: Vec<u8> = (0..3u64).flat_map(|block| lcg_bytes(100 + block, 40_000)).collect();
    assert!(sample.len() > 32768 * 3, "must exceed three window sizes to span block boundaries");
    let theirs = miniz_oxide::deflate::compress_to_vec(&sample, 6);
    assert_eq!(inflate(&theirs, sample.len()).expect("inflate large miniz output"), sample);
    assert_eq!(streamed_inflate(&theirs), sample);
    let ours = deflate(&sample);
    assert_eq!(inflate(&ours, sample.len()).expect("inflate our own large output"), sample);
    assert_eq!(streamed_inflate(&ours), sample);
}

fn retained_inflate(compressed: &[u8], maximum_output_bytes: usize) -> (Vec<u8>, RetainedInflateProgress, usize, usize) {
    let mut inflater = Inflater::try_new_retained(maximum_output_bytes, RETAINED_INFLATE_WINDOW_BYTES).expect("retained inflater bounds");
    let demand = inflater.next_retained_allocation_bytes().expect("retained history demand");
    let step = inflater.reserve_retained_history(demand).expect("retained history allocation");
    assert!(step.progressed && step.allocated_bytes >= demand);
    let pointer = inflater.retained_history_ptr().expect("retained history pointer");
    let mut output = Vec::new();
    let mut input = 0usize;
    let mut pending = None;
    loop {
        if pending.is_none() && input < compressed.len() {
            pending = Some(compressed[input]);
            input += 1;
        }
        match inflater.advance(&mut pending, input == compressed.len()).expect("retained decode") {
            InflateOutcome::NeedInput if input == compressed.len() => panic!("retained decoder stalled at sealed input"),
            InflateOutcome::NeedInput => {}
            InflateOutcome::Wrote(byte) => output.push(byte),
            InflateOutcome::Done => break,
        }
    }
    let progress = inflater.retained_progress().expect("retained progress");
    assert_eq!(close_retained(&mut inflater), step.allocated_bytes);
    (output, progress, step.allocated_bytes, pointer)
}

fn close_retained(inflater: &mut Inflater) -> usize {
    let mut released = 0usize;
    loop {
        let demand = inflater.next_retained_release_allocation_bytes().unwrap_or(0);
        match inflater.close_retained_step(1, demand) {
            RetainedInflateCloseStep::Pending { released_bytes, .. } => released += released_bytes,
            RetainedInflateCloseStep::Complete => return released,
        }
    }
}

#[test]
fn retained_inflater_physical_allocation_requires_exact_grant_and_releases_actual_backing() {
    assert_eq!(
        Inflater::try_new_retained(RETAINED_INFLATE_WINDOW_BYTES, RETAINED_INFLATE_WINDOW_BYTES - 1).err(),
        Some(DeflateError::OutputLimitExceeded)
    );
    let mut inflater = Inflater::try_new_retained(64 * 1024, RETAINED_INFLATE_WINDOW_BYTES).expect("retained bounds");
    assert_eq!(inflater.retained_allocated_bytes(), 0);
    assert_eq!(inflater.next_retained_allocation_bytes(), Some(RETAINED_INFLATE_WINDOW_BYTES));
    let mut pending = Some(0b11);
    assert_eq!(inflater.advance(&mut pending, false).expect("allocation backpressure"), InflateOutcome::NeedInput);
    assert_eq!(pending, Some(0b11));
    let before = inflater.retained_progress().expect("pre-allocation progress");
    assert_eq!(inflater.reserve_retained_history(0).expect("zero allocation grant"), RetainedInflateAllocationStep::default());
    assert_eq!(inflater.reserve_retained_history(RETAINED_INFLATE_WINDOW_BYTES - 1).expect("subexact allocation grant"), RetainedInflateAllocationStep::default());
    assert_eq!(inflater.retained_progress().expect("unchanged pre-allocation progress"), before);
    let step = inflater.reserve_retained_history(RETAINED_INFLATE_WINDOW_BYTES).expect("exact allocation grant");
    assert!(step.progressed && step.allocated_bytes >= RETAINED_INFLATE_WINDOW_BYTES);
    assert_eq!(inflater.retained_allocated_bytes(), step.allocated_bytes);
    let pointer = inflater.retained_history_ptr().expect("history pointer");
    assert_eq!(inflater.close_retained_step(0, 0), RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(inflater.retained_history_ptr(), Some(pointer));
    assert_eq!(inflater.close_retained_step(1, 0), RetainedInflateCloseStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(inflater.close_retained_step(1, 0), RetainedInflateCloseStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(inflater.next_retained_release_allocation_bytes(), Some(step.allocated_bytes));
    assert_eq!(inflater.close_retained_step(0, step.allocated_bytes - 1), RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(inflater.retained_history_ptr(), Some(pointer));
    assert_eq!(inflater.close_retained_step(0, step.allocated_bytes), RetainedInflateCloseStep::Pending { released_items: 0, released_bytes: step.allocated_bytes });
    assert_eq!(inflater.close_retained_step(0, 0), RetainedInflateCloseStep::Complete);
    assert!(inflater.retained_terminal_is_empty());
    eprintln!("[DEBUG] retained-inflater allocation={} pointer-stable=true exact-release={}", step.allocated_bytes, step.allocated_bytes);
}

#[test]
fn retained_inflater_physical_decodes_stored_fixed_dynamic_and_window_wrap_with_oracle() {
    let stored_raw = [0, 0, 0, 0xff, 0xff];
    let mut stored = vec![0b001u8];
    stored.extend_from_slice(&(stored_raw.len() as u16).to_le_bytes());
    stored.extend_from_slice(&(!(stored_raw.len() as u16)).to_le_bytes());
    stored.extend_from_slice(&stored_raw);
    let fixed_raw = vec![b'x'; 10_000];
    let fixed = deflate(&fixed_raw);
    let dynamic_raw = "the quick brown fox jumps over the lazy dog. ".repeat(2000).into_bytes();
    let dynamic = miniz_oxide::deflate::compress_to_vec(&dynamic_raw, 6);
    let wrap_raw: Vec<u8> = (0..3u64).flat_map(|block| lcg_bytes(100 + block, 40_000)).collect();
    let wrap = miniz_oxide::deflate::compress_to_vec(&wrap_raw, 6);
    for (name, compressed, expected) in [("stored", stored, stored_raw.to_vec()), ("fixed", fixed, fixed_raw), ("dynamic", dynamic, dynamic_raw), ("window-wrap", wrap, wrap_raw)] {
        let oracle = miniz_oxide::inflate::decompress_to_vec_with_limit(&compressed, expected.len().max(1)).expect("miniz retained oracle");
        let (actual, progress, allocated, _) = retained_inflate(&compressed, expected.len().max(RETAINED_INFLATE_WINDOW_BYTES));
        assert_eq!(actual, oracle, "retained {name} differs from miniz");
        assert_eq!(actual, expected, "retained {name} output");
        assert!(progress.history_bytes <= RETAINED_INFLATE_WINDOW_BYTES);
        assert_eq!(allocated, progress.allocated_bytes);
    }
}

#[test]
fn retained_inflater_physical_observes_overlap_max_distance_repeats_and_reuses_backing() {
    let overlap_raw = vec![b'z'; 20_000];
    let overlap = miniz_oxide::deflate::compress_to_vec(&overlap_raw, 9);
    let (_, overlap_progress, _, _) = retained_inflate(&overlap, RETAINED_INFLATE_WINDOW_BYTES);
    assert!(overlap_progress.observed_distance_one);

    let prefix = lcg_bytes(0x5eed, RETAINED_INFLATE_WINDOW_BYTES);
    let max_distance_raw: Vec<u8> = prefix.iter().copied().chain(prefix.iter().copied()).collect();
    let max_distance = deflate(&max_distance_raw);
    assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&max_distance, max_distance_raw.len()).expect("miniz maximum-distance oracle"), max_distance_raw);
    let (_, distance_progress, _, _) = retained_inflate(&max_distance, max_distance_raw.len());
    assert_eq!(distance_progress.maximum_distance, RETAINED_INFLATE_WINDOW_BYTES);

    let dynamic_raw = "aaaaaaaaaabbbbbbbbbbcccccccccc0123456789".repeat(4096).into_bytes();
    let dynamic = miniz_oxide::deflate::compress_to_vec(&dynamic_raw, 9);
    let (_, repeat_progress, _, _) = retained_inflate(&dynamic, dynamic_raw.len());
    assert_eq!(repeat_progress.dynamic_repeat_codes & 0b111, 0b111);

    let mut inflater = Inflater::try_new_retained(64 * 1024, RETAINED_INFLATE_WINDOW_BYTES).expect("retained reuse bounds");
    let exact = inflater.next_retained_allocation_bytes().expect("reuse allocation");
    let allocated = inflater.reserve_retained_history(exact).expect("reuse reserve").allocated_bytes;
    let pointer = inflater.retained_history_ptr().expect("reuse pointer");
    for raw in [b"first compressed segment".repeat(800), b"second compressed segment".repeat(800)] {
        let compressed = miniz_oxide::deflate::compress_to_vec(&raw, 6);
        let mut output = Vec::new();
        let mut input = 0usize;
        let mut pending = None;
        loop {
            if pending.is_none() && input < compressed.len() {
                pending = Some(compressed[input]);
                input += 1;
            }
            match inflater.advance(&mut pending, input == compressed.len()).expect("reused retained decode") {
                InflateOutcome::NeedInput => {}
                InflateOutcome::Wrote(byte) => output.push(byte),
                InflateOutcome::Done => break,
            }
        }
        assert_eq!(output, raw);
        inflater.reset_retained().expect("retain backing for next segment");
        assert_eq!(inflater.retained_history_ptr(), Some(pointer));
        assert_eq!(inflater.retained_allocated_bytes(), allocated);
    }
    assert_eq!(close_retained(&mut inflater), allocated);
    assert!(inflater.retained_terminal_is_empty());
    eprintln!("[DEBUG] retained-inflater distance-1=true distance-32768=true repeat-codes=16,17,18 reuse=true exact-release={allocated}/{allocated}");
}

//#region 🧪️Oracle
/// 🧪️ `miniz_oxide` lives ONLY in `[dev-dependencies]` here — the differential oracle proving
/// round-trip compatibility in both directions with what is already persisted.
#[derive(serde::Deserialize)]
struct CorpusFile {
    cases: Vec<CorpusCase>,
}

#[derive(serde::Deserialize)]
struct CorpusCase {
    seed: u64,
    len: usize,
}

#[derive(serde::Deserialize)]
struct TailCorpusFile {
    cases: Vec<TailCorpusCase>,
}

#[derive(serde::Deserialize)]
struct TailCorpusCase {
    name: String,
    stored: Vec<u8>,
    raw: Vec<u8>,
}

#[test]
fn ours_inflates_miniz_oxide_output_and_vice_versa() {
    let raw = include_str!("../../🧫️fixtures/🧪️deflate-corpus.json");
    let file: CorpusFile = serde_json::from_str(raw).expect("valid deflate corpus fixture");
    assert!(file.cases.len() >= 8, "expected a real length sweep");
    for case in &file.cases {
        let sample = lcg_bytes(case.seed, case.len);
        let theirs = miniz_oxide::deflate::compress_to_vec(&sample, 6);
        let ours_from_theirs = inflate(&theirs, sample.len()).unwrap_or_else(|error| panic!("ours failed to inflate miniz_oxide output at seed={} len={}: {error:?}", case.seed, case.len));
        assert_eq!(ours_from_theirs, sample, "ours-from-theirs mismatch at seed={} len={}", case.seed, case.len);

        let ours = deflate(&sample);
        let theirs_from_ours = miniz_oxide::inflate::decompress_to_vec_with_limit(&ours, sample.len().max(1)).unwrap_or_else(|error| panic!("miniz_oxide failed to inflate our output at seed={} len={}: {error:?}", case.seed, case.len));
        assert_eq!(theirs_from_ours, sample, "theirs-from-ours mismatch at seed={} len={}", case.seed, case.len);
    }
}
//#endregion 🧪️Oracle
