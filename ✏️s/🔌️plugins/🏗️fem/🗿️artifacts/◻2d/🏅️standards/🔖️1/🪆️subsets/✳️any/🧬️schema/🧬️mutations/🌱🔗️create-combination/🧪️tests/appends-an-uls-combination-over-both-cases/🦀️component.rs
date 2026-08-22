//! 🧪️ `create-combination` fixture — `appends-an-uls-combination-over-both-cases`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Both terms must resolve against existing cases before the combination is coined; the factors are ordered as authored.

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

/// ▶️ `create-combination` appends the ULS combination and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("create-combination applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-combination/appends-an-uls-combination-over-both-cases: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.combinations.len(), 1, "create-combination/appends-an-uls-combination-over-both-cases: exactly one combination may be coined");
    assert_eq!(snapshot.combinations[0].terms.len(), 2, "create-combination/appends-an-uls-combination-over-both-cases: both weighted terms must survive");
    assert_eq!(snapshot.combinations[0].terms[0].factor, 1.25, "create-combination/appends-an-uls-combination-over-both-cases: the dead-load factor must survive the round trip exactly");
}

/// ↩️ The inverse is a `delete-combination` of `uls`, restoring the combination-free document.
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
    assert_eq!(snapshot, base, "create-combination/appends-an-uls-combination-over-both-cases: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-combination/appends-an-uls-combination-over-both-cases: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-combination/appends-an-uls-combination-over-both-cases: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-combination/appends-an-uls-combination-over-both-cases: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-combination/appends-an-uls-combination-over-both-cases: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-combination/appends-an-uls-combination-over-both-cases: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-combination/appends-an-uls-combination-over-both-cases: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `combinations.added` entry holding both weighted terms.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().combinations.is_some(), "create-combination/appends-an-uls-combination-over-both-cases: the coined combination must surface in the combinations delta");
    assert!(outcome.diff().load_cases.is_none(), "create-combination/appends-an-uls-combination-over-both-cases: the referenced cases are read-only");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-combination/appends-an-uls-combination-over-both-cases: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-combination/appends-an-uls-combination-over-both-cases: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `combinations.added` entry on `before` must reproduce the combination verbatim.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-combination/appends-an-uls-combination-over-both-cases: committed diff did not carry before to after");
}
