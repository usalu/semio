//! 🧬️ En1993 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use protocol::{ColdDiffAlgebra, DiffAlgebra, MutationDiff};

use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Rows

//#region 🧱️RowGroups
pub use semio_s_artifact_norm_en1993_diff_rows_members::*;
pub use semio_s_artifact_norm_en1993_diff_rows_joints::*;
pub use semio_s_artifact_norm_en1993_diff_rows_fatigue_fire::*;
pub use semio_s_artifact_norm_en1993_diff_rows_thin_walled::*;
pub use semio_s_artifact_norm_en1993_diff_rows_structures::*;
pub use semio_s_artifact_norm_en1993_diff_rows_frame::*;
//#endregion 🧱️RowGroups

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1993 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm-eurocode-structural.en1993")]
pub struct En1993Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub materials: En1993MaterialDelta,
    #[state(artifact)]
    pub sections: En1993SectionDelta,
    #[state(artifact)]
    pub members: En1993MemberDelta,
    #[state(artifact)]
    pub load_cases: En1993LoadCaseDelta,
    #[state(artifact)]
    pub member_actions: En1993MemberActionDelta,
    #[state(artifact)]
    pub joints: En1993JointDelta,
    #[state(artifact)]
    pub fatigue_details: En1993FatigueDetailDelta,
    #[state(artifact)]
    pub fire_exposures: En1993FireExposureDelta,
    #[state(artifact)]
    pub cold_formed_members: En1993ColdFormedMemberDelta,
    #[state(artifact)]
    pub plated_panels: En1993PlatedPanelDelta,
    #[state(artifact)]
    pub silo_shells: En1993SiloShellDelta,
    #[state(artifact)]
    pub tension_components: En1993TensionComponentDelta,
    #[state(artifact)]
    pub bridge_fatigue: En1993BridgeFatigueDelta,
    #[state(artifact)]
    pub tower_legs: En1993TowerLegDelta,
    #[state(artifact)]
    pub piles: En1993PileDelta,
    #[state(artifact)]
    pub crane_runways: En1993CraneRunwayDelta,
    #[state(artifact)]
    pub storeys: En1993StoreyDelta,
    #[state(artifact)]
    pub restraints: En1993RestraintDelta,
    #[state(artifact)]
    pub built_up_members: En1993BuiltUpMemberDelta,
    #[state(artifact)]
    pub weld_groups: En1993WeldGroupDelta,
    #[state(artifact)]
    pub parametric_sections: En1993ParametricSectionDelta,
    #[state(artifact)]
    pub hollow_joints: En1993HollowJointDelta,
    #[state(artifact)]
    pub joint_rows: En1993JointRowDelta,
    #[state(artifact)]
    pub frame_nodes: En1993FrameNodeDelta,
    #[state(artifact)]
    pub frame_bars: En1993FrameBarDelta,
    #[state(artifact)]
    pub placements: En1993PlacementDelta,
    #[state(artifact)]
    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,
    #[state(artifact)]
    pub joint_plates: En1993JointPlateDelta,
    #[state(artifact)]
    pub foundation_blocks: En1993FoundationBlockDelta,
    #[state(artifact)]
    pub material_ductility: En1993MaterialDuctilityDelta,
    #[state(artifact)]
    pub forming_gains: En1993FormingGainDelta,
    #[state(artifact)]
    pub buckling_lengths: En1993BucklingLengthDelta,
    #[state(artifact)]
    pub fatigue_stresses: En1993FatigueStressDelta,
    #[state(artifact)]
    pub frame_actions: En1993FrameActionDelta,
    #[state(artifact)]
    pub frame_member_bases: En1993FrameMemberBasisDelta,
    #[state(artifact)]
    pub fastened_joints: En1993FastenedJointDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1993Snapshot> for En1993Diff {}

