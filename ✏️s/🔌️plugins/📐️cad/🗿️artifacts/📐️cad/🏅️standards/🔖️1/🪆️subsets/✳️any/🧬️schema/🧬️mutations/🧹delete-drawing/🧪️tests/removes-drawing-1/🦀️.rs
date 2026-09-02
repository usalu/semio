//! 🧪️ `delete-drawing` fixture — `removes-drawing-1`.
//!
//! Proves the addressed drawing handle is filtered out of the whole-list diff, leaving an empty collection.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> CadSnapshot {
    serde_json::from_str(BEFORE).expect("delete-drawing/removes-drawing-1: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("delete-drawing/removes-drawing-1: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("delete-drawing/removes-drawing-1: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("delete-drawing applies to its committed before-snapshot")
}

/// ▶️ `delete-drawing` drops one handle from the Vec-cardinality composition slot.
#[semio_framework_async_macros::async_test]
async fn filters_the_addressed_drawing_out_of_the_list() {
    let after = applied();
    assert!(after.drawings.is_empty(), "delete-drawing must remove the addressed handle, leaving the collection empty");
    assert!(after.shape_model.is_some() && after.building_model.is_some(), "delete-drawing must not touch the fixed model slots");
    assert_eq!(after.nodes.len(), 2, "delete-drawing must not cascade into the node tree");
    assert_eq!(after, expected_after(), "delete-drawing/removes-drawing-1: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-drawing` carrying the escrowed handle's id AND target URI.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_drawing_with_its_target() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "delete-drawing inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateDrawing(step) => {
            assert_eq!(step.child_id, "drawing-1", "the inverse must recreate the removed drawing id");
            assert_eq!(step.target, "cad-drawing-1!s.stdio.semio@v1/drawing", "the inverse must carry the removed handle's target URI, not a stub");
        }
        other => panic!("delete-drawing must invert to create-drawing, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("delete-drawing/removes-drawing-1: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-drawing/removes-drawing-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-drawing/removes-drawing-1: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-drawing/removes-drawing-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-drawing`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-drawing/removes-drawing-1: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-drawing/removes-drawing-1: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let list = produced.diff().drawings.as_ref().expect("delete-drawing fills the drawings child list");
    assert!(list.values.is_empty(), "delete-drawing emits the WHOLE post-state list — empty here — rather than a removed-id delta");
    assert!(produced.diff().shape_model.is_none(), "delete-drawing must not emit a model-slot diff");
}

/// 🔺️ The sparse delta `delete-drawing` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here `drawings` carries the WHOLE post-state list, empty here — the removed id never appears in the diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-drawing/removes-drawing-1: delete-drawing must emit the whole post-state drawings list rather than a removed-id delta");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-drawing/removes-drawing-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `delete-drawing` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-drawing/removes-drawing-1: committed diff did not carry before to after");
}
