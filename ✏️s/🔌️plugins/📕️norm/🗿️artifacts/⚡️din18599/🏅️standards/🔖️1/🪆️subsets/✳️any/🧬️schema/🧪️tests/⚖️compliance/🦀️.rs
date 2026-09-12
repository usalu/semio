use super::*;

fn reference_100m2_inputs() -> BalancingInputs {
    from_building(&reference_wall_layers(), 100.0, 4, ClimateZoneDe::Zone2, 0.0).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn from_building_computes_h_t_from_u_value() {
    let inputs = reference_100m2_inputs();
    let r = total_resistance(&reference_wall_layers(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
    let u = u_value_from_resistance(r);
    let a_env = envelope_area_m2(100.0);
    let side = 10.0;
    let psi_l = 0.10 * side * 4.0;
    let expected_h_t = transmission_loss_coefficient(u * a_env, 0.10 * psi_l, 15.0);
    assert!((inputs.h_t - expected_h_t).abs() < 1e-6);
    assert!(inputs.h_t > 40.0);
}

#[semio_framework_async_macros::async_test]
async fn from_building_computes_h_v_from_ventilation() {
    let inputs = reference_100m2_inputs();
    let airflow = residential_ventilation_rate(100.0, 4);
    assert!((airflow - 120.0).abs() < 1e-9);
    let expected_h_v = ventilation_loss_coefficient(airflow, 0.0);
    assert!((inputs.h_v - expected_h_v).abs() < 1e-6);
    assert!((inputs.h_v - 40.8).abs() < 0.1);
}

#[semio_framework_async_macros::async_test]
async fn reference_100m2_q_t_numeric() {
    let inputs = reference_100m2_inputs();
    let q_t = part_2::transmission_losses_kwh(&inputs);
    assert!((q_t - 11_054.56).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn reference_100m2_q_v_numeric() {
    let inputs = reference_100m2_inputs();
    let q_v = part_3::ventilation_losses_kwh(&inputs);
    assert!((q_v - 4_896.01).abs() < 1.0);
}

#[semio_framework_async_macros::async_test]
async fn reference_100m2_q_p_numeric() {
    let inputs = reference_100m2_inputs();
    let q_p = part_10::primary_energy_kwh(&inputs);
    assert!((q_p - 19_608.96).abs() < 5.0);
}

#[semio_framework_async_macros::async_test]
async fn dhw_from_occupants_4_persons() {
    let inputs = reference_100m2_inputs();
    let q_w = part_9::dhw_demand_kwh(&inputs);
    assert!((q_w - 3600.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn cooling_degree_hours_zone2_positive() {
    let climate = MonthlyClimate::german_reference(ClimateZoneDe::Zone2);
    let cdh = part_8::cooling_degree_hours(&climate);
    assert!(cdh > 1000.0);
    let inputs = reference_100m2_inputs();
    let q_c = part_8::cooling_demand_kwh(&inputs);
    assert!(q_c > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn part_4_internal_gains_positive() {
    let inputs = reference_100m2_inputs();
    let q_i = part_4::internal_gains_kwh(&inputs);
    assert!(q_i > 1000.0);
}

#[semio_framework_async_macros::async_test]
async fn part_5_solar_gains_positive() {
    let inputs = reference_100m2_inputs();
    let q_s = part_5::solar_gains_kwh(&inputs);
    assert!(q_s > 0.0, "q_s={q_s}");
}

#[semio_framework_async_macros::async_test]
async fn part_6_system_losses_scale_with_area() {
    let inputs = reference_100m2_inputs();
    let q_sys = part_6::system_losses_kwh(&inputs);
    assert!((q_sys - 800.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn part_11_automation_factor_residential() {
    let inputs = reference_100m2_inputs();
    assert!((part_11::automation_factor(&inputs) - 0.95).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn part_12_tabular_distinct_from_part_10() {
    let inputs = reference_100m2_inputs();
    let q_p = part_10::primary_energy_kwh(&inputs);
    let q_tab = part_12::tabular_primary_energy_kwh(&inputs);
    assert!((q_tab - 9500.0).abs() < 1.0, "q_tab={q_tab}");
    assert!((q_p - q_tab).abs() > 100.0, "tabular must differ from detailed: q_p={q_p}, q_tab={q_tab}");
}

#[semio_framework_async_macros::async_test]
async fn primary_energy_factor_table_cited() {
    assert!((primary_energy_factor("natural_gas") - 1.1).abs() < 1e-9);
    assert!((primary_energy_factor("district_heat") - 0.7).abs() < 1e-9);
    assert!((primary_energy_factor("electricity_grid") - 1.8).abs() < 1e-9);
    assert!((reference_area_factor(UseClass::Office) - 1.20).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn part_1_primary_energy_aggregation_worked_example() {
    let deliveries = vec![part_1::FinalEnergyDelivery { carrier: "natural_gas".into(), q_f_kwh: 10_000.0 }, part_1::FinalEnergyDelivery { carrier: "electricity_grid".into(), q_f_kwh: 2_000.0 }];
    let q_p = part_1::aggregate_primary_energy_kwh(&deliveries);
    assert!((q_p - 14_600.0).abs() < 1e-9, "q_p = {q_p}, expected 10000*1.1 + 2000*1.8 = 14600");
}

#[semio_framework_async_macros::async_test]
async fn part_8_cooling_demand_numeric_worked_example() {
    let inputs = reference_100m2_inputs();
    let cdh = part_8::cooling_degree_hours(&crate::din18599_climate(&inputs));
    let expected_q_c = (inputs.h_t + inputs.h_v) * cdh * 0.35 / 1000.0;
    let q_c = part_8::cooling_demand_kwh(&inputs);
    assert!((q_c - expected_q_c).abs() < 1e-6, "q_c = {q_c}, expected {expected_q_c}");
    assert!((q_c - 69.23).abs() < 1.0, "q_c = {q_c}, expected ~69.23 kWh");
}

#[semio_framework_async_macros::async_test]
async fn reference_residential_matches_from_building() {
    let via_helper = reference_residential(ClimateZoneDe::Zone2, 100.0);
    let via_from_building = reference_100m2_inputs();
    assert_eq!(via_helper, via_from_building);
}
