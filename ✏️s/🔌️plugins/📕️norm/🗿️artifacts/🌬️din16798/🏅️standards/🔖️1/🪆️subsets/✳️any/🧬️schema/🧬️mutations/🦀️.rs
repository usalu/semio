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
//! are mounted directly as `mutations`-sibling modules in `🦀️.rs`, each with its own unique
//! emoji-prefixed directory (this lane's agent owns `🦀️.rs` and the emoji-uniqueness policy
//! rule, so the wave-2 precedent's self-wiring `#[path = "."]` blocks and reused `🔧` emoji across
//! all 61 dirs are both retired here in favour of real glue mounts + distinct emoji). That covers the
//! PRODUCTION mounts only: the handcrafted mutation-fixture tests in `🧪️FixtureTests` at the foot of
//! this file ARE self-wired with `#[path = "."]`, because `🦀️.rs` is shared with the agents
//! migrating the other thirteen norm artifacts and must not absorb this artifact's test mounts.

use crate::diff::Din16798Diff;
use crate::Din16798Snapshot;

//#region 🔖️Leaves
use super::change_air_speed_m_s;
use super::change_annex;
use super::change_bedrooms;
use super::change_cellar_area_m2;
use super::change_cellar_ventilation_m3_h;
use super::change_chiller_type;
use super::change_co2_ppm;
use super::change_comfort_category;
use super::change_cooling_delta_t_h;
use super::change_cooling_gains_kwh;
use super::change_cooling_reference_kwh;
use super::change_cooling_utilization_factor;
use super::change_data_center_supply_c;
use super::change_df_percent;
use super::change_dhw_delivery_c;
use super::change_duct_class;
use super::change_duct_leakage_m3_s_m2;
use super::change_duct_test_pressure_pa;
use super::change_dwelling_ventilation_m3_h;
use super::change_eer_actual;
use super::change_fan_energy_reference_kwh;
use super::change_fan_q_v_m3_s;
use super::change_fan_t_run_h;
use super::change_floor_area_m2;
use super::change_generation_reference_kwh;
use super::change_h_st_w_k;
use super::change_h_tr_w_k;
use super::change_h_ve_w_k;
use super::change_heat_recovery_eta;
use super::change_heat_recovery_eta_min;
use super::change_hr_cp_j_kgk;
use super::change_hr_delta_t_c;
use super::change_hr_m_dot_kg_s;
use super::change_hr_savings_reference_kwh;
use super::change_hr_t_h;
use super::change_humidification_provided_kg_h;
use super::change_humidification_required_kg_h;
use super::change_ida_class;
use super::change_infiltration_allowance_m3_h;
use super::change_l_aeq_db;
use super::change_n50_h_inv;
use super::change_night_setback_k;
use super::change_occupancy;
use super::change_occupants;
use super::change_persons;
use super::change_q_c_kwh;
use super::change_residential_ventilation_m3_h;
use super::change_rh_percent;
use super::change_sfp_required_class;
use super::change_sfp_w_m3_s;
use super::change_storage_allowance_kwh;
use super::change_storage_t_h;
use super::change_system_type;
use super::change_t_op_c;
use super::change_theta_amb_c;
use super::change_theta_e_c;
use super::change_theta_rm_c;
use super::change_theta_set_c;
use super::change_theta_st_c;
use super::change_ventilation_m3_h;
use super::change_volume_m3;
use super::change_years_since_inspection;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the din16798 document, derived per
/// `📓️derivation-rules.md` from `Din16798Snapshot`'s flat scalar shape.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Din16798Snapshot, diff = Din16798Diff, schema = "norm.din16798")]
pub enum Din16798Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeOccupancy(change_occupancy::ChangeOccupancy),
    ChangeComfortCategory(change_comfort_category::ChangeComfortCategory),
    ChangeTOpC(change_t_op_c::ChangeTOpC),
    ChangeRhPercent(change_rh_percent::ChangeRhPercent),
    ChangeAirSpeedMS(change_air_speed_m_s::ChangeAirSpeedMS),
    ChangeThetaRmC(change_theta_rm_c::ChangeThetaRmC),
    ChangeCo2Ppm(change_co2_ppm::ChangeCo2Ppm),
    ChangeDfPercent(change_df_percent::ChangeDfPercent),
    ChangeLAeqDb(change_l_aeq_db::ChangeLAeqDb),
    ChangePersons(change_persons::ChangePersons),
    ChangeIdaClass(change_ida_class::ChangeIdaClass),
    ChangeVentilationM3H(change_ventilation_m3_h::ChangeVentilationM3H),
    ChangeFloorAreaM2(change_floor_area_m2::ChangeFloorAreaM2),
    ChangeBedrooms(change_bedrooms::ChangeBedrooms),
    ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::ChangeDwellingVentilationM3H),
    ChangeOccupants(change_occupants::ChangeOccupants),
    ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::ChangeResidentialVentilationM3H),
    ChangeSfpWM3S(change_sfp_w_m3_s::ChangeSfpWM3S),
    ChangeSfpRequiredClass(change_sfp_required_class::ChangeSfpRequiredClass),
    ChangeHeatRecoveryEta(change_heat_recovery_eta::ChangeHeatRecoveryEta),
    ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::ChangeHeatRecoveryEtaMin),
    ChangeSystemType(change_system_type::ChangeSystemType),
    ChangeYearsSinceInspection(change_years_since_inspection::ChangeYearsSinceInspection),
    ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::ChangeHumidificationRequiredKgH),
    ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::ChangeHumidificationProvidedKgH),
    ChangeFanQVM3S(change_fan_q_v_m3_s::ChangeFanQVM3S),
    ChangeFanTRunH(change_fan_t_run_h::ChangeFanTRunH),
    ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::ChangeFanEnergyReferenceKwh),
    ChangeNightSetbackK(change_night_setback_k::ChangeNightSetbackK),
    ChangeHrMDotKgS(change_hr_m_dot_kg_s::ChangeHrMDotKgS),
    ChangeHrCpJKgk(change_hr_cp_j_kgk::ChangeHrCpJKgk),
    ChangeHrDeltaTC(change_hr_delta_t_c::ChangeHrDeltaTC),
    ChangeHrTH(change_hr_t_h::ChangeHrTH),
    ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::ChangeHrSavingsReferenceKwh),
    ChangeN50HInv(change_n50_h_inv::ChangeN50HInv),
    ChangeVolumeM3(change_volume_m3::ChangeVolumeM3),
    ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::ChangeInfiltrationAllowanceM3H),
    ChangeCellarAreaM2(change_cellar_area_m2::ChangeCellarAreaM2),
    ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::ChangeCellarVentilationM3H),
    ChangeHTrWK(change_h_tr_w_k::ChangeHTrWK),
    ChangeHVeWK(change_h_ve_w_k::ChangeHVeWK),
    ChangeThetaEC(change_theta_e_c::ChangeThetaEC),
    ChangeThetaSetC(change_theta_set_c::ChangeThetaSetC),
    ChangeCoolingDeltaTH(change_cooling_delta_t_h::ChangeCoolingDeltaTH),
    ChangeCoolingGainsKwh(change_cooling_gains_kwh::ChangeCoolingGainsKwh),
    ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::ChangeCoolingUtilizationFactor),
    ChangeCoolingReferenceKwh(change_cooling_reference_kwh::ChangeCoolingReferenceKwh),
    ChangeChillerType(change_chiller_type::ChangeChillerType),
    ChangeEerActual(change_eer_actual::ChangeEerActual),
    ChangeQCKwh(change_q_c_kwh::ChangeQCKwh),
    ChangeGenerationReferenceKwh(change_generation_reference_kwh::ChangeGenerationReferenceKwh),
    ChangeDataCenterSupplyC(change_data_center_supply_c::ChangeDataCenterSupplyC),
    ChangeHStWK(change_h_st_w_k::ChangeHStWK),
    ChangeThetaStC(change_theta_st_c::ChangeThetaStC),
    ChangeThetaAmbC(change_theta_amb_c::ChangeThetaAmbC),
    ChangeStorageTH(change_storage_t_h::ChangeStorageTH),
    ChangeStorageAllowanceKwh(change_storage_allowance_kwh::ChangeStorageAllowanceKwh),
    ChangeDhwDeliveryC(change_dhw_delivery_c::ChangeDhwDeliveryC),
    ChangeDuctClass(change_duct_class::ChangeDuctClass),
    ChangeDuctTestPressurePa(change_duct_test_pressure_pa::ChangeDuctTestPressurePa),
    ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::ChangeDuctLeakageM3SM2),
}

