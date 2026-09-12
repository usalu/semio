use super::*;

#[semio_framework_async_macros::async_test]
async fn rc_beam_e2e() {
    let report = check_rc_beam(120.0, 80.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, 200.0, AnnexChoice::De);
    assert!(!report.checks.is_empty());
    assert!(report.checks[0].utilization > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn punching_v_rd_max_c30() {
    let v = part_1_1::punching_v_rd_max_mpa(30.0, AnnexChoice::De);
    assert!((v - 4.488).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn slenderness_column() {
    let i = part_1_1::radius_of_gyration_mm(300_000.0, 2.25e9);
    let lambda = part_1_1::slenderness_lambda(3000.0, i);
    assert!((lambda - 34.6).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn crack_width_wk() {
    let eps_sm = part_1_1::steel_strain_eps_sm(200.0, 0.01, 2.9, 200_000.0);
    let wk = part_1_1::crack_width_wk_mm(eps_sm, 0.0001, 300.0);
    assert!(wk > 0.0 && wk < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn deflection_ss_udl() {
    let delta = part_1_1::deflection_ss_udl_mm(20.0, 6.0, 30_000.0, 1.875e9);
    assert!((delta - 6.0).abs() < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn fire_cover_beam_r60() {
    let req = part_1_2::min_axis_distance_mm(part_1_2::ElementType::Beam, FireRating::R60);
    assert!((req - 35.0).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn liquid_retaining_stress_limit() {
    assert!((part_3::steel_stress_limit_mpa("XD1") - 160.0).abs() < 0.1);
    let wk = part_3::crack_width_liquid_mm(220.0, "XD1", 250.0, 200_000.0);
    assert!(wk < 0.25);
}

#[semio_framework_async_macros::async_test]
async fn na_de_alpha_cc() {
    assert!((na_de::AnnexParams::de().alpha_cc - 0.85).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
#[cfg(feature = "cross-fem")]
fn rc_beam_from_fem_e2e() {
    let report = check_rc_beam_from_fem(6.0, 20.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, AnnexChoice::De).expect("fem solve");
    assert!(!report.checks.is_empty());
    let m_ed = report.checks[0].computed.value / 1_000_000.0;
    assert!((m_ed - 90.0).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn prestress_transfer_c30() {
    let sigma = part_1_1::prestress_transfer_stress_mpa(800.0, 135_000.0);
    assert!((sigma - 5.93).abs() < 0.1);
    let limit = part_1_1::prestress_transfer_limit_mpa(30.0);
    assert!((limit - 18.0).abs() < 1e-9);
    let report = check_full_rc_beam(120.0, 80.0, 30.0, 300.0, 450.0, 1200.0, 500.0, 0.01, 0.0, 800.0, 135_000.0, AnnexChoice::De);
    assert_eq!(report.checks.len(), 3);
    assert!(report.checks[2].utilization < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn annex_params_alpha_cc_de_vs_en_divergence() {
    let f_ck = 30.0;
    let f_cd_de = na_de::AnnexParams::de().f_cd_mpa(f_ck);
    let f_cd_en = na_de::AnnexParams::en().f_cd_mpa(f_ck);
    assert!((f_cd_de - 17.0).abs() < 1e-9);
    assert!((f_cd_en - 20.0).abs() < 1e-9);
    assert!(f_cd_en > f_cd_de);
}

#[semio_framework_async_macros::async_test]
async fn flexural_resistance_annex_divergence() {
    let m_rd_de = part_1_1::flexural_resistance_knm(30.0, 300.0, 450.0, 1200.0, 500.0, AnnexChoice::De);
    let m_rd_en = part_1_1::flexural_resistance_knm(30.0, 300.0, 450.0, 1200.0, 500.0, AnnexChoice::En);
    assert!(m_rd_en > m_rd_de);
}

#[semio_framework_async_macros::async_test]
async fn fire_r60_required_axis_distance_at_160mm() {
    let a = part_1_2::required_axis_distance_beam_mm(160.0, FireRating::R60);
    assert!((a - 35.0).abs() < 1e-9);
    let pass = part_1_2::check_fire_beam_axis_distance(160.0, 35.0, FireRating::R60);
    assert!(pass.status != CheckStatus::Fail);
    let fail = part_1_2::check_fire_beam_axis_distance(160.0, 20.0, FireRating::R60);
    assert_eq!(fail.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn bridge_concrete_stress_and_fatigue() {
    let limit = part_2::concrete_stress_limit_frequent_mpa(30.0);
    assert!((limit - 18.0).abs() < 1e-9);
    let ok = part_2::check_bridge_concrete_stress(12.0, 30.0);
    assert!(ok.status != CheckStatus::Fail);
    let resistance = part_2::fatigue_resistance_design_mpa();
    assert!((resistance - 141.304_347_826_086_96).abs() < 1e-6);
    let fatigue_ok = part_2::check_bridge_fatigue(100.0);
    assert!(fatigue_ok.status != CheckStatus::Fail);
    let fatigue_fail = part_2::check_bridge_fatigue(150.0);
    assert_eq!(fatigue_fail.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn tightness_class_crack_width_limits() {
    assert!(part_3::tightness_crack_width_limit_mm(TightnessClass::Tc0, 10.0).is_none());
    assert!((part_3::tightness_crack_width_limit_mm(TightnessClass::Tc1, 10.0).unwrap() - 0.3).abs() < 1e-9);
    let tc2_mid = part_3::tightness_crack_width_limit_mm(TightnessClass::Tc2, 20.0).unwrap();
    assert!((tc2_mid - 0.125).abs() < 1e-9);
    let tc0_check = part_3::check_tightness_crack_width(0.2, TightnessClass::Tc0, 10.0);
    assert_eq!(tc0_check.status, CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn anchor_m12_steel_and_concrete_cone_uncracked() {
    let f_ck = 30.0;
    let h_ef = 80.0;
    let n_rk_c = part_4::concrete_cone_resistance_n0_rk_c_n(f_ck, h_ef, false);
    assert!((n_rk_c - 43_111.0).abs() < 2.0);
    let n_rd_c = part_4::concrete_cone_resistance_design_n(f_ck, h_ef, false);
    assert!((n_rd_c - 28_740.7).abs() < 0.2);

    let f_uk = 800.0;
    let f_yk = 640.0;
    let a_s = 84.3;
    let gamma_ms = part_4::gamma_ms(f_uk, f_yk);
    assert!((gamma_ms - 1.5).abs() < 1e-9);
    let n_rk_s = part_4::steel_resistance_n_rk_s_n(a_s, f_uk);
    assert!((n_rk_s - 67_440.0).abs() < 1e-6);
    let n_rd_s = part_4::steel_resistance_design_n(a_s, f_uk, f_yk);
    assert!((n_rd_s - 44_960.0).abs() < 1e-3);

    let steel_check = part_4::check_anchor_steel(10_000.0, a_s, f_uk, f_yk);
    assert!(steel_check.status != CheckStatus::Fail);
    let cone_check = part_4::check_anchor_concrete_cone(10_000.0, f_ck, h_ef, false);
    assert!(cone_check.status != CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn anchor_edge_shear_breakout() {
    let v_rk_c = part_4::concrete_edge_resistance_v0_rk_c_n(12.0, 80.0, 30.0, 100.0);
    assert!(v_rk_c > 0.0);
    let check = part_4::check_anchor_edge_shear(5_000.0, 12.0, 80.0, 30.0, 100.0);
    assert!(check.status != CheckStatus::Fail);
}
