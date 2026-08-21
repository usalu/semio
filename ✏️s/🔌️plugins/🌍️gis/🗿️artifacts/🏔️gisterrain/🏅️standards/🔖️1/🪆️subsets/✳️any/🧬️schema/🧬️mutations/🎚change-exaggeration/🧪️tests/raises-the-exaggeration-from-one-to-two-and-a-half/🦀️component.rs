//! 🧪️ `change-exaggeration` fixture — `raises-the-exaggeration-from-one-to-two-and-a-half`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, never here.
//!
//! 🎚️ `change-exaggeration` sets ONE scalar: its diff carries `exaggeration` and leaves
//! `importedFeaturesJson` `None`, so the untouched import survives verbatim. Because `exaggeration` is one
//! half of the mesh child's content key, changing it re-mints `mesh` — the fixture asserts both. Its inverse
//! is BASE-derived: the prior scalar.
//!
//! 🕸️ `GisTerrainSnapshot` carries one DERIVED composed child, `mesh`, whose `child_id` is a
//! `std::collections::hash_map::DefaultHasher` digest of
//! `gis_terrain_mesh_content_key(exaggeration, imported_features_json)` — the terrain's only two
//! persisted fields, so EVERY verb in this vocabulary re-mints it. `std` leaves that digest
//! deliberately unspecified, so it is never frozen into a fixture file: both committed snapshots
//! carry the readable placeholder `gisterrain-mesh-derived`, and `before()`/`expected_after()`
//! re-mint it through gis's own `gis_terrain_snapshot_with_derived_mesh` — the identical call
//! `apply_gis_terrain_mutation` and `GisTerrainDiff::apply` each make after every edit.

use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::mutations::{apply_gis_terrain_mutation, inverse_gis_terrain_mutation, GisTerrainMutation};
use crate::artifacts::gisterrain::{gis_terrain_snapshot_with_derived_mesh, GisTerrainSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

/// 🏔️ The committed `⬅️before`, with its DERIVED mesh handle minted by the artifact's own
/// composition funnel (see this file's module doc) rather than read from the placeholder JSON.
fn before() -> GisTerrainSnapshot {
    gis_terrain_snapshot_with_derived_mesh(serde_json::from_str(BEFORE).expect("before snapshot decodes"))
}
/// 🏔️ The committed `➡️after`, funnelled through the identical derivation.
fn expected_after() -> GisTerrainSnapshot {
    gis_terrain_snapshot_with_derived_mesh(serde_json::from_str(AFTER).expect("after snapshot decodes"))
}
fn mutation() -> GisTerrainMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-exaggeration` carries `before` to exactly the committed `after`, and — because the field it
/// edits is half of the mesh child's content key — re-mints the composed `mesh` handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_gis_terrain_mutation(&mut snapshot, &mutation()).expect("change-exaggeration applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: applied state differs from committed after-snapshot");
    assert_ne!(snapshot.mesh, base.mesh, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: editing a content-key field must re-mint the derived mesh handle");
    assert!(snapshot.mesh.is_some(), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: the terrain always owns a mesh child — the slot is never emptied by an edit");
}

/// ↩️ Applying `change-exaggeration` then its inverse restores `before` exactly — including the derived mesh
/// handle, which converges again only because both content-key fields did.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_gis_terrain_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_gis_terrain_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_gis_terrain_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GisTerrainSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-exaggeration` actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: this fixture pins an applied outcome");
    let produced = <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: an applied outcome with no declared messages must emit none, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_gis_terrain_mutation(&mut snapshot, &mutation()).expect("change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: declared applied but the mutation was rejected");
    assert_ne!(snapshot, before(), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: an applied change-exaggeration must actually change the document");
}

/// 🔺️ The sparse delta `change-exaggeration` produces is exactly the committed diff — it pins WHICH of the
/// terrain's two artifact-lane fields the mutation is allowed to touch, not merely the end state.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisTerrainDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it. `GisTerrainDiff::apply` re-derives
/// the composed mesh itself, exactly as `apply_gis_terrain_mutation` does.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisTerrainDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <GisTerrainDiff as protocol::MutationDiff<GisTerrainSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: committed diff did not carry before to after");
}

/// 🎚️ `change-exaggeration` sets ONE scalar: its diff carries `exaggeration` and leaves
/// `importedFeaturesJson` `None`, so the untouched import survives verbatim. Because `exaggeration` is one
/// half of the mesh child's content key, changing it re-mints `mesh` — the fixture asserts both. Its inverse
/// is BASE-derived: the prior scalar.
#[semio_framework_async_macros::async_test]
async fn changes_only_the_exaggeration_scalar_and_inverts_to_the_base_scalar() {
    let base = before();
    let after = expected_after();
    let produced = <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::diff(&mutation(), &base);
    assert!(produced.messages().is_empty(), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: a genuinely different value must be diagnostic-free (the no-op warning is the other branch), got {:?}", produced.messages());
    assert_eq!(produced.diff().exaggeration, Some(2.5), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: the delta carries the payload's own scalar");
    assert!(produced.diff().imported_features_json.is_none(), "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: the terrain's OTHER artifact-lane field must stay out of the delta");
    assert_eq!(after.imported_features_json, base.imported_features_json, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: the untouched import must survive the edit verbatim");
    let inverse = inverse_gis_terrain_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: a scalar change undoes with exactly one step, got {inverse:?}");
    let GisTerrainMutation::ChangeExaggeration(undo) = &inverse[0] else {
        panic!("change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: the inverse must be another change-exaggeration, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_exaggeration, 1.0, "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: the inverse restores BASE's scalar, not the diff's");
    let semantics = <GisTerrainMutation as protocol::SemanticMutation<GisTerrainSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("change", "exaggeration", "change-exaggeration", "ChangedExaggeration"),
        "change-exaggeration/raises-the-exaggeration-from-one-to-two-and-a-half: the fixture must be bound to change-exaggeration's own descriptor"
    );
}
