use super::*;
use crate::editor::layout::unit_tests::context::{layout_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_inspector_always_summarises_the_document() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_INSPECTION).await;
    assert!(json.contains(LAYOUT_DOCUMENT_SCHEMA));
    assert!(json.contains("page-1"));
    assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree: {json}");
}

#[test]
fn selected_frame_page_and_document_expose_edit_inputs() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let config = LayoutWindowConfig { active_page_id: "page-1".into(), ..LayoutWindowConfig::default() };
    let interaction = LayoutInteractionSnapshot { ids: vec!["frame-1".into()], ..Default::default() };
    let labels = crate::editor::layout::terminology::layout_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let node = render(&snapshot, &config, &interaction, labels).expect("inspector");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project inspector");
    assert!(json.contains("patchFrame"), "{json}");
    assert!(json.contains("patchPage"), "{json}");
    assert!(json.contains("patchDocument"), "{json}");
    assert!(json.contains("layout-play-inspector.patchFrame.x"), "{json}");
    assert!(json.contains("layout-play-inspector.patchDocument.baselineGrid"), "{json}");
    assert!(json.contains("layout-play-inspector.patchDocument.paragraph.body.fontSize"), "{json}");
    assert!(json.contains("layout-play-inspector.patchFrame.locked"), "{json}");
    assert!(json.contains("layout-play-inspector.patchPage.delete"), "{json}");
    assert!(json.contains("layout-play-inspector.patchPage.layer-1.name"), "{json}");
    assert!(json.contains("layout-play-inspector.patchDocument.addCharacterStyle"), "{json}");
    assert!(json.contains("layout-play-inspector.patchDocument.spread-1.name"), "{json}");
    assert!(json.contains("layout-play-inspector.patchPage.parentPageId"), "{json}");
    assert!(json.contains("\"type\":\"input\""), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_INSPECTION));
}