impl protocol::ColdMutationDiff<En1993Snapshot> for En1993Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
        }
        self.materials.absorb(other.materials);
        self.sections.absorb(other.sections);
        self.members.absorb(other.members);
        self.load_cases.absorb(other.load_cases);
        self.member_actions.absorb(other.member_actions);
        self.joints.absorb(other.joints);
        self.fatigue_details.absorb(other.fatigue_details);
        self.fire_exposures.absorb(other.fire_exposures);
        self.cold_formed_members.absorb(other.cold_formed_members);
        self.plated_panels.absorb(other.plated_panels);
        self.silo_shells.absorb(other.silo_shells);
        self.tension_components.absorb(other.tension_components);
        self.bridge_fatigue.absorb(other.bridge_fatigue);
        self.tower_legs.absorb(other.tower_legs);
        self.piles.absorb(other.piles);
        self.crane_runways.absorb(other.crane_runways);
        self.storeys.absorb(other.storeys);
        self.restraints.absorb(other.restraints);
        self.built_up_members.absorb(other.built_up_members);
        self.weld_groups.absorb(other.weld_groups);
        self.parametric_sections.absorb(other.parametric_sections);
        self.hollow_joints.absorb(other.hollow_joints);
        self.joint_rows.absorb(other.joint_rows);
        self.frame_nodes.absorb(other.frame_nodes);
        self.frame_bars.absorb(other.frame_bars);
        self.placements.absorb(other.placements);
        self.joint_plates.absorb(other.joint_plates);
        self.foundation_blocks.absorb(other.foundation_blocks);
        self.material_ductility.absorb(other.material_ductility);
        self.forming_gains.absorb(other.forming_gains);
        self.buckling_lengths.absorb(other.buckling_lengths);
        self.fatigue_stresses.absorb(other.fatigue_stresses);
        self.frame_member_bases.absorb(other.frame_member_bases);
        self.fastened_joints.absorb(other.fastened_joints);
        self.frame_actions.absorb(other.frame_actions);
    }

    fn apply(&self, unpublished: &mut En1993Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = self.annex {
            next.annex = value;
        }
        if let Some(value) = self.reliability_class {
            next.reliability_class = value;
        }
        next.materials = self.materials.commit_onto(&base.materials, capability).map_err(|error| error.under(["materials"]))?;
        next.sections = self.sections.commit_onto(&base.sections, capability).map_err(|error| error.under(["sections"]))?;
        next.members = self.members.commit_onto(&base.members, capability).map_err(|error| error.under(["members"]))?;
        next.load_cases = self.load_cases.commit_onto(&base.load_cases, capability).map_err(|error| error.under(["loadCases"]))?;
        next.member_actions = self.member_actions.commit_onto(&base.member_actions, capability).map_err(|error| error.under(["memberActions"]))?;
        next.joints = self.joints.commit_onto(&base.joints, capability).map_err(|error| error.under(["joints"]))?;
        next.fatigue_details = self.fatigue_details.commit_onto(&base.fatigue_details, capability).map_err(|error| error.under(["fatigueDetails"]))?;
        next.fire_exposures = self.fire_exposures.commit_onto(&base.fire_exposures, capability).map_err(|error| error.under(["fireExposures"]))?;
        next.cold_formed_members = self.cold_formed_members.commit_onto(&base.cold_formed_members, capability).map_err(|error| error.under(["coldFormedMembers"]))?;
        next.plated_panels = self.plated_panels.commit_onto(&base.plated_panels, capability).map_err(|error| error.under(["platedPanels"]))?;
        next.silo_shells = self.silo_shells.commit_onto(&base.silo_shells, capability).map_err(|error| error.under(["siloShells"]))?;
        next.tension_components = self.tension_components.commit_onto(&base.tension_components, capability).map_err(|error| error.under(["tensionComponents"]))?;
        next.bridge_fatigue = self.bridge_fatigue.commit_onto(&base.bridge_fatigue, capability).map_err(|error| error.under(["bridgeFatigue"]))?;
        next.tower_legs = self.tower_legs.commit_onto(&base.tower_legs, capability).map_err(|error| error.under(["towerLegs"]))?;
        next.piles = self.piles.commit_onto(&base.piles, capability).map_err(|error| error.under(["piles"]))?;
        next.crane_runways = self.crane_runways.commit_onto(&base.crane_runways, capability).map_err(|error| error.under(["craneRunways"]))?;
        next.storeys = self.storeys.commit_onto(&base.storeys, capability).map_err(|error| error.under(["storeys"]))?;
        next.restraints = self.restraints.commit_onto(&base.restraints, capability).map_err(|error| error.under(["restraints"]))?;
        next.built_up_members = self.built_up_members.commit_onto(&base.built_up_members, capability).map_err(|error| error.under(["builtUpMembers"]))?;
        next.weld_groups = self.weld_groups.commit_onto(&base.weld_groups, capability).map_err(|error| error.under(["weldGroups"]))?;
        next.parametric_sections = self.parametric_sections.commit_onto(&base.parametric_sections, capability).map_err(|error| error.under(["parametricSections"]))?;
        next.hollow_joints = self.hollow_joints.commit_onto(&base.hollow_joints, capability).map_err(|error| error.under(["hollowJoints"]))?;
        next.joint_rows = self.joint_rows.commit_onto(&base.joint_rows, capability).map_err(|error| error.under(["jointRows"]))?;
        next.frame_nodes = self.frame_nodes.commit_onto(&base.frame_nodes, capability).map_err(|error| error.under(["frameNodes"]))?;
        next.frame_bars = self.frame_bars.commit_onto(&base.frame_bars, capability).map_err(|error| error.under(["frameBars"]))?;
        next.placements = self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;
        next.joint_plates = self.joint_plates.commit_onto(&base.joint_plates, capability).map_err(|error| error.under(["jointPlates"]))?;
        next.foundation_blocks = self.foundation_blocks.commit_onto(&base.foundation_blocks, capability).map_err(|error| error.under(["foundationBlocks"]))?;
        next.material_ductility = self.material_ductility.commit_onto(&base.material_ductility, capability).map_err(|error| error.under(["materialDuctility"]))?;
        next.forming_gains = self.forming_gains.commit_onto(&base.forming_gains, capability).map_err(|error| error.under(["formingGains"]))?;
        next.buckling_lengths = self.buckling_lengths.commit_onto(&base.buckling_lengths, capability).map_err(|error| error.under(["bucklingLengths"]))?;
        next.fatigue_stresses = self.fatigue_stresses.commit_onto(&base.fatigue_stresses, capability).map_err(|error| error.under(["fatigueStresses"]))?;
        next.frame_member_bases = self.frame_member_bases.commit_onto(&base.frame_member_bases, capability).map_err(|error| error.under(["frameMemberBases"]))?;
        next.fastened_joints = self.fastened_joints.commit_onto(&base.fastened_joints, capability).map_err(|error| error.under(["fastenedJoints"]))?;
        next.frame_actions = self.frame_actions.commit_onto(&base.frame_actions, capability).map_err(|error| error.under(["frameActions"]))?;
        *unpublished = next;
        Ok(())
    }
}

