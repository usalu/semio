//! 🧪️ `replace-material` fixture — `🏗️restates-steel-as-s355-in-its-original-slot`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! The *first* catalogue row is swapped whole-value, proving a patch keeps its slot instead of migrating to the tail.

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

/// ▶️ `replace-material` restates `steel` as S355 and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("replace-material applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-material/restates-steel-as-s355-in-its-original-slot: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.materials[0].name, "Steel S355", "replace-material/restates-steel-as-s355-in-its-original-slot: the replacement value must have landed");
    assert_eq!(snapshot.materials[0].e, 205000000000.0, "replace-material/restates-steel-as-s355-in-its-original-slot: the new Young modulus must survive the round trip exactly");
    assert_eq!(snapshot.materials[1], before().materials[1], "replace-material/restates-steel-as-s355-in-its-original-slot: patching the first row must not shuffle the concrete row behind it");
}

/// ↩️ The inverse is a `replace-material` carrying the S235 row recovered from `base`.
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
    assert_eq!(snapshot, base, "replace-material/restates-steel-as-s355-in-its-original-slot: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-material/restates-steel-as-s355-in-its-original-slot: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-material/restates-steel-as-s355-in-its-original-slot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-material/restates-steel-as-s355-in-its-original-slot: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-material/restates-steel-as-s355-in-its-original-slot: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-material/restates-steel-as-s355-in-its-original-slot: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-material/restates-steel-as-s355-in-its-original-slot: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `materials.patched` entry keyed by `steel`.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().materials.as_ref().expect("materials delta").patched.len(), 1, "replace-material/restates-steel-as-s355-in-its-original-slot: exactly one material may be patched");
    assert!(outcome.diff().materials.as_ref().expect("materials delta").added.is_empty(), "replace-material/restates-steel-as-s355-in-its-original-slot: a replacement is never an addition");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-material/restates-steel-as-s355-in-its-original-slot: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-material/restates-steel-as-s355-in-its-original-slot: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `materials.patched` entry on `before` must keep S355 ahead of the concrete row.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-material/restates-steel-as-s355-in-its-original-slot: committed diff did not carry before to after");
}
