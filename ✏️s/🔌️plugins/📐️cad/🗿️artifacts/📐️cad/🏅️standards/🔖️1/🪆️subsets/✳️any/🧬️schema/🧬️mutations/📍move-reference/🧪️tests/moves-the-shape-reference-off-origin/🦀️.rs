//! 🧪️ `move-reference` fixture — `moves-the-shape-reference-off-origin`.
//!
//! Proves the 3-component origin is replaced as a unit while orientation and scale stay fixed.
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
    serde_json::from_str(BEFORE).expect("move-reference/moves-the-shape-reference-off-origin: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("move-reference/moves-the-shape-reference-off-origin: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("move-reference/moves-the-shape-reference-off-origin: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("move-reference applies to its committed before-snapshot")
}

/// ▶️ `move-reference` writes all three origin components at once; orientation and scale are untouched.
#[semio_framework_async_macros::async_test]
async fn translates_the_reference_origin_only() {
    let after = applied();
    let rows = &after.references_by_model_definition_id["spatial.shape"];
    let reference = rows.iter().find(|reference| reference.id == "ref-1").expect("ref-1 survives");
    assert_eq!(reference.origin, [1.0, 2.0, 3.0], "move-reference must write all three origin components from the payload");
    assert!(reference.orientation.is_none(), "move-reference must not invent an orientation");
    assert_eq!(reference.scale, Some(1.5), "move-reference must leave the uniform scale factor untouched");
    assert_eq!(after, expected_after(), "move-reference/moves-the-shape-reference-off-origin: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `move-reference` carrying BASE's origin triple.
#[semio_framework_async_macros::async_test]
async fn inverse_moves_the_reference_back_to_the_origin() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "move-reference inverts to exactly one step");
    match &inverse[0] {
        CadMutation::MoveReference(step) => {
            assert_eq!((step.model_definition_id.as_str(), step.reference_id.as_str()), ("spatial.shape", "ref-1"), "the inverse must address the same reference in the same bucket");
            assert_eq!(step.new_origin, [0.0, 0.0, 0.0], "the inverse must carry the pre-move origin triple");
        }
        other => panic!("move-reference must invert to move-reference, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("move-reference/moves-the-shape-reference-off-origin: inverse step applies");
    }
    assert_eq!(snapshot, base, "move-reference/moves-the-shape-reference-off-origin: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-reference/moves-the-shape-reference-off-origin: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-reference/moves-the-shape-reference-off-origin: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `move-reference`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-reference/moves-the-shape-reference-off-origin: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "move-reference/moves-the-shape-reference-off-origin: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let map = produced.diff().references_by_model_definition_id.as_ref().expect("move-reference fills the references map");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert_eq!(rows[0].origin, [1.0, 2.0, 3.0], "the emitted bucket carries the whole post-patch reference row");
    assert_eq!(rows[0].width_world, 8.0, "the emitted row keeps the world width the mutation did not address");
}

/// 🔺️ The sparse delta `move-reference` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the emitted row differs from BASE in `origin` alone.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-reference/moves-the-shape-reference-off-origin: move-reference must emit one bucket whose row differs from BASE in `origin` alone");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-reference/moves-the-shape-reference-off-origin: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `move-reference` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-reference/moves-the-shape-reference-off-origin: committed diff did not carry before to after");
}
