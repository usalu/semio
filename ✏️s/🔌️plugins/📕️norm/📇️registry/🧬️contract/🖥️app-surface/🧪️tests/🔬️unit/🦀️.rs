use crate::*;
use semio_framework_plugin::{FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

fn inspect(tree: &semio_framework_value::DslValue, locale: Locale, meta: Option<NormFieldMetaFn>, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    render_inspector_value(tree, &CheckReport::default(), locale, "norm.mock", meta, windows, &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert))
}

fn demo_check(section: impl Into<String>) -> semio_s_plugin_norm_compliance::CheckResult {
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};
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
    let spec = artifact_kind_spec("en1990", "EN 1990", "semio.norm.en1990/v1");
    assert_eq!(spec.id, "computation.norm.en1990");
    assert_eq!(spec.schema, "semio.norm.en1990/v1");
    assert_eq!(spec.source_format, "norm.en1990.document");
    assert_eq!(spec.dimension, "data");
    assert_eq!(spec.component_kind, "norm");
    assert_eq!(spec.media_type.class, MediaClass::Data);
    assert_eq!(spec.media_type.form, MediaForm::Value);
}

#[semio_framework_async_macros::async_test]
async fn render_checks_falls_back_to_a_placeholder_when_nothing_was_computed() {
    let json = report_body(&CheckReport::default(), Vec::new());
    assert!(json.contains("No checks computed."), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn render_checks_assigns_a_distinct_identity_to_each_check_row() {
    let json = report_body(&oversized_report(2), Vec::new());
    assert!(json.contains("\"norm-checks-check-0\""), "the first row keeps its own identity: {json}");
    assert!(json.contains("\"norm-checks-check-1\""), "the second row keeps its own identity: {json}");
}

#[semio_framework_async_macros::async_test]
async fn long_unicode_document_text_is_admitted_in_exact_utf8_chunks() {
    let source = format!("start-{}-end", "ä".repeat(ui::UI_TEXT_MAX_BYTES));
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_text_chunks(&source).expect("chunked node assembly"))).expect("chunked projection");
    let projected: serde_json::Value = serde_json::from_str(&json).expect("third-party projection oracle");
    let reassembled = projected["children"].as_array().expect("chunk children").iter().map(|child| child["component"]["value"].as_str().expect("text chunk")).collect::<String>();
    assert_eq!(reassembled, source);
}

#[semio_framework_async_macros::async_test]
async fn render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index() {
    let mut report = CheckReport::default();
    report.push(demo_check("1.1"));
    let inside = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_inspection(&report, Some(0), Locale::En, None).expect("node assembly"))).expect("json");
    let outside = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_inspection(&report, Some(99), Locale::En, None).expect("node assembly"))).expect("json");
    assert_eq!(inside, outside, "an out-of-range index must fall back to the first check");
    let empty = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_inspection(&CheckReport::default(), None, Locale::En, None).expect("node assembly"))).expect("json");
    assert!(empty.contains("No checks"));
    assert!(empty.contains("\"type\":\"tree\""), "norm inspection must be a tree: {empty}");
}

#[semio_framework_async_macros::async_test]
async fn byte_properties_render_their_bounded_localized_extent() {
    for (locale, extent) in [(Locale::En, "65537 bytes"), (Locale::De, "65537 Bytes")] {
        let tree = semio_framework_value::DslValue::object([("payload".into(), semio_framework_value::DslValue::Bytes(vec![255; 65_537]))]);
        let root = inspect(&tree, locale, None, &TreeWindows::unhosted()).expect("byte extent");
        let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(root)).expect("projection");
        let projected: serde_json::Value = serde_json::from_str(&json).expect("independent JSON oracle");
        assert!(projected.to_string().contains(extent));
        assert!(json.len() < 1024);
    }
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
    assert_eq!(selected_check_index_arg(Some(&semio_framework_value::DslValue::from(&serde_json::json!({ "index": 3 })))), Some(3));
    assert_eq!(selected_check_index_arg(Some(&semio_framework_value::DslValue::from(&serde_json::json!({})))), None);
    assert_eq!(selected_check_index_arg(None), None);
}

//#region 🪟️WindowLaws
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::TreeWindowRequest;
use semio_framework_plugin::ViewModel;

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
    let view = ViewModel { tree_windows: requests, tree_viewport_rows: Some(MEASURED_VIEWPORT_ROWS), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = render_checks(report, &TreeWindows::for_body(&view, "norm.x.play.results"), Locale::En, None).expect("report tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the report tree")
}

