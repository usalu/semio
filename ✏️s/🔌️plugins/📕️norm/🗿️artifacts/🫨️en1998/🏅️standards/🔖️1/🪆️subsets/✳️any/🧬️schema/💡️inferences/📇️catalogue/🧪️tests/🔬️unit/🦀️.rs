use super::*;
use design_case::CaseCatalogue;

fn run(id: &str) {
    let entry = design_case::entry::<En1998Cases>(id).expect("catalogue entry");
    let case = design_case::seed::<En1998Cases>(&entry, "case-test");
    assert!(design_case::issues::<En1998Cases>(&entry, &case.values).is_empty(), "{id}: the example values are valid vocabulary rows");
    assert_eq!(design_case::declared::<En1998Cases>(&entry).len(), entry.inputs.len(), "{id}: every declared input has a vocabulary row");
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
    base_shear_from_design_spectrum => "C98-01",
    elastic_response_force => "C98-02",
    vertical_seismic_force => "C98-03",
    modal_response_spectrum => "C98-04",
    ground_type_from_layers => "C98-05",
    liquefaction_safety => "C98-06",
    pile_inertial_moment => "C98-07",
    diaphragm_chord_force => "C98-08",
}

#[test]
fn a_missing_input_is_a_single_not_applicable_result() {
    let entry = design_case::entry::<En1998Cases>("C98-01").expect("entry");
    let mut case = design_case::seed::<En1998Cases>(&entry, "case-gap");
    case.values.numbers.remove("mass");
    let results = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
}

#[test]
fn the_demo_cases_cover_every_entry_once() {
    let rows = catalogue_demo_cases();
    let entries = En1998Cases::entries();
    assert_eq!(rows.len(), entries.len());
    assert_eq!(entries.len(), ENTRIES.len());
    for (row, entry) in rows.iter().zip(&entries) {
        assert_eq!(row.entry_id, entry.id);
        assert!(!evaluate_design_case(&DesignCase::from(row), AnnexChoice::De).is_empty());
    }
}
