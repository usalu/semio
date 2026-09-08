
use super::*;

#[semio_framework_async_macros::async_test]
async fn pmv_simplified_neutral_at_reference() {
    let pmv = part_1::pmv_simplified(25.0, 50.0, 0.1);
    assert!(pmv.abs() < 0.01);
}

#[semio_framework_async_macros::async_test]
async fn pmv_comfort_passes_for_office_conditions() {
    let check = part_1::check_pmv_comfort(24.0, 50.0, 0.1);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
    assert!(check.utilization < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn pmv_iso7730_neutral_at_comfort_conditions() {
    let pmv = part_1::pmv_iso7730(22.0, 50.0, 0.1);
    assert!(pmv.abs() < 0.5, "pmv={pmv}");
    let ppd = part_1::ppd_from_pmv(pmv);
    assert!(ppd < 10.0, "ppd={ppd}");
}

#[semio_framework_async_macros::async_test]
async fn adaptive_comfort_center_and_category_band() {
    let centre = part_1::adaptive_comfort_temperature_c(20.0);
    assert!((centre - 25.4).abs() < 1e-9, "centre={centre}");
    let check = part_1::check_adaptive_comfort(20.0, 24.0, part_1::ComfortCategory::II);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn co2_annex_divergence_de_stricter_than_en() {
    let en = annex_params::AnnexParams::en();
    let de = annex_params::AnnexParams::de();
    assert!((en.co2_limit_classroom_ppm - 1000.0).abs() < 1e-9);
    assert!((de.co2_limit_classroom_ppm - 800.0).abs() < 1e-9);
    let en_check = part_1::check_co2_level(OccupancyType::Classroom, 850.0, &en);
    let de_check = part_1::check_co2_level(OccupancyType::Classroom, 850.0, &de);
    assert_eq!(en_check.status, crate::document::CheckStatus::Pass);
    assert_eq!(de_check.status, crate::document::CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn daylight_factor_category_ii_minimum() {
    assert!((part_1::daylight_factor_min_percent(part_1::ComfortCategory::II) - 2.0).abs() < 1e-9);
    let pass = part_1::check_daylight_factor(part_1::ComfortCategory::II, 2.5);
    assert_eq!(pass.status, crate::document::CheckStatus::Pass);
    let fail = part_1::check_daylight_factor(part_1::ComfortCategory::II, 1.0);
    assert_eq!(fail.status, crate::document::CheckStatus::Fail);
}

#[semio_framework_async_macros::async_test]
async fn acoustic_category_ii_limit() {
    let check = part_1::check_acoustic_category(part_1::ComfortCategory::II, 24.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn ventilation_rates_per_room_type_at_ida2() {
    assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Office), 36.0);
    assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Meeting), 36.0);
    assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Classroom), 36.0);
    assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Retail), 20.0);
    assert_eq!(part_3::outdoor_air_per_person(OccupancyType::Kitchen), 60.0);
    let office = part_3::check_ventilation_rate(OccupancyType::Office, 10, part_3::IdaClass::Ida2, 360.0);
    assert_eq!(office.status, crate::document::CheckStatus::Pass);
    assert!((office.limit.value - 360.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn sfp_1500w_1m3s_falls_in_class_4() {
    let sfp_w_m3_s = 1500.0_f64 / 1.0;
    assert!((sfp_w_m3_s - 1500.0).abs() < 1e-9);
    assert_eq!(part_3::classify_sfp(sfp_w_m3_s), part_3::SfpClass::Sfp4);
    let check = part_3::check_design_sfp(sfp_w_m3_s, part_3::SfpClass::Sfp4);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn dwelling_ventilation_85m2_3_bedrooms() {
    let rate = part_3::dwelling_ventilation_rate(85.0, 3);
    assert!((rate - 63.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn residential_ventilation_rate_100m2_4_occupants() {
    let rate = part_3::residential_ventilation_rate(100.0, 4);
    assert!((rate - 120.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn heat_recovery_efficiency_minimum() {
    let check = part_3::check_heat_recovery_efficiency(0.75, 0.70);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn inspection_due_within_interval() {
    let check = part_3::check_inspection_due("central_mech", 1);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn humidification_capacity_meets_requirement() {
    let check = part_3::check_humidification_capacity(2.0, 2.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn fan_energy_1500w_1m3s_8h_is_12kwh() {
    let energy = part_5_1::fan_energy_kwh(1500.0, 1.0, 8.0);
    assert!((energy - 12.0).abs() < 1e-9, "energy={energy}");
    let check = part_5_1::check_building_fan_energy(1500.0, 1.0, 8.0, 15.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn night_setback_residential_minimum() {
    let check = part_5_1::check_night_setback(OccupancyType::Residential, 3.5);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn heat_recovery_savings_worked_example() {
    let savings = part_5_2::heat_recovery_savings_kwh(0.75, 0.5, part_5_2::AIR_CP_J_KGK, 15.0, 10.0);
    assert!((savings - 56.53125).abs() < 1e-6, "savings={savings}");
    let check = part_5_2::check_heat_recovery_savings(0.75, 0.5, part_5_2::AIR_CP_J_KGK, 15.0, 10.0, 50.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn infiltration_n50_1_5_volume_500m3_is_37_5() {
    let rate = part_7::infiltration_rate_m3_h(1.5, 500.0);
    assert!((rate - 37.5).abs() < 1e-9, "rate={rate}");
    let check = part_7::check_infiltration(1.5, 500.0, 45.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn cellar_ventilation_50m2() {
    let rate = part_7::cellar_ventilation_rate(50.0);
    assert!((rate - 15.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn cooling_energy_need_worked_example() {
    let net = part_9::cooling_energy_need_kwh(200.0, 100.0, 32.0, 26.0, 10.0, 5.0, 0.8);
    assert!((net - 14.0).abs() < 1e-9, "net={net}");
    let check = part_9::check_cooling_energy_need(200.0, 100.0, 32.0, 26.0, 10.0, 5.0, 0.8, 20.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn chiller_eer_table_lookup_air_cooled() {
    assert!((part_13::eer_min(part_13::ChillerType::AirCooled) - 2.5).abs() < 1e-9);
    let check = part_13::check_chiller_eer(part_13::ChillerType::AirCooled, 3.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
    let generation = part_13::generation_energy_kwh(1000.0, 3.0);
    assert!((generation - 333.3333333333333).abs() < 1e-6, "generation={generation}");
}

#[semio_framework_async_macros::async_test]
async fn data_center_supply_air_22c_passes() {
    let check = part_13::check_supply_air_temperature(22.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn storage_losses_worked_example() {
    let losses = part_15::storage_losses_kwh(5.0, 60.0, 20.0, 24.0);
    assert!((losses - 4.8).abs() < 1e-9, "losses={losses}");
    let check = part_15::check_storage_losses(5.0, 60.0, 20.0, 24.0, 6.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn dhw_delivery_temperature_58c_passes() {
    let check = part_15::check_dhw_temperature(58.0);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn duct_leakage_class_c_400pa_worked_example() {
    let limit = part_17::leakage_limit_m3_s_m2(part_17::DuctLeakageClass::C, 400.0);
    assert!((limit - 0.1473873631338949).abs() < 1e-6, "limit={limit}");
    let check = part_17::check_duct_leakage(part_17::DuctLeakageClass::C, 400.0, 0.10);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}
