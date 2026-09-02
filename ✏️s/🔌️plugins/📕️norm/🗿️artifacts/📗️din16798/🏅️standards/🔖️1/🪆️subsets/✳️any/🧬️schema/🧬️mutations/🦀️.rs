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

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
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
/// mutation catalog and `../../../../../🧪️tests/mutate-din16798-1` registers its scenarios from. The
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
        mutations.push(Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: snapshot.annex.clone() }));
        mutations.push(Din16798Mutation::ChangeOccupancy(change_occupancy::ChangeOccupancy { new_occupancy: snapshot.occupancy.clone() }));
        mutations.push(Din16798Mutation::ChangeComfortCategory(change_comfort_category::ChangeComfortCategory { new_comfort_category: snapshot.comfort_category.clone() }));
        mutations.push(Din16798Mutation::ChangeTOpC(change_t_op_c::ChangeTOpC { new_t_op_c: snapshot.t_op_c.clone() }));
        mutations.push(Din16798Mutation::ChangeRhPercent(change_rh_percent::ChangeRhPercent { new_rh_percent: snapshot.rh_percent.clone() }));
        mutations.push(Din16798Mutation::ChangeAirSpeedMS(change_air_speed_m_s::ChangeAirSpeedMS { new_air_speed_m_s: snapshot.air_speed_m_s.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaRmC(change_theta_rm_c::ChangeThetaRmC { new_theta_rm_c: snapshot.theta_rm_c.clone() }));
        mutations.push(Din16798Mutation::ChangeCo2Ppm(change_co2_ppm::ChangeCo2Ppm { new_co2_ppm: snapshot.co2_ppm.clone() }));
        mutations.push(Din16798Mutation::ChangeDfPercent(change_df_percent::ChangeDfPercent { new_df_percent: snapshot.df_percent.clone() }));
        mutations.push(Din16798Mutation::ChangeLAeqDb(change_l_aeq_db::ChangeLAeqDb { new_l_aeq_db: snapshot.l_aeq_db.clone() }));
        mutations.push(Din16798Mutation::ChangePersons(change_persons::ChangePersons { new_persons: snapshot.persons.clone() }));
        mutations.push(Din16798Mutation::ChangeIdaClass(change_ida_class::ChangeIdaClass { new_ida_class: snapshot.ida_class.clone() }));
        mutations.push(Din16798Mutation::ChangeVentilationM3H(change_ventilation_m3_h::ChangeVentilationM3H { new_ventilation_m3_h: snapshot.ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeFloorAreaM2(change_floor_area_m2::ChangeFloorAreaM2 { new_floor_area_m2: snapshot.floor_area_m2.clone() }));
        mutations.push(Din16798Mutation::ChangeBedrooms(change_bedrooms::ChangeBedrooms { new_bedrooms: snapshot.bedrooms.clone() }));
        mutations.push(Din16798Mutation::ChangeDwellingVentilationM3H(change_dwelling_ventilation_m3_h::ChangeDwellingVentilationM3H { new_dwelling_ventilation_m3_h: snapshot.dwelling_ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeOccupants(change_occupants::ChangeOccupants { new_occupants: snapshot.occupants.clone() }));
        mutations.push(Din16798Mutation::ChangeResidentialVentilationM3H(change_residential_ventilation_m3_h::ChangeResidentialVentilationM3H { new_residential_ventilation_m3_h: snapshot.residential_ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeSfpWM3S(change_sfp_w_m3_s::ChangeSfpWM3S { new_sfp_w_m3_s: snapshot.sfp_w_m3_s.clone() }));
        mutations.push(Din16798Mutation::ChangeSfpRequiredClass(change_sfp_required_class::ChangeSfpRequiredClass { new_sfp_required_class: snapshot.sfp_required_class.clone() }));
        mutations.push(Din16798Mutation::ChangeHeatRecoveryEta(change_heat_recovery_eta::ChangeHeatRecoveryEta { new_heat_recovery_eta: snapshot.heat_recovery_eta.clone() }));
        mutations.push(Din16798Mutation::ChangeHeatRecoveryEtaMin(change_heat_recovery_eta_min::ChangeHeatRecoveryEtaMin { new_heat_recovery_eta_min: snapshot.heat_recovery_eta_min.clone() }));
        mutations.push(Din16798Mutation::ChangeSystemType(change_system_type::ChangeSystemType { new_system_type: snapshot.system_type.clone() }));
        mutations.push(Din16798Mutation::ChangeYearsSinceInspection(change_years_since_inspection::ChangeYearsSinceInspection { new_years_since_inspection: snapshot.years_since_inspection.clone() }));
        mutations.push(Din16798Mutation::ChangeHumidificationRequiredKgH(change_humidification_required_kg_h::ChangeHumidificationRequiredKgH { new_humidification_required_kg_h: snapshot.humidification_required_kg_h.clone() }));
        mutations.push(Din16798Mutation::ChangeHumidificationProvidedKgH(change_humidification_provided_kg_h::ChangeHumidificationProvidedKgH { new_humidification_provided_kg_h: snapshot.humidification_provided_kg_h.clone() }));
        mutations.push(Din16798Mutation::ChangeFanQVM3S(change_fan_q_v_m3_s::ChangeFanQVM3S { new_fan_q_v_m3_s: snapshot.fan_q_v_m3_s.clone() }));
        mutations.push(Din16798Mutation::ChangeFanTRunH(change_fan_t_run_h::ChangeFanTRunH { new_fan_t_run_h: snapshot.fan_t_run_h.clone() }));
        mutations.push(Din16798Mutation::ChangeFanEnergyReferenceKwh(change_fan_energy_reference_kwh::ChangeFanEnergyReferenceKwh { new_fan_energy_reference_kwh: snapshot.fan_energy_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeNightSetbackK(change_night_setback_k::ChangeNightSetbackK { new_night_setback_k: snapshot.night_setback_k.clone() }));
        mutations.push(Din16798Mutation::ChangeHrMDotKgS(change_hr_m_dot_kg_s::ChangeHrMDotKgS { new_hr_m_dot_kg_s: snapshot.hr_m_dot_kg_s.clone() }));
        mutations.push(Din16798Mutation::ChangeHrCpJKgk(change_hr_cp_j_kgk::ChangeHrCpJKgk { new_hr_cp_j_kgk: snapshot.hr_cp_j_kgk.clone() }));
        mutations.push(Din16798Mutation::ChangeHrDeltaTC(change_hr_delta_t_c::ChangeHrDeltaTC { new_hr_delta_t_c: snapshot.hr_delta_t_c.clone() }));
        mutations.push(Din16798Mutation::ChangeHrTH(change_hr_t_h::ChangeHrTH { new_hr_t_h: snapshot.hr_t_h.clone() }));
        mutations.push(Din16798Mutation::ChangeHrSavingsReferenceKwh(change_hr_savings_reference_kwh::ChangeHrSavingsReferenceKwh { new_hr_savings_reference_kwh: snapshot.hr_savings_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeN50HInv(change_n50_h_inv::ChangeN50HInv { new_n50_h_inv: snapshot.n50_h_inv.clone() }));
        mutations.push(Din16798Mutation::ChangeVolumeM3(change_volume_m3::ChangeVolumeM3 { new_volume_m3: snapshot.volume_m3.clone() }));
        mutations.push(Din16798Mutation::ChangeInfiltrationAllowanceM3H(change_infiltration_allowance_m3_h::ChangeInfiltrationAllowanceM3H { new_infiltration_allowance_m3_h: snapshot.infiltration_allowance_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeCellarAreaM2(change_cellar_area_m2::ChangeCellarAreaM2 { new_cellar_area_m2: snapshot.cellar_area_m2.clone() }));
        mutations.push(Din16798Mutation::ChangeCellarVentilationM3H(change_cellar_ventilation_m3_h::ChangeCellarVentilationM3H { new_cellar_ventilation_m3_h: snapshot.cellar_ventilation_m3_h.clone() }));
        mutations.push(Din16798Mutation::ChangeHTrWK(change_h_tr_w_k::ChangeHTrWK { new_h_tr_w_k: snapshot.h_tr_w_k.clone() }));
        mutations.push(Din16798Mutation::ChangeHVeWK(change_h_ve_w_k::ChangeHVeWK { new_h_ve_w_k: snapshot.h_ve_w_k.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaEC(change_theta_e_c::ChangeThetaEC { new_theta_e_c: snapshot.theta_e_c.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaSetC(change_theta_set_c::ChangeThetaSetC { new_theta_set_c: snapshot.theta_set_c.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingDeltaTH(change_cooling_delta_t_h::ChangeCoolingDeltaTH { new_cooling_delta_t_h: snapshot.cooling_delta_t_h.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingGainsKwh(change_cooling_gains_kwh::ChangeCoolingGainsKwh { new_cooling_gains_kwh: snapshot.cooling_gains_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingUtilizationFactor(change_cooling_utilization_factor::ChangeCoolingUtilizationFactor { new_cooling_utilization_factor: snapshot.cooling_utilization_factor.clone() }));
        mutations.push(Din16798Mutation::ChangeCoolingReferenceKwh(change_cooling_reference_kwh::ChangeCoolingReferenceKwh { new_cooling_reference_kwh: snapshot.cooling_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeChillerType(change_chiller_type::ChangeChillerType { new_chiller_type: snapshot.chiller_type.clone() }));
        mutations.push(Din16798Mutation::ChangeEerActual(change_eer_actual::ChangeEerActual { new_eer_actual: snapshot.eer_actual.clone() }));
        mutations.push(Din16798Mutation::ChangeQCKwh(change_q_c_kwh::ChangeQCKwh { new_q_c_kwh: snapshot.q_c_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeGenerationReferenceKwh(change_generation_reference_kwh::ChangeGenerationReferenceKwh { new_generation_reference_kwh: snapshot.generation_reference_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeDataCenterSupplyC(change_data_center_supply_c::ChangeDataCenterSupplyC { new_data_center_supply_c: snapshot.data_center_supply_c.clone() }));
        mutations.push(Din16798Mutation::ChangeHStWK(change_h_st_w_k::ChangeHStWK { new_h_st_w_k: snapshot.h_st_w_k.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaStC(change_theta_st_c::ChangeThetaStC { new_theta_st_c: snapshot.theta_st_c.clone() }));
        mutations.push(Din16798Mutation::ChangeThetaAmbC(change_theta_amb_c::ChangeThetaAmbC { new_theta_amb_c: snapshot.theta_amb_c.clone() }));
        mutations.push(Din16798Mutation::ChangeStorageTH(change_storage_t_h::ChangeStorageTH { new_storage_t_h: snapshot.storage_t_h.clone() }));
        mutations.push(Din16798Mutation::ChangeStorageAllowanceKwh(change_storage_allowance_kwh::ChangeStorageAllowanceKwh { new_storage_allowance_kwh: snapshot.storage_allowance_kwh.clone() }));
        mutations.push(Din16798Mutation::ChangeDhwDeliveryC(change_dhw_delivery_c::ChangeDhwDeliveryC { new_dhw_delivery_c: snapshot.dhw_delivery_c.clone() }));
        mutations.push(Din16798Mutation::ChangeDuctClass(change_duct_class::ChangeDuctClass { new_duct_class: snapshot.duct_class.clone() }));
        mutations.push(Din16798Mutation::ChangeDuctTestPressurePa(change_duct_test_pressure_pa::ChangeDuctTestPressurePa { new_duct_test_pressure_pa: snapshot.duct_test_pressure_pa.clone() }));
        mutations.push(Din16798Mutation::ChangeDuctLeakageM3SM2(change_duct_leakage_m3_s_m2::ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: snapshot.duct_leakage_m3_s_m2.clone() }));
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;
    use protocol::SemanticMutation;

    /// ⚖️ One value per `Din16798Mutation` variant — the closed set the semantics/round-trip tests
    /// iterate, mirroring `process3d`'s own `every_mutation()` fixture.
    fn every_mutation() -> Vec<Din16798Mutation> {
        vec![
            Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
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

    fn round_trip(base: &Din16798Snapshot, mutation: &Din16798Mutation) -> Din16798Snapshot {
        let forward = vcs::apply_mutation(base, mutation).expect("valid mutation").0;
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
        forward
    }

    #[semio_framework_async_macros::async_test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<Din16798Mutation as protocol::SemanticMutation<Din16798Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    #[semio_framework_async_macros::async_test]
    fn every_variant_round_trips_via_inverse() {
        let base = Din16798Snapshot::default();
        for mutation in every_mutation() {
            round_trip(&base, &mutation);
        }
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
    /// (reachable here as `protocol::os_spr::testkit` — the bare `protocol::testkit` path is ambiguous crate-wide because `os_pack` also re-exports a `testkit` module), exercised against the three most structurally
    /// distinct variants: the repurposed enum-typed slot (`change-annex`), a typical `f64` scalar
    /// (`change-t-op-c`), and a `String` scalar (`change-occupancy`).

    #[semio_framework_async_macros::async_test]
    fn change_annex_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = Din16798Mutation::ChangeOccupancy(change_occupancy::ChangeOccupancy { new_occupancy: "office".to_string() }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    fn change_t_op_c_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeTOpC(change_t_op_c::ChangeTOpC { new_t_op_c: 24.5 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = Din16798Mutation::ChangeBedrooms(change_bedrooms::ChangeBedrooms { new_bedrooms: 4 }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    #[semio_framework_async_macros::async_test]
    fn change_occupancy_satisfies_the_inverse_and_absorb_laws() {
        let base = Din16798Snapshot::default();
        let mutation = Din16798Mutation::ChangeOccupancy(change_occupancy::ChangeOccupancy { new_occupancy: "office".to_string() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).diff().clone();
        let d2 = Din16798Mutation::ChangeDuctClass(change_duct_class::ChangeDuctClass { new_duct_class: "B".to_string() }).diff(&base).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the agents
// migrating the other thirteen norm artifacts, so the production mounts above stay untouched while
// each artifact owns its own test mounts. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes each leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🔀change-air-speed-ms/🧪️tests/doubles-the-draught-air-speed-to-0-point-25-ms/🦀️.rs"]
    mod tests_change_air_speed_m_s_doubles_the_draught_air_speed_to_0_point_25_ms;
    #[path = "🏷️change-annex/🧪️tests/switches-the-check-to-the-en-annex/🦀️.rs"]
    mod tests_change_annex_switches_the_check_to_the_en_annex;
    #[path = "🔢change-bedrooms/🧪️tests/adds-a-fourth-bedroom/🦀️.rs"]
    mod tests_change_bedrooms_adds_a_fourth_bedroom;
    #[path = "🛡️change-cellar-area-m2/🧪️tests/grows-the-cellar-floor-area-to-62-point-5-m2/🦀️.rs"]
    mod tests_change_cellar_area_m2_grows_the_cellar_floor_area_to_62_point_5_m2;
    #[path = "🧯change-cellar-ventilation-m3-h/🧪️tests/raises-the-cellar-airflow-to-22-point-5-m3-per-hour/🦀️.rs"]
    mod tests_change_cellar_ventilation_m3_h_raises_the_cellar_airflow_to_22_point_5_m3_per_hour;
    #[path = "🚨change-chiller-type/🧪️tests/switches-to-a-water-cooled-chiller/🦀️.rs"]
    mod tests_change_chiller_type_switches_to_a_water_cooled_chiller;
    #[path = "🛠️change-co2-ppm/🧪️tests/raises-the-measured-co2-to-950-ppm/🦀️.rs"]
    mod tests_change_co2_ppm_raises_the_measured_co2_to_950_ppm;
    #[path = "🪛change-comfort-category/🧪️tests/tightens-the-comfort-category-to-i/🦀️.rs"]
    mod tests_change_comfort_category_tightens_the_comfort_category_to_i;
    #[path = "🪚change-cooling-delta-th/🧪️tests/extends-the-cooling-period-to-12-point-5-hours/🦀️.rs"]
    mod tests_change_cooling_delta_t_h_extends_the_cooling_period_to_12_point_5_hours;
    #[path = "🪜change-cooling-gains-kwh/🧪️tests/raises-the-internal-cooling-gains-to-7-point-5-kwh/🦀️.rs"]
    mod tests_change_cooling_gains_kwh_raises_the_internal_cooling_gains_to_7_point_5_kwh;
    #[path = "🪝change-cooling-reference-kwh/🧪️tests/raises-the-cooling-reference-to-25-kwh/🦀️.rs"]
    mod tests_change_cooling_reference_kwh_raises_the_cooling_reference_to_25_kwh;
    #[path = "🪣change-cooling-utilization-factor/🧪️tests/raises-the-cooling-utilization-factor-to-0-point-875/🦀️.rs"]
    mod tests_change_cooling_utilization_factor_raises_the_cooling_utilization_factor_to_0_point_875;
    #[path = "🧰change-data-center-supply-c/🧪️tests/raises-the-data-centre-supply-air-to-27-c/🦀️.rs"]
    mod tests_change_data_center_supply_c_raises_the_data_centre_supply_air_to_27_c;
    #[path = "🧵change-df-percent/🧪️tests/raises-the-daylight-factor-to-3-point-75-percent/🦀️.rs"]
    mod tests_change_df_percent_raises_the_daylight_factor_to_3_point_75_percent;
    #[path = "🧶change-dhw-delivery-c/🧪️tests/raises-the-dhw-delivery-temperature-to-60-c/🦀️.rs"]
    mod tests_change_dhw_delivery_c_raises_the_dhw_delivery_temperature_to_60_c;
    #[path = "🪡change-duct-class/🧪️tests/upgrades-the-duct-tightness-class-to-d/🦀️.rs"]
    mod tests_change_duct_class_upgrades_the_duct_tightness_class_to_d;
    #[path = "🪢change-duct-leakage-m3-sm2/🧪️tests/halves-the-measured-duct-leakage-to-0-point-0625/🦀️.rs"]
    mod tests_change_duct_leakage_m3_s_m2_halves_the_measured_duct_leakage_to_0_point_0625;
    #[path = "🧷change-duct-test-pressure-pa/🧪️tests/raises-the-duct-test-pressure-to-500-pa/🦀️.rs"]
    mod tests_change_duct_test_pressure_pa_raises_the_duct_test_pressure_to_500_pa;
    #[path = "🧲change-dwelling-ventilation-m3-h/🧪️tests/raises-the-dwelling-airflow-to-96-m3-per-hour/🦀️.rs"]
    mod tests_change_dwelling_ventilation_m3_h_raises_the_dwelling_airflow_to_96_m3_per_hour;
    #[path = "🪤change-eer-actual/🧪️tests/raises-the-achieved-eer-to-3-point-5/🦀️.rs"]
    mod tests_change_eer_actual_raises_the_achieved_eer_to_3_point_5;
    #[path = "🪒change-fan-energy-reference-kwh/🧪️tests/raises-the-fan-energy-reference-to-18-kwh/🦀️.rs"]
    mod tests_change_fan_energy_reference_kwh_raises_the_fan_energy_reference_to_18_kwh;
    #[path = "🪥change-fan-qvm3-s/🧪️tests/raises-the-fan-volume-flow-to-1-point-5-m3-per-second/🦀️.rs"]
    mod tests_change_fan_q_v_m3_s_raises_the_fan_volume_flow_to_1_point_5_m3_per_second;
    #[path = "🧴change-fan-t-run-h/🧪️tests/extends-the-daily-fan-runtime-to-12-hours/🦀️.rs"]
    mod tests_change_fan_t_run_h_extends_the_daily_fan_runtime_to_12_hours;
    #[path = "🧼change-floor-area-m2/🧪️tests/grows-the-conditioned-floor-area-to-120-m2/🦀️.rs"]
    mod tests_change_floor_area_m2_grows_the_conditioned_floor_area_to_120_m2;
    #[path = "🧽change-generation-reference-kwh/🧪️tests/raises-the-generation-reference-to-450-kwh/🦀️.rs"]
    mod tests_change_generation_reference_kwh_raises_the_generation_reference_to_450_kwh;
    #[path = "🪠change-h-st-wk/🧪️tests/raises-the-storage-loss-coefficient-to-6-point-5-w-per-k/🦀️.rs"]
    mod tests_change_h_st_w_k_raises_the_storage_loss_coefficient_to_6_point_5_w_per_k;
    #[path = "🧹change-h-tr-wk/🧪️tests/improves-the-transmission-heat-transfer-to-175-w-per-k/🦀️.rs"]
    mod tests_change_h_tr_w_k_improves_the_transmission_heat_transfer_to_175_w_per_k;
    #[path = "🧺change-h-ve-wk/🧪️tests/raises-the-ventilation-heat-transfer-to-125-w-per-k/🦀️.rs"]
    mod tests_change_h_ve_w_k_raises_the_ventilation_heat_transfer_to_125_w_per_k;
    #[path = "🪞change-heat-recovery-eta-min/🧪️tests/raises-the-required-heat-recovery-minimum-to-0-point-625/🦀️.rs"]
    mod tests_change_heat_recovery_eta_min_raises_the_required_heat_recovery_minimum_to_0_point_625;
    #[path = "🪑change-heat-recovery-eta/🧪️tests/raises-the-achieved-heat-recovery-to-0-point-875/🦀️.rs"]
    mod tests_change_heat_recovery_eta_raises_the_achieved_heat_recovery_to_0_point_875;
    #[path = "🛋️change-hr-cp-j-kgk/🧪️tests/corrects-the-air-specific-heat-to-1010-j-per-kgk/🦀️.rs"]
    mod tests_change_hr_cp_j_kgk_corrects_the_air_specific_heat_to_1010_j_per_kgk;
    #[path = "🛏️change-hr-delta-tc/🧪️tests/drops-the-heat-recovery-temperature-lift-to-12-point-5-c/🦀️.rs"]
    mod tests_change_hr_delta_t_c_drops_the_heat_recovery_temperature_lift_to_12_point_5_c;
    #[path = "🚿change-hr-m-dot-kg-s/🧪️tests/raises-the-heat-recovery-mass-flow-to-0-point-75-kg-per-second/🦀️.rs"]
    mod tests_change_hr_m_dot_kg_s_raises_the_heat_recovery_mass_flow_to_0_point_75_kg_per_second;
    #[path = "🛁change-hr-savings-reference-kwh/🧪️tests/raises-the-heat-recovery-savings-reference-to-65-kwh/🦀️.rs"]
    mod tests_change_hr_savings_reference_kwh_raises_the_heat_recovery_savings_reference_to_65_kwh;
    #[path = "🌿change-hr-th/🧪️tests/extends-the-heat-recovery-operating-hours-to-14/🦀️.rs"]
    mod tests_change_hr_t_h_extends_the_heat_recovery_operating_hours_to_14;
    #[path = "🍀change-humidification-provided-kg-h/🧪️tests/drops-the-provided-humidification-to-1-point-25-kg-per-hour/🦀️.rs"]
    mod tests_change_humidification_provided_kg_h_drops_the_provided_humidification_to_1_point_25_kg_per_hour;
    #[path = "🌾change-humidification-required-kg-h/🧪️tests/raises-the-required-humidification-to-3-point-5-kg-per-hour/🦀️.rs"]
    mod tests_change_humidification_required_kg_h_raises_the_required_humidification_to_3_point_5_kg_per_hour;
    #[path = "🌵change-ida-class/🧪️tests/relaxes-the-indoor-air-class-to-ida-3/🦀️.rs"]
    mod tests_change_ida_class_relaxes_the_indoor_air_class_to_ida_3;
    #[path = "🌴change-infiltration-allowance-m3-h/🧪️tests/raises-the-infiltration-allowance-to-52-point-5-m3-per-hour/🦀️.rs"]
    mod tests_change_infiltration_allowance_m3_h_raises_the_infiltration_allowance_to_52_point_5_m3_per_hour;
    #[path = "🌳change-l-aeq-db/🧪️tests/raises-the-equivalent-sound-level-to-30-db/🦀️.rs"]
    mod tests_change_l_aeq_db_raises_the_equivalent_sound_level_to_30_db;
    #[path = "🌲change-n50-h-inv/🧪️tests/loosens-the-blower-door-result-to-2-point-5-per-hour/🦀️.rs"]
    mod tests_change_n50_h_inv_loosens_the_blower_door_result_to_2_point_5_per_hour;
    #[path = "🍁change-night-setback-k/🧪️tests/deepens-the-night-setback-to-5-kelvin/🦀️.rs"]
    mod tests_change_night_setback_k_deepens_the_night_setback_to_5_kelvin;
    #[path = "🍂change-occupancy/🧪️tests/reclassifies-the-space-as-office/🦀️.rs"]
    mod tests_change_occupancy_reclassifies_the_space_as_office;
    #[path = "🍃change-occupants/🧪️tests/raises-the-household-to-five-occupants/🦀️.rs"]
    mod tests_change_occupants_raises_the_household_to_five_occupants;
    #[path = "🌱change-persons/🧪️tests/raises-the-design-occupancy-to-16-people/🦀️.rs"]
    mod tests_change_persons_raises_the_design_occupancy_to_16_people;
    #[path = "🌷change-qc-kwh/🧪️tests/raises-the-annual-cooling-demand-to-1250-kwh/🦀️.rs"]
    mod tests_change_q_c_kwh_raises_the_annual_cooling_demand_to_1250_kwh;
    #[path = "🌸change-residential-ventilation-m3-h/🧪️tests/raises-the-residential-airflow-to-110-m3-per-hour/🦀️.rs"]
    mod tests_change_residential_ventilation_m3_h_raises_the_residential_airflow_to_110_m3_per_hour;
    #[path = "🌹change-rh-percent/🧪️tests/drops-indoor-humidity-to-42-point-5-percent/🦀️.rs"]
    mod tests_change_rh_percent_drops_indoor_humidity_to_42_point_5_percent;
    #[path = "🌺change-sfp-required-class/🧪️tests/tightens-the-required-sfp-class-to-3/🦀️.rs"]
    mod tests_change_sfp_required_class_tightens_the_required_sfp_class_to_3;
    #[path = "🌻change-sfp-wm3-s/🧪️tests/improves-the-specific-fan-power-to-1250-w-per-m3-s/🦀️.rs"]
    mod tests_change_sfp_w_m3_s_improves_the_specific_fan_power_to_1250_w_per_m3_s;
    #[path = "🌼change-storage-allowance-kwh/🧪️tests/tightens-the-storage-loss-allowance-to-4-point-5-kwh/🦀️.rs"]
    mod tests_change_storage_allowance_kwh_tightens_the_storage_loss_allowance_to_4_point_5_kwh;
    #[path = "🍄change-storage-th/🧪️tests/shortens-the-storage-standby-period-to-18-hours/🦀️.rs"]
    mod tests_change_storage_t_h_shortens_the_storage_standby_period_to_18_hours;
    #[path = "🌰change-system-type/🧪️tests/switches-to-a-decentral-mechanical-system/🦀️.rs"]
    mod tests_change_system_type_switches_to_a_decentral_mechanical_system;
    #[path = "🌊change-t-op-c/🧪️tests/raises-the-operative-temperature-to-24-point-5-c/🦀️.rs"]
    mod tests_change_t_op_c_raises_the_operative_temperature_to_24_point_5_c;
    #[path = "🐚change-theta-amb-c/🧪️tests/lowers-the-storage-room-ambient-to-18-c/🦀️.rs"]
    mod tests_change_theta_amb_c_lowers_the_storage_room_ambient_to_18_c;
    #[path = "🪨change-theta-ec/🧪️tests/raises-the-external-design-temperature-to-34-point-5-c/🦀️.rs"]
    mod tests_change_theta_e_c_raises_the_external_design_temperature_to_34_point_5_c;
    #[path = "🌍️change-theta-rm-c/🧪️tests/raises-the-running-mean-outdoor-temperature-to-18-point-5-c/🦀️.rs"]
    mod tests_change_theta_rm_c_raises_the_running_mean_outdoor_temperature_to_18_point_5_c;
    #[path = "🌎️change-theta-set-c/🧪️tests/lowers-the-cooling-set-point-to-25-c/🦀️.rs"]
    mod tests_change_theta_set_c_lowers_the_cooling_set_point_to_25_c;
    #[path = "🌏️change-theta-st-c/🧪️tests/lowers-the-storage-temperature-to-55-c/🦀️.rs"]
    mod tests_change_theta_st_c_lowers_the_storage_temperature_to_55_c;
    #[path = "🌐change-ventilation-m3-h/🧪️tests/raises-the-supply-airflow-to-360-m3-per-hour/🦀️.rs"]
    mod tests_change_ventilation_m3_h_raises_the_supply_airflow_to_360_m3_per_hour;
    #[path = "🗻change-volume-m3/🧪️tests/grows-the-air-volume-to-640-m3/🦀️.rs"]
    mod tests_change_volume_m3_grows_the_air_volume_to_640_m3;
    #[path = "🏔️change-years-since-inspection/🧪️tests/ages-the-last-inspection-to-six-years/🦀️.rs"]
    mod tests_change_years_since_inspection_ages_the_last_inspection_to_six_years;
}
//#endregion 🧪️FixtureTests


//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`Din16798Mutation`]. The generated test host of
/// `../../../../../🧪️tests/mutate-din16798-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din16798_mutation_json(text: &str) -> Result<Din16798Mutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
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
/// `mutate-din16798-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_din16798_mutation(mutation: &Din16798Mutation, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
    /// committed `din16798-1-any` catalog. The framework never parses Rust, so this is the only thing
    /// standing between a renamed variant and a completeness gate that silently measures the wrong
    /// set.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Din16798Mutation as protocol::SemanticMutation<Din16798Snapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Din16798Mutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🔣️oracle.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsCatalog
