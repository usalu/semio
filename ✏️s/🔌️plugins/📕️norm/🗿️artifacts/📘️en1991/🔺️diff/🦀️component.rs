//! 🔺️ En1991 artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::en1991::schema::En1991Artifact;
use crate::artifacts::en1991::En1991Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1991Diff {
    pub fn apply_to_artifact(&self, artifact: &En1991Artifact) -> En1991Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = self.area_m2 { next.area_m2 = value; }
        if let Some(value) = self.category { next.category = value; }
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = &self.self_weight_material { next.self_weight_material = value.clone(); }
        if let Some(value) = self.self_weight_thickness_m { next.self_weight_thickness_m = value; }
        if let Some(value) = self.assumed_g_k_kn_m2 { next.assumed_g_k_kn_m2 = value; }
        if let Some(value) = self.fire_curve { next.fire_curve = value; }
        if let Some(value) = self.fire_resistance_min { next.fire_resistance_min = value; }
        if let Some(value) = self.fire_member_capacity_c { next.fire_member_capacity_c = value; }
        if let Some(value) = self.snow_zone { next.snow_zone = value; }
        if let Some(value) = self.snow_altitude_m { next.snow_altitude_m = value; }
        if let Some(value) = self.en_s_k_kn_m2 { next.en_s_k_kn_m2 = value; }
        if let Some(value) = self.wind_zone { next.wind_zone = value; }
        if let Some(value) = self.en_v_b_m_s { next.en_v_b_m_s = value; }
        if let Some(value) = self.delta_t_k { next.delta_t_k = value; }
        if let Some(value) = &self.construction_activity { next.construction_activity = value.clone(); }
        if let Some(value) = self.accidental_mass_t { next.accidental_mass_t = value; }
        if let Some(value) = self.accidental_speed_km_h { next.accidental_speed_km_h = value; }
        if let Some(value) = self.bridge_lane { next.bridge_lane = value; }
        if let Some(value) = self.bridge_span_m { next.bridge_span_m = value; }
        if let Some(value) = self.bridge_lane_width_m { next.bridge_lane_width_m = value; }
        if let Some(value) = self.bridge_moment_resistance_knm { next.bridge_moment_resistance_knm = value; }
        if let Some(value) = &self.crane_class { next.crane_class = value.clone(); }
        if let Some(value) = &self.hoist_class { next.hoist_class = value.clone(); }
        if let Some(value) = self.hoisting_speed_m_s { next.hoisting_speed_m_s = value; }
        if let Some(value) = self.silo_bulk_density_kn_m3 { next.silo_bulk_density_kn_m3 = value; }
        if let Some(value) = self.silo_height_m { next.silo_height_m = value; }
        if let Some(value) = self.silo_hydraulic_radius_m { next.silo_hydraulic_radius_m = value; }
        if let Some(value) = self.silo_mu { next.silo_mu = value; }
        if let Some(value) = self.silo_k { next.silo_k = value; }
        if let Some(value) = self.c_s { next.c_s = value; }
        if let Some(value) = self.c_d { next.c_d = value; }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<En1991Snapshot> for En1991Diff {
    fn apply(&self, snapshot: &En1991Snapshot) -> En1991Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = self.area_m2 { next.area_m2 = value; }
        if let Some(value) = self.category { next.category = value; }
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = &self.self_weight_material { next.self_weight_material = value.clone(); }
        if let Some(value) = self.self_weight_thickness_m { next.self_weight_thickness_m = value; }
        if let Some(value) = self.assumed_g_k_kn_m2 { next.assumed_g_k_kn_m2 = value; }
        if let Some(value) = self.fire_curve { next.fire_curve = value; }
        if let Some(value) = self.fire_resistance_min { next.fire_resistance_min = value; }
        if let Some(value) = self.fire_member_capacity_c { next.fire_member_capacity_c = value; }
        if let Some(value) = self.snow_zone { next.snow_zone = value; }
        if let Some(value) = self.snow_altitude_m { next.snow_altitude_m = value; }
        if let Some(value) = self.en_s_k_kn_m2 { next.en_s_k_kn_m2 = value; }
        if let Some(value) = self.wind_zone { next.wind_zone = value; }
        if let Some(value) = self.en_v_b_m_s { next.en_v_b_m_s = value; }
        if let Some(value) = self.delta_t_k { next.delta_t_k = value; }
        if let Some(value) = &self.construction_activity { next.construction_activity = value.clone(); }
        if let Some(value) = self.accidental_mass_t { next.accidental_mass_t = value; }
        if let Some(value) = self.accidental_speed_km_h { next.accidental_speed_km_h = value; }
        if let Some(value) = self.bridge_lane { next.bridge_lane = value; }
        if let Some(value) = self.bridge_span_m { next.bridge_span_m = value; }
        if let Some(value) = self.bridge_lane_width_m { next.bridge_lane_width_m = value; }
        if let Some(value) = self.bridge_moment_resistance_knm { next.bridge_moment_resistance_knm = value; }
        if let Some(value) = &self.crane_class { next.crane_class = value.clone(); }
        if let Some(value) = &self.hoist_class { next.hoist_class = value.clone(); }
        if let Some(value) = self.hoisting_speed_m_s { next.hoisting_speed_m_s = value; }
        if let Some(value) = self.silo_bulk_density_kn_m3 { next.silo_bulk_density_kn_m3 = value; }
        if let Some(value) = self.silo_height_m { next.silo_height_m = value; }
        if let Some(value) = self.silo_hydraulic_radius_m { next.silo_hydraulic_radius_m = value; }
        if let Some(value) = self.silo_mu { next.silo_mu = value; }
        if let Some(value) = self.silo_k { next.silo_k = value; }
        if let Some(value) = self.c_s { next.c_s = value; }
        if let Some(value) = self.c_d { next.c_d = value; }
        next
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
        take!(area_m2);
        take!(category);
        take!(annex);
        take!(self_weight_material);
        take!(self_weight_thickness_m);
        take!(assumed_g_k_kn_m2);
        take!(fire_curve);
        take!(fire_resistance_min);
        take!(fire_member_capacity_c);
        take!(snow_zone);
        take!(snow_altitude_m);
        take!(en_s_k_kn_m2);
        take!(wind_zone);
        take!(en_v_b_m_s);
        take!(delta_t_k);
        take!(construction_activity);
        take!(accidental_mass_t);
        take!(accidental_speed_km_h);
        take!(bridge_lane);
        take!(bridge_span_m);
        take!(bridge_lane_width_m);
        take!(bridge_moment_resistance_knm);
        take!(crane_class);
        take!(hoist_class);
        take!(hoisting_speed_m_s);
        take!(silo_bulk_density_kn_m3);
        take!(silo_height_m);
        take!(silo_hydraulic_radius_m);
        take!(silo_mu);
        take!(silo_k);
        take!(c_s);
        take!(c_d);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1991Snapshot) -> En1991Diff {
    En1991Diff {
        artifact: Some(Box::new(En1991Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers
