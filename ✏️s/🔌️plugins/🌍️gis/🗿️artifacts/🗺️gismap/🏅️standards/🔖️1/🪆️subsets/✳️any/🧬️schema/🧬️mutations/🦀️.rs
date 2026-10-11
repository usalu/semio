//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use crate::diff::GisMapDiff;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data, set_position_property, remove_position_property, set_route_property, remove_route_property, set_region_property, remove_region_property};
use crate::GisMapSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use protocol::Mutation;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔹Operation
/// 🗺️ Typed, invertible, semantic GIS map mutation vocabulary — every variant wraps exactly one
/// `protocol::MutationKind` payload struct declared in its own `🧬️mutations/<kind>/🦠️mutation`
/// triad leaf; `#[derive(dsl::Mutations)]` wires `Mutation`/`SemanticMutation` from those leaves.
/// `positions`/`routes`/`regions` are id-keyed `MapFeature` collections, each getting the same
/// four-verb vocabulary (`create`/`delete`/`replace-<noun>-data`/`reorder-<plural>`) per
/// `derivation-rules.md`'s per-id-keyed-collection recipe.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = GisMapSnapshot, diff = GisMapDiff, schema = "gis.gismap")]
pub enum GisMapMutation {
    CreatePosition(create_position::CreatePosition),
    DeletePosition(delete_position::DeletePosition),
    ReplacePositionData(replace_position_data::ReplacePositionData),
    ReorderPositions(reorder_positions::ReorderPositions),
    CreateRoute(create_route::CreateRoute),
    DeleteRoute(delete_route::DeleteRoute),
    ReplaceRouteData(replace_route_data::ReplaceRouteData),
    ReorderRoutes(reorder_routes::ReorderRoutes),
    CreateRegion(create_region::CreateRegion),
    DeleteRegion(delete_region::DeleteRegion),
    ReplaceRegionData(replace_region_data::ReplaceRegionData),
    ReorderRegions(reorder_regions::ReorderRegions),
    SetPositionProperty(set_position_property::SetPositionProperty),
    RemovePositionProperty(remove_position_property::RemovePositionProperty),
    SetRouteProperty(set_route_property::SetRouteProperty),
    RemoveRouteProperty(remove_route_property::RemoveRouteProperty),
    SetRegionProperty(set_region_property::SetRegionProperty),
    RemoveRegionProperty(remove_region_property::RemoveRegionProperty),
}

pub type GisMapEnvelope = ArtifactEnvelope<GisMapSnapshot, GisMapMutation>;
pub type GisMapStore = ArtifactStore<GisMapSnapshot, GisMapMutation>;
//#endregion 🔹Operation

//#region 🔹Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔹Tests

pub fn inverse_gis_map_mutation(snapshot: &GisMapSnapshot, mutation: &GisMapMutation) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(snapshot)?

    })
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `GisMapMutation` variant, in declaration order — the vocabulary the `gismap-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `mutate-gismap-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] =
    &["create-position", "delete-position", "replace-position-data", "reorder-positions", "create-route", "delete-route", "replace-route-data", "reorder-routes", "create-region", "delete-region", "replace-region-data", "reorder-regions", "set-position-property", "remove-position-property", "set-route-property", "remove-route-property", "set-region-property", "remove-region-property"];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
