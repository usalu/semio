//! 🧬️ En1991 snapshot schema — complete design-load-assumption subject.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 🔥 Occupancy class of the fire compartment: the nine rows of the fire load Table E.4 of EN 1991-1-2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum FireOccupancy {
    Dwelling,
    Office,
    Hospital,
    School,
    Hotel,
    Library,
    Shopping,
    Theatre,
    Transport,
}

/// 📸️ Persisted EN 1991 subject: site + building geometry + design load assumptions (SI: m, Pa, N, K, kg/m³).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1991", layout = "lines")]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Snapshot {
    #[state(artifact)]
    pub annex: semio_s_plugin_norm_compliance::AnnexChoice,
    #[state(artifact)]
    pub snow_zone: String,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub altitude: f64,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub en_sk: f64,
    #[state(artifact)]
    pub north_german_lowland_snow: bool,
    #[state(artifact)]
    pub wind_zone: u8,
    #[dsl(unit = "m/s")]
    #[state(artifact)]
    pub en_vb: f64,
    #[state(artifact)]
    pub terrain_category: u8,
    #[state(artifact)]
    pub mixed_terrain_upwind: u8,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub mixed_terrain_distance: f64,
    #[state(artifact)]
    pub orography_factor: f64,
    #[state(artifact)]
    pub coast_or_island: bool,
    #[dsl(unit = "kg/m3")]
    #[state(artifact)]
    pub air_density: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub height: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub width: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub depth: f64,
    #[dsl(unit = "K")]
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
    #[dsl(unit = "K")]
    #[state(artifact)]
    pub delta_t_m: f64,
    #[state(artifact)]
    pub storey_count: u8,
    #[state(artifact)]
    pub fire_mode: crate::FireMode,
    #[state(artifact)]
    pub fire_curve: crate::part_1_2::FireCurve,
    #[dsl(unit = "s")]
    #[state(artifact)]
    pub fire_duration: f64,
    #[dsl(unit = "K")]
    #[state(artifact)]
    pub assumed_gas_temperature: f64,
    #[state(artifact)]
    pub assumed_h_net: f64,
    #[dsl(unit = "m2")]
    #[state(artifact)]
    pub fire_compartment_area: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub fire_compartment_height: f64,
    #[state(artifact)]
    pub fire_opening_factor: f64,
    #[state(artifact)]
    pub fire_thermal_inertia: f64,
    #[state(artifact)]
    pub fire_occupancy: FireOccupancy,
    #[state(artifact)]
    pub fire_load_density_qf: f64,
    #[state(artifact)]
    pub assumed_qf_d: f64,
    #[state(artifact)]
    pub construction_activity: String,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub assumed_construction_qk: f64,
    #[state(artifact)]
    pub structure_kind: crate::StructureKind,
    #[state(artifact)]
    pub bridge_lane: u8,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_span: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_lane_width: f64,
    #[dsl(unit = "N")]
    #[state(artifact)]
    pub assumed_bridge_tandem: f64,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub assumed_bridge_udl: f64,
    #[dsl(unit = "N")]
    #[state(artifact)]
    pub assumed_bridge_lm2: f64,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub assumed_bridge_footway: f64,
    #[dsl(unit = "N")]
    #[state(artifact)]
    pub assumed_bridge_lm3: f64,
    #[dsl(unit = "Pa")]
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
    #[dsl(unit = "m/s")]
    #[state(artifact)]
    pub hoisting_speed: f64,
    #[dsl(unit = "N")]
    #[state(artifact)]
    pub assumed_crane_wheel: f64,
    #[dsl(unit = "N")]
    #[state(artifact)]
    pub assumed_crane_horizontal: f64,
    #[state(artifact)]
    pub silo_claimed: bool,
    #[state(artifact)]
    pub silo_kind: String,
    #[state(artifact)]
    pub silo_bulk_density: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_height: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_hydraulic_radius: f64,
    #[state(artifact)]
    pub silo_mu: f64,
    #[state(artifact)]
    pub silo_k: f64,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub assumed_silo_pressure: f64,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub assumed_silo_patch: f64,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub assumed_silo_wall_friction: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_tandem_x: f64,
    #[state(artifact)]
    pub rail_alpha: f64,
    #[dsl(unit = "N")]
    #[state(artifact)]
    pub assumed_rail_lm71: f64,
    #[dsl(unit = "N/m")]
    #[state(artifact)]
    pub assumed_rail_lm71_udl: f64,
    #[dsl(unit = "N/m")]
    #[state(artifact)]
    pub assumed_rail_sw0: f64,
    #[dsl(unit = "N/m")]
    #[state(artifact)]
    pub assumed_rail_sw2: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub crane_x: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub crane_runway_length: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub crane_wheel_base: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_diameter: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_eccentricity: f64,
    #[state(artifact)]
    pub silo_hopper_angle_deg: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_curve_radius: f64,
    #[state(artifact)]
    pub fire_load_basis: crate::FireLoadBasis,
    #[dsl(table)]
    #[state(artifact)]
    pub floors: Vec<FloorArea>,
    #[dsl(table)]
    #[state(artifact)]
    pub self_weight_elements: Vec<SelfWeightElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub roofs: Vec<RoofArea>,
    #[dsl(table)]
    #[state(artifact)]
    pub wind_faces: Vec<WindFace>,
    #[dsl(table)]
    #[state(artifact)]
    pub accidental_cases: Vec<crate::AccidentalCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub bridge_lanes: Vec<BridgeLane>,
    #[dsl(table)]
    #[state(artifact)]
    pub construction_stages: Vec<ConstructionStage>,
    #[dsl(table)]
    #[state(artifact)]
    pub roof_surfaces: Vec<RoofSurface>,
    #[dsl(table)]
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[dsl(table)]
    #[state(artifact)]
    pub special_vehicles: Vec<SpecialVehicle>,
    #[dsl(table)]
    #[state(artifact)]
    pub wind_bridges: Vec<WindBridge>,
    #[dsl(table)]
    #[state(artifact)]
    pub dynamic_elements: Vec<DynamicElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub impact_actions: Vec<ImpactAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub internal_explosions: Vec<InternalExplosion>,
    #[dsl(table)]
    #[state(artifact)]
    pub thermal_sections: Vec<ThermalSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub cases: Vec<CaseRow>,
    #[dsl(table)]
    #[state(artifact)]
    pub helicopter_landings: Vec<HelicopterLanding>,
    #[dsl(table)]
    #[state(artifact)]
    pub tie_runs: Vec<TieRun>,
    #[dsl(table)]
    #[state(artifact)]
    pub construction_loads: Vec<ConstructionLoad>,
    #[dsl(table)]
    #[state(artifact)]
    pub falling_loads: Vec<FallingLoad>,
    #[dsl(table)]
    #[state(artifact)]
    pub execution_imperfections: Vec<ExecutionImperfection>,
    #[dsl(table)]
    #[state(artifact)]
    pub derailment_cases: Vec<DerailmentCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub crane_hoists: Vec<CraneHoist>,
    #[dsl(table)]
    #[state(artifact)]
    pub footbridges: Vec<Footbridge>,
    #[dsl(table)]
    #[state(artifact)]
    pub rail_dynamics: Vec<RailDynamic>,
    #[dsl(table)]
    #[state(artifact)]
    pub load_dispersals: Vec<LoadDispersal>,
    #[dsl(table)]
    #[state(artifact)]
    pub bulk_solids: Vec<BulkSolid>,
    #[dsl(table)]
    #[state(artifact)]
    pub rotating_machines: Vec<RotatingMachine>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_models: Vec<FatigueModel>,
    #[dsl(table)]
    #[state(artifact)]
    pub support_collisions: Vec<SupportCollision>,
    #[dsl(table)]
    #[state(artifact)]
    pub local_fires: Vec<LocalFire>,
    #[dsl(table)]
    #[state(artifact)]
    pub snow_guards: Vec<SnowGuard>,
    #[dsl(table)]
    #[state(artifact)]
    pub rail_derailments: Vec<RailDerailment>,
    #[dsl(table)]
    #[state(artifact)]
    pub rail_load_groups: Vec<RailLoadGroup>,
    #[dsl(table)]
    #[state(artifact)]
    pub dust_explosions: Vec<DustExplosion>,
}

