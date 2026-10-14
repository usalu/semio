//! 🧬️ En1991 artifact — document mutation dispatch for design-load subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::EDIT_RULES;

//#region 🔖️Mutations
use semio_s_artifact_norm_en1991_mut_general::change_annex;
use semio_s_artifact_norm_en1991_mut_snow::change_snow_zone;
use semio_s_artifact_norm_en1991_mut_snow::change_altitude;
use semio_s_artifact_norm_en1991_mut_snow::change_en_sk;
use semio_s_artifact_norm_en1991_mut_snow::change_north_german_lowland_snow;
use semio_s_artifact_norm_en1991_mut_wind::change_wind_zone;
use semio_s_artifact_norm_en1991_mut_wind::change_en_vb;
use semio_s_artifact_norm_en1991_mut_wind::change_terrain_category;
use semio_s_artifact_norm_en1991_mut_wind::change_mixed_terrain_upwind;
use semio_s_artifact_norm_en1991_mut_wind::change_mixed_terrain_distance;
use semio_s_artifact_norm_en1991_mut_wind::change_orography_factor;
use semio_s_artifact_norm_en1991_mut_wind::change_coast_or_island;
use semio_s_artifact_norm_en1991_mut_wind::change_air_density;
use semio_s_artifact_norm_en1991_mut_general::change_height;
use semio_s_artifact_norm_en1991_mut_general::change_width;
use semio_s_artifact_norm_en1991_mut_general::change_depth;
use semio_s_artifact_norm_en1991_mut_thermal::change_assumed_delta_t;
use semio_s_artifact_norm_en1991_mut_gravity::change_construction_activity;
use semio_s_artifact_norm_en1991_mut_gravity::change_assumed_construction_qk;
use semio_s_artifact_norm_en1991_mut_general::change_structure_kind;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_lane;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_span;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_lane_width;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_tandem;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_udl;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_lm2;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_footway;
use semio_s_artifact_norm_en1991_mut_general::change_storey_count;
use semio_s_artifact_norm_en1991_mut_thermal::change_t_max;
use semio_s_artifact_norm_en1991_mut_thermal::change_t_min;
use semio_s_artifact_norm_en1991_mut_thermal::change_initial_temperature;
use semio_s_artifact_norm_en1991_mut_thermal::change_thermal_element_type;
use semio_s_artifact_norm_en1991_mut_thermal::change_thermal_bridge_type;
use semio_s_artifact_norm_en1991_mut_thermal::change_linear_temperature_gradient;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_mode;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_curve;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_duration;
use semio_s_artifact_norm_en1991_mut_fire::change_assumed_gas_temperature;
use semio_s_artifact_norm_en1991_mut_fire::change_assumed_h_net;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_compartment_area;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_compartment_height;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_opening_factor;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_thermal_inertia;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_occupancy;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_load_density_qf;
use semio_s_artifact_norm_en1991_mut_fire::change_assumed_qf_d;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_lm3;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_lm4;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_load_group;
use semio_s_artifact_norm_en1991_mut_cranes::change_crane_claimed;
use semio_s_artifact_norm_en1991_mut_cranes::change_crane_class;
use semio_s_artifact_norm_en1991_mut_cranes::change_hoist_class;
use semio_s_artifact_norm_en1991_mut_cranes::change_hoisting_speed;
use semio_s_artifact_norm_en1991_mut_cranes::change_assumed_crane_wheel;
use semio_s_artifact_norm_en1991_mut_cranes::change_assumed_crane_horizontal;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_claimed;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_kind;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_bulk_density;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_height;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_hydraulic_radius;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_mu;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_k;
use semio_s_artifact_norm_en1991_mut_silos::change_assumed_silo_pressure;
use semio_s_artifact_norm_en1991_mut_silos::change_assumed_silo_patch;
use semio_s_artifact_norm_en1991_mut_silos::change_assumed_silo_wall_friction;
use semio_s_artifact_norm_en1991_mut_gravity::change_floor_assumed_qk;
use semio_s_artifact_norm_en1991_mut_gravity::change_self_weight_assumed_gk;
use semio_s_artifact_norm_en1991_mut_snow::change_roof_assumed_sk;
use semio_s_artifact_norm_en1991_mut_wind_faces::change_wind_face_assumed_wp;
use semio_s_artifact_norm_en1991_mut_general::change_accidental_assumed_force;
use semio_s_artifact_norm_en1991_mut_gravity::insert_floors;
use semio_s_artifact_norm_en1991_mut_gravity::remove_floors;
use semio_s_artifact_norm_en1991_mut_gravity::insert_self_weight_elements;
use semio_s_artifact_norm_en1991_mut_gravity::remove_self_weight_elements;
use semio_s_artifact_norm_en1991_mut_snow::insert_roofs;
use semio_s_artifact_norm_en1991_mut_snow::remove_roofs;
use semio_s_artifact_norm_en1991_mut_wind_faces::insert_wind_faces;
use semio_s_artifact_norm_en1991_mut_wind_faces::remove_wind_faces;
use semio_s_artifact_norm_en1991_mut_general::insert_accidental_cases;
use semio_s_artifact_norm_en1991_mut_general::remove_accidental_cases;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1991Snapshot, diff = En1991Diff, schema = "s.norm.en1991")]
pub enum En1991Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeSnowZone(change_snow_zone::ChangeSnowZone),
    ChangeAltitude(change_altitude::ChangeAltitude),
    ChangeEnSk(change_en_sk::ChangeEnSk),
    ChangeNorthGermanLowlandSnow(change_north_german_lowland_snow::ChangeNorthGermanLowlandSnow),
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
    "change-north-german-lowland-snow",
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

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1991Mutation;
    use semio_s_artifact_norm_en1991_model::En1991Snapshot;

/// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
pub fn apply_en1991_mutation(base: &En1991Snapshot, mutation: &En1991Mutation) -> Result<(En1991Snapshot, Vec<String>), String> {
    let raised = <En1991Mutation as protocol::Mutation<En1991Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}
/// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
pub fn inverse_en1991_mutation(mutation: &En1991Mutation, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok({
    <En1991Mutation as protocol::Mutation<En1991Snapshot>>::inverse(mutation, base)?

    })
}
}
pub use crate::mutation_bridge::*;