/// 🪟️ Law (a): a 300-check report renders — the section stamps its full extent and no `+N` appears.
#[test]
fn an_oversized_report_stamps_its_total_and_never_a_continuation_row() {
    let json = report_body(&oversized_report(300), Vec::new());
    assert!(json.contains("\"total\":301"), "the check section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("norm-checks-check-").count() <= MEASURED_VIEWPORT_ROWS as usize, "first paint materialises the measured viewport and stops: {json}");
}

/// 🪟️ Law (b): a section the host closed stamps its total and materialises nothing.
#[test]
fn a_closed_report_section_stamps_its_total_and_materialises_no_rows() {
    let json = report_body(&oversized_report(300), vec![TreeWindowRequest { body_key: "norm.x.play.results".into(), node_key: NORM_CHECKS_SECTION_ID.into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":301"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("norm-checks-check-"), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the row id.
#[test]
fn a_host_window_materialises_exactly_its_report_slice() {
    let json = report_body(&oversized_report(300), vec![TreeWindowRequest { body_key: "norm.x.play.results".into(), node_key: NORM_CHECKS_SECTION_ID.into(), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the section reports its offset: {json}");
    for check_index in 99..109 {
        assert!(json.contains(&format!("\"norm-checks-check-{check_index}\"")), "check {check_index} is inside the window: {json}");
    }
    assert!(!json.contains("\"norm-checks-check-98\""), "the check before the window stays out: {json}");
    assert!(!json.contains("\"norm-checks-check-109\""), "the check after the window stays out: {json}");
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
    let en = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_checks(&report, &TreeWindows::for_body(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), "norm.x.play.results"), Locale::En, None).expect("en"))).expect("json");
    let de = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_checks(&report, &TreeWindows::for_body(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), "norm.x.play.results"), Locale::De, None).expect("de"))).expect("json");
    assert!(en.contains("Pass") || en.contains("demo check"), "{en}");
    assert!(de.contains("Bestanden") || de.contains("Demo-Prüfung"), "{de}");
}

#[test]
fn path_set_insert_remove_round_trip() {
    let mut root = semio_framework_value::DslValue::object([
        ("layers".into(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::object([("thicknessM".into(), semio_framework_value::DslValue::float(0.08))])])),
        ("name".into(), semio_framework_value::DslValue::String("wall".into())),
    ]);
    set_value_at_path(&mut root, "name", semio_framework_value::DslValue::String("north".into())).unwrap();
    set_value_at_path(&mut root, "layers[0].thicknessM", semio_framework_value::DslValue::float(0.12)).unwrap();
    insert_value_at_path(&mut root, "layers", 1, Some(semio_framework_value::DslValue::object([("thicknessM".into(), semio_framework_value::DslValue::float(0.05))]))).unwrap();
    assert_eq!(root.get("layers").and_then(|v| if let semio_framework_value::DslValue::Array(items) = v { Some(items.len()) } else { None }), Some(2));
    remove_value_at_path(&mut root, "layers", 0).unwrap();
    assert_eq!(root.get("layers").and_then(|v| if let semio_framework_value::DslValue::Array(items) = v { Some(items.len()) } else { None }), Some(1));
    assert_eq!(root.get("name"), Some(&semio_framework_value::DslValue::String("north".into())));
}

#[test]
fn format_quantity_uses_natural_units() {
    assert!(format_quantity(&semio_s_plugin_norm_compliance::Quantity::force_kn(12.0)).contains("kN"));
    assert!(format_quantity(&semio_s_plugin_norm_compliance::Quantity::stress_mpa(30.0)).contains("MPa"));
    assert!(format_quantity(&semio_s_plugin_norm_compliance::Quantity::length_m(0.08)).contains("mm"));
    assert!(format_quantity(&semio_s_plugin_norm_compliance::Quantity::u_value_w_m2k(0.24)).contains("W/(m²K)"));
}

#[semio_framework_async_macros::async_test]
async fn failing_check_renders_remedy_rows() {
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
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
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
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
    let mut tree = semio_framework_value::DslValue::object([
        ("resistanceKn".into(), semio_framework_value::DslValue::float(doc.resistance_kn)),
        ("className".into(), semio_framework_value::DslValue::String(doc.class_name.clone())),
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
    assert!(report.checks[0].remedies.iter().any(|r| matches!(r.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf) && r.applicable));
    let one_of_index = report.checks[0].remedies.iter().position(|r| matches!(r.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf)).expect("one_of");
    let mut tree = semio_framework_value::DslValue::object([
        ("resistanceKn".into(), semio_framework_value::DslValue::float(doc.resistance_kn)),
        ("className".into(), semio_framework_value::DslValue::String(doc.class_name.clone())),
    ]);
    apply_remedy_edit(&report, "mock.uls", one_of_index, 0, &mut tree).expect("one_of");
    assert_eq!(tree.get("className"), Some(&semio_framework_value::DslValue::String("C30".into())));
}

#[semio_framework_async_macros::async_test]
async fn the_inspector_emits_set_field_and_list_verbs() {
    use semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    const BODY: &str = "norm.mock.inputs";
    let tree = semio_framework_value::DslValue::object([
        ("resistanceKn".into(), semio_framework_value::DslValue::float(10.0)),
        ("items".into(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(1.0), semio_framework_value::DslValue::float(2.0)])),
    ]);
    let items_key = format!("norm-inputs-root{TREE_WINDOW_PATH_SEPARATOR}{}", inputs_section_id("items"));
    let view = ViewModel {
        tree_windows: vec![
            TreeWindowRequest { body_key: BODY.into(), node_key: inputs_section_id("").into(), open: Some(true), offset: 0, rows: 32 },
            TreeWindowRequest { body_key: BODY.into(), node_key: items_key, open: Some(true), offset: 0, rows: 32 },
        ],
        tree_viewport_rows: Some(32),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(inspect(&tree, Locale::En, None, &TreeWindows::for_body(&view, BODY)).expect("editor")))
    .expect("json");
    assert!(json.contains("setField"), "{json}");
    assert!(json.contains("insertItem") || json.contains("removeItem"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn locale_switch_en_to_de_changes_chrome_and_check_copy() {
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, SubjectRef};
    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess("x", "demo", ClauseId::new("demo", "§1", "1"), SubjectRef::whole(LocalizedCopy::new("Subject", "Gegenstand")), LocalizedCopy::new("demo check", "Demo-Prüfung"))
            .utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .explanation(LocalizedCopy::new("ok", "in Ordnung"))
            .annex(AnnexChoice::De)
            .build(),
    );
    let en = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_checks(&report, &TreeWindows::unhosted(), Locale::En, None).expect("en")))
    .expect("en json");
    let de = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_checks(&report, &TreeWindows::unhosted(), Locale::De, None).expect("de")))
    .expect("de json");
    assert!(en.contains("Checks") || en.contains("Pass") || en.contains("demo check"), "{en}");
    assert!(de.contains("Nachweise") || de.contains("Bestanden") || de.contains("Demo-Prüfung"), "{de}");
    assert_ne!(en, de);
}