impl En1991Snapshot {
    /// 🎪️ The full showcase: one bridge-kind document that populates every one of the 28 collections, decoded from the committed fixture `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (generated by the ticket script `🧰️scripts/showcase/en1991.py`).
    pub fn full_showcase() -> Self {
        semio_framework_pack_json::from_json_str(include_str!("../../🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed showcase fixture decodes")
    }

    /// 📸️ Reference EN 1991 subject under the explicitly chosen national annex; the annex is never defaulted.
    pub fn example(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
        Self {
            annex,
            snow_zone: "2".into(),
            altitude: 150.0,
            en_sk: 850.0,
            north_german_lowland_snow: false,
            wind_zone: 2,
            en_vb: 25.0,
            terrain_category: 2,
            mixed_terrain_upwind: 2,
            mixed_terrain_distance: 0.0,
            orography_factor: 1.0,
            coast_or_island: false,
            air_density: 1.25,
            height: 20.0,
            width: 15.0,
            depth: 25.0,
            assumed_delta_t: 40.0,
            t_max: 37.0,
            t_min: -24.0,
            t_0: 10.0,
            thermal_element_type: "building".into(),
            thermal_bridge_type: 1,
            delta_t_m: 0.0,
            storey_count: 3,
            fire_mode: crate::FireMode::None,
            fire_curve: crate::part_1_2::FireCurve::Standard,
            fire_duration: 3600.0,
            assumed_gas_temperature: 1200.0,
            assumed_h_net: 25_000.0,
            fire_compartment_area: 100.0,
            fire_compartment_height: 3.0,
            fire_opening_factor: 0.04,
            fire_thermal_inertia: 1160.0,
            fire_occupancy: FireOccupancy::Office,
            fire_load_density_qf: 420.0e6,
            assumed_qf_d: 420.0e6,
            construction_activity: "scaffolding".into(),
            assumed_construction_qk: 1500.0,
            structure_kind: crate::StructureKind::Building,
            bridge_lane: 1,
            bridge_span: 20.0,
            bridge_lane_width: 3.0,
            assumed_bridge_tandem: 0.0,
            assumed_bridge_udl: 0.0,
            assumed_bridge_lm2: 0.0,
            assumed_bridge_footway: 0.0,
            assumed_bridge_lm3: 0.0,
            assumed_bridge_lm4: 0.0,
            bridge_load_group: "gr1a".into(),
            crane_claimed: false,
            crane_class: "HC2".into(),
            hoist_class: "HC2".into(),
            hoisting_speed: 0.5,
            assumed_crane_wheel: 150_000.0,
            assumed_crane_horizontal: 25_000.0,
            silo_claimed: false,
            silo_kind: "silo".into(),
            silo_bulk_density: 8000.0,
            silo_height: 12.0,
            silo_hydraulic_radius: 1.5,
            silo_mu: 0.4,
            silo_k: 0.4,
            assumed_silo_pressure: 50_000.0,
            assumed_silo_patch: 20_000.0,
            assumed_silo_wall_friction: 15_000.0,
            bridge_tandem_x: 10.0,
            rail_alpha: 1.0,
            assumed_rail_lm71: 250_000.0,
            assumed_rail_lm71_udl: 80_000.0,
            assumed_rail_sw0: 133_000.0,
            assumed_rail_sw2: 150_000.0,
            crane_x: 15.0,
            crane_runway_length: 30.0,
            crane_wheel_base: 3.5,
            silo_diameter: 6.0,
            silo_eccentricity: 0.0,
            silo_hopper_angle_deg: 60.0,
            bridge_curve_radius: 0.0,
            fire_load_basis: crate::FireLoadBasis::OccupancyTable,
floors: vec![FloorArea {
    id: "office-l1".into(),
    category: "B1".into(),
    area: 240.0,
    assumed_qk: 2_500.0,
    assumed_qk_concentrated: 20_000.0,
    assumed_partitions: 800.0,
}],
self_weight_elements: vec![SelfWeightElement {
    id: "slab-rc".into(),
    material: "reinforced_concrete".into(),
    thickness: 0.2,
    assumed_gk: 6_000.0,
}],
roofs: vec![RoofArea {
    id: "roof-main".into(),
    roof_type: "duopitch".into(),
    pitch_deg: 15.0,
    c_e: 1.0,
    c_t: 1.0,
    has_parapet: false,
    parapet_height: 0.0,
    drift_obstruction_height: 0.0,
    multi_span: false,
    assumed_sk: 1_000.0,
}],
wind_faces: vec![WindFace {
    id: "facade-d".into(),
    zone: "D".into(),
    z: 8.0,
    c_pe10: 0.773,
    c_pe1: 1.0,
    c_pi: 0.2,
    c_s: 1.0,
    c_d: 1.0,
    loaded_area: 10.0,
    assumed_wp: 1_200.0,
}],
accidental_cases: vec![],
bridge_lanes: vec![],
construction_stages: vec![],
roof_surfaces: vec![],
placements: vec![],
special_vehicles: vec![],
wind_bridges: vec![],
dynamic_elements: vec![],
impact_actions: vec![],
internal_explosions: vec![],
thermal_sections: vec![],
cases: vec![CaseRow {
            id: "case-e11-04".into(),
            label_en: "Imposed-load reduction of the office floors".into(),
            label_de: "Nutzlastabminderung der Bürogeschosse".into(),
            entry_id: "E11.04".into(),
            numbers: vec![CaseNumber { id: "area".into(), value: 30.0 }, CaseNumber { id: "storeys".into(), value: 4.0 }],
            texts: vec![CaseText { id: "category".into(), value: "A".into() }],
        }],
helicopter_landings: vec![],
tie_runs: vec![],
construction_loads: vec![],
falling_loads: vec![],
execution_imperfections: vec![],
derailment_cases: vec![],
crane_hoists: vec![],
footbridges: vec![],
rail_dynamics: vec![],
load_dispersals: vec![],
bulk_solids: vec![],
rotating_machines: vec![],
dust_explosions: vec![],
rail_load_groups: vec![],
rail_derailments: vec![],
fatigue_models: vec![],
support_collisions: vec![],
local_fires: vec![],
snow_guards: vec![],
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1991_model_typescript::LEAF, graphql: semio_s_artifact_norm_en1991_model_graphql::LEAF, json_schema: semio_s_artifact_norm_en1991_model_json_schema::LEAF, proto: semio_s_artifact_norm_en1991_model_proto::LEAF };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

pub mod part_1_2 {
    /// 🔥️ Fire exposure curve family (EN 1991-1-2 §3.2 nominal / Annex A parametric).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    #[value(rename_all = "camelCase")]
    pub enum FireCurve {
        Standard,
        External,
        Hydrocarbon,
        Parametric,
    }
}

/// 🏢 Floor / use area with occupancy category and assumed imposed loads (Pa, N).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FloorArea {
    pub id: String,
    pub category: String,
    #[dsl(unit = "m2")]
    pub area: f64,
    #[dsl(unit = "Pa")]
    pub assumed_qk: f64,
    #[dsl(unit = "N")]
    pub assumed_qk_concentrated: f64,
    #[dsl(unit = "Pa")]
    pub assumed_partitions: f64,
}

/// 🧱 Structural layer / element self-weight with assumed permanent load (Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SelfWeightElement {
    pub id: String,
    pub material: String,
    #[dsl(unit = "m")]
    pub thickness: f64,
    #[dsl(unit = "Pa")]
    pub assumed_gk: f64,
}

/// 🏠 Roof zone with shape coefficients and assumed snow load (Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RoofArea {
    pub id: String,
    pub roof_type: String,
    pub pitch_deg: f64,
    pub c_e: f64,
    pub c_t: f64,
    pub has_parapet: bool,
    #[dsl(unit = "m")]
    pub parapet_height: f64,
    #[dsl(unit = "m")]
    pub drift_obstruction_height: f64,
    pub multi_span: bool,
    #[dsl(unit = "Pa")]
    pub assumed_sk: f64,
}