impl DiffAlgebra<En1993Snapshot> for En1993Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.reliability_class.is_none()
            && self.materials.is_empty()
            && self.sections.is_empty()
            && self.members.is_empty()
            && self.load_cases.is_empty()
            && self.member_actions.is_empty()
            && self.joints.is_empty()
            && self.fatigue_details.is_empty()
            && self.fire_exposures.is_empty()
            && self.cold_formed_members.is_empty()
            && self.plated_panels.is_empty()
            && self.silo_shells.is_empty()
            && self.tension_components.is_empty()
            && self.bridge_fatigue.is_empty()
            && self.tower_legs.is_empty()
            && self.piles.is_empty()
            && self.crane_runways.is_empty()
            && self.storeys.is_empty()
            && self.restraints.is_empty()
            && self.built_up_members.is_empty()
            && self.weld_groups.is_empty()
            && self.parametric_sections.is_empty()
            && self.hollow_joints.is_empty()
            && self.joint_rows.is_empty()
            && self.frame_nodes.is_empty()
            && self.frame_bars.is_empty()
            && self.placements.is_empty()
            && self.joint_plates.is_empty()
            && self.foundation_blocks.is_empty()
            && self.material_ductility.is_empty()
            && self.forming_gains.is_empty()
            && self.buckling_lengths.is_empty()
            && self.fatigue_stresses.is_empty()
            && self.frame_member_bases.is_empty()
            && self.fastened_joints.is_empty()
            && self.frame_actions.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1993Snapshot> for En1993Diff {
    fn inverse(&self, base: &En1993Snapshot) -> Self {
        Self {
            annex: self.annex.map(|_| base.annex),
            reliability_class: self.reliability_class.map(|_| base.reliability_class),
            materials: self.materials.inverse(&base.materials),
            sections: self.sections.inverse(&base.sections),
            members: self.members.inverse(&base.members),
            load_cases: self.load_cases.inverse(&base.load_cases),
            member_actions: self.member_actions.inverse(&base.member_actions),
            joints: self.joints.inverse(&base.joints),
            fatigue_details: self.fatigue_details.inverse(&base.fatigue_details),
            fire_exposures: self.fire_exposures.inverse(&base.fire_exposures),
            cold_formed_members: self.cold_formed_members.inverse(&base.cold_formed_members),
            plated_panels: self.plated_panels.inverse(&base.plated_panels),
            silo_shells: self.silo_shells.inverse(&base.silo_shells),
            tension_components: self.tension_components.inverse(&base.tension_components),
            bridge_fatigue: self.bridge_fatigue.inverse(&base.bridge_fatigue),
            tower_legs: self.tower_legs.inverse(&base.tower_legs),
            piles: self.piles.inverse(&base.piles),
            crane_runways: self.crane_runways.inverse(&base.crane_runways),
            storeys: self.storeys.inverse(&base.storeys),
            restraints: self.restraints.inverse(&base.restraints),
            built_up_members: self.built_up_members.inverse(&base.built_up_members),
            weld_groups: self.weld_groups.inverse(&base.weld_groups),
            parametric_sections: self.parametric_sections.inverse(&base.parametric_sections),
            hollow_joints: self.hollow_joints.inverse(&base.hollow_joints),
            joint_rows: self.joint_rows.inverse(&base.joint_rows),
            frame_nodes: self.frame_nodes.inverse(&base.frame_nodes),
            frame_bars: self.frame_bars.inverse(&base.frame_bars),
            placements: self.placements.inverse(&base.placements),
            joint_plates: self.joint_plates.inverse(&base.joint_plates),
            foundation_blocks: self.foundation_blocks.inverse(&base.foundation_blocks),
            material_ductility: self.material_ductility.inverse(&base.material_ductility),
            forming_gains: self.forming_gains.inverse(&base.forming_gains),
            buckling_lengths: self.buckling_lengths.inverse(&base.buckling_lengths),
            fatigue_stresses: self.fatigue_stresses.inverse(&base.fatigue_stresses),
            frame_member_bases: self.frame_member_bases.inverse(&base.frame_member_bases),
            fastened_joints: self.fastened_joints.inverse(&base.fastened_joints),
            frame_actions: self.frame_actions.inverse(&base.frame_actions),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1993_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1993_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1993_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1993_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves

#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
pub mod binary;
