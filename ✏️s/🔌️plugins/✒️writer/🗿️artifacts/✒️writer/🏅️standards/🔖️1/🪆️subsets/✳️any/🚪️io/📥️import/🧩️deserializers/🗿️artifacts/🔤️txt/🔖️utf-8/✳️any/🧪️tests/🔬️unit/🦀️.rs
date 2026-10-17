use semio_framework::io_schema::IoPayload;
#[allow(unused_imports)]
use crate::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_s_artifact_stdio_txt::TxtSnapshot;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::*;

#[semio_framework_async_macros::async_test]
async fn txt_into_writer_uses_the_raw_body_as_document_text() {
    let outcome = TxtIntoWriter::deserialize(&IoPayload::Text("hello\nworld".into())).await.expect("deserialize");
    assert_eq!(crate::writer_text(&outcome.value), "hello\nworld");
    assert_eq!(outcome.value.language_id, "plain");
}
