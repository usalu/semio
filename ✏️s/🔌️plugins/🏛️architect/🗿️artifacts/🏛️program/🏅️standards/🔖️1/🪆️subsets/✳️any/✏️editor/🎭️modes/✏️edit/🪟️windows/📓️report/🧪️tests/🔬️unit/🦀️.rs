use super::*;
use crate::registers::ReportKind;
use crate::sample_plugin;
use crate::standards::v1::subsets::any::schema::inferences::build_report;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_text_editor_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_REPORT);
    assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
}

#[semio_framework_async_macros::async_test]
async fn a_report_in_the_config_renders_its_section_headings() {
    let report = build_report(&sample_plugin(), ReportKind::ExecutiveSummary);
    let cfg = ArchitectConfig { active_report_json: serde_json::to_string(&report).expect("json"), ..ArchitectConfig::default() };
    let json = crate::editor::architect::testkit::project_render(render(&cfg));
    assert!(json.contains("Overview"));
    assert!(json.contains("architect-report.section"));
}

#[semio_framework_async_macros::async_test]
async fn no_report_renders_the_placeholder() {
    let json = crate::editor::architect::testkit::project_render(render(&ArchitectConfig::default()));
    assert!(json.contains("Run validation, analysis, or report"));
}
