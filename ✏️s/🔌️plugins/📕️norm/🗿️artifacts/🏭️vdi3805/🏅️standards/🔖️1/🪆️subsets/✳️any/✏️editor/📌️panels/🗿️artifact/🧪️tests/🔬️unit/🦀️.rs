
use super::*;
use crate::editor::vdi3805::testkit;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    assert_eq!(definition().body_key.as_deref(), Some(BODY_DOCUMENT));
    assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_family_headline() {
    let mut app = testkit::app_with_registry().await;
    assert!(testkit::render(&mut app, BODY_DOCUMENT).await.contains("checks"));
}
