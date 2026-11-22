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
    concrete_jacket_corrections => "C98-15",
    steel_jacket_shear => "C98-16",
    steel_jacket_splice_clamping => "C98-17",
    frp_concrete_member_shear => "C98-18",
    welded_pipe_under_differential_ground_movement => "C98-19",
    concrete_pipe_strain_and_leak_tightness => "C98-20",
    buried_pipe_joint_combination => "C98-21",
    pipe_fault_crossing_design_measures => "C98-22",
    horizontal_silo_wall_pressure => "C98-23",
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

#[test]
fn differential_pipe_checks_require_evidence_and_report_each_limit() {
    let entry = design_case::entry::<En1998Cases>("C98-19").unwrap();
    let case = En1998Cases::new_document_case(&entry, "differential");
    assert_eq!(evaluate_design_case(&case, AnnexChoice::En)[0].status, CheckStatus::UnverifiedBasis);
    let keys = ["radius", "thickness", "tension", "compression", "global"];
    let fixtures = include_str!("../../../../../🧪️tests/🧫️fixtures/📏️differential-pipe/🔣️.tsv");
    for line in fixtures.lines().skip(1) {
        let parts = line.split('\t').collect::<Vec<_>>();
        let mut tested = case.clone();
        tested.values = tested.values.with_text("pipe_deformation_basis", parts[1]);
        for (key, value) in keys.iter().zip(&parts[2..7]) {
            let key = format!("pipe_deformation_{key}");
            if value.is_empty() {
                tested.values.numbers.remove(&key);
            } else {
                tested.values = tested.values.with_number(&key, value.parse::<f64>().unwrap());
            }
        }
        let results = evaluate_design_case(&tested, AnnexChoice::En);
        let statuses: String = results.iter().map(|row| match row.status {
            CheckStatus::Pass => '1',
            CheckStatus::Fail => '0',
            CheckStatus::UnverifiedBasis => 'U',
            other => panic!("Unexpected {other:?} for {}", parts[0]),
        }).collect();
        assert_eq!(statuses, parts[7], "{}", parts[0]);
    }
}

#[test]
fn concrete_pipe_and_joint_assessments_match_the_published_case_fixtures() {
    for (entry_id, evidence, fixture) in [
        ("C98-20", "pipe_concrete_basis", include_str!("../../../../../🧪️tests/🧫️fixtures/🧱️concrete-pipes/🧱️concrete.tsv")),
        ("C98-21", "pipe_joint_basis", include_str!("../../../../../🧪️tests/🧫️fixtures/🧱️concrete-pipes/🔗️joints.tsv")),
    ] {
        let entry = design_case::entry::<En1998Cases>(entry_id).unwrap();
        let mut lines = fixture.lines();
        let columns = lines.next().unwrap().split('\t').collect::<Vec<_>>();
        let keys = &columns[2..columns.len()-1];
        let initial = En1998Cases::new_document_case(&entry, entry_id);
        assert_eq!(evaluate_design_case(&initial, AnnexChoice::En)[0].status, CheckStatus::UnverifiedBasis);
        for line in lines {
            let values = line.split('\t').collect::<Vec<_>>();
            let mut case = initial.clone();
            case.values = case.values.with_text(evidence, values[1]);
            for (key, value) in keys.iter().zip(&values[2..values.len()-1]) {
                if value.is_empty() {
                    case.values.numbers.remove(*key);
                } else {
                    case.values = case.values.with_number(*key, value.parse().unwrap());
                }
            }
            let checks = evaluate_design_case(&case, AnnexChoice::En);
            let flags: String = checks.iter().map(|result| match result.status {
                CheckStatus::Pass => '1',
                CheckStatus::Fail => '0',
                CheckStatus::UnverifiedBasis => 'U',
                other => panic!("Unexpected {other:?} for {}", values[0]),
            }).collect();
            assert_eq!(flags, *values.last().unwrap(), "{}", values[0]);
        }
    }
}

#[test]
fn fault_crossing_assesses_the_two_sides_independently() {
    let entry = design_case::entry::<En1998Cases>("C98-22").unwrap();
    let initial = En1998Cases::new_document_case(&entry, "crossing");
    assert_eq!(evaluate_design_case(&initial, AnnexChoice::En)[0].status, CheckStatus::UnverifiedBasis);
    let mut records = include_str!("../../../../../🧪️tests/🧫️fixtures/🛤️fault-crossing.tsv").lines();
    let columns: Vec<_> = records.next().unwrap().split('\t').collect();
    for line in records {
        let fields: Vec<_> = line.split('\t').collect();
        let mut case = initial.clone();
        case.values = case.values.with_text(columns[1], fields[1]).with_text(columns[2], fields[2]);
        for (key, value) in columns[3..9].iter().zip(&fields[3..9]) {
            if value.is_empty() {
                case.values.numbers.remove(*key);
            } else {
                case.values = case.values.with_number(*key, value.parse().unwrap());
            }
        }
        let results = evaluate_design_case(&case, AnnexChoice::En);
        let flags: String = results.iter().map(|result| match result.status {
            CheckStatus::Pass => '1',
            CheckStatus::Fail => '0',
            CheckStatus::UnverifiedBasis => 'U',
            CheckStatus::Info => 'I',
            other => panic!("Unexpected {other:?} for {}", fields[0]),
        }).collect();
        assert_eq!(flags, fields[9], "{}", fields[0]);
    }
}

#[test]
fn silo_pressure_reports_a_signed_increment_and_refuses_suction() {
    let entry = design_case::entry::<En1998Cases>("C98-23").unwrap();
    let mut case = En1998Cases::new_document_case(&entry, "silo");
    assert_eq!(evaluate_design_case(&case, AnnexChoice::En)[0].status, CheckStatus::UnverifiedBasis);
    case.values = case.values.with_text("silo_pressure_basis", "confirmed")
        .with_number("silo_pressure_azimuth", 180.0);
    let rows = evaluate_design_case(&case, AnnexChoice::En);
    assert!(rows.iter().all(|row| row.status == CheckStatus::Info));
    assert_eq!(rows.iter().map(|row| row.computed.value).collect::<Vec<_>>(), [8000.0, -8000.0, 2000.0]);

    case.values = case.values.with_number("silo_pressure_static", 7999.0);
    let rows = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].status, CheckStatus::UnverifiedBasis);
    assert!(rows[0].id.ends_with(".redistribution.silo"));

    case.values = case.values.with_text("silo_pressure_section", "hopper")
        .with_number("silo_pressure_height", 1.0)
        .with_number("silo_pressure_slope", 60.0)
        .with_number("silo_pressure_static", 20_000.0);
    let rows = evaluate_design_case(&case, AnnexChoice::En);
    assert!((rows[0].computed.value - 16_000.0).abs() < 1e-6);
    assert!((rows[2].computed.value - 4_000.0).abs() < 1e-6);
}