/// 🌬️ Façade / roof wind zone with pressure coefficients and assumed wind pressure (Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WindFace {
    pub id: String,
    pub zone: String,
    #[dsl(unit = "m")]
    pub z: f64,
    pub c_pe10: f64,
    pub c_pe1: f64,
    pub c_pi: f64,
    pub c_s: f64,
    pub c_d: f64,
    #[dsl(unit = "m2")]
    pub loaded_area: f64,
    #[dsl(unit = "Pa")]
    pub assumed_wp: f64,
}

/// 🔥 Fire design mode — `None` | nominal curves | Annex A parametric (EN 1991-1-2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum FireMode {
    None,
    Nominal,
    Parametric,
}

/// 🧯 Basis of the characteristic fire load density q_f,k (EN 1991-1-2, Annex E): the 80 % fractile of the occupancy row of Table E.4, or the value the user specifies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum FireLoadBasis {
    OccupancyTable,
    UserSpecified,
}

/// 🏗️ Structure kind — building vs bridge (EN 1991-2 traffic only on bridge).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum StructureKind {
    Building,
    Bridge,
}

/// 🚗 Vehicle impact inputs (EN 1991-1-7 Annex C) — only present on impact rows.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AccidentalImpact {
    #[dsl(unit = "kg")]
    pub vehicle_mass: f64,
    #[dsl(unit = "m/s")]
    pub vehicle_speed: f64,
    #[dsl(unit = "N")]
    pub assumed_force: f64,
}

