use super::*;
use crate::editor::en1996::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    assert_eq!(definition().body_key.as_deref(), Some(BODY_INSPECTION));
    assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert!(matches!(definition().group, PanelGroup::Details));
}

/// 👁️ The config-driven pointer: an out-of-range index falls back to the first check, so both
/// renders agree for a document whose report has fewer rows than the index.
#[semio_framework_async_macros::async_test]
async fn an_out_of_range_selected_index_falls_back_to_the_first_check() {
    let host = NormHost::<En1996Family>::from_document(crate::En1996Snapshot::default());
    let first = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render(&host, None).expect("node assembly") }).expect("json");
    let clamped = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render(&host, Some(9_999)).expect("node assembly") }).expect("json");
    assert_eq!(first, clamped);
}

#[semio_framework_async_macros::async_test]
async fn renders_a_single_check() {
    let mut app = context::app_with_registry().await;
    assert!(!context::render(&mut app, BODY_INSPECTION).await.contains("Unknown body"));
}
