//! 🧪️ `change-shot-height` fixture — `heightens-shot-close-to-768`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-shot-height diff applies")
}

/// ▶️ `change-shot-height` patches `height` alone — the sibling of `change-shot-width`, and just as
/// independent: the shot's `width` is left where it was.
#[semio_framework_async_macros::async_test]
async fn heightens_without_dragging_the_width_along() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-shot-height/heightens-shot-close-to-768: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.shots[1].height, 768, "change-shot-height/heightens-shot-close-to-768: the new height must land on \"shot-close\"");
    assert_eq!(snapshot.shots[1].width, before().shots[1].width, "change-shot-height/heightens-shot-close-to-768: width is NOT kept in aspect with height");
    assert_eq!(snapshot.shots[1].format, before().shots[1].format, "change-shot-height/heightens-shot-close-to-768: the render format is outside this patch");
    assert_eq!(snapshot.shots[0], before().shots[0], "change-shot-height/heightens-shot-close-to-768: the other shot is untouched");
}

/// ↩️ The inverse is a `change-shot-height` back to the BASE height.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_height() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-shot-height/heightens-shot-close-to-768: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-shot-height/heightens-shot-close-to-768: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-shot-height/heightens-shot-close-to-768: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and this leaf's own positivity invariant: a zero
/// height is `mutation.invariant` at Fatal, while an unchanged height is only a Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_zero_height_is_fatal() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-shot-height/heightens-shot-close-to-768: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-shot-height/heightens-shot-close-to-768: a real resize must raise no diagnostic");

    let collapsed: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeShotHeight","id":"shot-close","new_height":0}"#).expect("probe mutation decodes");
    let rejected = collapsed.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Fatal), "change-shot-height/heightens-shot-close-to-768: a zero height must be Fatal");
    assert_eq!(rejected.messages()[0].code.0, "mutation.invariant", "change-shot-height/heightens-shot-close-to-768: the positivity guard's frozen code");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-shot-height/heightens-shot-close-to-768: re-applying the same height is a no-op");
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "change-shot-height/heightens-shot-close-to-768: the no-op stays at Warning so it still applies");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the mirror-image sparsity of its width sibling: `height` filled, `width` explicitly
/// null in the same patch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-shot-height/heightens-shot-close-to-768: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["shots"]["patched"][0]["patch"]["height"], 768, "change-shot-height/heightens-shot-close-to-768: `height` is the one filled patch slot");
    assert!(committed["shots"]["patched"][0]["patch"]["width"].is_null(), "change-shot-height/heightens-shot-close-to-768: `width` is null IN THE DELTA — the proof there is no aspect coupling");
    assert!(committed["shots"]["patched"][0]["patch"]["format"].is_null(), "change-shot-height/heightens-shot-close-to-768: the render-format slot stays null");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed height patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-shot-height/heightens-shot-close-to-768: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single height patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-shot-height/heightens-shot-close-to-768: committed diff did not carry before to after");
}
