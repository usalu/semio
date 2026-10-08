//! 🧪️ `delete-element` fixture — `📍️removes-a-middle-row`.
//!
//! 📍️ The removed record sits in the MIDDLE of its collection, so the concrete inverse must restore it at its original index, not append it.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation,inverse_fem2d_mutation};

use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/📍️removes-a-middle-row/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/📍️removes-a-middle-row/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/📍️removes-a-middle-row/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/📍️removes-a-middle-row/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-element/📍️removes-a-middle-row/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `delete-element` carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-element/removes-a-middle-row: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, base, "delete-element/removes-a-middle-row: the forward mutation left the model untouched, so nothing was proved");
    assert_eq!(snapshot.elements.len(), 8, "delete-element/removes-a-middle-row: the elements collection must end up 8 records long");
    assert!(!snapshot.elements.iter().any(|item| crate::element_id(item) == "br1"), "delete-element/removes-a-middle-row: no record named br1 may survive");
}

/// ↩️ Applying the computed inverse after the forward step lands back on `before`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "delete-element/removes-a-middle-row: delete-element undoes with exactly one step, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-element/removes-a-middle-row: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — applied, with no diagnostic at all — is what this kind really emits.
#[test]
fn declared_outcome_holds() {
    let outcome: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_value::DslValue::as_str), Some("applied"), "delete-element/removes-a-middle-row declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-element/removes-a-middle-row: a clean application raises no diagnostic, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-element/removes-a-middle-row: declared applied but the mutation was rejected");
}

/// 🔀️ Each verb writes exactly ONE of the nine members. An after-snapshot comparison cannot make
/// this check on its own: an implementation that re-derived a sibling collection on every edit
/// would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-element applies to its committed before-snapshot");
    assert_eq!(snapshot.nodes, base.nodes, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but nodes moved");
    assert_eq!(snapshot.regions, base.regions, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but regions moved");
    assert_eq!(snapshot.materials, base.materials, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but materials moved");
    assert_eq!(snapshot.sections, base.sections, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but sections moved");
    assert_eq!(snapshot.supports, base.supports, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but supports moved");
    assert_eq!(snapshot.load_cases, base.load_cases, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but loadCases moved");
    assert_eq!(snapshot.combinations, base.combinations, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but combinations moved");
    assert_eq!(snapshot.analysis, base.analysis, "delete-element/removes-a-middle-row: this verb writes elements and nothing else, but analysis moved");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-element/removes-a-middle-row: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-element/removes-a-middle-row: committed mutation JSON is not canonical");
}

/// 🔺️ The delta must be exactly the committed one, on exactly the `elements` slot.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    let delta = outcome.diff().elements.as_ref().expect("elements delta");
    assert_eq!((delta.added.len(), delta.removed.len(), delta.patched.len()), (0, 1, 0), "delete-element/removes-a-middle-row: the delta must be exactly one removed entry");
    assert!(delta.reordered.is_none(), "delete-element/removes-a-middle-row: no verb in this vocabulary re-orders a collection");
    assert!(outcome.diff().nodes.is_none(), "delete-element/removes-a-middle-row: no nodes delta may be opened by this verb");
    assert!(outcome.diff().regions.is_none(), "delete-element/removes-a-middle-row: no regions delta may be opened by this verb");
    assert!(outcome.diff().materials.is_none(), "delete-element/removes-a-middle-row: no materials delta may be opened by this verb");
    assert!(outcome.diff().sections.is_none(), "delete-element/removes-a-middle-row: no sections delta may be opened by this verb");
    assert!(outcome.diff().supports.is_none(), "delete-element/removes-a-middle-row: no supports delta may be opened by this verb");
    assert!(outcome.diff().load_cases.is_none(), "delete-element/removes-a-middle-row: no loadCases delta may be opened by this verb");
    assert!(outcome.diff().combinations.is_none(), "delete-element/removes-a-middle-row: no combinations delta may be opened by this verb");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-element/removes-a-middle-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-element/removes-a-middle-row: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed delta on `before` must reproduce the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-element/removes-a-middle-row: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
