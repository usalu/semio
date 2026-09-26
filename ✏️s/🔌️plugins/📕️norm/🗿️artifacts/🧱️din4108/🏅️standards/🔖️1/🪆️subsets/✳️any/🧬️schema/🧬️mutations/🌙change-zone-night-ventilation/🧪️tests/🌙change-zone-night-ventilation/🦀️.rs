//! Named mutation smoke for `change-zone-night-ventilation`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_zone_night_ventilation() {
    let _ = crate::Din4108Snapshot::default();
}
