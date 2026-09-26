//! 🔺️ En1993 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1993Artifact;
use crate::En1993Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1993Diff {
    pub fn apply_to_artifact(&self, artifact: &En1993Artifact) -> protocol::MutationApplyResult<En1993Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.annex {
            next.annex = *value;
        }
        if let Some(list) = &self.materials {
            next.materials = list.values.clone();
        }
        if let Some(list) = &self.sections {
            next.sections = list.values.clone();
        }
        if let Some(list) = &self.members {
            next.members = list.values.clone();
        }
        if let Some(list) = &self.load_cases {
            next.load_cases = list.values.clone();
        }
        if let Some(list) = &self.member_actions {
            next.member_actions = list.values.clone();
        }
        if let Some(list) = &self.joints {
            next.joints = list.values.clone();
        }
        if let Some(list) = &self.fatigue_details {
            next.fatigue_details = list.values.clone();
        }
        if let Some(list) = &self.fire_exposures {
            next.fire_exposures = list.values.clone();
        }
        if let Some(list) = &self.cold_formed_members {
            next.cold_formed_members = list.values.clone();
        }
        if let Some(list) = &self.plated_panels {
            next.plated_panels = list.values.clone();
        }
        if let Some(list) = &self.silo_shells {
            next.silo_shells = list.values.clone();
        }
        if let Some(list) = &self.tension_components {
            next.tension_components = list.values.clone();
        }
        if let Some(list) = &self.bridge_fatigue {
            next.bridge_fatigue = list.values.clone();
        }
        if let Some(list) = &self.tower_legs {
            next.tower_legs = list.values.clone();
        }
        if let Some(list) = &self.piles {
            next.piles = list.values.clone();
        }
        if let Some(list) = &self.crane_runways {
            next.crane_runways = list.values.clone();
        }
        Ok(next)
    }
}

impl MutationDiff<En1993Snapshot> for En1993Diff {
    fn apply(&self, snapshot: &En1993Snapshot) -> protocol::MutationApplyResult<En1993Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.annex {
            next.annex = *value;
        }
        if let Some(list) = &self.materials {
            next.materials = list.values.clone();
        }
        if let Some(list) = &self.sections {
            next.sections = list.values.clone();
        }
        if let Some(list) = &self.members {
            next.members = list.values.clone();
        }
        if let Some(list) = &self.load_cases {
            next.load_cases = list.values.clone();
        }
        if let Some(list) = &self.member_actions {
            next.member_actions = list.values.clone();
        }
        if let Some(list) = &self.joints {
            next.joints = list.values.clone();
        }
        if let Some(list) = &self.fatigue_details {
            next.fatigue_details = list.values.clone();
        }
        if let Some(list) = &self.fire_exposures {
            next.fire_exposures = list.values.clone();
        }
        if let Some(list) = &self.cold_formed_members {
            next.cold_formed_members = list.values.clone();
        }
        if let Some(list) = &self.plated_panels {
            next.plated_panels = list.values.clone();
        }
        if let Some(list) = &self.silo_shells {
            next.silo_shells = list.values.clone();
        }
        if let Some(list) = &self.tension_components {
            next.tension_components = list.values.clone();
        }
        if let Some(list) = &self.bridge_fatigue {
            next.bridge_fatigue = list.values.clone();
        }
        if let Some(list) = &self.tower_legs {
            next.tower_legs = list.values.clone();
        }
        if let Some(list) = &self.piles {
            next.piles = list.values.clone();
        }
        if let Some(list) = &self.crane_runways {
            next.crane_runways = list.values.clone();
        }
        Ok(next)
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
        take!(annex);
        take!(materials);
        take!(sections);
        take!(members);
        take!(load_cases);
        take!(member_actions);
        take!(joints);
        take!(fatigue_details);
        take!(fire_exposures);
        take!(cold_formed_members);
        take!(plated_panels);
        take!(silo_shells);
        take!(tension_components);
        take!(bridge_fatigue);
        take!(tower_legs);
        take!(piles);
        take!(crane_runways);
    }
}
//#endregion 🔖️Apply

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type En1993DiffText = String;
//#endregion 🚚️Carrier
