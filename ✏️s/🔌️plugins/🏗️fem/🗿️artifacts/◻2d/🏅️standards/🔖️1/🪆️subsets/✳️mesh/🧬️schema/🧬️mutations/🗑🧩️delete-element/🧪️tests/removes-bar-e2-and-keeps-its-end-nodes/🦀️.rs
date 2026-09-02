//! 🧪️ `delete-element` fixture — `removes-bar-e2-and-keeps-its-end-nodes`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Deleting the trailing bar must not garbage-collect the nodes, material or section it referenced.

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

/// ▶️ `delete-element` drops bar `e2` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-element/removes-bar-e2-and-keeps-its-end-nodes: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 1, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: only the beam may remain");
    assert_eq!(snapshot.nodes.len(), 3, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: n3 was only reachable through e2 and must still survive");
    assert_eq!(snapshot.sections, before().sections, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: the section catalogue is untouched by an element deletion");
}

/// ↩️ The inverse is a `create-element` rebuilt from `base`, re-appending the bar with its variant intact.
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
    assert_eq!(snapshot, base, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-element/removes-bar-e2-and-keeps-its-end-nodes: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-element/removes-bar-e2-and-keeps-its-end-nodes: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.removed` id — a deleted element frees nothing else.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().elements.as_ref().expect("elements delta").removed, vec!["e2".to_string()], "delete-element/removes-bar-e2-and-keeps-its-end-nodes: exactly e2 may be removed");
    assert!(outcome.diff().nodes.is_none(), "delete-element/removes-bar-e2-and-keeps-its-end-nodes: no node may be swept up with the element");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-element/removes-bar-e2-and-keeps-its-end-nodes: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.removed` id on `before` must leave only beam `e1`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-element/removes-bar-e2-and-keeps-its-end-nodes: committed diff did not carry before to after");
}
