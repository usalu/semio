use super::*;
use semio_framework::io::io_mechanism::Deserializer;

#[semio_framework_async_macros::async_test]
async fn writer_into_json_round_trips_through_json_into_writer() {
    let original = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
    let outcome = WriterIntoJson::serialize(&original, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.expect("serialize");
    let IoPayload::Text(text) = &outcome.value else { panic!("JSON native Text carrier") };
    let json = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("actual JSON native owner");
    let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&original)).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&json.to_pack_value())).unwrap();
    assert_eq!(observed, expected, "independent full Writer JSON fields");
    let back = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::JsonIntoWriter::deserialize(&outcome.value).await.expect("deserialize");
    assert_eq!(back.value, original);
}
