#[allow(unused_imports)]
use crate::{WriterMutation, WriterSnapshot, WRITER_DIALECT, WRITER_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
#[allow(unused_imports)]
use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
use crate::standards::v1::subsets::any::io::{entries};
use crate::standards::v1::subsets::any::io::*;

#[semio_framework_async_macros::async_test]
async fn io_declares_ten_entries_five_formats_both_directions() {
    assert_eq!(entries().len(), 10);
}
