//! Named mutation smoke for `change-element-kind`.
#[semio_framework_async_macros::async_test]
async fn smoke_change_element_kind() {
    let _ = crate::Din4108Snapshot::default();
}
