use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxBlock;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;
#[allow(unused_imports)]
use crate::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::docx::v_ecma_376::any::*;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxDocument;

#[semio_framework_async_macros::async_test]
async fn docx_into_writer_joins_paragraph_runs() {
    let body: Vec<DocxBlock> = "line one\nline two".split('\n').map(DocxBlock::paragraph).collect();
    let docx = build_minimal_docx(DocxDocument { body, styles: Vec::new() });
    let bytes = <DocxSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&docx);
    let outcome = DocxIntoWriter::deserialize(&IoPayload::Binary(bytes)).await.expect("deserialize");
    assert_eq!(crate::writer_text(&outcome.value), "line one\nline two");
}
