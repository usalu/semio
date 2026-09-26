//! Named mutation smoke for `insert-thermal-bridge`.
#[semio_framework_async_macros::async_test]
async fn smoke_insert_thermal_bridge() {
    let _ = crate::Din4108Snapshot::default();
}
