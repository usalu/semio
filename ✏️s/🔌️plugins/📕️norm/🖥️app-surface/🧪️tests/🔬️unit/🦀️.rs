use super::*;
use semio_framework_plugin::{FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};


fn demo_check(section: impl Into<String>) -> crate::document::CheckResult {
    use crate::document::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};
    let section = section.into();
    CheckResult::assess(
        format!("demo.{section}"),
        "demo",
        ClauseId::new("demo", "§1", section),
        SubjectRef::whole(LocalizedCopy::new("Subject", "Gegenstand")),
        LocalizedCopy::new("demo check", "Demo-Prüfung"),
    )
    .utilization(
        Quantity::new(QuantityKind::Dimensionless, 0.5),
        Quantity::new(QuantityKind::Dimensionless, 1.0),
    )
    .annex(AnnexChoice::De)
    .explanation(LocalizedCopy::new("ok", "ok"))
    .build()
}

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
    report.push(demo_check("1.1"));
    let inside = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_inspection(&report, Some(0), Locale::En, None).expect("node assembly") }).expect("json");
    let outside = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_inspection(&report, Some(99), Locale::En, None).expect("node assembly") }).expect("json");
    assert_eq!(inside, outside, "an out-of-range index must fall back to the first check");
    let empty = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render_inspection(&CheckReport::default(), None, Locale::En, None).expect("node assembly") }).expect("json");
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
    report.push(demo_check("1.1"));
    let columns = report_table_columns(Locale::En);
    let rows = report_table_rows(&report, Locale::En);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), columns.len());
    assert!(rows[0][5].contains("demo check"));
    let rows_de = report_table_rows(&report, Locale::De);
    assert!(rows_de[0][5].contains("Demo-Prüfung"));
}

#[semio_framework_async_macros::async_test]
async fn selected_check_index_arg_reads_the_shell_wire_shape() {
    assert_eq!(selected_check_index_arg(Some(&dsl::DslValue::from(&serde_json::json!({ "index": 3 })))), Some(3));
    assert_eq!(selected_check_index_arg(Some(&dsl::DslValue::from(&serde_json::json!({})))), None);
    assert_eq!(selected_check_index_arg(None), None);
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{Locale, TreeWindowRequest, ViewModel};

/// 🪟 The viewport the host reports for these laws. Deliberately small: a first paint is priced in
/// real `UiValue` argument-arena credit, shared process-wide, so a law that materialised a full
/// 48-row viewport would starve the sibling panel tests running beside it.
const MEASURED_VIEWPORT_ROWS: u32 = 4;

/// 🪟️ A compliance run with far more clause checks than the 32 siblings a plain column admitted.
fn oversized_report(checks: usize) -> CheckReport {
    let mut report = CheckReport::default();
    for index in 0..checks {
        report.push(demo_check(format!("1.{index}")));
    }
    report
}

/// 🪟️ The results body exactly as the host reads it, for the host-known windows in `requests`.
fn report_body(report: &CheckReport, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, tree_viewport_rows: Some(MEASURED_VIEWPORT_ROWS), ..Default::default() };
    let node = render_report(report, &TreeWindows::for_body(&view, "norm.x.play.results"), Locale::En, None).expect("report tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the report tree")
}

