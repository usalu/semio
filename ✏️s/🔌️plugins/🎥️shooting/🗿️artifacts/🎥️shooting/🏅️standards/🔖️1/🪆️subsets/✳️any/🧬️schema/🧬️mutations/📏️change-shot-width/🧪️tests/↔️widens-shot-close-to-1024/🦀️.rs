//! 🧪️ `change-shot-width` fixture — `↔️widens-shot-close-to-1024`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-shot-width diff applies")
}

/// ▶️ `change-shot-width` patches `width` alone — there is no aspect-ratio coupling, so the shot's
/// `height` stays at its old value and the render becomes non-square.
#[semio_framework_async_macros::async_test]
async fn widens_without_dragging_the_height_along() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-shot-width/widens-shot-close-to-1024: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.shots[1].width, 1024, "change-shot-width/widens-shot-close-to-1024: the new width must land on \"shot-close\"");
    assert_eq!(snapshot.shots[1].height, before().shots[1].height, "change-shot-width/widens-shot-close-to-1024: height is NOT kept in aspect with width");
    assert_eq!(snapshot.shots[1].shape, before().shots[1].shape, "change-shot-width/widens-shot-close-to-1024: the mask shape is outside this patch");
    assert_eq!(snapshot.shots[0], before().shots[0], "change-shot-width/widens-shot-close-to-1024: the other shot is untouched");
}

/// ↩️ The inverse is a `change-shot-width` back to the BASE width.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_width() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-shot-width/widens-shot-close-to-1024: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-shot-width/widens-shot-close-to-1024: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-shot-width/widens-shot-close-to-1024: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — this leaf orders its guards target-missing, then the
/// positivity invariant (a zero width is Fatal), then the equality no-op.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_zero_width_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-shot-width/widens-shot-close-to-1024: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-shot-width/widens-shot-close-to-1024: a real resize must raise no diagnostic");

    let collapsed: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeShotWidth","id":"shot-close","new_width":0}"#).expect("probe mutation decodes");
    let rejected = collapsed.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Fatal), "change-shot-width/widens-shot-close-to-1024: a zero width must be Fatal");
    assert_eq!(rejected.messages()[0].code.0, "mutation.invariant", "change-shot-width/widens-shot-close-to-1024: the positivity guard's frozen code");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-shot-width/widens-shot-close-to-1024: re-applying the same width is a no-op, not an invariant breach");
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "change-shot-width/widens-shot-close-to-1024: the no-op stays at Warning so it still applies");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the aspect independence at DELTA level: `width` is filled and `height` is explicitly
/// null in the same patch, so no proportional resize can hide behind a matching end state.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-shot-width/widens-shot-close-to-1024: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["shots"]["patched"][0]["patch"]["width"], 1024, "change-shot-width/widens-shot-close-to-1024: `width` is the one filled patch slot");
    assert!(committed["shots"]["patched"][0]["patch"]["height"].is_null(), "change-shot-width/widens-shot-close-to-1024: `height` is null IN THE DELTA — the proof there is no aspect coupling");
    assert!(committed["shots"]["patched"][0]["patch"]["shape"].is_null(), "change-shot-width/widens-shot-close-to-1024: the mask shape slot stays null");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed width patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-shot-width/widens-shot-close-to-1024: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single width patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-shot-width/widens-shot-close-to-1024: committed diff did not carry before to after");
}
