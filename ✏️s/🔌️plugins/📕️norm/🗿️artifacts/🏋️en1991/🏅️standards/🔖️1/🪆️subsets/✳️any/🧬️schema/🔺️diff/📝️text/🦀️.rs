//! 🔺️ En1991 artifact — sparse field diff runtime.

use crate::artifact_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifact_schema::En1991Artifact;
use crate::En1991Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1991Diff {
    pub fn apply_to_artifact(&self, artifact: &En1991Artifact) -> protocol::MutationApplyResult<En1991Artifact> {
        if let Some(replacement) = &self.artifact {
            return Ok((**replacement).clone());
        }
        let mut next = artifact.clone();
        if let Some(value) = &self.annex { next.annex = *value; }
        if let Some(value) = &self.snow_zone { next.snow_zone = value.clone(); }
        if let Some(value) = &self.altitude { next.altitude = *value; }
        if let Some(value) = &self.en_sk { next.en_sk = *value; }
        if let Some(value) = &self.north_german_lowland_snow { next.north_german_lowland_snow = *value; }
        if let Some(value) = &self.wind_zone { next.wind_zone = *value; }
        if let Some(value) = &self.en_vb { next.en_vb = *value; }
        if let Some(value) = &self.terrain_category { next.terrain_category = *value; }
        if let Some(value) = &self.mixed_terrain_upwind { next.mixed_terrain_upwind = *value; }
        if let Some(value) = &self.mixed_terrain_distance { next.mixed_terrain_distance = *value; }
        if let Some(value) = &self.orography_factor { next.orography_factor = *value; }
        if let Some(value) = &self.coast_or_island { next.coast_or_island = *value; }
        if let Some(value) = &self.air_density { next.air_density = *value; }
        if let Some(value) = &self.height { next.height = *value; }
        if let Some(value) = &self.width { next.width = *value; }
        if let Some(value) = &self.depth { next.depth = *value; }
        if let Some(value) = &self.assumed_delta_t { next.assumed_delta_t = *value; }
        if let Some(value) = &self.t_max { next.t_max = *value; }
        if let Some(value) = &self.t_min { next.t_min = *value; }
        if let Some(value) = &self.t_0 { next.t_0 = *value; }
        if let Some(value) = &self.thermal_element_type { next.thermal_element_type = value.clone(); }
        if let Some(value) = &self.thermal_bridge_type { next.thermal_bridge_type = *value; }
        if let Some(value) = &self.delta_t_m { next.delta_t_m = *value; }
        if let Some(value) = &self.storey_count { next.storey_count = *value; }
        if let Some(value) = &self.fire_mode { next.fire_mode = *value; }
        if let Some(value) = &self.fire_curve { next.fire_curve = *value; }
        if let Some(value) = &self.fire_duration { next.fire_duration = *value; }
        if let Some(value) = &self.assumed_gas_temperature { next.assumed_gas_temperature = *value; }
        if let Some(value) = &self.assumed_h_net { next.assumed_h_net = *value; }
        if let Some(value) = &self.fire_compartment_area { next.fire_compartment_area = *value; }
        if let Some(value) = &self.fire_compartment_height { next.fire_compartment_height = *value; }
        if let Some(value) = &self.fire_opening_factor { next.fire_opening_factor = *value; }
        if let Some(value) = &self.fire_thermal_inertia { next.fire_thermal_inertia = *value; }
        if let Some(value) = &self.fire_occupancy { next.fire_occupancy = value.clone(); }
        if let Some(value) = &self.fire_load_density_qf { next.fire_load_density_qf = *value; }
        if let Some(value) = &self.assumed_qf_d { next.assumed_qf_d = *value; }
        if let Some(value) = &self.construction_activity { next.construction_activity = value.clone(); }
        if let Some(value) = &self.assumed_construction_qk { next.assumed_construction_qk = *value; }
        if let Some(value) = &self.structure_kind { next.structure_kind = *value; }
        if let Some(value) = &self.bridge_lane { next.bridge_lane = *value; }
        if let Some(value) = &self.bridge_span { next.bridge_span = *value; }
        if let Some(value) = &self.bridge_lane_width { next.bridge_lane_width = *value; }
        if let Some(value) = &self.assumed_bridge_tandem { next.assumed_bridge_tandem = *value; }
        if let Some(value) = &self.assumed_bridge_udl { next.assumed_bridge_udl = *value; }
        if let Some(value) = &self.assumed_bridge_lm2 { next.assumed_bridge_lm2 = *value; }
        if let Some(value) = &self.assumed_bridge_footway { next.assumed_bridge_footway = *value; }
        if let Some(value) = &self.assumed_bridge_lm3 { next.assumed_bridge_lm3 = *value; }
        if let Some(value) = &self.assumed_bridge_lm4 { next.assumed_bridge_lm4 = *value; }
        if let Some(value) = &self.bridge_load_group { next.bridge_load_group = value.clone(); }
        if let Some(value) = &self.crane_claimed { next.crane_claimed = *value; }
        if let Some(value) = &self.crane_class { next.crane_class = value.clone(); }
        if let Some(value) = &self.hoist_class { next.hoist_class = value.clone(); }
        if let Some(value) = &self.hoisting_speed { next.hoisting_speed = *value; }
        if let Some(value) = &self.assumed_crane_wheel { next.assumed_crane_wheel = *value; }
        if let Some(value) = &self.assumed_crane_horizontal { next.assumed_crane_horizontal = *value; }
        if let Some(value) = &self.silo_claimed { next.silo_claimed = *value; }
        if let Some(value) = &self.silo_kind { next.silo_kind = value.clone(); }
        if let Some(value) = &self.silo_bulk_density { next.silo_bulk_density = *value; }
        if let Some(value) = &self.silo_height { next.silo_height = *value; }
        if let Some(value) = &self.silo_hydraulic_radius { next.silo_hydraulic_radius = *value; }
        if let Some(value) = &self.silo_mu { next.silo_mu = *value; }
        if let Some(value) = &self.silo_k { next.silo_k = *value; }
        if let Some(value) = &self.assumed_silo_pressure { next.assumed_silo_pressure = *value; }
        if let Some(value) = &self.assumed_silo_patch { next.assumed_silo_patch = *value; }
        if let Some(value) = &self.assumed_silo_wall_friction { next.assumed_silo_wall_friction = *value; }
        if let Some(list) = &self.floors { next.floors = list.values.clone(); }
        if let Some(list) = &self.self_weight_elements { next.self_weight_elements = list.values.clone(); }
        if let Some(list) = &self.roofs { next.roofs = list.values.clone(); }
        if let Some(list) = &self.wind_faces { next.wind_faces = list.values.clone(); }
        if let Some(list) = &self.accidental_cases { next.accidental_cases = list.values.clone(); }
        Ok(next)
    }
}

