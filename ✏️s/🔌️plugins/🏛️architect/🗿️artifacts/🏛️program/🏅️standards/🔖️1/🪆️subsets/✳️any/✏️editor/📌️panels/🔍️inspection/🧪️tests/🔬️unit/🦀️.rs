use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn the_tab_is_the_framework_inspection_tab_bound_to_this_apps_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_INSPECTION));
    assert!(matches!(definition.group, PanelGroup::Details));
}

#[semio_framework_async_macros::async_test]
async fn the_inspector_always_summarises_the_document_wide_register_counts() {
    let program = sample_plugin();
    let json = crate::editor::architect::unit_tests::context::project_render(render(&program));
    assert!(json.contains("architect-inspection.summary.schema"));
    assert!(json.contains(&program.elements.len().to_string()));
}
