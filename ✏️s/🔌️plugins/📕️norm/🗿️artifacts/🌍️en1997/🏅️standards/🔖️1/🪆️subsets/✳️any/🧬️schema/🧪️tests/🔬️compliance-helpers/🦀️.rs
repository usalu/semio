use super::*;
use crate::document::CheckStatus;

#[semio_framework_async_macros::async_test]
async fn bearing_factor_n_c_phi30() {
    let n_c = part_1::bearing_factor_n_c(30.0, 1.5, 2.0);
    assert!((n_c - 30.1).abs() < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn shallow_foundation_e2e() {
    let report = check_shallow_foundation(500.0, 80.0, 2.0, 30.0, 0.0, 18.0, 2.0, 1.5, 30_000.0, 0.3, DesignApproach::Da1Str, AnnexChoice::De, 25.0);
    assert!(!report.checks.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn pile_design_resistance_worked() {
    let r_s = part_1::shaft_resistance_kn(0.7, 0.6, 80.0, 12.0);
    assert!((r_s - 1266.69).abs() < 1.0);
    let r_b = part_1::base_resistance_kn(2500.0, 0.28);
    assert!((r_b - 700.0).abs() < 0.1);
    let r_c_d = part_1::pile_design_resistance_kn(r_b, r_s, DesignApproach::Da2, AnnexChoice::De);
    let expected = r_b / 1.1 + r_s / 1.1;
    assert!((r_c_d - expected).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn pile_correlation_factors_n2_matches_annex_a_table() {
    let (xi_3, xi_4) = part_1::pile_correlation_factors(2);
    assert!((xi_3 - 1.35).abs() < 1e-9);
    assert!((xi_4 - 1.27).abs() < 1e-9);
    let mean_r_cal_kn = 1000.0;
    let min_r_cal_kn = 900.0;
    let r_k = part_1::pile_characteristic_resistance_kn(mean_r_cal_kn, min_r_cal_kn, 2);
    let expected = (mean_r_cal_kn / 1.35_f64).min(min_r_cal_kn / 1.27_f64);
    assert!((r_k - expected).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn pile_correlation_factors_boundary_cases() {
    assert_eq!(part_1::pile_correlation_factors(1), (1.40, 1.40));
    assert_eq!(part_1::pile_correlation_factors(5), (1.30, 1.15));
    assert_eq!(part_1::pile_correlation_factors(9), (1.30, 1.15));
}

#[semio_framework_async_macros::async_test]
async fn investigation_depth_check_pass_and_fail() {
    let b_m = 2.0;
    let min_depth = part_2::min_investigation_depth_m(b_m);
    assert!((min_depth - 6.0).abs() < 1e-9);
    let pass = part_2::check_investigation_depth(8.0, b_m, AnnexChoice::De);
    assert_eq!(pass.status, CheckStatus::Pass);
    let fail = part_2::check_investigation_depth(4.0, b_m, AnnexChoice::De);
    assert_eq!(fail.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn phi_from_cpt_worked_example() {
    let q_c_kpa = 15_000.0;
    let sigma_v0_kpa = 100.0;
    let phi = part_2::phi_from_cpt_deg(q_c_kpa, sigma_v0_kpa);
    let expected = (0.38 * (q_c_kpa / sigma_v0_kpa).log10() + 0.1).atan().to_degrees();
    assert!((phi - expected).abs() < 1e-9);
    assert!(phi > 25.0 && phi < 45.0);
}

#[semio_framework_async_macros::async_test]
async fn phi_from_spt_worked_example() {
    let phi = part_2::phi_from_spt_deg(20.0);
    let expected = 27.1 + 0.3 * 20.0 - 0.00054 * 20.0 * 20.0;
    assert!((phi - expected).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn da2_star_de_diverges_from_da2_en_on_same_footing() {
    let q_d_de = part_1::design_bearing_capacity_kpa(30.0, 0.0, 18.0, 2.0, 1.5, DesignApproach::Da2, AnnexChoice::De);
    let q_d_en = part_1::design_bearing_capacity_kpa(30.0, 0.0, 18.0, 2.0, 1.5, DesignApproach::Da2, AnnexChoice::En);
    assert!(q_d_de < q_d_en);
    assert!((q_d_en / q_d_de - 1.4).abs() < 1e-9);
    assert_eq!(DesignApproach::Da2.label(AnnexChoice::De), "DA2*");
    assert_eq!(DesignApproach::Da2.label(AnnexChoice::En), "DA2");
}
