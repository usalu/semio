//! Named mutation smoke for `insert-zone`.
#[semio_framework_async_macros::async_test]
async fn smoke_insert_zone() {
    let _ = crate::Din4108Snapshot::default();
}
