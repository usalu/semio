//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "occupancy",
    "comfort_category",
    "t_op_c",
    "rh_percent",
    "air_speed_m_s",
    "theta_rm_c",
    "co2_ppm",
    "df_percent",
    "l_aeq_db",
    "persons",
    "ida_class",
    "ventilation_m3_h",
    "floor_area_m2",
    "bedrooms",
    "dwelling_ventilation_m3_h",
    "occupants",
    "residential_ventilation_m3_h",
    "sfp_w_m3_s",
    "sfp_required_class",
    "heat_recovery_eta",
    "heat_recovery_eta_min",
    "system_type",
    "years_since_inspection",
    "humidification_required_kg_h",
    "humidification_provided_kg_h",
    "fan_q_v_m3_s",
    "fan_t_run_h",
    "fan_energy_reference_kwh",
    "night_setback_k",
    "hr_m_dot_kg_s",
    "hr_cp_j_kgk",
    "hr_delta_t_c",
    "hr_t_h",
    "hr_savings_reference_kwh",
    "n50_h_inv",
    "volume_m3",
    "infiltration_allowance_m3_h",
    "cellar_area_m2",
    "cellar_ventilation_m3_h",
    "h_tr_w_k",
    "h_ve_w_k",
    "theta_e_c",
    "theta_set_c",
    "cooling_delta_t_h",
    "cooling_gains_kwh",
    "cooling_utilization_factor",
    "cooling_reference_kwh",
    "chiller_type",
    "eer_actual",
    "q_c_kwh",
    "generation_reference_kwh",
    "data_center_supply_c",
    "h_st_w_k",
    "theta_st_c",
    "theta_amb_c",
    "storage_t_h",
    "storage_allowance_kwh",
    "dhw_delivery_c",
    "duct_class",
    "duct_test_pressure_pa",
    "duct_leakage_m3_s_m2",
];

/// 🧾️ `Din16798` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Din16798Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Din16798Outline {
    pub fn compute(_snapshot: &Din16798Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Din16798Outline {
    fn default() -> Self {
        Self::compute(&Din16798Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = Din16798Outline::compute(&Din16798Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = Din16798Snapshot::default();
        assert_eq!(Din16798Outline::compute(&snapshot), Din16798Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
