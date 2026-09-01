//! 🧪️ `create-combination` fixture — `appends-a-serviceability-combination-keyed-by-case-id`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! fem3d keys its combination terms by a map, not a term list — the factors must serialise in the map's own key order.

use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Fem3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `create-combination` appends the SLS combination and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("create-combination applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-combination/appends-a-serviceability-combination-keyed-by-case-id: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.combinations.len(), 1, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: exactly one combination may be coined");
    assert_eq!(snapshot.combinations[0].terms.len(), 2, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: both keyed factors must survive");
    assert_eq!(snapshot.combinations[0].terms.get("wind").copied(), Some(0.5), "create-combination/appends-a-serviceability-combination-keyed-by-case-id: the wind factor must be reachable by its case id and exact");
}

/// ↩️ The inverse is a `delete-combination` of `sls`, restoring the combination-free document.
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
    assert_eq!(snapshot, base, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-combination/appends-a-serviceability-combination-keyed-by-case-id: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-combination/appends-a-serviceability-combination-keyed-by-case-id: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `combinations.added` entry holding both keyed factors.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().combinations.is_some(), "create-combination/appends-a-serviceability-combination-keyed-by-case-id: the coined combination must surface in the combinations delta");
    assert!(outcome.diff().load_cases.is_none(), "create-combination/appends-a-serviceability-combination-keyed-by-case-id: the referenced cases are read-only");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-combination/appends-a-serviceability-combination-keyed-by-case-id: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `combinations.added` entry on `before` must reproduce the combination verbatim.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-combination/appends-a-serviceability-combination-keyed-by-case-id: committed diff did not carry before to after");
}