/// 🪟️ Law (a): a 300-check report renders — the section stamps its full extent and no `+N` appears.
#[test]
fn an_oversized_report_stamps_its_total_and_never_a_continuation_row() {
    let json = report_body(&oversized_report(300), Vec::new());
    assert!(json.contains("\"total\":301"), "the check section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("norm-report-check-").count() <= MEASURED_VIEWPORT_ROWS as usize, "first paint materialises the measured viewport and stops: {json}");
}

/// 🪟️ Law (b): a section the host closed stamps its total and materialises nothing.
#[test]
fn a_closed_report_section_stamps_its_total_and_materialises_no_rows() {
    let json = report_body(&oversized_report(300), vec![TreeWindowRequest { body_key: "norm.x.play.results".into(), node_key: NORM_REPORT_SECTION_ID.into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":301"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("norm-report-check-"), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the row id.
#[test]
fn a_host_window_materialises_exactly_its_report_slice() {
    // Flat list is [part-header, check-0, ...]; offset counts flat rows so offset 100 → check_index 99.
    let json = report_body(&oversized_report(300), vec![TreeWindowRequest { body_key: "norm.x.play.results".into(), node_key: NORM_REPORT_SECTION_ID.into(), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the section reports its offset: {json}");
    for check_index in 99..109 {
        assert!(json.contains(&format!("\"norm-report-check-{check_index}\"")), "check {check_index} is inside the window: {json}");
    }
    assert!(!json.contains("\"norm-report-check-98\""), "the check before the window stays out: {json}");
    assert!(!json.contains("\"norm-report-check-109\""), "the check after the window stays out: {json}");
}

/// 🪟️ Law (d) for a read-only report: no interaction domain, no pick granularity, no row binding.
#[test]
fn the_report_tree_binds_no_interaction_domain() {
    let json = report_body(&oversized_report(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "the report tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "the report tree stamps no pick granularity: {json}");
    assert!(!json.contains("\"bindings\":[{"), "a report row carries no action of its own: {json}");
}

#[semio_framework_async_macros::async_test]
async fn report_and_summary_localize_en_and_de() {
    let mut report = CheckReport::default();
    report.push(demo_check("1.1"));
    let en = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_report(&report, &TreeWindows::for_body(&ViewModel::default(), "norm.x.play.results"), Locale::En, None).expect("en"),
    }).expect("json");
    let de = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_report(&report, &TreeWindows::for_body(&ViewModel::default(), "norm.x.play.results"), Locale::De, None).expect("de"),
    }).expect("json");
    assert!(en.contains("Pass") || en.contains("demo check"), "{en}");
    assert!(de.contains("Bestanden") || de.contains("Demo-Prüfung"), "{de}");
}

#[test]
fn path_set_insert_remove_round_trip() {
    let mut root = dsl::DslValue::object([
        ("layers".into(), dsl::DslValue::Array(vec![dsl::DslValue::object([("thicknessM".into(), dsl::DslValue::float(0.08))])])),
        ("name".into(), dsl::DslValue::String("wall".into())),
    ]);
    set_value_at_path(&mut root, "name", dsl::DslValue::String("north".into())).unwrap();
    set_value_at_path(&mut root, "layers[0].thicknessM", dsl::DslValue::float(0.12)).unwrap();
    insert_value_at_path(&mut root, "layers", 1, Some(dsl::DslValue::object([("thicknessM".into(), dsl::DslValue::float(0.05))]))).unwrap();
    assert_eq!(root.get("layers").and_then(|v| if let dsl::DslValue::Array(items) = v { Some(items.len()) } else { None }), Some(2));
    remove_value_at_path(&mut root, "layers", 0).unwrap();
    assert_eq!(root.get("layers").and_then(|v| if let dsl::DslValue::Array(items) = v { Some(items.len()) } else { None }), Some(1));
    assert_eq!(root.get("name"), Some(&dsl::DslValue::String("north".into())));
}

#[test]
fn format_quantity_uses_natural_units() {
    assert!(format_quantity(&crate::document::Quantity::force_kn(12.0)).contains("kN"));
    assert!(format_quantity(&crate::document::Quantity::stress_mpa(30.0)).contains("MPa"));
    assert!(format_quantity(&crate::document::Quantity::length_m(0.08)).contains("mm"));
    assert!(format_quantity(&crate::document::Quantity::u_value_w_m2k(0.24)).contains("W/(m²K)"));
}

#[semio_framework_async_macros::async_test]
async fn failing_check_renders_remedy_rows() {
    use crate::document::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
    let subject = SubjectRef::new("m1", "resistanceKn", LocalizedCopy::new("Member", "Bauteil"));
    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess("x.fail", "demo", ClauseId::new("demo", "§1", "1"), subject.clone(), LocalizedCopy::new("ULS", "GZT"))
            .utilization(Quantity::force_kn(120.0), Quantity::force_kn(100.0))
            .explanation(LocalizedCopy::new("too high", "zu hoch"))
            .remedy(Remedy::at_least(subject, Quantity::force_kn(100.0), Quantity::force_kn(120.0), LocalizedCopy::new("Raise R", "R erhöhen")))
            .annex(AnnexChoice::De)
            .build(),
    );
    let json = report_body(&report, Vec::new());
    assert!(json.contains("Raise R") || json.contains("remedy"), "{json}");
    assert!(json.contains("Explanation") || json.contains("too high"), "{json}");
    assert!(json.contains("Member") || json.contains("Bauteil") || json.contains("resistanceKn"), "subject in row: {json}");
}


#[derive(Clone, Debug, Default, PartialEq)]
struct MockDoc {
    resistance_kn: f64,
    class_name: String,
}

fn mock_evaluate(document: &MockDoc) -> CheckReport {
    use crate::document::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
    let demand = 150_000.0;
    let mut report = CheckReport::default();
    let subject = SubjectRef::new("m1", "resistanceKn", LocalizedCopy::new("Member", "Bauteil"));
    let mut builder = CheckResult::assess("mock.uls", "demo", ClauseId::new("demo", "§1", "1"), subject.clone(), LocalizedCopy::new("ULS capacity", "GZT-Tragfähigkeit"))
        .utilization(Quantity::new(QuantityKind::Force, demand), Quantity::new(QuantityKind::Force, document.resistance_kn))
        .explanation(LocalizedCopy::new("raise resistance", "Widerstand erhöhen"))
        .annex(AnnexChoice::De);
    if document.resistance_kn + 1e-9 < demand {
        builder = builder.remedy(Remedy::at_least(subject.clone(), Quantity::new(QuantityKind::Force, document.resistance_kn), Quantity::new(QuantityKind::Force, demand), LocalizedCopy::new("Raise R", "R erhöhen")));
    }
    if document.class_name != "C30" {
        builder = builder.remedy(Remedy::one_of(SubjectRef::new("m1", "className", LocalizedCopy::new("Class", "Klasse")), vec!["C30".into(), "C40".into()], LocalizedCopy::new("Pick class", "Klasse wählen")));
    }
    report.push(builder.build());
    report
}

#[test]
fn dispatch_apply_remedy_at_least_flips_failing_check_to_pass() {
    let doc = MockDoc { resistance_kn: 80_000.0, class_name: "C30".into() };
    let before = mock_evaluate(&doc);
    assert_eq!(before.checks[0].status, CheckStatus::Fail);
    let mut tree = dsl::DslValue::object([
        ("resistanceKn".into(), dsl::DslValue::float(doc.resistance_kn)),
        ("className".into(), dsl::DslValue::String(doc.class_name.clone())),
    ]);
    apply_remedy_edit(&before, "mock.uls", 0, 0, &mut tree).expect("apply");
    let resistance = tree.get("resistanceKn").and_then(|v| v.as_f64()).expect("resistance");
    let after_doc = MockDoc { resistance_kn: resistance, class_name: doc.class_name.clone() };
    let after = mock_evaluate(&after_doc);
    assert_eq!(after.checks[0].status, CheckStatus::Pass, "{after:?}");
}

#[test]
fn one_of_remedy_application_writes_selected_option() {
    let doc = MockDoc { resistance_kn: 120_000.0, class_name: "C20".into() };
    let report = mock_evaluate(&doc);
    assert!(report.checks[0].remedies.iter().any(|r| matches!(r.bound, crate::document::RemedyBound::OneOf) && r.applicable));
    let one_of_index = report.checks[0].remedies.iter().position(|r| matches!(r.bound, crate::document::RemedyBound::OneOf)).expect("one_of");
    let mut tree = dsl::DslValue::object([
        ("resistanceKn".into(), dsl::DslValue::float(doc.resistance_kn)),
        ("className".into(), dsl::DslValue::String(doc.class_name.clone())),
    ]);
    apply_remedy_edit(&report, "mock.uls", one_of_index, 0, &mut tree).expect("one_of");
    assert_eq!(tree.get("className"), Some(&dsl::DslValue::String("C30".into())));
}

#[semio_framework_async_macros::async_test]
async fn render_document_editor_emits_set_field_and_list_verbs() {
    use semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    const BODY: &str = "norm.mock.inputs";
    let tree = dsl::DslValue::object([
        ("resistanceKn".into(), dsl::DslValue::float(10.0)),
        ("items".into(), dsl::DslValue::Array(vec![dsl::DslValue::float(1.0), dsl::DslValue::float(2.0)])),
    ]);
    let items_key = format!("norm-inputs-root{TREE_WINDOW_PATH_SEPARATOR}{}", inputs_section_id("items"));
    let view = ViewModel {
        tree_windows: vec![
            TreeWindowRequest { body_key: BODY.into(), node_key: inputs_section_id("").into(), open: Some(true), offset: 0, rows: 32 },
            TreeWindowRequest { body_key: BODY.into(), node_key: items_key, open: Some(true), offset: 0, rows: 32 },
        ],
        tree_viewport_rows: Some(32),
        ..Default::default()
    };
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_value_editor("", &tree, Locale::En, "norm.mock", None, &TreeWindows::for_body(&view, BODY), 0).expect("editor"),
    })
    .expect("json");
    assert!(json.contains("setField"), "{json}");
    assert!(json.contains("insertItem") || json.contains("removeItem"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn locale_switch_en_to_de_changes_chrome_and_check_copy() {
    use crate::document::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, SubjectRef};
    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess("x", "demo", ClauseId::new("demo", "§1", "1"), SubjectRef::whole(LocalizedCopy::new("Subject", "Gegenstand")), LocalizedCopy::new("demo check", "Demo-Prüfung"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .explanation(LocalizedCopy::new("ok", "in Ordnung"))
            .annex(AnnexChoice::De)
            .build(),
    );
    let en = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_report(&report, &TreeWindows::unhosted(), Locale::En, None).expect("en"),
    })
    .expect("en json");
    let de = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_report(&report, &TreeWindows::unhosted(), Locale::De, None).expect("de"),
    })
    .expect("de json");
    assert!(en.contains("Checks") || en.contains("Pass") || en.contains("demo check"), "{en}");
    assert!(de.contains("Nachweise") || de.contains("Bestanden") || de.contains("Demo-Prüfung"), "{de}");
    assert_ne!(en, de);
}

#[test]
fn evaluate_job_progress_and_cancellation() {
    let mut state = NormEvaluateWorkState::new();
    let mut buf = [0u8; 4];
    assert_eq!(state.checkpoint(&mut buf).expect("cp"), 1);
    assert_eq!(buf[0], 0);
    assert_eq!(state.emit_progress_stage(), Some("norm-evaluate-prepare"));
    assert_eq!(state.emit_progress_stage(), Some("norm-evaluate-run"));
    assert!(state.ready_to_evaluate());
    state.begin_close();
    assert!(state.cancelled);
    state.restore(&[1]).expect("restore");
    assert_eq!(state.phase, 1);
    assert!(!state.cancelled);
}

#[test]
fn field_meta_longest_prefix_and_wildcard_lookup() {
    const TABLE: &[(&str, NormFieldMeta)] = &[
        ("buildings", NormFieldMeta { label_en: "Buildings", label_de: "Hochbauten", unit: None, choices: None }),
        ("buildings[].storeys[].massKg", NormFieldMeta { label_en: "Storey mass", label_de: "Geschossmasse", unit: Some("kg"), choices: None }),
    ];
    let meta = lookup_norm_field_meta(TABLE, "buildings[0].storeys[2].massKg").expect("wildcard");
    assert_eq!(meta.label_en, "Storey mass");
    assert_eq!(meta.unit, Some("kg"));
    let prefix = lookup_norm_field_meta(TABLE, "buildings[3].unknown").expect("prefix");
    assert_eq!(prefix.label_en, "Buildings");
}

#[test]
fn parse_path_accepts_index_and_id_selectors() {
    assert_eq!(
        parse_path("members[0].actions[1].nEd").unwrap(),
        vec![
            PathSegment::Field("members".into()),
            PathSegment::Index(0),
            PathSegment::Field("actions".into()),
            PathSegment::Index(1),
            PathSegment::Field("nEd".into()),
        ]
    );
    assert_eq!(
        parse_path("members[id=B1].actions[id=ULS-1].nEd").unwrap(),
        vec![
            PathSegment::Field("members".into()),
            PathSegment::Id("B1".into()),
            PathSegment::Field("actions".into()),
            PathSegment::Id("ULS-1".into()),
            PathSegment::Field("nEd".into()),
        ]
    );
    assert!(parse_path("members[id=]").unwrap_err().contains("empty"));
    assert!(parse_path("members[id=a.b]").unwrap_err().contains("must not contain"));
    assert!(parse_path("members[id=a=b]").unwrap_err().contains("must not contain"));
    assert!(parse_path("members[foo]").unwrap_err().contains("malformed"));
}

#[test]
fn set_value_at_path_resolves_id_selectors() {
    let mut root = dsl::DslValue::object([(
        "members".into(),
        dsl::DslValue::Array(vec![
            dsl::DslValue::object([("id".into(), dsl::DslValue::String("A".into())), ("nEd".into(), dsl::DslValue::float(1.0))]),
            dsl::DslValue::object([("id".into(), dsl::DslValue::String("B1".into())), ("nEd".into(), dsl::DslValue::float(2.0))]),
        ]),
    )]);
    set_value_at_path(&mut root, "members[id=B1].nEd", dsl::DslValue::float(9.0)).unwrap();
    assert_eq!(get_value_at_path(&root, "members[id=B1].nEd").unwrap(), &dsl::DslValue::float(9.0));
    assert_eq!(get_value_at_path(&root, "members[1].nEd").unwrap(), &dsl::DslValue::float(9.0));
    assert!(get_value_at_path(&root, "members[id=missing].nEd").unwrap_err().contains("unknown"));
    let mut dup = dsl::DslValue::object([(
        "members".into(),
        dsl::DslValue::Array(vec![
            dsl::DslValue::object([("id".into(), dsl::DslValue::String("X".into()))]),
            dsl::DslValue::object([("id".into(), dsl::DslValue::String("X".into()))]),
        ]),
    )]);
    assert!(set_value_at_path(&mut dup, "members[id=X].nEd", dsl::DslValue::float(1.0)).unwrap_err().contains("duplicate"));
}

#[test]
fn apply_remedy_with_id_path_target_flips_failing_check() {
    use crate::document::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
    let demand = 150_000.0;
    let mut report = CheckReport::default();
    let subject = SubjectRef::new("B1", "members[id=B1].resistanceKn", LocalizedCopy::new("Member B1", "Bauteil B1"));
    report.push(
        CheckResult::assess("mock.id", "demo", ClauseId::new("demo", "§1", "1"), subject.clone(), LocalizedCopy::new("ULS", "GZT"))
            .utilization(Quantity::new(QuantityKind::Force, demand), Quantity::new(QuantityKind::Force, 80_000.0))
            .explanation(LocalizedCopy::new("raise", "erhöhen"))
            .remedy(Remedy::at_least(subject, Quantity::new(QuantityKind::Force, 80_000.0), Quantity::new(QuantityKind::Force, demand), LocalizedCopy::new("Raise R", "R erhöhen")))
            .annex(AnnexChoice::De)
            .build(),
    );
    assert_eq!(report.checks[0].status, CheckStatus::Fail);
    let mut tree = dsl::DslValue::object([(
        "members".into(),
        dsl::DslValue::Array(vec![dsl::DslValue::object([
            ("id".into(), dsl::DslValue::String("B1".into())),
            ("resistanceKn".into(), dsl::DslValue::float(80_000.0)),
        ])]),
    )]);
    apply_remedy_edit(&report, "mock.id", 0, 0, &mut tree).expect("apply");
    assert_eq!(get_value_at_path(&tree, "members[id=B1].resistanceKn").unwrap(), &dsl::DslValue::float(demand));
    remove_value_at_path(&mut tree, "members[id=B1]", 0).unwrap();
    assert_eq!(tree.get("members").and_then(|v| if let dsl::DslValue::Array(items) = v { Some(items.len()) } else { None }), Some(0));
}

#[test]
fn field_meta_wildcard_matches_id_path() {
    const TABLE: &[(&str, NormFieldMeta)] = &[(
        "members[].actions[].nEd",
        NormFieldMeta { label_en: "N_Ed", label_de: "N_Ed", unit: Some("N"), choices: None },
    )];
    let meta = lookup_norm_field_meta(TABLE, "members[id=B1].actions[id=ULS-1].nEd").expect("id wildcard");
    assert_eq!(meta.label_en, "N_Ed");
    assert_eq!(meta.unit, Some("N"));
}

#[test]
fn list_element_path_prefers_id_when_present() {
    let with_id = dsl::DslValue::object([("id".into(), dsl::DslValue::String("B1".into())), ("nEd".into(), dsl::DslValue::float(1.0))]);
    let without = dsl::DslValue::object([("nEd".into(), dsl::DslValue::float(1.0))]);
    assert_eq!(list_element_path("members", &with_id, 3), "members[id=B1]");
    assert_eq!(list_element_path("members", &without, 3), "members[3]");
}

#[semio_framework_async_macros::async_test]
async fn de_locale_inputs_and_inspection_omit_hardcoded_english_chrome() {
    use crate::document::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, SubjectRef};

    const ANNEX_CHOICES: &[NormFieldChoice] = &[
        NormFieldChoice { value: "en", label_en: "EN annex", label_de: "EN-Anhang" },
        NormFieldChoice { value: "de", label_en: "DE annex", label_de: "DE-Anhang" },
    ];
    fn field_meta(path: &str) -> Option<NormFieldMeta> {
        match path {
            "annex" => Some(NormFieldMeta {
                label_en: "National annex",
                label_de: "Nationaler Anhang",
                unit: None,
                choices: Some(ANNEX_CHOICES),
            }),
            "note" => Some(NormFieldMeta {
                label_en: "Note",
                label_de: "Hinweis",
                unit: None,
                choices: None,
            }),
            _ => None,
        }
    }

    let tree = dsl::DslValue::object([
        ("annex".into(), dsl::DslValue::String("de".into())),
        ("note".into(), dsl::DslValue::Null),
    ]);
    let inputs = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_value_editor("", &tree, Locale::De, "norm.mock", Some(field_meta), &TreeWindows::unhosted(), 0).expect("inputs"),
    })
    .expect("inputs json");

    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess(
            "check-1",
            "demo",
            ClauseId::new("demo", "§1", "1"),
            SubjectRef::whole(LocalizedCopy::new("Subject", "Gegenstand")),
            LocalizedCopy::new("demo check", "Demo-Prüfung"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .explanation(LocalizedCopy::new("ok", "in Ordnung"))
        .annex(AnnexChoice::De)
        .build(),
    );
    let inspection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_inspection(&report, Some(0), Locale::De, None).expect("inspection"),
    })
    .expect("inspection json");
    let unknown = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render_unknown_body("norm.mock.missing", Locale::De).expect("unknown"),
    })
    .expect("unknown json");
    let io = norm_io("mock", "mock.schema");

    assert!(inputs.contains("leer"), "{inputs}");
    assert!(inputs.contains("DE-Anhang"), "{inputs}");
    assert!(inspection.contains("Kennung"), "{inspection}");
    assert!(unknown.contains("Unbekannter Inhalt"), "{unknown}");
    assert!(io.ports.iter().any(|port| port.id == "model:in" && port.label.contains("Modell")), "{io:?}");
    assert!(io.ports.iter().any(|port| port.id == "report:out" && port.label.contains("Bericht")), "{io:?}");

    let combined = format!("{inputs}\n{inspection}\n{unknown}");
    for english in ["Unknown body:", "Id", "null", "Model", "Report"] {
        // Node ids may embed english tokens (e.g. `.id`); only reject chrome payloads / port labels.
        match english {
            "Id" => assert!(inspection.contains("Kennung") && !inspection.contains("\"Id\""), "{inspection}"),
            "null" => assert!(inputs.contains("leer") && !inputs.contains("\"null\""), "{inputs}"),
            "Model" | "Report" => {}
            _ => assert!(!combined.contains(english), "unexpected English chrome {english:?} in {combined}"),
        }
    }
    assert!(!io.ports.iter().any(|port| port.label == "Model" || port.label == "Report"), "{io:?}");
}


