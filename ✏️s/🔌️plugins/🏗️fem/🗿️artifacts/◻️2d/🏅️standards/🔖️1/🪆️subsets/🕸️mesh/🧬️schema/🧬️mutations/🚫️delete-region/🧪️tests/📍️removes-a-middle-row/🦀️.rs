//! 🧪️ `delete-region` fixture — `📍️removes-a-middle-row`.
//!
//! 📍️ The removed record sits in the MIDDLE of its collection, so the concrete inverse must restore it at its original index, not append it.

use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::inverse_fem2d_mutation;
use semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

static BEFORE: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-region/📍️removes-a-middle-row/📸️snapshot/⬅️before/🔣️.json");
static AFTER: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-region/📍️removes-a-middle-row/📸️snapshot/➡️after/🔣️.json");
static MUTATION: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-region/📍️removes-a-middle-row/🦠️mutation/🔣️.json");
static DIFF: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-region/📍️removes-a-middle-row/🔺️diff/🔣️.json");
static OUTCOME: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-region/📍️removes-a-middle-row/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshotValue {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshotValue {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `delete-region` drops the slab and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-region applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-region/removes-a-middle-row: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.regions.iter().map(|region| region.id.as_str()).collect::<Vec<_>>(), ["slab-before", "slab-after"], "delete-region/removes-a-middle-row: both neighbours of the deleted row survive in order");
    assert_eq!(snapshot.materials, before().materials, "delete-region/removes-a-middle-row: c30 stays even though it lost its only consumer");
}

/// ↩️ The inverse is a `create-region` rebuilt from `base`, restoring the outline verbatim.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-region/removes-a-middle-row: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshotValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-region/removes-a-middle-row: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-region/removes-a-middle-row: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(OUTCOME.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    let status = outcome.get("status").and_then(semio_framework_value::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-region/removes-a-middle-row: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-region/removes-a-middle-row: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-region/removes-a-middle-row: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-region/removes-a-middle-row: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `regions.removed` id — a region never drags its material out with it.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshotValue>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().regions.as_ref().expect("regions delta").removed.iter().map(|entry| entry.id.clone()).collect::<Vec<_>>(), vec!["slab".to_string()], "delete-region/removes-a-middle-row: exactly the slab may be removed");
    assert!(outcome.diff().materials.is_none(), "delete-region/removes-a-middle-row: no material delta may be opened");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-region/removes-a-middle-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-region/removes-a-middle-row: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `regions.removed` id on `before` must leave an empty region list.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-region/removes-a-middle-row: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
