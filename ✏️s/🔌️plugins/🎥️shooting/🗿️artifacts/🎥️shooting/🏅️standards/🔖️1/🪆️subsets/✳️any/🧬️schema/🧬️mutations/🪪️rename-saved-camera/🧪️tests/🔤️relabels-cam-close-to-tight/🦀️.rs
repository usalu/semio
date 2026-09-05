//! 🧪️ `rename-saved-camera` fixture — `🔤️relabels-cam-close-to-tight`.
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
    step.diff(base).into_parts().0.apply(base).expect("rename-saved-camera diff applies")
}

/// ▶️ `rename-saved-camera` builds the saved-camera patch with `label: Some(..)` and `camera: None`
/// — that explicit `None` is what keeps the stored pose out of the write.
#[semio_framework_async_macros::async_test]
async fn relabels_without_rewriting_the_pose() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "rename-saved-camera/relabels-cam-close-to-tight: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.saved_cameras[1].label, "Tight", "rename-saved-camera/relabels-cam-close-to-tight: the new label must land on \"cam-close\"");
    assert_eq!(snapshot.saved_cameras[1].camera, before().saved_cameras[1].camera, "rename-saved-camera/relabels-cam-close-to-tight: the patch's `camera: None` leaves the pose untouched");
    assert_eq!(snapshot.saved_cameras[1].id, "cam-close", "rename-saved-camera/relabels-cam-close-to-tight: a relabel never re-keys the camera");
    assert_eq!(snapshot.saved_cameras[0], before().saved_cameras[0], "rename-saved-camera/relabels-cam-close-to-tight: the other camera is untouched");
}

/// ↩️ The inverse is a `rename-saved-camera` back to the BASE label.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_label() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "rename-saved-camera/relabels-cam-close-to-tight: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-saved-camera/relabels-cam-close-to-tight: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-saved-camera/relabels-cam-close-to-tight: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the equality guard: relabelling to the label the
/// camera already carries is `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_relabelling_to_the_same_label_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-saved-camera/relabels-cam-close-to-tight: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "rename-saved-camera/relabels-cam-close-to-tight: a real relabel must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "rename-saved-camera/relabels-cam-close-to-tight: relabelling to the current label is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "rename-saved-camera/relabels-cam-close-to-tight: the equality guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "rename-saved-camera/relabels-cam-close-to-tight: a no-op relabel applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the `ShootingSavedCameraPatch`'s two slots are used as `{label: Some, camera: None}` —
/// the explicit null `camera` is what keeps the stored pose out of the write.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-saved-camera/relabels-cam-close-to-tight: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["savedCameras"]["patched"][0]["patch"]["label"], "Tight", "rename-saved-camera/relabels-cam-close-to-tight: `label` is the filled patch slot");
    assert!(committed["savedCameras"]["patched"][0]["patch"]["camera"].is_null(), "rename-saved-camera/relabels-cam-close-to-tight: the `camera` slot is explicitly null in the delta");
    assert!(committed["shots"].is_null(), "rename-saved-camera/relabels-cam-close-to-tight: relabelling a camera never opens the shot collection");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed rename-saved-camera patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-saved-camera/relabels-cam-close-to-tight: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — a one-slot patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-saved-camera/relabels-cam-close-to-tight: committed diff did not carry before to after");
}
