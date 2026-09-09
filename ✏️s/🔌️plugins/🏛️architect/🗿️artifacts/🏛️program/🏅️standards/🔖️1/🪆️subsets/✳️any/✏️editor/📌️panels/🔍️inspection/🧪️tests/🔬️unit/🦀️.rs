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
    let cfg = ArchitectConfig::default();
    let json = crate::editor::architect::testkit::project_render(render(&program, &cfg));
    assert!(json.contains("architect-inspection.summary.schema"));
    assert!(json.contains(&program.elements.len().to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_summary_reflects_the_active_register() {
    let program = sample_plugin();
    let cfg = ArchitectConfig { active_register: "risks".into(), ..ArchitectConfig::default() };
    let json = crate::editor::architect::testkit::project_render(render(&program, &cfg));
    assert!(json.contains("\"value\":\"risks\""));
}
