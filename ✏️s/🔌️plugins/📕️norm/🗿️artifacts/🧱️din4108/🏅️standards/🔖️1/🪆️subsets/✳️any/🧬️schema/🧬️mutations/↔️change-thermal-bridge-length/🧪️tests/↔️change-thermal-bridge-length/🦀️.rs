//! Named mutation smoke for `change-thermal-bridge-length`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_thermal_bridge_length() {
    let _ = crate::Din4108Snapshot::default();
}
