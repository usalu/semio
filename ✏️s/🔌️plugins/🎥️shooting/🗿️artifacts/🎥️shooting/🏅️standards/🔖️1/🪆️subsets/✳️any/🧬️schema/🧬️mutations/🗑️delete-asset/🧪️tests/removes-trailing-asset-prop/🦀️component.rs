//! 🧪️ `delete-asset` fixture — `removes-trailing-asset-prop`.
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
    step.diff(base).into_parts().0.apply(base).expect("delete-asset diff applies")
}

/// ▶️ `delete-asset` drops "asset-prop" from `assets` and cascades nowhere: the shots, the saved
/// cameras and both active cursors are left exactly as they were.
#[semio_framework_async_macros::async_test]
async fn removes_only_the_named_asset() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "delete-asset/removes-trailing-asset-prop: applied state differs from committed after-snapshot");
    assert!(!snapshot.assets.iter().any(|asset| asset.id == "asset-prop"), "delete-asset/removes-trailing-asset-prop: the addressed asset must be gone");
    assert_eq!(snapshot.assets[..], before().assets[..1], "delete-asset/removes-trailing-asset-prop: every other asset survives in place");
    assert_eq!(snapshot.shots, before().shots, "delete-asset/removes-trailing-asset-prop: deleting an asset must not cascade into shots");
    assert_eq!(snapshot.active_asset_id, "asset-hero", "delete-asset/removes-trailing-asset-prop: the active-asset cursor is untouched by this diff");
}

/// ↩️ The inverse is a `create-asset` rebuilt from the BASE record, so the deleted asset returns
/// with all of its fields — here it was the trailing entry, so the list order returns too.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_deleted_asset() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "delete-asset/removes-trailing-asset-prop: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-asset/removes-trailing-asset-prop: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-asset/removes-trailing-asset-prop: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the presence guard: deleting the same asset
/// twice is `mutation.target-missing` at Error, not a silent second no-op.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_second_delete_is_target_missing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-asset/removes-trailing-asset-prop: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "delete-asset/removes-trailing-asset-prop: deleting a present asset must raise no diagnostic");

    let second = mutation().diff(&expected_after());
    assert_eq!(second.worst_level(), Some(protocol::Severity::Error), "delete-asset/removes-trailing-asset-prop: deleting an absent asset is an Error, not a warning");
    assert_eq!(second.messages()[0].code.0, "mutation.target-missing", "delete-asset/removes-trailing-asset-prop: the absence guard's frozen code");
    assert_eq!(second.messages()[0].target, vec!["asset-prop".to_string()], "delete-asset/removes-trailing-asset-prop: the missing target is named");
    let unchanged = second.into_parts().0.apply(&expected_after()).expect("an Error outcome carries the default diff");
    assert_eq!(unchanged, expected_after(), "delete-asset/removes-trailing-asset-prop: a rejected delete must leave the snapshot untouched");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves `delete-asset` travels as a BARE ID in `assets.removed` — no record body, and above
/// all no cascading patch into `shots` or the active cursors.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-asset/removes-trailing-asset-prop: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["assets"]["removed"][0], "asset-prop", "delete-asset/removes-trailing-asset-prop: a delete is an id, never a record");
    assert!(committed["assets"]["patched"].as_array().expect("patched is an array").is_empty(), "delete-asset/removes-trailing-asset-prop: nothing is patched on the way out");
    assert!(committed["shots"].is_null() && committed["activeAssetId"].is_null(), "delete-asset/removes-trailing-asset-prop: the diff performs no referential cascade at all");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed delete-asset delta round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-asset/removes-trailing-asset-prop: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the lone removed id is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-asset/removes-trailing-asset-prop: committed diff did not carry before to after");
}