/// 🏷️ Every declared kind of [`Din16798Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🔣️oracle.json` publishes as the `din16798-1-any`
/// mutation catalog and `../../../../../🧪️tests/🌬️mutate-din16798-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
    "change-annex",
    "change-occupancy",
    "change-comfort-category",
    "change-t-op-c",
    "change-rh-percent",
    "change-air-speed-ms",
    "change-theta-rm-c",
    "change-co2-ppm",
    "change-df-percent",
    "change-l-aeq-db",
    "change-persons",
    "change-ida-class",
    "change-ventilation-m3-h",
    "change-floor-area-m2",
    "change-bedrooms",
    "change-dwelling-ventilation-m3-h",
    "change-occupants",
    "change-residential-ventilation-m3-h",
    "change-sfp-wm3-s",
    "change-sfp-required-class",
    "change-heat-recovery-eta",
    "change-heat-recovery-eta-min",
    "change-system-type",
    "change-years-since-inspection",
    "change-humidification-required-kg-h",
    "change-humidification-provided-kg-h",
    "change-fan-qvm3-s",
    "change-fan-t-run-h",
    "change-fan-energy-reference-kwh",
    "change-night-setback-k",
    "change-hr-m-dot-kg-s",
    "change-hr-cp-j-kgk",
    "change-hr-delta-tc",
    "change-hr-th",
    "change-hr-savings-reference-kwh",
    "change-n50-h-inv",
    "change-volume-m3",
    "change-infiltration-allowance-m3-h",
    "change-cellar-area-m2",
    "change-cellar-ventilation-m3-h",
    "change-h-tr-wk",
    "change-h-ve-wk",
    "change-theta-ec",
    "change-theta-set-c",
    "change-cooling-delta-th",
    "change-cooling-gains-kwh",
    "change-cooling-utilization-factor",
    "change-cooling-reference-kwh",
    "change-chiller-type",
    "change-eer-actual",
    "change-qc-kwh",
    "change-generation-reference-kwh",
    "change-data-center-supply-c",
    "change-h-st-wk",
    "change-theta-st-c",
    "change-theta-amb-c",
    "change-storage-th",
    "change-storage-allowance-kwh",
    "change-dhw-delivery-c",
    "change-duct-class",
    "change-duct-test-pressure-pa",
    "change-duct-leakage-m3-sm2",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl Din16798Mutation {
    /// 📤️ Decomposes a whole `Din16798Snapshot` into one `change-<field>` mutation per persistent
    /// field — the closed-vocabulary replacement for the banned whole-document-replace variant, used
    /// by `import_media`'s `"model:in"` port and the `set-snapshot` app command to bundle a bulk
    /// document replacement into a single atomic `Emit::commit`.
    pub fn from_snapshot(snapshot: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        let mut mutations = Vec::with_capacity(62);
        mutations.push(Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex }));
        mutations.push(Din16798Mutation::ChangeOccupancy(change_occupancy::ChangeOccupancy { new_occupancy: snapshot.occupancy.clone() }));
        mutations.push(Din16798Mutation::ChangeComfortCategory(change_comfort_category::ChangeComfortCategory { new_comfort_category: snapshot.comfort_category.clone() }));
        mutations.push(Din16798Mutation::ChangeTOpC(change_t_op_c::ChangeTOpC { new_t_op_c: snapshot.t_op_c }));
        mutations.push(Din16798Mutation::ChangeRhPercent(change_rh_percent::ChangeRhPercent { new_rh_percent: snapshot.rh_percent }));
        mutations.push(Din16798Mutation::ChangeAirSpeedMS(change_air_speed_m_s::ChangeAirSpeedMS { new_air_speed_m_s: snapshot.air_speed_m_s }));
        mutations.push(Din16798Mutation::ChangeThetaRmC(change_theta_rm_c::ChangeThetaRmC { new_theta_rm_c: snapshot.theta_rm_c }));
        mutations.push(Din16798Mutation::ChangeCo2Ppm(change_co2_ppm::ChangeCo2Ppm { new_co2_ppm: snapshot.co2_ppm }));
        mutations.push(Din16798Mutation::ChangeDfPercent(change_df_percent::ChangeDfPercent { new_df_percent: snapshot.df_percent }));
        mutations.push(Din16798Mutation::ChangeLAeqDb(change_l_aeq_db::ChangeLAeqDb { new_l_aeq_db: snapshot.l_aeq_db }));
        mutations.push(Din16798Mutation::ChangePersons(change_persons::ChangePersons { new_persons: snapshot.persons }));
        mutations.push(Din16798Mutation::ChangeIdaClass(change_ida_class::ChangeIdaClass { new_ida_class: snapshot.ida_class.clone() }));
        mutations.push(Din16798Mutation::ChangeVentilationM3H(change_ventilation_m3_h::ChangeVentilationM3H { new_ventilation_m3_h: snapshot.ventilation_m3_h }));
        mutations.push(Din16798Mutation::ChangeFloorAreaM2(change_floor_area_m2::ChangeFloorAreaM2 { new_floor_area_m2: snapshot.floor_area_m2 }));
        mutations.push(Din16798Mutation::ChangeBedrooms(change_bedrooms::ChangeBedrooms { new_bedrooms: snapshot.bedrooms }));
        mutations.push(Din16798Mutation::ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: snapshot.dwelling_ventilation_m3_h }));
        mutations.push(Din16798Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: snapshot.occupants }));
        mutations.push(Din16798Mutation::ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: snapshot.residential_ventilation_m3_h }));
        mutations.push(Din16798Mutation::ChangeSfpWM3S(change_sfp_w_m3_s::ChangeSfpWM3S { new_sfp_w_m3_s: snapshot.sfp_w_m3_s }));
        mutations.push(Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::ChangeSfpRequiredClass { new_sfp_required_class: snapshot.sfp_required_class }));
        mutations.push(Din16798Mutation::ChangeHeatRecoveryEta(change_heat_recovery_eta::ChangeHeatRecoveryEta { new_heat_recovery_eta: snapshot.heat_recovery_eta }));
        mutations.push(Din16798Mutation::ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: snapshot.heat_recovery_eta_min }));
        mutations.push(Din16798Mutation::ChangeSystemType(change_system_type::ChangeSystemType { new_system_type: snapshot.system_type.clone() }));
        mutations.push(Din16798Mutation::ChangeYearsSinceInspection(change_years_since_inspection::ChangeYearsSinceInspection { new_years_since_inspection: snapshot.years_since_inspection }));
        mutations.push(Din16798Mutation::ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: snapshot.humidification_required_kg_h }));
        mutations.push(Din16798Mutation::ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: snapshot.humidification_provided_kg_h }));
        mutations.push(Din16798Mutation::ChangeFanQVM3S(change_fan_q_v_m3_s::ChangeFanQVM3S { new_fan_q_v_m3_s: snapshot.fan_q_v_m3_s }));
        mutations.push(Din16798Mutation::ChangeFanTRunH(change_fan_t_run_h::ChangeFanTRunH { new_fan_t_run_h: snapshot.fan_t_run_h }));
        mutations.push(Din16798Mutation::ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: snapshot.fan_energy_reference_kwh }));
        mutations.push(Din16798Mutation::ChangeNightSetbackK(change_night_setback_k::ChangeNightSetbackK { new_night_setback_k: snapshot.night_setback_k }));
        mutations.push(Din16798Mutation::ChangeHrMDotKgS(change_hr_m_dot_kg_s::ChangeHrMDotKgS { new_hr_m_dot_kg_s: snapshot.hr_m_dot_kg_s }));
        mutations.push(Din16798Mutation::ChangeHrCpJKgk(change_hr_cp_j_kgk::ChangeHrCpJKgk { new_hr_cp_j_kgk: snapshot.hr_cp_j_kgk }));
        mutations.push(Din16798Mutation::ChangeHrDeltaTC(change_hr_delta_t_c::ChangeHrDeltaTC { new_hr_delta_t_c: snapshot.hr_delta_t_c }));
        mutations.push(Din16798Mutation::ChangeHrTH(change_hr_t_h::ChangeHrTH { new_hr_t_h: snapshot.hr_t_h }));
        mutations.push(Din16798Mutation::ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: snapshot.hr_savings_reference_kwh }));
        mutations.push(Din16798Mutation::ChangeN50HInv(change_n50_h_inv::ChangeN50HInv { new_n50_h_inv: snapshot.n50_h_inv }));
        mutations.push(Din16798Mutation::ChangeVolumeM3(change_volume_m3::ChangeVolumeM3 { new_volume_m3: snapshot.volume_m3 }));
        mutations.push(Din16798Mutation::ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: snapshot.infiltration_allowance_m3_h }));
        mutations.push(Din16798Mutation::ChangeCellarAreaM2(change_cellar_area_m2::ChangeCellarAreaM2 { new_cellar_area_m2: snapshot.cellar_area_m2 }));
        mutations.push(Din16798Mutation::ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: snapshot.cellar_ventilation_m3_h }));
        mutations.push(Din16798Mutation::ChangeHTrWK(change_h_tr_w_k::ChangeHTrWK { new_h_tr_w_k: snapshot.h_tr_w_k }));
        mutations.push(Din16798Mutation::ChangeHVeWK(change_h_ve_w_k::ChangeHVeWK { new_h_ve_w_k: snapshot.h_ve_w_k }));
        mutations.push(Din16798Mutation::ChangeThetaEC(change_theta_e_c::ChangeThetaEC { new_theta_e_c: snapshot.theta_e_c }));
        mutations.push(Din16798Mutation::ChangeThetaSetC(change_theta_set_c::ChangeThetaSetC { new_theta_set_c: snapshot.theta_set_c }));
        mutations.push(Din16798Mutation::ChangeCoolingDeltaTH(change_cooling_delta_t_h::ChangeCoolingDeltaTH { new_cooling_delta_t_h: snapshot.cooling_delta_t_h }));
        mutations.push(Din16798Mutation::ChangeCoolingGainsKwh(change_cooling_gains_kwh::ChangeCoolingGainsKwh { new_cooling_gains_kwh: snapshot.cooling_gains_kwh }));
        mutations.push(Din16798Mutation::ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: snapshot.cooling_utilization_factor }));
        mutations.push(Din16798Mutation::ChangeCoolingReferenceKwh(change_cooling_reference_kwh::ChangeCoolingReferenceKwh { new_cooling_reference_kwh: snapshot.cooling_reference_kwh }));
        mutations.push(Din16798Mutation::ChangeChillerType(change_chiller_type::ChangeChillerType { new_chiller_type: snapshot.chiller_type.clone() }));
        mutations.push(Din16798Mutation::ChangeEerActual(change_eer_actual::ChangeEerActual { new_eer_actual: snapshot.eer_actual }));
        mutations.push(Din16798Mutation::ChangeQCKwh(change_q_c_kwh::ChangeQCKwh { new_q_c_kwh: snapshot.q_c_kwh }));
        mutations.push(Din16798Mutation::ChangeGenerationReferenceKwh(change_generation_reference_kwh::ChangeGenerationReferenceKwh { new_generation_reference_kwh: snapshot.generation_reference_kwh }));
        mutations.push(Din16798Mutation::ChangeDataCenterSupplyC(change_data_center_supply_c::ChangeDataCenterSupplyC { new_data_center_supply_c: snapshot.data_center_supply_c }));
        mutations.push(Din16798Mutation::ChangeHStWK(change_h_st_w_k::ChangeHStWK { new_h_st_w_k: snapshot.h_st_w_k }));
        mutations.push(Din16798Mutation::ChangeThetaStC(change_theta_st_c::ChangeThetaStC { new_theta_st_c: snapshot.theta_st_c }));
        mutations.push(Din16798Mutation::ChangeThetaAmbC(change_theta_amb_c::ChangeThetaAmbC { new_theta_amb_c: snapshot.theta_amb_c }));
        mutations.push(Din16798Mutation::ChangeStorageTH(change_storage_t_h::ChangeStorageTH { new_storage_t_h: snapshot.storage_t_h }));
        mutations.push(Din16798Mutation::ChangeStorageAllowanceKwh(change_storage_allowance_kwh::ChangeStorageAllowanceKwh { new_storage_allowance_kwh: snapshot.storage_allowance_kwh }));
        mutations.push(Din16798Mutation::ChangeDhwDeliveryC(change_dhw_delivery_c::ChangeDhwDeliveryC { new_dhw_delivery_c: snapshot.dhw_delivery_c }));
        mutations.push(Din16798Mutation::ChangeDuctClass(change_duct_class::ChangeDuctClass { new_duct_class: snapshot.duct_class.clone() }));
        mutations.push(Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: snapshot.duct_test_pressure_pa }));
        mutations.push(Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: snapshot.duct_leakage_m3_s_m2 }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the agents
// migrating the other thirteen norm artifacts, so the production mounts above stay untouched while
// each artifact owns its own test mounts. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes each leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`Din16798Mutation`]. The generated test host of
/// `../../../../../🧪️tests/🌬️mutate-din16798-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din16798_mutation_json(text: &str) -> Result<Din16798Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_din16798_mutation(base: &Din16798Snapshot, mutation: &Din16798Mutation) -> Result<(Din16798Snapshot, Vec<String>), String> {
    let raised = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <Din16798Diff as protocol::MutationDiff<Din16798Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `🌬️mutate-din16798-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_din16798_mutation(mutation: &Din16798Mutation, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
