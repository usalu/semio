use semio_framework_os_kernel::io::io_mechanism::ArchiveChildren;
use semio_framework::io_schema::IoPayload;
#[allow(unused_imports)]
use crate::{writer_text, WriterSnapshot};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Serializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
#[allow(unused_imports)]
use semio_s_artifact_stdio_md::MdSnapshot;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::md::v_commonmark::any::*;

#[semio_framework_async_macros::async_test]
async fn writer_into_md_carries_the_document_text() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello world");
    let outcome = WriterIntoMd::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("serialize");
    let IoPayload::Text(text) = outcome.value else { panic!("expected text payload") };
    assert!(text.contains("hello world"));
}
