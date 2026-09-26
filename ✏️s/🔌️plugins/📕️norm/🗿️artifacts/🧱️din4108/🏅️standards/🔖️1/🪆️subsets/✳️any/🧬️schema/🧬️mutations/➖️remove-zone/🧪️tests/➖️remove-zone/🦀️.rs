//! Named mutation smoke for `remove-zone`.
#[semio_framework_async_macros::async_test]
async fn smoke_remove_zone() {
    let _ = crate::Din4108Snapshot::default();
}
