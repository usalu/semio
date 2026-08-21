//! 🧪️ `replace-element` fixture — `converts-beam-e1-into-a-bar-in-place`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! A whole-value swap that changes the element's *variant* (bending `Beam` → axial `Bar`) while keeping its id and slot.

use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::mutations::Fem2dMutation;
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

/// ▶️ `replace-element` swaps `e1`'s variant in place and carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("replace-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-element/converts-beam-e1-into-a-bar-in-place: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 1, "replace-element/converts-beam-e1-into-a-bar-in-place: a replacement must not change the element count");
    assert!(matches!(snapshot.elements[0], crate::artifacts::fem2d::FemElement::Bar { .. }), "replace-element/converts-beam-e1-into-a-bar-in-place: the beam must have become a bar");
    assert_eq!(crate::artifacts::fem2d::element_id(&snapshot.elements[0]), "e1", "replace-element/converts-beam-e1-into-a-bar-in-place: the identity must survive the variant change");
}

/// ↩️ The inverse is a `replace-element` carrying the beam recovered from `base`, restoring the bending member.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-element/converts-beam-e1-into-a-bar-in-place: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-element/converts-beam-e1-into-a-bar-in-place: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-element/converts-beam-e1-into-a-bar-in-place: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-element/converts-beam-e1-into-a-bar-in-place: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-element/converts-beam-e1-into-a-bar-in-place: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-element/converts-beam-e1-into-a-bar-in-place: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-element/converts-beam-e1-into-a-bar-in-place: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.patched` entry — a replacement never removes-then-adds.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().elements.as_ref().expect("elements delta").patched.len(), 1, "replace-element/converts-beam-e1-into-a-bar-in-place: exactly one element may be patched");
    assert!(outcome.diff().elements.as_ref().expect("elements delta").removed.is_empty(), "replace-element/converts-beam-e1-into-a-bar-in-place: a replacement must never be encoded as a remove-then-add");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-element/converts-beam-e1-into-a-bar-in-place: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-element/converts-beam-e1-into-a-bar-in-place: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.patched` entry on `before` must leave the bar in `e1`'s original slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-element/converts-beam-e1-into-a-bar-in-place: committed diff did not carry before to after");
}
