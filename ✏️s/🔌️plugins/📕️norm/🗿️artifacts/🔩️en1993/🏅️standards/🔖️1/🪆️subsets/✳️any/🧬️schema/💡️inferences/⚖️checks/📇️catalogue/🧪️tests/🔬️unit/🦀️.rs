use super::*;
use design_case::CaseCatalogue;
use semio_framework_test_fixtures::fixture_text;

fn run(id: &str) {
    let entry = design_case::entry::<En1993Cases>(id).expect("catalogue entry");
    let case = design_case::seed::<En1993Cases>(&entry, "case-test");
    assert!(design_case::issues::<En1993Cases>(&entry, &case.values).is_empty(), "{id}: the example values are valid vocabulary rows");
    assert_eq!(design_case::declared::<En1993Cases>(&entry).len(), entry.inputs.len(), "{id}: every declared input has a vocabulary row");
    let results = evaluate_design_case(&case, AnnexChoice::En);
    assert!(!results.is_empty(), "{id}: no result");
    for result in &results {
        assert!(!result.clause.section.is_empty() && result.clause.family == entry.part, "{id}: clause link");
        assert!(result.utilization.is_finite() && result.utilization > 0.0, "{id}: utilisation {}", result.utilization);
        assert!(result.subject.path == format!("cases[id={}]", case.id), "{id}: addressed to the case row");
        assert!(!result.id.contains(".input."), "{id}: input issue {}", result.id);
    }
}

