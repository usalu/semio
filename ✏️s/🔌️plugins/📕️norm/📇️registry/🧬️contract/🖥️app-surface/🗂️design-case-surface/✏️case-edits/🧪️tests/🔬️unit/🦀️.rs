use super::*;
use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseText, CaseValues, InputChoice};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult};

struct Toy;

static VOCABULARY: &[InputSpec] = &[
    InputSpec::number("b", "Width b", "Breite b", UnitId::Millimetre).range(1.0, 5000.0).example(200.0),
    InputSpec::number("angle", "Angle", "Winkel", UnitId::Degree).example(30.0),
    InputSpec::choice("class", "Class", "Klasse", &[InputChoice::new("C24", "C24", "C24"), InputChoice::new("GL24h", "GL24h", "GL24h")]).example_text("C24"),
    InputSpec::text("note", "Note", "Notiz"),
    InputSpec::vector("profile", "Profile", "Profil", UnitId::Millimetre),
];

impl CaseCatalogue for Toy {
    fn entries() -> Vec<CaseEntry> {
        vec![CaseEntry { id: "T-01", part: "EN TOY-1", clause: "6.1.2", en: "Tension", de: "Zug", inputs: &["class", "b", "angle", "note", "profile"], geometry: None }]
    }

    fn vocabulary() -> &'static [InputSpec] {
        VOCABULARY
    }

    fn evaluate(_: &CaseEntry, _: AnnexChoice, _: &DesignCase) -> Vec<CheckResult> {
        Vec::new()
    }
}

fn row(id: &str) -> CaseRow {
    CaseRow { id: id.into(), label_en: "Tie".into(), label_de: "Zugstab".into(), entry_id: "T-01".into(), numbers: vec![CaseNumber { id: "b".into(), value: 200.0 }], texts: vec![CaseText { id: "class".into(), value: "C24".into() }] }
}

fn field<'a>(value: &'a DslValue, name: &str) -> &'a DslValue {
    let DslValue::Object(entries) = value else { panic!("object expected") };
    &entries.iter().find(|(key, _)| key == name).unwrap_or_else(|| panic!("field {name}")).1
}

fn input_row(rows: &[CaseRow], key: &str) -> CaseInputRow {
    case_input_rows::<Toy>(&DesignCase::from(&rows[0])).into_iter().find(|r| case_input_key(&r.path).is_some_and(|(k, _)| k == key)).unwrap_or_else(|| panic!("row {key}"))
}

#[test]
fn ids_are_slugs_that_stay_unique() {
    assert_eq!(case_slug("Case E90.04"), "case-e90-04");
    assert_eq!(next_case_id(&[], "case-T-01"), "case-t-01");
    assert_eq!(next_case_id(&[row("case-t-01")], "case-T-01"), "case-t-01-2");
    assert_eq!(next_case_id(&[row("case-t-01"), row("case-t-01-2")], "case-T-01"), "case-t-01-3");
}

#[test]
fn the_browser_adds_a_case_seeded_from_the_catalogue_entry() {
    let NormEdit::InsertItem { path, index, value: Some(case) } = insert_case_edit::<Toy>(&[row("a")], "T-01", 9).expect("edit") else { panic!("insert expected") };
    assert_eq!((path.as_str(), index), ("cases", 1));
    assert_eq!(field(&case, "id"), &DslValue::String("case-t-01".into()));
    assert_eq!(field(&case, "entryId"), &DslValue::String("T-01".into()));
    assert_eq!(field(&case, "labelEn"), &DslValue::String("Tension".into()));
    let DslValue::Array(numbers) = field(&case, "numbers") else { panic!("numbers") };
    let keys: Vec<&DslValue> = numbers.iter().map(|n| field(n, "id")).collect();
    assert!(keys.contains(&&DslValue::String("b".into())) && keys.contains(&&DslValue::String("angle".into())));
    assert!(insert_case_edit::<Toy>(&[], "T-99", 0).is_err());
}

#[test]
fn an_input_is_set_with_its_number_or_text_path() {
    assert_eq!(set_number_edit("c1", "b", 250.0), NormEdit::SetField { path: "cases[id=c1].numbers[id=b].value".into(), value: DslValue::float(250.0) });
    assert_eq!(set_text_edit("c1", "class", "GL24h"), NormEdit::SetField { path: "cases[id=c1].texts[id=class].value".into(), value: DslValue::String("GL24h".into()) });
}

