//! 🧪️ `change-it-mm4` fixture — `raises-torsion-constant-to-10240-mm4` (EN 1999 aluminium).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1999::{En1999Diff, En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1999Snapshot {
    serde_json::from_str(BEFORE).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: mutation decodes")
}

/// ▶️ `change-it-mm4` carries `i_t_mm4` from 6400.0 to 10240.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: mutation applies to its committed before-snapshot");
    assert_eq!(produced.i_t_mm4, 10240.0, "change-it-mm4/raises-torsion-constant-to-10240-mm4: `i_t_mm4` must read 10240.0 after the mutation");
    assert_eq!(produced.l_cr_mm, base.l_cr_mm, "change-it-mm4/raises-torsion-constant-to-10240-mm4: `l_cr_mm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-it-mm4/raises-torsion-constant-to-10240-mm4: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `i_t_mm4` (6400.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: inverse step applies");
    }
    assert_eq!(snapshot.i_t_mm4, base.i_t_mm4, "change-it-mm4/raises-torsion-constant-to-10240-mm4: inverse must put `i_t_mm4` back to 6400.0");
    assert_eq!(snapshot, base, "change-it-mm4/raises-torsion-constant-to-10240-mm4: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: snapshot reparses");
        assert_eq!(reencoded, original, "change-it-mm4/raises-torsion-constant-to-10240-mm4: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: mutation reparses");
    assert_eq!(reencoded, original, "change-it-mm4/raises-torsion-constant-to-10240-mm4: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 6400.0→10240.0 edit of `i_t_mm4` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-it-mm4/raises-torsion-constant-to-10240-mm4: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-it-mm4/raises-torsion-constant-to-10240-mm4: changing `i_t_mm4` away from 6400.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-it-mm4/raises-torsion-constant-to-10240-mm4: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `iTMm4` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().i_t_mm4, Some(10240.0), "change-it-mm4/raises-torsion-constant-to-10240-mm4: the diff must carry `i_t_mm4` = 10240.0");
    assert!(outcome.diff().l_cr_mm.is_none(), "change-it-mm4/raises-torsion-constant-to-10240-mm4: the diff must leave `l_cr_mm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: committed diff decodes");
    assert_eq!(produced, committed, "change-it-mm4/raises-torsion-constant-to-10240-mm4: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: committed diff decodes");
    assert_eq!(decoded.i_t_mm4, Some(10240.0), "change-it-mm4/raises-torsion-constant-to-10240-mm4: the committed diff must name `i_t_mm4` = 10240.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: committed diff reparses");
    assert_eq!(reencoded, original, "change-it-mm4/raises-torsion-constant-to-10240-mm4: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 10240.0 `i_t_mm4` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-it-mm4/raises-torsion-constant-to-10240-mm4: committed diff applies to the before-snapshot");
    assert_eq!(produced.i_t_mm4, 10240.0, "change-it-mm4/raises-torsion-constant-to-10240-mm4: the committed diff must set `i_t_mm4` to 10240.0");
    assert_eq!(produced, expected_after(), "change-it-mm4/raises-torsion-constant-to-10240-mm4: committed diff did not carry before to after");
}
