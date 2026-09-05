//! 🧪️ `set-active-asset` fixture — `📌️activates-asset-prop`.
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
    step.diff(base).into_parts().0.apply(base).expect("set-active-asset diff applies")
}

/// ▶️ `set-active-asset` writes the document-root `activeAssetId` scalar and validates the id
/// against `assets` — the asset records themselves are never part of the diff.
#[semio_framework_async_macros::async_test]
async fn moves_only_the_active_asset_cursor() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "set-active-asset/activates-asset-prop: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.active_asset_id, "asset-prop", "set-active-asset/activates-asset-prop: the cursor must name the requested asset");
    assert_eq!(snapshot.active_shot_id, before().active_shot_id, "set-active-asset/activates-asset-prop: the active-shot cursor is a separate mutation's business");
    assert_eq!(snapshot.assets, before().assets, "set-active-asset/activates-asset-prop: no asset record is touched");
    assert_eq!(snapshot.saved_cameras, before().saved_cameras, "set-active-asset/activates-asset-prop: the camera library is untouched");
}

/// ↩️ The inverse re-reads the BASE cursor and names the previously active asset.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_cursor() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "set-active-asset/activates-asset-prop: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-active-asset/activates-asset-prop: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-active-asset/activates-asset-prop: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and this leaf's guards: re-activating the already
/// active asset is `mutation.no-op`, while an unknown id is `mutation.target-missing` at Error.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_reactivating_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-active-asset/activates-asset-prop: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "set-active-asset/activates-asset-prop: activating a different asset must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "set-active-asset/activates-asset-prop: re-activating the current asset is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "set-active-asset/activates-asset-prop: the cursor-equality guard's frozen code");

    let ghost: ShootingMutation = serde_json::from_str(r#"{"mutation":"setActiveAsset","asset_id":"asset-ghost"}"#).expect("probe mutation decodes");
    let rejected = ghost.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Error), "set-active-asset/activates-asset-prop: activating an unknown asset is an Error");
    assert_eq!(rejected.messages()[0].code.0, "mutation.target-missing", "set-active-asset/activates-asset-prop: the existence guard's frozen code");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the asset cursor writes `activeAssetId` and NOT `activeShotId` — two same-shaped
/// scalar slots that only the delta can tell apart.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-active-asset/activates-asset-prop: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["activeAssetId"], "asset-prop", "set-active-asset/activates-asset-prop: the scalar slot carries the new cursor");
    assert!(committed["activeShotId"].is_null(), "set-active-asset/activates-asset-prop: the sibling shot cursor slot stays null");
    assert!(committed["assets"].is_null(), "set-active-asset/activates-asset-prop: validating the id against `assets` does not mean patching `assets`");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed scalar delta round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-active-asset/activates-asset-prop: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single scalar is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-active-asset/activates-asset-prop: committed diff did not carry before to after");
}
