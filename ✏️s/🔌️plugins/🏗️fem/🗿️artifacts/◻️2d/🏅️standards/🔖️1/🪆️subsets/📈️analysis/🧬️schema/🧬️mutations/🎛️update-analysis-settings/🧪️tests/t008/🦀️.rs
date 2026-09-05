//! 🧪️ `update-analysis-settings` fixture — `🔢️doubles-the-modal-count-and-halves-the-deformation-scale`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! The only fem2d mutation whose diff payload is a scalar facet rather than a collection delta — settings move as one indivisible record.

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

/// ▶️ `update-analysis-settings` rewrites the analysis facet and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("update-analysis-settings applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.analysis.modal_count, 6, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: the modal count must have doubled");
    assert_eq!(snapshot.analysis.deformation_scale, 25.0, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: the deformation scale must have halved exactly");
    assert_eq!(snapshot.elements, before().elements, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: an analysis-settings edit must never touch the model itself");
}

/// ↩️ The inverse is the same mutation carrying the prior settings record read back out of `base`.
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
    assert_eq!(snapshot, base, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must set `analysis` alone — every collection delta stays closed.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().analysis.is_some(), "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: the settings record must surface in the analysis field");
    assert!(
        outcome.diff().nodes.is_none() && outcome.diff().elements.is_none() && outcome.diff().load_cases.is_none(),
        "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: no collection delta may be opened by a settings edit"
    );
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `analysis` value on `before` must rewrite the settings and nothing else.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-analysis-settings/doubles-the-modal-count-and-halves-the-deformation-scale: committed diff did not carry before to after");
}
