use super::*;

#[semio_framework_async_macros::async_test]
async fn steel_member_e2e() {
    let report = check_steel_member(500.0, 150.0, 5000.0, 500_000.0, 355.0, 0.75);
    assert_eq!(report.checks.len(), 3);
    let params = AnnexParams::de();
    let n_rd = part_1_1::axial_resistance_kn(5000.0, 355.0, params);
    let n_b_rd = part_1_1::buckling_resistance_kn(5000.0, 355.0, 0.75, params);
    let m_rd = part_1_1::bending_resistance_knm(500_000.0, 355.0, params);
    assert!((report.checks[0].utilization - 500.0 / n_rd).abs() < 1e-6);
    assert!((report.checks[1].utilization - 500.0 / n_b_rd).abs() < 1e-6);
    assert!((report.checks[2].utilization - 150.0 / m_rd).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
#[cfg(feature = "cross-fem")]
fn steel_member_from_fem_e2e() {
    let report = check_steel_member_from_fem(6.0, 20.0, 5000.0, 500_000.0, 2500.0, 355.0, 0.75).expect("fem solve");
    assert_eq!(report.checks.len(), 4);
    let m_ed = report.checks[2].computed.value / 1_000_000.0;
    assert!((m_ed - 90.0).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn plated_buckling_reduction() {
    let rho = part_1_5::plate_reduction_factor(0.5);
    assert!((rho - 1.0).abs() < 1e-9);
    let b_eff = part_1_5::effective_width_mm(200.0, 0.5);
    assert!((b_eff - 200.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn hea200_section_classification() {
    let eps = part_1_1::epsilon(355.0);
    assert!((eps - 0.814).abs() < 0.01);
    let flange_c = (200.0 - 9.0) / 2.0 - 12.0;
    let web_c = 190.0 - 2.0 * 15.5 - 2.0 * 12.0;
    let class = part_1_1::section_class(flange_c, 15.5, web_c, 9.0, 355.0);
    assert!((1..=4).contains(&class));
}

#[semio_framework_async_macros::async_test]
async fn hea200_chi_at_lambda_1() {
    let chi = part_1_1::chi(1.0, part_1_1::BucklingCurve::A0);
    assert!((chi - 0.73).abs() < 0.05);
}

#[semio_framework_async_macros::async_test]
async fn axial_resistance_s355() {
    let n_rd = part_1_1::axial_resistance_kn(5382.0, 355.0, AnnexParams::de());
    assert!((n_rd - 1910.6).abs() < 5.0);
}

#[semio_framework_async_macros::async_test]
async fn bolt_shear_m20() {
    let f_v = part_1_8::bolt_shear_resistance_kn(2, 245.0, 800.0, 1.25);
    assert!((f_v - 188.0).abs() < 5.0);
}

/// 🔩️ M20 bolt bearing worked example: e₁=e₂=40mm, d₀=22mm, t=10mm, f_u=510MPa, f_ub=800MPa → F_b,Rd ≈ 123.6 kN.
#[semio_framework_async_macros::async_test]
async fn bolt_bearing_m20_worked_example() {
    let alpha_b = part_1_8::bearing_alpha_b(40.0, 22.0, 800.0, 510.0);
    let k1 = part_1_8::bearing_k1(40.0, 22.0);
    assert!((alpha_b - (40.0_f64 / 66.0)).abs() < 1e-9);
    assert!((k1 - 2.5).abs() < 1e-9);
    let f_b_rd = part_1_8::bolt_bearing_resistance_kn(k1, alpha_b, 510.0, 20.0, 10.0, 1.25);
    assert!((f_b_rd - 123.636_363_636).abs() < 1e-3);
}

/// 🪛️ Fillet weld resistance sanity check for an S355 weld, β_w=0.9.
#[semio_framework_async_macros::async_test]
async fn fillet_weld_s355() {
    let beta_w = part_1_8::beta_w("S355");
    assert!((beta_w - 0.9).abs() < 1e-9);
    let f_w_rd = part_1_8::fillet_weld_resistance_kn(5.0, 100.0, 510.0, beta_w, 1.25);
    assert!(f_w_rd > 0.0);
    assert!((f_w_rd - (5.0 * 100.0 * 510.0 / (3.0_f64.sqrt() * 0.9 * 1.25) / 1000.0)).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn annex_params_gamma_m1_divergence() {
    let en = AnnexParams::en();
    let de = AnnexParams::de();
    assert!((en.gamma_m0 - de.gamma_m0).abs() < 1e-9);
    assert!((en.gamma_m2 - de.gamma_m2).abs() < 1e-9);
    assert!((de.gamma_m1 / en.gamma_m1 - 1.1).abs() < 1e-9);
    let n_rd_en = part_1_1::buckling_resistance_kn(5000.0, 355.0, 0.75, en);
    let n_rd_de = part_1_1::buckling_resistance_kn(5000.0, 355.0, 0.75, de);
    assert!((n_rd_en / n_rd_de - 1.1).abs() < 1e-9);
}

/// 🔥️ Critical steel temperature worked example, EN 1993-1-2 Eq. 4.22 at μ₀=0.5.
#[semio_framework_async_macros::async_test]
async fn critical_steel_temperature_mu_0_5() {
    let mu_0: f64 = 0.5;
    let expected = 39.19 * (1.0 / (0.9674 * mu_0.powf(3.833)) - 1.0).ln() + 482.0;
    let theta = part_1_2::critical_temperature_c(mu_0);
    assert!((theta - expected).abs() < 1e-9);
    assert!((theta - 584.665).abs() < 1e-2);
}

#[semio_framework_async_macros::async_test]
async fn net_tension_uses_gamma_m2_for_rupture() {
    let params = AnnexParams::en();
    let n_t_rd = part_1_1::net_tension_resistance_kn(5000.0, 4250.0, 355.0, 510.0, params);
    let net_rupture_kn = 0.9 * 4250.0 * 510.0 / params.gamma_m2 / 1000.0;
    let gross_yield_kn = 5000.0 * 355.0 / params.gamma_m0 / 1000.0;
    assert!((n_t_rd - net_rupture_kn.min(gross_yield_kn)).abs() < 1e-6);
    assert!(net_rupture_kn < gross_yield_kn, "worked example should be governed by net rupture");
}

#[semio_framework_async_macros::async_test]
async fn chs_classification_table_5_2() {
    let class = part_1_1::chs_class(200.0, 8.0, 355.0);
    assert!((1..=4).contains(&class));
    assert_eq!(part_1_1::chs_class(10.0, 8.0, 355.0), 1);
}

/// 🐚️ Shell meridional critical buckling stress worked example: t=8mm, r=3000mm, E=210000MPa.
#[semio_framework_async_macros::async_test]
async fn shell_sigma_x_rcr_worked_example() {
    let sigma = part_1_6::sigma_x_rcr_mpa(8.0, 3000.0, 210_000.0);
    assert!((sigma - 338.8).abs() < 1e-9);
}

/// 📐️ Cold-formed reduction factor worked example: λ_p=1.0, ψ=1 → ρ=(1−0.055·4)/1=0.78.
#[semio_framework_async_macros::async_test]
async fn cold_formed_reduction_factor_worked_example() {
    let rho = part_1_3::reduction_factor(1.0, 1.0);
    assert!((rho - 0.78).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn fatigue_detail_71() {
    assert!((part_1_9::fatigue_strength_mpa(71) - 71.0).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn fatigue_cycles_to_failure_slope_m3() {
    let n = part_1_9::cycles_to_failure(71.0, 71);
    assert!((n - 2.0e6).abs() < 1.0);
    let n_half_stress = part_1_9::cycles_to_failure(35.5, 71);
    assert!((n_half_stress / n - 8.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn fatigue_assessment_methods() {
    assert!((part_1_9::AssessmentMethod::LowConsequence.gamma_mf() - 1.0).abs() < 1e-9);
    assert!((part_1_9::AssessmentMethod::DamageTolerant.gamma_mf() - 1.15).abs() < 1e-9);
    assert!((part_1_9::AssessmentMethod::SafeLife.gamma_mf() - 1.35).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn tension_component_resistance() {
    let f_rd = part_1_11::tension_component_resistance_kn(500.0, 350.0);
    assert!((f_rd - (500.0_f64 / 1.5).min(350.0)).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn high_strength_steel_class_4_guard() {
    assert!(part_1_12::is_high_strength_restricted(460.0));
    assert!(!part_1_12::is_high_strength_restricted(355.0));
    let m_rd_class4 = part_1_12::elastic_bending_resistance_knm(400_000.0, 460.0, 4, AnnexParams::de());
    assert_eq!(m_rd_class4, 0.0);
    let m_rd_class2 = part_1_12::elastic_bending_resistance_knm(400_000.0, 460.0, 2, AnnexParams::de());
    assert!(m_rd_class2 > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn stainless_steel_gamma_is_1_1_regardless_of_annex() {
    assert!((part_1_4::GAMMA_M_STAINLESS - 1.1).abs() < 1e-9);
    let m_rd = part_1_4::bending_resistance_knm(300_000.0, 220.0);
    assert!(m_rd > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn fire_board_r60() {
    let t = part_1_2::board_thickness_mm(part_1_2::FireRating::R60, 150.0);
    assert!(t > 15.0);
}

#[semio_framework_async_macros::async_test]
async fn through_thickness_table_lookup() {
    let t_max = part_1_10::max_permissible_thickness_mm("J2", 0.0);
    assert!((t_max - 65.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn silo_membrane_hoop_stress() {
    let p_h = part_4::janssen_pressure_kpa(0.4, 18.0, 5.0);
    assert!((p_h - 36.0).abs() < 1e-9);
    let sigma = part_4::membrane_hoop_stress_mpa(p_h, 3000.0, 8.0);
    assert!((sigma - 13.5).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn crane_runway_web_stress() {
    let l_eff = part_6::effective_length_mm(100.0, 50.0);
    assert!((l_eff - 200.0).abs() < 1e-9);
    let sigma = part_6::wheel_load_web_stress_mpa(50.0, l_eff, 10.0);
    assert!((sigma - 25.0).abs() < 1e-9);
}
