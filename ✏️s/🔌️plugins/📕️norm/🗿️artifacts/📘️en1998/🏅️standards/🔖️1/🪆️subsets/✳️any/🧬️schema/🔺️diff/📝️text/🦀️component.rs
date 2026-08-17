//! 🔺️ En1998 artifact — sparse field diff runtime.

use crate::artifacts::en1998::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::en1998::schema::En1998Artifact;
use crate::artifacts::en1998::En1998Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1998Diff {
    pub fn apply_to_artifact(&self, artifact: &En1998Artifact) -> protocol::MutationApplyResult<En1998Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.seismic_zone {
                next.seismic_zone = value.clone();
            }
            if let Some(value) = &self.ground_type {
                next.ground_type = value.clone();
            }
            if let Some(value) = &self.importance_class {
                next.importance_class = value.clone();
            }
            if let Some(value) = &self.structural_system {
                next.structural_system = value.clone();
            }
            if let Some(value) = &self.t1_s {
                next.t1_s = value.clone();
            }
            if let Some(value) = &self.mass_t {
                next.mass_t = value.clone();
            }
            if let Some(value) = &self.v_rd_kn {
                next.v_rd_kn = value.clone();
            }
            if let Some(value) = &self.drift_mm {
                next.drift_mm = value.clone();
            }
            if let Some(value) = &self.height_m {
                next.height_m = value.clone();
            }
            if let Some(value) = &self.multiple_resisting_systems {
                next.multiple_resisting_systems = value.clone();
            }
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            if let Some(value) = &self.en_a_gr {
                next.en_a_gr = value.clone();
            }
            if let Some(value) = &self.en_ground_type {
                next.en_ground_type = value.clone();
            }
            if let Some(value) = &self.en_spectrum_type {
                next.en_spectrum_type = value.clone();
            }
            if let Some(value) = &self.period_ratio {
                next.period_ratio = value.clone();
            }
            if let Some(value) = &self.bridge_v_rd_kn {
                next.bridge_v_rd_kn = value.clone();
            }
            if let Some(value) = &self.bearing_d_ed_mm {
                next.bearing_d_ed_mm = value.clone();
            }
            if let Some(value) = &self.bearing_d_rd_mm {
                next.bearing_d_rd_mm = value.clone();
            }
            if let Some(value) = &self.retrofit_knowledge_level {
                next.retrofit_knowledge_level = value.clone();
            }
            if let Some(value) = &self.retrofit_limit_state {
                next.retrofit_limit_state = value.clone();
            }
            if let Some(value) = &self.retrofit_e_d_kn {
                next.retrofit_e_d_kn = value.clone();
            }
            if let Some(value) = &self.retrofit_r_k_kn {
                next.retrofit_r_k_kn = value.clone();
            }
            if let Some(value) = &self.retrofit_gamma_el {
                next.retrofit_gamma_el = value.clone();
            }
            if let Some(value) = &self.silo_height_m {
                next.silo_height_m = value.clone();
            }
            if let Some(value) = &self.silo_radius_m {
                next.silo_radius_m = value.clone();
            }
            if let Some(value) = &self.silo_n_rd_kn {
                next.silo_n_rd_kn = value.clone();
            }
            if let Some(value) = &self.silo_v_ed_kn {
                next.silo_v_ed_kn = value.clone();
            }
            if let Some(value) = &self.silo_v_rd_kn {
                next.silo_v_rd_kn = value.clone();
            }
            if let Some(value) = &self.silo_q_nominal {
                next.silo_q_nominal = value.clone();
            }
            if let Some(value) = &self.tank_height_m {
                next.tank_height_m = value.clone();
            }
            if let Some(value) = &self.tank_radius_m {
                next.tank_radius_m = value.clone();
            }
            if let Some(value) = &self.tank_mass_t {
                next.tank_mass_t = value.clone();
            }
            if let Some(value) = &self.tank_v_rd_kn {
                next.tank_v_rd_kn = value.clone();
            }
            if let Some(value) = &self.tower_m_ed_knm {
                next.tower_m_ed_knm = value.clone();
            }
            if let Some(value) = &self.tower_m_rd_knm {
                next.tower_m_rd_knm = value.clone();
            }
            if let Some(value) = &self.tower_is_chimney {
                next.tower_is_chimney = value.clone();
            }
            if let Some(value) = &self.tower_q_nominal {
                next.tower_q_nominal = value.clone();
            }
            if let Some(value) = &self.tower_mass_t {
                next.tower_mass_t = value.clone();
            }
            if let Some(value) = &self.foundation_area_m2 {
                next.foundation_area_m2 = value.clone();
            }
            if let Some(value) = &self.foundation_p_rd_kpa {
                next.foundation_p_rd_kpa = value.clone();
            }
            if let Some(value) = &self.foundation_h_ed_kn {
                next.foundation_h_ed_kn = value.clone();
            }
            if let Some(value) = &self.foundation_h_rd_kn {
                next.foundation_h_rd_kn = value.clone();
            }
            if let Some(value) = &self.k_foundation {
                next.k_foundation = value.clone();
            }
            if let Some(value) = &self.k_soil {
                next.k_soil = value.clone();
            }
            if let Some(value) = &self.wall_height_m {
                next.wall_height_m = value.clone();
            }
            if let Some(value) = &self.wall_phi_deg {
                next.wall_phi_deg = value.clone();
            }
            if let Some(value) = &self.wall_soil_gamma_kn_m3 {
                next.wall_soil_gamma_kn_m3 = value.clone();
            }
            if let Some(value) = &self.wall_r {
                next.wall_r = value.clone();
            }
            if let Some(value) = &self.wall_h_rd_kn {
                next.wall_h_rd_kn = value.clone();
            }
            if let Some(value) = &self.selected_check_index {
                next.selected_check_index = *value;
            }
            next
        })
    }
}

