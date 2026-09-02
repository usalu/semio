//! 🧪️ `change-fire-rating` fixture — `upgrades-fire-rating-to-r90` (EN 1994 composite).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1994Snapshot {
    serde_json::from_str(BEFORE).expect("change-fire-rating/upgrades-fire-rating-to-r90: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-fire-rating/upgrades-fire-rating-to-r90: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-fire-rating/upgrades-fire-rating-to-r90: mutation decodes")
}

/// ▶️ `change-fire-rating` carries `fire_rating` from r60 to r90 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-fire-rating/upgrades-fire-rating-to-r90: mutation applies to its committed before-snapshot");
    assert_eq!(produced.fire_rating, "r90", "change-fire-rating/upgrades-fire-rating-to-r90: `fire_rating` must read r90 after the mutation");
    assert_eq!(produced.deck_type, base.deck_type, "change-fire-rating/upgrades-fire-rating-to-r90: `deck_type` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-fire-rating/upgrades-fire-rating-to-r90: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `fire_rating` (r60) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-fire-rating/upgrades-fire-rating-to-r90: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-fire-rating/upgrades-fire-rating-to-r90: inverse step applies");
    }
    assert_eq!(snapshot.fire_rating, base.fire_rating, "change-fire-rating/upgrades-fire-rating-to-r90: inverse must put `fire_rating` back to r60");
    assert_eq!(snapshot, base, "change-fire-rating/upgrades-fire-rating-to-r90: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-fire-rating/upgrades-fire-rating-to-r90: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-fire-rating/upgrades-fire-rating-to-r90: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-fire-rating/upgrades-fire-rating-to-r90: snapshot reparses");
        assert_eq!(reencoded, original, "change-fire-rating/upgrades-fire-rating-to-r90: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-fire-rating/upgrades-fire-rating-to-r90: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-fire-rating/upgrades-fire-rating-to-r90: mutation reparses");
    assert_eq!(reencoded, original, "change-fire-rating/upgrades-fire-rating-to-r90: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean r60→r90 edit of `fire_rating` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-fire-rating/upgrades-fire-rating-to-r90: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fire-rating/upgrades-fire-rating-to-r90: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-fire-rating/upgrades-fire-rating-to-r90: changing `fire_rating` away from r60 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-fire-rating/upgrades-fire-rating-to-r90: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `fireRating` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().fire_rating.as_deref(), Some("r90"), "change-fire-rating/upgrades-fire-rating-to-r90: the diff must carry `fire_rating` = r90");
    assert!(outcome.diff().deck_type.is_none(), "change-fire-rating/upgrades-fire-rating-to-r90: the diff must leave `deck_type` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-fire-rating/upgrades-fire-rating-to-r90: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-rating/upgrades-fire-rating-to-r90: committed diff decodes");
    assert_eq!(produced, committed, "change-fire-rating/upgrades-fire-rating-to-r90: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-fire-rating/upgrades-fire-rating-to-r90: committed diff decodes");
    assert_eq!(decoded.fire_rating.as_deref(), Some("r90"), "change-fire-rating/upgrades-fire-rating-to-r90: the committed diff must name `fire_rating` = r90");
    let reencoded = serde_json::to_value(&decoded).expect("change-fire-rating/upgrades-fire-rating-to-r90: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-fire-rating/upgrades-fire-rating-to-r90: committed diff reparses");
    assert_eq!(reencoded, original, "change-fire-rating/upgrades-fire-rating-to-r90: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the r90 `fire_rating` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-fire-rating/upgrades-fire-rating-to-r90: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-fire-rating/upgrades-fire-rating-to-r90: committed diff applies to the before-snapshot");
    assert_eq!(produced.fire_rating, "r90", "change-fire-rating/upgrades-fire-rating-to-r90: the committed diff must set `fire_rating` to r90");
    assert_eq!(produced, expected_after(), "change-fire-rating/upgrades-fire-rating-to-r90: committed diff did not carry before to after");
}
