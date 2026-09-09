use super::*;
use semio_framework::io::io_mechanism::Deserializer;

#[semio_framework_async_macros::async_test]
async fn writer_into_json_round_trips_through_json_into_writer() {
    let original = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
    let outcome = WriterIntoJson::serialize(&original).await.expect("serialize");
    let back = crate::io::import::deserializers::artifacts::json::v_rfc8259::any::JsonIntoWriter::deserialize(&outcome.value).await.expect("deserialize");
    assert_eq!(back.value, original);
}
