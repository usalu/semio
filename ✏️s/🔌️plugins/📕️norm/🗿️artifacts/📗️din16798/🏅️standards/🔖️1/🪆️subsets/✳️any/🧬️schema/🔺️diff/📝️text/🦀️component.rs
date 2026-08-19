//! 🔺️ Din16798 artifact — sparse field diff runtime.

use crate::artifacts::din16798::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::din16798::schema::Din16798Artifact;
use crate::artifacts::din16798::Din16798Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Din16798Diff {
    pub async fn apply_to_artifact(&self, artifact: &Din16798Artifact) -> protocol::MutationApplyResult<Din16798Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            if let Some(value) = &self.occupancy {
                next.occupancy = value.clone();
            }
            if let Some(value) = &self.comfort_category {
                next.comfort_category = value.clone();
            }
            if let Some(value) = &self.t_op_c {
                next.t_op_c = value.clone();
            }
            if let Some(value) = &self.rh_percent {
                next.rh_percent = value.clone();
            }
            if let Some(value) = &self.air_speed_m_s {
                next.air_speed_m_s = value.clone();
            }
            if let Some(value) = &self.theta_rm_c {
                next.theta_rm_c = value.clone();
            }
            if let Some(value) = &self.co2_ppm {
                next.co2_ppm = value.clone();
            }
            if let Some(value) = &self.df_percent {
                next.df_percent = value.clone();
            }
            if let Some(value) = &self.l_aeq_db {
                next.l_aeq_db = value.clone();
            }
            if let Some(value) = &self.persons {
                next.persons = value.clone();
            }
            if let Some(value) = &self.ida_class {
                next.ida_class = value.clone();
            }
            if let Some(value) = &self.ventilation_m3_h {
                next.ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.floor_area_m2 {
                next.floor_area_m2 = value.clone();
            }
            if let Some(value) = &self.bedrooms {
                next.bedrooms = value.clone();
            }
            if let Some(value) = &self.dwelling_ventilation_m3_h {
                next.dwelling_ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.occupants {
                next.occupants = value.clone();
            }
            if let Some(value) = &self.residential_ventilation_m3_h {
                next.residential_ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.sfp_w_m3_s {
                next.sfp_w_m3_s = value.clone();
            }
            if let Some(value) = &self.sfp_required_class {
                next.sfp_required_class = value.clone();
            }
            if let Some(value) = &self.heat_recovery_eta {
                next.heat_recovery_eta = value.clone();
            }
            if let Some(value) = &self.heat_recovery_eta_min {
                next.heat_recovery_eta_min = value.clone();
            }
            if let Some(value) = &self.system_type {
                next.system_type = value.clone();
            }
            if let Some(value) = &self.years_since_inspection {
                next.years_since_inspection = value.clone();
            }
            if let Some(value) = &self.humidification_required_kg_h {
                next.humidification_required_kg_h = value.clone();
            }
            if let Some(value) = &self.humidification_provided_kg_h {
                next.humidification_provided_kg_h = value.clone();
            }
            if let Some(value) = &self.fan_q_v_m3_s {
                next.fan_q_v_m3_s = value.clone();
            }
            if let Some(value) = &self.fan_t_run_h {
                next.fan_t_run_h = value.clone();
            }
            if let Some(value) = &self.fan_energy_reference_kwh {
                next.fan_energy_reference_kwh = value.clone();
            }
            if let Some(value) = &self.night_setback_k {
                next.night_setback_k = value.clone();
            }
            if let Some(value) = &self.hr_m_dot_kg_s {
                next.hr_m_dot_kg_s = value.clone();
            }
            if let Some(value) = &self.hr_cp_j_kgk {
                next.hr_cp_j_kgk = value.clone();
            }
            if let Some(value) = &self.hr_delta_t_c {
                next.hr_delta_t_c = value.clone();
            }
            if let Some(value) = &self.hr_t_h {
                next.hr_t_h = value.clone();
            }
            if let Some(value) = &self.hr_savings_reference_kwh {
                next.hr_savings_reference_kwh = value.clone();
            }
            if let Some(value) = &self.n50_h_inv {
                next.n50_h_inv = value.clone();
            }
            if let Some(value) = &self.volume_m3 {
                next.volume_m3 = value.clone();
            }
            if let Some(value) = &self.infiltration_allowance_m3_h {
                next.infiltration_allowance_m3_h = value.clone();
            }
            if let Some(value) = &self.cellar_area_m2 {
                next.cellar_area_m2 = value.clone();
            }
            if let Some(value) = &self.cellar_ventilation_m3_h {
                next.cellar_ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.h_tr_w_k {
                next.h_tr_w_k = value.clone();
            }
            if let Some(value) = &self.h_ve_w_k {
                next.h_ve_w_k = value.clone();
            }
            if let Some(value) = &self.theta_e_c {
                next.theta_e_c = value.clone();
            }
            if let Some(value) = &self.theta_set_c {
                next.theta_set_c = value.clone();
            }
            if let Some(value) = &self.cooling_delta_t_h {
                next.cooling_delta_t_h = value.clone();
            }
            if let Some(value) = &self.cooling_gains_kwh {
                next.cooling_gains_kwh = value.clone();
            }
            if let Some(value) = &self.cooling_utilization_factor {
                next.cooling_utilization_factor = value.clone();
            }
            if let Some(value) = &self.cooling_reference_kwh {
                next.cooling_reference_kwh = value.clone();
            }
            if let Some(value) = &self.chiller_type {
                next.chiller_type = value.clone();
            }
            if let Some(value) = &self.eer_actual {
                next.eer_actual = value.clone();
            }
            if let Some(value) = &self.q_c_kwh {
                next.q_c_kwh = value.clone();
            }
            if let Some(value) = &self.generation_reference_kwh {
                next.generation_reference_kwh = value.clone();
            }
            if let Some(value) = &self.data_center_supply_c {
                next.data_center_supply_c = value.clone();
            }
            if let Some(value) = &self.h_st_w_k {
                next.h_st_w_k = value.clone();
            }
            if let Some(value) = &self.theta_st_c {
                next.theta_st_c = value.clone();
            }
            if let Some(value) = &self.theta_amb_c {
                next.theta_amb_c = value.clone();
            }
            if let Some(value) = &self.storage_t_h {
                next.storage_t_h = value.clone();
            }
            if let Some(value) = &self.storage_allowance_kwh {
                next.storage_allowance_kwh = value.clone();
            }
            if let Some(value) = &self.dhw_delivery_c {
                next.dhw_delivery_c = value.clone();
            }
            if let Some(value) = &self.duct_class {
                next.duct_class = value.clone();
            }
            if let Some(value) = &self.duct_test_pressure_pa {
                next.duct_test_pressure_pa = value.clone();
            }
            if let Some(value) = &self.duct_leakage_m3_s_m2 {
                next.duct_leakage_m3_s_m2 = value.clone();
            }
            if let Some(value) = &self.selected_check_index {
                next.selected_check_index = *value;
            }
            next
        })
    }
}

impl MutationDiff<Din16798Snapshot> for Din16798Diff {
    async fn apply(&self, snapshot: &Din16798Snapshot) -> protocol::MutationApplyResult<Din16798Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            if let Some(value) = &self.occupancy {
                next.occupancy = value.clone();
            }
            if let Some(value) = &self.comfort_category {
                next.comfort_category = value.clone();
            }
            if let Some(value) = &self.t_op_c {
                next.t_op_c = value.clone();
            }
            if let Some(value) = &self.rh_percent {
                next.rh_percent = value.clone();
            }
            if let Some(value) = &self.air_speed_m_s {
                next.air_speed_m_s = value.clone();
            }
            if let Some(value) = &self.theta_rm_c {
                next.theta_rm_c = value.clone();
            }
            if let Some(value) = &self.co2_ppm {
                next.co2_ppm = value.clone();
            }
            if let Some(value) = &self.df_percent {
                next.df_percent = value.clone();
            }
            if let Some(value) = &self.l_aeq_db {
                next.l_aeq_db = value.clone();
            }
            if let Some(value) = &self.persons {
                next.persons = value.clone();
            }
            if let Some(value) = &self.ida_class {
                next.ida_class = value.clone();
            }
            if let Some(value) = &self.ventilation_m3_h {
                next.ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.floor_area_m2 {
                next.floor_area_m2 = value.clone();
            }
            if let Some(value) = &self.bedrooms {
                next.bedrooms = value.clone();
            }
            if let Some(value) = &self.dwelling_ventilation_m3_h {
                next.dwelling_ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.occupants {
                next.occupants = value.clone();
            }
            if let Some(value) = &self.residential_ventilation_m3_h {
                next.residential_ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.sfp_w_m3_s {
                next.sfp_w_m3_s = value.clone();
            }
            if let Some(value) = &self.sfp_required_class {
                next.sfp_required_class = value.clone();
            }
            if let Some(value) = &self.heat_recovery_eta {
                next.heat_recovery_eta = value.clone();
            }
            if let Some(value) = &self.heat_recovery_eta_min {
                next.heat_recovery_eta_min = value.clone();
            }
            if let Some(value) = &self.system_type {
                next.system_type = value.clone();
            }
            if let Some(value) = &self.years_since_inspection {
                next.years_since_inspection = value.clone();
            }
            if let Some(value) = &self.humidification_required_kg_h {
                next.humidification_required_kg_h = value.clone();
            }
            if let Some(value) = &self.humidification_provided_kg_h {
                next.humidification_provided_kg_h = value.clone();
            }
            if let Some(value) = &self.fan_q_v_m3_s {
                next.fan_q_v_m3_s = value.clone();
            }
            if let Some(value) = &self.fan_t_run_h {
                next.fan_t_run_h = value.clone();
            }
            if let Some(value) = &self.fan_energy_reference_kwh {
                next.fan_energy_reference_kwh = value.clone();
            }
            if let Some(value) = &self.night_setback_k {
                next.night_setback_k = value.clone();
            }
            if let Some(value) = &self.hr_m_dot_kg_s {
                next.hr_m_dot_kg_s = value.clone();
            }
            if let Some(value) = &self.hr_cp_j_kgk {
                next.hr_cp_j_kgk = value.clone();
            }
            if let Some(value) = &self.hr_delta_t_c {
                next.hr_delta_t_c = value.clone();
            }
            if let Some(value) = &self.hr_t_h {
                next.hr_t_h = value.clone();
            }
            if let Some(value) = &self.hr_savings_reference_kwh {
                next.hr_savings_reference_kwh = value.clone();
            }
            if let Some(value) = &self.n50_h_inv {
                next.n50_h_inv = value.clone();
            }
            if let Some(value) = &self.volume_m3 {
                next.volume_m3 = value.clone();
            }
            if let Some(value) = &self.infiltration_allowance_m3_h {
                next.infiltration_allowance_m3_h = value.clone();
            }
            if let Some(value) = &self.cellar_area_m2 {
                next.cellar_area_m2 = value.clone();
            }
            if let Some(value) = &self.cellar_ventilation_m3_h {
                next.cellar_ventilation_m3_h = value.clone();
            }
            if let Some(value) = &self.h_tr_w_k {
                next.h_tr_w_k = value.clone();
            }
            if let Some(value) = &self.h_ve_w_k {
                next.h_ve_w_k = value.clone();
            }
            if let Some(value) = &self.theta_e_c {
                next.theta_e_c = value.clone();
            }
            if let Some(value) = &self.theta_set_c {
                next.theta_set_c = value.clone();
            }
            if let Some(value) = &self.cooling_delta_t_h {
                next.cooling_delta_t_h = value.clone();
            }
            if let Some(value) = &self.cooling_gains_kwh {
                next.cooling_gains_kwh = value.clone();
            }
            if let Some(value) = &self.cooling_utilization_factor {
                next.cooling_utilization_factor = value.clone();
            }
            if let Some(value) = &self.cooling_reference_kwh {
                next.cooling_reference_kwh = value.clone();
            }
            if let Some(value) = &self.chiller_type {
                next.chiller_type = value.clone();
            }
            if let Some(value) = &self.eer_actual {
                next.eer_actual = value.clone();
            }
            if let Some(value) = &self.q_c_kwh {
                next.q_c_kwh = value.clone();
            }
            if let Some(value) = &self.generation_reference_kwh {
                next.generation_reference_kwh = value.clone();
            }
            if let Some(value) = &self.data_center_supply_c {
                next.data_center_supply_c = value.clone();
            }
            if let Some(value) = &self.h_st_w_k {
                next.h_st_w_k = value.clone();
            }
            if let Some(value) = &self.theta_st_c {
                next.theta_st_c = value.clone();
            }
            if let Some(value) = &self.theta_amb_c {
                next.theta_amb_c = value.clone();
            }
            if let Some(value) = &self.storage_t_h {
                next.storage_t_h = value.clone();
            }
            if let Some(value) = &self.storage_allowance_kwh {
                next.storage_allowance_kwh = value.clone();
            }
            if let Some(value) = &self.dhw_delivery_c {
                next.dhw_delivery_c = value.clone();
            }
            if let Some(value) = &self.duct_class {
                next.duct_class = value.clone();
            }
            if let Some(value) = &self.duct_test_pressure_pa {
                next.duct_test_pressure_pa = value.clone();
            }
            if let Some(value) = &self.duct_leakage_m3_s_m2 {
                next.duct_leakage_m3_s_m2 = value.clone();
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(annex);
        take!(occupancy);
        take!(comfort_category);
        take!(t_op_c);
        take!(rh_percent);
        take!(air_speed_m_s);
        take!(theta_rm_c);
        take!(co2_ppm);
        take!(df_percent);
        take!(l_aeq_db);
        take!(persons);
        take!(ida_class);
        take!(ventilation_m3_h);
        take!(floor_area_m2);
        take!(bedrooms);
        take!(dwelling_ventilation_m3_h);
        take!(occupants);
        take!(residential_ventilation_m3_h);
        take!(sfp_w_m3_s);
        take!(sfp_required_class);
        take!(heat_recovery_eta);
        take!(heat_recovery_eta_min);
        take!(system_type);
        take!(years_since_inspection);
        take!(humidification_required_kg_h);
        take!(humidification_provided_kg_h);
        take!(fan_q_v_m3_s);
        take!(fan_t_run_h);
        take!(fan_energy_reference_kwh);
        take!(night_setback_k);
        take!(hr_m_dot_kg_s);
        take!(hr_cp_j_kgk);
        take!(hr_delta_t_c);
        take!(hr_t_h);
        take!(hr_savings_reference_kwh);
        take!(n50_h_inv);
        take!(volume_m3);
        take!(infiltration_allowance_m3_h);
        take!(cellar_area_m2);
        take!(cellar_ventilation_m3_h);
        take!(h_tr_w_k);
        take!(h_ve_w_k);
        take!(theta_e_c);
        take!(theta_set_c);
        take!(cooling_delta_t_h);
        take!(cooling_gains_kwh);
        take!(cooling_utilization_factor);
        take!(cooling_reference_kwh);
        take!(chiller_type);
        take!(eer_actual);
        take!(q_c_kwh);
        take!(generation_reference_kwh);
        take!(data_center_supply_c);
        take!(h_st_w_k);
        take!(theta_st_c);
        take!(theta_amb_c);
        take!(storage_t_h);
        take!(storage_allowance_kwh);
        take!(dhw_delivery_c);
        take!(duct_class);
        take!(duct_test_pressure_pa);
        take!(duct_leakage_m3_s_m2);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply
