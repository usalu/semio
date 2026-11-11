use super::*;
use norm::{ClauseId, Quantity, SubjectRef};

fn check(id: &str, case: &str, demand: f64, capacity: f64) -> CheckResult {
    CheckResult::assess(id, "EN TOY-1", ClauseId::new("EN TOY-1", "§6", "6.1.2"), SubjectRef::new(case, format!("cases[id={case}]"), norm::LocalizedCopy::new("Tie", "Zugstab")), norm::LocalizedCopy::new(format!("Check {id}"), format!("Nachweis {id}")))
        .utilization(Quantity::new(norm::QuantityKind::Force, demand), Quantity::new(norm::QuantityKind::Force, capacity))
        .build()
}

fn report() -> CheckReport {
    let mut report = CheckReport::default();
    report.push(check("a", "c1", 50.0, 100.0));
    report.push(check("b", "c1", 130.0, 100.0));
    report.push(check("other", "c2", 10.0, 100.0));
    report
}

#[test]
fn diagram_has_one_marker_and_table_row_per_check_of_the_case() {
    let diagram = case_diagram("c1", &LocalizedCopy::new("Tie", "Zugstab"), &report(), None).expect("diagram");
    assert_eq!(diagram.id, "case-c1");
    assert_eq!(diagram.markers.len(), 2);
    assert_eq!(diagram.table_twin.rows(), 2);
    assert_eq!(diagram.markers[0].bound_check.as_deref(), Some("a"));
    assert_eq!(diagram.markers[1].tone, Tone::Error);
    assert_eq!(diagram.markers[0].tone, Tone::Info);
    assert!((diagram.table_twin.columns[1].values[1] - 1.3).abs() < 1e-12);
    assert!(diagram.series.iter().any(|s| matches!(s, Series::Limit { value, .. } if (*value - 1.0).abs() < 1e-12)));
    assert_eq!(diagram.ids().len(), 4);
    assert!(diagram.validate().is_ok(), "{:?}", diagram.validate());
}

#[test]
fn diagram_adds_the_section_only_for_positive_sizes() {
    let with = case_diagram("c1", &LocalizedCopy::new("Tie", "Zugstab"), &report(), Some((0.2, 0.4))).expect("diagram");
    assert!(with.series.iter().any(|s| matches!(s, Series::Section { .. })));
    let without = case_diagram("c1", &LocalizedCopy::new("Tie", "Zugstab"), &report(), Some((0.0, 0.4))).expect("diagram");
    assert!(!without.series.iter().any(|s| matches!(s, Series::Section { .. })));
}

#[test]
fn a_case_without_applicable_checks_has_no_diagram() {
    assert!(case_diagram("missing", &LocalizedCopy::new("x", "x"), &report(), None).is_none());
}

struct Toy;

static VOCABULARY: &[norm::design_case::InputSpec] = &[
    norm::design_case::InputSpec::number("b", "Width b", "Breite b", UnitId::Millimetre),
    norm::design_case::InputSpec::number("h", "Height h", "Höhe h", UnitId::Metre),
    norm::design_case::InputSpec::number("ratio", "Ratio", "Verhältnis", UnitId::One),
];

impl norm::design_case::CaseCatalogue for Toy {
    fn entries() -> Vec<norm::design_case::CaseEntry> {
        Vec::new()
    }

    fn vocabulary() -> &'static [norm::design_case::InputSpec] {
        VOCABULARY
    }

    fn evaluate(_: &norm::design_case::CaseEntry, _: norm::AnnexChoice, _: &norm::design_case::DesignCase) -> Vec<CheckResult> {
        Vec::new()
    }
}

fn row() -> CaseRow {
    CaseRow { id: "c1".into(), label_en: String::new(), label_de: String::new(), entry_id: "T-01".into(), numbers: vec![norm::design_case::CaseNumber { id: "b".into(), value: 250.0 }, norm::design_case::CaseNumber { id: "h".into(), value: 4.0 }, norm::design_case::CaseNumber { id: "ratio".into(), value: 2.0 }], texts: Vec::new() }
}

#[test]
fn inputs_convert_from_their_stored_unit_to_metres() {
    assert!((input_metres::<Toy>(&row(), "b").expect("width") - 0.25).abs() < 1e-12);
    assert!((input_metres::<Toy>(&row(), "h").expect("height") - 4.0).abs() < 1e-12);
    assert_eq!(input_metres::<Toy>(&row(), "missing"), None);
    assert_eq!(input_metres::<Toy>(&row(), "ratio"), None);
}

#[test]
fn a_size_binding_scales_the_drag_in_metres_into_the_stored_unit() {
    let width = size_binding::<Toy>(&row(), "b", 2.0, 0.05).expect("binding");
    assert_eq!(width.path, "cases[id=c1].numbers[id=b].value");
    assert_eq!(width.unit, "mm");
    assert!((width.map.base - 250.0).abs() < 1e-12 && (width.map.k - 2000.0).abs() < 1e-9);
    assert!(matches!(width.quantise, Some(Quantise::Round { step }) if (step - 50.0).abs() < 1e-9));
    let height = size_binding::<Toy>(&row(), "h", -1.0, 0.05).expect("binding");
    assert!((height.map.k + 1.0).abs() < 1e-12 && height.unit == "m");
    let mut unset = row();
    unset.numbers.retain(|n| n.id != "b");
    assert!(size_binding::<Toy>(&unset, "b", 1.0, 0.05).is_none());
}
