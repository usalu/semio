//! 🧪️ `set-active-shot` fixture — `activates-shot-close`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingDiff, ShootingSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

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
    step.diff(base).into_parts().0.apply(base).expect("set-active-shot diff applies")
}

/// ▶️ `set-active-shot` writes the document-root `activeShotId` scalar — no collection delta at all
/// — and leaves the sibling `activeAssetId` cursor exactly where it was.
#[semio_framework_async_macros::async_test]
async fn moves_only_the_active_shot_cursor() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "set-active-shot/activates-shot-close: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.active_shot_id, "shot-close", "set-active-shot/activates-shot-close: the cursor must name the requested shot");
    assert_eq!(snapshot.active_asset_id, before().active_asset_id, "set-active-shot/activates-shot-close: the active-asset cursor is a separate mutation's business");
    assert_eq!(snapshot.shots, before().shots, "set-active-shot/activates-shot-close: no shot record is touched");
    assert_eq!(snapshot.scene, before().scene, "set-active-shot/activates-shot-close: the scene is untouched");
}

/// ↩️ The inverse re-reads the BASE cursor: a non-empty `activeShotId` comes back as `Some(..)`,
/// which is why the round trip lands exactly on the before-snapshot.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_cursor() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "set-active-shot/activates-shot-close: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-active-shot/activates-shot-close: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-active-shot/activates-shot-close: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — plus this leaf's two distinctive guards: an unknown
/// id is `mutation.target-missing`, while a `null` `shot_id` is legal and CLEARS the cursor to `""`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_null_shot_id_clears_the_cursor() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-active-shot/activates-shot-close: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "set-active-shot/activates-shot-close: activating a real shot must raise no diagnostic");

    let ghost: ShootingMutation = serde_json::from_str(r#"{"mutation":"setActiveShot","shot_id":"shot-ghost"}"#).expect("probe mutation decodes");
    let rejected = ghost.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Error), "set-active-shot/activates-shot-close: activating an unknown shot is an Error");
    assert_eq!(rejected.messages()[0].code.0, "mutation.target-missing", "set-active-shot/activates-shot-close: the existence guard's frozen code");

    let cleared: ShootingMutation = serde_json::from_str(r#"{"mutation":"setActiveShot","shot_id":null}"#).expect("probe mutation decodes");
    let empty = apply(&before(), &cleared);
    assert_eq!(empty.active_shot_id, "", "set-active-shot/activates-shot-close: a null `shot_id` is the legal \"no active shot\" state, stored as the empty string");
    assert!(cleared.diff(&before()).messages().is_empty(), "set-active-shot/activates-shot-close: clearing a set cursor is a real change, not a no-op");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the cursor move is a bare document-root scalar: `activeShotId` filled, and ALL THREE
/// collection slots null — a cursor move can never smuggle a collection edit.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-active-shot/activates-shot-close: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["activeShotId"], "shot-close", "set-active-shot/activates-shot-close: the scalar slot carries the new cursor");
    assert!(committed["assets"].is_null() && committed["shots"].is_null() && committed["savedCameras"].is_null(), "set-active-shot/activates-shot-close: no collection delta is opened at all");
    assert!(committed["activeAssetId"].is_null(), "set-active-shot/activates-shot-close: the sibling asset cursor slot stays null");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed scalar delta round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-active-shot/activates-shot-close: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single scalar is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-active-shot/activates-shot-close: committed diff did not carry before to after");
}
