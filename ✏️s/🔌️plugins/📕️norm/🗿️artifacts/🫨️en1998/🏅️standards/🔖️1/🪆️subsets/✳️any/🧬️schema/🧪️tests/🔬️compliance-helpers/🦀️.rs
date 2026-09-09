use super::*;

#[semio_framework_async_macros::async_test]
async fn zone2_spectrum_sd_at_t1() {
    let a_g = na_de::SeismicZone::Zone2.a_g();
    let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
    let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
    assert!((s_e - 0.375).abs() < 1e-9);
    let sd = part_1::design_spectrum_sd(s_e, 1.0, 1.0);
    assert!((sd - 0.375).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn zone2_spectrum_sd_at_half_second() {
    let a_g = na_de::SeismicZone::Zone2.a_g();
    let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
    let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.5);
    assert!((s_e - 0.3).abs() < 1e-9);
    let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
    let q = part_1::StructuralSystem::MomentFrameDch.q();
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    assert!((s_d - 0.075).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn base_shear_uses_design_spectrum() {
    let a_g = 0.15;
    let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
    let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
    let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
    let q = part_1::StructuralSystem::MomentFrameDch.q();
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    let v_b = part_1::base_shear_kn(s_e, 100.0, gamma_i, q);
    assert!(v_b > 0.0);
    let expected = s_e * 100.0 * 9.81 * gamma_i / q;
    assert!((v_b - expected).abs() < 1e-6);
    assert!((part_1::base_shear_from_design_kn(s_d, 100.0) - v_b).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn drift_rho_limit() {
    let rho = part_1::redundancy_factor(false);
    assert!((rho - 1.3).abs() < 1e-9);
    let limit = part_1::drift_limit_mm(12.0, rho, part_1::DuctilityClass::Dcm, 1.0);
    assert!((limit - 109.2).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn building_seismic_base_shear_uses_sd() {
    let a_g = na_de::SeismicZone::Zone2.a_g();
    let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
    let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
    let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
    let q = part_1::StructuralSystem::MomentFrameDch.q();
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    let expected_v_b = part_1::base_shear_from_design_kn(s_d, 500.0);

    let report = check_building_seismic(na_de::SeismicZone::Zone2, na_de::GroundType::B, part_1::ImportanceClass::Cc2, part_1::StructuralSystem::MomentFrameDch, 0.3, 500.0, 800.0, 20.0, 12.0, true);
    assert_eq!(report.checks.len(), 2);
    assert!((report.checks[0].computed.value - expected_v_b * 1000.0).abs() < 1e-3);
}

#[semio_framework_async_macros::async_test]
async fn en_type1_vs_de_zone_divergence_same_nominal_ag() {
    let a_g = 0.15;
    let annex_de = AnnexParams::De { zone: na_de::SeismicZone::Zone2, ground: na_de::GroundType::B };
    let annex_en = AnnexParams::En { a_gr: a_g, ground: part_1::EnGroundType::B, spectrum: part_1::SpectrumType::Type1 };
    let s_e_de = annex_de.elastic_response_spectrum(0.3);
    let s_e_en = annex_en.elastic_response_spectrum(0.3);
    assert!((s_e_de - 0.375).abs() < 1e-9);
    assert!((s_e_en - 0.45).abs() < 1e-9);
    assert!((s_e_en - s_e_de).abs() > 0.05);
}

#[semio_framework_async_macros::async_test]
async fn building_seismic_e2e() {
    let report = check_building_seismic(na_de::SeismicZone::Zone2, na_de::GroundType::B, part_1::ImportanceClass::Cc2, part_1::StructuralSystem::MomentFrameDch, 0.3, 500.0, 800.0, 20.0, 12.0, true);
    assert_eq!(report.checks.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn silo_impulsive_convective() {
    let h = 10.0;
    let r = 5.0;
    let t_i = part_4::impulsive_period_s(h, r);
    let t_c = part_4::convective_period_s(r);
    assert!(t_i < t_c);
    let v = part_4::silo_base_shear_kn(200.0, 150.0);
    assert!((v - 250.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn silo_behaviour_factor_capped() {
    assert!((part_4::silo_behaviour_factor(2.0) - 1.5).abs() < 1e-9);
    assert!((part_4::silo_behaviour_factor(1.0) - 1.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn tank_base_shear_combines_impulsive_and_convective() {
    let a_g = 0.15;
    let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
    let t_i = part_4::impulsive_period_s(8.0, 4.0);
    let t_c = part_4::convective_period_s(4.0);
    let s_e_i = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_i);
    let s_e_c = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_c);
    let v_tank = part_4::tank_base_shear_kn(100.0, s_e_i, 50.0, s_e_c);
    assert!((v_tank - 412.8624420576831).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn bridge_isolation_distinct() {
    let q_isol = part_2::isolation_reduction_factor(2.0);
    assert!((q_isol - 4.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn retrofit_confidence_factor_scales_capacity_exactly() {
    let r_k = 400.0;
    let r_d_kl3 = part_3::design_capacity_kn(r_k, part_3::KnowledgeLevel::Kl3.confidence_factor(), 1.0);
    let r_d_kl1 = part_3::design_capacity_kn(r_k, part_3::KnowledgeLevel::Kl1.confidence_factor(), 1.0);
    assert!((r_d_kl3 / r_d_kl1 - 1.35).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn mononobe_okabe_k_ae_matches_hand_calc_and_reduces_to_rankine() {
    let k_ae = part_5::mononobe_okabe_k_ae(30.0, 0.2);
    assert!((k_ae - 0.46407409106465564).abs() < 1e-9);
    let k_a_static = part_5::mononobe_okabe_k_ae(30.0, 0.0);
    let rankine_ka = (1.0 - 30.0_f64.to_radians().sin()) / (1.0 + 30.0_f64.to_radians().sin());
    assert!((k_a_static - rankine_ka).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn retaining_wall_thrust_from_k_ae() {
    let k_h = part_5::horizontal_seismic_coefficient(0.15, 1.0, 1.5);
    assert!((k_h - 0.1).abs() < 1e-9);
    let k_ae = part_5::mononobe_okabe_k_ae(30.0, 0.2);
    let thrust = part_5::retaining_wall_thrust_kn_m(18.0, 4.0, k_ae);
    assert!((thrust - 66.82666911331042).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn cantilever_modal_participation_factor_matches_closed_form() {
    let gamma = part_6::cantilever_modal_participation_factor();
    assert!((gamma - 1.602484997695127).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn tower_behaviour_factor_capped_by_type() {
    assert!((part_6::tower_behaviour_factor(3.0, true) - 1.5).abs() < 1e-9);
    assert!((part_6::tower_behaviour_factor(3.0, false) - 2.0).abs() < 1e-9);
}
