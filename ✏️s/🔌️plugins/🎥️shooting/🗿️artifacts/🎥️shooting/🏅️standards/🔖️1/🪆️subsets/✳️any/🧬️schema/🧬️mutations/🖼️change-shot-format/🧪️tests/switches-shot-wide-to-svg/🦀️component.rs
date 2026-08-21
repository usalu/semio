//! 🧪️ `change-shot-format` fixture — `switches-shot-wide-to-svg`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-shot-format diff applies")
}

/// ▶️ `change-shot-format` swaps the raster/vector target of one shot. The format string is stored
/// verbatim and is NOT validated against the exporter's vocabulary by this diff.
#[semio_framework_async_macros::async_test]
async fn switches_only_the_render_format() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-shot-format/switches-shot-wide-to-svg: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.shots[0].format, "svg", "change-shot-format/switches-shot-wide-to-svg: the new format must land on \"shot-wide\"");
    assert_eq!((snapshot.shots[0].width, snapshot.shots[0].height), (before().shots[0].width, before().shots[0].height), "change-shot-format/switches-shot-wide-to-svg: switching to a vector format does not resize the shot");
    assert_eq!(snapshot.shots[0].camera_id, before().shots[0].camera_id, "change-shot-format/switches-shot-wide-to-svg: the saved-camera binding is outside this patch");
    assert_eq!(snapshot.shots[1], before().shots[1], "change-shot-format/switches-shot-wide-to-svg: the other shot keeps its own format");
}

/// ↩️ The inverse is a `change-shot-format` back to the BASE format.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_format() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-shot-format/switches-shot-wide-to-svg: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-shot-format/switches-shot-wide-to-svg: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-shot-format/switches-shot-wide-to-svg: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — this leaf has no vocabulary invariant at all: its
/// only guards are target-missing and the equality `mutation.no-op`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_an_unknown_shot_is_target_missing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-shot-format/switches-shot-wide-to-svg: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-shot-format/switches-shot-wide-to-svg: a real format switch must raise no diagnostic");

    let ghost: ShootingMutation = serde_json::from_str(r#"{"mutation":"changeShotFormat","id":"shot-ghost","new_format":"svg"}"#).expect("probe mutation decodes");
    let rejected = ghost.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Error), "change-shot-format/switches-shot-wide-to-svg: an unknown shot is an Error");
    assert_eq!(rejected.messages()[0].code.0, "mutation.target-missing", "change-shot-format/switches-shot-wide-to-svg: the absence guard's frozen code");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-shot-format/switches-shot-wide-to-svg: re-applying the same format is a no-op");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves `format` and `shape` are independent patch slots: switching to a vector format fills
/// `format` and leaves `shape` null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-shot-format/switches-shot-wide-to-svg: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["shots"]["patched"][0]["patch"]["format"], "svg", "change-shot-format/switches-shot-wide-to-svg: `format` is the one filled patch slot");
    assert!(committed["shots"]["patched"][0]["patch"]["shape"].is_null(), "change-shot-format/switches-shot-wide-to-svg: the mask shape is a separate slot and stays null");
    assert_eq!(committed["shots"]["patched"][0]["id"], "shot-wide", "change-shot-format/switches-shot-wide-to-svg: exactly one shot is addressed");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed format patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-shot-format/switches-shot-wide-to-svg: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single format patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-shot-format/switches-shot-wide-to-svg: committed diff did not carry before to after");
}