#[test]
fn collapsed_deep_document_stays_within_retire_slots() {
    const BODY: &str = "norm.mock.inputs";
    let mut rows = Vec::new();
    for index in 0..120 {
        rows.push(dsl::DslValue::object([
            ("id".into(), dsl::DslValue::String(format!("row-{index}"))),
            ("massKg".into(), dsl::DslValue::float(index as f64)),
            ("nested".into(), dsl::DslValue::object([
                ("a".into(), dsl::DslValue::float(1.0)),
                ("b".into(), dsl::DslValue::float(2.0)),
            ])),
        ]));
    }
    let tree = dsl::DslValue::object([
        ("catalogue".into(), dsl::DslValue::object([
            ("products".into(), dsl::DslValue::Array(rows)),
        ])),
        ("dictionary".into(), dsl::DslValue::object([
            ("subjects".into(), dsl::DslValue::Array((0..80).map(|i| dsl::DslValue::object([
                ("id".into(), dsl::DslValue::String(format!("s-{i}"))),
                ("name".into(), dsl::DslValue::String("x".into())),
            ])).collect())),
        ])),
        ("geometry".into(), dsl::DslValue::object([
            ("objects".into(), dsl::DslValue::object((0..90).map(|i| (format!("g-{i}"), dsl::DslValue::object([
                ("kind".into(), dsl::DslValue::String("box".into())),
            ]))).collect::<Vec<_>>())),
        ])),
    ]);
    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest {
            body_key: BODY.into(),
            node_key: inputs_section_id("").into(),
            open: Some(true),
            offset: 0,
            rows: 16,
        }],
        tree_viewport_rows: Some(16),
        ..Default::default()
    };
    let root = render_value_editor("", &tree, Locale::En, "norm.mock", None, &TreeWindows::for_body(&view, BODY), 0).expect("assemble");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root }).expect("retire within slots");
    assert!(json.contains("norm-inputs-catalogue"), "{json}");
    assert!(json.contains("norm-inputs-dictionary"), "{json}");
    assert!(json.contains("norm-inputs-geometry"), "{json}");
    assert!(json.contains("catalogue (1)") && json.contains("dictionary (1)") && json.contains("geometry (1)"), "{json}");
    assert!(!json.contains("massKg") && !json.contains("row-50"), "collapsed products must not materialise deep leaves: {json}");
}

