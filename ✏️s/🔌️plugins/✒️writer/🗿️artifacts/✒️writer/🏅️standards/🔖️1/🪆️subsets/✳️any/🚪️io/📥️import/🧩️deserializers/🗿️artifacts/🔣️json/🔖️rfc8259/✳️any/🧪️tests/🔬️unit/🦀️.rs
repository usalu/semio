use super::*;

#[semio_framework_async_macros::async_test]
async fn json_into_writer_round_trips_a_real_snapshot() {
    let original = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
    let json = JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&original)));
    let text = store::ArtifactDsl::print_dsl(&json);
    let outcome = JsonIntoWriter::deserialize(&IoPayload::Text(text)).await.expect("deserialize");
    assert_eq!(outcome.value, original);
}
