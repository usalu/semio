//! 🧪️ `change-accidental-mass-t` fixture — `lightens-impact-vehicle-to-12-5-t` (EN 1991 actions).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1991Snapshot {
    serde_json::from_str(BEFORE).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: before snapshot decodes")
}
fn expected_after() -> En1991Snapshot {
    serde_json::from_str(AFTER).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: after snapshot decodes")
}
fn mutation() -> En1991Mutation {
    serde_json::from_str(MUTATION).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: mutation decodes")
}

/// ▶️ `change-accidental-mass-t` carries `accidental_mass_t` from 30.0 to 12.5 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: mutation applies to its committed before-snapshot");
    assert_eq!(produced.accidental_mass_t, 12.5, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: `accidental_mass_t` must read 12.5 after the mutation");
    assert_eq!(produced.accidental_speed_km_h, base.accidental_speed_km_h, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: `accidental_speed_km_h` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `accidental_mass_t` (30.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: inverse step applies");
    }
    assert_eq!(snapshot.accidental_mass_t, base.accidental_mass_t, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: inverse must put `accidental_mass_t` back to 30.0");
    assert_eq!(snapshot, base, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1991Snapshot = serde_json::from_str(text).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: snapshot reparses");
        assert_eq!(reencoded, original, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: mutation reparses");
    assert_eq!(reencoded, original, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 30.0→12.5 edit of `accidental_mass_t` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: changing `accidental_mass_t` away from 30.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `accidentalMassT` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().accidental_mass_t, Some(12.5), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: the diff must carry `accidental_mass_t` = 12.5");
    assert!(outcome.diff().accidental_speed_km_h.is_none(), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: the diff must leave `accidental_speed_km_h` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed diff decodes");
    assert_eq!(produced, committed, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1991Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed diff decodes");
    assert_eq!(decoded.accidental_mass_t, Some(12.5), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: the committed diff must name `accidental_mass_t` = 12.5");
    let reencoded = serde_json::to_value(&decoded).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed diff reparses");
    assert_eq!(reencoded, original, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 12.5 `accidental_mass_t` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1991Diff = serde_json::from_str(DIFF).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed diff applies to the before-snapshot");
    assert_eq!(produced.accidental_mass_t, 12.5, "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: the committed diff must set `accidental_mass_t` to 12.5");
    assert_eq!(produced, expected_after(), "change-accidental-mass-t/lightens-impact-vehicle-to-12-5-t: committed diff did not carry before to after");
}
