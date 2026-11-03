//! 🏗️ Bridge, tank and tower response: the schema kernels against the independent scipy oracle, the checks with their remedies on the compliant structures subject, and the unverified constants they cite.

use crate::schema_compliance::{family_any_dir, find, oracle_values};
use semio_s_artifact_norm_en1998_inferences as inferences;
use semio_s_artifact_norm_en1998_model::{En1998Bearing, En1998Pier, En1998Snapshot, PierStiffnessBasis};
use semio_s_artifact_norm_en1998_schema::{bridge, part_1, part_2, part_3, part_4, tank, tower};
use semio_s_plugin_norm_compliance::CheckStatus;

fn close(py: &serde_json::Value, key: &str, rust: f64, relative: f64) {
    let expected = py.get(key).and_then(|v| v.as_f64()).unwrap_or_else(|| panic!("oracle key {key} missing"));
    assert!((expected - rust).abs() <= 1e-12 + relative * expected.abs(), "{key}: oracle={expected} rust={rust}");
}

fn oracle_bridge() -> (Vec<En1998Pier>, Vec<En1998Bearing>) {
    let pier = |id: &str, x_m: f64| En1998Pier { id: id.into(), bridge_id: "b".into(), x_m, height_m: 8.0, section_b_m: 2.0, section_h_m: 1.5, modulus_mpa: 30000.0, moment_rd_nm: 6.0e6, shear_rd_n: 1.5e6, reinforcement_ratio: 0.0, cover_m: 0.0, concrete_fck_mpa: 0.0, steel_fyk_mpa: 0.0, axial_load_n: 0.0, stiffness_basis: PierStiffnessBasis::Declared };
    let bearing = |id: &str, pier_id: &str, x_m: f64, k: f64| En1998Bearing { id: id.into(), bridge_id: "b".into(), pier_id: pier_id.into(), device: semio_s_artifact_norm_en1998_model::BearingDevice::Elastomeric, x_m, stiffness_n_per_m: k, d_rd_m: 0.1, seat_length_m: 0.6, min_support_length_m: 0.4 };
    (vec![pier("p1", 10.0), pier("p2", 30.0)], vec![bearing("a", "", 0.0, 2.0e7), bearing("p1a", "p1", 0.0, 5.0e7), bearing("p1b", "p1", 0.0, 5.0e7), bearing("p2a", "p2", 0.0, 5.0e7), bearing("p2b", "p2", 0.0, 5.0e7), bearing("b", "", 40.0, 2.0e7)])
}

#[semio_framework_async_macros::async_test]
async fn bridge_kernel_matches_the_independent_oracle() {
    use bridge::Direction::{Longitudinal, Transverse};
    let py = oracle_values();
    let (piers, bearings) = oracle_bridge();
    let supports = bridge::supports("b", &piers, &bearings, 1.0, &annex_a());
    assert_eq!(supports.iter().map(|s| s.x_m).collect::<Vec<_>>(), vec![0.0, 10.0, 30.0, 40.0]);
    close(&py, "br_k_pier_long", supports[1].pier_longitudinal_n_per_m, 1e-12);
    close(&py, "br_k_pier_trans", supports[1].pier_transverse_n_per_m, 1e-12);
    close(&py, "br_k_support_long", supports[1].stiffness_n_per_m(Longitudinal), 1e-12);
    let xs: Vec<f64> = supports.iter().map(|s| s.x_m).collect();
    let k_long: Vec<f64> = supports.iter().map(|s| s.stiffness_n_per_m(Longitudinal)).collect();
    let k_trans: Vec<f64> = supports.iter().map(|s| s.stiffness_n_per_m(Transverse)).collect();
    close(&py, "br_k_total_long", k_long.iter().sum(), 1e-12);
    let design = |t: f64| part_1::design_spectrum_at(0.6, 1.0, 1.25, 0.05, 0.25, 2.0, 1.5, t);
    let elastic = |t: f64| part_1::elastic_response_spectrum(0.6, 1.25, 0.05, 0.25, 2.0, t);
    let long = bridge::analyse(Longitudinal, 9.0e5, 40.0, 0.0, &xs, &k_long, &design, &elastic).expect("rigid deck");
    close(&py, "br_t_long", long.periods_s[0], 1e-12);
    close(&py, "br_v_long", long.base_shear_n, 1e-9);
    close(&py, "br_f_pier_long", long.support_force_n[1], 1e-9);
    close(&py, "br_u_long", long.support_displacement_m[1], 1e-9);
    let trans = bridge::analyse(Transverse, 9.0e5, 40.0, 5.0e11, &xs, &k_trans, &design, &elastic).expect("kernel modes");
    for (key, mode) in [("br_t_trans_1", 0), ("br_t_trans_2", 1), ("br_t_trans_3", 2)] {
        close(&py, key, trans.periods_s[mode], 1e-5);
    }
    close(&py, "br_meff_trans_1", trans.effective_mass_ratio[0], 1e-5);
    close(&py, "br_meff_trans_sum", trans.participation(), 1e-4);
    close(&py, "br_f_pier_trans", trans.support_force_n[1], 1e-4);
    close(&py, "br_f_abutment_trans", trans.support_force_n[0], 1e-4);
    close(&py, "br_u_pier_trans", trans.support_displacement_m[1], 1e-4);
    close(&py, "br_v_trans", trans.base_shear_n, 1e-4);
    let d_g = bridge::ground_displacement_m(0.025, 0.6, 1.25, 0.25, 2.0);
    close(&py, "br_d_g", d_g, 1e-12);
    close(&py, "br_seat", bridge::seat_length_m(0.4, 2.0, d_g, long.support_displacement_m[1]), 1e-9);
    close(&py, "br_m_ed_pier", bridge::pier_base_moment_nm(long.support_force_n[1], 8.0), 1e-9);
    close(&py, "br_v_cd", part_2::pier_capacity_design_shear_n(1.35, 6.0e6, 8.0), 1e-12);
    assert!((trans.base_shear_n - trans.support_force_n.iter().sum::<f64>()).abs() < 0.5 * trans.base_shear_n, "the combined support forces carry the base shear");
}

