//! 🧪️ `replace-position-data` fixture — `rewrites-harbor-position-payload`.
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
//! 🕸️ `GisMapSnapshot` carries two DERIVED composed children, `drawing` and `value`, whose
//! `child_id` is a `std::collections::hash_map::DefaultHasher` digest of the current
//! `(positions, routes, regions)` (`gis_map_drawing_child_handle`/`gis_map_value_child_handle`).
//! `std` leaves that digest deliberately unspecified, so it is never frozen into a fixture file:
//! both committed snapshots carry the readable placeholder `gismap-drawing-derived`/
//! `gismap-value-derived` in those two slots, and `before()`/`expected_after()` re-mint them
//! through gis's own `gis_map_snapshot_with_derived_children` — the identical call
//! `apply_gis_map_mutation` and `GisMapDiff::apply` each make after every edit. Only the
//! artifact-lane feature collections are hand-authored, and those are asserted verbatim.

use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
use crate::artifacts::gismap::{gis_map_snapshot_with_derived_children, GisMapSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

/// 🗺️ The committed `⬅️before`, with its two DERIVED child handles minted by the artifact's own
/// composition funnel (see this file's module doc) rather than read from the placeholder JSON.
fn before() -> GisMapSnapshot {
    gis_map_snapshot_with_derived_children(serde_json::from_str(BEFORE).expect("before snapshot decodes"))
}
/// 🗺️ The committed `➡️after`, funnelled through the identical derivation.
fn expected_after() -> GisMapSnapshot {
    gis_map_snapshot_with_derived_children(serde_json::from_str(AFTER).expect("after snapshot decodes"))
}
fn mutation() -> GisMapMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `replace-position-data` carries `before` to exactly the committed `after`, and — because the edited
/// `positions` collection is one third of the composed children's content key — re-mints both
/// `drawing` and `value` while leaving the honestly-absent `image` slot alone.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("replace-position-data applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-position-data/rewrites-harbor-position-payload: applied state differs from committed after-snapshot");
    assert_ne!(snapshot.drawing.child_id, base.drawing.child_id, "replace-position-data/rewrites-harbor-position-payload: editing positions must re-mint the derived drawing handle");
    assert_ne!(snapshot.value.child_id, base.value.child_id, "replace-position-data/rewrites-harbor-position-payload: editing positions must re-mint the derived value handle");
    assert!(snapshot.image.is_none(), "replace-position-data/rewrites-harbor-position-payload: gis carries no raster basemap, so the image child stays absent");
}

/// ↩️ Applying `replace-position-data` then its inverse restores `before` exactly — including both derived
/// child handles, which converge again only because the feature collections did.
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
        let decoded: GisMapSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-position-data/rewrites-harbor-position-payload: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
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
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-position-data/rewrites-harbor-position-payload: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisMapDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-position-data/rewrites-harbor-position-payload: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it. `GisMapDiff::apply` re-derives the
/// composed children itself, exactly as `apply_gis_map_mutation` does.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisMapDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
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
