
use crate::editor::lowpoly::testkit::render;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    use semio_framework_plugin::PanelTabDefinition;
    let definition: PanelTabDefinition = super::definition();
    assert_eq!(definition.id(), semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(super::LOWPOLY_PLAY_BODY_DOCUMENT));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_lists_active_object() {
    let mut a = crate::editor::lowpoly::testkit::app().await;
    assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_DOCUMENT).await.contains("lowpoly-document."));
}
