//! 🧬️ En1993 artifact schema — hierarchical steel-structure subject + compliance helpers.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_model::{BridgeFatigue, BucklingLength, BuiltUpMember, ColdFormedMember, CraneRunway, FastenedJoint, FatigueDetail, FatigueStress, FireExposure, FormingGain, FoundationBlock, FrameAction, FrameBar, FrameMemberBasis, FrameNode, HollowJoint, JointPlate, JointRow, LoadCase, MaterialDuctility, MemberAction, ParametricSection, Placement, PlatedPanel, Restraint, SiloShell, SteelJoint, SteelMaterial, SteelMember, SteelPile, SteelSection, Storey, TensionComponent, TowerLeg, WeldGroup};
use semio_s_plugin_norm_compliance::AnnexChoice;
use framework_schema::ArtifactSchema;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Artifact
/// 🧬️ Full En1993 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
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
    #[state(artifact)]
    pub storeys: Vec<Storey>,
    #[state(artifact)]
    pub restraints: Vec<Restraint>,
    #[state(artifact)]
    pub built_up_members: Vec<BuiltUpMember>,
    #[state(artifact)]
    pub weld_groups: Vec<WeldGroup>,
    #[state(artifact)]
    pub parametric_sections: Vec<ParametricSection>,
    #[state(artifact)]
    pub hollow_joints: Vec<HollowJoint>,
    #[state(artifact)]
    pub joint_rows: Vec<JointRow>,
    #[state(artifact)]
    pub frame_nodes: Vec<FrameNode>,
    #[state(artifact)]
    pub frame_bars: Vec<FrameBar>,
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[dsl(table)]
    #[state(artifact)]
    pub joint_plates: Vec<JointPlate>,
    #[dsl(table)]
    #[state(artifact)]
    pub foundation_blocks: Vec<FoundationBlock>,
    #[state(artifact)]
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    #[dsl(table)]
    #[state(artifact)]
    pub frame_actions: Vec<FrameAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub frame_member_bases: Vec<FrameMemberBasis>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_stresses: Vec<FatigueStress>,
    #[dsl(table)]
    #[state(artifact)]
    pub buckling_lengths: Vec<BucklingLength>,
    #[dsl(table)]
    #[state(artifact)]
    pub forming_gains: Vec<FormingGain>,
    #[dsl(table)]
    #[state(artifact)]
    pub material_ductility: Vec<MaterialDuctility>,
    #[dsl(table)]
    #[state(artifact)]
    pub fastened_joints: Vec<FastenedJoint>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1993Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> En1993Snapshot {
        En1993Snapshot {
            annex: self.annex,
            reliability_class: self.reliability_class,
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
            storeys: self.storeys.clone(),
            restraints: self.restraints.clone(),
            built_up_members: self.built_up_members.clone(),
            weld_groups: self.weld_groups.clone(),
            parametric_sections: self.parametric_sections.clone(),
            hollow_joints: self.hollow_joints.clone(),
            joint_rows: self.joint_rows.clone(),
            frame_nodes: self.frame_nodes.clone(),
            frame_bars: self.frame_bars.clone(),
            placements: self.placements.clone(),
            joint_plates: self.joint_plates.clone(),
            foundation_blocks: self.foundation_blocks.clone(),
            material_ductility: self.material_ductility.clone(),
            forming_gains: self.forming_gains.clone(),
            buckling_lengths: self.buckling_lengths.clone(),
            fatigue_stresses: self.fatigue_stresses.clone(),
            frame_member_bases: self.frame_member_bases.clone(),
            fastened_joints: self.fastened_joints.clone(),
            frame_actions: self.frame_actions.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: En1993Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            reliability_class: snapshot.reliability_class,
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
            storeys: snapshot.storeys,
            restraints: snapshot.restraints,
            built_up_members: snapshot.built_up_members,
            weld_groups: snapshot.weld_groups,
            parametric_sections: snapshot.parametric_sections,
            hollow_joints: snapshot.hollow_joints,
            joint_rows: snapshot.joint_rows,
            frame_nodes: snapshot.frame_nodes,
            frame_bars: snapshot.frame_bars,
            placements: snapshot.placements,
            joint_plates: snapshot.joint_plates,
            foundation_blocks: snapshot.foundation_blocks,
            material_ductility: snapshot.material_ductility,
            forming_gains: snapshot.forming_gains,
            buckling_lengths: snapshot.buckling_lengths,
            fatigue_stresses: snapshot.fatigue_stresses,
            frame_member_bases: snapshot.frame_member_bases,
            fastened_joints: snapshot.fastened_joints,
            frame_actions: snapshot.frame_actions,
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

