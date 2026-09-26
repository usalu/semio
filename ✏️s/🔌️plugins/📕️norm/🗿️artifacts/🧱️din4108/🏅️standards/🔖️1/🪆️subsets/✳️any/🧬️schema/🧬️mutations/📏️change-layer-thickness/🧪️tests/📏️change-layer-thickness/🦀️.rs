//! Named mutation smoke for `change-layer-thickness`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_layer_thickness() {
    let _ = crate::Din4108Snapshot::default();
}
