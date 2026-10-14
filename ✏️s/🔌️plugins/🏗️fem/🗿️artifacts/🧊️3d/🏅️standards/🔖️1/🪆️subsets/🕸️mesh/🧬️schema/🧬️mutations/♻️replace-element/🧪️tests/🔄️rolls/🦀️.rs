//! 🧪️ `replace-element` fixture — `🔄️rolls`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Only `roll` changes — the 3D-only local-axis angle that has no counterpart in the fem2d element at all.

use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::{inverse_fem3d_mutation};
use semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

static BEFORE: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls/📸️snapshot/⬅️before/🔣️.json");
static AFTER: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls/📸️snapshot/➡️after/🔣️.json");
static MUTATION: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls/🦠️mutation/🔣️.json");
static DIFF: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls/🔺️diff/🔣️.json");
static OUTCOME: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls/🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `replace-element` rolls frame `f1` in place and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("replace-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-element/rolls-the-column-50f732: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 1, "replace-element/rolls-the-column-50f732: a replacement must not change the element count");
    assert!(matches!(snapshot.elements[0], semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement::Frame { roll, .. } if roll == 1.5), "replace-element/rolls-the-column-50f732: the new roll angle must survive the round trip exactly");
    assert_eq!(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id(&snapshot.elements[0]), "f1", "replace-element/rolls-the-column-50f732: the identity must survive the whole-value swap");
}

/// ↩️ The inverse is a `replace-element` carrying the un-rolled frame recovered from `base`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-element/rolls-the-column-50f732: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE.as_str()), ("after", AFTER.as_str())] {
        let decoded: Fem3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-element/rolls-the-column-50f732: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-element/rolls-the-column-50f732: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
///
/// 🚦️ Refusal is read off the OUTCOME, never off the `Result`. `vcs::apply_mutation` is
/// policy-agnostic: a refused mutation carries the empty diff, so it still applies cleanly and
/// still returns `Ok` — asserting `is_err()` here would be a branch that can never fire.
#[test]
fn declared_outcome_holds() {
    let outcome: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(OUTCOME.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    let status = outcome.get("status").and_then(semio_framework_value::DslValue::as_str).expect("outcome carries a status");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let refused = produced.messages().iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error);
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("the produced diff applies to its own before-snapshot");
    match status {
        "applied" => assert!(!refused, "replace-element/rolls-the-column-50f732: declared applied but the diff builder refused with {:?}", produced.messages()),
        "rejected" => {
            assert!(refused, "replace-element/rolls-the-column-50f732: declared rejected but the diff builder raised no Error or Fatal, only {:?}", produced.messages());
            assert_eq!(produced.diff(), &semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "replace-element/rolls-the-column-50f732: a refused mutation must carry the empty diff");
            assert_eq!(snapshot, before(), "replace-element/rolls-the-column-50f732: a refused mutation must leave the snapshot untouched");
        }
        other => panic!("replace-element/rolls-the-column-50f732: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.modified` entry — a replacement never removes-then-adds.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().elements.as_ref().expect("elements delta").modified.len(), 1, "replace-element/rolls-the-column-50f732: exactly one element may be patched");
    assert!(outcome.diff().elements.as_ref().expect("elements delta").removed.is_empty(), "replace-element/rolls-the-column-50f732: a replacement must never be encoded as a remove-then-add");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-element/rolls-the-column-50f732: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-element/rolls-the-column-50f732: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.modified` entry on `before` must leave the rolled frame in `f1`'s slot.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-element/rolls-the-column-50f732: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