impl MutationDiff<En1991Snapshot> for En1991Diff {
    fn apply(&self, snapshot: &En1991Snapshot) -> protocol::MutationApplyResult<En1991Snapshot> {
        if let Some(replacement) = &self.artifact {
            return Ok(replacement.to_snapshot());
        }
        let mut next = snapshot.clone();
        if let Some(value) = &self.annex { next.annex = *value; }
        if let Some(value) = &self.snow_zone { next.snow_zone = value.clone(); }
        if let Some(value) = &self.altitude { next.altitude = *value; }
        if let Some(value) = &self.en_sk { next.en_sk = *value; }
        if let Some(value) = &self.north_german_lowland_snow { next.north_german_lowland_snow = *value; }
        if let Some(value) = &self.wind_zone { next.wind_zone = *value; }
        if let Some(value) = &self.en_vb { next.en_vb = *value; }
        if let Some(value) = &self.terrain_category { next.terrain_category = *value; }
        if let Some(value) = &self.mixed_terrain_upwind { next.mixed_terrain_upwind = *value; }
        if let Some(value) = &self.mixed_terrain_distance { next.mixed_terrain_distance = *value; }
        if let Some(value) = &self.orography_factor { next.orography_factor = *value; }
        if let Some(value) = &self.coast_or_island { next.coast_or_island = *value; }
        if let Some(value) = &self.air_density { next.air_density = *value; }
        if let Some(value) = &self.height { next.height = *value; }
        if let Some(value) = &self.width { next.width = *value; }
        if let Some(value) = &self.depth { next.depth = *value; }
        if let Some(value) = &self.assumed_delta_t { next.assumed_delta_t = *value; }
        if let Some(value) = &self.t_max { next.t_max = *value; }
        if let Some(value) = &self.t_min { next.t_min = *value; }
        if let Some(value) = &self.t_0 { next.t_0 = *value; }
        if let Some(value) = &self.thermal_element_type { next.thermal_element_type = value.clone(); }
        if let Some(value) = &self.thermal_bridge_type { next.thermal_bridge_type = *value; }
        if let Some(value) = &self.delta_t_m { next.delta_t_m = *value; }
        if let Some(value) = &self.storey_count { next.storey_count = *value; }
        if let Some(value) = &self.fire_mode { next.fire_mode = *value; }
        if let Some(value) = &self.fire_curve { next.fire_curve = *value; }
        if let Some(value) = &self.fire_duration { next.fire_duration = *value; }
        if let Some(value) = &self.assumed_gas_temperature { next.assumed_gas_temperature = *value; }
        if let Some(value) = &self.assumed_h_net { next.assumed_h_net = *value; }
        if let Some(value) = &self.fire_compartment_area { next.fire_compartment_area = *value; }
        if let Some(value) = &self.fire_compartment_height { next.fire_compartment_height = *value; }
        if let Some(value) = &self.fire_opening_factor { next.fire_opening_factor = *value; }
        if let Some(value) = &self.fire_thermal_inertia { next.fire_thermal_inertia = *value; }
        if let Some(value) = &self.fire_occupancy { next.fire_occupancy = value.clone(); }
        if let Some(value) = &self.fire_load_density_qf { next.fire_load_density_qf = *value; }
        if let Some(value) = &self.assumed_qf_d { next.assumed_qf_d = *value; }
        if let Some(value) = &self.construction_activity { next.construction_activity = value.clone(); }
        if let Some(value) = &self.assumed_construction_qk { next.assumed_construction_qk = *value; }
        if let Some(value) = &self.structure_kind { next.structure_kind = *value; }
        if let Some(value) = &self.bridge_lane { next.bridge_lane = *value; }
        if let Some(value) = &self.bridge_span { next.bridge_span = *value; }
        if let Some(value) = &self.bridge_lane_width { next.bridge_lane_width = *value; }
        if let Some(value) = &self.assumed_bridge_tandem { next.assumed_bridge_tandem = *value; }
        if let Some(value) = &self.assumed_bridge_udl { next.assumed_bridge_udl = *value; }
        if let Some(value) = &self.assumed_bridge_lm2 { next.assumed_bridge_lm2 = *value; }
        if let Some(value) = &self.assumed_bridge_footway { next.assumed_bridge_footway = *value; }
        if let Some(value) = &self.assumed_bridge_lm3 { next.assumed_bridge_lm3 = *value; }
        if let Some(value) = &self.assumed_bridge_lm4 { next.assumed_bridge_lm4 = *value; }
        if let Some(value) = &self.bridge_load_group { next.bridge_load_group = value.clone(); }
        if let Some(value) = &self.crane_claimed { next.crane_claimed = *value; }
        if let Some(value) = &self.crane_class { next.crane_class = value.clone(); }
        if let Some(value) = &self.hoist_class { next.hoist_class = value.clone(); }
        if let Some(value) = &self.hoisting_speed { next.hoisting_speed = *value; }
        if let Some(value) = &self.assumed_crane_wheel { next.assumed_crane_wheel = *value; }
        if let Some(value) = &self.assumed_crane_horizontal { next.assumed_crane_horizontal = *value; }
        if let Some(value) = &self.silo_claimed { next.silo_claimed = *value; }
        if let Some(value) = &self.silo_kind { next.silo_kind = value.clone(); }
        if let Some(value) = &self.silo_bulk_density { next.silo_bulk_density = *value; }
        if let Some(value) = &self.silo_height { next.silo_height = *value; }
        if let Some(value) = &self.silo_hydraulic_radius { next.silo_hydraulic_radius = *value; }
        if let Some(value) = &self.silo_mu { next.silo_mu = *value; }
        if let Some(value) = &self.silo_k { next.silo_k = *value; }
        if let Some(value) = &self.assumed_silo_pressure { next.assumed_silo_pressure = *value; }
        if let Some(value) = &self.assumed_silo_patch { next.assumed_silo_patch = *value; }
        if let Some(value) = &self.assumed_silo_wall_friction { next.assumed_silo_wall_friction = *value; }
        if let Some(list) = &self.floors { next.floors = list.values.clone(); }
        if let Some(list) = &self.self_weight_elements { next.self_weight_elements = list.values.clone(); }
        if let Some(list) = &self.roofs { next.roofs = list.values.clone(); }
        if let Some(list) = &self.wind_faces { next.wind_faces = list.values.clone(); }
        if let Some(list) = &self.accidental_cases { next.accidental_cases = list.values.clone(); }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() { self.artifact = other.artifact; }
        if other.annex.is_some() { self.annex = other.annex; }
        if other.snow_zone.is_some() { self.snow_zone = other.snow_zone; }
        if other.altitude.is_some() { self.altitude = other.altitude; }
        if other.en_sk.is_some() { self.en_sk = other.en_sk; }
        if other.north_german_lowland_snow.is_some() { self.north_german_lowland_snow = other.north_german_lowland_snow; }
        if other.wind_zone.is_some() { self.wind_zone = other.wind_zone; }
        if other.en_vb.is_some() { self.en_vb = other.en_vb; }
        if other.terrain_category.is_some() { self.terrain_category = other.terrain_category; }
        if other.mixed_terrain_upwind.is_some() { self.mixed_terrain_upwind = other.mixed_terrain_upwind; }
        if other.mixed_terrain_distance.is_some() { self.mixed_terrain_distance = other.mixed_terrain_distance; }
        if other.orography_factor.is_some() { self.orography_factor = other.orography_factor; }
        if other.coast_or_island.is_some() { self.coast_or_island = other.coast_or_island; }
        if other.air_density.is_some() { self.air_density = other.air_density; }
        if other.height.is_some() { self.height = other.height; }
        if other.width.is_some() { self.width = other.width; }
        if other.depth.is_some() { self.depth = other.depth; }
        if other.assumed_delta_t.is_some() { self.assumed_delta_t = other.assumed_delta_t; }
        if other.t_max.is_some() { self.t_max = other.t_max; }
        if other.t_min.is_some() { self.t_min = other.t_min; }
        if other.t_0.is_some() { self.t_0 = other.t_0; }
        if other.thermal_element_type.is_some() { self.thermal_element_type = other.thermal_element_type; }
        if other.thermal_bridge_type.is_some() { self.thermal_bridge_type = other.thermal_bridge_type; }
        if other.delta_t_m.is_some() { self.delta_t_m = other.delta_t_m; }
        if other.storey_count.is_some() { self.storey_count = other.storey_count; }
        if other.fire_mode.is_some() { self.fire_mode = other.fire_mode; }
        if other.fire_curve.is_some() { self.fire_curve = other.fire_curve; }
        if other.fire_duration.is_some() { self.fire_duration = other.fire_duration; }
        if other.assumed_gas_temperature.is_some() { self.assumed_gas_temperature = other.assumed_gas_temperature; }
        if other.assumed_h_net.is_some() { self.assumed_h_net = other.assumed_h_net; }
        if other.fire_compartment_area.is_some() { self.fire_compartment_area = other.fire_compartment_area; }
        if other.fire_compartment_height.is_some() { self.fire_compartment_height = other.fire_compartment_height; }
        if other.fire_opening_factor.is_some() { self.fire_opening_factor = other.fire_opening_factor; }
        if other.fire_thermal_inertia.is_some() { self.fire_thermal_inertia = other.fire_thermal_inertia; }
        if other.fire_occupancy.is_some() { self.fire_occupancy = other.fire_occupancy; }
        if other.fire_load_density_qf.is_some() { self.fire_load_density_qf = other.fire_load_density_qf; }
        if other.assumed_qf_d.is_some() { self.assumed_qf_d = other.assumed_qf_d; }
        if other.construction_activity.is_some() { self.construction_activity = other.construction_activity; }
        if other.assumed_construction_qk.is_some() { self.assumed_construction_qk = other.assumed_construction_qk; }
        if other.structure_kind.is_some() { self.structure_kind = other.structure_kind; }
        if other.bridge_lane.is_some() { self.bridge_lane = other.bridge_lane; }
        if other.bridge_span.is_some() { self.bridge_span = other.bridge_span; }
        if other.bridge_lane_width.is_some() { self.bridge_lane_width = other.bridge_lane_width; }
        if other.assumed_bridge_tandem.is_some() { self.assumed_bridge_tandem = other.assumed_bridge_tandem; }
        if other.assumed_bridge_udl.is_some() { self.assumed_bridge_udl = other.assumed_bridge_udl; }
        if other.assumed_bridge_lm2.is_some() { self.assumed_bridge_lm2 = other.assumed_bridge_lm2; }
        if other.assumed_bridge_footway.is_some() { self.assumed_bridge_footway = other.assumed_bridge_footway; }
        if other.assumed_bridge_lm3.is_some() { self.assumed_bridge_lm3 = other.assumed_bridge_lm3; }
        if other.assumed_bridge_lm4.is_some() { self.assumed_bridge_lm4 = other.assumed_bridge_lm4; }
        if other.bridge_load_group.is_some() { self.bridge_load_group = other.bridge_load_group; }
        if other.crane_claimed.is_some() { self.crane_claimed = other.crane_claimed; }
        if other.crane_class.is_some() { self.crane_class = other.crane_class; }
        if other.hoist_class.is_some() { self.hoist_class = other.hoist_class; }
        if other.hoisting_speed.is_some() { self.hoisting_speed = other.hoisting_speed; }
        if other.assumed_crane_wheel.is_some() { self.assumed_crane_wheel = other.assumed_crane_wheel; }
        if other.assumed_crane_horizontal.is_some() { self.assumed_crane_horizontal = other.assumed_crane_horizontal; }
        if other.silo_claimed.is_some() { self.silo_claimed = other.silo_claimed; }
        if other.silo_kind.is_some() { self.silo_kind = other.silo_kind; }
        if other.silo_bulk_density.is_some() { self.silo_bulk_density = other.silo_bulk_density; }
        if other.silo_height.is_some() { self.silo_height = other.silo_height; }
        if other.silo_hydraulic_radius.is_some() { self.silo_hydraulic_radius = other.silo_hydraulic_radius; }
        if other.silo_mu.is_some() { self.silo_mu = other.silo_mu; }
        if other.silo_k.is_some() { self.silo_k = other.silo_k; }
        if other.assumed_silo_pressure.is_some() { self.assumed_silo_pressure = other.assumed_silo_pressure; }
        if other.assumed_silo_patch.is_some() { self.assumed_silo_patch = other.assumed_silo_patch; }
        if other.assumed_silo_wall_friction.is_some() { self.assumed_silo_wall_friction = other.assumed_silo_wall_friction; }
        if other.floors.is_some() { self.floors = other.floors; }
        if other.self_weight_elements.is_some() { self.self_weight_elements = other.self_weight_elements; }
        if other.roofs.is_some() { self.roofs = other.roofs; }
        if other.wind_faces.is_some() { self.wind_faces = other.wind_faces; }
        if other.accidental_cases.is_some() { self.accidental_cases = other.accidental_cases; }
    }
}
//#endregion 🔖️Apply
