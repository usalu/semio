use super::*;

#[semio_framework_async_macros::async_test]
async fn snow_zone2_ground_load_pa() {
    let sk = part_1_3::ground_snow_pa("2", 150.0);
    assert!((sk - 850.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn snow_zone1a_diverges_from_zone1() {
    let z1 = part_1_3::ground_snow_pa("1", 500.0);
    let z1a = part_1_3::ground_snow_pa("1a", 500.0);
    assert!(z1a > z1);
    assert!((z1a / z1 - 1.25).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn de_vs_en_snow_diverges() {
    let de = part_1_3::design_ground_snow_pa(AnnexChoice::De, "2", 400.0, 850.0);
    let en = part_1_3::design_ground_snow_pa(AnnexChoice::En, "2", 400.0, 850.0);
    assert!(de > en);
    assert!((en - 850.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn imposed_category_b1_de() {
    let q = part_1_1::imposed_qk_pa("B1", AnnexChoice::De);
    assert!((q - 2000.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn wind_basic_and_exposure() {
    let q_b = part_1_4::basic_velocity_pressure_pa(1.25, 25.0);
    assert!((q_b - 390.625).abs() < 1e-6);
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    assert!(c_e > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn wind_na_b3_zone2_cat2() {
    let qp = part_1_4::na_b3_qp_pa(2, 2, 12.0);
    assert!(qp > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn de_wind_zone_2_basic_velocity() {
    assert!((na_de::WindZone::Zone2.v_b0_m_s() - 25.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn bridge_lm1_alpha_q_diverges_de_vs_en() {
    let de = part_2::lm1_tandem_n(AnnexChoice::De, 1);
    let en = part_2::lm1_tandem_n(AnnexChoice::En, 1);
    assert!((de - 270_000.0).abs() < 1e-6);
    assert!((en - 300_000.0).abs() < 1e-6);
    assert!(de < en);
}

#[semio_framework_async_macros::async_test]
async fn bridge_lm1_udl_de_alpha_q() {
    let de = part_2::lm1_udl_pa(AnnexChoice::De, 1);
    let en = part_2::lm1_udl_pa(AnnexChoice::En, 1);
    assert!((de - 3600.0).abs() < 1e-6);
    assert!((en - 9000.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn crane_horizontal_de_vs_en() {
    let de = part_3::design_horizontal_force_n("HC2", AnnexChoice::De);
    let en = part_3::design_horizontal_force_n("HC2", AnnexChoice::En);
    assert!(de > en);
}

#[semio_framework_async_macros::async_test]
async fn silo_patch_de_vs_en() {
    let de = part_4::patch_pressure_pa(8000.0, 1.5, 0.4, 0.4, 12.0, AnnexChoice::De);
    let en = part_4::patch_pressure_pa(8000.0, 1.5, 0.4, 0.4, 12.0, AnnexChoice::En);
    assert!(de > en);
}
