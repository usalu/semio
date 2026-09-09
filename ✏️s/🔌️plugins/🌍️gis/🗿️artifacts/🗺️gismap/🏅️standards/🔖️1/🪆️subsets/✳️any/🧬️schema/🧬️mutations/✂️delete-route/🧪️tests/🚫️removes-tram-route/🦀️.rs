//! 🧪️ `delete-route` fixture — `🚫️removes-tram-route`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, never here.
//!
//! ✂️ `delete-route` writes ONE `removed` id into `routes` and — unlike a graph vocabulary's delete —
//! cascades nothing: gis map features are independent, so no position or region is disturbed by dropping a
//! route. Its inverse is BASE-derived: a `create-route` rebuilt from the pre-deletion snapshot at the index
//! the route occupied.
//!
//! 🧩️ Committed snapshots preserve the stable drawing and value child identities across edits.
//! Feature collections and their sparse deltas are asserted directly against the neutral fixtures.

use crate::diff::GisMapDiff;
use crate::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
use crate::GisMapSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-route/🚫️removes-tram-route/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-route/🚫️removes-tram-route/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-route/🚫️removes-tram-route/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-route/🚫️removes-tram-route/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️delete-route/🚫️removes-tram-route/🎯️outcome/🔣️.json");

/// 🗺️ Decodes the committed snapshot with its stable child identities.
fn before() -> GisMapSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
/// 🎯️ Decodes the expected snapshot without normalizing its child identities.
fn expected_after() -> GisMapSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> GisMapMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ Applies the mutation while preserving child identities and matching the committed snapshot.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("delete-route applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-route/removes-tram-route: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.drawing.child_id, base.drawing.child_id, "delete-route/removes-tram-route: editing routes must preserve the stable drawing identity");
    assert_eq!(snapshot.value.child_id, base.value.child_id, "delete-route/removes-tram-route: editing routes must preserve the stable value identity");
    assert!(snapshot.image.is_none(), "delete-route/removes-tram-route: gis carries no raster basemap, so the image child stays absent");
}

/// ↩️ Restores the complete committed snapshot through the inverse mutations.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_gis_map_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_gis_map_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_gis_map_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-route/removes-tram-route: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GisMapSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-route/removes-tram-route: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-route/removes-tram-route: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-route` actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "delete-route/removes-tram-route: this fixture pins an applied outcome");
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-route/removes-tram-route: an applied outcome with no declared messages must emit none, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("delete-route/removes-tram-route: declared applied but the mutation was rejected");
    assert_ne!(snapshot, before(), "delete-route/removes-tram-route: an applied delete-route must actually change the document");
}

/// 🔺️ The sparse delta `delete-route` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH of `positions`/`routes`/`regions` the
/// mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-route/removes-tram-route: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-route/removes-tram-route: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it. `GisMapDiff::apply` re-derives the
/// composed children itself, exactly as `apply_gis_map_mutation` does.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-route/removes-tram-route: committed diff did not carry before to after");
}

/// ✂️ `delete-route` writes ONE `removed` id into `routes` and — unlike a graph vocabulary's delete —
/// cascades nothing: gis map features are independent, so no position or region is disturbed by dropping a
/// route. Its inverse is BASE-derived: a `create-route` rebuilt from the pre-deletion snapshot at the index
/// the route occupied.
#[semio_framework_async_macros::async_test]
async fn removes_exactly_one_route_without_cascading_and_inverts_from_base() {
    let base = before();
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &base);
    assert!(produced.messages().is_empty(), "delete-route/removes-tram-route: deleting a present id must be diagnostic-free, got {:?}", produced.messages());
    let delta = produced.diff().routes.as_ref().expect("delete-route writes a routes delta");
    assert_eq!(delta.removed, vec!["route-tram".to_string()], "delete-route/removes-tram-route: exactly the payload's own id is removed");
    assert!(delta.added.is_empty() && delta.patched.is_empty() && delta.reordered.is_none(), "delete-route/removes-tram-route: a delete must not add, patch or reorder anything, got {delta:?}");
    assert!(produced.diff().positions.is_none() && produced.diff().regions.is_none(), "delete-route/removes-tram-route: deleting a route cascades to nothing — positions and regions stay absent from the delta");
    let inverse = inverse_gis_map_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "delete-route/removes-tram-route: a delete undoes with exactly one step — there is no severed-reference repair to add, got {inverse:?}");
    let GisMapMutation::CreateRoute(undo) = &inverse[0] else {
        panic!("delete-route/removes-tram-route: the inverse must be a create-route, got {:?}", inverse[0]);
    };
    assert_eq!(undo.index, 1, "delete-route/removes-tram-route: the inverse restores the route at the index BASE held it at");
    assert_eq!(undo.item, base.routes[1], "delete-route/removes-tram-route: the inverse re-creates the whole BASE feature, payload included");
    let semantics = <GisMapMutation as protocol::SemanticMutation<GisMapSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "route", "delete-route", "DeletedRoute"), "delete-route/removes-tram-route: the fixture must be bound to delete-route's own descriptor");
}
