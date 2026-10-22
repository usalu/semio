//! 🧪️ `delete-solid` fixture — `🚫️removes`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! The last solid leaves; the `c30` material it was the only consumer of stays in the catalogue.

use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::inverse_fem3d_mutation_cold;
use semio_s_artifact_fem_3d_mutations::central_apply::apply_fem3d_mutation;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

static BEFORE: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-solid/🚫️removes/📸️snapshot/⬅️before/🔣️.json");
static AFTER: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-solid/🚫️removes/📸️snapshot/➡️after/🔣️.json");
static MUTATION: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-solid/🚫️removes/🦠️mutation/🔣️.json");
static DIFF: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-solid/🚫️removes/🔺️diff/🔣️.json");
static OUTCOME: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-solid/🚫️removes/🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshotValue {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshotValue {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `delete-solid` drops the roof slab and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("delete-solid applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-solid/removes-the-roof-slab-f0fb64: applied state differs from committed after-snapshot");
    assert!(snapshot.solids.is_empty(), "delete-solid/removes-the-roof-slab-f0fb64: the solid list must be empty afterwards");
    assert_eq!(snapshot.materials, before().materials, "delete-solid/removes-the-roof-slab-f0fb64: c30 stays even though it lost its only consumer");
}

/// ↩️ The inverse is a `create-solid` rebuilt from `base`, restoring the footprint verbatim.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation_cold(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-solid/removes-the-roof-slab-f0fb64: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshotValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-solid/removes-the-roof-slab-f0fb64: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-solid/removes-the-roof-slab-f0fb64: committed mutation JSON is not canonical");
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
    let produced = <Fem3dMutation as protocol::ColdMutation<Fem3dSnapshotValue>>::diff(&mutation(), &before());
    let refused = produced.messages().iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error);
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("the produced diff applies to its own before-snapshot");
    match status {
        "applied" => assert!(!refused, "delete-solid/removes-the-roof-slab-f0fb64: declared applied but the diff builder refused with {:?}", produced.messages()),
        "rejected" => {
            assert!(refused, "delete-solid/removes-the-roof-slab-f0fb64: declared rejected but the diff builder raised no Error or Fatal, only {:?}", produced.messages());
            assert_eq!(produced.diff(), &semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "delete-solid/removes-the-roof-slab-f0fb64: a refused mutation must carry the empty diff");
            assert_eq!(snapshot, before(), "delete-solid/removes-the-roof-slab-f0fb64: a refused mutation must leave the snapshot untouched");
        }
        other => panic!("delete-solid/removes-the-roof-slab-f0fb64: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `solids.removed` id — a solid never drags its material out with it.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::ColdMutation<Fem3dSnapshotValue>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().solids.as_ref().expect("solids delta").removed.iter().map(|entry| entry.id.clone()).collect::<Vec<_>>(), vec!["slab".to_string()], "delete-solid/removes-the-roof-slab-f0fb64: exactly the slab may be removed");
    assert!(outcome.diff().materials.is_none(), "delete-solid/removes-the-roof-slab-f0fb64: no material delta may be opened");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-solid/removes-the-roof-slab-f0fb64: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-solid/removes-the-roof-slab-f0fb64: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `solids.removed` id on `before` must leave an empty solid list.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-solid/removes-the-roof-slab-f0fb64: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
