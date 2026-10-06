//! 🧬️ En1993 diff schema — sparse field delta over the hierarchical steel subject.

use crate::{
    BridgeFatigue, ColdFormedMember, CraneRunway, FatigueDetail, FireExposure, LoadCase, MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial,
    SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1993 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1993Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub materials: Option<En1993MaterialList>,
    #[state(artifact)]
    pub sections: Option<En1993SectionList>,
    #[state(artifact)]
    pub members: Option<En1993MemberList>,
    #[state(artifact)]
    pub load_cases: Option<En1993LoadCaseList>,
    #[state(artifact)]
    pub member_actions: Option<En1993MemberActionList>,
    #[state(artifact)]
    pub joints: Option<En1993JointList>,
    #[state(artifact)]
    pub fatigue_details: Option<En1993FatigueList>,
    #[state(artifact)]
    pub fire_exposures: Option<En1993FireList>,
    #[state(artifact)]
    pub cold_formed_members: Option<En1993ColdFormedList>,
    #[state(artifact)]
    pub plated_panels: Option<En1993PlatedList>,
    #[state(artifact)]
    pub silo_shells: Option<En1993SiloList>,
    #[state(artifact)]
    pub tension_components: Option<En1993TensionList>,
    #[state(artifact)]
    pub bridge_fatigue: Option<En1993BridgeList>,
    #[state(artifact)]
    pub tower_legs: Option<En1993TowerList>,
    #[state(artifact)]
    pub piles: Option<En1993PileList>,
    #[state(artifact)]
    pub crane_runways: Option<En1993CraneList>,
}

macro_rules! list_wrap {
    ($name:ident, $ty:ty) => {
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name {
            pub values: Vec<$ty>,
        }
    };
}

list_wrap!(En1993MaterialList, SteelMaterial);
list_wrap!(En1993SectionList, SteelSection);
list_wrap!(En1993MemberList, SteelMember);
list_wrap!(En1993LoadCaseList, LoadCase);
list_wrap!(En1993MemberActionList, MemberAction);
list_wrap!(En1993JointList, SteelJoint);
list_wrap!(En1993FatigueList, FatigueDetail);
list_wrap!(En1993FireList, FireExposure);
list_wrap!(En1993ColdFormedList, ColdFormedMember);
list_wrap!(En1993PlatedList, PlatedPanel);
list_wrap!(En1993SiloList, SiloShell);
list_wrap!(En1993TensionList, TensionComponent);
list_wrap!(En1993BridgeList, BridgeFatigue);
list_wrap!(En1993TowerList, TowerLeg);
list_wrap!(En1993PileList, SteelPile);
list_wrap!(En1993CraneList, CraneRunway);

//#endregion 🔖️Diff

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1993Artifact;
use crate::En1993Snapshot;
use protocol::MutationDiff;

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
