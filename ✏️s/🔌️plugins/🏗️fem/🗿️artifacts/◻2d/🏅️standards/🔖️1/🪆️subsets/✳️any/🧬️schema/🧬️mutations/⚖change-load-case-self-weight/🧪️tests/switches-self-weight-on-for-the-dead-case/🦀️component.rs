//! 🧪️ `change-load-case-self-weight` fixture — `switches-self-weight-on-for-the-dead-case`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! One boolean flips, yet the diff still carries the whole case — the load list must come back through the patch unchanged.

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Fem2dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-load-case-self-weight` turns the flag on and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("change-load-case-self-weight applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: applied state differs from committed after-snapshot");
    assert!(snapshot.load_cases[0].self_weight, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: the flag must be on afterwards");
    assert_eq!(snapshot.load_cases[0].loads, before().load_cases[0].loads, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: the load list must survive the whole-case patch unchanged");
    assert_eq!(snapshot.load_cases[0].name, "Dead", "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: the case name must not be rewritten");
}

/// ↩️ The inverse is the same mutation carrying the prior flag read back out of `base`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be one `loadCases.patched` entry differing from `before` in the `selfWeight` flag alone.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().load_cases.as_ref().expect("loadCases delta").patched.len(), 1, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: exactly the named case may be patched");
    assert!(outcome.diff().analysis.is_none(), "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: self-weight is a load-case flag, not an analysis setting");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `loadCases.patched` entry on `before` must flip the flag and nothing else.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-load-case-self-weight/switches-self-weight-on-for-the-dead-case: committed diff did not carry before to after");
}
