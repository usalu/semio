//! 🧪️ `change-sigma-ed-shell-mpa` fixture — `raises-shell-design-stress-to-165-mpa` (EN 1999 aluminium).
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
    serde_json::from_str(BEFORE).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: before snapshot decodes")
}
fn expected_after() -> En1999Snapshot {
    serde_json::from_str(AFTER).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: after snapshot decodes")
}
fn mutation() -> En1999Mutation {
    serde_json::from_str(MUTATION).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: mutation decodes")
}

/// ▶️ `change-sigma-ed-shell-mpa` carries `sigma_ed_shell_mpa` from 120.0 to 165.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: mutation applies to its committed before-snapshot");
    assert_eq!(produced.sigma_ed_shell_mpa, 165.0, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: `sigma_ed_shell_mpa` must read 165.0 after the mutation");
    assert_eq!(produced.annex, base.annex, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: `annex` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `sigma_ed_shell_mpa` (120.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: inverse step applies");
    }
    assert_eq!(snapshot.sigma_ed_shell_mpa, base.sigma_ed_shell_mpa, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: inverse must put `sigma_ed_shell_mpa` back to 120.0");
    assert_eq!(snapshot, base, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1999Snapshot = serde_json::from_str(text).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: snapshot reparses");
        assert_eq!(reencoded, original, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: mutation reparses");
    assert_eq!(reencoded, original, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 120.0→165.0 edit of `sigma_ed_shell_mpa` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: changing `sigma_ed_shell_mpa` away from 120.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `sigmaEdShellMpa` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().sigma_ed_shell_mpa, Some(165.0), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: the diff must carry `sigma_ed_shell_mpa` = 165.0");
    assert!(outcome.diff().annex.is_none(), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: the diff must leave `annex` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed diff decodes");
    assert_eq!(produced, committed, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1999Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed diff decodes");
    assert_eq!(decoded.sigma_ed_shell_mpa, Some(165.0), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: the committed diff must name `sigma_ed_shell_mpa` = 165.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed diff reparses");
    assert_eq!(reencoded, original, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 165.0 `sigma_ed_shell_mpa` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1999Diff = serde_json::from_str(DIFF).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed diff applies to the before-snapshot");
    assert_eq!(produced.sigma_ed_shell_mpa, 165.0, "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: the committed diff must set `sigma_ed_shell_mpa` to 165.0");
    assert_eq!(produced, expected_after(), "change-sigma-ed-shell-mpa/raises-shell-design-stress-to-165-mpa: committed diff did not carry before to after");
}