macro_rules! entry_tests {
    ($($name:ident => $id:literal),* $(,)?) => { $(#[test] fn $name() { run($id); })* };
}

entry_tests! {
    plate_under_transverse_pressure => "C93-01",
    bridge_fatigue_damage_equivalence => "C93-02",
    fatigue_combined_stress_ranges => "C93-03",
    sheet_pile_bending => "C93-04",
    cable_tension => "C93-05",
    tower_shaft_compression => "C93-06",
    base_plate_t_stub => "C93-07",
    crane_runway_web => "C93-08",
    von_mises_criterion => "C93-09",
    class_3_fibre_stress => "C93-10",
    biaxial_bending_with_axial_force => "C93-11",
    bending_shear_interaction => "C93-12",
    critical_temperature => "C93-13",
    pipe_hoop_thickness => "C93-14",
    pipe_ovalisation => "C93-15",
    hot_spot_fatigue_verifies_both_stress_ranges => "C93-16",
    lattice_joint_fatigue_uses_secondary_moment_factor => "C93-17",
    runway_junction_fatigue_uses_table_category => "C93-18",
    open_stringer_direct_fatigue_uses_detail_thickness => "C93-19",
    continuous_open_stringer_fatigue_uses_web_stress => "C93-20",
    closed_stringer_connection_uses_table_categories => "C93-21",
    closed_stringer_cutout_uses_selected_stress_method => "C93-22",
    closed_stringer_splice_uses_table_8_3_size_effect => "C93-23",
    closed_stringer_weld_calculates_bending_stress => "C93-24",
}

#[test]
fn closed_stringer_table_8_8_fatigue() {
    use semio_s_plugin_norm_compliance::CheckStatus;
    use std::collections::BTreeMap;

    type Case = (String, BTreeMap<String, String>, BTreeMap<String, f64>, Vec<(f64, f64, String)>);
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../📇️catalogue/🧪️tests/table-8-8.json")).expect("Table 8.8 fixture");
    for row in fixture["cases"].as_array().expect("fixture cases") {
        let (id, choices, numbers, expected): Case = serde_json::from_value(row.clone()).expect("fixture row");
        let entry = design_case::entry::<En1993Cases>(&id).expect("entry");
        let mut case = design_case::seed::<En1993Cases>(&entry, &id);
        case.values.texts.extend(choices);
        case.values.numbers.extend(numbers);
        let results = evaluate_design_case(&case, AnnexChoice::En);
        if expected.is_empty() {
            assert_eq!(results.len(), 1, "{id}");
            assert_eq!(results[0].status, CheckStatus::NotApplicable, "{id}");
            continue;
        }
        assert_eq!(results.len(), expected.len(), "{id}");
        for (actual, (demand, capacity, status)) in results.iter().zip(expected) {
            assert_eq!(actual.clause.section, "8", "{id}");
            assert_eq!(actual.status, if status == "pass" { CheckStatus::Pass } else { CheckStatus::Fail }, "{id}");
            assert!((actual.computed.value / 1e6 - demand).abs() < 1e-9, "{id}");
            assert!((actual.limit.value / 1e6 - capacity).abs() < 1e-9, "{id}");
        }
    }
}

#[test]
fn open_stringer_table_8_9_fatigue() {
    use semio_s_plugin_norm_compliance::{CheckStatus, QuantityKind};
    use std::collections::BTreeMap;

    type Case = (String, String, BTreeMap<String, f64>, Vec<(f64, f64, String)>);
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../📇️catalogue/🧪️tests/table-8-9.json")).expect("Table 8.9 fixture");
    for row in fixture["cases"].as_array().expect("fixture cases") {
        let (id, basis, numbers, expected): Case = serde_json::from_value(row.clone()).expect("fixture row");
        let entry = design_case::entry::<En1993Cases>(&id).expect("entry");
        let mut case = design_case::seed::<En1993Cases>(&entry, &id);
        let basis_key = if id == "C93-19" { "deck_open_end_basis" } else { "deck_open_cross_basis" };
        case.values.texts.insert(basis_key.into(), basis);
        case.values.numbers.extend(numbers);
        let results = evaluate_design_case(&case, AnnexChoice::En);
        if expected.is_empty() {
            assert_eq!(results.len(), 1, "{id}");
            assert_eq!(results[0].status, CheckStatus::NotApplicable, "{id}");
            continue;
        }
        assert_eq!(results.len(), expected.len(), "{id}");
        for (actual, (demand, capacity, status)) in results.iter().zip(expected) {
            let scale = if actual.computed.kind == QuantityKind::Stress { 1e6 } else { 1.0 };
            assert_eq!(actual.clause.section, "8", "{id}");
            assert_eq!(actual.status, if status == "pass" { CheckStatus::Pass } else { CheckStatus::Fail }, "{id}");
            assert!((actual.computed.value / scale - demand).abs() < 1e-9, "{id}");
            assert!((actual.limit.value / scale - capacity).abs() < 1e-9, "{id}");
        }
    }
}

#[test]
fn runway_table_8_10_categories_and_applicability() {
    use semio_s_plugin_norm_compliance::CheckStatus;

    type TableCase = (String, String, String, f64, f64, f64, f64, f64, Option<f64>, Option<f64>, String, String);
    let fixture = fixture_text!("../../../📇️catalogue/🧪️tests/table-8-10.json");
    let document: serde_json::Value = serde_json::from_str(fixture).expect("Table 8.10 fixture");
    let entry = design_case::entry::<En1993Cases>("C93-18").expect("entry");
    let status = |value: &str| match value {
        "pass" => CheckStatus::Pass,
        "fail" => CheckStatus::Fail,
        "not-applicable" => CheckStatus::NotApplicable,
        _ => panic!("unknown fixture status {value}"),
    };

    for row in document["cases"].as_array().expect("fixture cases") {
        let (detail, location, basis, equivalent, frequent, fy, gamma_ff, gamma_mf, demand, capacity, fatigue_status, frequent_status): TableCase =
            serde_json::from_value(row.clone()).expect("fixture row");
        let mut case = design_case::seed::<En1993Cases>(&entry, &detail);
        for (key, value) in [("runway_detail", &detail), ("runway_location", &location), ("runway_basis", &basis)] {
            case.values.texts.insert(key.into(), value.clone());
        }
        for (key, value) in [("runway_range", equivalent), ("runway_frequent", frequent), ("fy", fy), ("gamma_ff", gamma_ff), ("gamma_mf", gamma_mf)] {
            case.values.numbers.insert(key.into(), value);
        }
        let results = evaluate_design_case(&case, AnnexChoice::En);
        assert_eq!(results[0].status, status(&fatigue_status), "{detail}");
        assert_eq!(results[0].clause.section, "8", "{detail}");
        let (Some(demand), Some(capacity)) = (demand, capacity) else {
            assert_eq!(results.len(), 1, "{detail}");
            continue;
        };
        assert_eq!(results.len(), 2, "{detail}");
        assert!(results[0].id.contains(".fatigue."), "{detail}");
        assert!((results[0].computed.value / 1e6 - demand).abs() < 1e-9, "{detail}");
        assert!((results[0].limit.value / 1e6 - capacity).abs() < 1e-9, "{detail}");
        assert!(results[1].id.contains(".frequent-range."), "{detail}");
        assert_eq!(results[1].status, status(&frequent_status), "{detail}");
        assert!((results[1].computed.value / 1e6 - frequent).abs() < 1e-9, "{detail}");
        assert!((results[1].limit.value / 1e6 - 1.5 * fy).abs() < 1e-9, "{detail}");
    }
}

#[test]
fn a_missing_input_is_a_single_not_applicable_result() {
    let entry = design_case::entry::<En1993Cases>("C93-01").expect("entry");
    let mut case = design_case::seed::<En1993Cases>(&entry, "case-gap");
    case.values.numbers.remove("p_plate");
    let results = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
}

#[test]
fn the_demo_cases_cover_every_entry_once() {
    let rows = catalogue_demo_cases();
    let entries = En1993Cases::entries();
    assert_eq!(rows.len(), entries.len());
    assert_eq!(entries.len(), ENTRIES.len());
    for (row, entry) in rows.iter().zip(&entries) {
        assert_eq!(row.entry_id, entry.id);
        assert!(!evaluate_design_case(&DesignCase::from(row), AnnexChoice::De).is_empty());
    }
}

#[test]
fn hollow_section_table_8_6_categories_and_applicability() {
    verify_member_fatigue_fixture("C93-26", fixture_text!("../../../📇️catalogue/🧪️tests/table-8-6.json"));
}

#[test]
fn lattice_node_table_8_7_categories_and_applicability() {
    verify_member_fatigue_fixture("C93-25", fixture_text!("../../../📇️catalogue/🧪️tests/table-8-7.json"));
}

#[test]
fn load_carrying_weld_table_8_5_categories_and_applicability() {
    verify_member_fatigue_fixture("C93-27", fixture_text!("../../../📇️catalogue/🧪️tests/table-8-5.json"));
}

#[test]
fn weld_attachment_table_8_4_categories_and_applicability() {
    verify_member_fatigue_fixture("C93-28", fixture_text!("../../../📇️catalogue/🧪️tests/table-8-4.json"));
}

#[test]
fn transverse_butt_weld_table_8_3_categories_and_applicability() {
    verify_member_fatigue_fixture("C93-29", fixture_text!("../../../📇️catalogue/🧪️tests/table-8-3.json"));
}

#[test]
fn built_up_weld_table_8_2_categories_and_applicability() {
    verify_member_fatigue_fixture("C93-30", fixture_text!("../../../📇️catalogue/🧪️tests/table-8-2.json"));
}

#[test]
fn plain_and_fastened_table_8_1_categories_and_applicability() {
    verify_member_fatigue_fixture("C93-31", fixture_text!("../../../📇️catalogue/🧪️tests/table-8-1.json"));
}

fn verify_member_fatigue_fixture(entry_id: &str, source: &str) {
    use semio_s_plugin_norm_compliance::CheckStatus;

    type MemberCase = (String, std::collections::BTreeMap<String, String>, std::collections::BTreeMap<String, f64>, Vec<(f64, f64, String)>);
    let fixture: serde_json::Value = serde_json::from_str(source).expect("member fatigue fixture");
    let entry = design_case::entry::<En1993Cases>(entry_id).expect("case entry");
    for row in fixture["cases"].as_array().expect("cases") {
        let (id, choices, values, expected): MemberCase = serde_json::from_value(row.clone()).expect("fixture row");
        let mut case = design_case::seed::<En1993Cases>(&entry, &id);
        for (key, value) in choices {
            case.values.texts.insert(key, value);
        }
        for (key, value) in values {
            case.values.numbers.insert(key, value);
        }
        let results = evaluate_design_case(&case, AnnexChoice::En);
        if expected.is_empty() {
            assert_eq!(results.len(), 1, "{id}");
            assert_eq!(results[0].status, CheckStatus::NotApplicable, "{id}");
            continue;
        }
        assert_eq!(results.len(), expected.len(), "{id}");
        for (actual, (demand, capacity, status)) in results.iter().zip(expected) {
            let scale = if actual.computed.kind == semio_s_plugin_norm_compliance::QuantityKind::Stress { 1e6 } else { 1.0 };
            assert!((actual.computed.value / scale - demand).abs() < 1e-9, "{id}");
            assert!((actual.limit.value / scale - capacity).abs() < 1e-9, "{id}");
            assert_eq!(actual.status, if status == "pass" { CheckStatus::Pass } else { CheckStatus::Fail }, "{id}");
        }
    }
}
