//! 🧪️ `replace-element` fixture — `🔄️rolls-the-column-about-its-own-axis`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Only `roll` changes — the 3D-only local-axis angle that has no counterpart in the fem2d element at all.

use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `replace-element` rolls frame `f1` in place and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("replace-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-element/rolls-the-column-about-its-own-axis: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 1, "replace-element/rolls-the-column-about-its-own-axis: a replacement must not change the element count");
    assert!(matches!(snapshot.elements[0], crate::artifacts::fem3d::FemElement::Frame { roll, .. } if roll == 1.5), "replace-element/rolls-the-column-about-its-own-axis: the new roll angle must survive the round trip exactly");
    assert_eq!(crate::artifacts::fem3d::element_id(&snapshot.elements[0]), "f1", "replace-element/rolls-the-column-about-its-own-axis: the identity must survive the whole-value swap");
}

/// ↩️ The inverse is a `replace-element` carrying the un-rolled frame recovered from `base`.
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
    assert_eq!(snapshot, base, "replace-element/rolls-the-column-about-its-own-axis: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-element/rolls-the-column-about-its-own-axis: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-element/rolls-the-column-about-its-own-axis: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-element/rolls-the-column-about-its-own-axis: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-element/rolls-the-column-about-its-own-axis: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-element/rolls-the-column-about-its-own-axis: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-element/rolls-the-column-about-its-own-axis: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.patched` entry — a replacement never removes-then-adds.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().elements.as_ref().expect("elements delta").patched.len(), 1, "replace-element/rolls-the-column-about-its-own-axis: exactly one element may be patched");
    assert!(outcome.diff().elements.as_ref().expect("elements delta").removed.is_empty(), "replace-element/rolls-the-column-about-its-own-axis: a replacement must never be encoded as a remove-then-add");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-element/rolls-the-column-about-its-own-axis: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-element/rolls-the-column-about-its-own-axis: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.patched` entry on `before` must leave the rolled frame in `f1`'s slot.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-element/rolls-the-column-about-its-own-axis: committed diff did not carry before to after");
}
