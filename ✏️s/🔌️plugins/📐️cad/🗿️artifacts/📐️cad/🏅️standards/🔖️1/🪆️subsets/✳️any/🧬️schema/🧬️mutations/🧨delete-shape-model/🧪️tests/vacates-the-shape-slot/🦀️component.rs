//! 🧪️ `delete-shape-model` fixture — `vacates-the-shape-slot`.
//!
//! Proves `delete-shape-model` clears the fixed slot to the vacated arm while the sibling slots and the drawings collection stay put.
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
    serde_json::from_str(BEFORE).expect("delete-shape-model/vacates-the-shape-slot: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("delete-shape-model/vacates-the-shape-slot: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("delete-shape-model/vacates-the-shape-slot: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("delete-shape-model applies to its committed before-snapshot")
}

/// ▶️ `delete-shape-model` empties the fixed `shape_model` slot; the composed child document itself is not this parent's business.
#[semio_framework_async_macros::async_test]
async fn clears_the_shape_slot_only() {
    let after = applied();
    assert!(after.shape_model.is_none(), "delete-shape-model must empty the fixed slot");
    assert!(
        after.building_model.as_ref().map(|c| c.child_id.as_str()) == Some("building-model-1")
            && after.energy_model.as_ref().map(|c| c.child_id.as_str()) == Some("energy-model-1")
            && after.structure_classic_model.as_ref().map(|c| c.child_id.as_str()) == Some("structure-classic-model-1"),
        "delete-shape-model must leave the other three fixed model slots occupied"
    );
    assert_eq!(after.drawings.len(), 1, "delete-shape-model must not touch the drawings child collection");
    assert_eq!(after.nodes.len(), 2, "delete-shape-model must not cascade into the node tree");
    assert_eq!(after, expected_after(), "delete-shape-model/vacates-the-shape-slot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-shape-model` carrying the handle escrowed from BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_reinstalls_the_escrowed_shape_handle() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "delete-shape-model on an occupied slot inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateShapeModel(step) => {
            assert_eq!(step.child_id, "shape-model-1", "the inverse must reinstall the escrowed child id");
            assert_eq!(step.target, "cad-shape-1!s.stdio.semio@v1/model", "the inverse must carry the escrowed handle's target URI");
        }
        other => panic!("delete-shape-model must invert to create-shape-model, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("delete-shape-model/vacates-the-shape-slot: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-shape-model/vacates-the-shape-slot: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-shape-model/vacates-the-shape-slot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-shape-model/vacates-the-shape-slot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-shape-model`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-shape-model/vacates-the-shape-slot: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-shape-model/vacates-the-shape-slot: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let slot = produced.diff().shape_model.as_ref().expect("delete-shape-model fills the `shape_model` slot diff");
    assert!(slot.is_none(), "delete-shape-model's diff sets the slot to the vacated arm (outer Some = changed, inner None = now empty)");
    assert!(produced.diff().drawings.is_none() && produced.diff().nodes.is_none(), "delete-shape-model emits nothing but its own slot field");
}

/// 🔺️ The sparse delta `delete-shape-model` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here `shapeModel` is set to the VACATED arm (`Some(None)` in memory), which
/// serde renders as `null` — the SAME rendering an untouched field gets — so the committed diff is
/// all-null; see the ⚠️ on `committed_diff_applies_to_after` below.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-shape-model/vacates-the-shape-slot: delete-shape-model must touch nothing but `shapeModel`, whose vacated arm renders as `null`");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-shape-model/vacates-the-shape-slot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before`. ⚠️ `CadDiff::shapeModel` is
/// `Option<Option<CadModelChild>>` with a plain serde derive: `Some(None)` ("vacate the slot") and
/// `None` ("leave the slot alone") BOTH serialize to `null`, and JSON `null` decodes back to the
/// OUTER `None`. So `delete-shape-model`'s vacate intent does not survive a JSON round trip, and the
/// decoded diff is inert. This test pins that hole rather than papering over it: the in-memory diff
/// DOES carry `before` to `after`; the JSON-decoded one is indistinguishable from an empty diff.
/// See `📓️census/📓️fixtures-layout-cad.md`. Fixing the wire shape (a `double_option` helper, or
/// `skip_serializing_if` so an untouched slot is OMITTED rather than `null`) must flip this test to
/// the plain `assert_eq!(produced, expected_after())` every other case uses.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let base = before();
    let in_memory = mutation().diff(&base).diff().clone();
    assert_eq!(in_memory.apply(&base).expect("the in-memory diff applies"), expected_after(), "delete-shape-model/vacates-the-shape-slot: the in-memory diff must carry before to after");

    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    assert_eq!(decoded, crate::artifacts::cad::diff::CadDiff::default(), "delete-shape-model/vacates-the-shape-slot: a `null` shapeModel is indistinguishable from an untouched one, so the decoded diff is empty");
    assert_eq!(decoded.apply(&base).expect("the decoded diff applies"), base, "delete-shape-model/vacates-the-shape-slot: the JSON-decoded diff is inert — the vacate intent is lost on the wire");
}
