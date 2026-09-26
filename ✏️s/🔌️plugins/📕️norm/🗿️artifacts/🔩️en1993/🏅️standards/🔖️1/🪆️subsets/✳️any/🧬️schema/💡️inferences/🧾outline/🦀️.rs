//! 🧾 Document outline for EN 1993 steel structure.

use crate::En1993Snapshot;

const SECTION_FIELDS: &[&str] = &[
    "annex",
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
];

/// 🧾️ `En1993` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
        ) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1993Outline {
    fn default() -> Self {
        Self::compute(&En1993Snapshot::default())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