impl MutationDiff<En1998Snapshot> for En1998Diff {
    fn apply(&self, snapshot: &En1998Snapshot) -> protocol::MutationApplyResult<En1998Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.seismic_zone {
                next.seismic_zone = value.clone();
            }
            if let Some(value) = &self.ground_type {
                next.ground_type = value.clone();
            }
            if let Some(value) = &self.importance_class {
                next.importance_class = value.clone();
            }
            if let Some(value) = &self.structural_system {
                next.structural_system = value.clone();
            }
            if let Some(value) = &self.t1_s {
                next.t1_s = value.clone();
            }
            if let Some(value) = &self.mass_t {
                next.mass_t = value.clone();
            }
            if let Some(value) = &self.v_rd_kn {
                next.v_rd_kn = value.clone();
            }
            if let Some(value) = &self.drift_mm {
                next.drift_mm = value.clone();
            }
            if let Some(value) = &self.height_m {
                next.height_m = value.clone();
            }
            if let Some(value) = &self.multiple_resisting_systems {
                next.multiple_resisting_systems = value.clone();
            }
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            if let Some(value) = &self.en_a_gr {
                next.en_a_gr = value.clone();
            }
            if let Some(value) = &self.en_ground_type {
                next.en_ground_type = value.clone();
            }
            if let Some(value) = &self.en_spectrum_type {
                next.en_spectrum_type = value.clone();
            }
            if let Some(value) = &self.period_ratio {
                next.period_ratio = value.clone();
            }
            if let Some(value) = &self.bridge_v_rd_kn {
                next.bridge_v_rd_kn = value.clone();
            }
            if let Some(value) = &self.bearing_d_ed_mm {
                next.bearing_d_ed_mm = value.clone();
            }
            if let Some(value) = &self.bearing_d_rd_mm {
                next.bearing_d_rd_mm = value.clone();
            }
            if let Some(value) = &self.retrofit_knowledge_level {
                next.retrofit_knowledge_level = value.clone();
            }
            if let Some(value) = &self.retrofit_limit_state {
                next.retrofit_limit_state = value.clone();
            }
            if let Some(value) = &self.retrofit_e_d_kn {
                next.retrofit_e_d_kn = value.clone();
            }
            if let Some(value) = &self.retrofit_r_k_kn {
                next.retrofit_r_k_kn = value.clone();
            }
            if let Some(value) = &self.retrofit_gamma_el {
                next.retrofit_gamma_el = value.clone();
            }
            if let Some(value) = &self.silo_height_m {
                next.silo_height_m = value.clone();
            }
            if let Some(value) = &self.silo_radius_m {
                next.silo_radius_m = value.clone();
            }
            if let Some(value) = &self.silo_n_rd_kn {
                next.silo_n_rd_kn = value.clone();
            }
            if let Some(value) = &self.silo_v_ed_kn {
                next.silo_v_ed_kn = value.clone();
            }
            if let Some(value) = &self.silo_v_rd_kn {
                next.silo_v_rd_kn = value.clone();
            }
            if let Some(value) = &self.silo_q_nominal {
                next.silo_q_nominal = value.clone();
            }
            if let Some(value) = &self.tank_height_m {
                next.tank_height_m = value.clone();
            }
            if let Some(value) = &self.tank_radius_m {
                next.tank_radius_m = value.clone();
            }
            if let Some(value) = &self.tank_mass_t {
                next.tank_mass_t = value.clone();
            }
            if let Some(value) = &self.tank_v_rd_kn {
                next.tank_v_rd_kn = value.clone();
            }
            if let Some(value) = &self.tower_m_ed_knm {
                next.tower_m_ed_knm = value.clone();
            }
            if let Some(value) = &self.tower_m_rd_knm {
                next.tower_m_rd_knm = value.clone();
            }
            if let Some(value) = &self.tower_is_chimney {
                next.tower_is_chimney = value.clone();
            }
            if let Some(value) = &self.tower_q_nominal {
                next.tower_q_nominal = value.clone();
            }
            if let Some(value) = &self.tower_mass_t {
                next.tower_mass_t = value.clone();
            }
            if let Some(value) = &self.foundation_area_m2 {
                next.foundation_area_m2 = value.clone();
            }
            if let Some(value) = &self.foundation_p_rd_kpa {
                next.foundation_p_rd_kpa = value.clone();
            }
            if let Some(value) = &self.foundation_h_ed_kn {
                next.foundation_h_ed_kn = value.clone();
            }
            if let Some(value) = &self.foundation_h_rd_kn {
                next.foundation_h_rd_kn = value.clone();
            }
            if let Some(value) = &self.k_foundation {
                next.k_foundation = value.clone();
            }
            if let Some(value) = &self.k_soil {
                next.k_soil = value.clone();
            }
            if let Some(value) = &self.wall_height_m {
                next.wall_height_m = value.clone();
            }
            if let Some(value) = &self.wall_phi_deg {
                next.wall_phi_deg = value.clone();
            }
            if let Some(value) = &self.wall_soil_gamma_kn_m3 {
                next.wall_soil_gamma_kn_m3 = value.clone();
            }
            if let Some(value) = &self.wall_r {
                next.wall_r = value.clone();
            }
            if let Some(value) = &self.wall_h_rd_kn {
                next.wall_h_rd_kn = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
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
        take!(seismic_zone);
        take!(ground_type);
        take!(importance_class);
        take!(structural_system);
        take!(t1_s);
        take!(mass_t);
        take!(v_rd_kn);
        take!(drift_mm);
        take!(height_m);
        take!(multiple_resisting_systems);
        take!(annex);
        take!(en_a_gr);
        take!(en_ground_type);
        take!(en_spectrum_type);
        take!(period_ratio);
        take!(bridge_v_rd_kn);
        take!(bearing_d_ed_mm);
        take!(bearing_d_rd_mm);
        take!(retrofit_knowledge_level);
        take!(retrofit_limit_state);
        take!(retrofit_e_d_kn);
        take!(retrofit_r_k_kn);
        take!(retrofit_gamma_el);
        take!(silo_height_m);
        take!(silo_radius_m);
        take!(silo_n_rd_kn);
        take!(silo_v_ed_kn);
        take!(silo_v_rd_kn);
        take!(silo_q_nominal);
        take!(tank_height_m);
        take!(tank_radius_m);
        take!(tank_mass_t);
        take!(tank_v_rd_kn);
        take!(tower_m_ed_knm);
        take!(tower_m_rd_knm);
        take!(tower_is_chimney);
        take!(tower_q_nominal);
        take!(tower_mass_t);
        take!(foundation_area_m2);
        take!(foundation_p_rd_kpa);
        take!(foundation_h_ed_kn);
        take!(foundation_h_rd_kn);
        take!(k_foundation);
        take!(k_soil);
        take!(wall_height_m);
        take!(wall_phi_deg);
        take!(wall_soil_gamma_kn_m3);
        take!(wall_r);
        take!(wall_h_rd_kn);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1998Snapshot) -> En1998Diff {
    En1998Diff { artifact: Some(Box::new(En1998Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1998::mutations::En1998Mutation;
    use protocol::{Mutation as _, MutationDiff};

    #[test]
    fn change_mutation_diff_updates_only_its_field() {
        let base = En1998Snapshot::default();
        let mutation = En1998Mutation::ChangeSeismicZone(crate::artifacts::en1998::mutations::change_seismic_zone::mutation::ChangeSeismicZone { new_seismic_zone: 3 });
        let outcome = mutation.diff(&base);
        let mut expected = base.clone();
        expected.seismic_zone = 3;
        assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
    }
}
//#endregion 🧪️Tests
