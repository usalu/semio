//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::diff::GisMapDiff;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data};
use crate::GisMapSnapshot;
use dsl::{FromValue, ToValue};
use protocol::Mutation;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔹Operation
/// 🗺️ Typed, invertible, semantic GIS map mutation vocabulary — every variant wraps exactly one
/// `protocol::MutationKind` payload struct declared in its own `🧬️mutations/<kind>/🦠️mutation`
/// triad leaf; `#[derive(dsl::Mutations)]` wires `Mutation`/`SemanticMutation` from those leaves.
/// `positions`/`routes`/`regions` are id-keyed `MapFeature` collections, each getting the same
/// four-verb vocabulary (`create`/`delete`/`replace-<noun>-data`/`reorder-<plural>`) per
/// `derivation-rules.md`'s per-id-keyed-collection recipe.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum, dsl::Mutations, ToValue, FromValue)]
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
}

pub type GisMapEnvelope = ArtifactEnvelope<GisMapSnapshot, GisMapMutation>;
pub type GisMapStore = ArtifactStore<GisMapSnapshot, GisMapMutation>;
//#endregion 🔹Operation

//#region 🔹Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔹Tests

/// 🕸️ Applies one parent mutation while preserving the stable drawing/value member coordinates.
pub fn apply_gis_map_mutation(snapshot: &mut GisMapSnapshot, mutation: &GisMapMutation) -> protocol::MutationApplyResult<()> {
    let (next, _messages) = vcs::apply_mutation(snapshot, mutation)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_gis_map_mutation(snapshot: &GisMapSnapshot, mutation: &GisMapMutation) -> Vec<GisMapMutation> {
    mutation.inverse(snapshot)
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `GisMapMutation` variant, in declaration order — the vocabulary the `gismap-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `mutate-gismap-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] =
    &["create-position", "delete-position", "replace-position-data", "reorder-positions", "create-route", "delete-route", "replace-route-data", "reorder-routes", "create-region", "delete-region", "replace-region-data", "reorder-regions"];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `GisMapMutation` nor
/// `GisMapSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
/// 🧩️ Committed snapshots include the stable drawing and value child identities. The report
/// decodes them directly, so comparisons validate those identities alongside feature content.
///
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn gis_map_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<GisMapSnapshot, String> { dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: GisMapMutation = dsl::os_pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <GisMapMutation as Mutation<GisMapSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <GisMapMutation as Mutation<GisMapSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <GisMapMutation as Mutation<GisMapSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = dsl::os_pack::json::object([
        ("base".to_string(), dsl::os_pack::json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), dsl::os_pack::json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), dsl::os_pack::json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), dsl::os_pack::json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), dsl::os_pack::json::from_dsl_value(&forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), dsl::os_pack::json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), dsl::os_pack::json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), dsl::os_pack::json::from_dsl_value(&inverse_messages.to_value())),
    ]);
    Ok(dsl::os_pack::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
