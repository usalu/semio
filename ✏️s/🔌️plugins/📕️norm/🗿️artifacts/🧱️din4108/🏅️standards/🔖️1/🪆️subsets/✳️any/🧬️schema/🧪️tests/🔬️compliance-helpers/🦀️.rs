
use super::*;

fn sample_wall() -> Vec<part_2::Layer> {
    vec![part_2::Layer { thickness_m: 0.24, lambda_w_mk: 0.81 }, part_2::Layer { thickness_m: 0.14, lambda_w_mk: 0.035 }]
}

fn sample_moisture_wall() -> Vec<part_3::MoistureLayer> {
    vec![part_3::MoistureLayer { thickness_m: 0.24, lambda_w_mk: 0.81, mu: 15.0 }, part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 }]
}

#[semio_framework_async_macros::async_test]
async fn worked_example_u_value_known_wall() {
    let layers = sample_wall();
    let r = part_2::total_resistance(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
    let u = part_2::u_value_from_resistance(r);
    assert!((u - 0.224).abs() < 0.01, "U = {u}, expected ~0.224");
    assert!((r - 4.466).abs() < 0.02, "R = {r}, expected ~4.466");
}

#[semio_framework_async_macros::async_test]
async fn worked_example_f_rsi_above_minimum() {
    let layers = sample_moisture_wall();
    let f = part_3::interior_surface_temperature_factor(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.5);
    assert!(f > F_RSI_MINIMUM, "f_Rsi = {f}, must exceed {F_RSI_MINIMUM}");
    let check = part_3::check_surface_temperature(&layers, 20.0, -14.0, 0.5).unwrap();
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn worked_example_glaser_no_condensation_insulated_wall() {
    let layers = sample_moisture_wall();
    assert!(!part_3::condensation_at_interfaces(&layers, 20.0, -14.0, 0.5));
    let check = part_3::check_glaser_moisture(&layers, 20.0, -14.0, 0.5).unwrap();
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn worked_example_magnus_saturation_at_zero_c() {
    let e = part_3::saturation_vapor_pressure_pa(0.0);
    assert!((e - 611.2).abs() < 1.0, "e_sat(0°C) = {e}");
}

#[semio_framework_async_macros::async_test]
async fn worked_example_vapor_resistance_formula() {
    let layer = part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 };
    let r_mu = part_3::vapor_resistance(&layer);
    let expected = 0.14 / (1.3 * 0.035);
    assert!((r_mu - expected).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn part_2_colder_zone_allows_higher_u_limit() {
    let limit_warm = part_2::climate_adjusted_u_limit(part_2::BuildingCategory::Residential, ClimateZoneDe::Zone4);
    let limit_cold = part_2::climate_adjusted_u_limit(part_2::BuildingCategory::Residential, ClimateZoneDe::Zone1);
    assert!(limit_cold > limit_warm, "zone1={limit_cold}, zone4={limit_warm}");
    assert!((limit_cold - 0.308).abs() < 0.01);
}

#[semio_framework_async_macros::async_test]
async fn part_4_mineral_wool_lambda() {
    let r = part_4::check_design_lambda("mineral_wool", 0.038).unwrap();
    assert_eq!(r.status, crate::document::CheckStatus::Pass);
    let design = part_4::design_lambda_for_material("mineral_wool").unwrap();
    assert!((design - 0.0385).abs() < 0.001);
}

#[semio_framework_async_macros::async_test]
async fn part_4_has_fifteen_plus_materials() {
    let materials = ["mineral_wool", "glass_wool", "eps", "xps", "pur", "pir", "wood_fibre", "cellulose", "concrete", "aerated_concrete", "brick", "sand_lime_brick", "timber", "plywood", "gypsum_plaster", "lime_plaster", "clay_plaster"];
    assert!(materials.len() >= 15);
    for m in materials {
        assert!(part_4::material_design(m).is_ok(), "missing {m}");
    }
}

#[semio_framework_async_macros::async_test]
async fn part_5_summer_heat_zone_dependent() {
    let layers = sample_wall();
    let flux_z2 = part_5::peak_summer_heat_flux_w_m2(&layers, ClimateZoneDe::Zone2, 26.0, 0.6, 600.0);
    let flux_z4 = part_5::peak_summer_heat_flux_w_m2(&layers, ClimateZoneDe::Zone4, 26.0, 0.6, 600.0);
    assert!(flux_z4 > flux_z2);
    let check = part_5::check_summer_heat_protection(&layers, ClimateZoneDe::Zone2, 26.0, 0.6, 600.0).unwrap();
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn part_6_thermal_bridge_increases_u() {
    let layers = sample_wall();
    let u_element = part_2::u_value_from_resistance(part_2::total_resistance(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
    let u_bridged = part_6::u_value_with_thermal_bridges(u_element, 0.05);
    assert!((u_bridged - (u_element + 0.05)).abs() < 1e-9);
    let check = part_6::check_u_value_with_bridges(&layers, 0.05, 0.35).unwrap();
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn part_8_catalog_lookup() {
    let entry = part_8::catalog_entry("AW-01").unwrap();
    assert!((entry.u_typical_w_m2k - 0.24).abs() < 0.01);
    let u = part_2::u_value_from_resistance(part_2::total_resistance(&sample_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
    let check = part_8::check_against_catalog("AW-01", u).unwrap();
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn part_1_plausibility_flags_implausible_u_value() {
    let layers = sample_wall();
    let ok = part_1::check_input_plausibility(&layers, 0.224);
    assert_eq!(ok.status, crate::document::CheckStatus::Pass);
    let bad = part_1::check_input_plausibility(&layers, 12.0);
    assert_eq!(bad.status, crate::document::CheckStatus::Fail);
    let na = part_1::check_input_plausibility(&[], 0.3);
    assert_eq!(na.status, crate::document::CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn part_10_application_class_admissibility() {
    let admissible = part_10::check_application_class(part_10::ApplicationType::Deo, part_10::ApplicationClass::Dk);
    assert_eq!(admissible.status, crate::document::CheckStatus::Pass);
    let inadmissible = part_10::check_application_class(part_10::ApplicationType::Duk, part_10::ApplicationClass::Dm);
    assert_eq!(inadmissible.status, crate::document::CheckStatus::Fail);
    assert_eq!(part_10::minimum_class(part_10::ApplicationType::Duk), part_10::ApplicationClass::Dg);
}

#[semio_framework_async_macros::async_test]
async fn bb2_worked_example_conform_details_pass() {
    let psi_l_sum = 18.0;
    let area = 400.0;
    let delta = bb_2::delta_u_wb_actual_w_m2k(psi_l_sum, area);
    assert!((delta - 0.045).abs() < 1e-9, "delta = {delta}");
    let check = bb_2::check_beiblatt_2_equivalence(psi_l_sum, area, true).unwrap();
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn bb2_non_conform_details_fall_back_to_flat_rate_surcharge() {
    let psi_l_sum = 32.0;
    let area = 400.0;
    let check = bb_2::check_beiblatt_2_equivalence(psi_l_sum, area, false).unwrap();
    assert!((check.limit.value - bb_2::DELTA_U_WB_FLAT_RATE_W_M2K).abs() < 1e-9);
    assert_eq!(check.status, crate::document::CheckStatus::Pass);
    let over_flat_rate = bb_2::check_beiblatt_2_equivalence(45.0, area, false).unwrap();
    assert_eq!(over_flat_rate.status, crate::document::CheckStatus::Fail);
}
