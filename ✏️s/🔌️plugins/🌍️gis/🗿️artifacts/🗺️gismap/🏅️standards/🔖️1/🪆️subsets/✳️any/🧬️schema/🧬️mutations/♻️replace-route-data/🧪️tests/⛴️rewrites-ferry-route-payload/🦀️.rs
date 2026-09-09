//! 🧪️ `replace-route-data` fixture — `⛴️rewrites-ferry-route-payload`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, never here.
//!
//! ♻️ `replace-route-data` swaps the route's whole opaque payload — the replacement here even GROWS the
//! object with a key the prior payload never had, which is exactly why the vocabulary offers no partial
//! `change`: `MapFeature::data` is deliberately untyped, so only a whole-value replace is expressible. Its
//! inverse is BASE-derived.
//!
//! 🧩️ Committed snapshots preserve the stable drawing and value child identities across edits.
//! Feature collections and their sparse deltas are asserted directly against the neutral fixtures.

use crate::diff::GisMapDiff;
use crate::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
use crate::GisMapSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-route-data/⛴️rewrites-ferry-route-payload/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-route-data/⛴️rewrites-ferry-route-payload/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-route-data/⛴️rewrites-ferry-route-payload/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-route-data/⛴️rewrites-ferry-route-payload/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-route-data/⛴️rewrites-ferry-route-payload/🎯️outcome/🔣️.json");

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
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("replace-route-data applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-route-data/rewrites-ferry-route-payload: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.drawing.child_id, base.drawing.child_id, "replace-route-data/rewrites-ferry-route-payload: editing routes must preserve the stable drawing identity");
    assert_eq!(snapshot.value.child_id, base.value.child_id, "replace-route-data/rewrites-ferry-route-payload: editing routes must preserve the stable value identity");
    assert!(snapshot.image.is_none(), "replace-route-data/rewrites-ferry-route-payload: gis carries no raster basemap, so the image child stays absent");
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
    assert_eq!(snapshot, base, "replace-route-data/rewrites-ferry-route-payload: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GisMapSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-route-data/rewrites-ferry-route-payload: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-route-data/rewrites-ferry-route-payload: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-route-data` actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "replace-route-data/rewrites-ferry-route-payload: this fixture pins an applied outcome");
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "replace-route-data/rewrites-ferry-route-payload: an applied outcome with no declared messages must emit none, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("replace-route-data/rewrites-ferry-route-payload: declared applied but the mutation was rejected");
    assert_ne!(snapshot, before(), "replace-route-data/rewrites-ferry-route-payload: an applied replace-route-data must actually change the document");
}

/// 🔺️ The sparse delta `replace-route-data` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH of `positions`/`routes`/`regions` the
/// mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-route-data/rewrites-ferry-route-payload: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-route-data/rewrites-ferry-route-payload: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it. `GisMapDiff::apply` re-derives the
/// composed children itself, exactly as `apply_gis_map_mutation` does.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-route-data/rewrites-ferry-route-payload: committed diff did not carry before to after");
}

/// ♻️ `replace-route-data` swaps the route's whole opaque payload — the replacement here even GROWS the
/// object with a key the prior payload never had, which is exactly why the vocabulary offers no partial
/// `change`: `MapFeature::data` is deliberately untyped, so only a whole-value replace is expressible. Its
/// inverse is BASE-derived.
#[semio_framework_async_macros::async_test]
async fn patches_only_the_ferry_payload_and_inverts_to_the_base_payload() {
    let base = before();
    let after = expected_after();
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &base);
    assert!(produced.messages().is_empty(), "replace-route-data/rewrites-ferry-route-payload: a genuinely different payload must be diagnostic-free (the no-op warning is the other branch), got {:?}", produced.messages());
    let delta = produced.diff().routes.as_ref().expect("replace-route-data writes a routes delta");
    assert_eq!(delta.patched.len(), 1, "replace-route-data/rewrites-ferry-route-payload: exactly one feature is patched, got {delta:?}");
    assert_eq!(delta.patched[0].id, "route-ferry", "replace-route-data/rewrites-ferry-route-payload: the patch is addressed by the payload's own id");
    assert_eq!(delta.patched[0].patch.data.as_ref(), Some(&after.routes[0].data), "replace-route-data/rewrites-ferry-route-payload: the patch carries the committed replacement payload verbatim");
    assert!(
        base.routes[0].data.get("seasonal").is_none() && after.routes[0].data.get("seasonal").is_some(),
        "replace-route-data/rewrites-ferry-route-payload: this case exists to prove a WHOLE-value swap, so the replacement must introduce a key the prior payload lacked"
    );
    assert!(delta.added.is_empty() && delta.removed.is_empty() && delta.reordered.is_none(), "replace-route-data/rewrites-ferry-route-payload: a payload swap must not add, remove or reorder anything, got {delta:?}");
    assert!(produced.diff().positions.is_none() && produced.diff().regions.is_none(), "replace-route-data/rewrites-ferry-route-payload: replace-route-data must never touch the positions or regions collections");
    let inverse = inverse_gis_map_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "replace-route-data/rewrites-ferry-route-payload: a payload swap undoes with exactly one step, got {inverse:?}");
    let GisMapMutation::ReplaceRouteData(undo) = &inverse[0] else {
        panic!("replace-route-data/rewrites-ferry-route-payload: the inverse must be another replace-route-data, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "route-ferry", "replace-route-data/rewrites-ferry-route-payload: the inverse addresses the same route");
    assert_eq!(undo.new_data, base.routes[0].data, "replace-route-data/rewrites-ferry-route-payload: the inverse restores BASE's payload, not the diff's");
    let semantics = <GisMapMutation as protocol::SemanticMutation<GisMapSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("replace", "route-data", "replace-route-data", "ReplacedRouteData"),
        "replace-route-data/rewrites-ferry-route-payload: the fixture must be bound to replace-route-data's own descriptor"
    );
}
