//! 🧪️ `replace-support` fixture — `frees-the-three-rotations-at-the-column-base`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Rx/Ry/Rz are dropped from the DOF list, turning the fixed base into a spherical hinge — the list is swapped, never merged.

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

/// ▶️ `replace-support` frees the base rotations and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("replace-support applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-support/frees-the-three-rotations-at-the-column-base: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.supports.len(), 2, "replace-support/frees-the-three-rotations-at-the-column-base: a replacement must not change the support count");
    assert_eq!(snapshot.supports[0].fixed.len(), 3, "replace-support/frees-the-three-rotations-at-the-column-base: the three rotational restraints must be gone");
    assert!(!snapshot.supports[0].fixed.contains(&crate::artifacts::fem3d::FemDof::Rz), "replace-support/frees-the-three-rotations-at-the-column-base: Rz in particular must no longer be restrained");
}

/// ↩️ The inverse is a `replace-support` carrying the six-DOF clamp recovered from `base`.
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
    assert_eq!(snapshot, base, "replace-support/frees-the-three-rotations-at-the-column-base: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-support/frees-the-three-rotations-at-the-column-base: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-support/frees-the-three-rotations-at-the-column-base: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-support/frees-the-three-rotations-at-the-column-base: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-support/frees-the-three-rotations-at-the-column-base: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-support/frees-the-three-rotations-at-the-column-base: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-support/frees-the-three-rotations-at-the-column-base: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `supports.patched` entry keyed by `s1`.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().supports.as_ref().expect("supports delta").patched.len(), 1, "replace-support/frees-the-three-rotations-at-the-column-base: exactly one support may be patched");
    assert!(outcome.diff().supports.as_ref().expect("supports delta").added.is_empty(), "replace-support/frees-the-three-rotations-at-the-column-base: a replacement is never an addition");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-support/frees-the-three-rotations-at-the-column-base: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-support/frees-the-three-rotations-at-the-column-base: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `supports.patched` entry on `before` must hinge the base without moving the pin.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-support/frees-the-three-rotations-at-the-column-base: committed diff did not carry before to after");
}
