//! 🧪️ `change-construction-activity` fixture — `switches-construction-activity-to-concreting` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-construction-activity/switches-construction-activity-to-concreting: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-construction-activity/switches-construction-activity-to-concreting: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-construction-activity/switches-construction-activity-to-concreting: mutation decodes")
}

/// ▶️ `change-construction-activity` carries `construction_activity` from scaffolding to concreting and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-construction-activity/switches-construction-activity-to-concreting: mutation applies to its committed before-snapshot");
    assert_eq!(produced.construction_activity, "concreting", "change-construction-activity/switches-construction-activity-to-concreting: `construction_activity` must read concreting after the mutation");
    assert_eq!(produced.accidental_mass_t, base.accidental_mass_t, "change-construction-activity/switches-construction-activity-to-concreting: `accidental_mass_t` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-construction-activity/switches-construction-activity-to-concreting: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `construction_activity` (scaffolding) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-construction-activity/switches-construction-activity-to-concreting: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-construction-activity/switches-construction-activity-to-concreting: inverse step applies");
    }
    assert_eq!(snapshot.construction_activity, base.construction_activity, "change-construction-activity/switches-construction-activity-to-concreting: inverse must put `construction_activity` back to scaffolding");
    assert_eq!(snapshot, base, "change-construction-activity/switches-construction-activity-to-concreting: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-construction-activity/switches-construction-activity-to-concreting: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-construction-activity/switches-construction-activity-to-concreting: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-construction-activity/switches-construction-activity-to-concreting: snapshot reparses");
        assert_eq!(reencoded, original, "change-construction-activity/switches-construction-activity-to-concreting: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-construction-activity/switches-construction-activity-to-concreting: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-construction-activity/switches-construction-activity-to-concreting: mutation reparses");
    assert_eq!(reencoded, original, "change-construction-activity/switches-construction-activity-to-concreting: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean scaffolding→concreting edit of `construction_activity` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-construction-activity/switches-construction-activity-to-concreting: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-construction-activity/switches-construction-activity-to-concreting: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-construction-activity/switches-construction-activity-to-concreting: changing `construction_activity` away from scaffolding must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-construction-activity/switches-construction-activity-to-concreting: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `constructionActivity` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().construction_activity.as_deref(), Some("concreting"), "change-construction-activity/switches-construction-activity-to-concreting: the diff must carry `construction_activity` = concreting");
    assert!(outcome.diff().accidental_mass_t.is_none(), "change-construction-activity/switches-construction-activity-to-concreting: the diff must leave `accidental_mass_t` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-construction-activity/switches-construction-activity-to-concreting: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-construction-activity/switches-construction-activity-to-concreting: committed diff decodes");
    assert_eq!(produced, committed, "change-construction-activity/switches-construction-activity-to-concreting: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-construction-activity/switches-construction-activity-to-concreting: committed diff decodes");
    assert_eq!(decoded.construction_activity.as_deref(), Some("concreting"), "change-construction-activity/switches-construction-activity-to-concreting: the committed diff must name `construction_activity` = concreting");
    let reencoded = serde_json::to_value(&decoded).expect("change-construction-activity/switches-construction-activity-to-concreting: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-construction-activity/switches-construction-activity-to-concreting: committed diff reparses");
    assert_eq!(reencoded, original, "change-construction-activity/switches-construction-activity-to-concreting: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the concreting `construction_activity` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-construction-activity/switches-construction-activity-to-concreting: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-construction-activity/switches-construction-activity-to-concreting: committed diff applies to the before-snapshot");
    assert_eq!(produced.construction_activity, "concreting", "change-construction-activity/switches-construction-activity-to-concreting: the committed diff must set `construction_activity` to concreting");
    assert_eq!(produced, expected_after(), "change-construction-activity/switches-construction-activity-to-concreting: committed diff did not carry before to after");
}
