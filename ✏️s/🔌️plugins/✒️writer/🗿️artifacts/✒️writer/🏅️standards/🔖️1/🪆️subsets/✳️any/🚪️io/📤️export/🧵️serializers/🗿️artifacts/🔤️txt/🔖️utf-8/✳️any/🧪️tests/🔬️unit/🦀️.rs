use super::*;

#[semio_framework_async_macros::async_test]
async fn writer_into_txt_emits_plain_document_text() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello\nworld");
    let outcome = WriterIntoTxt::serialize(&snapshot, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.expect("serialize");
    assert_eq!(outcome.value, IoPayload::Text("hello\nworld".into()));
}
