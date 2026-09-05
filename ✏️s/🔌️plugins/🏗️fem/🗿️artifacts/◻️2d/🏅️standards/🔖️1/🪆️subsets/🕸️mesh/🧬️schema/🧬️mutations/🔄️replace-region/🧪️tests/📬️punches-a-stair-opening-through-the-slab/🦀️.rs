//! 🧪️ `replace-region` fixture — `📬️punches-a-stair-opening-through-the-slab`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! An inner loop appears in `🐼️holes` and the mesh size halves — nested geometry must survive the whole-value swap.

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

/// ▶️ `replace-region` restates the slab with a hole and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("replace-region applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-region/punches-a-stair-opening-through-the-slab: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.regions.len(), 1, "replace-region/punches-a-stair-opening-through-the-slab: a replacement must not change the region count");
    assert_eq!(snapshot.regions[0].holes.len(), 1, "replace-region/punches-a-stair-opening-through-the-slab: the stair opening must have appeared as one hole loop");
    assert_eq!(snapshot.regions[0].mesh_size, 0.25, "replace-region/punches-a-stair-opening-through-the-slab: the refined mesh size must survive the round trip exactly");
}

/// ↩️ The inverse is a `replace-region` carrying the solid slab recovered from `base`.
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
    assert_eq!(snapshot, base, "replace-region/punches-a-stair-opening-through-the-slab: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-region/punches-a-stair-opening-through-the-slab: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-region/punches-a-stair-opening-through-the-slab: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-region/punches-a-stair-opening-through-the-slab: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-region/punches-a-stair-opening-through-the-slab: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-region/punches-a-stair-opening-through-the-slab: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-region/punches-a-stair-opening-through-the-slab: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `regions.patched` entry keyed by `slab`, hole loop included.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().regions.as_ref().expect("regions delta").patched.len(), 1, "replace-region/punches-a-stair-opening-through-the-slab: exactly one region may be patched");
    assert!(outcome.diff().regions.as_ref().expect("regions delta").added.is_empty(), "replace-region/punches-a-stair-opening-through-the-slab: a replacement is never an addition");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-region/punches-a-stair-opening-through-the-slab: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-region/punches-a-stair-opening-through-the-slab: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `regions.patched` entry on `before` must reproduce the perforated slab.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-region/punches-a-stair-opening-through-the-slab: committed diff did not carry before to after");
}
