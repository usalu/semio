//! Named mutation smoke for `insert-element`.
#[semio_framework_async_macros::async_test]
async fn smoke_insert_element() {
    let _ = crate::Din4108Snapshot::default();
}
