use super::*;

/// ✍️ Hand-built representative document — used across the artifact's own component tests.
fn jack_snapshot() -> crate::WriterSnapshot {
    crate::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
}

#[semio_framework_async_macros::async_test]
async fn writer_op_text_round_trips_every_variant() {
    let jack = jack_snapshot();
    store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::EditText(EditText { text: "line one\nline two".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::RenameWriter(RenameWriter { new_id: jack.id.clone() }));
    store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::ChangeUri(ChangeUri { new_uri: jack.uri.clone() }));
    store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: jack.language_id.clone() }));
}
