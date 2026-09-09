use super::*;
#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_annex() {
    store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_t_op_c() {
    store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeTOpC(change_t_op_c::ChangeTOpC { new_t_op_c: 21.5 }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_occupancy() {
    store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeOccupancy(change_occupancy::ChangeOccupancy { new_occupancy: "office".into() }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_persons() {
    store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangePersons(change_persons::ChangePersons { new_persons: 6 }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_sfp_required_class() {
    store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::ChangeSfpRequiredClass { new_sfp_required_class: 2 }));
}

/// ⚖️ Every variant, not just the five hand-picked above — full-coverage `OpText` round trip
/// over the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

fn every_mutation() -> Vec<Din16798Mutation> {
    vec![
        Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        Din16798Mutation::ChangeOccupancy(change_occupancy::ChangeOccupancy { new_occupancy: "office".to_string() }),
        Din16798Mutation::ChangeComfortCategory(change_comfort_category::ChangeComfortCategory { new_comfort_category: "I".to_string() }),
        Din16798Mutation::ChangeTOpC(change_t_op_c::ChangeTOpC { new_t_op_c: 24.5 }),
        Din16798Mutation::ChangeRhPercent(change_rh_percent::ChangeRhPercent { new_rh_percent: 45.0 }),
        Din16798Mutation::ChangeAirSpeedMS(change_air_speed_m_s::ChangeAirSpeedMS { new_air_speed_m_s: 0.15 }),
        Din16798Mutation::ChangeThetaRmC(change_theta_rm_c::ChangeThetaRmC { new_theta_rm_c: 18.0 }),
        Din16798Mutation::ChangeCo2Ppm(change_co2_ppm::ChangeCo2Ppm { new_co2_ppm: 900.0 }),
        Din16798Mutation::ChangeDfPercent(change_df_percent::ChangeDfPercent { new_df_percent: 3.0 }),
        Din16798Mutation::ChangeLAeqDb(change_l_aeq_db::ChangeLAeqDb { new_l_aeq_db: 28.0 }),
        Din16798Mutation::ChangePersons(change_persons::ChangePersons { new_persons: 12 }),
        Din16798Mutation::ChangeIdaClass(change_ida_class::ChangeIdaClass { new_ida_class: "1".to_string() }),
        Din16798Mutation::ChangeVentilationM3H(change_ventilation_m3_h::ChangeVentilationM3H { new_ventilation_m3_h: 320.0 }),
        Din16798Mutation::ChangeFloorAreaM2(change_floor_area_m2::ChangeFloorAreaM2 { new_floor_area_m2: 110.0 }),
        Din16798Mutation::ChangeBedrooms(change_bedrooms::ChangeBedrooms { new_bedrooms: 4 }),
        Din16798Mutation::ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: 70.0 }),
        Din16798Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: 4 }),
        Din16798Mutation::ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: 90.0 }),
        Din16798Mutation::ChangeSfpWM3S(change_sfp_w_m3_s::ChangeSfpWM3S { new_sfp_w_m3_s: 1600.0 }),
        Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::ChangeSfpRequiredClass { new_sfp_required_class: 3 }),
        Din16798Mutation::ChangeHeatRecoveryEta(change_heat_recovery_eta::ChangeHeatRecoveryEta { new_heat_recovery_eta: 0.8 }),
        Din16798Mutation::ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: 0.72 }),
        Din16798Mutation::ChangeSystemType(change_system_type::ChangeSystemType { new_system_type: "decentral_mech".to_string() }),
        Din16798Mutation::ChangeYearsSinceInspection(change_years_since_inspection::ChangeYearsSinceInspection { new_years_since_inspection: 2 }),
        Din16798Mutation::ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: 2.5 }),
        Din16798Mutation::ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: 2.5 }),
        Din16798Mutation::ChangeFanQVM3S(change_fan_q_v_m3_s::ChangeFanQVM3S { new_fan_q_v_m3_s: 1.2 }),
        Din16798Mutation::ChangeFanTRunH(change_fan_t_run_h::ChangeFanTRunH { new_fan_t_run_h: 10.0 }),
        Din16798Mutation::ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: 18.0 }),
        Din16798Mutation::ChangeNightSetbackK(change_night_setback_k::ChangeNightSetbackK { new_night_setback_k: 4.0 }),
        Din16798Mutation::ChangeHrMDotKgS(change_hr_m_dot_kg_s::ChangeHrMDotKgS { new_hr_m_dot_kg_s: 0.6 }),
        Din16798Mutation::ChangeHrCpJKgk(change_hr_cp_j_kgk::ChangeHrCpJKgk { new_hr_cp_j_kgk: 1006.0 }),
        Din16798Mutation::ChangeHrDeltaTC(change_hr_delta_t_c::ChangeHrDeltaTC { new_hr_delta_t_c: 16.0 }),
        Din16798Mutation::ChangeHrTH(change_hr_t_h::ChangeHrTH { new_hr_t_h: 12.0 }),
        Din16798Mutation::ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: 55.0 }),
        Din16798Mutation::ChangeN50HInv(change_n50_h_inv::ChangeN50HInv { new_n50_h_inv: 1.2 }),
        Din16798Mutation::ChangeVolumeM3(change_volume_m3::ChangeVolumeM3 { new_volume_m3: 540.0 }),
        Din16798Mutation::ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: 50.0 }),
        Din16798Mutation::ChangeCellarAreaM2(change_cellar_area_m2::ChangeCellarAreaM2 { new_cellar_area_m2: 55.0 }),
        Din16798Mutation::ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: 18.0 }),
        Din16798Mutation::ChangeHTrWK(change_h_tr_w_k::ChangeHTrWK { new_h_tr_w_k: 220.0 }),
        Din16798Mutation::ChangeHVeWK(change_h_ve_w_k::ChangeHVeWK { new_h_ve_w_k: 110.0 }),
        Din16798Mutation::ChangeThetaEC(change_theta_e_c::ChangeThetaEC { new_theta_e_c: 33.0 }),
        Din16798Mutation::ChangeThetaSetC(change_theta_set_c::ChangeThetaSetC { new_theta_set_c: 25.0 }),
        Din16798Mutation::ChangeCoolingDeltaTH(change_cooling_delta_t_h::ChangeCoolingDeltaTH { new_cooling_delta_t_h: 12.0 }),
        Din16798Mutation::ChangeCoolingGainsKwh(change_cooling_gains_kwh::ChangeCoolingGainsKwh { new_cooling_gains_kwh: 6.0 }),
        Din16798Mutation::ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: 0.85 }),
        Din16798Mutation::ChangeCoolingReferenceKwh(change_cooling_reference_kwh::ChangeCoolingReferenceKwh { new_cooling_reference_kwh: 24.0 }),
        Din16798Mutation::ChangeChillerType(change_chiller_type::ChangeChillerType { new_chiller_type: "water_cooled".to_string() }),
        Din16798Mutation::ChangeEerActual(change_eer_actual::ChangeEerActual { new_eer_actual: 3.4 }),
        Din16798Mutation::ChangeQCKwh(change_q_c_kwh::ChangeQCKwh { new_q_c_kwh: 1200.0 }),
        Din16798Mutation::ChangeGenerationReferenceKwh(change_generation_reference_kwh::ChangeGenerationReferenceKwh { new_generation_reference_kwh: 420.0 }),
        Din16798Mutation::ChangeDataCenterSupplyC(change_data_center_supply_c::ChangeDataCenterSupplyC { new_data_center_supply_c: 24.0 }),
        Din16798Mutation::ChangeHStWK(change_h_st_w_k::ChangeHStWK { new_h_st_w_k: 6.0 }),
        Din16798Mutation::ChangeThetaStC(change_theta_st_c::ChangeThetaStC { new_theta_st_c: 62.0 }),
        Din16798Mutation::ChangeThetaAmbC(change_theta_amb_c::ChangeThetaAmbC { new_theta_amb_c: 21.0 }),
        Din16798Mutation::ChangeStorageTH(change_storage_t_h::ChangeStorageTH { new_storage_t_h: 20.0 }),
        Din16798Mutation::ChangeStorageAllowanceKwh(change_storage_allowance_kwh::ChangeStorageAllowanceKwh { new_storage_allowance_kwh: 7.0 }),
        Din16798Mutation::ChangeDhwDeliveryC(change_dhw_delivery_c::ChangeDhwDeliveryC { new_dhw_delivery_c: 60.0 }),
        Din16798Mutation::ChangeDuctClass(change_duct_class::ChangeDuctClass { new_duct_class: "B".to_string() }),
        Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: 450.0 }),
        Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: 0.08 }),
    ]
}
