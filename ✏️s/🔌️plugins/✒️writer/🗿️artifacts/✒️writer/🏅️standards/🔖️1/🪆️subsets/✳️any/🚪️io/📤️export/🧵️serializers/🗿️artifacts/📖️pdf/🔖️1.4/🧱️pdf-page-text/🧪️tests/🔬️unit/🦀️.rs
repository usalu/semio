use semio_framework_os_kernel::io::io_mechanism::ArchiveChildren;
use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use crate::{writer_text, WriterSnapshot};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Serializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
#[allow(unused_imports)]
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
#[allow(unused_imports)]
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::STDIO_PDF_DOCUMENT_SCHEMA;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::base::*;
#[semio_framework_async_macros::async_test]
async fn writer_into_pdf_preserves_text_and_page_size() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plaintext", "writer://id", "hello");
    let outcome = WriterIntoPdf::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("serialize");
    let IoPayload::Binary(bytes) = outcome.value else { panic!("expected binary payload") };
    let decoded = <PdfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&decoded)).unwrap();
    assert_eq!(actual["pages"], serde_json::json!([{"width":612.0,"height":792.0,"text":"hello"}]));
}
