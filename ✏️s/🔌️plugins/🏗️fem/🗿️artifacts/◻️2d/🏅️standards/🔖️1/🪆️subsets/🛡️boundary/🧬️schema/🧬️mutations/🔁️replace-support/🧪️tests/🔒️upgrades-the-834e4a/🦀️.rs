//! 🧪️ `replace-support` fixture — `🔒️upgrades-the-834e4a`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! The DOF list grows from one entry to three under the same support id — the whole record is swapped, not merged.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-support/🔒️upgrades-the-834e4a/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-support/🔒️upgrades-the-834e4a/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-support/🔒️upgrades-the-834e4a/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-support/🔒️upgrades-the-834e4a/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-support/🔒️upgrades-the-834e4a/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `replace-support` restates `s2` as a full fixity and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("replace-support applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-support/upgrades-the-834e4a: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.supports.len(), 2, "replace-support/upgrades-the-834e4a: a replacement must not change the support count");
    assert_eq!(snapshot.supports[1].fixed.len(), 3, "replace-support/upgrades-the-834e4a: the DOF list is swapped wholesale, from one restraint to three");
    assert_eq!(snapshot.supports[0], before().supports[0], "replace-support/upgrades-the-834e4a: the pin ahead of the patched row must be untouched");
}

/// ↩️ The inverse is a `replace-support` carrying the single-DOF roller recovered from `base`.
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
    assert_eq!(snapshot, base, "replace-support/upgrades-the-834e4a: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-support/upgrades-the-834e4a: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-support/upgrades-the-834e4a: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-support/upgrades-the-834e4a: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-support/upgrades-the-834e4a: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-support/upgrades-the-834e4a: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-support/upgrades-the-834e4a: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `supports.patched` entry keyed by `s2`.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().supports.as_ref().expect("supports delta").patched.len(), 1, "replace-support/upgrades-the-834e4a: exactly one support may be patched");
    assert!(outcome.diff().supports.as_ref().expect("supports delta").added.is_empty(), "replace-support/upgrades-the-834e4a: a replacement is never an addition");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-support/upgrades-the-834e4a: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-support/upgrades-the-834e4a: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `supports.patched` entry on `before` must fix `n2` without moving the pin.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-support/upgrades-the-834e4a: committed diff did not carry before to after");
}
