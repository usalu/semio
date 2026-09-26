//! 🧬️ En1991 artifact — document mutation dispatch for design-load subject.

use crate::{En1991Diff, En1991Snapshot};

//#region 🔖️Mutations
use super::change_annex;
use super::change_snow_zone;
use super::change_altitude;
use super::change_en_sk;
use super::change_exceptional_snow_north_german_lowlands;
use super::change_wind_zone;
use super::change_en_vb;
use super::change_terrain_category;
use super::change_mixed_terrain_upwind;
use super::change_mixed_terrain_distance;
use super::change_orography_factor;
use super::change_coast_or_island;
use super::change_air_density;
use super::change_height;
use super::change_width;
use super::change_depth;
use super::change_assumed_delta_t;
use super::change_construction_activity;
use super::change_assumed_construction_qk;
use super::change_structure_kind;
use super::change_bridge_lane;
use super::change_bridge_span;
use super::change_bridge_lane_width;
use super::change_assumed_bridge_tandem;
use super::change_assumed_bridge_udl;
use super::change_assumed_bridge_lm2;
use super::change_assumed_bridge_footway;
use super::change_storey_count;
use super::change_t_max;
use super::change_t_min;
use super::change_initial_temperature;
use super::change_thermal_element_type;
use super::change_thermal_bridge_type;
use super::change_linear_temperature_gradient;
use super::change_fire_mode;
use super::change_fire_curve;
use super::change_fire_duration;
use super::change_assumed_gas_temperature;
use super::change_assumed_h_net;
use super::change_fire_compartment_area;
use super::change_fire_compartment_height;
use super::change_fire_opening_factor;
use super::change_fire_thermal_inertia;
use super::change_fire_occupancy;
use super::change_fire_load_density_qf;
use super::change_assumed_qf_d;
use super::change_assumed_bridge_lm3;
use super::change_assumed_bridge_lm4;
use super::change_bridge_load_group;
use super::change_crane_claimed;
use super::change_crane_class;
use super::change_hoist_class;
use super::change_hoisting_speed;
use super::change_assumed_crane_wheel;
use super::change_assumed_crane_horizontal;
use super::change_silo_claimed;
use super::change_silo_kind;
use super::change_silo_bulk_density;
use super::change_silo_height;
use super::change_silo_hydraulic_radius;
use super::change_silo_mu;
use super::change_silo_k;
use super::change_assumed_silo_pressure;
use super::change_assumed_silo_patch;
use super::change_assumed_silo_wall_friction;
use super::change_floor_assumed_qk;
use super::change_self_weight_assumed_gk;
use super::change_roof_assumed_sk;
use super::change_wind_face_assumed_wp;
use super::change_accidental_assumed_force;
use super::insert_floors;
use super::remove_floors;
use super::insert_self_weight_elements;
use super::remove_self_weight_elements;
use super::insert_roofs;
use super::remove_roofs;
use super::insert_wind_faces;
use super::remove_wind_faces;
use super::insert_accidental_cases;
use super::remove_accidental_cases;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1991Snapshot, diff = En1991Diff, schema = "s.norm.en1991")]
pub enum En1991Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeSnowZone(change_snow_zone::ChangeSnowZone),
    ChangeAltitude(change_altitude::ChangeAltitude),
    ChangeEnSk(change_en_sk::ChangeEnSk),
    ChangeExceptionalSnowNorthGermanLowlands(change_exceptional_snow_north_german_lowlands::ChangeExceptionalSnowNorthGermanLowlands),
    ChangeWindZone(change_wind_zone::ChangeWindZone),
    ChangeEnVb(change_en_vb::ChangeEnVb),
    ChangeTerrainCategory(change_terrain_category::ChangeTerrainCategory),
    ChangeMixedTerrainUpwind(change_mixed_terrain_upwind::ChangeMixedTerrainUpwind),
    ChangeMixedTerrainDistance(change_mixed_terrain_distance::ChangeMixedTerrainDistance),
    ChangeOrographyFactor(change_orography_factor::ChangeOrographyFactor),
    ChangeCoastOrIsland(change_coast_or_island::ChangeCoastOrIsland),
    ChangeAirDensity(change_air_density::ChangeAirDensity),
    ChangeHeight(change_height::ChangeHeight),
    ChangeWidth(change_width::ChangeWidth),
    ChangeDepth(change_depth::ChangeDepth),
    ChangeAssumedDeltaT(change_assumed_delta_t::ChangeAssumedDeltaT),
    ChangeConstructionActivity(change_construction_activity::ChangeConstructionActivity),
    ChangeAssumedConstructionQk(change_assumed_construction_qk::ChangeAssumedConstructionQk),
    ChangeStructureKind(change_structure_kind::ChangeStructureKind),
    ChangeBridgeLane(change_bridge_lane::ChangeBridgeLane),
    ChangeBridgeSpan(change_bridge_span::ChangeBridgeSpan),
    ChangeBridgeLaneWidth(change_bridge_lane_width::ChangeBridgeLaneWidth),
    ChangeAssumedBridgeTandem(change_assumed_bridge_tandem::ChangeAssumedBridgeTandem),
    ChangeAssumedBridgeUdl(change_assumed_bridge_udl::ChangeAssumedBridgeUdl),
    ChangeAssumedBridgeLm2(change_assumed_bridge_lm2::ChangeAssumedBridgeLm2),
    ChangeAssumedBridgeFootway(change_assumed_bridge_footway::ChangeAssumedBridgeFootway),
    ChangeStoreyCount(change_storey_count::ChangeStoreyCount),
    ChangeTMax(change_t_max::ChangeTMax),
    ChangeTMin(change_t_min::ChangeTMin),
    ChangeInitialTemperature(change_initial_temperature::ChangeInitialTemperature),
    ChangeThermalElementType(change_thermal_element_type::ChangeThermalElementType),
    ChangeThermalBridgeType(change_thermal_bridge_type::ChangeThermalBridgeType),
    ChangeLinearTemperatureGradient(change_linear_temperature_gradient::ChangeLinearTemperatureGradient),
    ChangeFireMode(change_fire_mode::ChangeFireMode),
    ChangeFireCurve(change_fire_curve::ChangeFireCurve),
    ChangeFireDuration(change_fire_duration::ChangeFireDuration),
    ChangeAssumedGasTemperature(change_assumed_gas_temperature::ChangeAssumedGasTemperature),
    ChangeAssumedHNet(change_assumed_h_net::ChangeAssumedHNet),
    ChangeFireCompartmentArea(change_fire_compartment_area::ChangeFireCompartmentArea),
    ChangeFireCompartmentHeight(change_fire_compartment_height::ChangeFireCompartmentHeight),
    ChangeFireOpeningFactor(change_fire_opening_factor::ChangeFireOpeningFactor),
    ChangeFireThermalInertia(change_fire_thermal_inertia::ChangeFireThermalInertia),
    ChangeFireOccupancy(change_fire_occupancy::ChangeFireOccupancy),
    ChangeFireLoadDensityQf(change_fire_load_density_qf::ChangeFireLoadDensityQf),
    ChangeAssumedQfD(change_assumed_qf_d::ChangeAssumedQfD),
    ChangeAssumedBridgeLm3(change_assumed_bridge_lm3::ChangeAssumedBridgeLm3),
    ChangeAssumedBridgeLm4(change_assumed_bridge_lm4::ChangeAssumedBridgeLm4),
    ChangeBridgeLoadGroup(change_bridge_load_group::ChangeBridgeLoadGroup),
    ChangeCraneClaimed(change_crane_claimed::ChangeCraneClaimed),
    ChangeCraneClass(change_crane_class::ChangeCraneClass),
    ChangeHoistClass(change_hoist_class::ChangeHoistClass),
    ChangeHoistingSpeed(change_hoisting_speed::ChangeHoistingSpeed),
    ChangeAssumedCraneWheel(change_assumed_crane_wheel::ChangeAssumedCraneWheel),
    ChangeAssumedCraneHorizontal(change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal),
    ChangeSiloClaimed(change_silo_claimed::ChangeSiloClaimed),
    ChangeSiloKind(change_silo_kind::ChangeSiloKind),
    ChangeSiloBulkDensity(change_silo_bulk_density::ChangeSiloBulkDensity),
    ChangeSiloHeight(change_silo_height::ChangeSiloHeight),
    ChangeSiloHydraulicRadius(change_silo_hydraulic_radius::ChangeSiloHydraulicRadius),
    ChangeSiloMu(change_silo_mu::ChangeSiloMu),
    ChangeSiloK(change_silo_k::ChangeSiloK),
    ChangeAssumedSiloPressure(change_assumed_silo_pressure::ChangeAssumedSiloPressure),
    ChangeAssumedSiloPatch(change_assumed_silo_patch::ChangeAssumedSiloPatch),
    ChangeAssumedSiloWallFriction(change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction),
    ChangeFloorAssumedQk(change_floor_assumed_qk::ChangeFloorAssumedQk),
    ChangeSelfWeightAssumedGk(change_self_weight_assumed_gk::ChangeSelfWeightAssumedGk),
    ChangeRoofAssumedSk(change_roof_assumed_sk::ChangeRoofAssumedSk),
    ChangeWindFaceAssumedWp(change_wind_face_assumed_wp::ChangeWindFaceAssumedWp),
    ChangeAccidentalAssumedForce(change_accidental_assumed_force::ChangeAccidentalAssumedForce),
    InsertFloors(insert_floors::InsertFloors),
    RemoveFloors(remove_floors::RemoveFloors),
    InsertSelfWeightElements(insert_self_weight_elements::InsertSelfWeightElements),
    RemoveSelfWeightElements(remove_self_weight_elements::RemoveSelfWeightElements),
    InsertRoofs(insert_roofs::InsertRoofs),
    RemoveRoofs(remove_roofs::RemoveRoofs),
    InsertWindFaces(insert_wind_faces::InsertWindFaces),
    RemoveWindFaces(remove_wind_faces::RemoveWindFaces),
    InsertAccidentalCases(insert_accidental_cases::InsertAccidentalCases),
    RemoveAccidentalCases(remove_accidental_cases::RemoveAccidentalCases),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-snow-zone",
    "change-altitude",
    "change-en-sk",
    "change-exceptional-snow-north-german-lowlands",
    "change-wind-zone",
    "change-en-vb",
    "change-terrain-category",
    "change-mixed-terrain-upwind",
    "change-mixed-terrain-distance",
    "change-orography-factor",
    "change-coast-or-island",
    "change-air-density",
    "change-height",
    "change-width",
    "change-depth",
    "change-assumed-delta-t",
    "change-construction-activity",
    "change-assumed-construction-qk",
    "change-structure-kind",
    "change-bridge-lane",
    "change-bridge-span",
    "change-bridge-lane-width",
    "change-assumed-bridge-tandem",
    "change-assumed-bridge-udl",
    "change-assumed-bridge-lm2",
    "change-assumed-bridge-footway",
    "change-storey-count",
    "change-t-max",
    "change-t-min",
    "change-initial-temperature",
    "change-thermal-element-type",
    "change-thermal-bridge-type",
    "change-linear-temperature-gradient",
    "change-fire-mode",
    "change-fire-curve",
    "change-fire-duration",
    "change-assumed-gas-temperature",
    "change-assumed-h-net",
    "change-fire-compartment-area",
    "change-fire-compartment-height",
    "change-fire-opening-factor",
    "change-fire-thermal-inertia",
    "change-fire-occupancy",
    "change-fire-load-density-qf",
    "change-assumed-qf-d",
    "change-assumed-bridge-lm3",
    "change-assumed-bridge-lm4",
    "change-bridge-load-group",
    "change-crane-claimed",
    "change-crane-class",
    "change-hoist-class",
    "change-hoisting-speed",
    "change-assumed-crane-wheel",
    "change-assumed-crane-horizontal",
    "change-silo-claimed",
    "change-silo-kind",
    "change-silo-bulk-density",
    "change-silo-height",
    "change-silo-hydraulic-radius",
    "change-silo-mu",
    "change-silo-k",
    "change-assumed-silo-pressure",
    "change-assumed-silo-patch",
    "change-assumed-silo-wall-friction",
    "change-floor-assumed-qk",
    "change-self-weight-assumed-gk",
    "change-roof-assumed-sk",
    "change-wind-face-assumed-wp",
    "change-accidental-assumed-force",
    "insert-floors",
    "remove-floors",
    "insert-self-weight-elements",
    "remove-self-weight-elements",
    "insert-roofs",
    "remove-roofs",
    "insert-wind-faces",
    "remove-wind-faces",
    "insert-accidental-cases",
    "remove-accidental-cases",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1991Mutation {
    /// 🩹 Diff `base` → `target` into one undoable mutation bundle (B2 setField/insert/remove/applyRemedy).
    pub fn from_snapshot(base: &En1991Snapshot, target: &En1991Snapshot) -> Vec<En1991Mutation> {
        let mut out = Vec::new();
        if base.annex != target.annex {
            out.push(En1991Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.snow_zone != target.snow_zone {
            out.push(En1991Mutation::ChangeSnowZone(change_snow_zone::ChangeSnowZone { new_snow_zone: target.snow_zone.clone() }));
        }
        if base.construction_activity != target.construction_activity {
            out.push(En1991Mutation::ChangeConstructionActivity(change_construction_activity::ChangeConstructionActivity { new_construction_activity: target.construction_activity.clone() }));
        }
        if base.crane_class != target.crane_class {
            out.push(En1991Mutation::ChangeCraneClass(change_crane_class::ChangeCraneClass { new_crane_class: target.crane_class.clone() }));
        }
        if base.hoist_class != target.hoist_class {
            out.push(En1991Mutation::ChangeHoistClass(change_hoist_class::ChangeHoistClass { new_hoist_class: target.hoist_class.clone() }));
        }
        if base.silo_kind != target.silo_kind {
            out.push(En1991Mutation::ChangeSiloKind(change_silo_kind::ChangeSiloKind { new_silo_kind: target.silo_kind.clone() }));
        }
        if base.structure_kind != target.structure_kind {
            out.push(En1991Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind { new_structure_kind: target.structure_kind }));
        }
        if base.crane_claimed != target.crane_claimed {
            out.push(En1991Mutation::ChangeCraneClaimed(change_crane_claimed::ChangeCraneClaimed { new_crane_claimed: target.crane_claimed }));
        }
        if base.silo_claimed != target.silo_claimed {
            out.push(En1991Mutation::ChangeSiloClaimed(change_silo_claimed::ChangeSiloClaimed { new_silo_claimed: target.silo_claimed }));
        }
        if base.exceptional_snow_north_german_lowlands != target.exceptional_snow_north_german_lowlands {
            out.push(En1991Mutation::ChangeExceptionalSnowNorthGermanLowlands(change_exceptional_snow_north_german_lowlands::ChangeExceptionalSnowNorthGermanLowlands { new_exceptional_snow_north_german_lowlands: target.exceptional_snow_north_german_lowlands }));
        }
        if base.coast_or_island != target.coast_or_island {
            out.push(En1991Mutation::ChangeCoastOrIsland(change_coast_or_island::ChangeCoastOrIsland { new_coast_or_island: target.coast_or_island }));
        }
        if base.wind_zone != target.wind_zone {
            out.push(En1991Mutation::ChangeWindZone(change_wind_zone::ChangeWindZone { new_wind_zone: target.wind_zone }));
        }
        if base.terrain_category != target.terrain_category {
            out.push(En1991Mutation::ChangeTerrainCategory(change_terrain_category::ChangeTerrainCategory { new_terrain_category: target.terrain_category }));
        }
        if base.mixed_terrain_upwind != target.mixed_terrain_upwind {
            out.push(En1991Mutation::ChangeMixedTerrainUpwind(change_mixed_terrain_upwind::ChangeMixedTerrainUpwind { new_mixed_terrain_upwind: target.mixed_terrain_upwind }));
        }
        if base.bridge_lane != target.bridge_lane {
            out.push(En1991Mutation::ChangeBridgeLane(change_bridge_lane::ChangeBridgeLane { new_bridge_lane: target.bridge_lane }));
        }
        if base.altitude != target.altitude {
            out.push(En1991Mutation::ChangeAltitude(change_altitude::ChangeAltitude { new_altitude: target.altitude }));
        }
        if base.en_sk != target.en_sk {
            out.push(En1991Mutation::ChangeEnSk(change_en_sk::ChangeEnSk { new_en_sk: target.en_sk }));
        }
        if base.en_vb != target.en_vb {
            out.push(En1991Mutation::ChangeEnVb(change_en_vb::ChangeEnVb { new_en_vb: target.en_vb }));
        }
        if base.mixed_terrain_distance != target.mixed_terrain_distance {
            out.push(En1991Mutation::ChangeMixedTerrainDistance(change_mixed_terrain_distance::ChangeMixedTerrainDistance { new_mixed_terrain_distance: target.mixed_terrain_distance }));
        }
        if base.orography_factor != target.orography_factor {
            out.push(En1991Mutation::ChangeOrographyFactor(change_orography_factor::ChangeOrographyFactor { new_orography_factor: target.orography_factor }));
        }
        if base.air_density != target.air_density {
            out.push(En1991Mutation::ChangeAirDensity(change_air_density::ChangeAirDensity { new_air_density: target.air_density }));
        }
        if base.height != target.height {
            out.push(En1991Mutation::ChangeHeight(change_height::ChangeHeight { new_height: target.height }));
        }
        if base.width != target.width {
            out.push(En1991Mutation::ChangeWidth(change_width::ChangeWidth { new_width: target.width }));
        }
        if base.depth != target.depth {
            out.push(En1991Mutation::ChangeDepth(change_depth::ChangeDepth { new_depth: target.depth }));
        }
        if base.assumed_delta_t != target.assumed_delta_t {
            out.push(En1991Mutation::ChangeAssumedDeltaT(change_assumed_delta_t::ChangeAssumedDeltaT { new_assumed_delta_t: target.assumed_delta_t }));
        }
        if base.assumed_construction_qk != target.assumed_construction_qk {
            out.push(En1991Mutation::ChangeAssumedConstructionQk(change_assumed_construction_qk::ChangeAssumedConstructionQk { new_assumed_construction_qk: target.assumed_construction_qk }));
        }
        if base.bridge_span != target.bridge_span {
            out.push(En1991Mutation::ChangeBridgeSpan(change_bridge_span::ChangeBridgeSpan { new_bridge_span: target.bridge_span }));
        }
        if base.bridge_lane_width != target.bridge_lane_width {
            out.push(En1991Mutation::ChangeBridgeLaneWidth(change_bridge_lane_width::ChangeBridgeLaneWidth { new_bridge_lane_width: target.bridge_lane_width }));
        }
        if base.assumed_bridge_tandem != target.assumed_bridge_tandem {
            out.push(En1991Mutation::ChangeAssumedBridgeTandem(change_assumed_bridge_tandem::ChangeAssumedBridgeTandem { new_assumed_bridge_tandem: target.assumed_bridge_tandem }));
        }
        if base.assumed_bridge_udl != target.assumed_bridge_udl {
            out.push(En1991Mutation::ChangeAssumedBridgeUdl(change_assumed_bridge_udl::ChangeAssumedBridgeUdl { new_assumed_bridge_udl: target.assumed_bridge_udl }));
        }
        if base.assumed_bridge_lm2 != target.assumed_bridge_lm2 {
            out.push(En1991Mutation::ChangeAssumedBridgeLm2(change_assumed_bridge_lm2::ChangeAssumedBridgeLm2 { new_assumed_bridge_lm2: target.assumed_bridge_lm2 }));
        }
        if base.assumed_bridge_footway != target.assumed_bridge_footway {
            out.push(En1991Mutation::ChangeAssumedBridgeFootway(change_assumed_bridge_footway::ChangeAssumedBridgeFootway { new_assumed_bridge_footway: target.assumed_bridge_footway }));
        }
        if base.hoisting_speed != target.hoisting_speed {
            out.push(En1991Mutation::ChangeHoistingSpeed(change_hoisting_speed::ChangeHoistingSpeed { new_hoisting_speed: target.hoisting_speed }));
        }
        if base.assumed_crane_wheel != target.assumed_crane_wheel {
            out.push(En1991Mutation::ChangeAssumedCraneWheel(change_assumed_crane_wheel::ChangeAssumedCraneWheel { new_assumed_crane_wheel: target.assumed_crane_wheel }));
        }
        if base.assumed_crane_horizontal != target.assumed_crane_horizontal {
            out.push(En1991Mutation::ChangeAssumedCraneHorizontal(change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal { new_assumed_crane_horizontal: target.assumed_crane_horizontal }));
        }
        if base.silo_bulk_density != target.silo_bulk_density {
            out.push(En1991Mutation::ChangeSiloBulkDensity(change_silo_bulk_density::ChangeSiloBulkDensity { new_silo_bulk_density: target.silo_bulk_density }));
        }
        if base.silo_height != target.silo_height {
            out.push(En1991Mutation::ChangeSiloHeight(change_silo_height::ChangeSiloHeight { new_silo_height: target.silo_height }));
        }
        if base.silo_hydraulic_radius != target.silo_hydraulic_radius {
            out.push(En1991Mutation::ChangeSiloHydraulicRadius(change_silo_hydraulic_radius::ChangeSiloHydraulicRadius { new_silo_hydraulic_radius: target.silo_hydraulic_radius }));
        }
        if base.silo_mu != target.silo_mu {
            out.push(En1991Mutation::ChangeSiloMu(change_silo_mu::ChangeSiloMu { new_silo_mu: target.silo_mu }));
        }
        if base.silo_k != target.silo_k {
            out.push(En1991Mutation::ChangeSiloK(change_silo_k::ChangeSiloK { new_silo_k: target.silo_k }));
        }
        if base.assumed_silo_pressure != target.assumed_silo_pressure {
            out.push(En1991Mutation::ChangeAssumedSiloPressure(change_assumed_silo_pressure::ChangeAssumedSiloPressure { new_assumed_silo_pressure: target.assumed_silo_pressure }));
        }
        if base.assumed_silo_patch != target.assumed_silo_patch {
            out.push(En1991Mutation::ChangeAssumedSiloPatch(change_assumed_silo_patch::ChangeAssumedSiloPatch { new_assumed_silo_patch: target.assumed_silo_patch }));
        }
        if base.assumed_silo_wall_friction != target.assumed_silo_wall_friction {
            out.push(En1991Mutation::ChangeAssumedSiloWallFriction(change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction { new_assumed_silo_wall_friction: target.assumed_silo_wall_friction }));
        }

                if base.storey_count != target.storey_count {
            out.push(En1991Mutation::ChangeStoreyCount(change_storey_count::ChangeStoreyCount { new_storey_count: target.storey_count }));
        }
        if base.t_max != target.t_max {
            out.push(En1991Mutation::ChangeTMax(change_t_max::ChangeTMax { new_t_max: target.t_max }));
        }
        if base.t_min != target.t_min {
            out.push(En1991Mutation::ChangeTMin(change_t_min::ChangeTMin { new_t_min: target.t_min }));
        }
        if base.t_0 != target.t_0 {
            out.push(En1991Mutation::ChangeInitialTemperature(change_initial_temperature::ChangeInitialTemperature { new_t_0: target.t_0 }));
        }
        if base.thermal_element_type != target.thermal_element_type {
            out.push(En1991Mutation::ChangeThermalElementType(change_thermal_element_type::ChangeThermalElementType { new_thermal_element_type: target.thermal_element_type.clone() }));
        }
        if base.thermal_bridge_type != target.thermal_bridge_type {
            out.push(En1991Mutation::ChangeThermalBridgeType(change_thermal_bridge_type::ChangeThermalBridgeType { new_thermal_bridge_type: target.thermal_bridge_type }));
        }
        if base.delta_t_m != target.delta_t_m {
            out.push(En1991Mutation::ChangeLinearTemperatureGradient(change_linear_temperature_gradient::ChangeLinearTemperatureGradient { new_delta_t_m: target.delta_t_m }));
        }
        if base.fire_mode != target.fire_mode {
            out.push(En1991Mutation::ChangeFireMode(change_fire_mode::ChangeFireMode { new_fire_mode: target.fire_mode }));
        }
        if base.fire_curve != target.fire_curve {
            out.push(En1991Mutation::ChangeFireCurve(change_fire_curve::ChangeFireCurve { new_fire_curve: target.fire_curve }));
        }
        if base.fire_duration != target.fire_duration {
            out.push(En1991Mutation::ChangeFireDuration(change_fire_duration::ChangeFireDuration { new_fire_duration: target.fire_duration }));
        }
        if base.assumed_gas_temperature != target.assumed_gas_temperature {
            out.push(En1991Mutation::ChangeAssumedGasTemperature(change_assumed_gas_temperature::ChangeAssumedGasTemperature { new_assumed_gas_temperature: target.assumed_gas_temperature }));
        }
        if base.assumed_h_net != target.assumed_h_net {
            out.push(En1991Mutation::ChangeAssumedHNet(change_assumed_h_net::ChangeAssumedHNet { new_assumed_h_net: target.assumed_h_net }));
        }
        if base.fire_compartment_area != target.fire_compartment_area {
            out.push(En1991Mutation::ChangeFireCompartmentArea(change_fire_compartment_area::ChangeFireCompartmentArea { new_fire_compartment_area: target.fire_compartment_area }));
        }
        if base.fire_compartment_height != target.fire_compartment_height {
            out.push(En1991Mutation::ChangeFireCompartmentHeight(change_fire_compartment_height::ChangeFireCompartmentHeight { new_fire_compartment_height: target.fire_compartment_height }));
        }
        if base.fire_opening_factor != target.fire_opening_factor {
            out.push(En1991Mutation::ChangeFireOpeningFactor(change_fire_opening_factor::ChangeFireOpeningFactor { new_fire_opening_factor: target.fire_opening_factor }));
        }
        if base.fire_thermal_inertia != target.fire_thermal_inertia {
            out.push(En1991Mutation::ChangeFireThermalInertia(change_fire_thermal_inertia::ChangeFireThermalInertia { new_fire_thermal_inertia: target.fire_thermal_inertia }));
        }
        if base.fire_occupancy != target.fire_occupancy {
            out.push(En1991Mutation::ChangeFireOccupancy(change_fire_occupancy::ChangeFireOccupancy { new_fire_occupancy: target.fire_occupancy.clone() }));
        }
        if base.fire_load_density_qf != target.fire_load_density_qf {
            out.push(En1991Mutation::ChangeFireLoadDensityQf(change_fire_load_density_qf::ChangeFireLoadDensityQf { new_fire_load_density_qf: target.fire_load_density_qf }));
        }
        if base.assumed_qf_d != target.assumed_qf_d {
            out.push(En1991Mutation::ChangeAssumedQfD(change_assumed_qf_d::ChangeAssumedQfD { new_assumed_qf_d: target.assumed_qf_d }));
        }
        if base.assumed_bridge_lm3 != target.assumed_bridge_lm3 {
            out.push(En1991Mutation::ChangeAssumedBridgeLm3(change_assumed_bridge_lm3::ChangeAssumedBridgeLm3 { new_assumed_bridge_lm3: target.assumed_bridge_lm3 }));
        }
        if base.assumed_bridge_lm4 != target.assumed_bridge_lm4 {
            out.push(En1991Mutation::ChangeAssumedBridgeLm4(change_assumed_bridge_lm4::ChangeAssumedBridgeLm4 { new_assumed_bridge_lm4: target.assumed_bridge_lm4 }));
        }
        if base.bridge_load_group != target.bridge_load_group {
            out.push(En1991Mutation::ChangeBridgeLoadGroup(change_bridge_load_group::ChangeBridgeLoadGroup { new_bridge_load_group: target.bridge_load_group.clone() }));
        }
        if base.floors != target.floors {
            // Rebuild floors list via remove-all + insert
            for index in (0..base.floors.len()).rev() {
                out.push(En1991Mutation::RemoveFloors(remove_floors::RemoveFloors { index }));
            }
            for (index, floor) in target.floors.iter().enumerate() {
                out.push(En1991Mutation::InsertFloors(insert_floors::InsertFloors { index, item: floor.clone() }));
            }
        }
        if base.self_weight_elements != target.self_weight_elements {
            for index in (0..base.self_weight_elements.len()).rev() {
                out.push(En1991Mutation::RemoveSelfWeightElements(remove_self_weight_elements::RemoveSelfWeightElements { index }));
            }
            for (index, element) in target.self_weight_elements.iter().enumerate() {
                out.push(En1991Mutation::InsertSelfWeightElements(insert_self_weight_elements::InsertSelfWeightElements { index, item: element.clone() }));
            }
        }
        if base.roofs != target.roofs {
            for index in (0..base.roofs.len()).rev() {
                out.push(En1991Mutation::RemoveRoofs(remove_roofs::RemoveRoofs { index }));
            }
            for (index, roof) in target.roofs.iter().enumerate() {
                out.push(En1991Mutation::InsertRoofs(insert_roofs::InsertRoofs { index, item: roof.clone() }));
            }
        }
        if base.wind_faces != target.wind_faces {
            for index in (0..base.wind_faces.len()).rev() {
                out.push(En1991Mutation::RemoveWindFaces(remove_wind_faces::RemoveWindFaces { index }));
            }
            for (index, face) in target.wind_faces.iter().enumerate() {
                out.push(En1991Mutation::InsertWindFaces(insert_wind_faces::InsertWindFaces { index, item: face.clone() }));
            }
        }
        if base.accidental_cases != target.accidental_cases {
            for index in (0..base.accidental_cases.len()).rev() {
                out.push(En1991Mutation::RemoveAccidentalCases(remove_accidental_cases::RemoveAccidentalCases { index }));
            }
            for (index, case) in target.accidental_cases.iter().enumerate() {
                out.push(En1991Mutation::InsertAccidentalCases(insert_accidental_cases::InsertAccidentalCases { index, item: case.clone() }));
            }
        }

        out
    }
}
//#endregion 🔖️FromSnapshot

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
