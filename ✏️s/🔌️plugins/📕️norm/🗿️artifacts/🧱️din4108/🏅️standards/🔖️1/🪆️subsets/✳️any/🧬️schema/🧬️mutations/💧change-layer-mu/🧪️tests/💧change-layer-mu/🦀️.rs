//! Named mutation smoke for `change-layer-mu`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_layer_mu() {
    let _ = crate::Din4108Snapshot::default();
}
