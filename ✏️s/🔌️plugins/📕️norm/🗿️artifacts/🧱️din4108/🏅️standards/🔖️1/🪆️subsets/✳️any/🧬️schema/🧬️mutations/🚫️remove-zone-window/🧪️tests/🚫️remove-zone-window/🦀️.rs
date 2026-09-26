//! Named mutation smoke for `remove-zone-window`.
#[semio_framework_async_macros::async_test]
async fn smoke_remove_zone_window() {
    let _ = crate::Din4108Snapshot::default();
}
