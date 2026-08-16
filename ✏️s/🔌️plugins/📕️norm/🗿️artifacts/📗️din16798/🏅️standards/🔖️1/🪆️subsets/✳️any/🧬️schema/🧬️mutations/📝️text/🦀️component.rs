//! 🔧️ Din16798 artifact — OpText/OpBinary codecs for `Din16798Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro, `impl_norm_set_snapshot_ops!`, no longer applies now that the
//! whole-document-replace variant is gone).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::din16798::schema::mutations::Din16798Mutation;
use crate::artifacts::din16798::schema::mutations::{
    change_air_speed_m_s, change_annex, change_bedrooms, change_cellar_area_m2, change_cellar_ventilation_m3_h, change_chiller_type, change_co2_ppm, change_comfort_category, change_cooling_delta_t_h, change_cooling_gains_kwh,
    change_cooling_reference_kwh, change_cooling_utilization_factor, change_data_center_supply_c, change_df_percent, change_dhw_delivery_c, change_duct_class, change_duct_leakage_m3_s_m2, change_duct_test_pressure_pa,
    change_dwelling_ventilation_m3_h, change_eer_actual, change_fan_energy_reference_kwh, change_fan_q_v_m3_s, change_fan_t_run_h, change_floor_area_m2, change_generation_reference_kwh, change_h_st_w_k, change_h_tr_w_k, change_h_ve_w_k,
    change_heat_recovery_eta, change_heat_recovery_eta_min, change_hr_cp_j_kgk, change_hr_delta_t_c, change_hr_m_dot_kg_s, change_hr_savings_reference_kwh, change_hr_t_h, change_humidification_provided_kg_h, change_humidification_required_kg_h,
    change_ida_class, change_infiltration_allowance_m3_h, change_l_aeq_db, change_n50_h_inv, change_night_setback_k, change_occupancy, change_occupants, change_persons, change_q_c_kwh, change_residential_ventilation_m3_h, change_rh_percent,
    change_sfp_required_class, change_sfp_w_m3_s, change_storage_allowance_kwh, change_storage_t_h, change_system_type, change_t_op_c, change_theta_amb_c, change_theta_e_c, change_theta_rm_c, change_theta_set_c, change_theta_st_c,
    change_ventilation_m3_h, change_volume_m3, change_years_since_inspection,
};
use crate::document::AnnexChoice;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `Din16798Mutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `Din16798Mutation` itself,
/// and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Din16798MutationDsl {
    ChangeAnnex { new_annex: AnnexChoice },
    ChangeOccupancy { new_occupancy: String },
    ChangeComfortCategory { new_comfort_category: String },
    ChangeTOpC { new_t_op_c: f64 },
    ChangeRhPercent { new_rh_percent: f64 },
    ChangeAirSpeedMS { new_air_speed_m_s: f64 },
    ChangeThetaRmC { new_theta_rm_c: f64 },
    ChangeCo2Ppm { new_co2_ppm: f64 },
    ChangeDfPercent { new_df_percent: f64 },
    ChangeLAeqDb { new_l_aeq_db: f64 },
    ChangePersons { new_persons: u32 },
    ChangeIdaClass { new_ida_class: String },
    ChangeVentilationM3H { new_ventilation_m3_h: f64 },
    ChangeFloorAreaM2 { new_floor_area_m2: f64 },
    ChangeBedrooms { new_bedrooms: u32 },
    ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: f64 },
    ChangeOccupants { new_occupants: u32 },
    ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: f64 },
    ChangeSfpWM3S { new_sfp_w_m3_s: f64 },
    ChangeSfpRequiredClass { new_sfp_required_class: u8 },
    ChangeHeatRecoveryEta { new_heat_recovery_eta: f64 },
    ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: f64 },
    ChangeSystemType { new_system_type: String },
    ChangeYearsSinceInspection { new_years_since_inspection: u32 },
    ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: f64 },
    ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: f64 },
    ChangeFanQVM3S { new_fan_q_v_m3_s: f64 },
    ChangeFanTRunH { new_fan_t_run_h: f64 },
    ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: f64 },
    ChangeNightSetbackK { new_night_setback_k: f64 },
    ChangeHrMDotKgS { new_hr_m_dot_kg_s: f64 },
    ChangeHrCpJKgk { new_hr_cp_j_kgk: f64 },
    ChangeHrDeltaTC { new_hr_delta_t_c: f64 },
    ChangeHrTH { new_hr_t_h: f64 },
    ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: f64 },
    ChangeN50HInv { new_n50_h_inv: f64 },
    ChangeVolumeM3 { new_volume_m3: f64 },
    ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: f64 },
    ChangeCellarAreaM2 { new_cellar_area_m2: f64 },
    ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: f64 },
    ChangeHTrWK { new_h_tr_w_k: f64 },
    ChangeHVeWK { new_h_ve_w_k: f64 },
    ChangeThetaEC { new_theta_e_c: f64 },
    ChangeThetaSetC { new_theta_set_c: f64 },
    ChangeCoolingDeltaTH { new_cooling_delta_t_h: f64 },
    ChangeCoolingGainsKwh { new_cooling_gains_kwh: f64 },
    ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: f64 },
    ChangeCoolingReferenceKwh { new_cooling_reference_kwh: f64 },
    ChangeChillerType { new_chiller_type: String },
    ChangeEerActual { new_eer_actual: f64 },
    ChangeQCKwh { new_q_c_kwh: f64 },
    ChangeGenerationReferenceKwh { new_generation_reference_kwh: f64 },
    ChangeDataCenterSupplyC { new_data_center_supply_c: f64 },
    ChangeHStWK { new_h_st_w_k: f64 },
    ChangeThetaStC { new_theta_st_c: f64 },
    ChangeThetaAmbC { new_theta_amb_c: f64 },
    ChangeStorageTH { new_storage_t_h: f64 },
    ChangeStorageAllowanceKwh { new_storage_allowance_kwh: f64 },
    ChangeDhwDeliveryC { new_dhw_delivery_c: f64 },
    ChangeDuctClass { new_duct_class: String },
    ChangeDuctTestPressurePa { new_duct_test_pressure_pa: f64 },
    ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: f64 },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for Din16798MutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Din16798MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn din16798_mutation_to_dsl(mutation: &Din16798Mutation) -> Din16798MutationDsl {
    match mutation {
        Din16798Mutation::ChangeAnnex(payload) => Din16798MutationDsl::ChangeAnnex { new_annex: payload.new_annex.clone() },
        Din16798Mutation::ChangeOccupancy(payload) => Din16798MutationDsl::ChangeOccupancy { new_occupancy: payload.new_occupancy.clone() },
        Din16798Mutation::ChangeComfortCategory(payload) => Din16798MutationDsl::ChangeComfortCategory { new_comfort_category: payload.new_comfort_category.clone() },
        Din16798Mutation::ChangeTOpC(payload) => Din16798MutationDsl::ChangeTOpC { new_t_op_c: payload.new_t_op_c.clone() },
        Din16798Mutation::ChangeRhPercent(payload) => Din16798MutationDsl::ChangeRhPercent { new_rh_percent: payload.new_rh_percent.clone() },
        Din16798Mutation::ChangeAirSpeedMS(payload) => Din16798MutationDsl::ChangeAirSpeedMS { new_air_speed_m_s: payload.new_air_speed_m_s.clone() },
        Din16798Mutation::ChangeThetaRmC(payload) => Din16798MutationDsl::ChangeThetaRmC { new_theta_rm_c: payload.new_theta_rm_c.clone() },
        Din16798Mutation::ChangeCo2Ppm(payload) => Din16798MutationDsl::ChangeCo2Ppm { new_co2_ppm: payload.new_co2_ppm.clone() },
        Din16798Mutation::ChangeDfPercent(payload) => Din16798MutationDsl::ChangeDfPercent { new_df_percent: payload.new_df_percent.clone() },
        Din16798Mutation::ChangeLAeqDb(payload) => Din16798MutationDsl::ChangeLAeqDb { new_l_aeq_db: payload.new_l_aeq_db.clone() },
        Din16798Mutation::ChangePersons(payload) => Din16798MutationDsl::ChangePersons { new_persons: payload.new_persons.clone() },
        Din16798Mutation::ChangeIdaClass(payload) => Din16798MutationDsl::ChangeIdaClass { new_ida_class: payload.new_ida_class.clone() },
        Din16798Mutation::ChangeVentilationM3H(payload) => Din16798MutationDsl::ChangeVentilationM3H { new_ventilation_m3_h: payload.new_ventilation_m3_h.clone() },
        Din16798Mutation::ChangeFloorAreaM2(payload) => Din16798MutationDsl::ChangeFloorAreaM2 { new_floor_area_m2: payload.new_floor_area_m2.clone() },
        Din16798Mutation::ChangeBedrooms(payload) => Din16798MutationDsl::ChangeBedrooms { new_bedrooms: payload.new_bedrooms.clone() },
        Din16798Mutation::ChangeDwellingVentilationM3H(payload) => Din16798MutationDsl::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: payload.new_dwelling_ventilation_m3_h.clone() },
        Din16798Mutation::ChangeOccupants(payload) => Din16798MutationDsl::ChangeOccupants { new_occupants: payload.new_occupants.clone() },
        Din16798Mutation::ChangeResidentialVentilationM3H(payload) => Din16798MutationDsl::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: payload.new_residential_ventilation_m3_h.clone() },
        Din16798Mutation::ChangeSfpWM3S(payload) => Din16798MutationDsl::ChangeSfpWM3S { new_sfp_w_m3_s: payload.new_sfp_w_m3_s.clone() },
        Din16798Mutation::ChangeSfpRequiredClass(payload) => Din16798MutationDsl::ChangeSfpRequiredClass { new_sfp_required_class: payload.new_sfp_required_class.clone() },
        Din16798Mutation::ChangeHeatRecoveryEta(payload) => Din16798MutationDsl::ChangeHeatRecoveryEta { new_heat_recovery_eta: payload.new_heat_recovery_eta.clone() },
        Din16798Mutation::ChangeHeatRecoveryEtaMin(payload) => Din16798MutationDsl::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: payload.new_heat_recovery_eta_min.clone() },
        Din16798Mutation::ChangeSystemType(payload) => Din16798MutationDsl::ChangeSystemType { new_system_type: payload.new_system_type.clone() },
        Din16798Mutation::ChangeYearsSinceInspection(payload) => Din16798MutationDsl::ChangeYearsSinceInspection { new_years_since_inspection: payload.new_years_since_inspection.clone() },
        Din16798Mutation::ChangeHumidificationRequiredKgH(payload) => Din16798MutationDsl::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: payload.new_humidification_required_kg_h.clone() },
        Din16798Mutation::ChangeHumidificationProvidedKgH(payload) => Din16798MutationDsl::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: payload.new_humidification_provided_kg_h.clone() },
        Din16798Mutation::ChangeFanQVM3S(payload) => Din16798MutationDsl::ChangeFanQVM3S { new_fan_q_v_m3_s: payload.new_fan_q_v_m3_s.clone() },
        Din16798Mutation::ChangeFanTRunH(payload) => Din16798MutationDsl::ChangeFanTRunH { new_fan_t_run_h: payload.new_fan_t_run_h.clone() },
        Din16798Mutation::ChangeFanEnergyReferenceKwh(payload) => Din16798MutationDsl::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: payload.new_fan_energy_reference_kwh.clone() },
        Din16798Mutation::ChangeNightSetbackK(payload) => Din16798MutationDsl::ChangeNightSetbackK { new_night_setback_k: payload.new_night_setback_k.clone() },
        Din16798Mutation::ChangeHrMDotKgS(payload) => Din16798MutationDsl::ChangeHrMDotKgS { new_hr_m_dot_kg_s: payload.new_hr_m_dot_kg_s.clone() },
        Din16798Mutation::ChangeHrCpJKgk(payload) => Din16798MutationDsl::ChangeHrCpJKgk { new_hr_cp_j_kgk: payload.new_hr_cp_j_kgk.clone() },
        Din16798Mutation::ChangeHrDeltaTC(payload) => Din16798MutationDsl::ChangeHrDeltaTC { new_hr_delta_t_c: payload.new_hr_delta_t_c.clone() },
        Din16798Mutation::ChangeHrTH(payload) => Din16798MutationDsl::ChangeHrTH { new_hr_t_h: payload.new_hr_t_h.clone() },
        Din16798Mutation::ChangeHrSavingsReferenceKwh(payload) => Din16798MutationDsl::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: payload.new_hr_savings_reference_kwh.clone() },
        Din16798Mutation::ChangeN50HInv(payload) => Din16798MutationDsl::ChangeN50HInv { new_n50_h_inv: payload.new_n50_h_inv.clone() },
        Din16798Mutation::ChangeVolumeM3(payload) => Din16798MutationDsl::ChangeVolumeM3 { new_volume_m3: payload.new_volume_m3.clone() },
        Din16798Mutation::ChangeInfiltrationAllowanceM3H(payload) => Din16798MutationDsl::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: payload.new_infiltration_allowance_m3_h.clone() },
        Din16798Mutation::ChangeCellarAreaM2(payload) => Din16798MutationDsl::ChangeCellarAreaM2 { new_cellar_area_m2: payload.new_cellar_area_m2.clone() },
        Din16798Mutation::ChangeCellarVentilationM3H(payload) => Din16798MutationDsl::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: payload.new_cellar_ventilation_m3_h.clone() },
        Din16798Mutation::ChangeHTrWK(payload) => Din16798MutationDsl::ChangeHTrWK { new_h_tr_w_k: payload.new_h_tr_w_k.clone() },
        Din16798Mutation::ChangeHVeWK(payload) => Din16798MutationDsl::ChangeHVeWK { new_h_ve_w_k: payload.new_h_ve_w_k.clone() },
        Din16798Mutation::ChangeThetaEC(payload) => Din16798MutationDsl::ChangeThetaEC { new_theta_e_c: payload.new_theta_e_c.clone() },
        Din16798Mutation::ChangeThetaSetC(payload) => Din16798MutationDsl::ChangeThetaSetC { new_theta_set_c: payload.new_theta_set_c.clone() },
        Din16798Mutation::ChangeCoolingDeltaTH(payload) => Din16798MutationDsl::ChangeCoolingDeltaTH { new_cooling_delta_t_h: payload.new_cooling_delta_t_h.clone() },
        Din16798Mutation::ChangeCoolingGainsKwh(payload) => Din16798MutationDsl::ChangeCoolingGainsKwh { new_cooling_gains_kwh: payload.new_cooling_gains_kwh.clone() },
        Din16798Mutation::ChangeCoolingUtilizationFactor(payload) => Din16798MutationDsl::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: payload.new_cooling_utilization_factor.clone() },
        Din16798Mutation::ChangeCoolingReferenceKwh(payload) => Din16798MutationDsl::ChangeCoolingReferenceKwh { new_cooling_reference_kwh: payload.new_cooling_reference_kwh.clone() },
        Din16798Mutation::ChangeChillerType(payload) => Din16798MutationDsl::ChangeChillerType { new_chiller_type: payload.new_chiller_type.clone() },
        Din16798Mutation::ChangeEerActual(payload) => Din16798MutationDsl::ChangeEerActual { new_eer_actual: payload.new_eer_actual.clone() },
        Din16798Mutation::ChangeQCKwh(payload) => Din16798MutationDsl::ChangeQCKwh { new_q_c_kwh: payload.new_q_c_kwh.clone() },
        Din16798Mutation::ChangeGenerationReferenceKwh(payload) => Din16798MutationDsl::ChangeGenerationReferenceKwh { new_generation_reference_kwh: payload.new_generation_reference_kwh.clone() },
        Din16798Mutation::ChangeDataCenterSupplyC(payload) => Din16798MutationDsl::ChangeDataCenterSupplyC { new_data_center_supply_c: payload.new_data_center_supply_c.clone() },
        Din16798Mutation::ChangeHStWK(payload) => Din16798MutationDsl::ChangeHStWK { new_h_st_w_k: payload.new_h_st_w_k.clone() },
        Din16798Mutation::ChangeThetaStC(payload) => Din16798MutationDsl::ChangeThetaStC { new_theta_st_c: payload.new_theta_st_c.clone() },
        Din16798Mutation::ChangeThetaAmbC(payload) => Din16798MutationDsl::ChangeThetaAmbC { new_theta_amb_c: payload.new_theta_amb_c.clone() },
        Din16798Mutation::ChangeStorageTH(payload) => Din16798MutationDsl::ChangeStorageTH { new_storage_t_h: payload.new_storage_t_h.clone() },
        Din16798Mutation::ChangeStorageAllowanceKwh(payload) => Din16798MutationDsl::ChangeStorageAllowanceKwh { new_storage_allowance_kwh: payload.new_storage_allowance_kwh.clone() },
        Din16798Mutation::ChangeDhwDeliveryC(payload) => Din16798MutationDsl::ChangeDhwDeliveryC { new_dhw_delivery_c: payload.new_dhw_delivery_c.clone() },
        Din16798Mutation::ChangeDuctClass(payload) => Din16798MutationDsl::ChangeDuctClass { new_duct_class: payload.new_duct_class.clone() },
        Din16798Mutation::ChangeDuctTestPressurePa(payload) => Din16798MutationDsl::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: payload.new_duct_test_pressure_pa.clone() },
        Din16798Mutation::ChangeDuctLeakageM3SM2(payload) => Din16798MutationDsl::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: payload.new_duct_leakage_m3_s_m2.clone() },
    }
}

fn din16798_mutation_from_dsl(mutation: Din16798MutationDsl) -> Din16798Mutation {
    match mutation {
        Din16798MutationDsl::ChangeAnnex { new_annex } => Din16798Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex }),
        Din16798MutationDsl::ChangeOccupancy { new_occupancy } => Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy }),
        Din16798MutationDsl::ChangeComfortCategory { new_comfort_category } => Din16798Mutation::ChangeComfortCategory(change_comfort_category::mutation::ChangeComfortCategory { new_comfort_category }),
        Din16798MutationDsl::ChangeTOpC { new_t_op_c } => Din16798Mutation::ChangeTOpC(change_t_op_c::mutation::ChangeTOpC { new_t_op_c }),
        Din16798MutationDsl::ChangeRhPercent { new_rh_percent } => Din16798Mutation::ChangeRhPercent(change_rh_percent::mutation::ChangeRhPercent { new_rh_percent }),
        Din16798MutationDsl::ChangeAirSpeedMS { new_air_speed_m_s } => Din16798Mutation::ChangeAirSpeedMS(change_air_speed_m_s::mutation::ChangeAirSpeedMS { new_air_speed_m_s }),
        Din16798MutationDsl::ChangeThetaRmC { new_theta_rm_c } => Din16798Mutation::ChangeThetaRmC(change_theta_rm_c::mutation::ChangeThetaRmC { new_theta_rm_c }),
        Din16798MutationDsl::ChangeCo2Ppm { new_co2_ppm } => Din16798Mutation::ChangeCo2Ppm(change_co2_ppm::mutation::ChangeCo2Ppm { new_co2_ppm }),
        Din16798MutationDsl::ChangeDfPercent { new_df_percent } => Din16798Mutation::ChangeDfPercent(change_df_percent::mutation::ChangeDfPercent { new_df_percent }),
        Din16798MutationDsl::ChangeLAeqDb { new_l_aeq_db } => Din16798Mutation::ChangeLAeqDb(change_l_aeq_db::mutation::ChangeLAeqDb { new_l_aeq_db }),
        Din16798MutationDsl::ChangePersons { new_persons } => Din16798Mutation::ChangePersons(change_persons::mutation::ChangePersons { new_persons }),
        Din16798MutationDsl::ChangeIdaClass { new_ida_class } => Din16798Mutation::ChangeIdaClass(change_ida_class::mutation::ChangeIdaClass { new_ida_class }),
        Din16798MutationDsl::ChangeVentilationM3H { new_ventilation_m3_h } => Din16798Mutation::ChangeVentilationM3H(change_ventilation_m3_h::mutation::ChangeVentilationM3H { new_ventilation_m3_h }),
        Din16798MutationDsl::ChangeFloorAreaM2 { new_floor_area_m2 } => Din16798Mutation::ChangeFloorAreaM2(change_floor_area_m2::mutation::ChangeFloorAreaM2 { new_floor_area_m2 }),
        Din16798MutationDsl::ChangeBedrooms { new_bedrooms } => Din16798Mutation::ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms { new_bedrooms }),
        Din16798MutationDsl::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h } => Din16798Mutation::ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h }),
        Din16798MutationDsl::ChangeOccupants { new_occupants } => Din16798Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants }),
        Din16798MutationDsl::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h } => {
            Din16798Mutation::ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h })
        }
        Din16798MutationDsl::ChangeSfpWM3S { new_sfp_w_m3_s } => Din16798Mutation::ChangeSfpWM3S(change_sfp_w_m3_s::mutation::ChangeSfpWM3S { new_sfp_w_m3_s }),
        Din16798MutationDsl::ChangeSfpRequiredClass { new_sfp_required_class } => Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::mutation::ChangeSfpRequiredClass { new_sfp_required_class }),
        Din16798MutationDsl::ChangeHeatRecoveryEta { new_heat_recovery_eta } => Din16798Mutation::ChangeHeatRecoveryEta(change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta { new_heat_recovery_eta }),
        Din16798MutationDsl::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min } => Din16798Mutation::ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min }),
        Din16798MutationDsl::ChangeSystemType { new_system_type } => Din16798Mutation::ChangeSystemType(change_system_type::mutation::ChangeSystemType { new_system_type }),
        Din16798MutationDsl::ChangeYearsSinceInspection { new_years_since_inspection } => Din16798Mutation::ChangeYearsSinceInspection(change_years_since_inspection::mutation::ChangeYearsSinceInspection { new_years_since_inspection }),
        Din16798MutationDsl::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h } => {
            Din16798Mutation::ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h })
        }
        Din16798MutationDsl::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h } => {
            Din16798Mutation::ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h })
        }
        Din16798MutationDsl::ChangeFanQVM3S { new_fan_q_v_m3_s } => Din16798Mutation::ChangeFanQVM3S(change_fan_q_v_m3_s::mutation::ChangeFanQVM3S { new_fan_q_v_m3_s }),
        Din16798MutationDsl::ChangeFanTRunH { new_fan_t_run_h } => Din16798Mutation::ChangeFanTRunH(change_fan_t_run_h::mutation::ChangeFanTRunH { new_fan_t_run_h }),
        Din16798MutationDsl::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh } => Din16798Mutation::ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh }),
        Din16798MutationDsl::ChangeNightSetbackK { new_night_setback_k } => Din16798Mutation::ChangeNightSetbackK(change_night_setback_k::mutation::ChangeNightSetbackK { new_night_setback_k }),
        Din16798MutationDsl::ChangeHrMDotKgS { new_hr_m_dot_kg_s } => Din16798Mutation::ChangeHrMDotKgS(change_hr_m_dot_kg_s::mutation::ChangeHrMDotKgS { new_hr_m_dot_kg_s }),
        Din16798MutationDsl::ChangeHrCpJKgk { new_hr_cp_j_kgk } => Din16798Mutation::ChangeHrCpJKgk(change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk { new_hr_cp_j_kgk }),
        Din16798MutationDsl::ChangeHrDeltaTC { new_hr_delta_t_c } => Din16798Mutation::ChangeHrDeltaTC(change_hr_delta_t_c::mutation::ChangeHrDeltaTC { new_hr_delta_t_c }),
        Din16798MutationDsl::ChangeHrTH { new_hr_t_h } => Din16798Mutation::ChangeHrTH(change_hr_t_h::mutation::ChangeHrTH { new_hr_t_h }),
        Din16798MutationDsl::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh } => Din16798Mutation::ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh }),
        Din16798MutationDsl::ChangeN50HInv { new_n50_h_inv } => Din16798Mutation::ChangeN50HInv(change_n50_h_inv::mutation::ChangeN50HInv { new_n50_h_inv }),
        Din16798MutationDsl::ChangeVolumeM3 { new_volume_m3 } => Din16798Mutation::ChangeVolumeM3(change_volume_m3::mutation::ChangeVolumeM3 { new_volume_m3 }),
        Din16798MutationDsl::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h } => {
            Din16798Mutation::ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h })
        }
        Din16798MutationDsl::ChangeCellarAreaM2 { new_cellar_area_m2 } => Din16798Mutation::ChangeCellarAreaM2(change_cellar_area_m2::mutation::ChangeCellarAreaM2 { new_cellar_area_m2 }),
        Din16798MutationDsl::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h } => Din16798Mutation::ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::mutation::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h }),
        Din16798MutationDsl::ChangeHTrWK { new_h_tr_w_k } => Din16798Mutation::ChangeHTrWK(change_h_tr_w_k::mutation::ChangeHTrWK { new_h_tr_w_k }),
        Din16798MutationDsl::ChangeHVeWK { new_h_ve_w_k } => Din16798Mutation::ChangeHVeWK(change_h_ve_w_k::mutation::ChangeHVeWK { new_h_ve_w_k }),
        Din16798MutationDsl::ChangeThetaEC { new_theta_e_c } => Din16798Mutation::ChangeThetaEC(change_theta_e_c::mutation::ChangeThetaEC { new_theta_e_c }),
        Din16798MutationDsl::ChangeThetaSetC { new_theta_set_c } => Din16798Mutation::ChangeThetaSetC(change_theta_set_c::mutation::ChangeThetaSetC { new_theta_set_c }),
        Din16798MutationDsl::ChangeCoolingDeltaTH { new_cooling_delta_t_h } => Din16798Mutation::ChangeCoolingDeltaTH(change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH { new_cooling_delta_t_h }),
        Din16798MutationDsl::ChangeCoolingGainsKwh { new_cooling_gains_kwh } => Din16798Mutation::ChangeCoolingGainsKwh(change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh { new_cooling_gains_kwh }),
        Din16798MutationDsl::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor } => {
            Din16798Mutation::ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor })
        }
        Din16798MutationDsl::ChangeCoolingReferenceKwh { new_cooling_reference_kwh } => Din16798Mutation::ChangeCoolingReferenceKwh(change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh { new_cooling_reference_kwh }),
        Din16798MutationDsl::ChangeChillerType { new_chiller_type } => Din16798Mutation::ChangeChillerType(change_chiller_type::mutation::ChangeChillerType { new_chiller_type }),
        Din16798MutationDsl::ChangeEerActual { new_eer_actual } => Din16798Mutation::ChangeEerActual(change_eer_actual::mutation::ChangeEerActual { new_eer_actual }),
        Din16798MutationDsl::ChangeQCKwh { new_q_c_kwh } => Din16798Mutation::ChangeQCKwh(change_q_c_kwh::mutation::ChangeQCKwh { new_q_c_kwh }),
        Din16798MutationDsl::ChangeGenerationReferenceKwh { new_generation_reference_kwh } => Din16798Mutation::ChangeGenerationReferenceKwh(change_generation_reference_kwh::mutation::ChangeGenerationReferenceKwh { new_generation_reference_kwh }),
        Din16798MutationDsl::ChangeDataCenterSupplyC { new_data_center_supply_c } => Din16798Mutation::ChangeDataCenterSupplyC(change_data_center_supply_c::mutation::ChangeDataCenterSupplyC { new_data_center_supply_c }),
        Din16798MutationDsl::ChangeHStWK { new_h_st_w_k } => Din16798Mutation::ChangeHStWK(change_h_st_w_k::mutation::ChangeHStWK { new_h_st_w_k }),
        Din16798MutationDsl::ChangeThetaStC { new_theta_st_c } => Din16798Mutation::ChangeThetaStC(change_theta_st_c::mutation::ChangeThetaStC { new_theta_st_c }),
        Din16798MutationDsl::ChangeThetaAmbC { new_theta_amb_c } => Din16798Mutation::ChangeThetaAmbC(change_theta_amb_c::mutation::ChangeThetaAmbC { new_theta_amb_c }),
        Din16798MutationDsl::ChangeStorageTH { new_storage_t_h } => Din16798Mutation::ChangeStorageTH(change_storage_t_h::mutation::ChangeStorageTH { new_storage_t_h }),
        Din16798MutationDsl::ChangeStorageAllowanceKwh { new_storage_allowance_kwh } => Din16798Mutation::ChangeStorageAllowanceKwh(change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh { new_storage_allowance_kwh }),
        Din16798MutationDsl::ChangeDhwDeliveryC { new_dhw_delivery_c } => Din16798Mutation::ChangeDhwDeliveryC(change_dhw_delivery_c::mutation::ChangeDhwDeliveryC { new_dhw_delivery_c }),
        Din16798MutationDsl::ChangeDuctClass { new_duct_class } => Din16798Mutation::ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass { new_duct_class }),
        Din16798MutationDsl::ChangeDuctTestPressurePa { new_duct_test_pressure_pa } => Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa { new_duct_test_pressure_pa }),
        Din16798MutationDsl::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2 } => Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::mutation::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2 }),
    }
}

impl OpText for Din16798Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(din16798_mutation_from_dsl(<Din16798MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <Din16798MutationDsl as OpText>::print_op(&din16798_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Din16798MutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for Din16798Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        din16798_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(din16798_mutation_from_dsl(Din16798MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn op_text_round_trips_change_annex() {
        store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: AnnexChoice::En }));
    }

    #[test]
    fn op_text_round_trips_change_t_op_c() {
        store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeTOpC(change_t_op_c::mutation::ChangeTOpC { new_t_op_c: 21.5 }));
    }

    #[test]
    fn op_text_round_trips_change_occupancy() {
        store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: "office".into() }));
    }

    #[test]
    fn op_text_round_trips_change_persons() {
        store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangePersons(change_persons::mutation::ChangePersons { new_persons: 6 }));
    }

    #[test]
    fn op_text_round_trips_change_sfp_required_class() {
        store::os_store::test_support::assert_op_line_round_trip(&Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::mutation::ChangeSfpRequiredClass { new_sfp_required_class: 2 }));
    }

    /// ⚖️ Every variant, not just the five hand-picked above — full-coverage `OpText` round trip
    /// over the closed vocabulary, one sample value per field.
    #[test]
    fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    fn every_mutation() -> Vec<Din16798Mutation> {
        vec![
            Din16798Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: AnnexChoice::En }),
            Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: "office".to_string() }),
            Din16798Mutation::ChangeComfortCategory(change_comfort_category::mutation::ChangeComfortCategory { new_comfort_category: "I".to_string() }),
            Din16798Mutation::ChangeTOpC(change_t_op_c::mutation::ChangeTOpC { new_t_op_c: 24.5 }),
            Din16798Mutation::ChangeRhPercent(change_rh_percent::mutation::ChangeRhPercent { new_rh_percent: 45.0 }),
            Din16798Mutation::ChangeAirSpeedMS(change_air_speed_m_s::mutation::ChangeAirSpeedMS { new_air_speed_m_s: 0.15 }),
            Din16798Mutation::ChangeThetaRmC(change_theta_rm_c::mutation::ChangeThetaRmC { new_theta_rm_c: 18.0 }),
            Din16798Mutation::ChangeCo2Ppm(change_co2_ppm::mutation::ChangeCo2Ppm { new_co2_ppm: 900.0 }),
            Din16798Mutation::ChangeDfPercent(change_df_percent::mutation::ChangeDfPercent { new_df_percent: 3.0 }),
            Din16798Mutation::ChangeLAeqDb(change_l_aeq_db::mutation::ChangeLAeqDb { new_l_aeq_db: 28.0 }),
            Din16798Mutation::ChangePersons(change_persons::mutation::ChangePersons { new_persons: 12 }),
            Din16798Mutation::ChangeIdaClass(change_ida_class::mutation::ChangeIdaClass { new_ida_class: "1".to_string() }),
            Din16798Mutation::ChangeVentilationM3H(change_ventilation_m3_h::mutation::ChangeVentilationM3H { new_ventilation_m3_h: 320.0 }),
            Din16798Mutation::ChangeFloorAreaM2(change_floor_area_m2::mutation::ChangeFloorAreaM2 { new_floor_area_m2: 110.0 }),
            Din16798Mutation::ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms { new_bedrooms: 4 }),
            Din16798Mutation::ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: 70.0 }),
            Din16798Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: 4 }),
            Din16798Mutation::ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: 90.0 }),
            Din16798Mutation::ChangeSfpWM3S(change_sfp_w_m3_s::mutation::ChangeSfpWM3S { new_sfp_w_m3_s: 1600.0 }),
            Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::mutation::ChangeSfpRequiredClass { new_sfp_required_class: 3 }),
            Din16798Mutation::ChangeHeatRecoveryEta(change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta { new_heat_recovery_eta: 0.8 }),
            Din16798Mutation::ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: 0.72 }),
            Din16798Mutation::ChangeSystemType(change_system_type::mutation::ChangeSystemType { new_system_type: "decentral_mech".to_string() }),
            Din16798Mutation::ChangeYearsSinceInspection(change_years_since_inspection::mutation::ChangeYearsSinceInspection { new_years_since_inspection: 2 }),
            Din16798Mutation::ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: 2.5 }),
            Din16798Mutation::ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: 2.5 }),
            Din16798Mutation::ChangeFanQVM3S(change_fan_q_v_m3_s::mutation::ChangeFanQVM3S { new_fan_q_v_m3_s: 1.2 }),
            Din16798Mutation::ChangeFanTRunH(change_fan_t_run_h::mutation::ChangeFanTRunH { new_fan_t_run_h: 10.0 }),
            Din16798Mutation::ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: 18.0 }),
            Din16798Mutation::ChangeNightSetbackK(change_night_setback_k::mutation::ChangeNightSetbackK { new_night_setback_k: 4.0 }),
            Din16798Mutation::ChangeHrMDotKgS(change_hr_m_dot_kg_s::mutation::ChangeHrMDotKgS { new_hr_m_dot_kg_s: 0.6 }),
            Din16798Mutation::ChangeHrCpJKgk(change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk { new_hr_cp_j_kgk: 1006.0 }),
            Din16798Mutation::ChangeHrDeltaTC(change_hr_delta_t_c::mutation::ChangeHrDeltaTC { new_hr_delta_t_c: 16.0 }),
            Din16798Mutation::ChangeHrTH(change_hr_t_h::mutation::ChangeHrTH { new_hr_t_h: 12.0 }),
            Din16798Mutation::ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: 55.0 }),
            Din16798Mutation::ChangeN50HInv(change_n50_h_inv::mutation::ChangeN50HInv { new_n50_h_inv: 1.2 }),
            Din16798Mutation::ChangeVolumeM3(change_volume_m3::mutation::ChangeVolumeM3 { new_volume_m3: 540.0 }),
            Din16798Mutation::ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: 50.0 }),
            Din16798Mutation::ChangeCellarAreaM2(change_cellar_area_m2::mutation::ChangeCellarAreaM2 { new_cellar_area_m2: 55.0 }),
            Din16798Mutation::ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::mutation::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: 18.0 }),
            Din16798Mutation::ChangeHTrWK(change_h_tr_w_k::mutation::ChangeHTrWK { new_h_tr_w_k: 220.0 }),
            Din16798Mutation::ChangeHVeWK(change_h_ve_w_k::mutation::ChangeHVeWK { new_h_ve_w_k: 110.0 }),
            Din16798Mutation::ChangeThetaEC(change_theta_e_c::mutation::ChangeThetaEC { new_theta_e_c: 33.0 }),
            Din16798Mutation::ChangeThetaSetC(change_theta_set_c::mutation::ChangeThetaSetC { new_theta_set_c: 25.0 }),
            Din16798Mutation::ChangeCoolingDeltaTH(change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH { new_cooling_delta_t_h: 12.0 }),
            Din16798Mutation::ChangeCoolingGainsKwh(change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh { new_cooling_gains_kwh: 6.0 }),
            Din16798Mutation::ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: 0.85 }),
            Din16798Mutation::ChangeCoolingReferenceKwh(change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh { new_cooling_reference_kwh: 24.0 }),
            Din16798Mutation::ChangeChillerType(change_chiller_type::mutation::ChangeChillerType { new_chiller_type: "water_cooled".to_string() }),
            Din16798Mutation::ChangeEerActual(change_eer_actual::mutation::ChangeEerActual { new_eer_actual: 3.4 }),
            Din16798Mutation::ChangeQCKwh(change_q_c_kwh::mutation::ChangeQCKwh { new_q_c_kwh: 1200.0 }),
            Din16798Mutation::ChangeGenerationReferenceKwh(change_generation_reference_kwh::mutation::ChangeGenerationReferenceKwh { new_generation_reference_kwh: 420.0 }),
            Din16798Mutation::ChangeDataCenterSupplyC(change_data_center_supply_c::mutation::ChangeDataCenterSupplyC { new_data_center_supply_c: 24.0 }),
            Din16798Mutation::ChangeHStWK(change_h_st_w_k::mutation::ChangeHStWK { new_h_st_w_k: 6.0 }),
            Din16798Mutation::ChangeThetaStC(change_theta_st_c::mutation::ChangeThetaStC { new_theta_st_c: 62.0 }),
            Din16798Mutation::ChangeThetaAmbC(change_theta_amb_c::mutation::ChangeThetaAmbC { new_theta_amb_c: 21.0 }),
            Din16798Mutation::ChangeStorageTH(change_storage_t_h::mutation::ChangeStorageTH { new_storage_t_h: 20.0 }),
            Din16798Mutation::ChangeStorageAllowanceKwh(change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh { new_storage_allowance_kwh: 7.0 }),
            Din16798Mutation::ChangeDhwDeliveryC(change_dhw_delivery_c::mutation::ChangeDhwDeliveryC { new_dhw_delivery_c: 60.0 }),
            Din16798Mutation::ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass { new_duct_class: "B".to_string() }),
            Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: 450.0 }),
            Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::mutation::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: 0.08 }),
        ]
    }
}
//#endregion 🧪️Tests
