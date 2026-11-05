//! 🧬️ En1991 artifact schema — every field of the artifact with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🔖️Artifact
use semio_s_plugin_norm_compliance::AnnexChoice;
use framework_schema::ArtifactSchema;

/// 🧬️ Full En1991 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub snow_zone: String,
    #[state(artifact)]
    pub altitude: f64,
    #[state(artifact)]
    pub en_sk: f64,
    #[state(artifact)]
    pub north_german_lowland_snow: bool,
    #[state(artifact)]
    pub wind_zone: u8,
    #[state(artifact)]
    pub en_vb: f64,
    #[state(artifact)]
    pub terrain_category: u8,
    #[state(artifact)]
    pub mixed_terrain_upwind: u8,
    #[state(artifact)]
    pub mixed_terrain_distance: f64,
    #[state(artifact)]
    pub orography_factor: f64,
    #[state(artifact)]
    pub coast_or_island: bool,
    #[state(artifact)]
    pub air_density: f64,
    #[state(artifact)]
    pub height: f64,
    #[state(artifact)]
    pub width: f64,
    #[state(artifact)]
    pub depth: f64,
    #[state(artifact)]
    pub assumed_delta_t: f64,
    #[state(artifact)]
    pub t_max: f64,
    #[state(artifact)]
    pub t_min: f64,
    #[state(artifact)]
    pub t_0: f64,
    #[state(artifact)]
    pub thermal_element_type: String,
    #[state(artifact)]
    pub thermal_bridge_type: u8,
    #[state(artifact)]
    pub delta_t_m: f64,
    #[state(artifact)]
    pub storey_count: u8,
    #[state(artifact)]
    pub fire_mode: semio_s_artifact_norm_en1991_model::FireMode,
    #[state(artifact)]
    pub fire_curve: semio_s_artifact_norm_en1991_model::part_1_2::FireCurve,
    #[state(artifact)]
    pub fire_duration: f64,
    #[state(artifact)]
    pub assumed_gas_temperature: f64,
    #[state(artifact)]
    pub assumed_h_net: f64,
    #[state(artifact)]
    pub fire_compartment_area: f64,
    #[state(artifact)]
    pub fire_compartment_height: f64,
    #[state(artifact)]
    pub fire_opening_factor: f64,
    #[state(artifact)]
    pub fire_thermal_inertia: f64,
    #[state(artifact)]
    pub fire_occupancy: semio_s_artifact_norm_en1991_model::FireOccupancy,
    #[state(artifact)]
    pub fire_load_density_qf: f64,
    #[state(artifact)]
    pub assumed_qf_d: f64,
    #[state(artifact)]
    pub construction_activity: String,
    #[state(artifact)]
    pub assumed_construction_qk: f64,
    #[state(artifact)]
    pub structure_kind: semio_s_artifact_norm_en1991_model::StructureKind,
    #[state(artifact)]
    pub bridge_lane: u8,
    #[state(artifact)]
    pub bridge_span: f64,
    #[state(artifact)]
    pub bridge_lane_width: f64,
    #[state(artifact)]
    pub assumed_bridge_tandem: f64,
    #[state(artifact)]
    pub assumed_bridge_udl: f64,
    #[state(artifact)]
    pub assumed_bridge_lm2: f64,
    #[state(artifact)]
    pub assumed_bridge_footway: f64,
    #[state(artifact)]
    pub assumed_bridge_lm3: f64,
    #[state(artifact)]
    pub assumed_bridge_lm4: f64,
    #[state(artifact)]
    pub bridge_load_group: String,
    #[state(artifact)]
    pub crane_claimed: bool,
    #[state(artifact)]
    pub crane_class: String,
    #[state(artifact)]
    pub hoist_class: String,
    #[state(artifact)]
    pub hoisting_speed: f64,
    #[state(artifact)]
    pub assumed_crane_wheel: f64,
    #[state(artifact)]
    pub assumed_crane_horizontal: f64,
    #[state(artifact)]
    pub silo_claimed: bool,
    #[state(artifact)]
    pub silo_kind: String,
    #[state(artifact)]
    pub silo_bulk_density: f64,
    #[state(artifact)]
    pub silo_height: f64,
    #[state(artifact)]
    pub silo_hydraulic_radius: f64,
    #[state(artifact)]
    pub silo_mu: f64,
    #[state(artifact)]
    pub silo_k: f64,
    #[state(artifact)]
    pub assumed_silo_pressure: f64,
    #[state(artifact)]
    pub assumed_silo_patch: f64,
    #[state(artifact)]
    pub assumed_silo_wall_friction: f64,
    #[state(artifact)]
    pub bridge_tandem_x: f64,
    #[state(artifact)]
    pub rail_alpha: f64,
    #[state(artifact)]
    pub assumed_rail_lm71: f64,
    #[state(artifact)]
    pub assumed_rail_lm71_udl: f64,
    #[state(artifact)]
    pub assumed_rail_sw0: f64,
    #[state(artifact)]
    pub assumed_rail_sw2: f64,
    #[state(artifact)]
    pub crane_x: f64,
    #[state(artifact)]
    pub crane_runway_length: f64,
    #[state(artifact)]
    pub crane_wheel_base: f64,
    #[state(artifact)]
    pub silo_diameter: f64,
    #[state(artifact)]
    pub silo_eccentricity: f64,
    #[state(artifact)]
    pub silo_hopper_angle_deg: f64,
    #[state(artifact)]
    pub bridge_curve_radius: f64,
    #[state(artifact)]
    pub fire_load_basis: semio_s_artifact_norm_en1991_model::FireLoadBasis,
    #[state(artifact)]
    pub floors: Vec<semio_s_artifact_norm_en1991_model::FloorArea>,
    #[state(artifact)]
    pub self_weight_elements: Vec<semio_s_artifact_norm_en1991_model::SelfWeightElement>,
    #[state(artifact)]
    pub roofs: Vec<semio_s_artifact_norm_en1991_model::RoofArea>,
    #[state(artifact)]
    pub wind_faces: Vec<semio_s_artifact_norm_en1991_model::WindFace>,
    #[state(artifact)]
    pub accidental_cases: Vec<semio_s_artifact_norm_en1991_model::AccidentalCase>,
    #[state(artifact)]
    pub bridge_lanes: Vec<semio_s_artifact_norm_en1991_model::BridgeLane>,
    #[state(artifact)]
    pub construction_stages: Vec<semio_s_artifact_norm_en1991_model::ConstructionStage>,
    #[state(artifact)]
    pub roof_surfaces: Vec<semio_s_artifact_norm_en1991_model::RoofSurface>,
    #[state(artifact)]
    pub placements: Vec<semio_s_artifact_norm_en1991_model::Placement>,
    #[state(artifact)]
    pub special_vehicles: Vec<semio_s_artifact_norm_en1991_model::SpecialVehicle>,
    #[state(artifact)]
    pub wind_bridges: Vec<semio_s_artifact_norm_en1991_model::WindBridge>,
    #[state(artifact)]
    pub dynamic_elements: Vec<semio_s_artifact_norm_en1991_model::DynamicElement>,
    #[state(artifact)]
    pub impact_actions: Vec<semio_s_artifact_norm_en1991_model::ImpactAction>,
    #[state(artifact)]
    pub internal_explosions: Vec<semio_s_artifact_norm_en1991_model::InternalExplosion>,
    #[state(artifact)]
    pub thermal_sections: Vec<semio_s_artifact_norm_en1991_model::ThermalSection>,
    #[state(artifact)]
    pub cases: Vec<semio_s_artifact_norm_en1991_model::CaseRow>,
    #[state(artifact)]
    pub helicopter_landings: Vec<semio_s_artifact_norm_en1991_model::HelicopterLanding>,
    #[state(artifact)]
    pub tie_runs: Vec<semio_s_artifact_norm_en1991_model::TieRun>,
    #[state(artifact)]
    pub construction_loads: Vec<semio_s_artifact_norm_en1991_model::ConstructionLoad>,
    #[state(artifact)]
    pub falling_loads: Vec<semio_s_artifact_norm_en1991_model::FallingLoad>,
    #[state(artifact)]
    pub execution_imperfections: Vec<semio_s_artifact_norm_en1991_model::ExecutionImperfection>,
    #[state(artifact)]
    pub derailment_cases: Vec<semio_s_artifact_norm_en1991_model::DerailmentCase>,
    #[state(artifact)]
    pub crane_hoists: Vec<semio_s_artifact_norm_en1991_model::CraneHoist>,
    #[state(artifact)]
    pub footbridges: Vec<semio_s_artifact_norm_en1991_model::Footbridge>,
    #[state(artifact)]
    pub rail_dynamics: Vec<semio_s_artifact_norm_en1991_model::RailDynamic>,
    #[state(artifact)]
    pub load_dispersals: Vec<semio_s_artifact_norm_en1991_model::LoadDispersal>,
    #[state(artifact)]
    pub bulk_solids: Vec<semio_s_artifact_norm_en1991_model::BulkSolid>,
    #[state(artifact)]
    pub rotating_machines: Vec<semio_s_artifact_norm_en1991_model::RotatingMachine>,
    #[state(artifact)]
    pub fatigue_models: Vec<semio_s_artifact_norm_en1991_model::FatigueModel>,
    #[state(artifact)]
    pub support_collisions: Vec<semio_s_artifact_norm_en1991_model::SupportCollision>,
    #[state(artifact)]
    pub local_fires: Vec<semio_s_artifact_norm_en1991_model::LocalFire>,
    #[state(artifact)]
    pub snow_guards: Vec<semio_s_artifact_norm_en1991_model::SnowGuard>,
    #[state(artifact)]
    pub rail_derailments: Vec<semio_s_artifact_norm_en1991_model::RailDerailment>,
    #[state(artifact)]
    pub rail_load_groups: Vec<semio_s_artifact_norm_en1991_model::RailLoadGroup>,
    #[state(artifact)]
    pub dust_explosions: Vec<semio_s_artifact_norm_en1991_model::DustExplosion>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1991Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> semio_s_artifact_norm_en1991_model::En1991Snapshot {
        semio_s_artifact_norm_en1991_model::En1991Snapshot {
            annex: self.annex,
            snow_zone: self.snow_zone.clone(),
            altitude: self.altitude,
            en_sk: self.en_sk,
            north_german_lowland_snow: self.north_german_lowland_snow,
            wind_zone: self.wind_zone,
            en_vb: self.en_vb,
            terrain_category: self.terrain_category,
            mixed_terrain_upwind: self.mixed_terrain_upwind,
            mixed_terrain_distance: self.mixed_terrain_distance,
            orography_factor: self.orography_factor,
            coast_or_island: self.coast_or_island,
            air_density: self.air_density,
            height: self.height,
            width: self.width,
            depth: self.depth,
            assumed_delta_t: self.assumed_delta_t,
            t_max: self.t_max,
            t_min: self.t_min,
            t_0: self.t_0,
            thermal_element_type: self.thermal_element_type.clone(),
            thermal_bridge_type: self.thermal_bridge_type,
            delta_t_m: self.delta_t_m,
            storey_count: self.storey_count,
            fire_mode: self.fire_mode,
            fire_curve: self.fire_curve,
            fire_duration: self.fire_duration,
            assumed_gas_temperature: self.assumed_gas_temperature,
            assumed_h_net: self.assumed_h_net,
            fire_compartment_area: self.fire_compartment_area,
            fire_compartment_height: self.fire_compartment_height,
            fire_opening_factor: self.fire_opening_factor,
            fire_thermal_inertia: self.fire_thermal_inertia,
            fire_occupancy: self.fire_occupancy.clone(),
            fire_load_density_qf: self.fire_load_density_qf,
            assumed_qf_d: self.assumed_qf_d,
            construction_activity: self.construction_activity.clone(),
            assumed_construction_qk: self.assumed_construction_qk,
            structure_kind: self.structure_kind,
            bridge_lane: self.bridge_lane,
            bridge_span: self.bridge_span,
            bridge_lane_width: self.bridge_lane_width,
            assumed_bridge_tandem: self.assumed_bridge_tandem,
            assumed_bridge_udl: self.assumed_bridge_udl,
            assumed_bridge_lm2: self.assumed_bridge_lm2,
            assumed_bridge_footway: self.assumed_bridge_footway,
            assumed_bridge_lm3: self.assumed_bridge_lm3,
            assumed_bridge_lm4: self.assumed_bridge_lm4,
            bridge_load_group: self.bridge_load_group.clone(),
            crane_claimed: self.crane_claimed,
            crane_class: self.crane_class.clone(),
            hoist_class: self.hoist_class.clone(),
            hoisting_speed: self.hoisting_speed,
            assumed_crane_wheel: self.assumed_crane_wheel,
            assumed_crane_horizontal: self.assumed_crane_horizontal,
            silo_claimed: self.silo_claimed,
            silo_kind: self.silo_kind.clone(),
            silo_bulk_density: self.silo_bulk_density,
            silo_height: self.silo_height,
            silo_hydraulic_radius: self.silo_hydraulic_radius,
            silo_mu: self.silo_mu,
            silo_k: self.silo_k,
            assumed_silo_pressure: self.assumed_silo_pressure,
            assumed_silo_patch: self.assumed_silo_patch,
            assumed_silo_wall_friction: self.assumed_silo_wall_friction,
            bridge_tandem_x: self.bridge_tandem_x,
            rail_alpha: self.rail_alpha,
            assumed_rail_lm71: self.assumed_rail_lm71,
            assumed_rail_lm71_udl: self.assumed_rail_lm71_udl,
            assumed_rail_sw0: self.assumed_rail_sw0,
            assumed_rail_sw2: self.assumed_rail_sw2,
            crane_x: self.crane_x,
            crane_runway_length: self.crane_runway_length,
            crane_wheel_base: self.crane_wheel_base,
            silo_diameter: self.silo_diameter,
            silo_eccentricity: self.silo_eccentricity,
            silo_hopper_angle_deg: self.silo_hopper_angle_deg,
            bridge_curve_radius: self.bridge_curve_radius,
            fire_load_basis: self.fire_load_basis,
            floors: self.floors.clone(),
            self_weight_elements: self.self_weight_elements.clone(),
            roofs: self.roofs.clone(),
            wind_faces: self.wind_faces.clone(),
            accidental_cases: self.accidental_cases.clone(),
            bridge_lanes: self.bridge_lanes.clone(),
            construction_stages: self.construction_stages.clone(),
            roof_surfaces: self.roof_surfaces.clone(),
            placements: self.placements.clone(),
            special_vehicles: self.special_vehicles.clone(),
            wind_bridges: self.wind_bridges.clone(),
            dynamic_elements: self.dynamic_elements.clone(),
            impact_actions: self.impact_actions.clone(),
            internal_explosions: self.internal_explosions.clone(),
            thermal_sections: self.thermal_sections.clone(),
            cases: self.cases.clone(),
            helicopter_landings: self.helicopter_landings.clone(),
            tie_runs: self.tie_runs.clone(),
            construction_loads: self.construction_loads.clone(),
            falling_loads: self.falling_loads.clone(),
            execution_imperfections: self.execution_imperfections.clone(),
            derailment_cases: self.derailment_cases.clone(),
            crane_hoists: self.crane_hoists.clone(),
            footbridges: self.footbridges.clone(),
            rail_dynamics: self.rail_dynamics.clone(),
            load_dispersals: self.load_dispersals.clone(),
            bulk_solids: self.bulk_solids.clone(),
            rotating_machines: self.rotating_machines.clone(),
            dust_explosions: self.dust_explosions.clone(),
            rail_load_groups: self.rail_load_groups.clone(),
            rail_derailments: self.rail_derailments.clone(),
            fatigue_models: self.fatigue_models.clone(),
            support_collisions: self.support_collisions.clone(),
            local_fires: self.local_fires.clone(),
            snow_guards: self.snow_guards.clone(),
        }
    }
    /// 🧬️ Lift a snapshot into the full artifact lanes.
    pub fn from_snapshot(snapshot: semio_s_artifact_norm_en1991_model::En1991Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            snow_zone: snapshot.snow_zone.clone(),
            altitude: snapshot.altitude,
            en_sk: snapshot.en_sk,
            north_german_lowland_snow: snapshot.north_german_lowland_snow,
            wind_zone: snapshot.wind_zone,
            en_vb: snapshot.en_vb,
            terrain_category: snapshot.terrain_category,
            mixed_terrain_upwind: snapshot.mixed_terrain_upwind,
            mixed_terrain_distance: snapshot.mixed_terrain_distance,
            orography_factor: snapshot.orography_factor,
            coast_or_island: snapshot.coast_or_island,
            air_density: snapshot.air_density,
            height: snapshot.height,
            width: snapshot.width,
            depth: snapshot.depth,
            assumed_delta_t: snapshot.assumed_delta_t,
            t_max: snapshot.t_max,
            t_min: snapshot.t_min,
            t_0: snapshot.t_0,
            thermal_element_type: snapshot.thermal_element_type.clone(),
            thermal_bridge_type: snapshot.thermal_bridge_type,
            delta_t_m: snapshot.delta_t_m,
            storey_count: snapshot.storey_count,
            fire_mode: snapshot.fire_mode,
            fire_curve: snapshot.fire_curve,
            fire_duration: snapshot.fire_duration,
            assumed_gas_temperature: snapshot.assumed_gas_temperature,
            assumed_h_net: snapshot.assumed_h_net,
            fire_compartment_area: snapshot.fire_compartment_area,
            fire_compartment_height: snapshot.fire_compartment_height,
            fire_opening_factor: snapshot.fire_opening_factor,
            fire_thermal_inertia: snapshot.fire_thermal_inertia,
            fire_occupancy: snapshot.fire_occupancy.clone(),
            fire_load_density_qf: snapshot.fire_load_density_qf,
            assumed_qf_d: snapshot.assumed_qf_d,
            construction_activity: snapshot.construction_activity.clone(),
            assumed_construction_qk: snapshot.assumed_construction_qk,
            structure_kind: snapshot.structure_kind,
            bridge_lane: snapshot.bridge_lane,
            bridge_span: snapshot.bridge_span,
            bridge_lane_width: snapshot.bridge_lane_width,
            assumed_bridge_tandem: snapshot.assumed_bridge_tandem,
            assumed_bridge_udl: snapshot.assumed_bridge_udl,
            assumed_bridge_lm2: snapshot.assumed_bridge_lm2,
            assumed_bridge_footway: snapshot.assumed_bridge_footway,
            assumed_bridge_lm3: snapshot.assumed_bridge_lm3,
            assumed_bridge_lm4: snapshot.assumed_bridge_lm4,
            bridge_load_group: snapshot.bridge_load_group.clone(),
            crane_claimed: snapshot.crane_claimed,
            crane_class: snapshot.crane_class.clone(),
            hoist_class: snapshot.hoist_class.clone(),
            hoisting_speed: snapshot.hoisting_speed,
            assumed_crane_wheel: snapshot.assumed_crane_wheel,
            assumed_crane_horizontal: snapshot.assumed_crane_horizontal,
            silo_claimed: snapshot.silo_claimed,
            silo_kind: snapshot.silo_kind.clone(),
            silo_bulk_density: snapshot.silo_bulk_density,
            silo_height: snapshot.silo_height,
            silo_hydraulic_radius: snapshot.silo_hydraulic_radius,
            silo_mu: snapshot.silo_mu,
            silo_k: snapshot.silo_k,
            assumed_silo_pressure: snapshot.assumed_silo_pressure,
            assumed_silo_patch: snapshot.assumed_silo_patch,
            assumed_silo_wall_friction: snapshot.assumed_silo_wall_friction,
            bridge_tandem_x: snapshot.bridge_tandem_x,
            rail_alpha: snapshot.rail_alpha,
            assumed_rail_lm71: snapshot.assumed_rail_lm71,
            assumed_rail_lm71_udl: snapshot.assumed_rail_lm71_udl,
            assumed_rail_sw0: snapshot.assumed_rail_sw0,
            assumed_rail_sw2: snapshot.assumed_rail_sw2,
            crane_x: snapshot.crane_x,
            crane_runway_length: snapshot.crane_runway_length,
            crane_wheel_base: snapshot.crane_wheel_base,
            silo_diameter: snapshot.silo_diameter,
            silo_eccentricity: snapshot.silo_eccentricity,
            silo_hopper_angle_deg: snapshot.silo_hopper_angle_deg,
            bridge_curve_radius: snapshot.bridge_curve_radius,
            fire_load_basis: snapshot.fire_load_basis,
            floors: snapshot.floors.clone(),
            self_weight_elements: snapshot.self_weight_elements.clone(),
            roofs: snapshot.roofs.clone(),
            wind_faces: snapshot.wind_faces.clone(),
            accidental_cases: snapshot.accidental_cases.clone(),
            bridge_lanes: snapshot.bridge_lanes.clone(),
            construction_stages: snapshot.construction_stages.clone(),
            roof_surfaces: snapshot.roof_surfaces.clone(),
            placements: snapshot.placements.clone(),
            special_vehicles: snapshot.special_vehicles.clone(),
            wind_bridges: snapshot.wind_bridges.clone(),
            dynamic_elements: snapshot.dynamic_elements.clone(),
            impact_actions: snapshot.impact_actions.clone(),
            internal_explosions: snapshot.internal_explosions.clone(),
            thermal_sections: snapshot.thermal_sections.clone(),
            cases: snapshot.cases.clone(),
            helicopter_landings: snapshot.helicopter_landings.clone(),
            tie_runs: snapshot.tie_runs.clone(),
            construction_loads: snapshot.construction_loads.clone(),
            falling_loads: snapshot.falling_loads.clone(),
            execution_imperfections: snapshot.execution_imperfections.clone(),
            derailment_cases: snapshot.derailment_cases.clone(),
            crane_hoists: snapshot.crane_hoists.clone(),
            footbridges: snapshot.footbridges.clone(),
            rail_dynamics: snapshot.rail_dynamics.clone(),
            load_dispersals: snapshot.load_dispersals.clone(),
            bulk_solids: snapshot.bulk_solids.clone(),
            rotating_machines: snapshot.rotating_machines.clone(),
            dust_explosions: snapshot.dust_explosions.clone(),
            rail_load_groups: snapshot.rail_load_groups.clone(),
            rail_derailments: snapshot.rail_derailments.clone(),
            fatigue_models: snapshot.fatigue_models.clone(),
            support_collisions: snapshot.support_collisions.clone(),
            local_fires: snapshot.local_fires.clone(),
            snow_guards: snapshot.snow_guards.clone(),
        }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1991` — twenty handcrafted schema leaves.
pub fn en1991_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1991",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1991_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1991_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1991_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

