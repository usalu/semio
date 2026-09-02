//! 🧪️ `add-load` fixture — `appends-a-member-udl-to-the-dead-case`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Loads have no collection of their own: adding one re-emits the *whole* owning case as a single patch entry.

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

/// ▶️ `add-load` attaches the member UDL `l2` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("add-load applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "add-load/appends-a-member-udl-to-the-dead-case: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.load_cases.len(), 1, "add-load/appends-a-member-udl-to-the-dead-case: adding a load must not coin a new case");
    assert_eq!(snapshot.load_cases[0].loads.len(), 2, "add-load/appends-a-member-udl-to-the-dead-case: the UDL must join the existing nodal load");
    assert_eq!(crate::artifacts::fem2d::load_id(&snapshot.load_cases[0].loads[1]), "l2", "add-load/appends-a-member-udl-to-the-dead-case: the new load must be appended at the tail of the case");
}

/// ↩️ The inverse is a `remove-load` of `l2` from `dead`, restoring the single-load case.
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
    assert_eq!(snapshot, base, "add-load/appends-a-member-udl-to-the-dead-case: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-load/appends-a-member-udl-to-the-dead-case: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-load/appends-a-member-udl-to-the-dead-case: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "add-load/appends-a-member-udl-to-the-dead-case: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "add-load/appends-a-member-udl-to-the-dead-case: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "add-load/appends-a-member-udl-to-the-dead-case: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("add-load/appends-a-member-udl-to-the-dead-case: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be one `loadCases.patched` entry whose item carries both loads — never a nested load delta.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().load_cases.as_ref().expect("loadCases delta").patched.len(), 1, "add-load/appends-a-member-udl-to-the-dead-case: the owning case must be patched exactly once");
    assert!(outcome.diff().load_cases.as_ref().expect("loadCases delta").added.is_empty(), "add-load/appends-a-member-udl-to-the-dead-case: attaching a load must never add a case");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-load/appends-a-member-udl-to-the-dead-case: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-load/appends-a-member-udl-to-the-dead-case: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `loadCases.patched` entry on `before` must yield the two-load case.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-load/appends-a-member-udl-to-the-dead-case: committed diff did not carry before to after");
}
