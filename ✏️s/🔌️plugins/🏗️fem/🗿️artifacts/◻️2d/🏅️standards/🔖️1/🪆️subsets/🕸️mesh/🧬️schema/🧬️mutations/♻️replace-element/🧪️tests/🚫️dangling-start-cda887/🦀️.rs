//! 🧪️ `replace-element` fixture — `🚫️dangling-start-cda887`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening).
//! Every value is in SI base units.
//!
//! 🛡️ This vector pins a branch the 26/09/06/FEM-PLUGIN-END-TO-END hardening wave ADDED; before it
//! the payload below was accepted (see `📓️w13-fem2d-semantics.md` for the per-kind rule table).
//!
//! 🚫️ `replace-element` now resolves the SAME four foreign keys `create-element` does, through the
//! same `guards::element_references`. The `start` node is read first, so `n42` is the address the
//! diagnostic carries even though the rest of the member is well formed.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-element/🚫️dangling-start-cda887/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-element/🚫️dangling-start-cda887/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-element/🚫️dangling-start-cda887/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-element/🚫️dangling-start-cda887/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused `replace-element` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-element/dangling-start-cda887: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "replace-element/dangling-start-cda887: a refused mutation must leave the snapshot exactly where it was");
}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem2dDiff::default(), "replace-element/dangling-start-cda887: a rejecting replace-element must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "replace-element/dangling-start-cda887: the refusal is reported as mutation.target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing target is an Error, the level a merge policy may still choose to tolerate");
    assert_eq!(messages[0].target, vec!["n42".to_string()], "the diagnostic addresses the offending id");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&mutation());
    assert_eq!(semantics.kind, "replace-element", "the fixture must be bound to replace-element's own descriptor");
}

/// ↩️ `replace-element`'s inverse is BASE-derived: it restores the element `before` holds, addressed by the SELECTED id — never by the id the refused replacement carried.
#[test]
fn inverse_of_the_refused_mutation() {
    let inverse = inverse_fem2d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "replace-element/dangling-start-cda887: replace-element undoes with exactly one step, got {inverse:?}");
    let Fem2dMutation::ReplaceElement(undo) = &inverse[0] else {
        panic!("replace-element's inverse must be a ReplaceElement, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "br1", "the inverse addresses the selected id, not the one the replacement carried");
    assert_eq!(undo.new_element.as_ref(), before().elements.iter().find(|item| crate::element_id(item) == "br1").expect("the target survives a refusal"), "the inverse restores the pre-mutation element");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "replace-element/dangling-start-cda887 declares a rejected outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared, message.target, "the declared path must match the emitted target");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-element/dangling-start-cda887: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "replace-element/dangling-start-cda887 changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-element/dangling-start-cda887: committed mutation JSON is not canonical");
}
