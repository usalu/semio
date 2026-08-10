//! 🧬️ Din16798 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din16798 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::din16798::schema::Din16798Artifact>>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub occupancy: Option<String>,
    #[state(persistent)] pub comfort_category: Option<String>,
    #[state(persistent)] pub t_op_c: Option<f64>,
    #[state(persistent)] pub rh_percent: Option<f64>,
    #[state(persistent)] pub air_speed_m_s: Option<f64>,
    #[state(persistent)] pub theta_rm_c: Option<f64>,
    #[state(persistent)] pub co2_ppm: Option<f64>,
    #[state(persistent)] pub df_percent: Option<f64>,
    #[state(persistent)] pub l_aeq_db: Option<f64>,
    #[state(persistent)] pub persons: Option<u32>,
    #[state(persistent)] pub ida_class: Option<String>,
    #[state(persistent)] pub ventilation_m3_h: Option<f64>,
    #[state(persistent)] pub floor_area_m2: Option<f64>,
    #[state(persistent)] pub bedrooms: Option<u32>,
    #[state(persistent)] pub dwelling_ventilation_m3_h: Option<f64>,
    #[state(persistent)] pub occupants: Option<u32>,
    #[state(persistent)] pub residential_ventilation_m3_h: Option<f64>,
    #[state(persistent)] pub sfp_w_m3_s: Option<f64>,
    #[state(persistent)] pub sfp_required_class: Option<u8>,
    #[state(persistent)] pub heat_recovery_eta: Option<f64>,
    #[state(persistent)] pub heat_recovery_eta_min: Option<f64>,
    #[state(persistent)] pub system_type: Option<String>,
    #[state(persistent)] pub years_since_inspection: Option<u32>,
    #[state(persistent)] pub humidification_required_kg_h: Option<f64>,
    #[state(persistent)] pub humidification_provided_kg_h: Option<f64>,
    #[state(persistent)] pub fan_q_v_m3_s: Option<f64>,
    #[state(persistent)] pub fan_t_run_h: Option<f64>,
    #[state(persistent)] pub fan_energy_reference_kwh: Option<f64>,
    #[state(persistent)] pub night_setback_k: Option<f64>,
    #[state(persistent)] pub hr_m_dot_kg_s: Option<f64>,
    #[state(persistent)] pub hr_cp_j_kgk: Option<f64>,
    #[state(persistent)] pub hr_delta_t_c: Option<f64>,
    #[state(persistent)] pub hr_t_h: Option<f64>,
    #[state(persistent)] pub hr_savings_reference_kwh: Option<f64>,
    #[state(persistent)] pub n50_h_inv: Option<f64>,
    #[state(persistent)] pub volume_m3: Option<f64>,
    #[state(persistent)] pub infiltration_allowance_m3_h: Option<f64>,
    #[state(persistent)] pub cellar_area_m2: Option<f64>,
    #[state(persistent)] pub cellar_ventilation_m3_h: Option<f64>,
    #[state(persistent)] pub h_tr_w_k: Option<f64>,
    #[state(persistent)] pub h_ve_w_k: Option<f64>,
    #[state(persistent)] pub theta_e_c: Option<f64>,
    #[state(persistent)] pub theta_set_c: Option<f64>,
    #[state(persistent)] pub cooling_delta_t_h: Option<f64>,
    #[state(persistent)] pub cooling_gains_kwh: Option<f64>,
    #[state(persistent)] pub cooling_utilization_factor: Option<f64>,
    #[state(persistent)] pub cooling_reference_kwh: Option<f64>,
    #[state(persistent)] pub chiller_type: Option<String>,
    #[state(persistent)] pub eer_actual: Option<f64>,
    #[state(persistent)] pub q_c_kwh: Option<f64>,
    #[state(persistent)] pub generation_reference_kwh: Option<f64>,
    #[state(persistent)] pub data_center_supply_c: Option<f64>,
    #[state(persistent)] pub h_st_w_k: Option<f64>,
    #[state(persistent)] pub theta_st_c: Option<f64>,
    #[state(persistent)] pub theta_amb_c: Option<f64>,
    #[state(persistent)] pub storage_t_h: Option<f64>,
    #[state(persistent)] pub storage_allowance_kwh: Option<f64>,
    #[state(persistent)] pub dhw_delivery_c: Option<f64>,
    #[state(persistent)] pub duct_class: Option<String>,
    #[state(persistent)] pub duct_test_pressure_pa: Option<f64>,
    #[state(persistent)] pub duct_leakage_m3_s_m2: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Din16798StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
