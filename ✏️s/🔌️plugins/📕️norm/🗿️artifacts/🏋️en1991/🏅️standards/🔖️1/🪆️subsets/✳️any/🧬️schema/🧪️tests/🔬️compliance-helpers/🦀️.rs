use super::*;

#[semio_framework_async_macros::async_test]
async fn snow_zone_2_ground_load() {
    assert!((part_1_3::ground_snow_load_zone(2) - 0.85).abs() < 1e-9);
    assert!((na_de::SnowZone::Zone2.s_k_kn_m2() - 0.85).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn wind_peak_velocity_pressure_vb_25() {
    let q_b = part_1_4::basic_velocity_pressure(1.25, 25.0);
    assert!((q_b - 0.39).abs() < 0.01);
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, 25.0, c_e);
    assert!(q_p > q_b);
}

#[semio_framework_async_macros::async_test]
async fn imposed_categories_table_6_1() {
    assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::A), 2.0);
    assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::B), 2.5);
    assert_eq!(part_1_1::imposed_load_kn_m2(ImposedCategory::H), 20.0);
}

#[semio_framework_async_macros::async_test]
async fn de_wind_zone_2_basic_velocity() {
    assert!((na_de::WindZone::Zone2.v_b_m_s() - 25.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn snow_and_wind_de_vs_en_diverge_at_altitude() {
    let doc = crate::En1991Snapshot { snow_altitude_m: 400.0, annex: AnnexChoice::De, ..crate::En1991Snapshot::default() };
    let de_s_k = part_1_3::design_ground_snow_load(doc.annex, doc.snow_zone, doc.snow_altitude_m, doc.en_s_k_kn_m2);
    let en_s_k = part_1_3::design_ground_snow_load(AnnexChoice::En, doc.snow_zone, doc.snow_altitude_m, doc.en_s_k_kn_m2);
    assert!(de_s_k > en_s_k);
    assert!((en_s_k - doc.en_s_k_kn_m2).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn bridge_lm1_alpha_q_diverges_de_vs_en() {
    let de = part_2::check_lm1_moment(AnnexChoice::De, 20.0, 1, 3.0, 3000.0);
    let en = part_2::check_lm1_moment(AnnexChoice::En, 20.0, 1, 3.0, 3000.0);
    assert!(de.computed.value < en.computed.value);
}
