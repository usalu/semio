//! 🧪️ Committed vector `➕️insert-fire-exposure` / `⛔dupe`: re-applying the op to the after-snapshot it already produced is refused with `mutation.duplicate-id` (fatal) and leaves the document untouched.
use crate::{En1993Diff, En1993Mutation, En1993Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-fire-exposure/⛔dupe/🎯️outcome/🔣️.json");
fn before() -> En1993Snapshot { semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before") }
fn mutation() -> En1993Mutation { semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation") }
#[test]
fn mutation_is_the_canonical_wire() {
    let _: En1993Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}
#[test]
fn refuses_with_the_declared_code() {
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &before());
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome");
    let raised_codes: Vec<(String, String)> = raised.messages().iter().map(|message| (format!("{:?}", message.level).to_lowercase(), message.code.0.clone())).collect();
    assert_eq!(raised_codes, vec![(outcome["messages"][0]["level"].as_str().expect("level").to_string(), outcome["code"].as_str().expect("code").to_string())]);
    assert_eq!(outcome["status"], "rejected");
    assert_eq!(*raised.diff(), En1993Diff::default());
    assert!(DIFF_ABSENT.is_empty());
}
#[test]
fn leaves_the_document_untouched() {
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation(), &before());
    let after = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(raised.diff(), &before()).expect("apply");
    assert_eq!(after, before());
    assert_eq!(semio_framework_pack_json::from_json_str::<En1993Snapshot>(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after"), before());
}
