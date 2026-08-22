//! 🧪️ `create-element` fixture — `appends-a-diagonal-bracing-bar`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! An axial `Bar` brace joins the 6-DOF `Frame` column — the bar variant carries no `roll`, and must not gain one.

use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Fem3dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `create-element` appends the bracing bar `b1` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("create-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-element/appends-a-diagonal-bracing-bar: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 2, "create-element/appends-a-diagonal-bracing-bar: the brace must be appended beside the frame");
    assert!(matches!(snapshot.elements[1], crate::artifacts::fem3d::FemElement::Bar { .. }), "create-element/appends-a-diagonal-bracing-bar: the brace must stay a Bar, never widen into a Frame");
    assert!(matches!(snapshot.elements[0], crate::artifacts::fem3d::FemElement::Frame { .. }), "create-element/appends-a-diagonal-bracing-bar: the pre-existing column must stay a Frame");
}

/// ↩️ The inverse is a `delete-element` of `b1`, leaving the lone frame behind.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-element/appends-a-diagonal-bracing-bar: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-element/appends-a-diagonal-bracing-bar: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-element/appends-a-diagonal-bracing-bar: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-element/appends-a-diagonal-bracing-bar: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-element/appends-a-diagonal-bracing-bar: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-element/appends-a-diagonal-bracing-bar: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-element/appends-a-diagonal-bracing-bar: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.added` entry tagged `bar`.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().elements.is_some(), "create-element/appends-a-diagonal-bracing-bar: the created brace must surface in the elements delta");
    assert!(outcome.diff().nodes.is_none() && outcome.diff().materials.is_none() && outcome.diff().sections.is_none(), "create-element/appends-a-diagonal-bracing-bar: the referenced node/material/section rows are read-only for this mutation");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-element/appends-a-diagonal-bracing-bar: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-element/appends-a-diagonal-bracing-bar: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.added` entry on `before` must reproduce frame-then-brace.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-element/appends-a-diagonal-bracing-bar: committed diff did not carry before to after");
}
