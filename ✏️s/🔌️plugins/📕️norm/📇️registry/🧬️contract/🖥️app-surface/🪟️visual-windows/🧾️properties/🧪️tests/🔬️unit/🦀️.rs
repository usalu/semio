use semio_plugin::{TreeWindowRequest, ViewModel};
use crate::*;
use semio_framework_value::DslValue;
use super::shown_in;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

const GOLDEN: &str = include_str!("../🔮️oracles/🔣️.json");

fn project(node: BuiltNode) -> String {
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(node)).expect("projection")
}

fn number(value: f64) -> DslValue {
    DslValue::float(value)
}

fn text(value: &str) -> DslValue {
    DslValue::String(value.into())
}

const KINDS: &[NormFieldChoice] = &[NormFieldChoice { value: "beam", label_en: "Beam", label_de: "Träger" }, NormFieldChoice { value: "column", label_en: "Column", label_de: "Stütze" }];
const BENDING: ClauseRef = ClauseRef { family: "EN 1993-1-1", part: "6", section: "6.2.5" };

fn member_meta(path: &str) -> Option<FieldSpec> {
    match path {
        path if path == "members" || path.starts_with("members[") && path.ends_with(']') => Some(FieldSpec { read_only: true, can_insert: path == "members", can_remove: true, ..FieldSpec::new("Members", "Bauteile") }),
        "members[].length" => Some(FieldSpec { range: Some(FieldRange::new(0.001, 0.5)), soft_range: Some(FieldRange::new(0.001, 0.1)), step: Some(0.001), help: Some(FieldLabel::new("Span between the supports", "Stützweite zwischen den Auflagern")), clause: Some(BENDING), ..FieldSpec::quantity("Length", "Länge", UnitId::Metre) }),
        "members[].kind" => Some(FieldSpec::choice("Kind", "Art", KINDS)),
        "members[].count" => Some(FieldSpec { kind: FieldKind::Int, ..FieldSpec::new("Count", "Anzahl") }),
        "members[].active" => Some(FieldSpec { kind: FieldKind::Bool, ..FieldSpec::new("Active", "Aktiv") }),
        "members[].force" => Some(FieldSpec { ndp: Some(NdpRef { id: None, symbol: "γ_M0", clause: BENDING, default: Some(100_000.0) }), ..FieldSpec::quantity("Force", "Kraft", UnitId::Newton) }),
        "members[].angle" => Some(FieldSpec { kind: FieldKind::Angle, ..FieldSpec::new("Angle", "Winkel") }),
        "members[].ratio" => Some(FieldSpec { kind: FieldKind::Percent, ..FieldSpec::new("Ratio", "Anteil") }),
        "members[].sectionId" => Some(FieldSpec { kind: FieldKind::EntityRef { to: "sections" }, ..FieldSpec::new("Section", "Querschnitt") }),
        "members[].advanced" => Some(FieldSpec { tier: Tier::Advanced, ..FieldSpec::new("Advanced", "Erweitert") }),
        "members[].columnWidth" => Some(FieldSpec { visible_if: Some(Predicate { path: "kind", op: PredicateOp::Equals, value: "column" }), ..FieldSpec::quantity("Column width", "Stützenbreite", UnitId::Metre) }),
        _ => None,
    }
}

fn document() -> DslValue {
    DslValue::object([
        ("annex".into(), text("De")),
        ("sections".into(), DslValue::Array(vec![DslValue::object([("id".into(), text("ipe200")), ("name".into(), text("IPE 200"))]), DslValue::object([("id".into(), text("hea300"))])])),
        (
            "members".into(),
            DslValue::Array(vec![
                DslValue::object([("id".into(), text("b1")), ("name".into(), text("Main beam")), ("kind".into(), text("beam")), ("sectionId".into(), text("ipe200")), ("length".into(), number(0.0015)), ("force".into(), number(125_000.0)), ("count".into(), DslValue::uint(3)), ("active".into(), DslValue::Bool(true)), ("angle".into(), number(std::f64::consts::FRAC_PI_2)), ("ratio".into(), number(12.5)), ("advanced".into(), number(1.0))]),
                DslValue::object([("id".into(), text("c2")), ("kind".into(), text("column")), ("sectionId".into(), text("hea300")), ("length".into(), number(4.5))]),
            ]),
        ),
    ])
}