/// 💥 Explosion inputs (EN 1991-1-7 Annex D) — only present on explosion rows.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AccidentalExplosion {
    #[dsl(unit = "kg")]
    pub explosion_mass: f64,
    #[dsl(unit = "m")]
    pub standoff: f64,
    #[dsl(unit = "Pa")]
    pub assumed_pressure: f64,
}

/// 💥 Accidental design situation — discriminated impact vs explosion (exactly one variant block).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AccidentalCase {
    pub id: String,
    /// 🚗 Impact variant block (empty when explosion).
    pub impact: Vec<AccidentalImpact>,
    /// 💥 Explosion variant block (empty when impact).
    pub explosion: Vec<AccidentalExplosion>,
}

/// 🛣️ Notional lane of the bridge carriageway with its width, lateral offset and LM1 lane number (EN 1991-2, 4.2.3 and Table 4.2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BridgeLane {
    pub id: String,
    #[dsl(unit = "m")]
    pub width: f64,
    #[dsl(unit = "m")]
    pub offset: f64,
    pub lm1_lane: u8,
}

/// 🏗️ Construction stage with its duration, return period of the climatic actions and assumed construction load (EN 1991-1-6, 3.1 and Table 3.1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ConstructionStage {
    pub id: String,
    pub activity: String,
    pub duration_days: f64,
    pub return_years: f64,
    #[dsl(unit = "Pa")]
    pub assumed_qk: f64,
}

