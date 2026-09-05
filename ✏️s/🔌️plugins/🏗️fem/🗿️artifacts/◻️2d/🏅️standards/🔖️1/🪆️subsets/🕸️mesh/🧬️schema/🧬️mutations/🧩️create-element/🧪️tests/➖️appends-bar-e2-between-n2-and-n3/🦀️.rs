//! 🧪️ `create-element` fixture — `➖️appends-bar-e2-between-n2-and-n3`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! A second span is added as an axial-only `Bar`, next to the existing `Beam` — the variant tag must survive the round trip.

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

/// ▶️ `create-element` appends bar `e2` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("create-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-element/appends-bar-e2-between-n2-and-n3: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 2, "create-element/appends-bar-e2-between-n2-and-n3: the new bar must be appended beside the existing beam");
    assert!(matches!(snapshot.elements[1], crate::artifacts::fem2d::FemElement::Bar { .. }), "create-element/appends-bar-e2-between-n2-and-n3: the appended element must keep its Bar variant, not decay into a Beam");
    assert_eq!(snapshot.nodes, before().nodes, "create-element/appends-bar-e2-between-n2-and-n3: wiring an element to n2/n3 must not rewrite the node table");
}

/// ↩️ The inverse is a `delete-element` of `e2`, leaving the lone beam `e1` behind.
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
    assert_eq!(snapshot, base, "create-element/appends-bar-e2-between-n2-and-n3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-element/appends-bar-e2-between-n2-and-n3: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-element/appends-bar-e2-between-n2-and-n3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-element/appends-bar-e2-between-n2-and-n3: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-element/appends-bar-e2-between-n2-and-n3: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-element/appends-bar-e2-between-n2-and-n3: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-element/appends-bar-e2-between-n2-and-n3: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.added` entry carrying the `bar` variant tag verbatim.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().elements.is_some(), "create-element/appends-bar-e2-between-n2-and-n3: the created bar must surface in the elements delta");
    assert!(outcome.diff().nodes.is_none() && outcome.diff().materials.is_none() && outcome.diff().sections.is_none(), "create-element/appends-bar-e2-between-n2-and-n3: the referenced node/material/section rows are read-only for this mutation");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-element/appends-bar-e2-between-n2-and-n3: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-element/appends-bar-e2-between-n2-and-n3: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.added` entry on `before` must reproduce the beam-then-bar order.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-element/appends-bar-e2-between-n2-and-n3: committed diff did not carry before to after");
}
