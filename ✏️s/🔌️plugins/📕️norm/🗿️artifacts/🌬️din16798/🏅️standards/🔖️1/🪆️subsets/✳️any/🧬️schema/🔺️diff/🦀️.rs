//! 🧬️ Din16798 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din16798 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::din16798::schema::Din16798Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub occupancy: Option<String>,
    #[state(artifact)]
    pub comfort_category: Option<String>,
    #[state(artifact)]
    pub t_op_c: Option<f64>,
    #[state(artifact)]
    pub rh_percent: Option<f64>,
    #[state(artifact)]
    pub air_speed_m_s: Option<f64>,
    #[state(artifact)]
    pub theta_rm_c: Option<f64>,
    #[state(artifact)]
    pub co2_ppm: Option<f64>,
    #[state(artifact)]
    pub df_percent: Option<f64>,
    #[state(artifact)]
    pub l_aeq_db: Option<f64>,
    #[state(artifact)]
    pub persons: Option<u32>,
    #[state(artifact)]
    pub ida_class: Option<String>,
    #[state(artifact)]
    pub ventilation_m3_h: Option<f64>,
    #[state(artifact)]
    pub floor_area_m2: Option<f64>,
    #[state(artifact)]
    pub bedrooms: Option<u32>,
    #[state(artifact)]
    pub dwelling_ventilation_m3_h: Option<f64>,
    #[state(artifact)]
    pub occupants: Option<u32>,
    #[state(artifact)]
    pub residential_ventilation_m3_h: Option<f64>,
    #[state(artifact)]
    pub sfp_w_m3_s: Option<f64>,
    #[state(artifact)]
    pub sfp_required_class: Option<u8>,
    #[state(artifact)]
    pub heat_recovery_eta: Option<f64>,
    #[state(artifact)]
    pub heat_recovery_eta_min: Option<f64>,
    #[state(artifact)]
    pub system_type: Option<String>,
    #[state(artifact)]
    pub years_since_inspection: Option<u32>,
    #[state(artifact)]
    pub humidification_required_kg_h: Option<f64>,
    #[state(artifact)]
    pub humidification_provided_kg_h: Option<f64>,
    #[state(artifact)]
    pub fan_q_v_m3_s: Option<f64>,
    #[state(artifact)]
    pub fan_t_run_h: Option<f64>,
    #[state(artifact)]
    pub fan_energy_reference_kwh: Option<f64>,
    #[state(artifact)]
    pub night_setback_k: Option<f64>,
    #[state(artifact)]
    pub hr_m_dot_kg_s: Option<f64>,
    #[state(artifact)]
    pub hr_cp_j_kgk: Option<f64>,
    #[state(artifact)]
    pub hr_delta_t_c: Option<f64>,
    #[state(artifact)]
    pub hr_t_h: Option<f64>,
    #[state(artifact)]
    pub hr_savings_reference_kwh: Option<f64>,
    #[state(artifact)]
    pub n50_h_inv: Option<f64>,
    #[state(artifact)]
    pub volume_m3: Option<f64>,
    #[state(artifact)]
    pub infiltration_allowance_m3_h: Option<f64>,
    #[state(artifact)]
    pub cellar_area_m2: Option<f64>,
    #[state(artifact)]
    pub cellar_ventilation_m3_h: Option<f64>,
    #[state(artifact)]
    pub h_tr_w_k: Option<f64>,
    #[state(artifact)]
    pub h_ve_w_k: Option<f64>,
    #[state(artifact)]
    pub theta_e_c: Option<f64>,
    #[state(artifact)]
    pub theta_set_c: Option<f64>,
    #[state(artifact)]
    pub cooling_delta_t_h: Option<f64>,
    #[state(artifact)]
    pub cooling_gains_kwh: Option<f64>,
    #[state(artifact)]
    pub cooling_utilization_factor: Option<f64>,
    #[state(artifact)]
    pub cooling_reference_kwh: Option<f64>,
    #[state(artifact)]
    pub chiller_type: Option<String>,
    #[state(artifact)]
    pub eer_actual: Option<f64>,
    #[state(artifact)]
    pub q_c_kwh: Option<f64>,
    #[state(artifact)]
    pub generation_reference_kwh: Option<f64>,
    #[state(artifact)]
    pub data_center_supply_c: Option<f64>,
    #[state(artifact)]
    pub h_st_w_k: Option<f64>,
    #[state(artifact)]
    pub theta_st_c: Option<f64>,
    #[state(artifact)]
    pub theta_amb_c: Option<f64>,
    #[state(artifact)]
    pub storage_t_h: Option<f64>,
    #[state(artifact)]
    pub storage_allowance_kwh: Option<f64>,
    #[state(artifact)]
    pub dhw_delivery_c: Option<f64>,
    #[state(artifact)]
    pub duct_class: Option<String>,
    #[state(artifact)]
    pub duct_test_pressure_pa: Option<f64>,
    #[state(artifact)]
    pub duct_leakage_m3_s_m2: Option<f64>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din16798StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
