//! 🧪️ `create-saved-camera` fixture — `appends-saved-camera-top`.
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
    step.diff(base).into_parts().0.apply(base).expect("create-saved-camera diff applies")
}

/// ▶️ `create-saved-camera` parks a new pose in the `savedCameras` library. Nothing points at it
/// yet: the diff never rebinds a shot's `cameraId` to the camera it just created.
#[semio_framework_async_macros::async_test]
async fn parks_the_new_pose_without_binding_any_shot() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "create-saved-camera/appends-saved-camera-top: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.saved_cameras.len(), 3, "create-saved-camera/appends-saved-camera-top: exactly one saved camera must be added");
    assert_eq!(snapshot.saved_cameras[2].id, "cam-top", "create-saved-camera/appends-saved-camera-top: the new camera lands at the end of `savedCameras`");
    assert_eq!(snapshot.saved_cameras[2].camera.position, [0.0, 0.0, 20.0], "create-saved-camera/appends-saved-camera-top: the payload's pose is stored verbatim");
    assert_eq!(snapshot.shots, before().shots, "create-saved-camera/appends-saved-camera-top: no shot is rebound onto the new camera");
}

/// ↩️ The inverse is a `delete-saved-camera` for the freshly minted id.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_created_saved_camera() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "create-saved-camera/appends-saved-camera-top: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-saved-camera/appends-saved-camera-top: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-saved-camera/appends-saved-camera-top: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the id guard: a second identical create is
/// `mutation.duplicate-id` at Fatal.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_duplicate_camera_id_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-saved-camera/appends-saved-camera-top: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "create-saved-camera/appends-saved-camera-top: creating a fresh camera id must raise no diagnostic");

    let second = mutation().diff(&expected_after());
    assert_eq!(second.worst_level(), Some(protocol::Severity::Fatal), "create-saved-camera/appends-saved-camera-top: re-creating \"cam-top\" must be Fatal");
    assert_eq!(second.messages()[0].code.0, "mutation.duplicate-id", "create-saved-camera/appends-saved-camera-top: the duplicate guard's frozen code");
    assert_eq!(second.messages()[0].target, vec!["cam-top".to_string()], "create-saved-camera/appends-saved-camera-top: the duplicate is reported against the colliding camera id");
    let unchanged = second.into_parts().0.apply(&expected_after()).expect("a Fatal outcome carries the default diff");
    assert_eq!(unchanged, expected_after(), "create-saved-camera/appends-saved-camera-top: a Fatal duplicate must leave the snapshot untouched");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the new pose enters via `savedCameras.added` and that `shots` stays NULL — no shot is
/// rebound onto the camera that was just created.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-saved-camera/appends-saved-camera-top: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["savedCameras"]["added"][0]["id"], "cam-top", "create-saved-camera/appends-saved-camera-top: the new record travels in `savedCameras.added`, by value");
    assert_eq!(committed["savedCameras"]["added"][0]["camera"]["fov"], 50.0, "create-saved-camera/appends-saved-camera-top: the whole pose rides along inside the record");
    assert!(committed["shots"].is_null(), "create-saved-camera/appends-saved-camera-top: no shot is rebound onto the new camera");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed create-saved-camera delta round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-saved-camera/appends-saved-camera-top: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the `savedCameras.added` entry alone is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-saved-camera/appends-saved-camera-top: committed diff did not carry before to after");
}
