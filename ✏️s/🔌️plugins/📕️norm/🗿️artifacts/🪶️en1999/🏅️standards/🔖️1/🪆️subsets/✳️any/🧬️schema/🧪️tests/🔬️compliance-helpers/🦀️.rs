
use super::*;

#[semio_framework_async_macros::async_test]
async fn alloy_6060_t6_m_c_rd() {
    let alloy = part_1_1::Alloy::Aw6060T6;
    let w_el = 24_000.0;
    let m_rd = part_1_1::m_c_rd_knm(w_el, alloy, na_de::AnnexParams::de().gamma_m1);
    assert!((m_rd - 4.145454545).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn section_classification_6060() {
    let alloy = part_1_1::Alloy::Aw6060T6;
    let class = part_1_1::classify_flange_outstand(10.0, 3.0, alloy);
    assert_eq!(class, part_1_1::SectionClass::Class1);
}

#[semio_framework_async_macros::async_test]
async fn haz_reduced_strength() {
    let alloy = part_1_1::Alloy::Aw6060T6;
    let f_haz = part_1_1::haz_strength_mpa(alloy);
    assert!((f_haz - 95.0).abs() < 1e-9);
    assert!((na_de::HAZ_ZONE_MM - 25.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn haz_reduction_factor_6082_t6_table_6_2() {
    // 📖️ EN 1999-1-1 Table 6.2 representative GMAW HAZ value for 6082-T6: f_o,haz ≈ 185 MPa vs f_o = 260 MPa ⇒ ρ_o,haz ≈ 0.71.
    let alloy = part_1_1::Alloy::Aw6082T6;
    let rho_o_haz = part_1_1::haz_reduction_factor(alloy);
    assert!((rho_o_haz - 185.0 / 260.0).abs() < 1e-9);
    assert!((rho_o_haz - 0.711).abs() < 1e-3);
}

#[semio_framework_async_macros::async_test]
async fn fatigue_cutoff_at_2e6() {
    let strength = part_1_3::fatigue_strength_mpa(80.0, 8.0, 2_000_000.0);
    assert!((strength - 0.0).abs() < 1e-9);
    let below = part_1_3::fatigue_strength_mpa(80.0, 8.0, 1_000_000.0);
    assert!(below > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn aluminium_member_e2e() {
    let alloy = part_1_1::Alloy::Aw6082T6;
    let report = check_aluminium_member(80.0, 12.0, 1200.0, 15_000.0, alloy, 0.8, 5000.0, 3000.0, AnnexChoice::De);
    assert_eq!(report.checks.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn weld_resistance_worked() {
    let a_w = part_1_1::weld_throat_area_mm2(4.0, 120.0);
    assert!((a_w - 480.0).abs() < 1e-9);
    let gamma_m2 = na_de::AnnexParams::de().gamma_m2;
    let v_rd = part_1_1::weld_resistance_kn(a_w, 215.0, 0.63, gamma_m2);
    assert!((v_rd - 480.0 * 215.0 / (0.63 * gamma_m2 * 1000.0)).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn fire_critical_temperature_6060() {
    let theta_cr = part_1_2::critical_temperature_c(190.0);
    assert!((theta_cr - 246.0).abs() < 0.1);
    let k_theta = part_1_2::strength_reduction_factor(300.0, theta_cr);
    assert!(k_theta < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn cold_formed_effective_width_factor_worked() {
    // 📖️ EN 1999-1-4 §5.4: ρ(λ_p=1.0) = (1 − 0.22/1.0)/1.0 = 0.78.
    let rho = part_1_4::effective_width_factor(1.0);
    assert!((rho - 0.78).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn cold_formed_effective_width_factor_below_limit_is_unity() {
    let rho = part_1_4::effective_width_factor(0.5);
    assert!((rho - 1.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn check_cold_formed_sheeting_worked() {
    let alloy = part_1_1::Alloy::Aw6060T6;
    let gamma_m1 = na_de::AnnexParams::de().gamma_m1;
    let w_eff = part_1_4::effective_section_modulus_mm3(8000.0, 0.78);
    let expected_m_rd_knm = w_eff * alloy.f_0_2_mpa() / gamma_m1 / 1_000_000.0;
    let result = part_1_4::check_cold_formed_sheeting(0.5, w_eff, alloy, gamma_m1, AnnexChoice::De);
    assert!((result.limit.value / 1_000_000.0 - expected_m_rd_knm).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn shell_critical_axial_stress_worked() {
    // 📖️ EN 1999-1-5 §5: σ_cr = 0.605·E·t/r = 0.605·70000·4/500 = 338.8 MPa.
    let sigma_cr = part_1_5::critical_axial_stress_mpa(4.0, 500.0);
    assert!((sigma_cr - 338.8).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn shell_buckling_pipeline_worked() {
    let alloy = part_1_1::Alloy::Aw6060T6;
    let gamma_m1 = na_de::AnnexParams::de().gamma_m1;
    let sigma_cr = part_1_5::critical_axial_stress_mpa(4.0, 500.0);
    let lambda_bar = part_1_5::relative_slenderness(alloy.f_0_2_mpa(), sigma_cr);
    assert!((lambda_bar - (alloy.f_0_2_mpa() / sigma_cr).sqrt()).abs() < 1e-9);
    let chi = part_1_5::buckling_reduction_factor(lambda_bar);
    let sigma_rd = part_1_5::design_buckling_stress_mpa(chi, alloy.f_0_2_mpa(), gamma_m1);
    assert!((sigma_rd - chi * alloy.f_0_2_mpa() / gamma_m1).abs() < 1e-9);
    let result = part_1_5::check_shell_buckling(150.0, sigma_rd, AnnexChoice::De);
    assert!(result.utilization < 1.0);
}
