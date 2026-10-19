//! 🧪️ `create-element` fixture — `➖️appends`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! An axial `Bar` brace joins the 6-DOF `Frame` column — the bar variant carries no `roll`, and must not gain one.

use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::inverse_fem3d_mutation;
use semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

static BEFORE: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧩️create-element/➖️appends/📸️snapshot/⬅️before/🔣️.json");
static AFTER: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧩️create-element/➖️appends/📸️snapshot/➡️after/🔣️.json");
static MUTATION: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧩️create-element/➖️appends/🦠️mutation/🔣️.json");
static DIFF: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧩️create-element/➖️appends/🔺️diff/🔣️.json");
static OUTCOME: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧩️create-element/➖️appends/🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshotValue {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshotValue {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `create-element` appends the bracing bar `b1` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("create-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-element/appends-a-diagonal-bracing-bar: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.elements.len(), 2, "create-element/appends-a-diagonal-bracing-bar: the brace must be appended beside the frame");
    assert!(matches!(snapshot.elements[1], semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement::Bar { .. }), "create-element/appends-a-diagonal-bracing-bar: the brace must stay a Bar, never widen into a Frame");
    assert!(matches!(snapshot.elements[0], semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement::Frame { .. }), "create-element/appends-a-diagonal-bracing-bar: the pre-existing column must stay a Frame");
}

/// ↩️ The inverse is a `delete-element` of `b1`, leaving the lone frame behind.
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
    assert_eq!(snapshot, base, "create-element/appends-a-diagonal-bracing-bar: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshotValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-element/appends-a-diagonal-bracing-bar: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-element/appends-a-diagonal-bracing-bar: committed mutation JSON is not canonical");
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
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshotValue>>::diff(&mutation(), &before());
    let refused = produced.messages().iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error);
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("the produced diff applies to its own before-snapshot");
    match status {
        "applied" => assert!(!refused, "create-element/appends-a-diagonal-bracing-bar: declared applied but the diff builder refused with {:?}", produced.messages()),
        "rejected" => {
            assert!(refused, "create-element/appends-a-diagonal-bracing-bar: declared rejected but the diff builder raised no Error or Fatal, only {:?}", produced.messages());
            assert_eq!(produced.diff(), &semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "create-element/appends-a-diagonal-bracing-bar: a refused mutation must carry the empty diff");
            assert_eq!(snapshot, before(), "create-element/appends-a-diagonal-bracing-bar: a refused mutation must leave the snapshot untouched");
        }
        other => panic!("create-element/appends-a-diagonal-bracing-bar: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `elements.added` entry tagged `bar`.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshotValue>>::diff(&mutation(), &base);
    assert!(outcome.diff().elements.is_some(), "create-element/appends-a-diagonal-bracing-bar: the created brace must surface in the elements delta");
    assert!(outcome.diff().nodes.is_none() && outcome.diff().materials.is_none() && outcome.diff().sections.is_none(), "create-element/appends-a-diagonal-bracing-bar: the referenced node/material/section rows are read-only for this mutation");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-element/appends-a-diagonal-bracing-bar: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-element/appends-a-diagonal-bracing-bar: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `elements.added` entry on `before` must reproduce frame-then-brace.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-element/appends-a-diagonal-bracing-bar: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
