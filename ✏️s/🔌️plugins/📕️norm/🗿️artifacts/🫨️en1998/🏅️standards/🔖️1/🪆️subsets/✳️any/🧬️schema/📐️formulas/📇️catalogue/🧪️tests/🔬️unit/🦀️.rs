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
        if result.status != semio_s_plugin_norm_compliance::CheckStatus::Info {
            assert!(result.utilization.is_finite() && result.utilization > 0.0, "{id}: utilisation {}", result.utilization);
        }
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
    tower_modal_damping_by_component => "C98-09",
    cable_stiffness_for_a_guyed_mast => "C98-10",
    buried_pipe_wave_passage => "C98-11",
    bridge_ground_motion_set_a => "C98-12",
    bridge_ground_motion_set_b => "C98-13",
    bridge_combined_spatial_effect => "C98-14",
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

#[test]
fn modal_damping_follows_annex_b_and_uses_the_correct_spectrum_branch() {
    use crate::part_6::MODAL_DAMPING_RATIOS;
    use semio_s_plugin_norm_compliance::CheckStatus;

    for (line, actual) in include_str!("tower-modal-damping.tsv").lines().skip(1).zip(MODAL_DAMPING_RATIOS) {
        let [key, lower, upper] = line.split('\t').collect::<Vec<_>>().try_into().unwrap();
        assert_eq!(actual, (key, lower.parse().unwrap(), upper.parse().unwrap()));
    }
    let entry = design_case::entry::<En1998Cases>("C98-09").unwrap();
    let mut case = design_case::seed::<En1998Cases>(&entry, "damping");
    case.values = case.values.with_number("t_1", 0.0).with_number("a_g", 1.5)
        .with_number("s_soil", 1.0).with_number("mass", 10000.0);
    let result = evaluate_design_case(&case, AnnexChoice::En);
    assert!((result[0].computed.value - 0.029).abs() < 1e-12);
    assert!((result[1].computed.value - 15000.0).abs() < 1e-8);
    assert!(result.iter().all(|check| check.status == CheckStatus::Pass));
    case.values = case.values.with_number("tw_sway_energy", 100.0).with_number("tw_sway_xi", 20.0);
    assert_eq!(evaluate_design_case(&case, AnnexChoice::En)[0].status, CheckStatus::Fail);
    case.values = case.values.with_text("tw_damping_source", "experimental").with_number("xi_percent", 20.0);
    assert_eq!(evaluate_design_case(&case, AnnexChoice::En)[0].status, CheckStatus::Pass);
}

#[test]
fn the_elastic_spectrum_scales_the_short_period_ramp_from_the_ground_acceleration() {
    let entry = design_case::entry::<En1998Cases>("C98-02").unwrap();
    let mut case = design_case::seed::<En1998Cases>(&entry, "short-period");
    case.values = case.values.with_number("t_1", 0.05).with_number("a_g", 1.5).with_number("t_b", 0.15)
        .with_number("s_soil", 1.0).with_number("xi_percent", 20.0).with_number("mass", 10000.0);
    let expected = 15000.0 * (1.0 + (2.5 * (10.0_f64 / 25.0).sqrt() - 1.0) / 3.0);
    assert!((evaluate_design_case(&case, AnnexChoice::En)[0].computed.value - expected).abs() < 1e-8);
}

#[test]
fn mast_cable_applies_the_strict_preload_boundary_and_reports_stiffness() {
    use crate::part_6::guy_cable_stiffness_n_m;
    use semio_s_plugin_norm_compliance::CheckStatus;
    for line in include_str!("guy-cable-stiffness.tsv").lines().skip(1) {
        let [length, area, angle, modulus, wrap, weight, stress, expected] =
            line.split('\t').map(|v| v.parse::<f64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
        let result = guy_cable_stiffness_n_m(length, area, angle, modulus, wrap, weight, stress).unwrap();
        assert!((result - expected).abs() < 1e-7);
    }
    let entry = design_case::entry::<En1998Cases>("C98-10").unwrap();
    let mut case = design_case::seed::<En1998Cases>(&entry, "guyed");
    let report = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(report[0].status, CheckStatus::Pass);
    assert_eq!(report[2].status, CheckStatus::Info);
    assert!((report[2].computed.value - 3_333_333.333_333_334).abs() < 1e-7);
    case.values = case.values.with_number("guy_seismic_stress", 250e6);
    assert_eq!(evaluate_design_case(&case, AnnexChoice::En)[0].status, CheckStatus::Fail);
    case.values = case.values.with_number("guy_mast_height", 40.0);
    assert_eq!(evaluate_design_case(&case, AnnexChoice::En)[0].status, CheckStatus::NotApplicable);
    case.values = case.values.with_text("guy_sag_model", "sag");
    let result = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(result[0].status, CheckStatus::Fail);
    assert!(result[2].computed.value < report[2].computed.value);
}

#[test]
fn buried_pipe_strain_verification_requires_a_wave_basis_and_sufficient_bond() {
    let entry = design_case::entry::<En1998Cases>("C98-11").unwrap();
    let mut case = En1998Cases::new_document_case(&entry, "pipe");
    assert_eq!(evaluate_design_case(&case, AnnexChoice::En)[0].status, CheckStatus::NotApplicable);
    case.values = case.values.with_text("pipe_wave_basis", "stable-welded");
    let report = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(report.len(), 3);
    assert!(report.iter().all(|result| result.status == CheckStatus::Pass));
    case.values = case.values.with_number("pipe_available_shear", 0.0);
    let report = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].status, CheckStatus::Fail);
}
