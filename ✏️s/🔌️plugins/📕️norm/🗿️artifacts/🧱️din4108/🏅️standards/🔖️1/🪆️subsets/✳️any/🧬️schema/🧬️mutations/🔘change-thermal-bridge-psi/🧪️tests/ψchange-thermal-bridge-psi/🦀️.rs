//! Named mutation smoke for `change-thermal-bridge-psi`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_thermal_bridge_psi() {
    let _ = crate::Din4108Snapshot::default();
}
