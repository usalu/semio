//! 🔺️ Din4108 artifact — sparse field diff runtime.

use crate::artifacts::din4108::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::din4108::schema::Din4108Artifact;
use crate::artifacts::din4108::Din4108Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Din4108Diff {
    pub fn apply_to_artifact(&self, artifact: &Din4108Artifact) -> Din4108Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.category {
            next.category = value.clone();
        }
        if let Some(list) = &self.layers {
            next.layers = list.values.clone();
        }
        if let Some(value) = &self.climate {
            next.climate = value.clone();
        }
        if let Some(value) = &self.airtightness_n50 {
            next.airtightness_n50 = value.clone();
        }
        if let Some(value) = &self.psi_times_l_sum {
            next.psi_times_l_sum = value.clone();
        }
        if let Some(value) = &self.rh_int {
            next.rh_int = value.clone();
        }
        if let Some(value) = &self.catalog_id {
            next.catalog_id = value.clone();
        }
        if let Some(value) = &self.material_id {
            next.material_id = value.clone();
        }
        if let Some(value) = &self.airtightness_class {
            next.airtightness_class = value.clone();
        }
        if let Some(value) = &self.t_int_c {
            next.t_int_c = value.clone();
        }
        if let Some(value) = &self.solar_absorptance {
            next.solar_absorptance = value.clone();
        }
        if let Some(value) = &self.irradiance_w_m2 {
            next.irradiance_w_m2 = value.clone();
        }
        if let Some(value) = &self.moisture_mu_exterior {
            next.moisture_mu_exterior = value.clone();
        }
        if let Some(value) = &self.moisture_mu_interior {
            next.moisture_mu_interior = value.clone();
        }
        if let Some(value) = &self.envelope_area_m2 {
            next.envelope_area_m2 = value.clone();
        }
        if let Some(value) = &self.bb2_details_conform {
            next.bb2_details_conform = value.clone();
        }
        if let Some(value) = &self.application_type {
            next.application_type = value.clone();
        }
        if let Some(value) = &self.declared_application_class {
            next.declared_application_class = value.clone();
        }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<Din4108Snapshot> for Din4108Diff {
    fn apply(&self, snapshot: &Din4108Snapshot) -> Din4108Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.category {
            next.category = value.clone();
        }
        if let Some(list) = &self.layers {
            next.layers = list.values.clone();
        }
        if let Some(value) = &self.climate {
            next.climate = value.clone();
        }
        if let Some(value) = &self.airtightness_n50 {
            next.airtightness_n50 = value.clone();
        }
        if let Some(value) = &self.psi_times_l_sum {
            next.psi_times_l_sum = value.clone();
        }
        if let Some(value) = &self.rh_int {
            next.rh_int = value.clone();
        }
        if let Some(value) = &self.catalog_id {
            next.catalog_id = value.clone();
        }
        if let Some(value) = &self.material_id {
            next.material_id = value.clone();
        }
        if let Some(value) = &self.airtightness_class {
            next.airtightness_class = value.clone();
        }
        if let Some(value) = &self.t_int_c {
            next.t_int_c = value.clone();
        }
        if let Some(value) = &self.solar_absorptance {
            next.solar_absorptance = value.clone();
        }
        if let Some(value) = &self.irradiance_w_m2 {
            next.irradiance_w_m2 = value.clone();
        }
        if let Some(value) = &self.moisture_mu_exterior {
            next.moisture_mu_exterior = value.clone();
        }
        if let Some(value) = &self.moisture_mu_interior {
            next.moisture_mu_interior = value.clone();
        }
        if let Some(value) = &self.envelope_area_m2 {
            next.envelope_area_m2 = value.clone();
        }
        if let Some(value) = &self.bb2_details_conform {
            next.bb2_details_conform = value.clone();
        }
        if let Some(value) = &self.application_type {
            next.application_type = value.clone();
        }
        if let Some(value) = &self.declared_application_class {
            next.declared_application_class = value.clone();
        }
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
        take!(category);
        take!(layers);
        take!(climate);
        take!(airtightness_n50);
        take!(psi_times_l_sum);
        take!(rh_int);
        take!(catalog_id);
        take!(material_id);
        take!(airtightness_class);
        take!(t_int_c);
        take!(solar_absorptance);
        take!(irradiance_w_m2);
        take!(moisture_mu_exterior);
        take!(moisture_mu_interior);
        take!(envelope_area_m2);
        take!(bb2_details_conform);
        take!(application_type);
        take!(declared_application_class);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { artifact: Some(Box::new(Din4108Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers
