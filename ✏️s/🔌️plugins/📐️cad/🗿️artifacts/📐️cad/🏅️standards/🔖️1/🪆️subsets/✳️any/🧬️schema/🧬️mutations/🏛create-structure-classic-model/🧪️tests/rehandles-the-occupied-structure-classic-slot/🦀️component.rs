//! 🧪️ `create-structure-classic-model` fixture — `rehandles-the-occupied-structure-classic-slot`.
//!
//! Proves `create-structure-classic-model` OVERWRITES an already-occupied fixed slot and that undo restores the displaced handle — it is not an insert-if-absent.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> CadSnapshot {
    serde_json::from_str(BEFORE).expect("create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("create-structure-classic-model applies to its committed before-snapshot")
}

/// ▶️ `create-structure-classic-model` writes the fixed `structure_classic_model` slot even when it is already occupied; the other three slots never move.
#[semio_framework_async_macros::async_test]
async fn replaces_the_structure_classic_handle_in_place() {
    let after = applied();
    let handle = after.structure_classic_model.as_ref().expect("create-structure-classic-model leaves the slot occupied");
    assert_eq!(handle.child_id, "structure-classic-model-2", "create-structure-classic-model must install the payload's child id");
    assert_eq!(handle.target.to_uri(), "cad-structure-2!s.stdio.semio@v1/model", "create-structure-classic-model must parse the payload target URI back into a real ArtifactRef");
    assert!(
        after.shape_model.as_ref().map(|c| c.child_id.as_str()) == Some("shape-model-1")
            && after.building_model.as_ref().map(|c| c.child_id.as_str()) == Some("building-model-1")
            && after.energy_model.as_ref().map(|c| c.child_id.as_str()) == Some("energy-model-1"),
        "create-structure-classic-model must leave the other three fixed model slots untouched"
    );
    assert_eq!(after.drawings.len(), 1, "create-structure-classic-model must not touch the drawings child collection");
    assert_eq!(after, expected_after(), "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: applied state differs from the committed after-snapshot");
}

/// ↩️ Because BASE's slot was occupied, the inverse is another `create-structure-classic-model` carrying the DISPLACED handle — never a bare delete.
#[semio_framework_async_macros::async_test]
async fn inverse_reinstalls_the_displaced_structure_classic_handle() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "create-structure-classic-model inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateStructureClassicModel(step) => {
            assert_eq!(step.child_id, "structure-classic-model-1", "the inverse must reinstall the handle create-structure-classic-model displaced");
            assert_eq!(step.target, "cad-structure-1!s.stdio.semio@v1/model", "the inverse must carry the displaced handle's target URI");
        }
        other => panic!("create-structure-classic-model over an OCCUPIED slot must invert to create-structure-classic-model, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-structure-classic-model`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let slot = produced.diff().structure_classic_model.as_ref().expect("create-structure-classic-model fills the `structure_classic_model` slot diff");
    let handle = slot.as_ref().expect("create-structure-classic-model's diff sets the slot to the occupied arm");
    assert_eq!(handle.child_id, "structure-classic-model-2", "the slot diff carries the payload child id");
    assert!(produced.diff().drawings.is_none() && produced.diff().nodes.is_none(), "create-structure-classic-model emits nothing but its own slot field");
}

/// 🔺️ The sparse delta `create-structure-classic-model` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the ONLY populated field is `structureClassicModel`, carrying the occupied arm — the other three fixed slots stay null even though this create overwrites one of them.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: create-structure-classic-model must emit a diff whose sole populated field is `structureClassicModel`");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-structure-classic-model` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-structure-classic-model/rehandles-the-occupied-structure-classic-slot: committed diff did not carry before to after");
}
