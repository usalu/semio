
use super::*;
use crate::schema;

#[semio_framework_async_macros::async_test]
async fn jack_example_dsl_round_trips() {
    let document = parse_dsl(JACK_EXAMPLE_TEXT).expect("parse jack example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn dag_jack_example_dsl_round_trips() {
    let document = parse_dsl(DAG_JACK_EXAMPLE_TEXT).expect("parse dag.jack example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

/// ✍️ Hand-built representative document exercising the multiline/quoted-text path.
fn jack_snapshot() -> WriterSnapshot {
    crate::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
}

#[semio_framework_async_macros::async_test]
async fn writer_dsl_round_trips_empty_and_jack_snapshots() {
    store::os_store::test_support::assert_dsl_round_trip(&schema::empty_writer_snapshot());
    store::os_store::test_support::assert_dsl_round_trip(&jack_snapshot());
}

/// 📄️ The hand-rolled `document` codec (`📸️snapshot/🦀️.rs`'s
/// `print_writer_snapshot_body`) prints one hex-encoded `key=value` line per persistent field —
/// `document` is now a two-string CHILD HANDLE, not the embedded text, so this law only checks
/// the scalar fields print readably; the actual text content is proven separately by
/// `writer_dsl_round_trips_empty_and_jack_snapshots` (round trip) and `writer_text` reads.
#[semio_framework_async_macros::async_test]
async fn writer_dsl_prints_readable_scalar_fields() {
    let printed = print_dsl(&jack_snapshot());
    assert!(printed.contains(&format!("schema={}", hex_encode_for_test("writer.document"))));
    assert!(printed.contains(&format!("id={}", hex_encode_for_test("jack"))));
    assert!(printed.contains(&format!("languageId={}", hex_encode_for_test("jack"))));
    assert!(printed.contains(&format!("uri={}", hex_encode_for_test("writer://jack"))));
    assert!(printed.contains("document=["));
}

fn hex_encode_for_test(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}
