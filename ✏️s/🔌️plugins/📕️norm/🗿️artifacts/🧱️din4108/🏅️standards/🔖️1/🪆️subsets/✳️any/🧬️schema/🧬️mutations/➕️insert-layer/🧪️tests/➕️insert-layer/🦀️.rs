//! Named mutation smoke for `insert-layer`.
#[semio_framework_async_macros::async_test]
async fn smoke_insert_layer() {
    let _ = crate::Din4108Snapshot::default();
}
