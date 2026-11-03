use semio_s_artifact_norm_en1991_checks::{part_1_4, part_2};
use semio_s_artifact_norm_en1991_examples::example_subjects::{de_accidental_variants, de_bridge_compliant, de_fire_parametric_compliant, de_office_compliant, multi_fail_noncompliant};
use semio_s_artifact_norm_en1991_inferences::evaluate;
use semio_s_artifact_norm_en1991_model::{AccidentalCase, AccidentalImpact, En1991Snapshot, FireMode, WindFace};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckStatus};

fn find<'a>(report: &'a CheckReport, id: &str) -> &'a semio_s_plugin_norm_compliance::CheckResult { report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}: {:?}", report.checks.iter().map(|c| &c.id).collect::<Vec<_>>())) }

#[semio_framework_async_macros::async_test]
async fn unknown_inputs_fail_with_a_remedy_instead_of_a_silent_default() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.wind_zone = 0;
    doc.snow_zone = "9".into();
    doc.construction_activity = "dancing".into();
    doc.thermal_element_type = "bridge7".into();
    doc.floors[0].category = "Q9".into();
    doc.self_weight_elements[0].material = "unobtainium".into();
    let report = evaluate(&doc);
    for id in ["en1991.input.windZone", "en1991.input.snowZone", "en1991.input.constructionActivity", "en1991.input.thermalElementType", "en1991.input.floors.office-l1.category", "en1991.input.selfWeightElements.slab-rc.material"] {
        let check = find(&report, id);
        assert_eq!(check.status, CheckStatus::Fail, "{id}");
        assert!(!check.remedies.is_empty(), "{id}");
    }
    assert!(report.checks.iter().all(|c| !c.id.starts_with("en1991.1-4.wind.")), "no wind pressure from an unknown zone");
    assert!(report.checks.iter().all(|c| c.id != "en1991.1-1.imposed.office-l1"));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn terrain_fire_curve_group_and_class_inputs_are_validated() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.terrain_category = 9;
    assert_eq!(find(&evaluate(&doc), "en1991.input.terrainCategory").status, CheckStatus::Fail);
    let mut fire = de_fire_parametric_compliant();
    fire.fire_mode = FireMode::Nominal;
    fire.fire_curve = semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::Parametric;
    assert_eq!(find(&evaluate(&fire), "en1991.input.fireCurve").status, CheckStatus::Fail);
    let mut bridge = de_bridge_compliant();
    bridge.bridge_load_group = String::new();
    assert_eq!(find(&evaluate(&bridge), "en1991.input.bridgeLoadGroup").status, CheckStatus::Fail);
    let mut crane = En1991Snapshot::example(AnnexChoice::De);
    crane.crane_claimed = true;
    crane.crane_class = "HC9".into();
    assert_eq!(find(&evaluate(&crane), "en1991.input.craneClass").status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn wind_suction_is_checked_with_the_signed_coefficient_and_both_internal_pressures() {
    let doc = de_office_compliant();
    let mut suction = doc.clone();
    suction.wind_faces = vec![WindFace { id: "facade-a".into(), zone: "A".into(), z: 8.0, c_pe10: -1.2, c_pe1: -1.4, c_pi: 0.2, c_s: 1.0, c_d: 1.0, loaded_area: 10.0, assumed_wp: 10_000.0 }];
    let report = evaluate(&suction);
    let check = find(&report, "en1991.1-4.wind.facade-a");
    let ze = suction.height;
    let qp = part_1_4::peak_velocity_pressure_pa(AnnexChoice::De, suction.wind_zone, suction.terrain_category, ze, 25.0, suction.air_density, suction.orography_factor, suction.mixed_terrain_upwind, suction.mixed_terrain_distance, suction.coast_or_island).expect("valid site");
    assert!((check.computed.value - qp * 1.4).abs() < 1e-6 * qp, "{} vs {}", check.computed.value, qp * 1.4);
    assert!(check.computed.value > qp * 1.2);
}

#[semio_framework_async_macros::async_test]
async fn failing_checks_exceed_one_and_warning_is_reachable() {
    let report = evaluate(&multi_fail_noncompliant());
    for check in report.checks.iter().filter(|c| c.status == CheckStatus::Fail) { assert!(!check.remedies.is_empty(), "{}", check.id); }
    for check in report.checks.iter().filter(|c| c.status == CheckStatus::Fail) { assert!(check.utilization > 1.0 || check.id.starts_with("en1991.input."), "{} has utilisation {}", check.id, check.utilization); }
    assert!(report.worst_utilization() > 1.0, "{}", report.worst_utilization());
    let default = evaluate(&En1991Snapshot::example(AnnexChoice::De));
    assert!(default.summary.warning >= 1, "partitions are assumed exactly at the allowance, which must read as a warning");
    assert!(default.checks.iter().any(|c| c.status == CheckStatus::Warning && c.utilization > 0.95 && c.utilization <= 1.0));
}

#[semio_framework_async_macros::async_test]
async fn every_impact_and_explosion_of_a_case_is_evaluated() {
    let mut doc = de_accidental_variants();
    doc.accidental_cases = vec![AccidentalCase {
        id: "multi".into(),
        impact: vec![AccidentalImpact { vehicle_mass: 1500.0, vehicle_speed: 15.0, assumed_force: 400_000.0 }, AccidentalImpact { vehicle_mass: 30_000.0, vehicle_speed: 10.0, assumed_force: 10_000.0 }],
        explosion: vec![],
    }];
    let report = evaluate(&doc);
    assert_eq!(find(&report, "en1991.1-7.impact.multi.0").status, CheckStatus::Pass);
    let second = find(&report, "en1991.1-7.impact.multi.1");
    assert_eq!(second.status, CheckStatus::Fail);
    assert!(!second.remedies.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn load_group_two_is_evaluated_and_a_non_governing_group_warns() {
    let mut doc = de_bridge_compliant();
    doc.bridge_load_group = "gr2".into();
    doc.assumed_bridge_footway = 5000.0;
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id == "en1991.2.group.selected"));
    assert!(part_2::group_members("gr2").is_some());
    let mut weak = de_bridge_compliant();
    weak.bridge_load_group = "gr3".into();
    let check = find(&evaluate(&weak), "en1991.2.group.selected").clone();
    assert_ne!(check.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn zero_degrees_is_a_valid_temperature_input() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.t_max = 0.0;
    doc.t_min = 0.0;
    doc.t_0 = 10.0;
    doc.assumed_delta_t = 10.0;
    let check = find(&evaluate(&doc), "en1991.1-5.thermal").clone();
    assert!((check.computed.value - 10.0).abs() < 1e-9, "{}", check.computed.value);
}

#[semio_framework_async_macros::async_test]
async fn parametric_fire_reports_validity_and_uses_the_full_curve() {
    let report = evaluate(&de_fire_parametric_compliant());
    assert!(report.checks.iter().all(|c| c.status != CheckStatus::Fail), "{:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
    let gas = find(&report, "en1991.1-2.gas-temperature");
    assert!(gas.computed.value > 700.0 + 273.15 && gas.computed.value < 1300.0 + 273.15);
    assert!(report.checks.iter().any(|c| c.id == "en1991.1-2.validity"));
}

#[semio_framework_async_macros::async_test]
async fn nominal_fire_uses_the_iso_834_curve() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.fire_mode = FireMode::Nominal;
    doc.fire_duration = 3600.0;
    doc.assumed_gas_temperature = 1300.0;
    let check = find(&evaluate(&doc), "en1991.1-2.gas-temperature").clone();
    assert!((check.computed.value - (945.3 + 273.15)).abs() < 0.1, "{}", check.computed.value);
    assert_eq!(check.status, CheckStatus::Pass);
    doc.assumed_gas_temperature = 1000.0;
    assert_eq!(find(&evaluate(&doc), "en1991.1-2.gas-temperature").status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn rail_load_models_are_checked_with_an_unverified_basis_and_the_tandem_must_stand_on_the_deck() {
    let mut doc = de_bridge_compliant();
    doc.rail_alpha = 1.21;
    let report = evaluate(&doc);
    let axle = find(&report, "en1991.2.rail.lm71-axle");
    assert!((axle.limit.value - 250_000.0).abs() < 1e-6 || (axle.computed.value - 1.21 * 250_000.0).abs() < 1e-6, "{axle:?}");
    assert_eq!(axle.status, CheckStatus::Fail);
    doc.assumed_rail_lm71 = 1.21 * 250_000.0;
    let covered = find(&evaluate(&doc), "en1991.2.rail.lm71-axle").clone();
    assert_eq!(covered.status, CheckStatus::UnverifiedBasis);
    assert!(covered.unverified_basis());
    doc.rail_alpha = 1.05;
    assert_eq!(find(&evaluate(&doc), "en1991.input.railAlpha").status, CheckStatus::Fail);
    let mut off = de_bridge_compliant();
    off.bridge_tandem_x = off.bridge_span + 5.0;
    let check = find(&evaluate(&off), "en1991.2.lm1-tandem-position").clone();
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(!check.remedies.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn crane_runway_and_silo_geometry_are_checked_with_remedies_and_flagged_unverified_values() {
    let mut doc = En1991Snapshot::example(AnnexChoice::De);
    doc.crane_claimed = true;
    doc.silo_claimed = true;
    let report = evaluate(&doc);
    assert_eq!(find(&report, "en1991.3.crane-position").status, CheckStatus::Pass);
    doc.crane_x = doc.crane_runway_length - doc.crane_wheel_base / 4.0;
    let off = find(&evaluate(&doc), "en1991.3.crane-position").clone();
    assert_eq!(off.status, CheckStatus::Fail);
    assert!(!off.remedies.is_empty());
    doc.crane_x = 15.0;
    doc.crane_wheel_base = doc.crane_runway_length + 1.0;
    assert_eq!(find(&evaluate(&doc), "en1991.3.crane-position").status, CheckStatus::Fail);
    let mut silo = En1991Snapshot::example(AnnexChoice::De);
    silo.silo_claimed = true;
    assert_eq!(find(&evaluate(&silo), "en1991.4.silo-hydraulic-radius").status, CheckStatus::Pass);
    silo.silo_diameter = 8.0;
    let mismatch = find(&evaluate(&silo), "en1991.4.silo-hydraulic-radius").clone();
    assert_eq!(mismatch.status, CheckStatus::Warning);
    assert!(!mismatch.remedies.is_empty());
    silo.silo_eccentricity = silo.silo_diameter;
    let eccentric = find(&evaluate(&silo), "en1991.4.silo-eccentricity").clone();
    assert_eq!(eccentric.status, CheckStatus::Fail);
    assert!(!eccentric.remedies.is_empty());
    silo.silo_hopper_angle_deg = 95.0;
    assert_eq!(find(&evaluate(&silo), "en1991.4.silo-hopper").status, CheckStatus::Fail);
}
