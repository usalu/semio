
use super::*;

#[semio_framework_async_macros::async_test]
async fn full_actions_de_na_numeric() {
    let doc = En1991Snapshot::default();
    let annex = NaDe;
    let report = check_full_actions(&doc);
    assert_eq!(report.checks.len(), 11);
    let imposed_q = part_1_1::imposed_load_kn_m2(ImposedCategory::B) * doc.area_m2 * annex.psi_0("office");
    assert!((report.checks[0].computed.value / 1000.0 - imposed_q).abs() < 1e-6);
    assert!((report.checks[1].computed.value / 1000.0 - 5.0).abs() < 1e-6);
    let theta_g = part_1_2::standard_gas_temperature_c(30.0);
    assert!((theta_g - 841.79588).abs() < 1e-4);
    assert!((report.checks[2].computed.value - theta_g).abs() < 1e-6);
    let snow = part_1_3::roof_snow_load(part_1_3::ground_snow_load_zone(2), 0.8);
    assert!((report.checks[3].computed.value - snow * 1000.0).abs() < 1e-6);
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, 25.0, c_e);
    let w_p = part_1_4::wind_pressure(q_p, 0.8, 0.2) * part_1_4::structural_factor(1.0, 1.0);
    assert!((report.checks[4].computed.value - w_p * 1000.0).abs() < 1e-3);
    assert!((report.checks[5].computed.value - doc.delta_t_k).abs() < 1e-6);
    assert!((report.checks[6].computed.value / 1000.0 - 1.0).abs() < 1e-6);
    let impact = part_1_7::impact_force_kn(30.0, 80.0);
    assert!((report.checks[7].computed.value / 1000.0 - impact).abs() < 1e-6);
    assert!((impact - 7.407407407407407).abs() < 1e-6);
    assert!((report.checks[8].computed.value / 1000.0 - 2700.0).abs() < 1e-6);
    let silo_p = part_4::janssen_horizontal_pressure_kpa(8.0, 1.5, 0.4, 0.4, 12.0);
    assert!((silo_p - 54.147).abs() < 1e-2);
    assert!((report.checks[10].computed.value - silo_p * 1000.0).abs() < 1e-6);
    assert!(report.all_pass());
}

#[semio_framework_async_macros::async_test]
async fn evaluate_reaches_every_part_module() {
    let report = evaluate(&En1991Snapshot::default());
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-1")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-2")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-3")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-4")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-5")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-6")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-1-7")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-2")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-3")));
    assert!(report.checks.iter().any(|c| c.clause.family.contains("1991-4")));
}
