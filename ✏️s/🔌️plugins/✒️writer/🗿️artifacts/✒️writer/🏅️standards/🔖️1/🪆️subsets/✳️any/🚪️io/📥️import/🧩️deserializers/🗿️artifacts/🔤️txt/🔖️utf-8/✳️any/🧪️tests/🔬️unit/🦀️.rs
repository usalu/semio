use super::*;

#[semio_framework_async_macros::async_test]
async fn txt_into_writer_uses_the_raw_body_as_document_text() {
    let outcome = TxtIntoWriter::deserialize(&IoPayload::Text("hello\nworld".into())).await.expect("deserialize");
    assert_eq!(crate::writer_text(&outcome.value), "hello\nworld");
    assert_eq!(outcome.value.language_id, "plain");
}
