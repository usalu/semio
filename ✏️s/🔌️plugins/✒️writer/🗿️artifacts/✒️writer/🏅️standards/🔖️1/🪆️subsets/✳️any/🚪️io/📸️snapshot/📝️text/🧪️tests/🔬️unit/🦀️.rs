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

/// 📄️ The derived text prints every persistent scalar readably and `document` as its two-string
/// CHILD HANDLE; the text content itself is proven by `writer_dsl_round_trips_empty_and_jack_snapshots`.
#[semio_framework_async_macros::async_test]
async fn writer_dsl_prints_readable_scalar_fields() {
    let printed = print_dsl(&jack_snapshot());
    assert!(printed.contains("schema=writer.document"));
    assert!(printed.contains("id=jack"));
    assert!(printed.contains("language-id=jack"));
    assert!(printed.contains("uri=\"writer://jack\""));
    assert!(printed.contains("document=child_id="));
}
