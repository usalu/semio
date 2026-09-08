
use super::*;
use protocol::DiffCodec;

fn jack_snapshot() -> WriterSnapshot {
    crate::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
}

#[semio_framework_async_macros::async_test]
async fn writer_diff_print_parse_round_trips() {
    let diffs = vec![diff_set_text("hello", "jack", "jack"), diff_set_snapshot(&jack_snapshot()), WriterDiff::default()];
    for diff in diffs {
        let printed = diff.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
        let parsed = WriterDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff failed for {printed:?}: {e}"));
        assert_eq!(parsed, diff, "DiffCodec text round trip diverged for {printed:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn writer_diff_encode_decode_round_trips_and_matches_text() {
    let diffs = vec![diff_set_text("hello", "jack", "jack"), diff_set_snapshot(&jack_snapshot()), WriterDiff::default()];
    for diff in diffs {
        let bytes = diff.encode_diff().expect("encode_diff");
        let decoded = WriterDiff::decode_diff(&bytes).expect("decode_diff");
        assert_eq!(decoded, diff, "DiffCodec binary round trip diverged");
    }
}

/// 🔺️ `diff_set_text` mints a real content-addressed `document` handle and seeds the working-
/// scene cache, honestly replacing the retired byte-range-edit law (composed-child handles are
/// whole-value replacements, not sub-string patches — see this file's `🔖️Builders` doc comment).
#[semio_framework_async_macros::async_test]
async fn diff_set_text_mints_a_document_handle_and_caches_its_text() {
    let base = WriterSnapshot::default();
    let diff = diff_set_text("hio", "jack", "plaintext");
    let next = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(crate::writer_text(&next), "hio");
}
