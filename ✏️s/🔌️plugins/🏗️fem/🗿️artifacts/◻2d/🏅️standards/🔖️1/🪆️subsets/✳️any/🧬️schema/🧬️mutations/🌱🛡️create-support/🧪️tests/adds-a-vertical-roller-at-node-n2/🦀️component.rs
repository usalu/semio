//! 🧪️ `create-support` fixture — `adds-a-vertical-roller-at-node-n2`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! The pin at `n1` is joined by a single-DOF roller at `n2`, completing a statically determinate beam.

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `create-support` appends the roller `s2` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("create-support applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-support/adds-a-vertical-roller-at-node-n2: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.supports.len(), 2, "create-support/adds-a-vertical-roller-at-node-n2: the roller must be appended behind the pin");
    assert_eq!(snapshot.supports[1].fixed, vec![crate::artifacts::fem2d::FemDof::Ty], "create-support/adds-a-vertical-roller-at-node-n2: a roller restrains Ty and nothing else");
    assert_eq!(snapshot.nodes, before().nodes, "create-support/adds-a-vertical-roller-at-node-n2: restraining n2 must not rewrite the node table");
}

/// ↩️ The inverse is a `delete-support` of `s2`, leaving only the pin.
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
    assert_eq!(snapshot, base, "create-support/adds-a-vertical-roller-at-node-n2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-support/adds-a-vertical-roller-at-node-n2: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-support/adds-a-vertical-roller-at-node-n2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-support/adds-a-vertical-roller-at-node-n2: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-support/adds-a-vertical-roller-at-node-n2: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-support/adds-a-vertical-roller-at-node-n2: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-support/adds-a-vertical-roller-at-node-n2: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `supports.added` entry whose `fixed` list is exactly one DOF.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().supports.is_some(), "create-support/adds-a-vertical-roller-at-node-n2: the new roller must surface in the supports delta");
    assert!(outcome.diff().nodes.is_none(), "create-support/adds-a-vertical-roller-at-node-n2: the node the support validates against is read-only");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-support/adds-a-vertical-roller-at-node-n2: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-support/adds-a-vertical-roller-at-node-n2: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `supports.added` entry on `before` must reproduce pin-then-roller.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-support/adds-a-vertical-roller-at-node-n2: committed diff did not carry before to after");
}