#[test]
fn expanding_subtree_materialises_windowed_children_within_slots() {
    use semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    const BODY: &str = "norm.mock.inputs";
    let mut map_entries = Vec::new();
    for index in 0..80 {
        map_entries.push((format!("g-{index}"), dsl::DslValue::object([("kind".into(), dsl::DslValue::String("box".into()))])));
    }
    let tree = dsl::DslValue::object([
        ("geometry".into(), dsl::DslValue::object([("objects".into(), dsl::DslValue::Object(map_entries))])),
    ]);
    let root_id = inputs_section_id("");
    let geometry_id = inputs_section_id("geometry");
    let objects_id = inputs_section_id("geometry.objects");
    let geometry_path = format!("{root_id}{TREE_WINDOW_PATH_SEPARATOR}{geometry_id}");
    let objects_path = format!("{geometry_path}{TREE_WINDOW_PATH_SEPARATOR}{objects_id}");
    let view = ViewModel {
        tree_windows: vec![
            TreeWindowRequest { body_key: BODY.into(), node_key: root_id, open: Some(true), offset: 0, rows: 8 },
            TreeWindowRequest { body_key: BODY.into(), node_key: geometry_path, open: Some(true), offset: 0, rows: 8 },
            TreeWindowRequest { body_key: BODY.into(), node_key: objects_path, open: Some(true), offset: 10, rows: 8 },
        ],
        tree_viewport_rows: Some(8),
        ..Default::default()
    };
    let root = render_value_editor("", &tree, Locale::En, "norm.mock", None, &TreeWindows::for_body(&view, BODY), 0).expect("assemble");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root }).expect("retire within slots");
    assert!(json.contains("\"offset\":10") || json.contains("\"total\":80"), "{json}");
    assert!(json.contains("g-10") || json.contains("g-11"), "windowed map rows materialise: {json}");
    assert!(!json.contains("g-50"), "rows outside the window stay out: {json}");
}

