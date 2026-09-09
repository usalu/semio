//! 🧪️ `replace-position-data` fixture — `⚓️rewrites-harbor-position-payload`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, never here.
//!
//! 🔁️ `replace-position-data` is a WHOLE-value swap of the opaque `MapFeature::data` payload —
//! `MapFeaturePatch` has exactly one field, so the delta is a single `patched` entry and the feature's
//! identity and list position are untouched. Its inverse is BASE-derived: the prior payload, read out of the
//! pre-mutation snapshot.
//!
//! 🧩️ Committed snapshots preserve the stable drawing and value child identities across edits.
//! Feature collections and their sparse deltas are asserted directly against the neutral fixtures.

use crate::diff::GisMapDiff;
use crate::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
use crate::GisMapSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁replace-position-data/⚓️rewrites-harbor-position-payload/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁replace-position-data/⚓️rewrites-harbor-position-payload/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁replace-position-data/⚓️rewrites-harbor-position-payload/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁replace-position-data/⚓️rewrites-harbor-position-payload/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁replace-position-data/⚓️rewrites-harbor-position-payload/🎯️outcome/🔣️.json");

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
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("replace-position-data applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-position-data/rewrites-harbor-position-payload: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.drawing.child_id, base.drawing.child_id, "replace-position-data/rewrites-harbor-position-payload: editing positions must preserve the stable drawing identity");
    assert_eq!(snapshot.value.child_id, base.value.child_id, "replace-position-data/rewrites-harbor-position-payload: editing positions must preserve the stable value identity");
    assert!(snapshot.image.is_none(), "replace-position-data/rewrites-harbor-position-payload: gis carries no raster basemap, so the image child stays absent");
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
    assert_eq!(snapshot, base, "replace-position-data/rewrites-harbor-position-payload: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GisMapSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-position-data/rewrites-harbor-position-payload: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-position-data/rewrites-harbor-position-payload: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-position-data` actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "replace-position-data/rewrites-harbor-position-payload: this fixture pins an applied outcome");
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "replace-position-data/rewrites-harbor-position-payload: an applied outcome with no declared messages must emit none, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("replace-position-data/rewrites-harbor-position-payload: declared applied but the mutation was rejected");
    assert_ne!(snapshot, before(), "replace-position-data/rewrites-harbor-position-payload: an applied replace-position-data must actually change the document");
}

/// 🔺️ The sparse delta `replace-position-data` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH of `positions`/`routes`/`regions` the
/// mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-position-data/rewrites-harbor-position-payload: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-position-data/rewrites-harbor-position-payload: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it. `GisMapDiff::apply` re-derives the
/// composed children itself, exactly as `apply_gis_map_mutation` does.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-position-data/rewrites-harbor-position-payload: committed diff did not carry before to after");
}

/// 🔁️ `replace-position-data` is a WHOLE-value swap of the opaque `MapFeature::data` payload —
/// `MapFeaturePatch` has exactly one field, so the delta is a single `patched` entry and the feature's
/// identity and list position are untouched. Its inverse is BASE-derived: the prior payload, read out of the
/// pre-mutation snapshot.
#[semio_framework_async_macros::async_test]
async fn patches_only_the_harbor_payload_and_inverts_to_the_base_payload() {
    let base = before();
    let after = expected_after();
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &base);
    assert!(produced.messages().is_empty(), "replace-position-data/rewrites-harbor-position-payload: a genuinely different payload must be diagnostic-free (the no-op warning is the other branch), got {:?}", produced.messages());
    let delta = produced.diff().positions.as_ref().expect("replace-position-data writes a positions delta");
    assert_eq!(delta.patched.len(), 1, "replace-position-data/rewrites-harbor-position-payload: exactly one feature is patched, got {delta:?}");
    assert_eq!(delta.patched[0].id, "pos-harbor", "replace-position-data/rewrites-harbor-position-payload: the patch is addressed by the payload's own id");
    assert_eq!(delta.patched[0].patch.data.as_ref(), Some(&after.positions[0].data), "replace-position-data/rewrites-harbor-position-payload: the patch carries the committed replacement payload verbatim");
    assert!(delta.added.is_empty() && delta.removed.is_empty() && delta.reordered.is_none(), "replace-position-data/rewrites-harbor-position-payload: a payload swap must not add, remove or reorder anything, got {delta:?}");
    assert!(produced.diff().routes.is_none() && produced.diff().regions.is_none(), "replace-position-data/rewrites-harbor-position-payload: replace-position-data must never touch the routes or regions collections");
    let inverse = inverse_gis_map_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "replace-position-data/rewrites-harbor-position-payload: a payload swap undoes with exactly one step, got {inverse:?}");
    let GisMapMutation::ReplacePositionData(undo) = &inverse[0] else {
        panic!("replace-position-data/rewrites-harbor-position-payload: the inverse must be another replace-position-data, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "pos-harbor", "replace-position-data/rewrites-harbor-position-payload: the inverse addresses the same position");
    assert_eq!(undo.new_data, base.positions[0].data, "replace-position-data/rewrites-harbor-position-payload: the inverse restores BASE's payload, not the diff's");
    let semantics = <GisMapMutation as protocol::SemanticMutation<GisMapSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("replace", "position-data", "replace-position-data", "ReplacedPositionData"),
        "replace-position-data/rewrites-harbor-position-payload: the fixture must be bound to replace-position-data's own descriptor"
    );
}