#[test]
fn field_meta_longest_prefix_and_wildcard_lookup() {
    const TABLE: &[(&str, FieldSpec)] = &[
        ("buildings", FieldSpec::new("Buildings", "Hochbauten")),
        ("buildings[].storeys[].massKg", FieldSpec::quantity("Storey mass", "Geschossmasse", UnitId::Kilogram)),
    ];
    let meta = lookup_norm_field_meta(TABLE, "buildings[0].storeys[2].massKg").expect("wildcard");
    assert_eq!(meta.label.en, "Storey mass");
    assert_eq!(meta.unit(), Some("kg"));
    let prefix = lookup_norm_field_meta(TABLE, "buildings[3].unknown").expect("prefix");
    assert_eq!(prefix.label.en, "Buildings");
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
    assert!(parse_path("members[id=]").unwrap_err().to_string().contains("empty"));
    assert!(parse_path("members[id=a.b]").unwrap_err().to_string().contains("must not contain"));
    assert!(parse_path("members[id=a=b]").unwrap_err().to_string().contains("must not contain"));
    assert!(parse_path("members[foo]").unwrap_err().to_string().contains("malformed"));
}

#[test]
fn set_value_at_path_resolves_id_selectors() {
    let mut root = semio_framework_value::DslValue::object([(
        "members".into(),
        semio_framework_value::DslValue::Array(vec![
            semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String("A".into())), ("nEd".into(), semio_framework_value::DslValue::float(1.0))]),
            semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String("B1".into())), ("nEd".into(), semio_framework_value::DslValue::float(2.0))]),
        ]),
    )]);
    set_value_at_path(&mut root, "members[id=B1].nEd", semio_framework_value::DslValue::float(9.0)).unwrap();
    assert_eq!(get_value_at_path(&root, "members[id=B1].nEd").unwrap(), &semio_framework_value::DslValue::float(9.0));
    assert_eq!(get_value_at_path(&root, "members[1].nEd").unwrap(), &semio_framework_value::DslValue::float(9.0));
    assert!(get_value_at_path(&root, "members[id=missing].nEd").unwrap_err().to_string().contains("unknown"));
    let mut dup = semio_framework_value::DslValue::object([(
        "members".into(),
        semio_framework_value::DslValue::Array(vec![
            semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String("X".into()))]),
            semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String("X".into()))]),
        ]),
    )]);
    assert!(set_value_at_path(&mut dup, "members[id=X].nEd", semio_framework_value::DslValue::float(1.0)).unwrap_err().to_string().contains("duplicate"));
}

