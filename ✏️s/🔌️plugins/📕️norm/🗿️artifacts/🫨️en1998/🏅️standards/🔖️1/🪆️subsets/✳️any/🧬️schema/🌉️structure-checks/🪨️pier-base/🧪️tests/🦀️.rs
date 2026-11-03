//! 🧪 Pier base checks on a pier of E 33 GPa, 2 × 1.5 m, H 8 m, half cracked, on a circular footing R 2.5 m, v_s 250 m/s, γ 19 kN/m³, ν 0.35 under the site a_g S = 0.15 g; the springs and the lengthening are recomputed in `🧰️scripts/en1998-wave3d/1-hand-values.py` (same formulas) and by the Python oracle `pier_base_rows`.

use super::*;
use compliance::CheckStatus;
use model::PierStiffnessBasis;

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1.0e-9 * b.abs().max(1.0), "{a} vs {b}");
}

fn pier() -> model::En1998Pier {
    model::En1998Pier { id: "pier-1".into(), bridge_id: "br-1".into(), x_m: 20.0, height_m: 8.0, section_b_m: 2.0, section_h_m: 1.5, modulus_mpa: 33000.0, moment_rd_nm: 1.8e7, shear_rd_n: 2.4e6, reinforcement_ratio: 0.01, cover_m: 0.08, concrete_fck_mpa: 30.0, steel_fyk_mpa: 500.0, axial_load_n: 7.0e6, stiffness_basis: PierStiffnessBasis::Declared }
}

fn base() -> Base {
    Base { id: "pb-1".into(), bridge_id: "br-1".into(), pier_id: "pier-1".into(), radius_m: 2.5, soil_vs_m_s: 250.0, soil_unit_weight_n_m3: 19000.0, poisson_ratio: 0.35 }
}

fn bridge(capacity: f64) -> Bridge {
    Bridge { id: "br-1".into(), period_ratio: 2.0, fundamental_period_s: 0.5, v_rd_n: 2.0e6, bearing_d_rd_m: capacity, permanent_gk_n: 8.0e6, correlated_occupancy: true, variables: Vec::new(), pier_height_m: 0.0, pier_moment_rd_nm: 0.0, pier_shear_rd_n: 0.0, deck_length_m: 60.0, deck_ei_nm2: 5.0e11, importance_class: 2.0, vertical_period_s: 0.3, pier_cracked_inertia_ratio: 0.5 }
}

fn run(base: &Base, capacity: f64, t_fixed: f64) -> CheckReport {
    let mut document = En1998Snapshot::compliant_de_office();
    document.piers = vec![pier()];
    document.bridges = vec![bridge(capacity)];
    let ground = super::super::Ground { a_g: 0.15 * 9.81 / 1.2, s: 1.2, tb: 0.15, tc: 0.5, td: 2.0, gamma_i: 1.0 };
    let mut report = CheckReport::default();
    check_pier_base(&mut report, AnnexChoice::En, &document, &bridge(capacity), base, &ground, t_fixed);
    report
}

fn check<'a>(report: &'a CheckReport, tail: &str) -> &'a CheckResult {
    report.checks.iter().find(|c| c.id == format!("en1998.2.br-1.base.pb-1.{tail}")).unwrap_or_else(|| panic!("{tail} missing"))
}

#[test]
fn the_springs_use_the_shear_modulus_degraded_by_table_4_1_at_the_design_acceleration() {
    let report = run(&base(), 0.5, 0.5);
    let g_max = 19000.0 / 9.81 * 250.0_f64.powi(2);
    let ratio = 0.9 + (0.15 - 0.10) / 0.10 * (0.7 - 0.9);
    let g = g_max * ratio * ratio;
    near(check(&report, "springs").computed.value, 8.0 * g * 2.5 / 1.65);
    assert_eq!(check(&report, "springs").status, CheckStatus::Info);
}

#[test]
fn the_period_lengthens_by_the_flexibility_of_the_springs_and_the_displacement_follows_the_spectrum() {
    let report = run(&base(), 0.5, 0.5);
    let g = 19000.0 / 9.81 * 250.0_f64.powi(2) * 0.8 * 0.8;
    let (k_x, k_rx) = (8.0 * g * 2.5 / 1.65, 8.0 * g * 2.5_f64.powi(3) / (3.0 * 0.65));
    let k_pier = 3.0 * 33000.0e6 * 0.5 * 1.5 * 2.0_f64.powi(3) / 12.0 / 8.0_f64.powi(3);
    let t_ssi = 0.5 * (1.0 + k_pier / k_x + k_pier * 64.0 / k_rx).sqrt();
    near(check(&report, "period").computed.value, t_ssi);
    let s_e = |t: f64| 0.15 * 9.81 * 2.5 * if t <= 2.0 { 0.5 / t.max(0.5) } else { 0.5 * 2.0 / (t * t) };
    near(check(&report, "displacement").computed.value, s_e(t_ssi) * (t_ssi / (2.0 * std::f64::consts::PI)).powi(2));
}

#[test]
fn a_bearing_capacity_below_the_displacement_fails_with_that_displacement_as_remedy() {
    let report = run(&base(), 0.01, 0.5);
    let displacement = check(&report, "displacement");
    assert_eq!(displacement.status, CheckStatus::Fail);
    near(displacement.remedies.first().expect("remedy").required.value, displacement.computed.value);
}

#[test]
fn a_footing_without_radius_or_with_a_poisson_ratio_of_one_half_fails_per_field() {
    let mut broken = base();
    broken.radius_m = 0.0;
    broken.poisson_ratio = 0.5;
    let report = run(&broken, 0.5, 0.5);
    let springs = check(&report, "springs");
    assert_eq!(springs.status, CheckStatus::Fail);
    assert_eq!(springs.remedies.len(), 2);
}

#[test]
fn a_base_naming_no_pier_of_its_bridge_is_reported_with_the_piers_as_options() {
    let mut lost = base();
    lost.pier_id = "pier-9".into();
    let report = run(&lost, 0.5, 0.5);
    let reference = check(&report, "pier");
    assert_eq!(reference.status, CheckStatus::Fail);
    assert_eq!(reference.remedies[0].options, vec!["pier-1".to_owned()]);
}
