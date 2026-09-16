//! 🧪️ `replace-combination` fixture — `🪪️denies-rename-a96d3c`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening), the
//! SECOND real-world fem2d model — the first is the timber portal frame the subset-level
//! differential cases share. Every value is in SI base units.
//!
//! 🪪️ A combination id is what a NESTED combination term resolves through, so a rename through a replacement would orphan every referrer silently. Renaming is `delete-` + `create-` + re-pointing the referrers.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-combination/🪪️denies-rename-a96d3c/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-combination/🪪️denies-rename-a96d3c/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-combination/🪪️denies-rename-a96d3c/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-combination/🪪️denies-rename-a96d3c/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused `replace-combination` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-combination/🪪️denies-rename-a96d3c: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "replace-combination/🪪️denies-rename-a96d3c: a refused mutation must leave the snapshot exactly where it was");
}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem2dDiff::default(), "replace-combination/🪪️denies-rename-a96d3c: a rejecting replace-combination must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.id-mismatch", "replace-combination/🪪️denies-rename-a96d3c: the refusal is reported as mutation.id-mismatch");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "a replacement that renames its target is an identity breach — Fatal, and no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["uls1".to_string(), "uls1_b".to_string()], "the diagnostic addresses the offending id");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&mutation());
    assert_eq!(semantics.kind, "replace-combination", "the fixture must be bound to replace-combination's own descriptor");
}

/// ↩️ `replace-combination`'s inverse is BASE-derived. It restores the combination `before` holds, addressed by the SELECTED id.
#[test]
fn inverse_of_the_refused_mutation() {
    let inverse = inverse_fem2d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "replace-combination/🪪️denies-rename-a96d3c: replace-combination undoes with exactly one step, got {inverse:?}");
    let Fem2dMutation::ReplaceCombination(undo) = &inverse[0] else {
        panic!("replace-combination's inverse must be a ReplaceCombination, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "uls1", "the inverse addresses the selected id, not the one the replacement carried");
    assert_eq!(&undo.new_combination, before().combinations.iter().find(|item| item.id == "uls1").expect("the target survives a refusal"), "the inverse restores the pre-mutation record");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "replace-combination/🪪️denies-rename-a96d3c declares a rejected outcome");
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
        assert_eq!(reencoded, original, "replace-combination/🪪️denies-rename-a96d3c: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "replace-combination/🪪️denies-rename-a96d3c changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-combination/🪪️denies-rename-a96d3c: committed mutation JSON is not canonical");
}
