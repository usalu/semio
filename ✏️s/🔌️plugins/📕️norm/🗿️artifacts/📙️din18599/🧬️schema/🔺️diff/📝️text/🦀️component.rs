//! 🔺️ Din18599 artifact — sparse field diff runtime.

use crate::artifacts::din18599::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::din18599::schema::Din18599Artifact;
use crate::artifacts::din18599::Din18599Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Din18599Diff {
    pub fn apply_to_artifact(&self, artifact: &Din18599Artifact) -> Din18599Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.use_class { next.use_class = value.clone(); }
        if let Some(value) = &self.heated_area_m2 { next.heated_area_m2 = value.clone(); }
        if let Some(value) = &self.occupants { next.occupants = value.clone(); }
        if let Some(value) = &self.h_t { next.h_t = value.clone(); }
        if let Some(value) = &self.h_v { next.h_v = value.clone(); }
        if let Some(value) = &self.climate { next.climate = value.clone(); }
        if let Some(value) = &self.internal_gains_w_m2 { next.internal_gains_w_m2 = value.clone(); }
        if let Some(value) = &self.solar_gains_kwh { next.solar_gains_kwh = value.clone(); }
        if let Some(value) = &self.system_losses_kwh { next.system_losses_kwh = value.clone(); }
        if let Some(value) = &self.renewable_kwh { next.renewable_kwh = value.clone(); }
        if let Some(value) = &self.annual_limit_kwh { next.annual_limit_kwh = value.clone(); }
        if let Some(value) = &self.energy_carrier { next.energy_carrier = value.clone(); }
        if let Some(value) = &self.reference_q_p_kwh { next.reference_q_p_kwh = value.clone(); }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<Din18599Snapshot> for Din18599Diff {
    fn apply(&self, snapshot: &Din18599Snapshot) -> Din18599Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.use_class { next.use_class = value.clone(); }
        if let Some(value) = &self.heated_area_m2 { next.heated_area_m2 = value.clone(); }
        if let Some(value) = &self.occupants { next.occupants = value.clone(); }
        if let Some(value) = &self.h_t { next.h_t = value.clone(); }
        if let Some(value) = &self.h_v { next.h_v = value.clone(); }
        if let Some(value) = &self.climate { next.climate = value.clone(); }
        if let Some(value) = &self.internal_gains_w_m2 { next.internal_gains_w_m2 = value.clone(); }
        if let Some(value) = &self.solar_gains_kwh { next.solar_gains_kwh = value.clone(); }
        if let Some(value) = &self.system_losses_kwh { next.system_losses_kwh = value.clone(); }
        if let Some(value) = &self.renewable_kwh { next.renewable_kwh = value.clone(); }
        if let Some(value) = &self.annual_limit_kwh { next.annual_limit_kwh = value.clone(); }
        if let Some(value) = &self.energy_carrier { next.energy_carrier = value.clone(); }
        if let Some(value) = &self.reference_q_p_kwh { next.reference_q_p_kwh = value.clone(); }
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
        take!(use_class);
        take!(heated_area_m2);
        take!(occupants);
        take!(h_t);
        take!(h_v);
        take!(climate);
        take!(internal_gains_w_m2);
        take!(solar_gains_kwh);
        take!(system_losses_kwh);
        take!(renewable_kwh);
        take!(annual_limit_kwh);
        take!(energy_carrier);
        take!(reference_q_p_kwh);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff {
        artifact: Some(Box::new(Din18599Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers
