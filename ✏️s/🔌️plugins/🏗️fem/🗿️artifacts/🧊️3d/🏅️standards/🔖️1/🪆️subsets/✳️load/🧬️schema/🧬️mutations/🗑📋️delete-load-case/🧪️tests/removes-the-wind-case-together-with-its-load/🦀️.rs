//! 🧪️ `delete-load-case` fixture — `removes-the-wind-case-together-with-its-load`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Loads are members of their case, so removing the case removes them with it — no separate load delta appears.

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

/// ▶️ `delete-load-case` drops the wind case and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("delete-load-case applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-load-case/removes-the-wind-case-together-with-its-load: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.load_cases.len(), 1, "delete-load-case/removes-the-wind-case-together-with-its-load: only the dead case may remain");
    assert!(snapshot.load_cases[0].self_weight, "delete-load-case/removes-the-wind-case-together-with-its-load: the surviving dead case keeps its self-weight flag");
    assert_eq!(snapshot.nodes, before().nodes, "delete-load-case/removes-the-wind-case-together-with-its-load: the node the removed load pushed on must stay");
}

/// ↩️ The inverse is a `create-load-case` rebuilt from `base`, restoring the case *and* its nodal load.
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
    assert_eq!(snapshot, base, "delete-load-case/removes-the-wind-case-together-with-its-load: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-load-case/removes-the-wind-case-together-with-its-load: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-load-case/removes-the-wind-case-together-with-its-load: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-load-case/removes-the-wind-case-together-with-its-load: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-load-case/removes-the-wind-case-together-with-its-load: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-load-case/removes-the-wind-case-together-with-its-load: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-load-case/removes-the-wind-case-together-with-its-load: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `loadCases.removed` id; the member loads are implied, never itemised.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().load_cases.as_ref().expect("loadCases delta").removed, vec!["wind".to_string()], "delete-load-case/removes-the-wind-case-together-with-its-load: exactly the wind case may be removed");
    assert!(outcome.diff().load_cases.as_ref().expect("loadCases delta").patched.is_empty(), "delete-load-case/removes-the-wind-case-together-with-its-load: member loads are never itemised as patches");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-load-case/removes-the-wind-case-together-with-its-load: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-load-case/removes-the-wind-case-together-with-its-load: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `loadCases.removed` id on `before` must leave only the dead case.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-load-case/removes-the-wind-case-together-with-its-load: committed diff did not carry before to after");
}
