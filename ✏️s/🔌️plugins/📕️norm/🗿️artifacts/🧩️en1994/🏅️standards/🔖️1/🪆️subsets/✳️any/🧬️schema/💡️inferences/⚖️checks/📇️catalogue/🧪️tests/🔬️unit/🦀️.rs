use super::*;
use design_case::CaseCatalogue;

fn run(id: &str) {
    let entry = design_case::entry::<En1994Cases>(id).expect("catalogue entry");
    let case = design_case::seed::<En1994Cases>(&entry, "case-test");
    assert!(design_case::issues::<En1994Cases>(&entry, &case.values).is_empty(), "{id}: the example values are valid vocabulary rows");
    assert_eq!(design_case::declared::<En1994Cases>(&entry).len(), entry.inputs.len(), "{id}: every declared input has a vocabulary row");
    let results = evaluate_design_case(&case, AnnexChoice::En);
    assert!(!results.is_empty(), "{id}: no result");
    for result in results.iter().filter(|result| result.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable) {
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
    stud_with_profiled_sheeting => "C94-01",
    patch_loading_with_bending => "C94-02",
    column_load_introduction => "C94-03",
    deflection_simple_span => "C94-04",
    deflection_two_span => "C94-05",
    steel_strength_in_fire => "C94-06",
    sagging_plastic_moment => "C94-07",
    hogging_plastic_moment => "C94-08",
    floor_vibration => "C94-09",
    continuous_beam_ltb => "C94-10",
    creep_and_shrinkage_deflection => "C94-11",
    protected_steel_in_fire => "C94-12",
}

#[test]
fn a_missing_input_is_a_single_not_applicable_result() {
    let entry = design_case::entry::<En1994Cases>("C94-01").expect("entry");
    let mut case = design_case::seed::<En1994Cases>(&entry, "case-gap");
    case.values.numbers.remove("p_ed");
    let results = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
}

#[test]
fn the_demo_cases_cover_every_entry_once() {
    let rows = catalogue_demo_cases();
    let entries = En1994Cases::entries();
    assert_eq!(rows.len(), entries.len());
    assert_eq!(entries.len(), ENTRIES.len());
    for (row, entry) in rows.iter().zip(&entries) {
        assert_eq!(row.entry_id, entry.id);
        assert!(!evaluate_design_case(&DesignCase::from(row), AnnexChoice::De).is_empty());
    }
}

#[test]
fn every_geometry_key_is_a_declared_input_of_its_entry() {
    for entry in En1994Cases::entries() {
        let geometry = entry.geometry.unwrap_or_else(|| panic!("{} declares no geometry", entry.id));
        for key in geometry.keys() {
            assert!(entry.inputs.contains(&key), "{}: geometry key {key} is no input", entry.id);
        }
    }
}

fn utilisation_with(id: &str, key: &str, value: f64) -> f64 {
    let entry = design_case::entry::<En1994Cases>(id).expect("catalogue entry");
    let mut case = design_case::seed::<En1994Cases>(&entry, "case-sweep");
    case.values.numbers.insert(key.into(), value);
    evaluate_design_case(&case, AnnexChoice::En)[0].utilization
}

#[test]
fn a_stiffer_floor_beam_lowers_the_vibration_utilisation() {
    assert!(utilisation_with("C94-09", "i_short", 1.2e-3) < utilisation_with("C94-09", "i_short", 6e-4));
}

#[test]
fn a_stiffer_slab_restraint_raises_the_buckling_resistance() {
    assert!(utilisation_with("C94-10", "i2_slab", 2e-4) < utilisation_with("C94-10", "i2_slab", 1e-5));
}

#[test]
fn creep_and_shrinkage_raise_the_deflection() {
    assert!(utilisation_with("C94-11", "phi_t", 3.5) > utilisation_with("C94-11", "phi_t", 0.0));
    assert!(utilisation_with("C94-11", "eps_cs", 5e-4) > utilisation_with("C94-11", "eps_cs", 0.0));
}

#[test]
fn a_thicker_protection_lowers_the_steel_temperature_ratio() {
    assert!(utilisation_with("C94-12", "d_p", 0.04) < utilisation_with("C94-12", "d_p", 0.01));
}

#[test]
fn the_floor_acceleration_is_reported_as_not_modelled_next_to_the_frequency() {
    let entry = design_case::entry::<En1994Cases>("C94-09").expect("entry");
    let results = evaluate_design_case(&design_case::seed::<En1994Cases>(&entry, "case-accel"), AnnexChoice::En);
    let gap = results.iter().find(|result| result.id.contains(".acceleration.")).expect("the acceleration gap is listed");
    assert_eq!(gap.status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
    assert_ne!(results[0].status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
    assert_eq!(results.len(), 2);
}