//#endregion 🪟WindowLaws

//#region 📚️CatalogueTables
fn sample_catalogue_table(rows: usize) -> CatalogueTable {
    CatalogueTable {
        id: "demo-table",
        title_en: "Demo strength classes",
        title_de: "Demo-Festigkeitsklassen",
        clause: ClauseId::new("EN DEMO", "1-1", "3.1"),
        columns: vec![
            CatalogueColumn { id: "class", label_en: "Class", label_de: "Klasse", unit: None },
            CatalogueColumn { id: "fck", label_en: "f_ck", label_de: "f_ck", unit: Some("MPa") },
        ],
        rows: (0..rows)
            .map(|index| CatalogueRow {
                id: format!("c{index}"),
                cells: vec![CatalogueCell::text(format!("C{index}")), CatalogueCell::number(index as f64, 0)],
            })
            .collect(),
    }
}

fn project_catalogue(tables: &[CatalogueTable], locale: Locale, windows: &TreeWindows<'_>) -> String {
    let node = render_catalogue(&[], tables, locale, "norm.catalogue", windows).expect("catalogue");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project")
}

#[test]
fn catalogue_empty_tables_list_renders_examples_only() {
    let json = project_catalogue(&[], Locale::En, &TreeWindows::unhosted());
    assert!(json.contains("Examples") || json.contains("norm-catalogue.examples"), "{json}");
    assert!(!json.contains("norm-catalogue.table-"), "empty tables must not invent sections: {json}");
}