/// 🏠 Roof surface of a building face with its shape, pitch and extent, several per face (EN 1991-1-3 Table 5.2, EN 1991-1-4 Table 7.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RoofSurface {
    pub id: String,
    pub face: String,
    pub shape: String,
    #[dsl(unit = "deg")]
    pub pitch_deg: f64,
    #[dsl(unit = "m")]
    pub extent: f64,
}

/// 📍️ Persisted pose of one scene entity (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Placement {
    pub id: String,
    /// ↔️ Origin x [m].
    pub x: f64,
    /// ↕️ Origin y [m].
    pub y: f64,
    /// ⬆️ Origin z [m].
    pub z: f64,
    /// 🧭️ Yaw about +Z [rad].
    pub yaw: f64,
}

/// 🚛 LM3 special vehicle placed on the bridge deck by its Annex A designation total/axle in kN (EN 1991-2, 4.3.4 and Annex A).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SpecialVehicle {
    pub id: String,
    pub designation: String,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
}

/// 🌉 Bridge deck exposed to wind: depth, parapets, traffic and height above ground for the force of EN 1991-1-4 section 8.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WindBridge {
    pub id: String,
    #[dsl(unit = "m")]
    pub deck_width: f64,
    #[dsl(unit = "m")]
    pub deck_depth: f64,
    pub parapet_kind: u8,
    #[dsl(unit = "m")]
    pub parapet_height: f64,
    pub traffic_kind: u8,
    #[dsl(unit = "m")]
    pub height_above_ground: f64,
    #[dsl(unit = "m")]
    pub span: f64,
    pub assumed_force: f64,
}

/// 〰️ Slender element for the structural factor c_s c_d and the vortex shedding check (EN 1991-1-4, 6 and Annexes B, E, F).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DynamicElement {
    pub id: String,
    #[dsl(unit = "m")]
    pub height: f64,
    #[dsl(unit = "m")]
    pub width: f64,
    pub first_frequency: f64,
    pub log_decrement: f64,
    pub mass_per_length: f64,
    pub strouhal: f64,
    pub assumed_cscd: f64,
}

/// 💥 Accidental impact of a vehicle, ship or forklift on a structural member (EN 1991-1-7, 4 and Annex C).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ImpactAction {
    pub id: String,
    pub kind: u8,
    pub mass: f64,
    pub speed: f64,
    pub stiffness: f64,
    #[dsl(unit = "m")]
    pub height: f64,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
    #[dsl(unit = "N")]
    pub assumed_force: f64,
}

