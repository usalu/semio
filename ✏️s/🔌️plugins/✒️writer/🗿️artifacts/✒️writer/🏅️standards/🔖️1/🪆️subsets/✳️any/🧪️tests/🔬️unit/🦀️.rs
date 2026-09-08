
use super::*;

#[semio_framework_async_macros::async_test]
async fn subset_dialect_is_the_canonical_writer_dialect() {
    assert_eq!(subset().dialect, crate::WRITER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn subset_declares_ten_io_entries() {
    assert_eq!(subset().io.entries.len(), 10);
}
