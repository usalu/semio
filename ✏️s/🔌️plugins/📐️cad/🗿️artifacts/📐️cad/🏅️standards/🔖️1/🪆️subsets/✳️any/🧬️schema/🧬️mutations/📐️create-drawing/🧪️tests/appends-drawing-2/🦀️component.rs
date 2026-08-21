//! 🧪️ `create-drawing` fixture — `appends-drawing-2`.
//!
//! Proves the Vec-cardinality drawings composition grows by one handle, emitted as a WHOLE post-state list.
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
    serde_json::from_str(BEFORE).expect("create-drawing/appends-drawing-2: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("create-drawing/appends-drawing-2: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("create-drawing/appends-drawing-2: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("create-drawing applies to its committed before-snapshot")
}

/// ▶️ `create-drawing` appends one `s.stdio.semio.drawing` handle to the forward composition slot.
#[semio_framework_async_macros::async_test]
async fn appends_a_second_drawing_handle() {
    let after = applied();
    assert_eq!(after.drawings.iter().map(|handle| handle.child_id.as_str()).collect::<Vec<_>>(), vec!["drawing-1", "drawing-2"], "create-drawing appends the new handle after the existing ones");
    assert_eq!(after.drawings[1].target.to_uri(), "cad-drawing-2!s.stdio.semio@v1/drawing", "create-drawing must parse the payload target URI into a real drawing-subset ArtifactRef");
    assert!(after.shape_model.is_some(), "create-drawing must not touch the fixed model slots");
    assert_eq!(after, expected_after(), "create-drawing/appends-drawing-2: applied state differs from the committed after-snapshot");
}

/// ↩️ `create-drawing` always inverts to `delete-drawing` of the id it minted — it never inspects BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_drawing_it_created() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "create-drawing inverts to exactly one step");
    match &inverse[0] {
        CadMutation::DeleteDrawing(step) => assert_eq!(step.child_id, "drawing-2", "the inverse must delete the drawing id create-drawing minted"),
        other => panic!("create-drawing must invert to delete-drawing, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("create-drawing/appends-drawing-2: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-drawing/appends-drawing-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-drawing/appends-drawing-2: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-drawing/appends-drawing-2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-drawing`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-drawing/appends-drawing-2: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-drawing/appends-drawing-2: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let list = produced.diff().drawings.as_ref().expect("create-drawing fills the drawings child list");
    assert_eq!(list.values.iter().map(|handle| handle.child_id.as_str()).collect::<Vec<_>>(), vec!["drawing-1", "drawing-2"], "create-drawing emits the WHOLE post-state list, not an added/removed delta");
    assert!(produced.diff().nodes.is_none(), "create-drawing must not emit a nodes delta");
}

/// 🔺️ The sparse delta `create-drawing` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here `drawings` carries the WHOLE post-state handle list (the existing handle plus the new one) — unlike `nodes` there is no added/removed delta for this composition slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-drawing/appends-drawing-2: create-drawing must emit the whole post-state drawings list, not an added-only delta");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-drawing/appends-drawing-2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-drawing` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-drawing/appends-drawing-2: committed diff did not carry before to after");
}
