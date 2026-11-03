//! 🧾 Document outline for EN 1993 steel structure.

use semio_s_artifact_norm_en1993_model::En1993Snapshot;

const SECTION_FIELDS: &[&str] = &[
    "annex",
    "reliabilityClass",
    "materials",
    "sections",
    "members",
    "loadCases",
    "memberActions",
    "joints",
    "fatigueDetails",
    "fireExposures",
    "coldFormedMembers",
    "platedPanels",
    "siloShells",
    "tensionComponents",
    "bridgeFatigue",
    "towerLegs",
    "piles",
    "craneRunways",
    "storeys",
    "restraints",
    "builtUpMembers",
    "weldGroups",
    "parametricSections",
    "hollowJoints",
    "jointRows",
    "frameNodes",
    "frameBars",
    "placements",
];

/// 🧾️ `En1993` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1993Outline {
    pub fn compute(snapshot: &En1993Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (
            snapshot.materials.len()
                + snapshot.sections.len()
                + snapshot.members.len()
                + snapshot.load_cases.len()
                + snapshot.member_actions.len()
                + snapshot.joints.len()
                + snapshot.fatigue_details.len()
                + snapshot.fire_exposures.len()
                + snapshot.cold_formed_members.len()
                + snapshot.plated_panels.len()
                + snapshot.silo_shells.len()
                + snapshot.tension_components.len()
                + snapshot.bridge_fatigue.len()
                + snapshot.tower_legs.len()
                + snapshot.piles.len()
                + snapshot.crane_runways.len()
                + snapshot.storeys.len()
                + snapshot.restraints.len()
                + snapshot.built_up_members.len()
                + snapshot.weld_groups.len()
                + snapshot.parametric_sections.len()
                + snapshot.hollow_joints.len()
                + snapshot.joint_rows.len()
                + snapshot.frame_nodes.len()
                + snapshot.frame_bars.len()
                + snapshot.joint_plates.len()
                + snapshot.foundation_blocks.len()
                + snapshot.material_ductility.len()
                + snapshot.forming_gains.len()
                + snapshot.buckling_lengths.len()
                + snapshot.fatigue_stresses.len()
                + snapshot.frame_member_bases.len()
                + snapshot.fastened_joints.len()
                + snapshot.frame_actions.len()
                + snapshot.placements.len()
        ) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1993Outline {
    fn default() -> Self {
        Self::compute(&En1993Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
