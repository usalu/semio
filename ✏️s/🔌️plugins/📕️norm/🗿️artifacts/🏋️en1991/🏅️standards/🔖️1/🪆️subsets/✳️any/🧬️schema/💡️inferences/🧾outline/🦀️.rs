//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::En1991Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "snow_zone",
    "altitude",
    "en_sk",
    "exceptional_snow_north_german_lowlands",
    "wind_zone",
    "en_vb",
    "terrain_category",
    "mixed_terrain_upwind",
    "mixed_terrain_distance",
    "orography_factor",
    "coast_or_island",
    "air_density",
    "height",
    "width",
    "depth",
    "assumed_delta_t",
    "t_max",
    "t_min",
    "t_0",
    "thermal_element_type",
    "storey_count",
    "fire_mode",
    "construction_activity",
    "assumed_construction_qk",
    "structure_kind",
    "bridge_lane",
    "bridge_span",
    "bridge_lane_width",
    "assumed_bridge_tandem",
    "assumed_bridge_footway",
    "assumed_bridge_lm2",
    "assumed_bridge_udl",
    "crane_claimed",
    "crane_class",
    "hoist_class",
    "hoisting_speed",
    "assumed_crane_wheel",
    "assumed_crane_horizontal",
    "silo_claimed",
    "silo_kind",
    "silo_bulk_density",
    "silo_height",
    "silo_hydraulic_radius",
    "silo_mu",
    "silo_k",
    "assumed_silo_pressure",
    "assumed_silo_patch",
    "assumed_silo_wall_friction",
    "floors",
    "self_weight_elements",
    "roofs",
    "wind_faces",
    "accidental_cases",
];

/// 🧾️ `En1991` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1991Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1991Outline {
    pub fn compute(snapshot: &En1991Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.floors.len()
            + snapshot.roofs.len()
            + snapshot.wind_faces.len()
            + snapshot.self_weight_elements.len()
            + snapshot.accidental_cases.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1991Outline {
    fn default() -> Self {
        Self::compute(&En1991Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
