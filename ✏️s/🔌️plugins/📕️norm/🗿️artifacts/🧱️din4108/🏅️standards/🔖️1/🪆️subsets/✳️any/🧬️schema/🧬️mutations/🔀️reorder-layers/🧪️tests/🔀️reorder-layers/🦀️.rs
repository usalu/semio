//! Named mutation smoke for `reorder-layers`.
#[semio_framework_async_macros::async_test]
async fn smoke_reorder_layers() {
    let _ = crate::Din4108Snapshot::default();
}
