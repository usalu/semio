use super::*;
use semio_s_artifact_stdio_docx::engine::build_minimal_docx;
use semio_s_artifact_stdio_docx::schema::snapshot::DocxDocument;

#[semio_framework_async_macros::async_test]
async fn docx_into_writer_joins_paragraph_runs() {
    let body: Vec<DocxBlock> = "line one\nline two".split('\n').map(DocxBlock::paragraph).collect();
    let docx = build_minimal_docx(DocxDocument { body, styles: Vec::new() });
    let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&docx);
    let outcome = DocxIntoWriter::deserialize(&IoPayload::Binary(bytes)).await.expect("deserialize");
    assert_eq!(crate::writer_text(&outcome.value), "line one\nline two");
}
