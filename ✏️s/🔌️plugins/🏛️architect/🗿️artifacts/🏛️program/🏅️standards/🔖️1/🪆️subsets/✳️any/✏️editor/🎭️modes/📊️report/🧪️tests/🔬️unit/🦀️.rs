use super::*;

#[semio_framework_async_macros::async_test]
async fn the_mode_declares_no_layout_of_its_own() {
    let definition = definition();
    assert_eq!(definition.id, ARCHITECT_MODE_REPORT);
    assert!(definition.layout_id.is_none());
}
