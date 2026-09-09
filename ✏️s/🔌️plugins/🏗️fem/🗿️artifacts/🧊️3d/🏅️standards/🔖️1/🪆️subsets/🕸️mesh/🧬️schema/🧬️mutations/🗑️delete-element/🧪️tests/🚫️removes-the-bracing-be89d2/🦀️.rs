//! 🧪️ `delete-element` fixture — `🚫️removes-the-bracing-be89d2`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Un-bracing the frame is a one-collection edit: the shared nodes, material and section rows all stay.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::Fem3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/🚫️removes-the-bracing-be89d2/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/🚫️removes-the-bracing-be89d2/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/🚫️removes-the-bracing-be89d2/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/🚫️removes-the-bracing-be89d2/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/🚫️removes-the-bracing-be89d2/🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `delete-element` drops the brace `b1` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("delete-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-element/removes-the-bracing-be89d2: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 1, "delete-element/removes-the-bracing-be89d2: only the frame may remain");
    assert_eq!(crate::element_id(&snapshot.elements[0]), "f1", "delete-element/removes-the-bracing-be89d2: the frame is the survivor");
    assert_eq!(snapshot.nodes.len(), 3, "delete-element/removes-the-bracing-be89d2: no node may be swept up with the brace");
}

/// ↩️ The inverse is a `create-element` rebuilt from `base`, re-appending the bar with its variant intact.
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
    assert_eq!(snapshot, base, "delete-element/removes-the-bracing-be89d2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-element/removes-the-bracing-be89d2: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-element/removes-the-bracing-be89d2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
///
/// 🚦️ Refusal is read off the OUTCOME, never off the `Result`. `vcs::apply_mutation` is
/// policy-agnostic: a refused mutation carries the empty diff, so it still applies cleanly and
/// still returns `Ok` — asserting `is_err()` here would be a branch that can never fire.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let refused = produced.messages().iter().any(|message| message.level >= protocol::Severity::Error);
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("the produced diff applies to its own before-snapshot");
    match status {
        "applied" => assert!(!refused, "delete-element/removes-the-bracing-be89d2: declared applied but the diff builder refused with {:?}", produced.messages()),
        "rejected" => {
            assert!(refused, "delete-element/removes-the-bracing-be89d2: declared rejected but the diff builder raised no Error or Fatal, only {:?}", produced.messages());
            assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "delete-element/removes-the-bracing-be89d2: a refused mutation must carry the empty diff");
            assert_eq!(snapshot, before(), "delete-element/removes-the-bracing-be89d2: a refused mutation must leave the snapshot untouched");
        }
        other => panic!("delete-element/removes-the-bracing-be89d2: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.removed` id — a deleted element frees nothing else.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().elements.as_ref().expect("elements delta").removed, vec!["b1".to_string()], "delete-element/removes-the-bracing-be89d2: exactly b1 may be removed");
    assert!(outcome.diff().nodes.is_none(), "delete-element/removes-the-bracing-be89d2: no node delta may be opened");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-element/removes-the-bracing-be89d2: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-element/removes-the-bracing-be89d2: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.removed` id on `before` must leave only the frame.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-element/removes-the-bracing-be89d2: committed diff did not carry before to after");
}
