use super::*;
use semio_framework_plugin::{FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

#[semio_framework_async_macros::async_test]
async fn the_edit_mode_is_the_same_for_every_app() {
    let mode = edit_mode_definition();
    assert_eq!(mode.id, MODE_EDIT);
    assert!(mode.tools.is_empty() && mode.commands.is_empty() && mode.layout_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn a_window_definition_is_a_plain_canvas2d_surface() {
    let window = window_definition("norm-x-inputs", LocalizedLabel::native("Inputs", "Eingaben"), "norm.x.play.inputs", "download");
    assert_eq!(window.body_key, "norm.x.play.inputs");
    assert!(matches!(window.surface_kind, SurfaceKind::Canvas2d));
    assert!(window.actions.is_empty() && window.utilities.is_empty() && window.options.measures.is_empty());
}

/// 📌️ Proves `panel_definition` reproduces the scalar `AppBuilder::panel_tab` shape exactly (an
/// `App`-kind leaf carrying the body key) — the property that keeps the manifest byte-identical
/// after the panel declarations moved into `📌️panels/*` nodes.
#[semio_framework_async_macros::async_test]
async fn a_panel_definition_is_an_app_kind_leaf_carrying_its_body_key() {
    let panel = panel_definition(FRAMEWORK_PANEL_TAB_ARTIFACT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"), PanelGroup::Workbench, "norm.x.play.artifact");
    assert!(matches!(&panel.kind, PanelTabKind::App(id) if id == FRAMEWORK_PANEL_TAB_ARTIFACT_ID));
    assert_eq!(panel.body_key.as_deref(), Some("norm.x.play.artifact"));
    assert!(panel.children.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn norm_io_declares_model_in_and_report_out_beside_the_implicit_document_ports() {
    let io = norm_io("din4108", "semio.norm.din4108/v1");
    assert!(io.ports.iter().any(|port| port.id == "model:in" && port.direction == MediaPortDirection::In));
    let report_out = io.ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
    assert_eq!(report_out.direction, MediaPortDirection::Out);
    assert_eq!(report_out.kind_id.as_deref(), Some("computation.norm.din4108"));
    assert_eq!(io.artifact.id, "computation.norm.din4108");
}

#[semio_framework_async_macros::async_test]
async fn the_artifact_kind_spec_is_a_data_value_document() {
    let spec = artifact_kind_spec("en1990", "EN 1990");
    assert_eq!(spec.id, "computation.norm.en1990");
    assert_eq!(spec.source_format, "norm.en1990.document");
    assert_eq!(spec.dimension, "data");
    assert_eq!(spec.component_kind, "norm");
    assert_eq!(spec.media_type.class, MediaClass::Data);
    assert_eq!(spec.media_type.form, MediaForm::Value);
}

#[semio_framework_async_macros::async_test]
async fn render_report_falls_back_to_a_placeholder_when_nothing_was_computed() {
    let json = report_body(&CheckReport::default(), Vec::new());
    assert!(json.contains("No checks computed."), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn render_report_assigns_a_distinct_identity_to_each_check_row() {
    let json = report_body(&oversized_report(2), Vec::new());
    assert!(json.contains("\"norm-report-check-0\""), "the first row keeps its own identity: {json}");
    assert!(json.contains("\"norm-report-check-1\""), "the second row keeps its own identity: {json}");
}

#[semio_framework_async_macros::async_test]
async fn long_unicode_document_text_is_admitted_in_exact_utf8_chunks() {
    let source = format!("start-{}-end", "ä".repeat(ui::UI_TEXT_MAX_BYTES));
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_text_chunks(&source).expect("chunked node assembly") }).expect("chunked projection");
    let projected: serde_json::Value = serde_json::from_str(&json).expect("third-party projection oracle");
    let reassembled = projected["children"].as_array().expect("chunk children").iter().map(|child| child["component"]["value"].as_str().expect("text chunk")).collect::<String>();
    assert_eq!(reassembled, source);
}

#[semio_framework_async_macros::async_test]
async fn render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index() {
    let mut report = CheckReport::default();
    report.push(crate::document::CheckResult::from_utilization(
        crate::document::ClauseId::new("demo", "§1", "1.1"),
        crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.5),
        crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
        "demo check",
        crate::document::AnnexChoice::De,
    ));
    let inside = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_inspection(&report, Some(0)).expect("node assembly") }).expect("json");
    let outside = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_inspection(&report, Some(99)).expect("node assembly") }).expect("json");
    assert_eq!(inside, outside, "an out-of-range index must fall back to the first check");
    let empty = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_inspection(&CheckReport::default(), None).expect("node assembly") }).expect("json");
    assert!(empty.contains("No checks"));
    assert!(empty.contains("\"type\":\"tree\""), "norm inspection must be a tree: {empty}");
}

