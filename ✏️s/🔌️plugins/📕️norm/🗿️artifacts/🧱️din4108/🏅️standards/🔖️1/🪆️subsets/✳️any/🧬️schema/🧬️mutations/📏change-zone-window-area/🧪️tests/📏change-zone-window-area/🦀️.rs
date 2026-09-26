//! Named mutation smoke for `change-zone-window-area`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_zone_window_area() {
    let _ = crate::Din4108Snapshot::default();
}
