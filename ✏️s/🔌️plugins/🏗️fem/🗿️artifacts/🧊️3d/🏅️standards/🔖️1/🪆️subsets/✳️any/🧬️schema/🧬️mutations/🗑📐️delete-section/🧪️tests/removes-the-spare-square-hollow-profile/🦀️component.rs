//! 🧪️ `delete-section` fixture — `removes-the-spare-square-hollow-profile`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! The trailing profile nobody references is dropped; frame `f1` keeps pointing at `hea200`.

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

/// ▶️ `delete-section` drops `shs120` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("delete-section applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-section/removes-the-spare-square-hollow-profile: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.sections.len(), 1, "delete-section/removes-the-spare-square-hollow-profile: only the HEA 200 profile may remain");
    assert_eq!(snapshot.sections[0].id, "hea200", "delete-section/removes-the-spare-square-hollow-profile: the profile f1 references must be the survivor");
    assert_eq!(snapshot.elements, before().elements, "delete-section/removes-the-spare-square-hollow-profile: element section references are never rewritten by a section deletion");
}

/// ↩️ The inverse is a `create-section` rebuilt from `base`, re-appending the hollow profile.
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
    assert_eq!(snapshot, base, "delete-section/removes-the-spare-square-hollow-profile: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-section/removes-the-spare-square-hollow-profile: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-section/removes-the-spare-square-hollow-profile: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-section/removes-the-spare-square-hollow-profile: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-section/removes-the-spare-square-hollow-profile: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-section/removes-the-spare-square-hollow-profile: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-section/removes-the-spare-square-hollow-profile: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `sections.removed` id — element section references are left alone.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().sections.as_ref().expect("sections delta").removed, vec!["shs120".to_string()], "delete-section/removes-the-spare-square-hollow-profile: exactly shs120 may be removed");
    assert!(outcome.diff().elements.is_none(), "delete-section/removes-the-spare-square-hollow-profile: no element delta may be opened");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-section/removes-the-spare-square-hollow-profile: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-section/removes-the-spare-square-hollow-profile: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `sections.removed` id on `before` must leave only the HEA 200 row.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-section/removes-the-spare-square-hollow-profile: committed diff did not carry before to after");
}
