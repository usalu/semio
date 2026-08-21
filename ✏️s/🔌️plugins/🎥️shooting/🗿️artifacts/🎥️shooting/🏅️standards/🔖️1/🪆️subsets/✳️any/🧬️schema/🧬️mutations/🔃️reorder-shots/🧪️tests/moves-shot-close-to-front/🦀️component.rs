//! 🧪️ `reorder-shots` fixture — `moves-shot-close-to-front`.
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
    step.diff(base).into_parts().0.apply(base).expect("reorder-shots diff applies")
}

/// ▶️ `reorder-shots` promotes "shot-close" to the head of the storyboard. The active-shot cursor
/// keeps naming "shot-wide" — it is an id, so it does not follow position 0.
#[semio_framework_async_macros::async_test]
async fn promotes_the_addressed_shot_to_the_head() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "reorder-shots/moves-shot-close-to-front: applied state differs from committed after-snapshot");
    let order: Vec<&str> = snapshot.shots.iter().map(|shot| shot.id.as_str()).collect();
    assert_eq!(order, vec!["shot-close", "shot-wide"], "reorder-shots/moves-shot-close-to-front: \"shot-close\" must sit at index 0");
    assert_eq!(snapshot.shots[0], before().shots[1], "reorder-shots/moves-shot-close-to-front: reordering never rewrites a record's fields");
    assert_eq!(snapshot.active_shot_id, "shot-wide", "reorder-shots/moves-shot-close-to-front: the active-shot cursor stays on its id, it does not follow index 0");
    assert_eq!(snapshot.saved_cameras, before().saved_cameras, "reorder-shots/moves-shot-close-to-front: reordering shots never touches the saved cameras");
}

/// ↩️ The inverse is a `reorder-shots` back to the shot's BASE index.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_original_index() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "reorder-shots/moves-shot-close-to-front: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-shots/moves-shot-close-to-front: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-shots/moves-shot-close-to-front: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the order guard: promoting an already-first
/// shot recomputes the identical sequence and reports `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_an_unchanged_order_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-shots/moves-shot-close-to-front: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "reorder-shots/moves-shot-close-to-front: a real reorder must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "reorder-shots/moves-shot-close-to-front: an order-preserving move is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "reorder-shots/moves-shot-close-to-front: the order guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "reorder-shots/moves-shot-close-to-front: a no-op reorder applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the storyboard permutation ships as a `shots.reordered` id sequence with `activeShotId`
/// left NULL — the delta itself is the proof the cursor does not follow the new index 0.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-shots/moves-shot-close-to-front: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["shots"]["reordered"][0], "shot-close", "reorder-shots/moves-shot-close-to-front: the promoted shot heads the sequence");
    assert!(committed["activeShotId"].is_null(), "reorder-shots/moves-shot-close-to-front: the active-shot cursor slot is untouched by a reorder");
    assert!(committed["shots"]["patched"].as_array().expect("patched is an array").is_empty(), "reorder-shots/moves-shot-close-to-front: reordering is pure permutation");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed reorder-shots sequence round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-shots/moves-shot-close-to-front: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the id sequence alone is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-shots/moves-shot-close-to-front: committed diff did not carry before to after");
}
