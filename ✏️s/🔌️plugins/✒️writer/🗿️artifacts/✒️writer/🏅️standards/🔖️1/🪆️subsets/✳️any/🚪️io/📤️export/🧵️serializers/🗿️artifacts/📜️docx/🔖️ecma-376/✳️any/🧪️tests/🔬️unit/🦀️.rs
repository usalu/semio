use super::*;

#[semio_framework_async_macros::async_test]
async fn writer_into_docx_round_trips_through_docx_into_writer() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
    let outcome = WriterIntoDocx::serialize(&snapshot, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.expect("serialize");
    let IoPayload::Binary(bytes) = outcome.value else { panic!("expected binary payload") };
    let decoded = <DocxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert!(!decoded.project_document().expect("project").body.is_empty());
}
