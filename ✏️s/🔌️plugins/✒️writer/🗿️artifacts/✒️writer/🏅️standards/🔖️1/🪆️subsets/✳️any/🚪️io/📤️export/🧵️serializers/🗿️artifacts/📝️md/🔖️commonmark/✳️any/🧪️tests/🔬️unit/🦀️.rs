use super::*;

#[semio_framework_async_macros::async_test]
async fn writer_into_md_carries_the_document_text() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello world");
    let outcome = WriterIntoMd::serialize(&snapshot).await.expect("serialize");
    let IoPayload::Text(text) = outcome.value else { panic!("expected text payload") };
    assert!(text.contains("hello world"));
}
