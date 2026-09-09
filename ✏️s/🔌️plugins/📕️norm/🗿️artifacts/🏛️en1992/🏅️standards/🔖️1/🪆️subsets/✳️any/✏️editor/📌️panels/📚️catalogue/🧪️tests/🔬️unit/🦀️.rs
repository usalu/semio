use super::*;
use crate::editor::en1992::testkit;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    assert_eq!(definition().body_key.as_deref(), Some(BODY_CATALOGUE));
    assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
}

#[semio_framework_async_macros::async_test]
async fn renders_this_standards_catalogue_headline() {
    let mut app = testkit::app_with_registry().await;
    assert!(testkit::render(&mut app, BODY_CATALOGUE).await.contains("catalogue"));
}
