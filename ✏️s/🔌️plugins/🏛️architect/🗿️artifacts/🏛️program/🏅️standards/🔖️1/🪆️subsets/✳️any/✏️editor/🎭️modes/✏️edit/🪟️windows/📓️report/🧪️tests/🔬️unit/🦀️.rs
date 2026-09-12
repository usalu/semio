use super::*;
use crate::editor::architect::catalog::report_record_from;
use crate::registers::ReportKind;
use crate::sample_plugin;
use crate::standards::v1::subsets::any::schema::inferences::build_report;
use semio_framework_plugin::{locale_from_str, ViewModel};

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_definition_declares_the_text_editor_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_REPORT);
    assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
}

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_selected_authored_record_renders_localized_metadata() {
    let mut program = sample_plugin();
    let report = build_report(&program, ReportKind::ExecutiveSummary);
    let record = report_record_from(&program, ReportKind::ExecutiveSummary, &report);
    let selected_report_id = record.header.id.clone();
    program.reports.push(record);
    let cfg = config::ArchitectReportWindowConfig { selected_report_id: Some(selected_report_id) };
    let json = crate::editor::architect::unit_tests::context::project_render(render(&program, &cfg, &ViewModel::default()));
    assert!(json.contains("Overview"));
    assert!(json.contains("architect-report.section"));
    let de = ViewModel { locale: locale_from_str("de"), ..Default::default() };
    let json = crate::editor::architect::unit_tests::context::project_render(render(&program, &cfg, &de));
    assert!(json.contains("Art:"));
    assert!(json.contains("Erstellt:"));
    assert!(json.contains("Version:"));
}

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_empty_selection_renders_a_localized_prompt() {
    let json = crate::editor::architect::unit_tests::context::project_render(render(&sample_plugin(), &config::ArchitectReportWindowConfig::default(), &ViewModel::default()));
    assert!(json.contains("Generate a report in this window"));
    let de = ViewModel { locale: locale_from_str("de"), ..Default::default() };
    let json = crate::editor::architect::unit_tests::context::project_render(render(&sample_plugin(), &config::ArchitectReportWindowConfig::default(), &de));
    assert!(json.contains("Erstellen Sie in diesem Fenster einen Bericht"));
}

#[semio_framework_async_macros::async_test]
async fn architect_window_ownership_report_deleted_selection_renders_a_localized_missing_state() {
    let cfg = config::ArchitectReportWindowConfig { selected_report_id: Some(crate::EntityId("missing-report".into())) };
    let de = ViewModel { locale: locale_from_str("de"), ..Default::default() };
    let json = crate::editor::architect::unit_tests::context::project_render(render(&sample_plugin(), &cfg, &de));
    assert!(json.contains("ist nicht verfügbar"));
}
