//! 🧪️ `create-load-case` fixture — `appends-a-live-case-carrying-one-nodal-load`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! A case is coined pre-seeded with a load, so the builder's per-load reference check against `n2` has to pass first.

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `create-load-case` appends the live case and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("create-load-case applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-load-case/appends-a-live-case-carrying-one-nodal-load: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.load_cases.len(), 2, "create-load-case/appends-a-live-case-carrying-one-nodal-load: the live case must be appended behind the dead case");
    assert_eq!(snapshot.load_cases[1].loads.len(), 1, "create-load-case/appends-a-live-case-carrying-one-nodal-load: the seeded load must arrive with the case, not separately");
    assert!(!snapshot.load_cases[1].self_weight, "create-load-case/appends-a-live-case-carrying-one-nodal-load: the live case explicitly excludes self-weight");
}

/// ↩️ The inverse is a `delete-load-case` of `live`, taking the seeded load with it.
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
    assert_eq!(snapshot, base, "create-load-case/appends-a-live-case-carrying-one-nodal-load: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-load-case/appends-a-live-case-carrying-one-nodal-load: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-load-case/appends-a-live-case-carrying-one-nodal-load: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-load-case/appends-a-live-case-carrying-one-nodal-load: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-load-case/appends-a-live-case-carrying-one-nodal-load: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-load-case/appends-a-live-case-carrying-one-nodal-load: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-load-case/appends-a-live-case-carrying-one-nodal-load: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `loadCases.added` entry that already contains its nodal load.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().load_cases.is_some(), "create-load-case/appends-a-live-case-carrying-one-nodal-load: the coined case must surface in the loadCases delta");
    assert!(outcome.diff().nodes.is_none(), "create-load-case/appends-a-live-case-carrying-one-nodal-load: the node the seeded load validates against is read-only");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-load-case/appends-a-live-case-carrying-one-nodal-load: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-load-case/appends-a-live-case-carrying-one-nodal-load: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `loadCases.added` entry on `before` must reproduce dead-then-live.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-load-case/appends-a-live-case-carrying-one-nodal-load: committed diff did not carry before to after");
}
