//! 🧪️ `create-asset` fixture — `➕️appends-asset-detail`.
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
    step.diff(base).into_parts().0.apply(base).expect("create-asset diff applies")
}

/// ▶️ `create-asset` appends "asset-detail" and touches nothing else — note the payload asks for
/// `index: 0` yet the diff's `added` list is applied by a plain push, so the new asset lands LAST.
#[semio_framework_async_macros::async_test]
async fn appends_the_new_asset_at_the_end() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "create-asset/appends-asset-detail: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.assets.len(), 3, "create-asset/appends-asset-detail: exactly one asset must be added");
    assert_eq!(snapshot.assets[2].id, "asset-detail", "create-asset/appends-asset-detail: apply pushes onto the END of `assets`, whatever `index` the payload requested");
    assert_eq!(snapshot.assets[..2], before().assets[..], "create-asset/appends-asset-detail: the pre-existing assets must survive verbatim");
    assert_eq!(snapshot.active_asset_id, before().active_asset_id, "create-asset/appends-asset-detail: creating an asset must not move the active-asset cursor");
}

/// ↩️ The inverse is a `delete-asset` for the freshly minted id, which removes it again.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_created_asset() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "create-asset/appends-asset-detail: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-asset/appends-asset-detail: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-asset/appends-asset-detail: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the id guard: a second identical create is
/// `mutation.duplicate-id` at Fatal, carrying the default (empty) diff.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_duplicate_id_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-asset/appends-asset-detail: this fixture declares `applied`");
    let first = mutation().diff(&before());
    assert!(first.messages().is_empty(), "create-asset/appends-asset-detail: creating a fresh id must raise no diagnostic");

    let second = mutation().diff(&expected_after());
    assert_eq!(second.worst_level(), Some(protocol::Severity::Fatal), "create-asset/appends-asset-detail: re-creating \"asset-detail\" must be Fatal, not a silent overwrite");
    assert_eq!(second.messages()[0].code.0, "mutation.duplicate-id", "create-asset/appends-asset-detail: the duplicate guard's frozen code");
    assert_eq!(second.messages()[0].target, vec!["asset-detail".to_string()], "create-asset/appends-asset-detail: the duplicate is reported against the colliding asset id");
    let unchanged = second.into_parts().0.apply(&expected_after()).expect("a Fatal outcome carries the default diff");
    assert_eq!(unchanged, expected_after(), "create-asset/appends-asset-detail: a Fatal duplicate must leave the snapshot untouched");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves `create-asset` ships the WHOLE new asset record in `assets.added` and never reaches
/// into `shots`, `savedCameras` or either active cursor.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-asset/appends-asset-detail: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["assets"]["added"][0]["id"], "asset-detail", "create-asset/appends-asset-detail: the new record travels in `assets.added`, by value");
    assert!(committed["assets"]["reordered"].is_null(), "create-asset/appends-asset-detail: a create carries no explicit ordering — the payload's `index` never reaches the diff");
    assert!(committed["shots"].is_null() && committed["savedCameras"].is_null() && committed["activeAssetId"].is_null(), "create-asset/appends-asset-detail: creating an asset must touch no other slot");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed create-asset delta round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-asset/appends-asset-detail: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the `assets.added` entry alone is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-asset/appends-asset-detail: committed diff did not carry before to after");
}
