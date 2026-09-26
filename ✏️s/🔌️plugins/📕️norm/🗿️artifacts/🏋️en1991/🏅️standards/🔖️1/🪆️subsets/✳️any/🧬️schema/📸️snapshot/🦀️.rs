//! 🧬️ En1991 snapshot schema — complete design-load-assumption subject.

use crate::{AccidentalCase, FloorArea, RoofArea, SelfWeightElement, WindFace};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1991 subject: site + building geometry + design load assumptions (SI: m, Pa, N, K, kg/m³).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1991", layout = "lines")]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Snapshot {
    #[state(artifact)]
    pub annex: crate::document::AnnexChoice,
    #[state(artifact)]
    pub snow_zone: String,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub altitude: f64,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub en_sk: f64,
    #[state(artifact)]
    pub exceptional_snow_north_german_lowlands: bool,
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
    pub fire_occupancy: String,
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
    #[dsl(table)]
    #[state(artifact)]
    pub floors: Vec<crate::FloorArea>,
    #[dsl(table)]
    #[state(artifact)]
    pub self_weight_elements: Vec<crate::SelfWeightElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub roofs: Vec<crate::RoofArea>,
    #[dsl(table)]
    #[state(artifact)]
    pub wind_faces: Vec<crate::WindFace>,
    #[dsl(table)]
    #[state(artifact)]
    pub accidental_cases: Vec<crate::AccidentalCase>,
}
//#region 🔖️HandcraftedArtifactCodecs
crate::impl_norm_artifact_record!(En1991Snapshot, extension = "en1991", envelope_id = "norm.en1991");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1991Snapshot {
    fn default() -> Self {
        Self {
            annex: crate::document::AnnexChoice::De,
            snow_zone: "2".into(),
            altitude: 150.0,
            en_sk: 850.0,
            exceptional_snow_north_german_lowlands: false,
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
            fire_occupancy: "office".into(),
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
    c_pe10: 0.7,
    c_pe1: 0.875,
    c_pi: 0.2,
    c_s: 1.0,
    c_d: 1.0,
    loaded_area: 10.0,
    assumed_wp: 1_200.0,
}],
accidental_cases: vec![],
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
/// 📤️ Canonical JSON projection of [`En1991Snapshot`].
pub fn encode_en1991_snapshot_json(snapshot: &En1991Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_en1991_snapshot_json`].
pub fn decode_en1991_snapshot_json(text: &str) -> Result<En1991Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parse committed `.dsl.semio` into [`En1991Snapshot`].
pub fn decode_en1991_dsl(text: &str) -> Result<En1991Snapshot, String> {
    <En1991Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Print [`En1991Snapshot`] to canonical `.dsl.semio`.
pub fn encode_en1991_dsl(snapshot: &En1991Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decode `.pack.semio` envelope.
pub fn decode_en1991_pack(bytes: &[u8]) -> Result<En1991Snapshot, String> {
    <En1991Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encode `.pack.semio` envelope.
pub fn encode_en1991_pack(snapshot: &En1991Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