fn failing_report() -> CheckReport {
    let mut report = CheckReport::default();
    let subject = SubjectRef::new("b1", "members[id=b1].length", LocalizedCopy::new("Beam b1", "Träger b1"));
    let remedy = Remedy::at_least(subject.clone(), Quantity::length_m(0.0015), Quantity::length_m(0.022), LocalizedCopy::new("Lengthen the beam", "Träger verlängern"));
    report.push(
        CheckResult::assess("b1.bending", "3", ClauseId::new("EN 1993-1-1", "6", "6.2.5"), subject, LocalizedCopy::new("Bending resistance", "Biegetragfähigkeit"))
            .utilization(Quantity::new(QuantityKind::Moment, 112.0), Quantity::new(QuantityKind::Moment, 100.0))
            .annex(AnnexChoice::De)
            .explanation(LocalizedCopy::new("too weak", "zu schwach"))
            .remedy(remedy)
            .build(),
    );
    report.push(
        CheckResult::assess("c2.buckling", "3", ClauseId::new("EN 1993-1-1", "6", "6.3.1"), SubjectRef::new("c2", "members[id=c2]", LocalizedCopy::new("Column c2", "Stütze c2")), LocalizedCopy::new("Buckling", "Knicken"))
            .utilization(Quantity::new(QuantityKind::Force, 64.0), Quantity::new(QuantityKind::Force, 100.0))
            .annex(AnnexChoice::De)
            .explanation(LocalizedCopy::new("ok", "ok"))
            .build(),
    );
    report
}

fn open_everything() -> ViewModel {
    use semio_plugin::TREE_WINDOW_PATH_SEPARATOR as SEP;
    let root = inputs_section_id("");
    let members = format!("{root}{SEP}{}", inputs_section_id("members"));
    let b1 = format!("{members}{SEP}{}", inputs_section_id("members[id=b1]"));
    let c2 = format!("{members}{SEP}{}", inputs_section_id("members[id=c2]"));
    let request = |node_key: String| TreeWindowRequest { body_key: BODY.into(), node_key, open: Some(true), offset: 0, rows: 32 };
    ViewModel { tree_windows: vec![request(root), request(members), request(b1), request(c2)], tree_viewport_rows: Some(32), ..ViewModel::new(Locale::En, semio_ui_locale::Terminology::Native) }
}

const BODY: &str = "norm.mock.inputs";

fn render(locale: Locale, report: &CheckReport, options: &InspectorOptions) -> String {
    let mut view = open_everything();
    if let Some(annex) = &options.annex_parameters {
        let rows = semio_s_plugin_norm_compliance::annex_parameter_rows(&annex.prefix, annex.annex);
        let id = format!("{}gamma-m1", annex.prefix);
        let offset = rows.iter().position(|row| row.entry.id == id).expect("annex fixture parameter");
        view.tree_windows.push(TreeWindowRequest { body_key: BODY.into(), node_key: inputs_section_id("annex-parameters"), open: Some(true), offset: offset.try_into().expect("annex row offset"), rows: 1 });
    }
    project(render_inspector_value(&document(), report, locale, "norm.mock", Some(member_meta), &TreeWindows::for_body(&view, BODY), options).expect("inspector"))
}

