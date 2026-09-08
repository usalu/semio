
use super::*;

#[semio_framework_async_macros::async_test]
async fn json_into_writer_round_trips_a_real_snapshot() {
    let original = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
    let text = dsl::os_pack::json::to_json_string(&original);
    let outcome = JsonIntoWriter::deserialize(&IoPayload::Text(text)).await.expect("deserialize");
    assert_eq!(outcome.value, original);
}
