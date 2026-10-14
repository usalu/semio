//! 🧬️ En1993 artifact schema — hierarchical steel-structure subject + compliance helpers.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_model::{BridgeFatigue, ColdFormedMember, CraneRunway, FatigueDetail, FireExposure, LoadCase, MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial, SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg};
use semio_s_plugin_norm_compliance::AnnexChoice;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full En1993 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub materials: Vec<SteelMaterial>,
    #[state(artifact)]
    pub sections: Vec<SteelSection>,
    #[state(artifact)]
    pub members: Vec<SteelMember>,
    #[state(artifact)]
    pub load_cases: Vec<LoadCase>,
    #[state(artifact)]
    pub member_actions: Vec<MemberAction>,
    #[state(artifact)]
    pub joints: Vec<SteelJoint>,
    #[state(artifact)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[state(artifact)]
    pub fire_exposures: Vec<FireExposure>,
    #[state(artifact)]
    pub cold_formed_members: Vec<ColdFormedMember>,
    #[state(artifact)]
    pub plated_panels: Vec<PlatedPanel>,
    #[state(artifact)]
    pub silo_shells: Vec<SiloShell>,
    #[state(artifact)]
    pub tension_components: Vec<TensionComponent>,
    #[state(artifact)]
    pub bridge_fatigue: Vec<BridgeFatigue>,
    #[state(artifact)]
    pub tower_legs: Vec<TowerLeg>,
    #[state(artifact)]
    pub piles: Vec<SteelPile>,
    #[state(artifact)]
    pub crane_runways: Vec<CraneRunway>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1993Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> En1993Snapshot {
        En1993Snapshot {
            annex: self.annex,
            materials: self.materials.clone(),
            sections: self.sections.clone(),
            members: self.members.clone(),
            load_cases: self.load_cases.clone(),
            member_actions: self.member_actions.clone(),
            joints: self.joints.clone(),
            fatigue_details: self.fatigue_details.clone(),
            fire_exposures: self.fire_exposures.clone(),
            cold_formed_members: self.cold_formed_members.clone(),
            plated_panels: self.plated_panels.clone(),
            silo_shells: self.silo_shells.clone(),
            tension_components: self.tension_components.clone(),
            bridge_fatigue: self.bridge_fatigue.clone(),
            tower_legs: self.tower_legs.clone(),
            piles: self.piles.clone(),
            crane_runways: self.crane_runways.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: En1993Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            materials: snapshot.materials,
            sections: snapshot.sections,
            members: snapshot.members,
            load_cases: snapshot.load_cases,
            member_actions: snapshot.member_actions,
            joints: snapshot.joints,
            fatigue_details: snapshot.fatigue_details,
            fire_exposures: snapshot.fire_exposures,
            cold_formed_members: snapshot.cold_formed_members,
            plated_panels: snapshot.plated_panels,
            silo_shells: snapshot.silo_shells,
            tension_components: snapshot.tension_components,
            bridge_fatigue: snapshot.bridge_fatigue,
            tower_legs: snapshot.tower_legs,
            piles: snapshot.piles,
            crane_runways: snapshot.crane_runways,
        }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1993` — twenty handcrafted schema leaves.
pub fn en1993_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1993",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1993_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1993_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1993_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

