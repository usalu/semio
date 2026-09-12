use super::*;

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
    let panel = panel_definition("document", LocalizedLabel::native("Document", "Dokument"), PanelGroup::Workbench, "norm.x.play.document");
    assert!(matches!(&panel.kind, PanelTabKind::App(id) if id == "document"));
    assert_eq!(panel.body_key.as_deref(), Some("norm.x.play.document"));
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
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_report(&CheckReport::default()).expect("node assembly") }).expect("json");
    assert!(json.contains("No checks computed."), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn render_report_assigns_a_distinct_identity_to_each_check_row() {
    let mut report = CheckReport::default();
    for _ in 0..2 {
        report.push(crate::document::CheckResult::from_utilization(
            crate::document::ClauseId::new("demo", "§1", "1.1"),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.5),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            "same check",
            crate::document::AnnexChoice::De,
        ));
    }
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_report(&report).expect("node assembly") }).expect("distinct row identities");
    let projected: serde_json::Value = serde_json::from_str(&json).expect("third-party projection oracle");
    let rows = projected["children"].as_array().expect("report rows");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["key"], "norm-report-check-0");
    assert_eq!(rows[1]["key"], "norm-report-check-1");
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
    assert!(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_inspection(&CheckReport::default(), None).expect("node assembly") }).expect("json").contains("No checks"));
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