#[semio_framework_async_macros::async_test]
async fn tank_kernel_matches_the_independent_oracle_and_the_housner_masses() {
    let py = oracle_values();
    let (h, r, rho, a_i, a_c) = (9.0, 5.0, 649.0, 1.875, 0.2);
    close(&py, "tk_bessel_i1_5", tank::bessel_i(1, 5.0), 1e-12);
    close(&py, "tk_bessel_i1_12", tank::bessel_i(1, 12.0), 1e-12);
    close(&py, "tk_ratio_5", tank::bessel_ratio(5.0), 1e-12);
    close(&py, "tk_j1_1p2", tank::bessel_j1(1.2), 1e-12);
    close(&py, "tk_p_imp_0", tank::impulsive_series_pa(0.0, h, r, rho, a_i, 0.0), 1e-9);
    close(&py, "tk_p_imp_mid", tank::impulsive_series_pa(h / 2.0, h, r, rho, a_i, 0.0), 1e-9);
    close(&py, "tk_p_imp_45", tank::impulsive_series_pa(0.0, h, r, rho, a_i, std::f64::consts::FRAC_PI_4), 1e-9);
    close(&py, "tk_p_conv_top", tank::convective_pa(h, h, r, rho, a_c, 0.0), 1e-12);
    close(&py, "tk_p_conv_0", tank::convective_pa(0.0, h, r, rho, a_c, 0.0), 1e-12);
    close(&py, "tk_p_hydro_0", tank::hydrostatic_pa(0.0, h, rho), 1e-12);
    close(&py, "tk_mi_series", tank::impulsive_series_mass_ratio(h, r), 1e-7);
    close(&py, "tk_mc_series", tank::convective_series_mass_ratio(h, r), 1e-7);
    close(&py, "tk_mc_closed", tank::convective_series_mass_ratio(h, r), 1e-12);
    close(&py, "tk_slosh", tank::slosh_height_m(0.84, r, a_c), 1e-12);
    close(&py, "tk_eta_half", tank::surface_elevation_m(tank::slosh_height_m(0.84, r, a_c), r, 0.5 * r, std::f64::consts::FRAC_PI_3), 1e-9);
    close(&py, "tk_hoop_mpa", tank::hoop_stress_mpa(63.5e3, r, 0.01), 1e-12);
    close(&py, "tk_t_req", tank::required_thickness_m(63.5e3, r, 200.0), 1e-12);
    let housner_impulsive = part_4::impulsive_mass_ratio(h / r);
    assert!((housner_impulsive / tank::impulsive_series_mass_ratio(h, r) - 1.0).abs() < 0.08, "Housner impulsive mass within 8 % of the rigid-tank series");
    assert!((part_4::convective_mass_ratio(h / r) / tank::convective_series_mass_ratio(h, r) - 1.0).abs() < 0.02, "Housner convective mass within 2 % of the first sloshing mode (the old coefficient 0.230 per R/H was half of it)");
    let profile = tank::wall_profile(h, r, rho, a_i, a_c, 400);
    let resultant: f64 = profile.windows(2).map(|w| 0.5 * (w[0].impulsive_pa + w[1].impulsive_pa) * (w[1].z_m - w[0].z_m)).sum::<f64>() * std::f64::consts::PI * r;
    let mass = rho * std::f64::consts::PI * r * r * h;
    assert!((resultant / (mass * a_i) / housner_impulsive - 1.0).abs() < 1e-3, "the scaled wall pressure carries the Housner impulsive mass");
    assert!(tank::governing(&profile).is_some_and(|p| p.z_m == 0.0), "the total pressure peaks at the base");
}