/// 🏠 Internal gas explosion of a vented enclosure by its volume and vent area (EN 1991-1-7, Annex D).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InternalExplosion {
    pub id: String,
    pub volume: f64,
    #[dsl(unit = "m2")]
    pub vent_area: f64,
    #[dsl(unit = "Pa")]
    pub vent_pressure: f64,
    #[dsl(unit = "Pa")]
    pub assumed_pressure: f64,
}

/// 🌡️ Bridge cross-section for the uniform and the linear temperature actions (EN 1991-1-5, 6.1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ThermalSection {
    pub id: String,
    pub deck_type: u8,
    pub surfacing: u8,
    #[dsl(unit = "m")]
    pub length: f64,
    #[dsl(unit = "m")]
    pub depth: f64,
    pub alpha_micro: f64,
    pub assumed_range: f64,
}

/// 🚁 Hard landing of a helicopter on a roof pad by its take-off mass (EN 1991-1-7, 4.7).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HelicopterLanding {
    pub id: String,
    pub mass: f64,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
    #[dsl(unit = "m")]
    pub height: f64,
    #[dsl(unit = "N")]
    pub assumed_force: f64,
}

/// 🪢 Horizontal tie of a floor with its spacing, span and loads for the robustness design (EN 1991-1-7, Annex A).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TieRun {
    pub id: String,
    pub kind: u8,
    #[dsl(unit = "m")]
    pub spacing: f64,
    #[dsl(unit = "m")]
    pub span: f64,
    #[dsl(unit = "Pa")]
    pub dead_load: f64,
    #[dsl(unit = "Pa")]
    pub imposed_load: f64,
    pub psi: f64,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
    #[dsl(unit = "N")]
    pub provided_force: f64,
}

/// 🧱 Construction load Q_c,i of a construction stage, summed per stage for the combination of EN 1991-1-6 (4.11, Annex A.2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ConstructionLoad {
    pub id: String,
    pub stage: String,
    pub kind: u8,
    #[dsl(unit = "Pa")]
    pub magnitude: f64,
}

/// 🪨 Accidental falling load or crane impact during execution by its mass, drop height and stopping distance (EN 1991-1-6, accidental construction loads).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FallingLoad {
    pub id: String,
    pub mass: f64,
    #[dsl(unit = "m")]
    pub drop_height: f64,
    #[dsl(unit = "m")]
    pub stopping_distance: f64,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
    #[dsl(unit = "N")]
    pub assumed_force: f64,
}

/// 📐 Geometric imperfection of a member during execution as equivalent horizontal force from its tolerance and vertical load (EN 1991-1-6, imperfections).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ExecutionImperfection {
    pub id: String,
    #[dsl(unit = "m")]
    pub height: f64,
    #[dsl(unit = "m")]
    pub tolerance: f64,
    #[dsl(unit = "N")]
    pub vertical_load: f64,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
    #[dsl(unit = "N")]
    pub assumed_force: f64,
}

/// 🚆 Derailment design situation next to a track by its distance from the track axis and the design forces F_dx, F_dy entered from the applicable table (EN 1991-1-7, 4.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DerailmentCase {
    pub id: String,
    #[dsl(unit = "m")]
    pub distance: f64,
    #[dsl(unit = "N")]
    pub design_force_x: f64,
    #[dsl(unit = "N")]
    pub design_force_y: f64,
    #[dsl(unit = "m")]
    pub x: f64,
    #[dsl(unit = "m")]
    pub y: f64,
    #[dsl(unit = "N")]
    pub assumed_force: f64,
}

/// Hoist of the crane with its rated load Q_H and the payload fraction released suddenly (EN 1991-3 2.5 and 2.9): the dynamic factor phi_3 = 1 - dm/m (1 + beta_3) and the test loads.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CraneHoist {
    pub id: String,
    #[dsl(unit = "N")]
    pub rated_load: f64,
    pub release_fraction: f64,
    pub release_device: String,
}

/// Footbridge deck with loaded length, width and first vertical natural frequency: uniform load q_fk = 2.0 + 120/(L + 30) kN/m² within 2.5 to 5.0 (EN 1991-2, 5.3.2.1), horizontal force Q_flk and the walking-resonance range of the vertical mode (5.7).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Footbridge {
    pub id: String,
    #[dsl(unit = "m")]
    pub loaded_length: f64,
    #[dsl(unit = "m")]
    pub deck_width: f64,
    pub natural_frequency: f64,
    /// 🚐 Axle load [N] of the service vehicle Q_sv the design assumes on the footbridge; 0 = no service vehicle, no value is assumed and the check is not applicable.
    #[dsl(unit = "N")]
    pub assumed_service_axle: f64,
}

