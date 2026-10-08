//! 🧬️ EN1991 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_artifact_norm_contract::{norm_list_delta, norm_row_patch};

use crate::En1991Snapshot;

//#region 🔖️Rows
norm_row_patch! {
    /// 🩹 Sparse field patch of one `FloorArea`.
    pub En1991FloorPatch of crate::FloorArea { set { category: String, area: f64, assumed_qk: f64, assumed_qk_concentrated: f64, assumed_partitions: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `FloorArea` list.
    pub En1991FloorDelta { addition: En1991FloorAddition, modification: En1991FloorModification, row: crate::FloorArea, patch: En1991FloorPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `SelfWeightElement`.
    pub En1991SelfWeightElementPatch of crate::SelfWeightElement { set { material: String, thickness: f64, assumed_gk: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SelfWeightElement` list.
    pub En1991SelfWeightElementDelta { addition: En1991SelfWeightElementAddition, modification: En1991SelfWeightElementModification, row: crate::SelfWeightElement, patch: En1991SelfWeightElementPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `RoofArea`.
    pub En1991RoofPatch of crate::RoofArea { set { roof_type: String, pitch_deg: f64, c_e: f64, c_t: f64, has_parapet: bool, parapet_height: f64, drift_obstruction_height: f64, multi_span: bool, assumed_sk: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `RoofArea` list.
    pub En1991RoofDelta { addition: En1991RoofAddition, modification: En1991RoofModification, row: crate::RoofArea, patch: En1991RoofPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `WindFace`.
    pub En1991WindFacePatch of crate::WindFace { set { zone: String, z: f64, c_pe10: f64, c_pe1: f64, c_pi: f64, c_s: f64, c_d: f64, loaded_area: f64, assumed_wp: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `WindFace` list.
    pub En1991WindFaceDelta { addition: En1991WindFaceAddition, modification: En1991WindFaceModification, row: crate::WindFace, patch: En1991WindFacePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `AccidentalCase`.
    pub En1991AccidentalCasePatch of crate::AccidentalCase { set { impact: Vec<crate::AccidentalImpact>, explosion: Vec<crate::AccidentalExplosion> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `AccidentalCase` list.
    pub En1991AccidentalCaseDelta { addition: En1991AccidentalCaseAddition, modification: En1991AccidentalCaseModification, row: crate::AccidentalCase, patch: En1991AccidentalCasePatch, key: id }
}

//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1991 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub snow_zone: Option<String>,
    #[state(artifact)]
    pub altitude: Option<f64>,
    #[state(artifact)]
    pub en_sk: Option<f64>,
    #[state(artifact)]
    pub north_german_lowland_snow: Option<bool>,
    #[state(artifact)]
    pub wind_zone: Option<u8>,
    #[state(artifact)]
    pub en_vb: Option<f64>,
    #[state(artifact)]
    pub terrain_category: Option<u8>,
    #[state(artifact)]
    pub mixed_terrain_upwind: Option<u8>,
    #[state(artifact)]
    pub mixed_terrain_distance: Option<f64>,
    #[state(artifact)]
    pub orography_factor: Option<f64>,
    #[state(artifact)]
    pub coast_or_island: Option<bool>,
    #[state(artifact)]
    pub air_density: Option<f64>,
    #[state(artifact)]
    pub height: Option<f64>,
    #[state(artifact)]
    pub width: Option<f64>,
    #[state(artifact)]
    pub depth: Option<f64>,
    #[state(artifact)]
    pub assumed_delta_t: Option<f64>,
    #[state(artifact)]
    pub t_max: Option<f64>,
    #[state(artifact)]
    pub t_min: Option<f64>,
    #[state(artifact)]
    pub t_0: Option<f64>,
    #[state(artifact)]
    pub thermal_element_type: Option<String>,
    #[state(artifact)]
    pub thermal_bridge_type: Option<u8>,
    #[state(artifact)]
    pub delta_t_m: Option<f64>,
    #[state(artifact)]
    pub storey_count: Option<u8>,
    #[state(artifact)]
    pub fire_mode: Option<crate::FireMode>,
    #[state(artifact)]
    pub fire_curve: Option<crate::part_1_2::FireCurve>,
    #[state(artifact)]
    pub fire_duration: Option<f64>,
    #[state(artifact)]
    pub assumed_gas_temperature: Option<f64>,
    #[state(artifact)]
    pub assumed_h_net: Option<f64>,
    #[state(artifact)]
    pub fire_compartment_area: Option<f64>,
    #[state(artifact)]
    pub fire_compartment_height: Option<f64>,
    #[state(artifact)]
    pub fire_opening_factor: Option<f64>,
    #[state(artifact)]
    pub fire_thermal_inertia: Option<f64>,
    #[state(artifact)]
    pub fire_occupancy: Option<String>,
    #[state(artifact)]
    pub fire_load_density_qf: Option<f64>,
    #[state(artifact)]
    pub assumed_qf_d: Option<f64>,
    #[state(artifact)]
    pub construction_activity: Option<String>,
    #[state(artifact)]
    pub assumed_construction_qk: Option<f64>,
    #[state(artifact)]
    pub structure_kind: Option<crate::StructureKind>,
    #[state(artifact)]
    pub bridge_lane: Option<u8>,
    #[state(artifact)]
    pub bridge_span: Option<f64>,
    #[state(artifact)]
    pub bridge_lane_width: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_tandem: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_udl: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm2: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_footway: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm3: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm4: Option<f64>,
    #[state(artifact)]
    pub bridge_load_group: Option<String>,
    #[state(artifact)]
    pub crane_claimed: Option<bool>,
    #[state(artifact)]
    pub crane_class: Option<String>,
    #[state(artifact)]
    pub hoist_class: Option<String>,
    #[state(artifact)]
    pub hoisting_speed: Option<f64>,
    #[state(artifact)]
    pub assumed_crane_wheel: Option<f64>,
    #[state(artifact)]
    pub assumed_crane_horizontal: Option<f64>,
    #[state(artifact)]
    pub silo_claimed: Option<bool>,
    #[state(artifact)]
    pub silo_kind: Option<String>,
    #[state(artifact)]
    pub silo_bulk_density: Option<f64>,
    #[state(artifact)]
    pub silo_height: Option<f64>,
    #[state(artifact)]
    pub silo_hydraulic_radius: Option<f64>,
    #[state(artifact)]
    pub silo_mu: Option<f64>,
    #[state(artifact)]
    pub silo_k: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_pressure: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_patch: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_wall_friction: Option<f64>,
    #[state(artifact)]
    pub floors: En1991FloorDelta,
    #[state(artifact)]
    pub self_weight_elements: En1991SelfWeightElementDelta,
    #[state(artifact)]
    pub roofs: En1991RoofDelta,
    #[state(artifact)]
    pub wind_faces: En1991WindFaceDelta,
    #[state(artifact)]
    pub accidental_cases: En1991AccidentalCaseDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1991Snapshot> for En1991Diff {
    fn apply(&self, base: &En1991Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1991Snapshot> {
        Ok(En1991Snapshot {
            annex: self.annex.unwrap_or(base.annex),
            snow_zone: self.snow_zone.clone().unwrap_or_else(|| base.snow_zone.clone()),
            altitude: self.altitude.unwrap_or(base.altitude),
            en_sk: self.en_sk.unwrap_or(base.en_sk),
            north_german_lowland_snow: self.north_german_lowland_snow.unwrap_or(base.north_german_lowland_snow),
            wind_zone: self.wind_zone.unwrap_or(base.wind_zone),
            en_vb: self.en_vb.unwrap_or(base.en_vb),
            terrain_category: self.terrain_category.unwrap_or(base.terrain_category),
            mixed_terrain_upwind: self.mixed_terrain_upwind.unwrap_or(base.mixed_terrain_upwind),
            mixed_terrain_distance: self.mixed_terrain_distance.unwrap_or(base.mixed_terrain_distance),
            orography_factor: self.orography_factor.unwrap_or(base.orography_factor),
            coast_or_island: self.coast_or_island.unwrap_or(base.coast_or_island),
            air_density: self.air_density.unwrap_or(base.air_density),
            height: self.height.unwrap_or(base.height),
            width: self.width.unwrap_or(base.width),
            depth: self.depth.unwrap_or(base.depth),
            assumed_delta_t: self.assumed_delta_t.unwrap_or(base.assumed_delta_t),
            t_max: self.t_max.unwrap_or(base.t_max),
            t_min: self.t_min.unwrap_or(base.t_min),
            t_0: self.t_0.unwrap_or(base.t_0),
            thermal_element_type: self.thermal_element_type.clone().unwrap_or_else(|| base.thermal_element_type.clone()),
            thermal_bridge_type: self.thermal_bridge_type.unwrap_or(base.thermal_bridge_type),
            delta_t_m: self.delta_t_m.unwrap_or(base.delta_t_m),
            storey_count: self.storey_count.unwrap_or(base.storey_count),
            fire_mode: self.fire_mode.unwrap_or(base.fire_mode),
            fire_curve: self.fire_curve.unwrap_or(base.fire_curve),
            fire_duration: self.fire_duration.unwrap_or(base.fire_duration),
            assumed_gas_temperature: self.assumed_gas_temperature.unwrap_or(base.assumed_gas_temperature),
            assumed_h_net: self.assumed_h_net.unwrap_or(base.assumed_h_net),
            fire_compartment_area: self.fire_compartment_area.unwrap_or(base.fire_compartment_area),
            fire_compartment_height: self.fire_compartment_height.unwrap_or(base.fire_compartment_height),
            fire_opening_factor: self.fire_opening_factor.unwrap_or(base.fire_opening_factor),
            fire_thermal_inertia: self.fire_thermal_inertia.unwrap_or(base.fire_thermal_inertia),
            fire_occupancy: self.fire_occupancy.clone().unwrap_or_else(|| base.fire_occupancy.clone()),
            fire_load_density_qf: self.fire_load_density_qf.unwrap_or(base.fire_load_density_qf),
            assumed_qf_d: self.assumed_qf_d.unwrap_or(base.assumed_qf_d),
            construction_activity: self.construction_activity.clone().unwrap_or_else(|| base.construction_activity.clone()),
            assumed_construction_qk: self.assumed_construction_qk.unwrap_or(base.assumed_construction_qk),
            structure_kind: self.structure_kind.unwrap_or(base.structure_kind),
            bridge_lane: self.bridge_lane.unwrap_or(base.bridge_lane),
            bridge_span: self.bridge_span.unwrap_or(base.bridge_span),
            bridge_lane_width: self.bridge_lane_width.unwrap_or(base.bridge_lane_width),
            assumed_bridge_tandem: self.assumed_bridge_tandem.unwrap_or(base.assumed_bridge_tandem),
            assumed_bridge_udl: self.assumed_bridge_udl.unwrap_or(base.assumed_bridge_udl),
            assumed_bridge_lm2: self.assumed_bridge_lm2.unwrap_or(base.assumed_bridge_lm2),
            assumed_bridge_footway: self.assumed_bridge_footway.unwrap_or(base.assumed_bridge_footway),
            assumed_bridge_lm3: self.assumed_bridge_lm3.unwrap_or(base.assumed_bridge_lm3),
            assumed_bridge_lm4: self.assumed_bridge_lm4.unwrap_or(base.assumed_bridge_lm4),
            bridge_load_group: self.bridge_load_group.clone().unwrap_or_else(|| base.bridge_load_group.clone()),
            crane_claimed: self.crane_claimed.unwrap_or(base.crane_claimed),
            crane_class: self.crane_class.clone().unwrap_or_else(|| base.crane_class.clone()),
            hoist_class: self.hoist_class.clone().unwrap_or_else(|| base.hoist_class.clone()),
            hoisting_speed: self.hoisting_speed.unwrap_or(base.hoisting_speed),
            assumed_crane_wheel: self.assumed_crane_wheel.unwrap_or(base.assumed_crane_wheel),
            assumed_crane_horizontal: self.assumed_crane_horizontal.unwrap_or(base.assumed_crane_horizontal),
            silo_claimed: self.silo_claimed.unwrap_or(base.silo_claimed),
            silo_kind: self.silo_kind.clone().unwrap_or_else(|| base.silo_kind.clone()),
            silo_bulk_density: self.silo_bulk_density.unwrap_or(base.silo_bulk_density),
            silo_height: self.silo_height.unwrap_or(base.silo_height),
            silo_hydraulic_radius: self.silo_hydraulic_radius.unwrap_or(base.silo_hydraulic_radius),
            silo_mu: self.silo_mu.unwrap_or(base.silo_mu),
            silo_k: self.silo_k.unwrap_or(base.silo_k),
            assumed_silo_pressure: self.assumed_silo_pressure.unwrap_or(base.assumed_silo_pressure),
            assumed_silo_patch: self.assumed_silo_patch.unwrap_or(base.assumed_silo_patch),
            assumed_silo_wall_friction: self.assumed_silo_wall_friction.unwrap_or(base.assumed_silo_wall_friction),
            floors: self.floors.commit_onto(&base.floors).map_err(|error| error.under(["floors"]))?,
            self_weight_elements: self.self_weight_elements.commit_onto(&base.self_weight_elements).map_err(|error| error.under(["self_weight_elements"]))?,
            roofs: self.roofs.commit_onto(&base.roofs).map_err(|error| error.under(["roofs"]))?,
            wind_faces: self.wind_faces.commit_onto(&base.wind_faces).map_err(|error| error.under(["wind_faces"]))?,
            accidental_cases: self.accidental_cases.commit_onto(&base.accidental_cases).map_err(|error| error.under(["accidental_cases"]))?,
        })
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.snow_zone.is_some() {
            self.snow_zone = other.snow_zone;
        }
        if other.altitude.is_some() {
            self.altitude = other.altitude;
        }
        if other.en_sk.is_some() {
            self.en_sk = other.en_sk;
        }
        if other.north_german_lowland_snow.is_some() {
            self.north_german_lowland_snow = other.north_german_lowland_snow;
        }
        if other.wind_zone.is_some() {
            self.wind_zone = other.wind_zone;
        }
        if other.en_vb.is_some() {
            self.en_vb = other.en_vb;
        }
        if other.terrain_category.is_some() {
            self.terrain_category = other.terrain_category;
        }
        if other.mixed_terrain_upwind.is_some() {
            self.mixed_terrain_upwind = other.mixed_terrain_upwind;
        }
        if other.mixed_terrain_distance.is_some() {
            self.mixed_terrain_distance = other.mixed_terrain_distance;
        }
        if other.orography_factor.is_some() {
            self.orography_factor = other.orography_factor;
        }
        if other.coast_or_island.is_some() {
            self.coast_or_island = other.coast_or_island;
        }
        if other.air_density.is_some() {
            self.air_density = other.air_density;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.depth.is_some() {
            self.depth = other.depth;
        }
        if other.assumed_delta_t.is_some() {
            self.assumed_delta_t = other.assumed_delta_t;
        }
        if other.t_max.is_some() {
            self.t_max = other.t_max;
        }
        if other.t_min.is_some() {
            self.t_min = other.t_min;
        }
        if other.t_0.is_some() {
            self.t_0 = other.t_0;
        }
        if other.thermal_element_type.is_some() {
            self.thermal_element_type = other.thermal_element_type;
        }
        if other.thermal_bridge_type.is_some() {
            self.thermal_bridge_type = other.thermal_bridge_type;
        }
        if other.delta_t_m.is_some() {
            self.delta_t_m = other.delta_t_m;
        }
        if other.storey_count.is_some() {
            self.storey_count = other.storey_count;
        }
        if other.fire_mode.is_some() {
            self.fire_mode = other.fire_mode;
        }
        if other.fire_curve.is_some() {
            self.fire_curve = other.fire_curve;
        }
        if other.fire_duration.is_some() {
            self.fire_duration = other.fire_duration;
        }
        if other.assumed_gas_temperature.is_some() {
            self.assumed_gas_temperature = other.assumed_gas_temperature;
        }
        if other.assumed_h_net.is_some() {
            self.assumed_h_net = other.assumed_h_net;
        }
        if other.fire_compartment_area.is_some() {
            self.fire_compartment_area = other.fire_compartment_area;
        }
        if other.fire_compartment_height.is_some() {
            self.fire_compartment_height = other.fire_compartment_height;
        }
        if other.fire_opening_factor.is_some() {
            self.fire_opening_factor = other.fire_opening_factor;
        }
        if other.fire_thermal_inertia.is_some() {
            self.fire_thermal_inertia = other.fire_thermal_inertia;
        }
        if other.fire_occupancy.is_some() {
            self.fire_occupancy = other.fire_occupancy;
        }
        if other.fire_load_density_qf.is_some() {
            self.fire_load_density_qf = other.fire_load_density_qf;
        }
        if other.assumed_qf_d.is_some() {
            self.assumed_qf_d = other.assumed_qf_d;
        }
        if other.construction_activity.is_some() {
            self.construction_activity = other.construction_activity;
        }
        if other.assumed_construction_qk.is_some() {
            self.assumed_construction_qk = other.assumed_construction_qk;
        }
        if other.structure_kind.is_some() {
            self.structure_kind = other.structure_kind;
        }
        if other.bridge_lane.is_some() {
            self.bridge_lane = other.bridge_lane;
        }
        if other.bridge_span.is_some() {
            self.bridge_span = other.bridge_span;
        }
        if other.bridge_lane_width.is_some() {
            self.bridge_lane_width = other.bridge_lane_width;
        }
        if other.assumed_bridge_tandem.is_some() {
            self.assumed_bridge_tandem = other.assumed_bridge_tandem;
        }
        if other.assumed_bridge_udl.is_some() {
            self.assumed_bridge_udl = other.assumed_bridge_udl;
        }
        if other.assumed_bridge_lm2.is_some() {
            self.assumed_bridge_lm2 = other.assumed_bridge_lm2;
        }
        if other.assumed_bridge_footway.is_some() {
            self.assumed_bridge_footway = other.assumed_bridge_footway;
        }
        if other.assumed_bridge_lm3.is_some() {
            self.assumed_bridge_lm3 = other.assumed_bridge_lm3;
        }
        if other.assumed_bridge_lm4.is_some() {
            self.assumed_bridge_lm4 = other.assumed_bridge_lm4;
        }
        if other.bridge_load_group.is_some() {
            self.bridge_load_group = other.bridge_load_group;
        }
        if other.crane_claimed.is_some() {
            self.crane_claimed = other.crane_claimed;
        }
        if other.crane_class.is_some() {
            self.crane_class = other.crane_class;
        }
        if other.hoist_class.is_some() {
            self.hoist_class = other.hoist_class;
        }
        if other.hoisting_speed.is_some() {
            self.hoisting_speed = other.hoisting_speed;
        }
        if other.assumed_crane_wheel.is_some() {
            self.assumed_crane_wheel = other.assumed_crane_wheel;
        }
        if other.assumed_crane_horizontal.is_some() {
            self.assumed_crane_horizontal = other.assumed_crane_horizontal;
        }
        if other.silo_claimed.is_some() {
            self.silo_claimed = other.silo_claimed;
        }
        if other.silo_kind.is_some() {
            self.silo_kind = other.silo_kind;
        }
        if other.silo_bulk_density.is_some() {
            self.silo_bulk_density = other.silo_bulk_density;
        }
        if other.silo_height.is_some() {
            self.silo_height = other.silo_height;
        }
        if other.silo_hydraulic_radius.is_some() {
            self.silo_hydraulic_radius = other.silo_hydraulic_radius;
        }
        if other.silo_mu.is_some() {
            self.silo_mu = other.silo_mu;
        }
        if other.silo_k.is_some() {
            self.silo_k = other.silo_k;
        }
        if other.assumed_silo_pressure.is_some() {
            self.assumed_silo_pressure = other.assumed_silo_pressure;
        }
        if other.assumed_silo_patch.is_some() {
            self.assumed_silo_patch = other.assumed_silo_patch;
        }
        if other.assumed_silo_wall_friction.is_some() {
            self.assumed_silo_wall_friction = other.assumed_silo_wall_friction;
        }
        self.floors.absorb(other.floors);
        self.self_weight_elements.absorb(other.self_weight_elements);
        self.roofs.absorb(other.roofs);
        self.wind_faces.absorb(other.wind_faces);
        self.accidental_cases.absorb(other.accidental_cases);
    }
}

impl DiffAlgebra<En1991Snapshot> for En1991Diff {
    fn inverse(&self, base: &En1991Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex),
            snow_zone: self.snow_zone.as_ref().map(|_| base.snow_zone.clone()),
            altitude: self.altitude.as_ref().map(|_| base.altitude),
            en_sk: self.en_sk.as_ref().map(|_| base.en_sk),
            north_german_lowland_snow: self.north_german_lowland_snow.as_ref().map(|_| base.north_german_lowland_snow),
            wind_zone: self.wind_zone.as_ref().map(|_| base.wind_zone),
            en_vb: self.en_vb.as_ref().map(|_| base.en_vb),
            terrain_category: self.terrain_category.as_ref().map(|_| base.terrain_category),
            mixed_terrain_upwind: self.mixed_terrain_upwind.as_ref().map(|_| base.mixed_terrain_upwind),
            mixed_terrain_distance: self.mixed_terrain_distance.as_ref().map(|_| base.mixed_terrain_distance),
            orography_factor: self.orography_factor.as_ref().map(|_| base.orography_factor),
            coast_or_island: self.coast_or_island.as_ref().map(|_| base.coast_or_island),
            air_density: self.air_density.as_ref().map(|_| base.air_density),
            height: self.height.as_ref().map(|_| base.height),
            width: self.width.as_ref().map(|_| base.width),
            depth: self.depth.as_ref().map(|_| base.depth),
            assumed_delta_t: self.assumed_delta_t.as_ref().map(|_| base.assumed_delta_t),
            t_max: self.t_max.as_ref().map(|_| base.t_max),
            t_min: self.t_min.as_ref().map(|_| base.t_min),
            t_0: self.t_0.as_ref().map(|_| base.t_0),
            thermal_element_type: self.thermal_element_type.as_ref().map(|_| base.thermal_element_type.clone()),
            thermal_bridge_type: self.thermal_bridge_type.as_ref().map(|_| base.thermal_bridge_type),
            delta_t_m: self.delta_t_m.as_ref().map(|_| base.delta_t_m),
            storey_count: self.storey_count.as_ref().map(|_| base.storey_count),
            fire_mode: self.fire_mode.as_ref().map(|_| base.fire_mode),
            fire_curve: self.fire_curve.as_ref().map(|_| base.fire_curve),
            fire_duration: self.fire_duration.as_ref().map(|_| base.fire_duration),
            assumed_gas_temperature: self.assumed_gas_temperature.as_ref().map(|_| base.assumed_gas_temperature),
            assumed_h_net: self.assumed_h_net.as_ref().map(|_| base.assumed_h_net),
            fire_compartment_area: self.fire_compartment_area.as_ref().map(|_| base.fire_compartment_area),
            fire_compartment_height: self.fire_compartment_height.as_ref().map(|_| base.fire_compartment_height),
            fire_opening_factor: self.fire_opening_factor.as_ref().map(|_| base.fire_opening_factor),
            fire_thermal_inertia: self.fire_thermal_inertia.as_ref().map(|_| base.fire_thermal_inertia),
            fire_occupancy: self.fire_occupancy.as_ref().map(|_| base.fire_occupancy.clone()),
            fire_load_density_qf: self.fire_load_density_qf.as_ref().map(|_| base.fire_load_density_qf),
            assumed_qf_d: self.assumed_qf_d.as_ref().map(|_| base.assumed_qf_d),
            construction_activity: self.construction_activity.as_ref().map(|_| base.construction_activity.clone()),
            assumed_construction_qk: self.assumed_construction_qk.as_ref().map(|_| base.assumed_construction_qk),
            structure_kind: self.structure_kind.as_ref().map(|_| base.structure_kind),
            bridge_lane: self.bridge_lane.as_ref().map(|_| base.bridge_lane),
            bridge_span: self.bridge_span.as_ref().map(|_| base.bridge_span),
            bridge_lane_width: self.bridge_lane_width.as_ref().map(|_| base.bridge_lane_width),
            assumed_bridge_tandem: self.assumed_bridge_tandem.as_ref().map(|_| base.assumed_bridge_tandem),
            assumed_bridge_udl: self.assumed_bridge_udl.as_ref().map(|_| base.assumed_bridge_udl),
            assumed_bridge_lm2: self.assumed_bridge_lm2.as_ref().map(|_| base.assumed_bridge_lm2),
            assumed_bridge_footway: self.assumed_bridge_footway.as_ref().map(|_| base.assumed_bridge_footway),
            assumed_bridge_lm3: self.assumed_bridge_lm3.as_ref().map(|_| base.assumed_bridge_lm3),
            assumed_bridge_lm4: self.assumed_bridge_lm4.as_ref().map(|_| base.assumed_bridge_lm4),
            bridge_load_group: self.bridge_load_group.as_ref().map(|_| base.bridge_load_group.clone()),
            crane_claimed: self.crane_claimed.as_ref().map(|_| base.crane_claimed),
            crane_class: self.crane_class.as_ref().map(|_| base.crane_class.clone()),
            hoist_class: self.hoist_class.as_ref().map(|_| base.hoist_class.clone()),
            hoisting_speed: self.hoisting_speed.as_ref().map(|_| base.hoisting_speed),
            assumed_crane_wheel: self.assumed_crane_wheel.as_ref().map(|_| base.assumed_crane_wheel),
            assumed_crane_horizontal: self.assumed_crane_horizontal.as_ref().map(|_| base.assumed_crane_horizontal),
            silo_claimed: self.silo_claimed.as_ref().map(|_| base.silo_claimed),
            silo_kind: self.silo_kind.as_ref().map(|_| base.silo_kind.clone()),
            silo_bulk_density: self.silo_bulk_density.as_ref().map(|_| base.silo_bulk_density),
            silo_height: self.silo_height.as_ref().map(|_| base.silo_height),
            silo_hydraulic_radius: self.silo_hydraulic_radius.as_ref().map(|_| base.silo_hydraulic_radius),
            silo_mu: self.silo_mu.as_ref().map(|_| base.silo_mu),
            silo_k: self.silo_k.as_ref().map(|_| base.silo_k),
            assumed_silo_pressure: self.assumed_silo_pressure.as_ref().map(|_| base.assumed_silo_pressure),
            assumed_silo_patch: self.assumed_silo_patch.as_ref().map(|_| base.assumed_silo_patch),
            assumed_silo_wall_friction: self.assumed_silo_wall_friction.as_ref().map(|_| base.assumed_silo_wall_friction),
            floors: self.floors.inverse(&base.floors),
            self_weight_elements: self.self_weight_elements.inverse(&base.self_weight_elements),
            roofs: self.roofs.inverse(&base.roofs),
            wind_faces: self.wind_faces.inverse(&base.wind_faces),
            accidental_cases: self.accidental_cases.inverse(&base.accidental_cases),
        }
    }

    fn between(base: &En1991Snapshot, other: &En1991Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex),
            snow_zone: (base.snow_zone != other.snow_zone).then(|| other.snow_zone.clone()),
            altitude: (base.altitude != other.altitude).then(|| other.altitude),
            en_sk: (base.en_sk != other.en_sk).then(|| other.en_sk),
            north_german_lowland_snow: (base.north_german_lowland_snow != other.north_german_lowland_snow).then(|| other.north_german_lowland_snow),
            wind_zone: (base.wind_zone != other.wind_zone).then(|| other.wind_zone),
            en_vb: (base.en_vb != other.en_vb).then(|| other.en_vb),
            terrain_category: (base.terrain_category != other.terrain_category).then(|| other.terrain_category),
            mixed_terrain_upwind: (base.mixed_terrain_upwind != other.mixed_terrain_upwind).then(|| other.mixed_terrain_upwind),
            mixed_terrain_distance: (base.mixed_terrain_distance != other.mixed_terrain_distance).then(|| other.mixed_terrain_distance),
            orography_factor: (base.orography_factor != other.orography_factor).then(|| other.orography_factor),
            coast_or_island: (base.coast_or_island != other.coast_or_island).then(|| other.coast_or_island),
            air_density: (base.air_density != other.air_density).then(|| other.air_density),
            height: (base.height != other.height).then(|| other.height),
            width: (base.width != other.width).then(|| other.width),
            depth: (base.depth != other.depth).then(|| other.depth),
            assumed_delta_t: (base.assumed_delta_t != other.assumed_delta_t).then(|| other.assumed_delta_t),
            t_max: (base.t_max != other.t_max).then(|| other.t_max),
            t_min: (base.t_min != other.t_min).then(|| other.t_min),
            t_0: (base.t_0 != other.t_0).then(|| other.t_0),
            thermal_element_type: (base.thermal_element_type != other.thermal_element_type).then(|| other.thermal_element_type.clone()),
            thermal_bridge_type: (base.thermal_bridge_type != other.thermal_bridge_type).then(|| other.thermal_bridge_type),
            delta_t_m: (base.delta_t_m != other.delta_t_m).then(|| other.delta_t_m),
            storey_count: (base.storey_count != other.storey_count).then(|| other.storey_count),
            fire_mode: (base.fire_mode != other.fire_mode).then(|| other.fire_mode),
            fire_curve: (base.fire_curve != other.fire_curve).then(|| other.fire_curve),
            fire_duration: (base.fire_duration != other.fire_duration).then(|| other.fire_duration),
            assumed_gas_temperature: (base.assumed_gas_temperature != other.assumed_gas_temperature).then(|| other.assumed_gas_temperature),
            assumed_h_net: (base.assumed_h_net != other.assumed_h_net).then(|| other.assumed_h_net),
            fire_compartment_area: (base.fire_compartment_area != other.fire_compartment_area).then(|| other.fire_compartment_area),
            fire_compartment_height: (base.fire_compartment_height != other.fire_compartment_height).then(|| other.fire_compartment_height),
            fire_opening_factor: (base.fire_opening_factor != other.fire_opening_factor).then(|| other.fire_opening_factor),
            fire_thermal_inertia: (base.fire_thermal_inertia != other.fire_thermal_inertia).then(|| other.fire_thermal_inertia),
            fire_occupancy: (base.fire_occupancy != other.fire_occupancy).then(|| other.fire_occupancy.clone()),
            fire_load_density_qf: (base.fire_load_density_qf != other.fire_load_density_qf).then(|| other.fire_load_density_qf),
            assumed_qf_d: (base.assumed_qf_d != other.assumed_qf_d).then(|| other.assumed_qf_d),
            construction_activity: (base.construction_activity != other.construction_activity).then(|| other.construction_activity.clone()),
            assumed_construction_qk: (base.assumed_construction_qk != other.assumed_construction_qk).then(|| other.assumed_construction_qk),
            structure_kind: (base.structure_kind != other.structure_kind).then(|| other.structure_kind),
            bridge_lane: (base.bridge_lane != other.bridge_lane).then(|| other.bridge_lane),
            bridge_span: (base.bridge_span != other.bridge_span).then(|| other.bridge_span),
            bridge_lane_width: (base.bridge_lane_width != other.bridge_lane_width).then(|| other.bridge_lane_width),
            assumed_bridge_tandem: (base.assumed_bridge_tandem != other.assumed_bridge_tandem).then(|| other.assumed_bridge_tandem),
            assumed_bridge_udl: (base.assumed_bridge_udl != other.assumed_bridge_udl).then(|| other.assumed_bridge_udl),
            assumed_bridge_lm2: (base.assumed_bridge_lm2 != other.assumed_bridge_lm2).then(|| other.assumed_bridge_lm2),
            assumed_bridge_footway: (base.assumed_bridge_footway != other.assumed_bridge_footway).then(|| other.assumed_bridge_footway),
            assumed_bridge_lm3: (base.assumed_bridge_lm3 != other.assumed_bridge_lm3).then(|| other.assumed_bridge_lm3),
            assumed_bridge_lm4: (base.assumed_bridge_lm4 != other.assumed_bridge_lm4).then(|| other.assumed_bridge_lm4),
            bridge_load_group: (base.bridge_load_group != other.bridge_load_group).then(|| other.bridge_load_group.clone()),
            crane_claimed: (base.crane_claimed != other.crane_claimed).then(|| other.crane_claimed),
            crane_class: (base.crane_class != other.crane_class).then(|| other.crane_class.clone()),
            hoist_class: (base.hoist_class != other.hoist_class).then(|| other.hoist_class.clone()),
            hoisting_speed: (base.hoisting_speed != other.hoisting_speed).then(|| other.hoisting_speed),
            assumed_crane_wheel: (base.assumed_crane_wheel != other.assumed_crane_wheel).then(|| other.assumed_crane_wheel),
            assumed_crane_horizontal: (base.assumed_crane_horizontal != other.assumed_crane_horizontal).then(|| other.assumed_crane_horizontal),
            silo_claimed: (base.silo_claimed != other.silo_claimed).then(|| other.silo_claimed),
            silo_kind: (base.silo_kind != other.silo_kind).then(|| other.silo_kind.clone()),
            silo_bulk_density: (base.silo_bulk_density != other.silo_bulk_density).then(|| other.silo_bulk_density),
            silo_height: (base.silo_height != other.silo_height).then(|| other.silo_height),
            silo_hydraulic_radius: (base.silo_hydraulic_radius != other.silo_hydraulic_radius).then(|| other.silo_hydraulic_radius),
            silo_mu: (base.silo_mu != other.silo_mu).then(|| other.silo_mu),
            silo_k: (base.silo_k != other.silo_k).then(|| other.silo_k),
            assumed_silo_pressure: (base.assumed_silo_pressure != other.assumed_silo_pressure).then(|| other.assumed_silo_pressure),
            assumed_silo_patch: (base.assumed_silo_patch != other.assumed_silo_patch).then(|| other.assumed_silo_patch),
            assumed_silo_wall_friction: (base.assumed_silo_wall_friction != other.assumed_silo_wall_friction).then(|| other.assumed_silo_wall_friction),
            floors: En1991FloorDelta::between(&base.floors, &other.floors),
            self_weight_elements: En1991SelfWeightElementDelta::between(&base.self_weight_elements, &other.self_weight_elements),
            roofs: En1991RoofDelta::between(&base.roofs, &other.roofs),
            wind_faces: En1991WindFaceDelta::between(&base.wind_faces, &other.wind_faces),
            accidental_cases: En1991AccidentalCaseDelta::between(&base.accidental_cases, &other.accidental_cases),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.snow_zone.is_none()
            && self.altitude.is_none()
            && self.en_sk.is_none()
            && self.north_german_lowland_snow.is_none()
            && self.wind_zone.is_none()
            && self.en_vb.is_none()
            && self.terrain_category.is_none()
            && self.mixed_terrain_upwind.is_none()
            && self.mixed_terrain_distance.is_none()
            && self.orography_factor.is_none()
            && self.coast_or_island.is_none()
            && self.air_density.is_none()
            && self.height.is_none()
            && self.width.is_none()
            && self.depth.is_none()
            && self.assumed_delta_t.is_none()
            && self.t_max.is_none()
            && self.t_min.is_none()
            && self.t_0.is_none()
            && self.thermal_element_type.is_none()
            && self.thermal_bridge_type.is_none()
            && self.delta_t_m.is_none()
            && self.storey_count.is_none()
            && self.fire_mode.is_none()
            && self.fire_curve.is_none()
            && self.fire_duration.is_none()
            && self.assumed_gas_temperature.is_none()
            && self.assumed_h_net.is_none()
            && self.fire_compartment_area.is_none()
            && self.fire_compartment_height.is_none()
            && self.fire_opening_factor.is_none()
            && self.fire_thermal_inertia.is_none()
            && self.fire_occupancy.is_none()
            && self.fire_load_density_qf.is_none()
            && self.assumed_qf_d.is_none()
            && self.construction_activity.is_none()
            && self.assumed_construction_qk.is_none()
            && self.structure_kind.is_none()
            && self.bridge_lane.is_none()
            && self.bridge_span.is_none()
            && self.bridge_lane_width.is_none()
            && self.assumed_bridge_tandem.is_none()
            && self.assumed_bridge_udl.is_none()
            && self.assumed_bridge_lm2.is_none()
            && self.assumed_bridge_footway.is_none()
            && self.assumed_bridge_lm3.is_none()
            && self.assumed_bridge_lm4.is_none()
            && self.bridge_load_group.is_none()
            && self.crane_claimed.is_none()
            && self.crane_class.is_none()
            && self.hoist_class.is_none()
            && self.hoisting_speed.is_none()
            && self.assumed_crane_wheel.is_none()
            && self.assumed_crane_horizontal.is_none()
            && self.silo_claimed.is_none()
            && self.silo_kind.is_none()
            && self.silo_bulk_density.is_none()
            && self.silo_height.is_none()
            && self.silo_hydraulic_radius.is_none()
            && self.silo_mu.is_none()
            && self.silo_k.is_none()
            && self.assumed_silo_pressure.is_none()
            && self.assumed_silo_patch.is_none()
            && self.assumed_silo_wall_friction.is_none()
            && self.floors.is_empty()
            && self.self_weight_elements.is_empty()
            && self.roofs.is_empty()
            && self.wind_faces.is_empty()
            && self.accidental_cases.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
