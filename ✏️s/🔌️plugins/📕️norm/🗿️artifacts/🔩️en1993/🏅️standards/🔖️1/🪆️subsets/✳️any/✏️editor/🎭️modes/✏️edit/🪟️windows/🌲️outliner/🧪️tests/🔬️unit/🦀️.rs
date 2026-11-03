use crate::modes::edit::windows::outliner::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_OUTLINER);
    assert_eq!(definition().id, WINDOW_OUTLINER);
}

#[semio_framework_async_macros::async_test]
async fn renders_every_collection_with_a_select_row() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_OUTLINER).await;
    assert!(rendered.contains("member-b1") && rendered.contains("joint-j1"), "outliner lists entities: {rendered}");
    context::close(&mut app);
}
