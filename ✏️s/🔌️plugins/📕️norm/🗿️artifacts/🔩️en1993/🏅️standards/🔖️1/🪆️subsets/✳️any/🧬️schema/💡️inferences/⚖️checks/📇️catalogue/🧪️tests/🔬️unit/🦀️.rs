use super::*;
use design_case::CaseCatalogue;

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
