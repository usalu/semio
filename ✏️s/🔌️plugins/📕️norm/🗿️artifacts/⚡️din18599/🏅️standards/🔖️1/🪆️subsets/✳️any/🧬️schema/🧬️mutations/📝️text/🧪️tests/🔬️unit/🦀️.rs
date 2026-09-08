
use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_use_class() {
    store::os_store::test_support::assert_op_line_round_trip(&Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: UseClass::Office }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_update_climate() {
    store::os_store::test_support::assert_op_line_round_trip(&Din18599Mutation::UpdateClimate(update_climate::UpdateClimate {
        new_climate: MonthlyClimate { theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0], g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0] },
    }));
}

/// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip over
/// the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

fn every_mutation() -> Vec<Din18599Mutation> {
    vec![
        Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: UseClass::Office }),
        Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 }),
        Din18599Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: 5 }),
        Din18599Mutation::ChangeHT(change_h_t::ChangeHT { new_h_t: 95.0 }),
        Din18599Mutation::ChangeHV(change_h_v::ChangeHV { new_h_v: 42.0 }),
        Din18599Mutation::ChangeInternalGainsWM2(change_internal_gains_w_m2::ChangeInternalGainsWM2 { new_internal_gains_w_m2: 4.0 }),
        Din18599Mutation::ChangeSolarGainsKwh(change_solar_gains_kwh::ChangeSolarGainsKwh { new_solar_gains_kwh: 90.0 }),
        Din18599Mutation::ChangeSystemLossesKwh(change_system_losses_kwh::ChangeSystemLossesKwh { new_system_losses_kwh: 850.0 }),
        Din18599Mutation::ChangeRenewableKwh(change_renewable_kwh::ChangeRenewableKwh { new_renewable_kwh: 1600.0 }),
        Din18599Mutation::ChangeAnnualLimitKwh(change_annual_limit_kwh::ChangeAnnualLimitKwh { new_annual_limit_kwh: 8000.0 }),
        Din18599Mutation::ChangeEnergyCarrier(change_energy_carrier::ChangeEnergyCarrier { new_energy_carrier: "district_heat".to_string() }),
        Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::ChangeReferenceQPKwh { new_reference_q_p_kwh: 10500.0 }),
        Din18599Mutation::UpdateClimate(update_climate::UpdateClimate {
            new_climate: MonthlyClimate { theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0], g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0] },
        }),
    ]
}
