//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use semio_s_artifact_norm_en1991_model::En1991Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "snow_zone",
    "altitude",
    "en_sk",
    "north_german_lowland_snow",
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
    "bridge_tandem_x",
    "rail_alpha",
    "assumed_rail_lm71",
    "assumed_rail_lm71_udl",
    "assumed_rail_sw0",
    "assumed_rail_sw2",
    "crane_x",
    "crane_runway_length",
    "crane_wheel_base",
    "silo_diameter",
    "silo_eccentricity",
    "silo_hopper_angle_deg",
    "bridge_curve_radius",
    "fire_load_basis",
    "floors",
    "self_weight_elements",
    "roofs",
    "wind_faces",
    "accidental_cases",
    "bridge_lanes",
    "construction_stages",
    "roof_surfaces",
    "placements",
    "special_vehicles",
    "wind_bridges",
    "dynamic_elements",
    "impact_actions",
    "internal_explosions",
    "thermal_sections",
    "cases",
    "helicopter_landings",
    "tie_runs",
    "construction_loads",
    "falling_loads",
    "execution_imperfections",
    "derailment_cases",
    "crane_hoists",
    "footbridges",
    "rail_dynamics",
    "load_dispersals",
    "bulk_solids",
    "rotating_machines",
    "dust_explosions",
    "rail_load_groups",
    "rail_derailments",
    "fatigue_models",
    "support_collisions",
    "local_fires",
    "snow_guards",
];

/// 🧾️ `En1991` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
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
            + snapshot.accidental_cases.len()
            + snapshot.bridge_lanes.len()
            + snapshot.construction_stages.len()
            + snapshot.roof_surfaces.len()
            + snapshot.placements.len()
            + snapshot.special_vehicles.len()
            + snapshot.wind_bridges.len()
            + snapshot.dynamic_elements.len()
            + snapshot.impact_actions.len()
            + snapshot.internal_explosions.len()
            + snapshot.thermal_sections.len()
            + snapshot.cases.len()
            + snapshot.helicopter_landings.len()
            + snapshot.tie_runs.len()
            + snapshot.construction_loads.len()
            + snapshot.falling_loads.len()
            + snapshot.execution_imperfections.len()
            + snapshot.derailment_cases.len()
            + snapshot.crane_hoists.len()
            + snapshot.footbridges.len()
            + snapshot.rail_dynamics.len()
            + snapshot.load_dispersals.len()
            + snapshot.bulk_solids.len()
            + snapshot.rotating_machines.len()
            + snapshot.dust_explosions.len()
            + snapshot.rail_load_groups.len()
            + snapshot.rail_derailments.len()
            + snapshot.fatigue_models.len()
            + snapshot.support_collisions.len()
            + snapshot.local_fires.len()
            + snapshot.snow_guards.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1991Outline {
    fn default() -> Self {
        Self::compute(&En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