#[test]
fn apply_remedy_with_id_path_target_flips_failing_check() {
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
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
    let mut tree = semio_framework_value::DslValue::object([(
        "members".into(),
        semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::object([
            ("id".into(), semio_framework_value::DslValue::String("B1".into())),
            ("resistanceKn".into(), semio_framework_value::DslValue::float(80_000.0)),
        ])]),
    )]);
    apply_remedy_edit(&report, "mock.id", 0, 0, &mut tree).expect("apply");
    assert_eq!(get_value_at_path(&tree, "members[id=B1].resistanceKn").unwrap(), &semio_framework_value::DslValue::float(demand));
    remove_value_at_path(&mut tree, "members[id=B1]", 0).unwrap();
    assert_eq!(tree.get("members").and_then(|v| if let semio_framework_value::DslValue::Array(items) = v { Some(items.len()) } else { None }), Some(0));
}

#[test]
fn field_meta_wildcard_matches_id_path() {
    const TABLE: &[(&str, FieldSpec)] = &[("members[].actions[].nEd", FieldSpec::quantity("N_Ed", "N_Ed", UnitId::Newton))];
    let meta = lookup_norm_field_meta(TABLE, "members[id=B1].actions[id=ULS-1].nEd").expect("id wildcard");
    assert_eq!(meta.label.en, "N_Ed");
    assert_eq!(meta.unit(), Some("N"));
}

#[test]
fn list_element_path_prefers_id_when_present() {
    let with_id = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String("B1".into())), ("nEd".into(), semio_framework_value::DslValue::float(1.0))]);
    let without = semio_framework_value::DslValue::object([("nEd".into(), semio_framework_value::DslValue::float(1.0))]);
    assert_eq!(list_element_path("members", &with_id, 3), "members[id=B1]");
    assert_eq!(list_element_path("members", &without, 3), "members[3]");
}

