//! 🧬️ En1993 diff schema — sparse field delta over the hierarchical steel subject.

use crate::{
    BridgeFatigue, ColdFormedMember, CraneRunway, FatigueDetail, FireExposure, LoadCase, MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial,
    SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1993 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