#[semio_framework_async_macros::async_test]
async fn tower_kernel_matches_the_independent_oracle_with_and_without_the_foundation_spring() {
    let py = oracle_values();
    let (mass, height, ei) = (1.0e5, 45.0, 2.0e10);
    for (label, k_theta) in [("fixed", 0.0), ("spring", 1.0e10)] {
        let modes = tower::modes(mass, height, ei, k_theta).expect("kernel modes");
        for j in 0..3 {
            close(&py, &format!("tw_{label}_t_{}", j + 1), modes.modes[j].period_s, 1e-5);
            close(&py, &format!("tw_{label}_exact_{}", j + 1), modes.modes[j].period_s, 1.2e-2);
            close(&py, &format!("tw_{label}_meff_{}", j + 1), modes.modes[j].mass_ratio, 1e-4);
            close(&py, &format!("tw_{label}_gamma_{}", j + 1), modes.modes[j].gamma, 1e-4);
        }
    }
    close(&py, "tw_lengthening", tower::period_lengthening(mass, height, ei, 1.0e10).expect("ratio"), 1e-5);
    let modes = tower::modes(mass, height, ei, 1.0e10).expect("modes");
    let response = tower::respond(&modes, &|t| 0.75 * (0.25 / t.max(0.25)));
    let first = &modes.modes[0];
    let expected = first.mass_ratio * modes.node_masses_kg.iter().sum::<f64>() * 0.75 * (0.25 / first.period_s);
    assert!((response.modal_shear_n[0] - expected).abs() < 1e-6 * expected, "the modal base shear is the effective mass times the spectral value");
    assert!(response.overturning_nm >= response.modal_moment_nm[0] * 0.999, "the combination cannot fall below the dominant mode");
    let required = tower::stiffness_for_lengthening(mass, height, ei, 1.1).expect("a stiff enough spring exists");
    let ratio = tower::period_lengthening(mass, height, ei, required).expect("ratio");
    assert!((ratio - 1.1).abs() < 1e-3, "the required stiffness gives the limit: {ratio}");
}

fn structures() -> En1998Snapshot {
    En1998Snapshot::compliant_de_structures()
}

