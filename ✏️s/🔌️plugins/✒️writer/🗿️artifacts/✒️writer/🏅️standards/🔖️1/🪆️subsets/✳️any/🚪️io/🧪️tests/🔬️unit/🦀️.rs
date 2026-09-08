
use super::*;

#[semio_framework_async_macros::async_test]
async fn io_declares_ten_entries_five_formats_both_directions() {
    assert_eq!(entries().len(), 10);
}
