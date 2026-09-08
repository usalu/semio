
use super::*;

#[semio_framework_async_macros::async_test]
async fn masonry_wall_sigma_vs_fd() {
    let sigma = 200.0 * 1000.0 / 500_000.0;
    let f_d = part_1_1::design_strength_mpa(5.0, na_de::gamma_m());
    assert!((sigma - 0.4_f64).abs() < 1e-9);
    assert!((f_d - (5.0 / 1.5)).abs() < 1e-9);
    let report = check_masonry_wall(200.0, 500_000.0, 5.0, na_de::gamma_m());
    assert!(!report.checks.is_empty());
    assert!(report.checks[0].utilization < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn masonry_wall_e2e() {
    let report = check_masonry_wall(200.0, 500_000.0, 5.0, 2.0);
    assert!(!report.checks.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn fire_wall_r60_clay() {
    let required = part_1_2::required_wall_thickness_mm(60, MasonryUnit::Clay);
    assert!((required - 90.0).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn exposure_mortar_mx1_clay_m1_admissible() {
    let result = part_2::check_exposure_mortar(crate::part_2::ExposureClass::Mx1, MasonryUnit::Clay, crate::part_2::MortarClass::M1);
    assert_eq!(result.status, CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn exposure_mortar_mx4_aac_inadmissible() {
    let result = part_2::check_exposure_mortar(crate::part_2::ExposureClass::Mx4, MasonryUnit::Aac, crate::part_2::MortarClass::M20);
    assert_eq!(result.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn bed_joint_thickness_range() {
    let ok = part_2::check_bed_joint_thickness(12.0);
    assert_eq!(ok.status, CheckStatus::Pass);
    let too_thin = part_2::check_bed_joint_thickness(3.0);
    assert_eq!(too_thin.status, CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn simplified_method_worked_example() {
    let phi_s = part_3::phi_s(3600.0, 240.0);
    assert!((phi_s - 0.6025).abs() < 1e-9);
    let gamma_m = AnnexParams { annex: AnnexChoice::De, masonry_class: MasonryClass::Class3, accidental: false }.gamma_m();
    assert!((gamma_m - 1.5).abs() < 1e-9);
    let f_d = part_1_1::design_strength_mpa(5.0, gamma_m);
    assert!((f_d - 10.0 / 3.0).abs() < 1e-9);
    let n_rd = part_3::n_rd_kn(phi_s, f_d, 240_000.0);
    assert!((n_rd - 482.0).abs() < 0.5);
}

#[semio_framework_async_macros::async_test]
async fn simplified_method_applicability_guard() {
    assert!(part_3::is_applicable(2, 2500.0, 240.0));
    assert!(!part_3::is_applicable(4, 2500.0, 240.0));
    assert!(!part_3::is_applicable(2, 8000.0, 240.0));
    let result = part_3::check_simplified_compression(100.0, 0.6, 3.0, 100_000.0, 4, 2500.0, 240.0, AnnexChoice::De);
    assert_eq!(result.status, CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn gamma_m_diverges_en_class2_vs_de_flat() {
    let en = AnnexParams { annex: AnnexChoice::En, masonry_class: MasonryClass::Class2, accidental: false };
    let de = AnnexParams { annex: AnnexChoice::De, masonry_class: MasonryClass::Class2, accidental: false };
    assert!((en.gamma_m() - 1.7).abs() < 1e-9);
    assert!((de.gamma_m() - 1.5).abs() < 1e-9);
    let phi_s = part_3::phi_s(3600.0, 240.0);
    let area_mm2 = 240_000.0;
    let n_rd_en = part_3::n_rd_kn(phi_s, part_1_1::design_strength_mpa(5.0, en.gamma_m()), area_mm2);
    let n_rd_de = part_3::n_rd_kn(phi_s, part_1_1::design_strength_mpa(5.0, de.gamma_m()), area_mm2);
    assert!(n_rd_de > n_rd_en);
    assert!((n_rd_de - n_rd_en).abs() > 1.0);
}
