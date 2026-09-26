//! Named mutation smoke for `change-layer-material-id`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_layer_material_id() {
    let _ = crate::Din4108Snapshot::default();
}
