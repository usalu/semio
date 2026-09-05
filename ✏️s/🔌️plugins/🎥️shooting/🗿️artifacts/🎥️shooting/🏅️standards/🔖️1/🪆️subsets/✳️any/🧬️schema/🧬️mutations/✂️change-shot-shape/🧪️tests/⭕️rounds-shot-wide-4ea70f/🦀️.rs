//! 🧪️ `change-shot-shape` fixture — `⭕️rounds-shot-wide-to-ellipse`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-shot-shape diff applies")
}

/// ▶️ `change-shot-shape` swaps the shot's mask outline (the export path lowers `"rectangle"` into
/// four line segments and `"ellipse"` into two arcs). Only the `shape` string moves.
#[semio_framework_async_macros::async_test]
async fn switches_only_the_mask_shape() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-shot-shape/rounds-shot-wide-to-ellipse: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.shots[0].shape, "ellipse", "change-shot-shape/rounds-shot-wide-to-ellipse: the new shape must land on \"shot-wide\"");
    assert_eq!(snapshot.shots[0].format, before().shots[0].format, "change-shot-shape/rounds-shot-wide-to-ellipse: the render format is a separate field from the mask shape");
    assert_eq!(snapshot.shots[0].background, before().shots[0].background, "change-shot-shape/rounds-shot-wide-to-ellipse: the per-shot background is outside this patch");
    assert_eq!(snapshot.shots[1], before().shots[1], "change-shot-shape/rounds-shot-wide-to-ellipse: the other shot keeps its own shape");
}

/// ↩️ The inverse is a `change-shot-shape` back to the BASE shape.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_shape() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-shot-shape/rounds-shot-wide-to-ellipse: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-shot-shape/rounds-shot-wide-to-ellipse: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-shot-shape/rounds-shot-wide-to-ellipse: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the equality guard: the second `ellipse` is
/// `mutation.no-op` at Warning. The shape vocabulary itself is never policed here.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_reshaping_to_the_same_shape_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-shot-shape/rounds-shot-wide-to-ellipse: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-shot-shape/rounds-shot-wide-to-ellipse: a real reshape must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "change-shot-shape/rounds-shot-wide-to-ellipse: reshaping to the current shape is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-shot-shape/rounds-shot-wide-to-ellipse: the equality guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "change-shot-shape/rounds-shot-wide-to-ellipse: a no-op reshape applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the converse of its format sibling: `shape` filled, `format` null — the mask outline
/// moves without touching the encoder.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-shot-shape/rounds-shot-wide-to-ellipse: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["shots"]["patched"][0]["patch"]["shape"], "ellipse", "change-shot-shape/rounds-shot-wide-to-ellipse: `shape` is the one filled patch slot");
    assert!(committed["shots"]["patched"][0]["patch"]["format"].is_null(), "change-shot-shape/rounds-shot-wide-to-ellipse: the render format is a separate slot and stays null");
    assert!(committed["shots"]["patched"][0]["patch"]["label"].is_null(), "change-shot-shape/rounds-shot-wide-to-ellipse: the caption slot stays null");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed shape patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-shot-shape/rounds-shot-wide-to-ellipse: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single shape patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-shot-shape/rounds-shot-wide-to-ellipse: committed diff did not carry before to after");
}
