//! 🧪️ `reorder-assets` fixture — `moves-asset-hero-behind-asset-prop`.
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
    step.diff(base).into_parts().0.apply(base).expect("reorder-assets diff applies")
}

/// ▶️ `reorder-assets` emits a whole-list `reordered` id sequence — remove-then-insert-at-`to_index`
/// — so the two records swap places while every field inside them stays byte-identical.
#[semio_framework_async_macros::async_test]
async fn moves_the_addressed_asset_to_the_requested_index() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "reorder-assets/moves-asset-hero-behind-asset-prop: applied state differs from committed after-snapshot");
    let order: Vec<&str> = snapshot.assets.iter().map(|asset| asset.id.as_str()).collect();
    assert_eq!(order, vec!["asset-prop", "asset-hero"], "reorder-assets/moves-asset-hero-behind-asset-prop: \"asset-hero\" must sit at index 1");
    assert_eq!(snapshot.assets[1], before().assets[0], "reorder-assets/moves-asset-hero-behind-asset-prop: reordering never rewrites a record's fields");
    assert_eq!(snapshot.assets[0], before().assets[1], "reorder-assets/moves-asset-hero-behind-asset-prop: the displaced record is carried, not rebuilt");
    assert_eq!(snapshot.active_asset_id, before().active_asset_id, "reorder-assets/moves-asset-hero-behind-asset-prop: the active-asset cursor is id-keyed, not index-keyed");
}

/// ↩️ The inverse is a `reorder-assets` back to the asset's BASE index.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_original_index() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "reorder-assets/moves-asset-hero-behind-asset-prop: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-assets/moves-asset-hero-behind-asset-prop: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-assets/moves-asset-hero-behind-asset-prop: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the order guard: replaying the same move on the
/// after-state recomputes the identical sequence, so it reports `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_an_unchanged_order_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-assets/moves-asset-hero-behind-asset-prop: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "reorder-assets/moves-asset-hero-behind-asset-prop: a real reorder must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "reorder-assets/moves-asset-hero-behind-asset-prop: an order-preserving move is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "reorder-assets/moves-asset-hero-behind-asset-prop: the order guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "reorder-assets/moves-asset-hero-behind-asset-prop: a no-op reorder applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves `reorder-assets` ships a whole-list `reordered` id sequence and NOTHING else — no
/// per-record patches, which is what guarantees the records are carried, not rewritten.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-assets/moves-asset-hero-behind-asset-prop: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["assets"]["reordered"][0], "asset-prop", "reorder-assets/moves-asset-hero-behind-asset-prop: the new order is a complete id sequence");
    assert_eq!(committed["assets"]["reordered"][1], "asset-hero", "reorder-assets/moves-asset-hero-behind-asset-prop: the moved asset is named last");
    assert!(committed["assets"]["patched"].as_array().expect("patched is an array").is_empty() && committed["assets"]["added"].as_array().expect("added is an array").is_empty(), "reorder-assets/moves-asset-hero-behind-asset-prop: reordering is pure permutation — no record is patched or re-added");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed reorder-assets sequence round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-assets/moves-asset-hero-behind-asset-prop: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the id sequence alone is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-assets/moves-asset-hero-behind-asset-prop: committed diff did not carry before to after");
}
