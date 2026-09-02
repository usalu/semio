//! 🧪️ `create-region` fixture — `adds-old-town-region-after-harbor-district`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, never here.
//!
//! 🌐 `create-region` writes ONE `added` entry into `regions`. The two regions in this case deliberately
//! share a `kind` — id, not payload, is what the duplicate-id guard and the delta address, so a second
//! `district` is a perfectly legal create. Its inverse is PAYLOAD-derived: a `delete-region` of the id it was
//! asked to create.
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

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

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

/// ▶️ `create-region` carries `before` to exactly the committed `after`, and — because the edited
/// `regions` collection is one third of the composed children's content key — re-mints both
/// `drawing` and `value` while leaving the honestly-absent `image` slot alone.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("create-region applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-region/adds-old-town-region-after-harbor-district: applied state differs from committed after-snapshot");
    assert_ne!(snapshot.drawing.child_id, base.drawing.child_id, "create-region/adds-old-town-region-after-harbor-district: editing regions must re-mint the derived drawing handle");
    assert_ne!(snapshot.value.child_id, base.value.child_id, "create-region/adds-old-town-region-after-harbor-district: editing regions must re-mint the derived value handle");
    assert!(snapshot.image.is_none(), "create-region/adds-old-town-region-after-harbor-district: gis carries no raster basemap, so the image child stays absent");
}

/// ↩️ Applying `create-region` then its inverse restores `before` exactly — including both derived
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
    assert_eq!(snapshot, base, "create-region/adds-old-town-region-after-harbor-district: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GisMapSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-region/adds-old-town-region-after-harbor-district: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-region/adds-old-town-region-after-harbor-district: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-region` actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "create-region/adds-old-town-region-after-harbor-district: this fixture pins an applied outcome");
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "create-region/adds-old-town-region-after-harbor-district: an applied outcome with no declared messages must emit none, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("create-region/adds-old-town-region-after-harbor-district: declared applied but the mutation was rejected");
    assert_ne!(snapshot, before(), "create-region/adds-old-town-region-after-harbor-district: an applied create-region must actually change the document");
}

/// 🔺️ The sparse delta `create-region` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH of `positions`/`routes`/`regions` the
/// mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-region/adds-old-town-region-after-harbor-district: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisMapDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-region/adds-old-town-region-after-harbor-district: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it. `GisMapDiff::apply` re-derives the
/// composed children itself, exactly as `apply_gis_map_mutation` does.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisMapDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-region/adds-old-town-region-after-harbor-district: committed diff did not carry before to after");
}

/// 🌐 `create-region` writes ONE `added` entry into `regions`. The two regions in this case deliberately
/// share a `kind` — id, not payload, is what the duplicate-id guard and the delta address, so a second
/// `district` is a perfectly legal create. Its inverse is PAYLOAD-derived: a `delete-region` of the id it was
/// asked to create.
#[semio_framework_async_macros::async_test]
async fn adds_exactly_one_region_and_inverts_to_a_delete_of_that_id() {
    let base = before();
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &base);
    assert!(produced.messages().is_empty(), "create-region/adds-old-town-region-after-harbor-district: creating a fresh id must be diagnostic-free even when an existing region shares its kind, got {:?}", produced.messages());
    let delta = produced.diff().regions.as_ref().expect("create-region writes a regions delta");
    assert_eq!(delta.added.iter().map(|feature| feature.id.as_str()).collect::<Vec<_>>(), vec!["region-old-town"], "create-region/adds-old-town-region-after-harbor-district: exactly the payload's own feature is added");
    assert!(delta.removed.is_empty() && delta.patched.is_empty() && delta.reordered.is_none(), "create-region/adds-old-town-region-after-harbor-district: a create must not remove, patch or reorder anything, got {delta:?}");
    assert!(produced.diff().positions.is_none() && produced.diff().routes.is_none(), "create-region/adds-old-town-region-after-harbor-district: create-region must never touch the positions or routes collections");
    let inverse = inverse_gis_map_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "create-region/adds-old-town-region-after-harbor-district: a create undoes with exactly one step, got {inverse:?}");
    let GisMapMutation::DeleteRegion(undo) = &inverse[0] else {
        panic!("create-region/adds-old-town-region-after-harbor-district: the inverse must be a delete-region, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "region-old-town", "create-region/adds-old-town-region-after-harbor-district: the inverse deletes exactly the id the payload carried");
    let semantics = <GisMapMutation as protocol::SemanticMutation<GisMapSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("create", "region", "create-region", "CreatedRegion"),
        "create-region/adds-old-town-region-after-harbor-district: the fixture must be bound to create-region's own descriptor"
    );
}
