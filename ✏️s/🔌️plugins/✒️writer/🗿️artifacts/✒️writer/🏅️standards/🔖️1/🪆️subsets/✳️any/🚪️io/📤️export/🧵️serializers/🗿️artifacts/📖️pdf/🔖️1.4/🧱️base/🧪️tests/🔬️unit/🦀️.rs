
use super::*;
#[semio_framework_async_macros::async_test]
async fn writer_into_pdf_preserves_text_and_page_size() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plaintext", "writer://id", "hello");
    let outcome = WriterIntoPdf::serialize(&snapshot).await.expect("serialize");
    let IoPayload::Binary(bytes) = outcome.value else { panic!("expected binary payload") };
    let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&decoded)).unwrap();
    assert_eq!(actual["pages"], serde_json::json!([{"width":612.0,"height":792.0,"text":"hello"}]));
}