#[semio_framework_async_macros::async_test]
async fn de_locale_inputs_and_inspection_omit_hardcoded_english_chrome() {
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, SubjectRef};

    const ANNEX_CHOICES: &[NormFieldChoice] = &[
        NormFieldChoice { value: "en", label_en: "EN annex", label_de: "EN-Anhang" },
        NormFieldChoice { value: "de", label_en: "DE annex", label_de: "DE-Anhang" },
    ];
    fn field_meta(path: &str) -> Option<FieldSpec> {
        match path {
            "annex" => Some(FieldSpec::choice("National annex", "Nationaler Anhang", ANNEX_CHOICES)),
            "note" => Some(FieldSpec::new("Note", "Hinweis")),
            _ => None,
        }
    }

    let tree = semio_framework_value::DslValue::object([
        ("annex".into(), semio_framework_value::DslValue::String("de".into())),
        ("note".into(), semio_framework_value::DslValue::Null),
    ]);
    let inputs = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(inspect(&tree, Locale::De, Some(field_meta), &TreeWindows::unhosted()).expect("inputs")))
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
    let inspection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_inspection(&report, Some(0), Locale::De, None).expect("inspection")))
    .expect("inspection json");
    let unknown = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_unknown_body("norm.mock.missing", Locale::De).expect("unknown")))
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
        rows.push(semio_framework_value::DslValue::object([
            ("id".into(), semio_framework_value::DslValue::String(format!("row-{index}"))),
            ("massKg".into(), semio_framework_value::DslValue::float(index as f64)),
            ("nested".into(), semio_framework_value::DslValue::object([
                ("a".into(), semio_framework_value::DslValue::float(1.0)),
                ("b".into(), semio_framework_value::DslValue::float(2.0)),
            ])),
        ]));
    }
    let tree = semio_framework_value::DslValue::object([
        ("catalogue".into(), semio_framework_value::DslValue::object([
            ("products".into(), semio_framework_value::DslValue::Array(rows)),
        ])),
        ("dictionary".into(), semio_framework_value::DslValue::object([
            ("subjects".into(), semio_framework_value::DslValue::Array((0..80).map(|i| semio_framework_value::DslValue::object([
                ("id".into(), semio_framework_value::DslValue::String(format!("s-{i}"))),
                ("name".into(), semio_framework_value::DslValue::String("x".into())),
            ])).collect())),
        ])),
        ("geometry".into(), semio_framework_value::DslValue::object([
            ("objects".into(), semio_framework_value::DslValue::object((0..90).map(|i| (format!("g-{i}"), semio_framework_value::DslValue::object([
                ("kind".into(), semio_framework_value::DslValue::String("box".into())),
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
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let root = inspect(&tree, Locale::En, None, &TreeWindows::for_body(&view, BODY)).expect("assemble");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(root)).expect("retire within slots");
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
        map_entries.push((format!("g-{index}"), semio_framework_value::DslValue::object([("kind".into(), semio_framework_value::DslValue::String("box".into()))])));
    }
    let tree = semio_framework_value::DslValue::object([
        ("geometry".into(), semio_framework_value::DslValue::object([("objects".into(), semio_framework_value::DslValue::Object(map_entries))])),
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
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let root = inspect(&tree, Locale::En, None, &TreeWindows::for_body(&view, BODY)).expect("assemble");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(root)).expect("retire within slots");
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
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
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
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let open = project_catalogue(std::slice::from_ref(&table), Locale::En, &TreeWindows::for_body(&view, BODY));
    assert!(open.contains("C40") || open.contains("C41"), "windowed rows materialise: {open}");
    assert!(!open.contains("C250"), "rows outside the window stay out: {open}");
}
//#endregion 📚️CatalogueTables

#[test]
fn norm_fault_notices_cover_the_raised_codes_in_both_languages() {
    let notices = norm_fault_notices();
    for code in ["mutation.no-op", "mutation.clamped", "mutation.invariant", "mutation.duplicate-id", "norm.edit-unresolved", "norm.value-path", "norm.apply-remedy-missing-check", "norm.set-fields-args", "norm.insert-placement"] {
        assert!(notices.iter().any(|(known, _)| *known == code), "no notice for {code}");
    }
    for (code, label) in notices {
        let (en, de) = (label.resolve(semio_framework_ui_locale::Terminology::Native, Locale::En), label.resolve(semio_framework_ui_locale::Terminology::Native, Locale::De));
        assert!(!en.trim().is_empty() && !de.trim().is_empty() && en != de, "{code} lacks a distinct EN and DE notice");
        assert!(!semio_s_plugin_norm_compliance::looks_english(de), "{code} German notice reads as English: {de}");
    }
}

#[test]
fn an_absent_row_is_addressed_by_its_id_so_the_set_kind_can_create_it() {
    use semio_framework_value::DslValue;
    let obj = |entries: Vec<(&str, DslValue)>| DslValue::object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let case = obj(vec![("id", DslValue::String("C1".into())), ("numbers", DslValue::Array(vec![obj(vec![("id", DslValue::String("h".into())), ("value", DslValue::float(1.0))])]))]);
    let tree = obj(vec![("cases", DslValue::Array(vec![case]))]);
    let segments = parse_path("cases[id=C1].numbers[id=b].value").expect("path");
    let pending = address_pending(&tree, &segments).expect("pending row");
    assert_eq!(pending.template, "cases[].numbers[].value");
    assert_eq!(pending.selected, vec![(0, Some("C1".to_string())), (1, Some("b".to_string()))]);
    assert!(address_pending(&tree, &parse_path("cases[id=C1].numbers[id=h].value").expect("path")).is_none());
    assert!(address_pending(&tree, &parse_path("cases[id=missing].numbers[id=b].value").expect("path")).is_some());
    assert!(address_pending(&tree, &parse_path("cases[3].numbers[id=b].value").expect("path")).is_none());
}

#[test]
fn nested_rows_are_set_inserted_and_removed_through_parent_selectors() {
    use semio_framework_value::DslValue;
    let obj = |entries: Vec<(&str, DslValue)>| DslValue::object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let action = |id: &str| obj(vec![("id", DslValue::String(id.into())), ("nK", DslValue::float(1.0))]);
    let member = |id: &str, actions: Vec<DslValue>| obj(vec![("id", DslValue::String(id.into())), ("actions", DslValue::Array(actions))]);
    let tree = obj(vec![("members", DslValue::Array(vec![member("M1", vec![action("A1")]), member("M2", vec![action("A1"), action("A2")])]))]);
    const RULES: NormEditRules = NormEditRules {
        move_item: &[],
        set_field: &[SetFieldRule { path: "members[].actions[].nK", kind: "change-action-n-k", selectors: &[SelectorKey::Id("memberId"), SelectorKey::Id("actionId")], value: "newNK" }],
        insert_item: &[InsertItemRule { path: "members[].actions", kind: "insert-member-action", selectors: &[SelectorKey::Id("memberId")], index: "index", item: "action" }],
        remove_item: &[],
    };
    const REMOVALS: &[IdRemoval] = &[IdRemoval { path: "members[].actions", kind: "remove-member-action", selectors: &[SelectorKey::Id("memberId")], id: "id" }];
    let entry = |entries: &[(String, DslValue)], key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
    let (kind, entries) = RULES.payload(REMOVALS, &tree, &NormEdit::SetField { path: "members[id=M2].actions[id=A1].nK".into(), value: DslValue::float(5.0) }).expect("set");
    assert_eq!((kind, entry(&entries, "memberId"), entry(&entries, "actionId")), ("change-action-n-k", Some(DslValue::String("M2".into())), Some(DslValue::String("A1".into()))));
    let (kind, entries) = RULES.payload(REMOVALS, &tree, &NormEdit::InsertItem { path: "members[id=M1].actions".into(), index: 1, value: Some(action("A9")) }).expect("insert");
    assert_eq!((kind, entry(&entries, "memberId"), entry(&entries, "index")), ("insert-member-action", Some(DslValue::String("M1".into())), Some(DslValue::uint(1))));
    for path in ["members[id=M2].actions[id=A2]", "members[1].actions[1]"] {
        let (kind, entries) = RULES.payload(REMOVALS, &tree, &NormEdit::RemoveItem { path: path.into(), index: 0 }).expect("remove");
        assert_eq!((kind, entry(&entries, "memberId"), entry(&entries, "id")), ("remove-member-action", Some(DslValue::String("M2".into())), Some(DslValue::String("A2".into()))));
    }
    let (_, entries) = RULES.payload(REMOVALS, &tree, &NormEdit::RemoveItem { path: "members[id=M2].actions".into(), index: 0 }).expect("remove by position");
    assert_eq!(entry(&entries, "id"), Some(DslValue::String("A1".into())));
    assert!(RULES.payload(REMOVALS, &tree, &NormEdit::RemoveItem { path: "members[id=M3].actions[id=A1]".into(), index: 0 }).is_err());
}

#[test]
fn a_reorder_names_its_row_by_position_and_raises_the_order_kind_with_parent_selectors() {
    use semio_framework_value::DslValue;
    let obj = |entries: Vec<(&str, DslValue)>| DslValue::object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let row = |id: &str| obj(vec![("id", DslValue::String(id.into()))]);
    let tree = obj(vec![("members", DslValue::Array(vec![row("a"), row("b"), row("c")])), ("groups", DslValue::Array(vec![obj(vec![("id", DslValue::String("g1".into())), ("items", DslValue::Array(vec![row("x"), row("y")]))])]))]);
    const RULES: NormEditRules = NormEditRules {
        move_item: &[
            MoveItemRule { path: "members", kind: "change-member-order", selectors: &[], from: "fromIndex", to: "toIndex" },
            MoveItemRule { path: "groups[].items", kind: "change-group-item-order", selectors: &[SelectorKey::Id("groupId")], from: "fromIndex", to: "toIndex" },
        ],
        set_field: &[],
        insert_item: &[],
        remove_item: &[],
    };
    let edit = move_item_edit(&tree, "members", "a", 2).expect("a drag to the end");
    assert_eq!(edit, NormEdit::MoveItem { path: "members".into(), from: 0, to: 2 });
    let (kind, entries) = RULES.payload(&[], &tree, &edit).expect("payload");
    assert_eq!((kind, entries), ("change-member-order", vec![("fromIndex".to_string(), DslValue::uint(0)), ("toIndex".to_string(), DslValue::uint(2))]));
    let (kind, entries) = RULES.payload(&[], &tree, &NormEdit::MoveItem { path: "groups[id=g1].items".into(), from: 1, to: 0 }).expect("nested payload");
    assert_eq!((kind, entries.len(), entries[0].1.clone()), ("change-group-item-order", 3, DslValue::String("g1".into())));
    assert_eq!(move_item_edit(&tree, "members", "a", 99), Ok(NormEdit::MoveItem { path: "members".into(), from: 0, to: 2 }), "the target is clamped");
    assert!(move_item_edit(&tree, "members", "c", 2).is_err(), "an unchanged position commits nothing");
    assert!(move_item_edit(&tree, "members", "ghost", 0).is_err());
    assert!(RULES.payload(&[], &tree, &NormEdit::MoveItem { path: "groups".into(), from: 0, to: 0 }).is_err(), "a list without a rule is refused");
    let mut reordered = tree.clone();
    crate::entity_editing::apply_to_tree(&mut reordered, &[edit]).expect("replay");
    let ids: Vec<_> = get_value_at_path(&reordered, "members").unwrap().as_array().unwrap().iter().map(|r| object_id_field(r).unwrap().to_string()).collect();
    assert_eq!(ids, ["b", "c", "a"]);
}

#[test]
fn a_null_write_raises_the_clear_kind_of_a_leaf_that_declares_one() {
    use semio_framework_value::DslValue;
    let obj = |entries: Vec<(&str, DslValue)>| DslValue::object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let tree = obj(vec![("members", DslValue::Array(vec![obj(vec![("id", DslValue::String("M1".into())), ("tightness", DslValue::String("leaky".into()))])]))]);
    const RULES: NormEditRules = NormEditRules {
        move_item: &[],
        set_field: &[
            SetFieldRule { path: "members[].tightness", kind: "change-member-tightness", selectors: &[SelectorKey::Id("memberId")], value: "newTightness" },
            SetFieldRule { path: "members[].tightness", kind: "clear-member-tightness", selectors: &[SelectorKey::Id("memberId")], value: "" },
        ],
        insert_item: &[],
        remove_item: &[],
    };
    let (kind, entries) = RULES.payload(&[], &tree, &NormEdit::SetField { path: "members[id=M1].tightness".into(), value: DslValue::Null }).expect("clear");
    assert_eq!((kind, entries), ("clear-member-tightness", vec![("memberId".to_string(), DslValue::String("M1".into()))]));
    let (kind, entries) = RULES.payload(&[], &tree, &NormEdit::SetField { path: "members[id=M1].tightness".into(), value: DslValue::String("tight".into()) }).expect("set");
    assert_eq!((kind, entries.len()), ("change-member-tightness", 2));
}

#[test]
fn a_remove_remedy_deletes_the_row_its_target_selects_and_is_no_value_write() {
    use semio_framework_value::DslValue;
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Remedy, RemedyBound, SubjectRef};
    let subject = SubjectRef::new("o1", "factorOverrides[id=o1]", LocalizedCopy::new("Override o1", "Überschreibung o1"));
    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess("mock.override", "demo", ClauseId::new("demo", "§1", "1"), subject.clone(), LocalizedCopy::new("Deviation", "Abweichung"))
            .explanation(LocalizedCopy::new("reset", "zurücksetzen"))
            .remedy(Remedy::remove(subject, LocalizedCopy::new("Reset to NA", "Auf NA zurücksetzen")))
            .annex(AnnexChoice::De)
            .build(),
    );
    assert_eq!(report.checks[0].remedies[0].bound, RemedyBound::Remove);
    let row = |id: &str| DslValue::object([("id".into(), DslValue::String(id.into())), ("value".into(), DslValue::float(1.5))]);
    let mut tree = DslValue::object([("factorOverrides".into(), DslValue::Array(vec![row("o0"), row("o1")]))]);
    assert_eq!(remedy_resolution(&report, "mock.override", 0, 0, &tree).unwrap(), RemedyEdit::Remove { path: "factorOverrides[id=o1]".into() });
    assert!(remedy_edit(&report, "mock.override", 0, 0, &tree).is_err());
    apply_remedy_edit(&report, "mock.override", 0, 0, &mut tree).expect("remove");
    assert!(get_value_at_path(&tree, "factorOverrides[id=o1]").is_err());
    assert!(get_value_at_path(&tree, "factorOverrides[id=o0]").is_ok());
}

#[test]
fn an_insert_remedy_appends_its_typed_row_at_the_computed_position_through_the_insert_rule() {
    use semio_framework_value::DslValue;
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, RemedyBound, SubjectRef};
    let subject = SubjectRef::new("g1", "stiffeners", LocalizedCopy::new("Stiffeners of girder g1", "Steifen des Trägers g1"));
    let mut report = CheckReport::default();
    report.push(
        CheckResult::assess("mock.stiffener", "demo", ClauseId::new("demo", "§1", "1"), subject.clone(), LocalizedCopy::new("Web buckling", "Stegbeulen"))
            .explanation(LocalizedCopy::new("add a stiffener", "Steife ergänzen"))
            .remedy(Remedy::insert(subject, r#"{"id":"s-mid","girderId":"g1","xM":"$required"}"#, Quantity::length_m(3.0), LocalizedCopy::new("Add a transverse stiffener at 3.0 m", "Quersteife bei 3,0 m ergänzen")))
            .annex(AnnexChoice::De)
            .build(),
    );
    assert_eq!(report.checks[0].remedies[0].bound, RemedyBound::Insert);
    let row = |id: &str| DslValue::object([("id".into(), DslValue::String(id.into())), ("girderId".into(), DslValue::String("g1".into())), ("xM".into(), DslValue::float(1.0))]);
    let mut tree = DslValue::object([("stiffeners".into(), DslValue::Array(vec![row("s-end")]))]);
    let expected = row("s-mid");
    let RemedyEdit::Insert { path, index, value } = remedy_resolution(&report, "mock.stiffener", 0, 0, &tree).expect("resolves") else { panic!("an insert") };
    assert_eq!((path.as_str(), index, value.get("xM").and_then(DslValue::as_f64)), ("stiffeners", 1, Some(3.0)));
    assert_eq!(value.get("id"), expected.get("id"));
    assert!(remedy_edit(&report, "mock.stiffener", 0, 0, &tree).is_err(), "an insert is no value write");
    apply_remedy_edit(&report, "mock.stiffener", 0, 0, &mut tree).expect("insert");
    assert_eq!(get_value_at_path(&tree, "stiffeners[id=s-mid].xM").ok().and_then(DslValue::as_f64), Some(3.0));
    assert!(apply_remedy_edit(&report, "mock.stiffener", 0, 0, &mut tree).is_err(), "a second application never duplicates the row");
    const RULES: NormEditRules = NormEditRules {
        move_item: &[],
        set_field: &[],
        insert_item: &[InsertItemRule { path: "stiffeners", kind: "insert-stiffener", selectors: &[], index: "index", item: "stiffener" }],
        remove_item: &[],
    };
    let edit = NormEdit::InsertItem { path: path.clone(), index, value: Some(value) };
    let (kind, entries) = RULES.payload(&[], &DslValue::object([("stiffeners".into(), DslValue::Array(vec![row("s-end")]))]), &edit).expect("the insert rule carries the remedy row");
    assert_eq!((kind, entries.len()), ("insert-stiffener", 2));
}

mod marker_cells {
    use super::*;
    use semio_s_plugin_norm_structural_scene::{Affine, Axis, AxisScale, DataTable, Diagram2d, FieldBinding, Interval, LocalizedCopy, Marker, MarkerShape, QuantityKind, Quantise, TickRule, Tone, Vec2};

    fn diagram() -> Diagram2d {
        let axis = |unit: &str, domain: Option<Interval>| Axis { label: LocalizedCopy::new("Position", "Lage"), quantity: QuantityKind::Length, unit: unit.into(), scale: AxisScale::Linear, domain, ticks: TickRule::Auto };
        let mut binding = FieldBinding::new("placements[id=p1].x", Affine { base: 0.0, k: 1.0 }, "m");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        let marker = |id: &str, label: (&str, &str), binding: Option<FieldBinding>| Marker { id: id.into(), at: Vec2::new(2.5, 14.0), shape: MarkerShape::Diamond, tone: Tone::Warning, label: LocalizedCopy::new(label.0, label.1), utilisation: Some(0.93), bound_check: Some("c1".into()), binding };
        Diagram2d {
            id: "plan".into(),
            title: LocalizedCopy::new("Plan", "Grundriss"),
            description: LocalizedCopy::new("Plan", "Grundriss"),
            axes: vec![axis("m", Some(Interval::new(0.0, 10.0))), axis("mm", None)],
            series: Vec::new(),
            markers: vec![marker("p1", ("Placement P1", "Platzierung P1"), Some(binding)), marker("p2", ("Placement P2", "Platzierung P2"), None)],
            annotations: Vec::new(),
            focus: None,
            legend: false,
            table_twin: DataTable { columns: Vec::new() },
        }
    }

    fn body(controller_id: Option<&'static str>, locale: Locale) -> String {
        let view = ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native);
        let spec = DiagramsWindowSpec { controller_id, ..DiagramsWindowSpec::default() };
        let node = render_diagrams("norm.x.play.diagrams", &[diagram()], &spec, &TreeWindows::for_body(&view, "norm.x.play.diagrams"), locale).expect("diagrams body");
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the diagrams body")
    }

    #[test]
    fn a_bound_marker_row_carries_an_inline_editor_cell_named_in_both_languages_and_an_unbound_one_stays_read_only() {
        let cell = "norm.x.play.diagrams.plan.marker.m0.input";
        let english = body(Some("s.norm.x@1/*#editor"), Locale::En);
        assert!(english.contains(cell), "{english}");
        assert!(!english.contains("norm.x.play.diagrams.plan.marker.m1.input"), "an unbound marker has no editor: {english}");
        assert!(english.contains("Placement P1, edit value"), "{english}");
        assert!(english.contains("marker:plan:p1") && english.contains("setFields"), "the cell commits the marker grip through setFields: {english}");
        let german = body(Some("s.norm.x@1/*#editor"), Locale::De);
        assert!(german.contains(cell) && german.contains("Platzierung P1, Wert bearbeiten"), "{german}");
        assert!(german.contains("2.5 m"), "the cell shows the bound value in the unit of the field: {german}");
        assert_eq!(marker_cell_id("norm.x.play.diagrams.plan", 0), cell);
    }

    #[test]
    fn without_a_controller_every_marker_row_is_read_only() {
        assert!(!body(None, Locale::En).contains(".marker.m0.input"));
    }
}
