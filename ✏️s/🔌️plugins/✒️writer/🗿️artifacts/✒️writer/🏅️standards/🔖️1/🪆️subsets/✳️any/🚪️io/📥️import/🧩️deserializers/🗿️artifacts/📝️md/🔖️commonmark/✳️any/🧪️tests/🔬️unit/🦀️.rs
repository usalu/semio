
use super::*;

#[semio_framework_async_macros::async_test]
async fn md_into_writer_uses_the_markdown_text_as_document_text() {
    let outcome = MdIntoWriter::deserialize(&IoPayload::Text("hello world".into())).await.expect("deserialize");
    assert_eq!(crate::writer_text(&outcome.value), "hello world");
    assert_eq!(outcome.value.language_id, "plain");
}
