//! 🧪️ `rename-shot` fixture — `relabels-shot-close-to-detail`.
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
    step.diff(base).into_parts().0.apply(base).expect("rename-shot diff applies")
}

/// ▶️ `rename-shot` patches the shot's `label` — the human caption — and leaves the `id` that every
/// cursor and camera binding is keyed on alone.
#[semio_framework_async_macros::async_test]
async fn relabels_without_rekeying() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "rename-shot/relabels-shot-close-to-detail: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.shots[1].label, "Detail", "rename-shot/relabels-shot-close-to-detail: the new label must land on \"shot-close\"");
    assert_eq!(snapshot.shots[1].id, "shot-close", "rename-shot/relabels-shot-close-to-detail: a relabel never re-keys the shot");
    assert_eq!((snapshot.shots[1].width, snapshot.shots[1].height), (before().shots[1].width, before().shots[1].height), "rename-shot/relabels-shot-close-to-detail: the pixel dimensions are outside this patch");
    assert_eq!(snapshot.shots[0], before().shots[0], "rename-shot/relabels-shot-close-to-detail: the other shot is untouched");
}

/// ↩️ The inverse is a `rename-shot` back to the BASE label.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_label() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "rename-shot/relabels-shot-close-to-detail: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-shot/relabels-shot-close-to-detail: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-shot/relabels-shot-close-to-detail: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the equality guard: relabelling to the label the
/// shot already carries is `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_relabelling_to_the_same_label_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-shot/relabels-shot-close-to-detail: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "rename-shot/relabels-shot-close-to-detail: a real relabel must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "rename-shot/relabels-shot-close-to-detail: relabelling to the current label is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "rename-shot/relabels-shot-close-to-detail: the equality guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "rename-shot/relabels-shot-close-to-detail: a no-op relabel applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the `ShootingShotPatch` has `label` filled and its four sibling slots null — and note
/// the patch type has no `background`/`cameraId` slot at all, so a relabel structurally cannot
/// disturb either.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-shot/relabels-shot-close-to-detail: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["shots"]["patched"][0]["patch"]["label"], "Detail", "rename-shot/relabels-shot-close-to-detail: `label` is the one filled patch slot");
    assert!(committed["shots"]["patched"][0]["patch"]["width"].is_null() && committed["shots"]["patched"][0]["patch"]["height"].is_null(), "rename-shot/relabels-shot-close-to-detail: the pixel-dimension slots stay null");
    assert!(committed["shots"]["patched"][0]["patch"].get("cameraId").is_none(), "rename-shot/relabels-shot-close-to-detail: `ShootingShotPatch` carries no camera slot, so a relabel cannot rebind a shot");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed rename-shot patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-shot/relabels-shot-close-to-detail: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — a one-slot patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-shot/relabels-shot-close-to-detail: committed diff did not carry before to after");
}
