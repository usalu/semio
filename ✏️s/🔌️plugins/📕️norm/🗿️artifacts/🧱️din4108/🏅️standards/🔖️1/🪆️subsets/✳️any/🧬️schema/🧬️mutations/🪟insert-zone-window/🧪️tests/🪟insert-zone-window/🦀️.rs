//! Named mutation smoke for `insert-zone-window`.
#[semio_framework_async_macros::async_test]
async fn smoke_insert_zone_window() {
    let _ = crate::Din4108Snapshot::default();
}
