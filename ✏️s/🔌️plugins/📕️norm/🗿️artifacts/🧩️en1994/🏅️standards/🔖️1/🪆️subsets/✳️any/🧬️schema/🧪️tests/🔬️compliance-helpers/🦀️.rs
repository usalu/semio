use super::*;

#[semio_framework_async_macros::async_test]
async fn composite_beam_e2e() {
    let report = check_composite_beam(200.0, 120.0, 80.0, 250.0, 0.75, 150.0, AnnexChoice::De);
    assert!(!report.checks.is_empty());
    let m_rd: f64 = 80.0 + 0.75 * (250.0 - 80.0);
    assert!((m_rd - 207.5).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn effective_width_8m_span() {
    let beff = part_1_1::effective_width_mm(8000.0, 80.0, 3000.0);
    assert!((beff - 2160.0).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn partial_shear_connection_eta() {
    let eta = part_1_1::shear_connection_degree(15, 20);
    assert!((eta - 0.75).abs() < 0.01);
    let m_rd = part_1_1::plastic_moment_partial_knm(100.0, 300.0, eta);
    assert!((m_rd - 250.0).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn longitudinal_shear_vl() {
    let v_l = part_1_1::longitudinal_shear_kn(500.0, 200.0);
    assert!((v_l - 2500.0).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn stud_connector_resistance_worked_example() {
    let d = 19.0_f64;
    let h_sc = 5.0 * d;
    let f_u: f64 = 450.0;
    let f_ck: f64 = 30.0;
    let e_cm: f64 = 33_000.0;
    let alpha = part_1_1::stud_alpha(h_sc, d);
    assert!((alpha - 1.0).abs() < 1e-9);
    let p_pl = 0.8 * f_u * std::f64::consts::PI * d * d / 4.0;
    let p_b = 0.29 * alpha * d * d * (f_ck * e_cm).sqrt();
    assert!(p_pl < p_b, "shank shear-off branch should govern for this worked example");
    let expected_p_rd_kn = p_pl / 1.25 / 1000.0;
    assert!((expected_p_rd_kn - 81.656).abs() < 0.01);
    let p_rd = part_1_1::connector_resistance_kn(d, h_sc, f_ck, f_u, e_cm, AnnexChoice::En);
    assert!((p_rd - expected_p_rd_kn).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn min_shear_connection_degree_span_8m() {
    let eta_min = part_1_1::min_shear_connection_degree(8.0, 355.0);
    assert!((eta_min - 0.49).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn fire_insulation_r60() {
    let t = part_1_2::insulation_thickness_mm(part_1_2::FireRating::R60, "trapezoidal");
    assert!((t - 18.0).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn bridge_composite_fatigue() {
    let report = part_2::check_bridge_composite(180.0, 250.0, 65.0, "stud_welded");
    assert_eq!(report.checks.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn stud_fatigue_resistance_at_reference_cycles() {
    let delta_tau_c = part_2::stud_fatigue_resistance_mpa(part_2::STUD_N_REF);
    assert!((delta_tau_c - 90.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn annex_params_document_equality() {
    let en = AnnexParams::en();
    let de = AnnexParams::de();
    assert!((en.gamma_v - de.gamma_v).abs() < 1e-9);
    assert!((en.gamma_c - de.gamma_c).abs() < 1e-9);
    assert!((en.gamma_s - de.gamma_s).abs() < 1e-9);
    let p_rd_en = part_1_1::connector_resistance_kn(19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::En);
    let p_rd_de = part_1_1::connector_resistance_kn(19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::De);
    assert!((p_rd_en - p_rd_de).abs() < 1e-9);
    let check_en = part_1_1::check_stud_resistance(40.0, 19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::En);
    let check_de = part_1_1::check_stud_resistance(40.0, 19.0, 95.0, 30.0, 450.0, 33_000.0, AnnexChoice::De);
    assert!((check_en.utilization - check_de.utilization).abs() < 1e-9);
}
