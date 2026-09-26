//! Named mutation smoke for `remove-thermal-bridge`.
#[semio_framework_async_macros::async_test]
async fn smoke_remove_thermal_bridge() {
    let _ = crate::Din4108Snapshot::default();
}
