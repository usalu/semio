//! Named mutation smoke for `remove-layer`.
#[semio_framework_async_macros::async_test]
async fn smoke_remove_layer() {
    let _ = crate::Din4108Snapshot::default();
}
