use semio_framework_os_kernel::io::io_mechanism::ArchiveChildren;
use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;
#[allow(unused_imports)]
use crate::{writer_text, WriterSnapshot};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Serializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
#[allow(unused_imports)]
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxBlock;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::docx::v_ecma_376::any::*;

#[semio_framework_async_macros::async_test]
async fn writer_into_docx_round_trips_through_docx_into_writer() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
    let outcome = WriterIntoDocx::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("serialize");
    let IoPayload::Binary(bytes) = outcome.value else { panic!("expected binary payload") };
    let decoded = <DocxSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert!(!decoded.project_document().expect("project").body.is_empty());
}
