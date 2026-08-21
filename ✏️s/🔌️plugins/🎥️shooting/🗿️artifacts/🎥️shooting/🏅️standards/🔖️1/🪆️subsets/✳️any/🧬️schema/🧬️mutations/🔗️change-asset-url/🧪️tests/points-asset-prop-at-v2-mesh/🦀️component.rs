//! 🧪️ `change-asset-url` fixture — `points-asset-prop-at-v2-mesh`.
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
    step.diff(base).into_parts().0.apply(base).expect("change-asset-url diff applies")
}

/// ▶️ `change-asset-url` repoints the mesh of "asset-prop" only. The asset's `format` is a separate
/// field the diff never derives from the new url, so it stays `"glb"` even here.
#[semio_framework_async_macros::async_test]
async fn repoints_only_the_url_field() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "change-asset-url/points-asset-prop-at-v2-mesh: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.assets[1].url, "/mesh/prop-v2.glb", "change-asset-url/points-asset-prop-at-v2-mesh: the new url must land on \"asset-prop\"");
    assert_eq!(snapshot.assets[1].format, before().assets[1].format, "change-asset-url/points-asset-prop-at-v2-mesh: `format` is never re-derived from the url");
    assert_eq!(snapshot.assets[1].name, before().assets[1].name, "change-asset-url/points-asset-prop-at-v2-mesh: the display name is outside this patch");
    assert_eq!(snapshot.assets[0], before().assets[0], "change-asset-url/points-asset-prop-at-v2-mesh: the other asset is untouched");
}

/// ↩️ The inverse is a `change-asset-url` back to the BASE url.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_url() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "change-asset-url/points-asset-prop-at-v2-mesh: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-asset-url/points-asset-prop-at-v2-mesh: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-asset-url/points-asset-prop-at-v2-mesh: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the equality guard: repointing at the url the
/// asset already carries is `mutation.no-op` at Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_repointing_at_the_same_url_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-asset-url/points-asset-prop-at-v2-mesh: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "change-asset-url/points-asset-prop-at-v2-mesh: a real repoint must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "change-asset-url/points-asset-prop-at-v2-mesh: repointing at the current url is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "change-asset-url/points-asset-prop-at-v2-mesh: the equality guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "change-asset-url/points-asset-prop-at-v2-mesh: a no-op url change applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves `change-asset-url` fills only the `url` patch slot — in particular `format` is not a
/// patch slot this mutation ever writes, so no url-derived format sneaks into the delta.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-asset-url/points-asset-prop-at-v2-mesh: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["assets"]["patched"][0]["patch"]["url"], "/mesh/prop-v2.glb", "change-asset-url/points-asset-prop-at-v2-mesh: `url` is the one filled patch slot");
    assert!(committed["assets"]["patched"][0]["patch"]["name"].is_null(), "change-asset-url/points-asset-prop-at-v2-mesh: the display name slot stays null");
    assert_eq!(committed["assets"]["patched"][0]["id"], "asset-prop", "change-asset-url/points-asset-prop-at-v2-mesh: exactly one asset is addressed");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed change-asset-url patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-asset-url/points-asset-prop-at-v2-mesh: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the single url patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-asset-url/points-asset-prop-at-v2-mesh: committed diff did not carry before to after");
}
