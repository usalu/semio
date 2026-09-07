//! 🧪️ `replace-section` fixture — `🪪️denies-rename-1d02dd`.
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
//! 🪪️ The roof beam names `ipe240`; renaming the profile row through a replace would leave it
//! pointing at nothing. `mutation.id-mismatch`, FATAL.

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
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

/// ▶️ A refused `replace-section` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-section/denies-rename-1d02dd: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "replace-section/denies-rename-1d02dd: a refused mutation must leave the snapshot exactly where it was");
}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem2d::diff::Fem2dDiff::default(), "replace-section/denies-rename-1d02dd: a rejecting replace-section must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.id-mismatch", "replace-section/denies-rename-1d02dd: the refusal is reported as mutation.id-mismatch");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "a replacement that renames its target is an identity breach — Fatal, and no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["ipe240".to_string(), "ipe240a".to_string()], "the diagnostic addresses the target first, then every referrer");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&mutation());
    assert_eq!(semantics.kind, "replace-section", "the fixture must be bound to replace-section's own descriptor");
}

/// ↩️ `replace-section`'s inverse is BASE-derived: it restores the section `before` holds, addressed by the SELECTED id.
#[test]
fn inverse_of_the_refused_mutation() {
    let inverse = inverse_fem2d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "replace-section/denies-rename-1d02dd: replace-section undoes with exactly one step, got {inverse:?}");
    let Fem2dMutation::ReplaceSection(undo) = &inverse[0] else {
        panic!("replace-section's inverse must be a ReplaceSection, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "ipe240", "the inverse addresses the selected id, not the one the replacement carried");
    assert_eq!(&undo.new_section, before().sections.iter().find(|item| item.id == "ipe240").expect("the target survives a refusal"), "the inverse restores the pre-mutation section");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "replace-section/denies-rename-1d02dd declares a rejected outcome");
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
        assert_eq!(reencoded, original, "replace-section/denies-rename-1d02dd: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "replace-section/denies-rename-1d02dd changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-section/denies-rename-1d02dd: committed mutation JSON is not canonical");
}
