use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_md::MdSnapshot;
#[allow(unused_imports)]
use crate::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::md::v_commonmark::any::*;

#[semio_framework_async_macros::async_test]
async fn md_into_writer_uses_the_markdown_text_as_document_text() {
    let outcome = MdIntoWriter::deserialize(&IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(&MdSnapshot::from_text("hello world")))).await.expect("deserialize");
    assert_eq!(crate::writer_text(&outcome.value), "hello world");
    assert_eq!(outcome.value.language_id, "plain");
}