#[test]
fn catalogue_table_headers_and_clause_localize_en_and_de() {
    let table = sample_catalogue_table(3);
    let en = project_catalogue(std::slice::from_ref(&table), Locale::En, &TreeWindows::unhosted());
    let de = project_catalogue(std::slice::from_ref(&table), Locale::De, &TreeWindows::unhosted());
    assert!(en.contains("Demo strength classes"), "{en}");
    assert!(en.contains("EN DEMO") || en.contains("3.1"), "clause label missing: {en}");
    assert!(de.contains("Demo-Festigkeitsklassen"), "{de}");
    assert!(de.contains("EN DEMO") || de.contains("3.1"), "clause label missing: {de}");
    // Collapsed by default — open to assert column headers.
    const BODY: &str = "norm.mock.catalogue";
    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest {
            body_key: BODY.into(),
            node_key: format!("norm-catalogue.table-{}", table.id),
            open: Some(true),
            offset: 0,
            rows: 8,
        }],
        tree_viewport_rows: Some(8),
        ..Default::default()
    };
    let en_open = project_catalogue(std::slice::from_ref(&table), Locale::En, &TreeWindows::for_body(&view, BODY));
    let de_open = project_catalogue(std::slice::from_ref(&table), Locale::De, &TreeWindows::for_body(&view, BODY));
    assert!(en_open.contains("Class") && en_open.contains("f_ck [MPa]"), "{en_open}");
    assert!(de_open.contains("Klasse") && de_open.contains("f_ck [MPa]"), "{de_open}");
}

#[test]
fn catalogue_large_table_stays_within_retire_slots_when_windowed() {
    const BODY: &str = "norm.mock.catalogue";
    let table = sample_catalogue_table(500);
    let collapsed = project_catalogue(std::slice::from_ref(&table), Locale::En, &TreeWindows::unhosted());
    assert!(collapsed.contains("Demo strength classes") || collapsed.contains("norm-catalogue.table-demo-table"), "{collapsed}");
    assert!(!collapsed.contains("C250"), "collapsed table must not materialise deep rows: {collapsed}");
    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest {
            body_key: BODY.into(),
            node_key: format!("norm-catalogue.table-{}", table.id),
            open: Some(true),
            offset: 40,
            rows: 8,
        }],
        tree_viewport_rows: Some(8),
        ..Default::default()
    };
    let open = project_catalogue(std::slice::from_ref(&table), Locale::En, &TreeWindows::for_body(&view, BODY));
    assert!(open.contains("C40") || open.contains("C41"), "windowed rows materialise: {open}");
    assert!(!open.contains("C250"), "rows outside the window stay out: {open}");
}
//#endregion 📚️CatalogueTables