#[semio_framework_async_macros::async_test]
async fn the_view_mode_is_the_same_for_every_viewer() {
    let mode = view_mode_definition();
    assert_eq!(mode.id, MODE_VIEW);
    assert!(mode.tools.is_empty() && mode.commands.is_empty() && mode.layout_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn single_window_layout_stacks_exactly_one_window() {
    let layout = single_window_layout("framework.window.table", "Report");
    let WindowLayoutRoot::Stack(stack) = layout.root else { panic!("expected a stack root") };
    assert_eq!(stack.children.len(), 1);
    assert_eq!(stack.children[0].window_kind_id, "framework.window.table");
}

#[semio_framework_async_macros::async_test]
async fn report_table_columns_and_rows_line_up_with_the_check_report() {
    let mut report = CheckReport::default();
    report.push(crate::document::CheckResult::from_utilization(
        crate::document::ClauseId::new("demo", "§1", "1.1"),
        crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.5),
        crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
        "demo check",
        crate::document::AnnexChoice::De,
    ));
    let columns = report_table_columns();
    let rows = report_table_rows(&report);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), columns.len());
    assert!(rows[0][3].contains("demo check"));
}

#[semio_framework_async_macros::async_test]
async fn selected_check_index_arg_reads_the_shell_wire_shape() {
    assert_eq!(selected_check_index_arg(Some(&dsl::DslValue::from(&serde_json::json!({ "index": 3 })))), Some(3));
    assert_eq!(selected_check_index_arg(Some(&dsl::DslValue::from(&serde_json::json!({})))), None);
    assert_eq!(selected_check_index_arg(None), None);
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

/// 🪟 The viewport the host reports for these laws. Deliberately small: a first paint is priced in
/// real `UiValue` argument-arena credit, shared process-wide, so a law that materialised a full
/// 48-row viewport would starve the sibling panel tests running beside it.
const MEASURED_VIEWPORT_ROWS: u32 = 4;

/// 🪟️ A compliance run with far more clause checks than the 32 siblings a plain column admitted.
fn oversized_report(checks: usize) -> CheckReport {
    let mut report = CheckReport::default();
    for index in 0..checks {
        report.push(crate::document::CheckResult::from_utilization(
            crate::document::ClauseId::new("demo", "§1", format!("1.{index}")),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.5),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            "demo check",
            crate::document::AnnexChoice::De,
        ));
    }
    report
}

/// 🪟️ The results body exactly as the host reads it, for the host-known windows in `requests`.
fn report_body(report: &CheckReport, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, tree_viewport_rows: Some(MEASURED_VIEWPORT_ROWS), ..Default::default() };
    let node = render_report(report, &TreeWindows::for_body(&view, "norm.x.play.results")).expect("report tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the report tree")
}

/// 🪟️ Law (a): a 300-check report renders — the section stamps its full extent and no `+N` appears.
#[test]
fn an_oversized_report_stamps_its_total_and_never_a_continuation_row() {
    let json = report_body(&oversized_report(300), Vec::new());
    assert!(json.contains("\"total\":300"), "the check section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("norm-report-check-").count() <= MEASURED_VIEWPORT_ROWS as usize, "first paint materialises the measured viewport and stops: {json}");
}

/// 🪟️ Law (b): a section the host closed stamps its total and materialises nothing.
#[test]
fn a_closed_report_section_stamps_its_total_and_materialises_no_rows() {
    let json = report_body(&oversized_report(300), vec![TreeWindowRequest { body_key: "norm.x.play.results".into(), node_key: NORM_REPORT_SECTION_ID.into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":300"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("norm-report-check-"), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the row id.
#[test]
fn a_host_window_materialises_exactly_its_report_slice() {
    let json = report_body(&oversized_report(300), vec![TreeWindowRequest { body_key: "norm.x.play.results".into(), node_key: NORM_REPORT_SECTION_ID.into(), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the section reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("\"norm-report-check-{index}\"")), "check {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"norm-report-check-99\""), "the check before the window stays out: {json}");
    assert!(!json.contains("\"norm-report-check-110\""), "the check after the window stays out: {json}");
}

/// 🪟️ Law (d) for a read-only report: no interaction domain, no pick granularity, no row binding.
#[test]
fn the_report_tree_binds_no_interaction_domain() {
    let json = report_body(&oversized_report(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "the report tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "the report tree stamps no pick granularity: {json}");
    assert!(!json.contains("\"bindings\":[{"), "a report row carries no action of its own: {json}");
}
//#endregion 🪟️WindowLaws