/// Railway span with its determinant length L_Φ and track maintenance: the dynamic factor Φ_2 (careful maintenance) or Φ_3 (standard maintenance) of EN 1991-2, 6.4.5.2.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RailDynamic {
    pub id: String,
    #[dsl(unit = "m")]
    pub determinant_length: f64,
    pub maintenance: String,
}

/// Wheel load spreading at 45 degrees through pavement and half the slab to the mid-plane (EN 1991-2, 4.3.6): the footprint used by punching checks.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LoadDispersal {
    pub id: String,
    #[dsl(unit = "m")]
    pub wheel_length: f64,
    #[dsl(unit = "m")]
    pub wheel_width: f64,
    #[dsl(unit = "m")]
    pub pavement_thickness: f64,
    #[dsl(unit = "m")]
    pub slab_thickness: f64,
}

/// Bulk solid with its angle of internal friction: the lateral pressure ratio K = 1 - sin(phi_i) and its mean K_m = 1.1 K (EN 1991-4, Annex E).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BulkSolid {
    pub id: String,
    pub internal_friction: f64,
}

/// Rotating machine with unbalanced mass, eccentricity and rotation frequency: the equivalent static force F_dyn = m e omega^2 (EN 1991-3, 5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RotatingMachine {
    pub id: String,
    pub mass: f64,
    #[dsl(unit = "m")]
    pub eccentricity: f64,
    pub frequency: f64,
}

/// 💥 💥 Dust explosion in a silo or bin (EN 1991-4 with the vent sizing of EN 14491): dust constant K_St, maximum explosion pressure, volume, vent area, static opening pressure and the reduced explosion pressure the structure is designed for.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DustExplosion {
    pub id: String,
    /// 💥 Dust explosion constant K_St [bar m/s] of the stored dust.
    pub kst: f64,
    /// 💥 Maximum explosion pressure p_max [Pa] of the dust in a closed vessel.
    #[dsl(unit = "Pa")]
    pub p_max: f64,
    /// 📦 Volume V [m³] of the silo or bin compartment.
    pub volume: f64,
    /// 🪟 Vent area A_v [m²] of the explosion venting.
    #[dsl(unit = "m2")]
    pub vent_area: f64,
    /// 🔓 Static opening pressure p_stat [Pa] of the vent panels.
    #[dsl(unit = "Pa")]
    pub static_opening_pressure: f64,
    /// 🛡️ Reduced explosion pressure p_red [Pa] the structure is designed for.
    #[dsl(unit = "Pa")]
    pub design_pressure: f64,
}

/// 🛤️ 🛤️ Rail load group of EN 1991-2, Table 6.11 (gr11 to gr31): the component factors entered from the table and the group actions of the LM71 axle, the LM71 distributed load, the longitudinal and the lateral force the design assumes.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RailLoadGroup {
    pub id: String,
    /// 🛤️ gr11 | gr12 | ... | gr31 of Table 6.11 (anything else is an invalid input).
    pub load_group: String,
    /// ⬇️ Factor of the vertical loads (LM71, SW/0) of the group, entered from Table 6.11.
    pub factor_vertical: f64,
    /// ↔️ Factor of the traction and braking force of the group, entered from Table 6.11.
    pub factor_longitudinal: f64,
    /// ↕️ Factor of the lateral force (nosing) of the group, entered from Table 6.11.
    pub factor_lateral: f64,
    /// 🏋️ LM71 axle load [N] of the group the design assumes (factor times alpha times 250 kN).
    #[dsl(unit = "N")]
    pub assumed_axle_load: f64,
    /// 📏 LM71 distributed load [N/m] of the group the design assumes.
    pub assumed_udl: f64,
    /// ↔️ Traction or braking force [N] of the group the design assumes.
    #[dsl(unit = "N")]
    pub assumed_longitudinal: f64,
    /// ↕️ Lateral force (nosing) [N] of the group the design assumes.
    #[dsl(unit = "N")]
    pub assumed_lateral: f64,
}

