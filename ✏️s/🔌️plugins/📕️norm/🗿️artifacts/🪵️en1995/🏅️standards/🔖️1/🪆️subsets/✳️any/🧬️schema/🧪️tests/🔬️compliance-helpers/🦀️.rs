
use super::*;

#[semio_framework_async_macros::async_test]
async fn k_mod_sc1_permanent() {
    assert!((k_mod(ServiceClass::Sc1, LoadDuration::Permanent) - 0.6).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn glulam_ltb_reduces_bending_resistance() {
    let w = 1_000_000.0;
    let f_m_k = 24.0;
    let km = k_mod(ServiceClass::Sc1, LoadDuration::Medium);
    let m_crit = 30.0;
    let lambda = lambda_rel_m(w, f_m_k, m_crit);
    let kc = k_crit(lambda);
    assert!(kc < 1.0);
    let m_rd = part_1_1::bending_resistance_knm(w, f_m_k, km, kc, AnnexChoice::De);
    let m_rd_full = part_1_1::bending_resistance_knm(w, f_m_k, km, 1.0, AnnexChoice::De);
    assert!(m_rd < m_rd_full);
}

#[semio_framework_async_macros::async_test]
async fn glulam_beam_e2e() {
    let report = check_glulam_beam(25.0, 50.0, 15.0, 1_000_000.0, 20_000.0, 200.0, 300.0, 24.0, 21.0, 4.0, ServiceClass::Sc1, LoadDuration::Medium, 80.0, AnnexChoice::De);
    assert_eq!(report.checks.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn connection_bearing_resistance_worked() {
    let km = k_mod(ServiceClass::Sc1, LoadDuration::Medium);
    let f_rd = part_1_1::connection_bearing_resistance_kn(12_000.0, 2.5, km, AnnexChoice::De);
    assert!((f_rd - 18.46).abs() < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn fire_char_depth_r30() {
    let charred = part_1_2::charred_depth_mm(30.0);
    assert!((charred - 19.5).abs() < 0.1);
    let remaining = part_1_2::residual_section_mm(300.0, charred);
    assert!((remaining - 261.0).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn annex_params_gamma_m_per_material() {
    let en = AnnexParams::en();
    let de = AnnexParams::de();
    assert!((en.gamma_m(TimberMaterial::Solid) - 1.3).abs() < 1e-9);
    assert!((en.gamma_m(TimberMaterial::Glulam) - 1.25).abs() < 1e-9);
    assert!((en.gamma_m(TimberMaterial::Connection) - 1.3).abs() < 1e-9);
    assert!((en.gamma_m(TimberMaterial::Solid) - de.gamma_m(TimberMaterial::Solid)).abs() < 1e-9);
    assert!((en.gamma_m(TimberMaterial::Glulam) - de.gamma_m(TimberMaterial::Glulam)).abs() < 1e-9);
    assert!((en.gamma_m(TimberMaterial::Connection) - de.gamma_m(TimberMaterial::Connection)).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn k_cr_diverges_between_en_and_de_for_c24() {
    let f_v_k = 4.0;
    let k_cr_en = AnnexParams::en().k_cr(f_v_k);
    let k_cr_de = AnnexParams::de().k_cr(f_v_k);
    assert!((k_cr_en - 0.67).abs() < 1e-9);
    assert!((k_cr_de - 0.625).abs() < 1e-9);
    assert!((k_cr_en - k_cr_de).abs() > 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn shear_utilization_diverges_between_annexes_for_c24() {
    let v_ed_kn = 15.0;
    let b_mm = 200.0;
    let h_mm = 300.0;
    let f_v_k = 4.0;
    let km = k_mod(ServiceClass::Sc1, LoadDuration::Medium);
    let check_en = part_1_1::check_shear(v_ed_kn, b_mm, h_mm, f_v_k, km, AnnexChoice::En);
    let check_de = part_1_1::check_shear(v_ed_kn, b_mm, h_mm, f_v_k, km, AnnexChoice::De);
    assert!((check_en.utilization - check_de.utilization).abs() > 1e-3, "EN and DE shear utilizations must diverge for C24");
    assert!(check_de.utilization > check_en.utilization, "smaller DE k_cr yields a smaller effective width and thus a higher utilization");
    assert!((check_de.utilization - 0.2438).abs() < 0.001);
    assert!((check_en.utilization - 0.2274).abs() < 0.001);
}

#[semio_framework_async_macros::async_test]
async fn pedestrian_vibration_within_comfort_limit() {
    let check = part_2::check_pedestrian_vibration(0.3);
    assert!(check.utilization < 1.0);
    let check_exceeding = part_2::check_pedestrian_vibration(1.0);
    assert!(check_exceeding.utilization > 1.0);
}

#[semio_framework_async_macros::async_test]
async fn fatigue_reduction_factor_degrades_with_cycles() {
    assert!((part_2::fatigue_reduction_factor(500_000.0) - 1.0).abs() < 1e-9);
    let k_fat = part_2::fatigue_reduction_factor(1.0e8);
    assert!((0.5..1.0).contains(&k_fat));
}
