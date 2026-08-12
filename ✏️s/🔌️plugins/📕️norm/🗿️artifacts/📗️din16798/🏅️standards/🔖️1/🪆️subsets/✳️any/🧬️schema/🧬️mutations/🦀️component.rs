//! 🧬️ Din16798 artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Din16798Snapshot`'s shape per `📓️derivation-rules.md` rule 1: a flat, id-less,
//! document-root parameter form (sixty-two persistent scalar fields describing occupancy,
//! ventilation, comfort, heat-recovery, infiltration, cooling, storage and duct-leakage inputs to
//! a DIN EN 16798-1 compliance check) — no id-keyed collections, no name/identity field to
//! `rename`. Every field becomes its own `change-<field>` mutation per the rule's "change-<field>
//! per remaining scalar" clause; none qualify for the `update-<facet>` grouping exception (each
//! parameter is independently measured/entered, never validated as an atomic multi-field bundle).
//! The pre-migration whole-document-replace variant is gone: banned outright per
//! `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6, with NO replacement mutation; file-open/import/
//! load-example now goes through `store::ArtifactStore::reset`, entirely outside this enum.
//!
//! All sixty-two triads (including the renamed former `set-snapshot` slot, now `change-annex`)
//! are mounted directly as `mutations`-sibling modules in `📦️glue.rs`, each with its own unique
//! emoji-prefixed directory (this lane's agent owns `📦️glue.rs` and the emoji-uniqueness policy
//! rule, so the wave-2 precedent's self-wiring `#[path = "."]` blocks and reused `🔧` emoji across
//! all 61 dirs are both retired here in favour of real glue mounts + distinct emoji).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Leaves
use super::change_annex;
use super::change_occupancy;
use super::change_comfort_category;
use super::change_t_op_c;
use super::change_rh_percent;
use super::change_air_speed_m_s;
use super::change_theta_rm_c;
use super::change_co2_ppm;
use super::change_df_percent;
use super::change_l_aeq_db;
use super::change_persons;
use super::change_ida_class;
use super::change_ventilation_m3_h;
use super::change_floor_area_m2;
use super::change_bedrooms;
use super::change_dwelling_ventilation_m3_h;
use super::change_occupants;
use super::change_residential_ventilation_m3_h;
use super::change_sfp_w_m3_s;
use super::change_sfp_required_class;
use super::change_heat_recovery_eta;
use super::change_heat_recovery_eta_min;
use super::change_system_type;
use super::change_years_since_inspection;
use super::change_humidification_required_kg_h;
use super::change_humidification_provided_kg_h;
use super::change_fan_q_v_m3_s;
use super::change_fan_t_run_h;
use super::change_fan_energy_reference_kwh;
use super::change_night_setback_k;
use super::change_hr_m_dot_kg_s;
use super::change_hr_cp_j_kgk;
use super::change_hr_delta_t_c;
use super::change_hr_t_h;
use super::change_hr_savings_reference_kwh;
use super::change_n50_h_inv;
use super::change_volume_m3;
use super::change_infiltration_allowance_m3_h;
use super::change_cellar_area_m2;
use super::change_cellar_ventilation_m3_h;
use super::change_h_tr_w_k;
use super::change_h_ve_w_k;
use super::change_theta_e_c;
use super::change_theta_set_c;
use super::change_cooling_delta_t_h;
use super::change_cooling_gains_kwh;
use super::change_cooling_utilization_factor;
use super::change_cooling_reference_kwh;
use super::change_chiller_type;
use super::change_eer_actual;
use super::change_q_c_kwh;
use super::change_generation_reference_kwh;
use super::change_data_center_supply_c;
use super::change_h_st_w_k;
use super::change_theta_st_c;
use super::change_theta_amb_c;
use super::change_storage_t_h;
use super::change_storage_allowance_kwh;
use super::change_dhw_delivery_c;
use super::change_duct_class;
use super::change_duct_test_pressure_pa;
use super::change_duct_leakage_m3_s_m2;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the din16798 document, derived per
/// `📓️derivation-rules.md` from `Din16798Snapshot`'s flat scalar shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Din16798Snapshot, diff = Din16798Diff, schema = "norm.din16798")]
pub enum Din16798Mutation {
    ChangeAnnex(change_annex::mutation::ChangeAnnex),
    ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy),
    ChangeComfortCategory(change_comfort_category::mutation::ChangeComfortCategory),
    ChangeTOpC(change_t_op_c::mutation::ChangeTOpC),
    ChangeRhPercent(change_rh_percent::mutation::ChangeRhPercent),
    ChangeAirSpeedMS(change_air_speed_m_s::mutation::ChangeAirSpeedMS),
    ChangeThetaRmC(change_theta_rm_c::mutation::ChangeThetaRmC),
    ChangeCo2Ppm(change_co2_ppm::mutation::ChangeCo2Ppm),
    ChangeDfPercent(change_df_percent::mutation::ChangeDfPercent),
    ChangeLAeqDb(change_l_aeq_db::mutation::ChangeLAeqDb),
    ChangePersons(change_persons::mutation::ChangePersons),
    ChangeIdaClass(change_ida_class::mutation::ChangeIdaClass),
    ChangeVentilationM3H(change_ventilation_m3_h::mutation::ChangeVentilationM3H),
    ChangeFloorAreaM2(change_floor_area_m2::mutation::ChangeFloorAreaM2),
    ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms),
    ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H),
    ChangeOccupants(change_occupants::mutation::ChangeOccupants),
    ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H),
    ChangeSfpWM3S(change_sfp_w_m3_s::mutation::ChangeSfpWM3S),
    ChangeSfpRequiredClass(change_sfp_required_class::mutation::ChangeSfpRequiredClass),
    ChangeHeatRecoveryEta(change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta),
    ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin),
    ChangeSystemType(change_system_type::mutation::ChangeSystemType),
    ChangeYearsSinceInspection(change_years_since_inspection::mutation::ChangeYearsSinceInspection),
    ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH),
    ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH),
    ChangeFanQVM3S(change_fan_q_v_m3_s::mutation::ChangeFanQVM3S),
    ChangeFanTRunH(change_fan_t_run_h::mutation::ChangeFanTRunH),
    ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh),
    ChangeNightSetbackK(change_night_setback_k::mutation::ChangeNightSetbackK),
    ChangeHrMDotKgS(change_hr_m_dot_kg_s::mutation::ChangeHrMDotKgS),
    ChangeHrCpJKgk(change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk),
    ChangeHrDeltaTC(change_hr_delta_t_c::mutation::ChangeHrDeltaTC),
    ChangeHrTH(change_hr_t_h::mutation::ChangeHrTH),
    ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh),
    ChangeN50HInv(change_n50_h_inv::mutation::ChangeN50HInv),
    ChangeVolumeM3(change_volume_m3::mutation::ChangeVolumeM3),
    ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H),
    ChangeCellarAreaM2(change_cellar_area_m2::mutation::ChangeCellarAreaM2),
    ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::mutation::ChangeCellarVentilationM3H),
    ChangeHTrWK(change_h_tr_w_k::mutation::ChangeHTrWK),
    ChangeHVeWK(change_h_ve_w_k::mutation::ChangeHVeWK),
    ChangeThetaEC(change_theta_e_c::mutation::ChangeThetaEC),
    ChangeThetaSetC(change_theta_set_c::mutation::ChangeThetaSetC),
    ChangeCoolingDeltaTH(change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH),
    ChangeCoolingGainsKwh(change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh),
    ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor),
    ChangeCoolingReferenceKwh(change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh),
    ChangeChillerType(change_chiller_type::mutation::ChangeChillerType),
    ChangeEerActual(change_eer_actual::mutation::ChangeEerActual),
    ChangeQCKwh(change_q_c_kwh::mutation::ChangeQCKwh),
    ChangeGenerationReferenceKwh(change_generation_reference_kwh::mutation::ChangeGenerationReferenceKwh),
    ChangeDataCenterSupplyC(change_data_center_supply_c::mutation::ChangeDataCenterSupplyC),
    ChangeHStWK(change_h_st_w_k::mutation::ChangeHStWK),
    ChangeThetaStC(change_theta_st_c::mutation::ChangeThetaStC),
    ChangeThetaAmbC(change_theta_amb_c::mutation::ChangeThetaAmbC),
    ChangeStorageTH(change_storage_t_h::mutation::ChangeStorageTH),
    ChangeStorageAllowanceKwh(change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh),
    ChangeDhwDeliveryC(change_dhw_delivery_c::mutation::ChangeDhwDeliveryC),
    ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass),
    ChangeDuctTestPressurePa(change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa),
    ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::mutation::ChangeDuctLeakageM3SM2),
}
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl Din16798Mutation {
    /// 📤️ Decomposes a whole `Din16798Snapshot` into one `change-<field>` mutation per persistent
    /// field — the closed-vocabulary replacement for the banned whole-document-replace variant, used
    /// by `import_media`'s `"model:in"` port and the `set-snapshot` app command to bundle a bulk
    /// document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        let mut mutations = Vec::with_capacity(62);
        mutations.push(Din16798Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: snapshot.annex.clone() }));
        mutations.push(Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: snapshot.occupancy.clone() }));
        mutations.push(Din16798Mutation::ChangeComfortCategory(change_comfort_category::mutation::ChangeComfortCategory { new_comfort_category: snapshot.comfort_category.clone() }));
        mutations.push(Din16798Mutation::ChangeTOpC(change_t_op_c::mutation::ChangeTOpC { new_t_op_c: snapshot.t_op_c.clone() }));
        mutations.push(Din16798Mutation::ChangeRhPercent(change_rh_percent::mutation::ChangeRhPercent { new_rh_percent: snapshot.rh_percent.clone() }));
        mutations.push(Din16798Mutation::ChangeAirSpeedMS(change_air_speed_m_s::mutation::ChangeAirSpeedMS { new_air_speed_m_s: snapshot.air_speed_m_s.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaRmC(change_theta_rm_c::mutation::ChangeThetaRmC { new_theta_rm_c: snapshot.theta_rm_c.clone() }));
        mutations.push(Din16798Mutation::ChangeCo2Ppm(change_co2_ppm::mutation::ChangeCo2Ppm { new_co2_ppm: snapshot.co2_ppm.clone() }));
        mutations.push(Din16798Mutation::ChangeDfPercent(change_df_percent::mutation::ChangeDfPercent { new_df_percent: snapshot.df_percent.clone() }));
        mutations.push(Din16798Mutation::ChangeLAeqDb(change_l_aeq_db::mutation::ChangeLAeqDb { new_l_aeq_db: snapshot.l_aeq_db.clone() }));
        mutations.push(Din16798Mutation::ChangePersons(change_persons::mutation::ChangePersons { new_persons: snapshot.persons.clone() }));
        mutations.push(Din16798Mutation::ChangeIdaClass(change_ida_class::mutation::ChangeIdaClass { new_ida_class: snapshot.ida_class.clone() }));
        mutations.push(Din16798Mutation::ChangeVentilationM3H(change_ventilation_m3_h::mutation::ChangeVentilationM3H { new_ventilation_m3_h: snapshot.ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeFloorAreaM2(change_floor_area_m2::mutation::ChangeFloorAreaM2 { new_floor_area_m2: snapshot.floor_area_m2.clone() }));
        mutations.push(Din16798Mutation::ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms { new_bedrooms: snapshot.bedrooms.clone() }));
        mutations.push(Din16798Mutation::ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::mutation::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: snapshot.dwelling_ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: snapshot.occupants.clone() }));
        mutations.push(Din16798Mutation::ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: snapshot.residential_ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeSfpWM3S(change_sfp_w_m3_s::mutation::ChangeSfpWM3S { new_sfp_w_m3_s: snapshot.sfp_w_m3_s.clone() }));
        mutations.push(Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::mutation::ChangeSfpRequiredClass { new_sfp_required_class: snapshot.sfp_required_class.clone() }));
        mutations.push(Din16798Mutation::ChangeHeatRecoveryEta(change_heat_recovery_eta::mutation::ChangeHeatRecoveryEta { new_heat_recovery_eta: snapshot.heat_recovery_eta.clone() }));
        mutations.push(Din16798Mutation::ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::mutation::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: snapshot.heat_recovery_eta_min.clone() }));
        mutations.push(Din16798Mutation::ChangeSystemType(change_system_type::mutation::ChangeSystemType { new_system_type: snapshot.system_type.clone() }));
        mutations.push(Din16798Mutation::ChangeYearsSinceInspection(change_years_since_inspection::mutation::ChangeYearsSinceInspection { new_years_since_inspection: snapshot.years_since_inspection.clone() }));
        mutations.push(Din16798Mutation::ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::mutation::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: snapshot.humidification_required_kg_h.clone() }));
        mutations.push(Din16798Mutation::ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: snapshot.humidification_provided_kg_h.clone() }));
        mutations.push(Din16798Mutation::ChangeFanQVM3S(change_fan_q_v_m3_s::mutation::ChangeFanQVM3S { new_fan_q_v_m3_s: snapshot.fan_q_v_m3_s.clone() }));
        mutations.push(Din16798Mutation::ChangeFanTRunH(change_fan_t_run_h::mutation::ChangeFanTRunH { new_fan_t_run_h: snapshot.fan_t_run_h.clone() }));
        mutations.push(Din16798Mutation::ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::mutation::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: snapshot.fan_energy_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeNightSetbackK(change_night_setback_k::mutation::ChangeNightSetbackK { new_night_setback_k: snapshot.night_setback_k.clone() }));
        mutations.push(Din16798Mutation::ChangeHrMDotKgS(change_hr_m_dot_kg_s::mutation::ChangeHrMDotKgS { new_hr_m_dot_kg_s: snapshot.hr_m_dot_kg_s.clone() }));
        mutations.push(Din16798Mutation::ChangeHrCpJKgk(change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk { new_hr_cp_j_kgk: snapshot.hr_cp_j_kgk.clone() }));
        mutations.push(Din16798Mutation::ChangeHrDeltaTC(change_hr_delta_t_c::mutation::ChangeHrDeltaTC { new_hr_delta_t_c: snapshot.hr_delta_t_c.clone() }));
        mutations.push(Din16798Mutation::ChangeHrTH(change_hr_t_h::mutation::ChangeHrTH { new_hr_t_h: snapshot.hr_t_h.clone() }));
        mutations.push(Din16798Mutation::ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::mutation::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: snapshot.hr_savings_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeN50HInv(change_n50_h_inv::mutation::ChangeN50HInv { new_n50_h_inv: snapshot.n50_h_inv.clone() }));
        mutations.push(Din16798Mutation::ChangeVolumeM3(change_volume_m3::mutation::ChangeVolumeM3 { new_volume_m3: snapshot.volume_m3.clone() }));
        mutations.push(Din16798Mutation::ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::mutation::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: snapshot.infiltration_allowance_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeCellarAreaM2(change_cellar_area_m2::mutation::ChangeCellarAreaM2 { new_cellar_area_m2: snapshot.cellar_area_m2.clone() }));
        mutations.push(Din16798Mutation::ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::mutation::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: snapshot.cellar_ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeHTrWK(change_h_tr_w_k::mutation::ChangeHTrWK { new_h_tr_w_k: snapshot.h_tr_w_k.clone() }));
        mutations.push(Din16798Mutation::ChangeHVeWK(change_h_ve_w_k::mutation::ChangeHVeWK { new_h_ve_w_k: snapshot.h_ve_w_k.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaEC(change_theta_e_c::mutation::ChangeThetaEC { new_theta_e_c: snapshot.theta_e_c.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaSetC(change_theta_set_c::mutation::ChangeThetaSetC { new_theta_set_c: snapshot.theta_set_c.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingDeltaTH(change_cooling_delta_t_h::mutation::ChangeCoolingDeltaTH { new_cooling_delta_t_h: snapshot.cooling_delta_t_h.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingGainsKwh(change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh { new_cooling_gains_kwh: snapshot.cooling_gains_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::mutation::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: snapshot.cooling_utilization_factor.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingReferenceKwh(change_cooling_reference_kwh::mutation::ChangeCoolingReferenceKwh { new_cooling_reference_kwh: snapshot.cooling_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeChillerType(change_chiller_type::mutation::ChangeChillerType { new_chiller_type: snapshot.chiller_type.clone() }));
        mutations.push(Din16798Mutation::ChangeEerActual(change_eer_actual::mutation::ChangeEerActual { new_eer_actual: snapshot.eer_actual.clone() }));
        mutations.push(Din16798Mutation::ChangeQCKwh(change_q_c_kwh::mutation::ChangeQCKwh { new_q_c_kwh: snapshot.q_c_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeGenerationReferenceKwh(change_generation_reference_kwh::mutation::ChangeGenerationReferenceKwh { new_generation_reference_kwh: snapshot.generation_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeDataCenterSupplyC(change_data_center_supply_c::mutation::ChangeDataCenterSupplyC { new_data_center_supply_c: snapshot.data_center_supply_c.clone() }));
        mutations.push(Din16798Mutation::ChangeHStWK(change_h_st_w_k::mutation::ChangeHStWK { new_h_st_w_k: snapshot.h_st_w_k.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaStC(change_theta_st_c::mutation::ChangeThetaStC { new_theta_st_c: snapshot.theta_st_c.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaAmbC(change_theta_amb_c::mutation::ChangeThetaAmbC { new_theta_amb_c: snapshot.theta_amb_c.clone() }));
        mutations.push(Din16798Mutation::ChangeStorageTH(change_storage_t_h::mutation::ChangeStorageTH { new_storage_t_h: snapshot.storage_t_h.clone() }));
        mutations.push(Din16798Mutation::ChangeStorageAllowanceKwh(change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh { new_storage_allowance_kwh: snapshot.storage_allowance_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeDhwDeliveryC(change_dhw_delivery_c::mutation::ChangeDhwDeliveryC { new_dhw_delivery_c: snapshot.dhw_delivery_c.clone() }));
        mutations.push(Din16798Mutation::ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass { new_duct_class: snapshot.duct_class.clone() }));
        mutations.push(Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::mutation::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: snapshot.duct_test_pressure_pa.clone() }));
        mutations.push(Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::mutation::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: snapshot.duct_leakage_m3_s_m2.clone() }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    /// ⚖️ One value per `Din16798Mutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring `process3d`'s own `every_mutation()` fixture.
    fn every_mutation() -> Vec<Din16798Mutation> {
        vec![
        Din16798Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
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

    fn round_trip(base: &Din16798Snapshot, mutation: &Din16798Mutation) -> Din16798Snapshot {
        let forward = vcs::apply_mutation(base, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<Din16798Mutation as protocol::SemanticMutation<Din16798Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[test]
    fn every_variant_round_trips_via_inverse() {
        let base = Din16798Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::os_spr::testkit` — the bare `protocol::testkit` path is ambiguous crate-wide because `os_pack` also re-exports a `testkit` module), exercised against the three most structurally
    /// distinct variants: the repurposed enum-typed slot (`change-annex`), a typical `f64` scalar
    /// (`change-t-op-c`), and a `String` scalar (`change-occupancy`).

    #[test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: "office".to_string() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_t_op_c_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeTOpC(change_t_op_c::mutation::ChangeTOpC { new_t_op_c: 24.5 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din16798Mutation::ChangeBedrooms(change_bedrooms::mutation::ChangeBedrooms { new_bedrooms: 4 }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[test]
    fn change_occupancy_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeOccupancy(change_occupancy::mutation::ChangeOccupancy { new_occupancy: "office".to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Din16798Mutation::ChangeDuctClass(change_duct_class::mutation::ChangeDuctClass { new_duct_class: "B".to_string() }).diff(&base);
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
