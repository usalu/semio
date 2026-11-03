//! 🧾 `outline` — document field structure for the scoped EN 1998 subject.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

const SECTION_FIELDS: &[&str] = &[
    "annex",
    "site",
    "buildings",
    "bridges",
    "assessments",
    "silos",
    "tanks",
    "foundations",
    "retainingWalls",
    "towers",
    "soilLayers",
    "diaphragms",
    "accelerograms",
    "piers",
    "bearings",
    "joints",
    "tieBeams",
    "slopes",
    "placements",
];

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1998Outline {
    pub fn compute(snapshot: &En1998Snapshot) -> Self {
        let entry_count = (snapshot.buildings.len()
            + snapshot.bridges.len()
            + snapshot.assessments.len()
            + snapshot.silos.len()
            + snapshot.tanks.len()
            + snapshot.foundations.len()
            + snapshot.retaining_walls.len()
            + snapshot.towers.len()
            + snapshot.soil_layers.len()
            + snapshot.diaphragms.len()
            + snapshot.accelerograms.len()
            + snapshot.piers.len()
            + snapshot.bearings.len()
            + snapshot.joints.len()
            + snapshot.tie_beams.len()
            + snapshot.slopes.len()
            + snapshot.placements.len()) as u32;
        Self {
            section_outline: SECTION_FIELDS.iter().map(|s| (*s).to_string()).collect(),
            field_count: SECTION_FIELDS.len() as u32,
            entry_count,
        }
    }
}
