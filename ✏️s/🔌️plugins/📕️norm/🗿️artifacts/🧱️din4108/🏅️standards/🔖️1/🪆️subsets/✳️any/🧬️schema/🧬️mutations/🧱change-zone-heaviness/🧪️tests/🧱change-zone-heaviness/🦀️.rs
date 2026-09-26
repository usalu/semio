//! Named mutation smoke for `change-zone-heaviness`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_zone_heaviness() {
    let _ = crate::Din4108Snapshot::default();
}