//#region 📏️Units
#[test]
fn the_engineering_system_shows_natural_units_and_the_si_system_the_stored_unit() {
    let spec = FieldSpec::quantity("Length", "Länge", UnitId::Metre);
    let shown = |spec: &FieldSpec, system| UnitDisplay::of_spec(spec, system).expect("quantity");
    assert_eq!(shown(&spec, UnitSystem::Engineering).symbol, "mm");
    assert_eq!(shown(&spec, UnitSystem::Si).symbol, "m");
    assert_eq!(shown(&FieldSpec::quantity("Stress", "Spannung", UnitId::Pascal), UnitSystem::Engineering).symbol, "MPa");
    assert_eq!(shown(&FieldSpec::quantity("Force", "Kraft", UnitId::Newton), UnitSystem::Si).symbol, "N");
    assert_eq!(shown(&FieldSpec::quantity("Flux", "Fluss", UnitId::WattPerSquareMetre), UnitSystem::Engineering).factor, 1.0);
    assert_eq!(shown(&FieldSpec { kind: FieldKind::Angle, ..FieldSpec::new("A", "A") }, UnitSystem::Si).symbol, "°");
    assert_eq!(shown(&FieldSpec { kind: FieldKind::Ratio, ..FieldSpec::new("R", "R") }, UnitSystem::Engineering).symbol, "");
    assert_eq!(UnitDisplay::between(UnitId::Celsius, UnitId::Kelvin).symbol, "°C", "offset units are never scaled");
    assert!(UnitDisplay::of_spec(&FieldSpec::new("Name", "Name"), UnitSystem::Si).is_none());
}

#[test]
fn unit_text_matches_the_independent_exact_arithmetic_oracle() {
    let rows: serde_json::Value = serde_json::from_str(GOLDEN).expect("golden");
    for row in rows.as_array().expect("rows") {
        let stored = UnitId::lookup(row["si"].as_str().expect("si")).expect("stored");
        let display = UnitDisplay::between(stored, UnitId::lookup(row["symbol"].as_str().expect("symbol")).expect("shown"));
        let precision = row["precision"].as_u64().expect("precision") as u16;
        let value = row["siValue"].as_f64().expect("value");
        assert_eq!(format_quantity_text(value, &display, precision, Locale::En), row["en"].as_str().expect("en"), "{row}");
        assert_eq!(format_quantity_text(value, &display, precision, Locale::De), row["de"].as_str().expect("de"), "{row}");
    }
}

#[test]
fn numbers_parse_per_locale_and_round_trip() {
    assert_eq!(parse_number("1.5", Locale::En), Some(1.5));
    assert_eq!(parse_number("1,5", Locale::De), Some(1.5));
    assert_eq!(parse_number("1,5", Locale::En), Some(1.5));
    assert_eq!(parse_number("1,234", Locale::En), Some(1234.0));
    assert_eq!(parse_number("1.234", Locale::De), Some(1234.0));
    assert_eq!(parse_number("1.234,5", Locale::De), Some(1234.5));
    assert_eq!(parse_number("1,234.5", Locale::En), Some(1234.5));
    assert_eq!(parse_number("-0,25", Locale::De), Some(-0.25));
    assert_eq!(parse_number("", Locale::En), None);
    assert_eq!(parse_number("abc", Locale::En), None);
    for value in [0.0, 1.5, -2.25, 1234.5, 0.001] {
        for locale in [Locale::En, Locale::De] {
            assert_eq!(parse_number(&format_number(value, 3, locale), locale), Some(value));
        }
    }
}

