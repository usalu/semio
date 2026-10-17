use semio_framework_os_kernel::io::io_mechanism::ArchiveChildren;
use semio_framework::io_schema::IoPayload;
#[allow(unused_imports)]
use crate::{writer_text, WriterSnapshot};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Serializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::*;

#[semio_framework_async_macros::async_test]
async fn writer_into_txt_emits_plain_document_text() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello\nworld");
    let outcome = WriterIntoTxt::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("serialize");
    assert_eq!(outcome.value, IoPayload::Text("hello\nworld".into()));
}
