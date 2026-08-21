//! 🧪️ `rename-asset` fixture — `renames-asset-hero-to-lead`.
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
    step.diff(base).into_parts().0.apply(base).expect("rename-asset diff applies")
}

/// ▶️ `rename-asset` patches exactly one field — `name` — of the addressed asset. Its `url`,
/// `origin`, `orientation` and `scale` are NOT part of the patch the diff builds.
#[semio_framework_async_macros::async_test]
async fn renames_only_the_name_field() {
    let snapshot = apply(&before(), &mutation());
    assert_eq!(snapshot, expected_after(), "rename-asset/renames-asset-hero-to-lead: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.assets[0].name, "Lead", "rename-asset/renames-asset-hero-to-lead: the new name must land on \"asset-hero\"");
    assert_eq!(snapshot.assets[0].id, before().assets[0].id, "rename-asset/renames-asset-hero-to-lead: a rename never re-keys the asset");
    assert_eq!(snapshot.assets[0].url, before().assets[0].url, "rename-asset/renames-asset-hero-to-lead: the mesh url is outside this patch");
    assert_eq!(snapshot.assets[0].origin, before().assets[0].origin, "rename-asset/renames-asset-hero-to-lead: the transform is outside this patch");
    assert_eq!(snapshot.assets[1], before().assets[1], "rename-asset/renames-asset-hero-to-lead: the other asset is untouched");
}

/// ↩️ The inverse is a `rename-asset` back to the BASE name.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_previous_name() {
    let base = before();
    let forward = mutation();
    let inverse = forward.inverse(&base);
    let mut snapshot = apply(&base, &forward);
    for step in &inverse {
        snapshot = apply(&snapshot, step);
    }
    assert_eq!(snapshot, base, "rename-asset/renames-asset-hero-to-lead: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the payload are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ShootingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-asset/renames-asset-hero-to-lead: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-asset/renames-asset-hero-to-lead: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — and the equality guard: renaming to the name the
/// asset already carries is `mutation.no-op` at Warning, i.e. applied with an empty diff.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_and_renaming_to_the_same_name_is_a_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-asset/renames-asset-hero-to-lead: this fixture declares `applied`");
    assert!(mutation().diff(&before()).messages().is_empty(), "rename-asset/renames-asset-hero-to-lead: a real rename must raise no diagnostic");

    let again = mutation().diff(&expected_after());
    assert_eq!(again.worst_level(), Some(protocol::Severity::Warning), "rename-asset/renames-asset-hero-to-lead: renaming to the current name is a Warning, never a rejection");
    assert_eq!(again.messages()[0].code.0, "mutation.no-op", "rename-asset/renames-asset-hero-to-lead: the equality guard's frozen code");
    let unchanged = again.into_parts().0.apply(&expected_after()).expect("a no-op outcome still applies");
    assert_eq!(unchanged, expected_after(), "rename-asset/renames-asset-hero-to-lead: a no-op rename applies an empty diff");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it proves `rename-asset` writes a SPARSE `ShootingAssetPatch` with only `name` filled — the
/// end-state test alone could not tell this apart from a whole-record replacement.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = mutation().diff(&before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-asset/renames-asset-hero-to-lead: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(committed["assets"]["patched"][0]["patch"]["name"], "Lead", "rename-asset/renames-asset-hero-to-lead: `name` is the one filled patch slot");
    assert!(committed["assets"]["patched"][0]["patch"]["url"].is_null() && committed["assets"]["patched"][0]["patch"]["origin"].is_null(), "rename-asset/renames-asset-hero-to-lead: url and transform slots stay null — this is a patch, not a replacement");
    assert!(committed["assets"]["added"].as_array().expect("added is an array").is_empty(), "rename-asset/renames-asset-hero-to-lead: a rename never re-adds the record");
}

/// 🔣️ The committed diff is itself canonical and decodes to `ShootingDiff` — the committed rename-asset patch round-trips through `ShootingDiff` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-asset/renames-asset-hero-to-lead: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — a one-slot patch is enough to rebuild the after-snapshot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ShootingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-asset/renames-asset-hero-to-lead: committed diff did not carry before to after");
}
