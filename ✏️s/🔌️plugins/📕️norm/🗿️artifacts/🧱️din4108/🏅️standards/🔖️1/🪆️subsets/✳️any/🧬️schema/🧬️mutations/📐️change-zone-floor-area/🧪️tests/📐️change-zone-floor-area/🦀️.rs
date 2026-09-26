//! Named mutation smoke for `change-zone-floor-area`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_zone_floor_area() {
    let _ = crate::Din4108Snapshot::default();
}