#[semio_framework_async_macros::async_test]
async fn the_structures_subject_complies_and_reports_every_new_check() {
    let report = inferences::evaluate(&structures());
    assert!(report.complies(), "fails: {:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| (&c.id, c.utilization)).collect::<Vec<_>>());
    for suffix in [
        ".analysis.massParticipation",
        ".pier.pier-1.flexure",
        ".pier.pier-1.shear",
        ".pier.pier-2.flexure",
        ".action.importance",
        ".pier.pier-1.behaviour",
        ".pier.pier-1.combination",
        ".bearing.bear-a.displacement",
        ".bearing.bear-1.seat",
        ".bearing.bear-b.seat",
        "tk-1.pressure",
        "tk-1.slosh",
        ".modal.massParticipation",
        ".modal.overturning",
        ".interaction",
    ] {
        assert!(report.checks.iter().any(|c| c.id.ends_with(suffix)), "{suffix}");
    }
    assert!(report.checks.iter().all(|c| !c.id.ends_with("en1998.2.br-1.bearing") && !c.id.ends_with("en1998.2.br-1.pier.shear")), "the single-pier checks give way to the rows");
}

fn failing(mutate: impl FnOnce(&mut En1998Snapshot), suffix: &str, path_suffix: &str) -> (En1998Snapshot, semio_s_plugin_norm_compliance::CheckResult) {
    let mut snap = structures();
    mutate(&mut snap);
    let report = inferences::evaluate(&snap);
    let check = find(&report, suffix).clone();
    assert_eq!(check.status, CheckStatus::Fail, "{suffix}");
    assert!(check.remedies.iter().any(|r| r.target.path.ends_with(path_suffix)), "{suffix} has a remedy on {path_suffix}: {:?}", check.remedies.iter().map(|r| &r.target.path).collect::<Vec<_>>());
    (snap, check)
}

#[semio_framework_async_macros::async_test]
async fn bridge_checks_fail_with_remedies_on_the_rows() {
    let (_, flexure) = failing(|s| s.piers[0].moment_rd_nm = 1.0e5, ".pier.pier-1.flexure", ".momentRdNm");
    let required = flexure.remedies[0].required.value;
    let mut snap = structures();
    snap.piers[0].moment_rd_nm = required * 1.001;
    assert_ne!(find(&inferences::evaluate(&snap), ".pier.pier-1.flexure").status, CheckStatus::Fail, "the remedy value complies");
    failing(|s| s.piers[0].shear_rd_n = 1.0e3, ".pier.pier-1.shear", ".shearRdN");
    failing(|s| s.bearings[0].d_rd_m = 1.0e-4, ".bearing.bear-a.displacement", ".dRdM");
    let (_, seat) = failing(|s| s.bearings[1].seat_length_m = 0.3, ".bearing.bear-1.seat", ".seatLengthM");
    assert!(seat.ndps.iter().any(|id| id == "ndp.en1998.en2.seat.ground-displacement-factor"));
}

#[semio_framework_async_macros::async_test]
async fn bridge_action_rules_match_the_independent_oracle() {
    use bridge::Direction::Longitudinal;
    let py = oracle_values();
    close(&py, "br_gamma_i_iii", part_2::bridge_gamma_i(3.0), 1e-12);
    assert_eq!(part_2::bridge_gamma_i(2.0), 1.0);
    assert_eq!(part_2::bridge_gamma_i(9.0), part_2::bridge_gamma_i(3.0), "classes above III are capped");
    let a_g = 1.3 * 0.6;
    for (key, t) in [("br_s_ve_005", 0.05), ("br_s_ve_010", 0.10), ("br_s_ve_050", 0.5), ("br_s_ve_200", 2.0)] {
        close(&py, key, part_2::bridge_vertical_spectrum(a_g, 0.7, t), 1e-9);
    }
    close(&py, "br_alpha_s", part_2::shear_span_ratio(8.0, 2.0), 1e-12);
    close(&py, "br_q_alpha_2", part_2::rc_pier_behaviour_factor(3.5, 2.0, 3.0, 1.0).expect("ductile"), 1e-12);
    close(&py, "br_q_alpha_5", part_2::rc_pier_behaviour_factor(3.5, 5.0, 3.0, 1.0).expect("full"), 1e-12);
    close(&py, "br_q_alpha_1", part_2::rc_pier_behaviour_factor(3.5, 1.0, 3.0, 1.0).expect("lower bound"), 1e-12);
    assert!(part_2::rc_pier_behaviour_factor(3.5, 0.9, 3.0, 1.0).is_none(), "a squat pier is outside the rule");
    close(&py, "br_combined", part_2::component_combination(5.0e6, -2.0e6, 1.0e6, 0.3), 1e-12);
    let (piers, bearings) = oracle_bridge();
    let gross = bridge::supports("b", &piers, &bearings, 1.0, &annex_a());
    let cracked = bridge::supports("b", &piers, &bearings, 0.5, &annex_a());
    close(&py, "br_k_cracked_half", cracked[1].pier_longitudinal_n_per_m, 1e-12);
    assert!(cracked[1].stiffness_n_per_m(Longitudinal) < gross[1].stiffness_n_per_m(Longitudinal), "the cracked pier is softer in series with the bearing");
    assert_eq!(bridge::supports("b", &piers, &bearings, 0.0, &annex_a())[1].pier_longitudinal_n_per_m, 0.0, "zero leaves the pier without stiffness, never the gross section");
    assert_eq!(bridge::supports("b", &piers, &bearings, 2.0, &annex_a())[1].pier_longitudinal_n_per_m, gross[1].pier_longitudinal_n_per_m, "a share above one is capped at the gross section");
}

#[semio_framework_async_macros::async_test]
async fn bridge_action_checks_follow_the_class_the_shear_span_and_the_components() {
    let report = inferences::evaluate(&structures());
    let importance = find(&report, ".action.importance");
    assert!(importance.ndps.iter().any(|n| n == "ndp.en1998.en2.gamma-i.ii"));
    let mut snap = structures();
    snap.bridges[0].importance_class = 3.0;
    let class_iii = inferences::evaluate(&snap);
    assert!(find(&class_iii, ".action.importance").ndps.iter().any(|n| n == "ndp.en1998.en2.gamma-i.iii"));
    assert!(find(&class_iii, ".bearing.bear-a.displacement").utilization > find(&report, ".bearing.bear-a.displacement").utilization, "gamma_I = 1.3 raises the bearing displacement");
    failing(|s| s.bridges[0].importance_class = 0.0, ".action.importance", ".importanceClass");
    let (_, squat) = failing(|s| s.piers[0].height_m = 1.5, ".pier.pier-1.behaviour", ".heightM");
    assert!(squat.ndps.iter().any(|n| n == "ndp.en1998.en2.q.shear-span-min"));
    failing(|s| s.piers[0].moment_rd_nm = 1.0e5, ".pier.pier-1.combination", ".momentRdNm");
    failing(|s| s.bridges[0].pier_cracked_inertia_ratio = 0.0, ".action.stiffness", ".pierCrackedInertiaRatio");
    let (_, over) = failing(|s| s.bridges[0].pier_cracked_inertia_ratio = 1.5, ".action.stiffness", ".pierCrackedInertiaRatio");
    assert_eq!(over.remedies[0].required.value, 1.0);
    assert!(report.checks.iter().all(|c| !c.id.ends_with(".action.vertical")), "no uplift rule is modelled: the vertical spectrum is a diagram, not a check");
}

#[semio_framework_async_macros::async_test]
async fn bridge_rows_are_checked_for_dangling_references() {
    let mut snap = structures();
    snap.bearings[1].pier_id = "no-such-pier".into();
    snap.piers[1].bridge_id = "no-such-bridge".into();
    let report = inferences::evaluate(&snap);
    assert_eq!(find(&report, "bearings.bear-1.pierId").status, CheckStatus::Fail);
    assert_eq!(find(&report, "piers.pier-2.bridgeId").status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn tank_checks_fail_with_remedies_and_the_thickness_remedy_complies() {
    let (_, pressure) = failing(|s| s.tanks[0].shell_thickness_m = 1.0e-4, "tk-1.pressure", ".shellThicknessM");
    let required = pressure.remedies[0].required.value;
    let mut snap = structures();
    snap.tanks[0].shell_thickness_m = required * 1.001;
    assert_eq!(find(&inferences::evaluate(&snap), "tk-1.pressure").status, CheckStatus::Pass);
    let (_, slosh) = failing(|s| s.tanks[0].filling_ratio = 0.999, "tk-1.slosh", ".fillingRatio");
    assert!(slosh.remedies[0].required.value < 0.999);
    assert!(slosh.ndps.iter().any(|id| id == "ndp.en1998.en4.tank.slosh-height-coefficient"));
    let mut bare = structures();
    bare.tanks[0].shell_thickness_m = 0.0;
    let report = inferences::evaluate(&bare);
    assert_eq!(find(&report, "tk-1.pressure").status, CheckStatus::Info, "without shell data the pressure is reported, not judged");
}

#[semio_framework_async_macros::async_test]
async fn tower_checks_fail_with_remedies_and_the_stiffness_remedy_complies() {
    failing(|s| s.towers[0].m_rd_nm = 1.0e3, "tw-1.modal.overturning", ".mRdNm");
    let (_, interaction) = failing(|s| s.towers[0].base_rotational_stiffness_nm = 1.0e8, "tw-1.interaction", ".baseRotationalStiffnessNm");
    let required = interaction.remedies[0].required.value;
    assert!(required > 1.0e8);
    let mut snap = structures();
    snap.towers[0].base_rotational_stiffness_nm = required * 1.01;
    assert_ne!(find(&inferences::evaluate(&snap), "tw-1.interaction").status, CheckStatus::Fail);
    let fixed = {
        let mut s = structures();
        s.towers[0].base_rotational_stiffness_nm = 0.0;
        inferences::evaluate(&s)
    };
    assert!(fixed.checks.iter().all(|c| !c.id.ends_with("tw-1.interaction")), "a fixed base has no interaction check");
    let (flexible, rigid) = (find(&inferences::evaluate(&structures()), "tw-1.period").computed.value, find(&fixed, "tw-1.period").computed.value);
    assert!(flexible > rigid, "the foundation spring lengthens the fundamental period");
}

#[semio_framework_async_macros::async_test]
async fn the_structure_constants_are_registered_unverified_and_cited_by_their_checks() {
    let report = inferences::evaluate(&structures());
    for (suffix, id) in [
        (".analysis.massParticipation", "ndp.en1998.en1.modal.min-mass-participation"),
        (".bearing.bear-1.seat", "ndp.en1998.en2.seat.ground-displacement-coefficient"),
        (".pier.pier-1.shear", "ndp.en1998.en2.pier.gamma-0"),
        (".pier.pier-1.behaviour", "ndp.en1998.en2.q.rc-pier"),
        (".pier.pier-1.combination", "ndp.en1998.en2.combination.minor-factor"),
        ("tk-1.slosh", "ndp.en1998.en4.tank.slosh-height-coefficient"),
        ("tw-1.modal.massParticipation", "ndp.en1998.en1.modal.min-mass-participation"),
        ("tw-1.interaction", "ndp.en1998.en6.interaction.max-period-lengthening"),
    ] {
        assert!(find(&report, suffix).ndps.iter().any(|n| n == id), "{suffix} cites {id}");
        let entry = semio_s_plugin_norm_compliance::ndp(id).expect("registered");
        assert_eq!(entry.status, semio_s_plugin_norm_compliance::NdpStatus::Unverified, "{id} stays unverified until read from the printed standard");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_new_checks_are_traced_in_the_spec_table() {
    use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, standing, Emitted};
    let report = inferences::evaluate(&structures());
    let base = inferences::evaluate(&En1998Snapshot::compliant_de_multipart());
    let assessed = inferences::evaluate(&En1998Snapshot::full_showcase());
    let emitted = Emitted::default().with_checks(report.checks.iter().chain(base.checks.iter()).chain(assessed.checks.iter()).map(|c| c.id.clone()));
    assert_traced(&inferences::spec_trace::TABLE, &emitted);
    let overview = standing(&inferences::spec_trace::TABLE, &report);
    for id in ["BR-01", "BR-03", "BR-04", "BR-06", "BR-07", "BR-08", "BR-09", "ST-02", "TW-02", "TW-03", "TW-04"] {
        assert!(overview.iter().any(|row| row.id == id && row.checks > 0), "{id} is traced to emitted checks");
    }
}

#[semio_framework_async_macros::async_test]
async fn accidental_torsion_amplifies_the_storey_shear() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let sid = snap.buildings[0].systems[0].id.clone();
    let storey = snap.buildings[0].storeys[1].id.clone();
    let suffix = format!(".{sid}.storeyForces.{storey}");
    snap.buildings[0].storeys[1].centre_of_stiffness_x_m += 2.0;
    snap.buildings[0].storeys[1].centre_of_stiffness_y_m += 2.0;
    let skewed = inferences::evaluate(&snap);
    let delta: f64 = find(&skewed, &format!(".{sid}.torsion")).computed.value;
    snap.buildings[0].storeys[1].centre_of_stiffness_x_m -= 2.0;
    snap.buildings[0].storeys[1].centre_of_stiffness_y_m -= 2.0;
    let centred = inferences::evaluate(&snap);
    let (a, b) = (find(&skewed, &suffix).computed.value, find(&centred, &suffix).computed.value);
    let torsion_centred = find(&centred, &format!(".{sid}.torsion")).computed.value;
    assert!(torsion_centred >= 1.0 && delta >= torsion_centred, "{delta} {torsion_centred}");
    assert!((a / b - delta / torsion_centred).abs() < 1e-6, "the storey shear carries the torsion factor: {a} {b} {delta} {torsion_centred}");
}

#[semio_framework_async_macros::async_test]
async fn plan_dimensions_status_follows_the_data_and_elevation_regularity_needs_no_plan() {
    let mut snap = En1998Snapshot::compliant_de_multipart();
    assert_eq!(find(&inferences::evaluate(&snap), ".planDims").status, CheckStatus::Pass);
    snap.buildings[0].plan_width_m = 0.0;
    let report = inferences::evaluate(&snap);
    assert_eq!(find(&report, ".planDims").status, CheckStatus::Fail);
    assert!(!find(&report, ".planDims").remedies.is_empty());
    assert!(report.checks.iter().any(|c| c.id.contains(".elevationRegularity.")), "emitted without plan dimensions");
}

#[semio_framework_async_macros::async_test]
async fn plan_regularity_compares_each_direction_with_its_own_torsional_radius() {
    assert!((part_1::plan_eccentricity_utilization(1.0, 0.0, 10.0, 2.0) - 1.0 / 3.0).abs() < 1e-12, "e0x pairs with rx only");
    assert!((part_1::plan_eccentricity_utilization(0.0, 1.0, 10.0, 2.0) - 1.0 / 0.6).abs() < 1e-12, "e0y pairs with ry only");
    assert!((part_1::plan_eccentricity_utilization(1.0, 0.5, 10.0, 2.0) - 0.5 / 0.6).abs() < 1e-12, "the larger of the two ratios governs");
    assert_eq!(part_1::plan_eccentricity_utilization(1.0, 0.0, 0.0, 2.0), 1e9);
    let mut snap = En1998Snapshot::compliant_de_multipart();
    let storey = snap.buildings[0].storeys[0].id.clone();
    let suffix = format!(".storey.{storey}.planRegularity");
    snap.buildings[0].storeys[0].stiffness_x *= 4.0;
    snap.buildings[0].storeys[0].centre_of_stiffness_x_m = snap.buildings[0].storeys[0].centre_of_mass_x_m + 0.25;
    snap.buildings[0].storeys[0].centre_of_stiffness_y_m = snap.buildings[0].storeys[0].centre_of_mass_y_m;
    let s = &snap.buildings[0].storeys[0];
    let b = &snap.buildings[0];
    let (e0x, e0y, rx, ry, ls) = part_1::plan_regularity_metrics(s.centre_of_mass_x_m, s.centre_of_mass_y_m, s.centre_of_stiffness_x_m, s.centre_of_stiffness_y_m, s.stiffness_x, s.stiffness_y, b.plan_width_m, b.plan_length_m);
    let expected = (e0x / (0.30 * rx)).max(e0y / (0.30 * ry)).max(ls / rx.min(ry));
    let report = inferences::evaluate(&snap);
    let check = find(&report, &suffix);
    assert!((check.computed.value / check.limit.value - expected).abs() < 1e-9, "{} {expected}", check.computed.value);
}

fn annex_a() -> part_3::AnnexA {
    part_3::AnnexA::from_registry(semio_s_plugin_norm_compliance::AnnexChoice::En)
}

fn computed_pier() -> En1998Pier {
    En1998Pier { id: "p".into(), bridge_id: "b".into(), x_m: 10.0, height_m: 8.0, section_b_m: 0.8, section_h_m: 1.2, modulus_mpa: 33000.0, moment_rd_nm: 6.0e6, shear_rd_n: 1.5e6, reinforcement_ratio: 0.008, cover_m: 0.08, concrete_fck_mpa: 30.0, steel_fyk_mpa: 500.0, axial_load_n: 1.0e6, stiffness_basis: PierStiffnessBasis::Computed }
}

fn pier_oracle() -> std::collections::BTreeMap<String, f64> {
    let script = family_any_dir().join("🔮️oracles/🌉️pier-stiffness/🐍️.py");
    let out = semio_s_plugin_norm_compliance::python_oracle::run_ok(&script, &[], None);
    String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect()
}

#[semio_framework_async_macros::async_test]
async fn the_computed_pier_stiffness_is_my_over_phi_y_of_the_fibre_section_and_matches_concreteproperties() {
    use bridge::Direction::Longitudinal;
    let (k, oracle) = (annex_a(), pier_oracle());
    for (name, n) in [("pier_n1000", 1.0e6), ("pier_n0", 0.0)] {
        let pier = En1998Pier { axial_load_n: n, ..computed_pier() };
        let response = bridge::pier_section(&pier, Longitudinal).section_response(&k).expect("response");
        let ei = bridge::pier_section(&pier, Longitudinal).effective_stiffness_nm2(&k).expect("EI_eff");
        assert!((ei - response.m_y / response.kappa_y).abs() <= 1e-9 * ei, "EI_eff is M_y / phi_y");
        for (key, value) in [("phi_y", response.kappa_y), ("m_y", response.m_y), ("ei_eff", ei)] {
            let expected = oracle[&format!("{name}.{key}")];
            assert!((expected - value).abs() <= 1.5e-2 * expected, "{name}.{key}: oracle={expected} rust={value}");
        }
    }
    let gross = bridge::pier_gross_ei_nm2(&computed_pier(), Longitudinal);
    assert!((gross - 33000.0e6 * 0.8 * 1.2f64.powi(3) / 12.0).abs() <= 1e-6 * gross);
    let compressed = bridge::pier_section(&computed_pier(), Longitudinal).effective_stiffness_nm2(&k).expect("N = 1 MN");
    let free = bridge::pier_section(&En1998Pier { axial_load_n: 0.0, ..computed_pier() }, Longitudinal).effective_stiffness_nm2(&k).expect("N = 0");
    assert!(compressed > free, "compression stiffens the cracked section");
}

#[semio_framework_async_macros::async_test]
async fn the_bridge_analysis_uses_the_basis_of_every_pier() {
    use bridge::Direction::{Longitudinal, Transverse};
    let (k, (mut piers, bearings)) = (annex_a(), oracle_bridge());
    let declared = bridge::supports("b", &piers, &bearings, 0.5, &k);
    piers[0] = En1998Pier { id: "p1".into(), x_m: 10.0, ..computed_pier() };
    let mixed = bridge::supports("b", &piers, &bearings, 0.5, &k);
    let ratio = bridge::pier_ei_ratio(&piers[0], Longitudinal, 0.5, &k);
    assert!(ratio > 0.0 && ratio < 1.0);
    assert_eq!(bridge::pier_ei_ratio(&piers[1], Longitudinal, 0.5, &k), 0.5, "the declared basis keeps the bridge share");
    let expected = 3.0 * piers[0].modulus_mpa * 1.0e6 * ratio * piers[0].section_b_m * piers[0].section_h_m.powi(3) / 12.0 / piers[0].height_m.powi(3);
    assert!((mixed[1].pier_longitudinal_n_per_m - expected).abs() <= 1e-9 * expected, "the pier stiffness is 3 EI_eff / H^3");
    assert_eq!(mixed[2].pier_longitudinal_n_per_m, declared[2].pier_longitudinal_n_per_m, "the declared pier is unchanged");
    assert!((mixed[1].pier_transverse_n_per_m - bridge::pier_ei_ratio(&piers[0], Transverse, 0.5, &k) * 3.0 * piers[0].modulus_mpa * 1.0e6 * piers[0].section_h_m * piers[0].section_b_m.powi(3) / 12.0 / piers[0].height_m.powi(3)).abs() <= 1e-6 * mixed[1].pier_transverse_n_per_m);
    let none = En1998Pier { reinforcement_ratio: 0.0, ..computed_pier() };
    assert_eq!(bridge::pier_ei_ratio(&none, Longitudinal, 0.5, &k), 0.0, "a section without reinforcement leaves the pier without stiffness");
}

#[semio_framework_async_macros::async_test]
async fn the_computed_basis_reports_ei_eff_and_asks_for_the_missing_reinforcement() {
    let mut snap = structures();
    snap.piers[0] = En1998Pier { id: "pier-1".into(), bridge_id: snap.piers[0].bridge_id.clone(), x_m: snap.piers[0].x_m, height_m: snap.piers[0].height_m, ..computed_pier() };
    let report = inferences::evaluate(&snap);
    let check = find(&report, ".pier.pier-1.stiffness");
    assert_eq!(check.status, CheckStatus::Pass, "{:?}", check.explanation);
    assert!(check.explanation.en.contains("EI_eff"));
    failing(|s| { s.piers[0] = En1998Pier { id: "pier-1".into(), bridge_id: s.piers[0].bridge_id.clone(), x_m: s.piers[0].x_m, height_m: s.piers[0].height_m, reinforcement_ratio: 0.0, ..computed_pier() } }, ".pier.pier-1.stiffness", ".reinforcementRatio");
}