/// 🚆 🚆 Derailment action on a railway bridge (EN 1991-2, 6.7.1): design situation I (vehicles stay in the track area on the deck) or II, the vertical line load, its loaded length and its offset from the track axis the design assumes.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RailDerailment {
    pub id: String,
    /// 🚆 i | ii: design situation I or II of the derailment (anything else is an invalid input).
    pub situation: String,
    /// 🏋️ Vertical line load [N/m] along the track the design assumes for the derailed vehicles.
    pub assumed_line_load: f64,
    /// ↔️ Length [m] over which the line load acts along the track.
    #[dsl(unit = "m")]
    pub loaded_length: f64,
    /// ↕️ Distance [m] of the line load from the track axis, transverse to the track.
    #[dsl(unit = "m")]
    pub offset: f64,
}

/// 🔁 🔁 Road fatigue load model of EN 1991-2, 4.6: FLM1 (LM1 with 0.7 Q_ik and 0.3 q_ik of its lane) or FLM3 (single vehicle of four 120 kN axles) with the axle load and distributed load the design assumes.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueModel {
    pub id: String,
    /// 🔁 flm1 | flm3 (anything else is an invalid input).
    pub model: String,
    /// 🛣️ Notional lane (1 to 3) that carries the fatigue model.
    pub lane: u8,
    /// 🏋️ Axle load [N] the design assumes per axle (FLM1 tandem axle, FLM3 vehicle axle).
    #[dsl(unit = "N")]
    pub assumed_axle_load: f64,
    /// 📏 Distributed load [Pa] the design assumes on the lane (FLM1 only, not read for FLM3).
    #[dsl(unit = "Pa")]
    pub assumed_udl: f64,
}

/// 🚛 🚛 Vehicle collision with a bridge support or soffit (EN 1991-2, 4.7): design force in driving direction and across it, height of the line of action and position along the bridge.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SupportCollision {
    pub id: String,
    /// 🚛 deck | soffit (anything else is an invalid input).
    pub location: String,
    /// ➡️ Design force F_dx [N] in driving direction.
    #[dsl(unit = "N")]
    pub assumed_force_x: f64,
    /// ↔️ Design force F_dy [N] across the driving direction.
    #[dsl(unit = "N")]
    pub assumed_force_y: f64,
    /// ↕️ Height [m] of the line of action above the carriageway.
    #[dsl(unit = "m")]
    pub height: f64,
    /// ↔️ Position [m] of the struck support from the bridge start.
    #[dsl(unit = "m")]
    pub position_x: f64,
}

/// 🔥 🔥 Localised fire of EN 1991-1-2, Annex C: fire diameter, heat release rate, ceiling height and the distance of the member for the flame length, the plume temperature and the heat flux of Hasemi.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LocalFire {
    pub id: String,
    /// ⭕ Diameter D [m] of the fire source.
    #[dsl(unit = "m")]
    pub fire_diameter: f64,
    /// 🔥 Total heat release rate Q [W].
    pub heat_release_rate: f64,
    /// ↕️ Height H [m] of the ceiling above the fire source (0 for a fire without ceiling).
    #[dsl(unit = "m")]
    pub ceiling_height: f64,
    /// ↔️ Horizontal distance r [m] from the fire axis to the member below the ceiling.
    #[dsl(unit = "m")]
    pub horizontal_distance: f64,
    /// 🌡️ Heat flux [W/m²] the fire design assumes at the member.
    pub assumed_flux: f64,
}

/// 🛡️ 🛡️ Snow guard on a roof (EN 1991-1-3, 6.3 and 5.3.8): the roof it protects, the slope length behind it and the line force the design assumes against F_s = s b sin α.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SnowGuard {
    pub id: String,
    /// 🏠 Id of the roof (roofs[]) whose snow the guard retains.
    pub roof_id: String,
    /// ↔️ Length b [m] of the roof slope behind the guard measured in plan.
    #[dsl(unit = "m")]
    pub slope_length: f64,
    /// 🏋️ Line force [N/m] along the guard the design assumes.
    pub assumed_force: f64,
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1991", "EN 1991", EN1991_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1991_DIALECT: semio_plugin::app::Dialect = semio_plugin::app::Dialect { artifact_kind: "s.norm.en1991", standard: semio_plugin::app::StandardId("1"), subset: semio_plugin::app::SubsetId::ANY };

pub const EN1991_DOCUMENT_SCHEMA: &str = "semio.norm.en1991/v1";

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;