#[test]
fn clearing_an_input_emits_the_null_set() {
    assert_eq!(clear_input_edit("c1", "b", true), NormEdit::SetField { path: "cases[id=c1].numbers[id=b].value".into(), value: DslValue::Null });
    assert_eq!(clear_input_edit("c1", "note", false), NormEdit::SetField { path: "cases[id=c1].texts[id=note].value".into(), value: DslValue::Null });
    let rows = [row("c1")];
    assert_eq!(input_row_edit(&input_row(&rows, "b"), "  ", UnitId::Millimetre), Ok(clear_input_edit("c1", "b", true)));
    assert_eq!(input_row_edit(&input_row(&rows, "class"), "", UnitId::One), Ok(clear_input_edit("c1", "class", false)));
}

#[test]
fn typed_numbers_use_their_unit_and_are_range_checked() {
    let rows = [row("c1")];
    let width = input_row(&rows, "b");
    let stored = |text: &str| match input_row_edit(&width, text, UnitId::Millimetre) {
        Ok(NormEdit::SetField { value: DslValue::Number(n), .. }) => n.as_f64(),
        other => panic!("{text}: {other:?}"),
    };
    assert!((stored("350") - 350.0).abs() < 1e-9);
    assert!((stored("0,25 m") - 250.0).abs() < 1e-9);
    assert!(matches!(input_row_edit(&width, "0", UnitId::Millimetre), Err(CaseInputError::OutOfRange { min, max, .. }) if min == 1.0 && max == 5000.0));
    assert!(matches!(input_row_edit(&width, "wide", UnitId::Millimetre), Err(CaseInputError::NotANumber(_))));
    assert!(input_row_edit(&width, "wide", UnitId::Millimetre).unwrap_err().message(Locale::De).contains("keine Zahl"));
    let angle = input_row(&rows, "angle");
    assert!(matches!(input_row_edit(&angle, "1.5707963267948966 rad", UnitId::Degree), Ok(NormEdit::SetField { value: DslValue::Number(n), .. }) if (n.as_f64() - 90.0).abs() < 1e-6));
}

#[test]
fn choices_must_be_options_and_text_is_kept() {
    let rows = [row("c1")];
    assert_eq!(input_row_edit(&input_row(&rows, "class"), "GL24h", UnitId::One), Ok(set_text_edit("c1", "class", "GL24h")));
    assert_eq!(input_row_edit(&input_row(&rows, "class"), "XX", UnitId::One), Err(CaseInputError::NotAChoice("XX".into())));
    assert_eq!(input_row_edit(&input_row(&rows, "note"), "free text", UnitId::One), Ok(set_text_edit("c1", "note", "free text")));
}

#[test]
fn vector_elements_are_rows_with_their_own_paths() {
    let mut with_profile = row("c1");
    with_profile.numbers.push(CaseNumber { id: "profile.0".into(), value: 10.0 });
    let rows = [with_profile];
    let next = input_row(&rows, "profile.1");
    assert_eq!(input_row_edit(&next, "20", UnitId::Millimetre), Ok(set_number_edit("c1", "profile.1", 20.0)));
    assert_eq!(input_row_edit(&input_row(&rows, "profile.0"), "", UnitId::Millimetre), Ok(clear_input_edit("c1", "profile.0", true)));
}

#[test]
fn current_values_show_in_the_requested_unit() {
    let rows = [row("c1")];
    assert!(input_row_text(&input_row(&rows, "b"), UnitId::Metre, Locale::En).starts_with("0.2"));
    assert!(input_row_text(&input_row(&rows, "b"), UnitId::Metre, Locale::De).starts_with("0,2"));
    assert_eq!(input_row_text(&input_row(&rows, "class"), UnitId::One, Locale::En), "C24");
    assert_eq!(input_row_text(&input_row(&rows, "angle"), UnitId::Degree, Locale::En), "");
}

#[test]
fn duplicate_copies_the_inputs_under_a_fresh_id_right_after_the_source() {
    let rows = [row("c1"), row("c2")];
    let NormEdit::InsertItem { index, value: Some(copy), .. } = duplicate_case_edit(&rows, 0).expect("duplicate") else { panic!("insert expected") };
    assert_eq!(index, 1);
    assert_eq!(field(&copy, "id"), &DslValue::String("c1-2".into()));
    assert_eq!(field(&copy, "labelEn"), &DslValue::String("Tie (copy)".into()));
    assert_eq!(field(&copy, "labelDe"), &DslValue::String("Zugstab (Kopie)".into()));
    assert_eq!(field(&copy, "numbers"), &semio_framework_value::ToValue::to_value(&rows[0].numbers));
    assert!(duplicate_case_edit(&rows, 2).is_err());
}