#[test]
fn typed_quantities_convert_to_the_stored_unit_and_name_their_problems() {
    let mm = UnitDisplay::between(UnitId::Metre, UnitId::Millimetre);
    assert!((parse_quantity("1,5 mm", &mm, Locale::De).expect("mm") - 0.0015).abs() < 1e-12);
    assert!((parse_quantity("1.5", &mm, Locale::En).expect("bare") - 0.0015).abs() < 1e-12);
    assert!((parse_quantity("2 m", &mm, Locale::En).expect("m") - 2.0).abs() < 1e-12);
    assert!((parse_quantity("30 cm", &mm, Locale::En).expect("cm") - 0.3).abs() < 1e-12);
    assert!((parse_quantity("1 ft", &mm, Locale::En).expect("ft") - 0.3048).abs() < 1e-12);
    assert_eq!(parse_quantity("", &mm, Locale::En), Err(QuantityIssue::Empty));
    assert_eq!(parse_quantity("mm", &mm, Locale::En), Err(QuantityIssue::NotANumber));
    assert_eq!(parse_quantity("3 parsec", &mm, Locale::En), Err(QuantityIssue::UnknownUnit("parsec".into())));
    assert_eq!(parse_quantity("3 kN", &mm, Locale::En), Err(QuantityIssue::UnknownUnit("kN".into())), "a unit of another family is refused");
    let degrees = UnitDisplay::between(UnitId::Radian, UnitId::Degree);
    assert!((parse_quantity("90", &degrees, Locale::En).expect("deg") - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
}
//#endregion 📏️Units

//#region 🧾️FieldViews
#[test]
fn field_kinds_come_from_the_field_spec_and_sibling_collections() {
    let root = document();
    let member = root.get("members").and_then(|members| if let DslValue::Array(items) = members { items.first() } else { None }).expect("member").clone();
    let view = |key: &str| field_view(&format!("members[id=b1].{key}"), member.get(key).expect("field"), &root, Locale::En, Some(member_meta), UnitSystem::Engineering);
    assert!(matches!(view("length").control, FieldControl::Quantity { ref unit } if unit.symbol == "mm"));
    assert!(matches!(view("force").control, FieldControl::Quantity { ref unit } if unit.symbol == "kN"));
    assert!(matches!(view("angle").control, FieldControl::Quantity { ref unit } if unit.symbol == "°"));
    assert!(matches!(view("ratio").control, FieldControl::Quantity { ref unit } if unit.symbol == "%"));
    assert!(matches!(view("kind").control, FieldControl::Choice(ref options) if options.len() == 2 && options[0].label == "Beam"));
    assert!(matches!(view("sectionId").control, FieldControl::EntityRef { ref to, ref options } if to == "sections" && options.len() == 2 && options[0].label == "ipe200 — IPE 200"));
    assert!(matches!(view("count").control, FieldControl::Int));
    assert!(matches!(view("active").control, FieldControl::Toggle));
    assert!(matches!(view("name").control, FieldControl::Text));
    assert_eq!(view("length").labelled(), "Length (mm)");
    let german = field_view("members[id=b1].kind", member.get("kind").expect("kind"), &root, Locale::De, Some(member_meta), UnitSystem::Engineering);
    assert!(matches!(german.control, FieldControl::Choice(ref options) if options[0].label == "Träger"));
}

#[test]
fn a_spec_carries_its_range_step_help_clause_and_annex_parameter_into_the_view() {
    let root = document();
    let length = field_view("members[id=b1].length", &number(0.0015), &root, Locale::De, Some(member_meta), UnitSystem::Engineering);
    assert_eq!((length.range, length.soft_range, length.step, length.precision), (Some((0.001, 0.5)), Some((0.001, 0.1)), Some(0.001), 3));
    assert_eq!(length.help.as_deref(), Some("Stützweite zwischen den Auflagern"));
    assert_eq!(length.clause, Some(BENDING));
    let force = field_view("members[id=b1].force", &number(125_000.0), &root, Locale::En, Some(member_meta), UnitSystem::Engineering);
    assert_eq!(force.ndp.as_ref().map(|ndp| (ndp.symbol.as_str(), ndp.default)), Some(("γ_M0", Some(100_000.0))));
}

#[test]
fn hard_ranges_refuse_with_localized_bounds_in_display_units() {
    let mm = UnitDisplay::between(UnitId::Metre, UnitId::Millimetre);
    let mut field = FieldView::new("members[id=b1].length", "Length", FieldControl::Quantity { unit: mm });
    field.range = Some((0.001, 0.5));
    assert_eq!(field.violation(0.01, Locale::En), None);
    assert_eq!(field.violation(2.0, Locale::En).as_deref(), Some("Out of range, allowed: 1 mm … 500 mm"));
    assert_eq!(field.violation(0.0, Locale::De).as_deref(), Some("Außerhalb des Bereichs, zulässig: 1 mm … 500 mm"));
}

#[test]
fn predicates_and_tiers_hide_fields_without_removing_them_from_the_document() {
    let siblings = vec![("kind".to_string(), text("beam")), ("advanced".to_string(), number(1.0))];
    let columns = vec![("kind".to_string(), text("column"))];
    assert!(!shown_in(member_meta("members[].columnWidth"), &siblings, Tier::Expert));
    assert!(shown_in(member_meta("members[].columnWidth"), &columns, Tier::Expert));
    assert!(!shown_in(member_meta("members[].advanced"), &siblings, Tier::Basic));
    assert!(shown_in(member_meta("members[].advanced"), &siblings, Tier::Advanced));
    assert!(shown_in(None, &siblings, Tier::Basic));
}
//#endregion 🧾️FieldViews

//#region ✅️Feedback
#[test]
fn feedback_binds_checks_and_remedies_to_the_exact_field_path() {
    let report = failing_report();
    let feedback = field_feedback(&report, "members[id=b1].length", Locale::De).expect("feedback");
    assert_eq!((feedback.check_id.as_str(), feedback.status, feedback.title.as_str()), ("b1.bending", CheckStatus::Fail, "Biegetragfähigkeit"));
    assert!((feedback.utilization - 1.12).abs() < 1e-9);
    let chip = feedback.remedy.expect("remedy");
    assert_eq!((chip.remedy_index, chip.applicable, chip.action.as_str()), (0, true, "Träger verlängern"));
    assert_eq!(chip.required, "22.0 mm");
    assert!(field_feedback(&report, "members[id=b1].force", Locale::En).is_none());
    assert!(field_feedback(&report, "members[id=c2]", Locale::En).is_some());
}

#[test]
fn entity_feedback_rolls_up_by_id_and_by_path_prefix() {
    let report = failing_report();
    let b1 = entity_feedback(&report, "b1", "members[id=b1]").expect("b1");
    assert_eq!((b1.status, b1.checks), (CheckStatus::Fail, 1));
    let c2 = entity_feedback(&report, "", "members[id=c2]").expect("c2");
    assert_eq!(c2.status, CheckStatus::Pass);
    assert!(entity_feedback(&report, "zz", "members[id=zz]").is_none());
    assert!(path_covers("members[id=b1]", "members[id=b1].length") && !path_covers("members[id=b1]", "members[id=b10].length") && !path_covers("", "x"));
}

#[test]
fn glyphs_never_rely_on_colour_alone() {
    assert_eq!(utilization_text(CheckStatus::Fail, 1.124), "✗ 112 %");
    assert_eq!(utilization_text(CheckStatus::Pass, 0.64), "✓ 64 %");
    assert_eq!(status_glyph(CheckStatus::Warning), "⚠");
    assert_eq!(status_glyph(CheckStatus::NotApplicable), "∅");
}
//#endregion ✅️Feedback

//#region 🪟️Inspector
#[test]
fn the_inspector_commits_every_control_through_set_field_with_unit_scaling_and_labels() {
    let json = render(Locale::En, &CheckReport::default(), &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    assert!(json.contains("setField") && json.contains("Length (mm)") && json.contains("Force (kN)"), "{json}");
    assert!(json.contains("displayFactor") && json.contains("1000"), "{json}");
    assert!(json.contains("Main beam") && json.contains("ipe200 — IPE 200"), "{json}");
    assert!(json.contains("insertItem") && json.contains("removeItem"), "{json}");
    assert!(json.contains("checkbox") || json.contains("toggle"), "{json}");
    assert!(json.contains("numberStepper") || json.contains("number-stepper") || json.contains("stepper"), "{json}");
}

#[test]
fn the_inspector_is_bilingual() {
    let report = failing_report();
    let en = render(Locale::En, &report, &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    let de = render(Locale::De, &report, &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    assert!(en.contains("Length (mm)") && en.contains("Beam") && en.contains("Add item"), "{en}");
    assert!(de.contains("Länge (mm)") && de.contains("Träger") && de.contains("Eintrag hinzufügen"), "{de}");
    assert!(!de.contains("Add item") && !en.contains("Eintrag hinzufügen"));
}

#[test]
fn inline_feedback_shows_the_bar_and_a_one_click_remedy_next_to_the_field() {
    let json = render(Locale::En, &failing_report(), &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    assert!(json.contains("norm-inputs.members[id=b1].length.utilisation") && json.contains("✗ 112 %"), "{json}");
    assert!(json.contains("applyRemedy") && json.contains("b1.bending") && json.contains("22.0 mm"), "{json}");
    assert!(!json.contains("norm-inputs.members[id=b1].force.utilisation"), "{json}");
    assert!(json.contains("✓ 64 %"), "entity roll-up of c2: {json}");
}

#[test]
fn the_si_preset_keeps_units_unscaled() {
    let json = render(Locale::En, &CheckReport::default(), &InspectorOptions { preset: UnitSystem::Si, ..InspectorOptions::new(UnitSystem::Engineering, Tier::Expert) });
    assert!(json.contains("Length (m)") && json.contains("Force (N)") && !json.contains("Length (mm)"), "{json}");
}

#[test]
fn the_annex_parameters_section_lists_both_annex_values_with_the_verification_badge() {
    let options = InspectorOptions { annex_parameters: Some(AnnexParametersView { prefix: "ndp.en1993.".into(), annex: AnnexChoice::De }), ..InspectorOptions::new(UnitSystem::Engineering, Tier::Expert) };
    let json = render(Locale::En, &CheckReport::default(), &options);
    assert!(json.contains("Annex parameters") && json.contains("norm-inputs.annex-parameters.ndp.en1993.gamma-m1"), "{json}");
    assert!(json.contains("[unverified]") && json.contains("EN 1 / DE 1.1 ≠") && json.contains("Active DE-NA: 1.1"), "{json}");
    let german = render(Locale::De, &CheckReport::default(), &options);
    assert!(german.contains("Anhangsparameter") && german.contains("[ungeprüft]") && german.contains("Aktiv DE-NA"), "{german}");
    let plain = render(Locale::En, &CheckReport::default(), &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    assert!(!plain.contains("Annex parameters"), "{plain}");
}

#[test]
fn no_json_text_is_ever_visible() {
    fn strings(value: &serde_json::Value, out: &mut Vec<String>) {
        match value {
            serde_json::Value::String(raw) => out.push(raw.clone()),
            serde_json::Value::Array(items) => items.iter().for_each(|item| strings(item, out)),
            serde_json::Value::Object(map) => map.values().for_each(|item| strings(item, out)),
            _ => {}
        }
    }
    let json = render(Locale::En, &failing_report(), &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    let projected: serde_json::Value = serde_json::from_str(&json).expect("projection");
    let mut all = Vec::new();
    strings(&projected, &mut all);
    for text in all {
        let trimmed = text.trim_start();
        assert!(!(trimmed.starts_with('{') && trimmed.ends_with('}')) && !(trimmed.starts_with("[{") && trimmed.ends_with(']')), "json-looking text: {text}");
    }
    assert!(!json.contains("documentJson"));
}
//#endregion 🪟️Inspector

#[test]
fn spec_help_clause_and_annex_parameter_render_in_both_languages() {
    let de = render(Locale::De, &CheckReport::default(), &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    assert!(de.contains("Stützweite zwischen den Auflagern") && de.contains("EN 1993-1-1 6 Abschn. 6.2.5"), "{de}");
    assert!(de.contains("[NDP γ_M0]") && de.contains("Auf Anhangswert zurücksetzen") && de.contains("100000"), "{de}");
    let en = render(Locale::En, &CheckReport::default(), &InspectorOptions::new(UnitSystem::Engineering, Tier::Expert));
    assert!(en.contains("Span between the supports") && en.contains("EN 1993-1-1 6 §6.2.5") && en.contains("Reset to annex default"), "{en}");
    assert!(!en.contains("Stützweite"), "{en}");
}

#[test]
fn the_tier_hides_advanced_fields_in_the_rendered_inspector() {
    let basic = render(Locale::En, &CheckReport::default(), &InspectorOptions { tier: Tier::Basic, ..InspectorOptions::new(UnitSystem::Engineering, Tier::Expert) });
    assert!(!basic.contains("norm-inputs.members[id=b1].advanced"), "{basic}");
}

#[test]
fn the_unit_picker_marks_the_chosen_system_and_writes_the_view_lane_in_live_editors_only() {
    let view = open_everything();
    let draw = |controller: &'static str, system| project(render_inspector_value(&document(), &CheckReport::default(), Locale::De, controller, Some(member_meta), &TreeWindows::for_body(&view, BODY), &InspectorOptions { preset: system, unit_picker: true, ..InspectorOptions::new(UnitSystem::Engineering, Tier::Expert) }).expect("inspector"));
    let live = draw("s.norm.en1990@1/*#editor", UnitSystem::Si);
    assert!(live.contains("viewportSetUnits") && live.contains("viewportAct") && live.contains("○ Ingenieureinheiten") && live.contains("● SI-Basiseinheiten") && live.contains("Länge (m)"), "{live}");
    assert!(!draw("norm.mock", UnitSystem::Si).contains("viewportSetUnits"), "apps without the viewport tool get no picker");
}

#[test]
fn a_multi_selection_shows_the_common_value_or_mixed_and_its_controls_raise_one_bulk_set() {
    fn admitted_metadata(path: &str) -> Option<FieldSpec> {
        admit_field_meta(None, !path.ends_with(".id"))
    }
    let row = |id: &str, x: f64, kind: &str| DslValue::object(vec![("id".to_string(), DslValue::String(id.into())), ("xM".to_string(), DslValue::float(x)), ("kind".to_string(), DslValue::String(kind.into()))]);
    let tree = DslValue::object(vec![("beams".to_string(), DslValue::Array(vec![row("b1", 1.0, "steel"), row("b2", 2.0, "steel"), row("b3", 3.0, "timber")]))]);
    let both = ["b1".to_string(), "b2".to_string()];
    let json = |locale| semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(render_bulk_inspector(&tree, "beams", &both, locale, "norm.mock", Some(admitted_metadata), UnitSystem::Engineering).expect("bulk inspector"))).expect("projection");
    let english = json(Locale::En);
    assert!(english.contains("2 selected") && english.contains("Mixed"), "xM differs between b1 and b2: {english}");
    assert!(english.contains("setFields") && english.contains("b1\\nb2") && english.contains("xM"), "a control writes every selected row: {english}");
    assert!(!english.contains("\"setField\""), "no single-row write remains: {english}");
    assert!(json(Locale::De).contains("2 ausgewählt") && json(Locale::De).contains("Gemischt"));
    let same = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(render_bulk_inspector(&tree, "beams", &["b1".to_string(), "b2".to_string()], Locale::En, "norm.mock", Some(admitted_metadata), UnitSystem::Engineering).expect("bulk"))).expect("projection");
    assert!(same.contains("steel"), "kind is the same in both rows: {same}");
    let mixed_kind = ["b2".to_string(), "b3".to_string()];
    assert_eq!(bulk_rows(&tree, "beams", &mixed_kind, false).iter().filter(|line| line.mixed).count(), 2);
    assert!(render_selection_inspector(&tree, &["b1".to_string()], Locale::En, "norm.mock", Some(admitted_metadata), UnitSystem::Engineering).unwrap().is_none(), "one row keeps the single-row inspector");
    assert!(render_selection_inspector(&tree, &both, Locale::En, "norm.mock", Some(admitted_metadata), UnitSystem::Engineering).unwrap().is_some());
    assert!(render_bulk_inspector(&tree, "nope", &both, Locale::En, "norm.mock", Some(admitted_metadata), UnitSystem::Engineering).is_err());
}
