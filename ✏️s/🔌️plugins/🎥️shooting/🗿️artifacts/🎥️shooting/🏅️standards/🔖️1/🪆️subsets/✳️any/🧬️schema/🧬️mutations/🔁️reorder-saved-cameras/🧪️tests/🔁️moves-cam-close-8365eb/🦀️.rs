//! 🧪️ `reorder-saved-cameras` fixture — `🔁️moves-cam-close-to-front`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingDiff, ShootingSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> ShootingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> ShootingSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> ShootingMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn apply(base: &ShootingSnapshot, step: &ShootingMutation) -> ShootingSnapshot {
    step.diff(base).into_parts().0.apply(base).expect("reorder-saved-cameras diff applies")
}

/// ▶️ `reorder-saved-cameras` reshuffles the camera library's presentation order. Because shots
/// reference cameras BY ID, "shot-wide" keeps resolving to "cam-wide" even though it is now last.
#[semio_framework_async_macros::async_test]
async fn reshuffles_the_library_without_breaking_shot_references() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "reorder-saved-cameras/moves-cam-close-to-front: applied state differs from committed after-snapshot");
    let order: Vec<&str> = snapshot.saved_cameras.iter().map(|camera| camera.id.as_str()).collect();
    assert_eq!(order, vec!["cam-close", "cam-wide"], "reorder-saved-cameras/moves-cam-close-to-front: \"cam-close\" must sit at index 0");
    assert_eq!(snapshot.saved_cameras[0], before().saved_cameras[1], "reorder-saved-cameras/moves-cam-close-to-front: reordering never rewrites a camera record");
    assert_eq!(snapshot.shots[0].camera_id.as_deref(), Some("cam-wide"), "reorder-saved-cameras/moves-cam-close-to-front: the shot's binding is an id, so it survives the reshuffle");
    assert_eq!(snapshot.shots, before().shots, "reorder-saved-cameras/moves-cam-close-to-front: no shot record is rewritten");
}

/// ↩️ The inverse is a `reorder-saved-cameras` back to the camera's BASE index.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_original_index() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "reorder-saved-cameras/moves-cam-close-to-front: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-saved-cameras/moves-cam-close-to-front: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-saved-cameras/moves-cam-close-to-front: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the order guard: promoting an already-first
/// camera reports `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_an_unchanged_order_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-saved-cameras/moves-cam-close-to-front: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "reorder-saved-cameras/moves-cam-close-to-front: a real reorder must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "reorder-saved-cameras/moves-cam-close-to-front: an order-preserving move is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "reorder-saved-cameras/moves-cam-close-to-front: the order guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "reorder-saved-cameras/moves-cam-close-to-front: a no-op reorder applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the library permutation ships as a `savedCameras.reordered` id sequence with `shots`
/// left NULL — the delta is the proof that id-keyed shot bindings need no repair.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-saved-cameras/moves-cam-close-to-front: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["savedCameras"]["reordered"][0], "cam-close", "reorder-saved-cameras/moves-cam-close-to-front: the promoted camera heads the sequence");
    assert!(committed["shots"].is_null(), "reorder-saved-cameras/moves-cam-close-to-front: no shot binding is rewritten to chase the new index");
    assert!(committed["savedCameras"]["patched"].as_array().expect("patched is an array").is_empty(), "reorder-saved-cameras/moves-cam-close-to-front: reordering is pure permutation");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed reorder-saved-cameras sequence round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-saved-cameras/moves-cam-close-to-front: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the id sequence alone is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-saved-cameras/moves-cam-close-to-front: committed diff did not carry before to after");
}