#[test]
fn remove_addresses_the_row_and_refuses_an_unknown_index() {
    let rows = [row("c1")];
    assert_eq!(remove_case_edit(&rows, 0), Ok(NormEdit::RemoveItem { path: "cases".into(), index: 0 }));
    assert!(remove_case_edit(&rows, 1).is_err());
}

#[test]
fn the_properties_tree_lists_declared_unset_inputs_as_empty_rows_without_touching_set_ones() {
    let document = DslValue::object(vec![("annex".to_string(), DslValue::String("en".into())), ("cases".to_string(), DslValue::Array(vec![semio_framework_value::ToValue::to_value(&row("c1"))]))]);
    let projected = case_input_tree::<Toy>(&document);
    let case = match field(&projected, "cases") {
        DslValue::Array(cases) => &cases[0],
        other => panic!("{other:?}"),
    };
    let ids = |name: &str| -> Vec<(String, DslValue)> {
        let DslValue::Array(items) = field(case, name) else { panic!("array") };
        items.iter().map(|item| match field(item, "id") {
            DslValue::String(id) => (id.clone(), field(item, "value").clone()),
            other => panic!("{other:?}"),
        }).collect()
    };
    assert_eq!(ids("numbers"), vec![("b".to_string(), DslValue::float(200.0)), ("angle".to_string(), DslValue::Null), ("profile.0".to_string(), DslValue::Null)]);
    assert_eq!(ids("texts"), vec![("class".to_string(), DslValue::String("C24".into())), ("note".to_string(), DslValue::String(String::new()))]);
    assert_eq!(field(&projected, "annex"), &DslValue::String("en".into()));
}

#[test]
fn edits_become_the_actions_the_browser_binds() {
    let rows = [row("c1")];
    let insert = insert_case_edit::<Toy>(&rows, "T-01", 1).expect("insert");
    assert!(edit_args(&insert).is_ok() && edit_args(&remove_case_edit(&rows, 0).expect("remove")).is_ok() && edit_args(&clear_input_edit("c1", "b", true)).is_ok());
}

#[test]
fn case_values_survive_the_row_round_trip_used_by_the_edits() {
    let case = DesignCase { id: "c1".into(), label_en: "Tie".into(), label_de: "Zugstab".into(), entry_id: "T-01".into(), values: CaseValues::default().with_number("b", 200.0).with_text("class", "C24") };
    assert_eq!(DesignCase::from(&CaseRow::from(&case)), case);
}

fn report_of(case: &CaseRow) -> semio_s_plugin_norm_compliance::CheckReport {
    let mut report = semio_s_plugin_norm_compliance::CheckReport::default();
    for result in design_case::evaluate_case::<Toy>(&DesignCase::from(case), AnnexChoice::En) {
        report.push(result);
    }
    report
}

#[test]
fn a_remedy_writes_the_stored_unit_value_into_the_case_input_and_the_case_then_passes() {
    let mut wide = row("c1");
    wide.numbers[0].value = 9000.0;
    wide.texts[0].value = "XX".into();
    let report = report_of(&wide);
    let range = report.checks.iter().find(|check| check.id.ends_with(".b")).expect("range issue");
    assert_eq!(range.status, semio_s_plugin_norm_compliance::CheckStatus::Fail);
    let choice = report.checks.iter().find(|check| check.id.ends_with(".class")).expect("choice issue");
    let mut document = DslValue::object(vec![("cases".to_string(), DslValue::Array(vec![semio_framework_value::ToValue::to_value(&wide)]))]);
    apply_remedy_edit(&report, &range.id, 0, 0, &mut document).expect("range remedy");
    apply_remedy_edit(&report, &choice.id, 0, 0, &mut document).expect("choice remedy");
    let DslValue::Array(cases) = field(&document, "cases") else { panic!("cases") };
    let fixed = <CaseRow as semio_framework_value::FromValue>::from_value(cases[0].clone()).expect("case row");
    assert_eq!(fixed.numbers[0].value, 5000.0, "the remedy bound is written in millimetres, the stored unit");
    assert_eq!(fixed.texts[0].value, "C24");
    assert!(report_of(&fixed).checks.iter().all(|check| check.status != semio_s_plugin_norm_compliance::CheckStatus::Fail));
}
