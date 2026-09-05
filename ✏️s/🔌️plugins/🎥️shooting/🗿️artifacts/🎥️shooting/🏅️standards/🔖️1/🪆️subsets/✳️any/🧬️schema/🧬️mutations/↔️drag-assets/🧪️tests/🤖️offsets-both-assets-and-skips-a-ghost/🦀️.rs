//! 🧪️ `drag-assets` fixture — `🤖️offsets-both-assets-and-skips-a-ghost`.
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
    step.diff(base).into_parts().0.apply(base).expect("drag-assets diff applies")
}

/// ▶️ `drag-assets` is the bulk RELATIVE gesture: every addressed asset's `origin` gains
/// `(dx, dy, dz)`, computed per-asset from its own base origin — the two assets here start at
/// different points and therefore land at different points.
#[semio_framework_async_macros::async_test]
async fn offsets_every_addressed_origin_relatively() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "drag-assets/offsets-both-assets-and-skips-a-ghost: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.assets[0].origin, [5.0, 1.0, 3.5], "drag-assets/offsets-both-assets-and-skips-a-ghost: \"asset-hero\" moves from its own [1, 2, 3]");
    assert_eq!(snapshot.assets[1].origin, [4.0, -1.0, 0.5], "drag-assets/offsets-both-assets-and-skips-a-ghost: \"asset-prop\" moves from its own [0, 0, 0]");
    assert_eq!(snapshot.assets[0].orientation, before().assets[0].orientation, "drag-assets/offsets-both-assets-and-skips-a-ghost: a drag never touches orientation");
    assert_eq!(snapshot.assets[0].scale, before().assets[0].scale, "drag-assets/offsets-both-assets-and-skips-a-ghost: a drag never touches scale");
}

/// ↩️ The inverse is the same drag with every component negated — it does not consult the base at
/// all, so it also re-addresses the ghost id and still lands exactly back on the before-snapshot.
#[semio_framework_async_macros::async_test]
async fn inverse_drags_the_offset_back() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "drag-assets/offsets-both-assets-and-skips-a-ghost: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "drag-assets/offsets-both-assets-and-skips-a-ghost: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "drag-assets/offsets-both-assets-and-skips-a-ghost: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` WITH a `mutation.partial` warning: the payload names three assets and only
/// two exist, so the drag proceeds for the two and reports the third — and when NONE of the named
/// assets exist the very same builder escalates to `mutation.target-missing` at Error.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_a_missing_id_only_degrades_to_partial() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "drag-assets/offsets-both-assets-and-skips-a-ghost: this fixture declares `applied`");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("this fixture declares one message");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some("mutation.partial"), "drag-assets/offsets-both-assets-and-skips-a-ghost: the declared message is the partial warning");

    let partial = mutation().diff(&before());
    assert_eq!(partial.worst_level(), Some(protocol::Severity::Warning), "drag-assets/offsets-both-assets-and-skips-a-ghost: a partly-resolvable drag stays at Warning so it still applies");
    assert_eq!(partial.messages()[0].code.0, "mutation.partial", "drag-assets/offsets-both-assets-and-skips-a-ghost: the skip guard's frozen code");
    assert_eq!(partial.messages()[0].target, vec!["asset-ghost".to_string()], "drag-assets/offsets-both-assets-and-skips-a-ghost: only the skipped id is named");

    let nothing_resolves: ShootingMutation = serde_json::from_str(r#"{"mutation":"dragAssets","asset_ids":["asset-ghost"],"dx":4.0,"dy":-1.0,"dz":0.5}"#).expect("probe mutation decodes");
    let rejected = nothing_resolves.diff(&before());
    assert_eq!(rejected.worst_level(), Some(protocol::Severity::Error), "drag-assets/offsets-both-assets-and-skips-a-ghost: a drag that resolves nothing is an Error");
    assert_eq!(rejected.messages()[0].code.0, "mutation.target-missing", "drag-assets/offsets-both-assets-and-skips-a-ghost: the empty-selection guard's frozen code");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves the bulk drag fans out into ONE patch entry per RESOLVED asset — two entries, not
/// three — each carrying an already-absolute origin, and no entry for the skipped ghost id.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "drag-assets/offsets-both-assets-and-skips-a-ghost: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed["assets"]["patched"].as_array().expect("patched is an array").len(), 2, "drag-assets/offsets-both-assets-and-skips-a-ghost: the unresolvable id contributes no patch entry");
    assert_eq!(committed["assets"]["patched"][0]["patch"]["origin"][0], 5.0, "drag-assets/offsets-both-assets-and-skips-a-ghost: the delta stores the RESOLVED absolute origin, not the relative offset");
    assert!(committed["assets"]["patched"][1]["patch"]["orientation"].is_null() && committed["assets"]["patched"][1]["patch"]["scale"].is_null(), "drag-assets/offsets-both-assets-and-skips-a-ghost: a drag fills only the `origin` slot");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed two-entry drag fan-out round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "drag-assets/offsets-both-assets-and-skips-a-ghost: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the two origin patches are enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "drag-assets/offsets-both-assets-and-skips-a-ghost: committed diff did not carry before to after");
}
