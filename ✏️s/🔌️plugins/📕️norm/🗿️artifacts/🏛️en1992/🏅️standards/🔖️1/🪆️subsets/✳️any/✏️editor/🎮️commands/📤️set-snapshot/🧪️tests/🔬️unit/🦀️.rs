use super::*;
use crate::standards::v1::subsets::any::schema::mutations::En1992Mutation;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn handle_commits_the_payload_document_as_its_field_mutations() {
    let projection = En1992Snapshot::default();
    let config = NoConfig::default();
    let emit = handle(&ReplaceSnapshot { snapshot: En1992Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle");
    assert_eq!(emit.artifact_mutations, En1992Mutation::from_snapshot(&projection, &En1992Snapshot::default()));
    assert!(emit.config_mutations.is_empty());
}

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-member-width/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-member-width/✅apply/📸️snapshot/➡️after/🔣️.json");

fn replace(projection: &En1992Snapshot, payload: &En1992Snapshot) -> Vec<En1992Mutation> {
    let config = NoConfig::default();
    handle(&ReplaceSnapshot { snapshot: payload.clone() }, &ArtifactView::new(projection, &HistoryView::empty()), &ConfigView { snapshot: &config, window: None }).expect("handle").artifact_mutations
}

#[test]
fn replacing_a_document_by_itself_emits_nothing() {
    let document: En1992Snapshot = semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before");
    assert_eq!(replace(&document, &document), Vec::<En1992Mutation>::new());
}

#[test]
fn replacing_a_document_reaches_the_payload() {
    let before: En1992Snapshot = semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before");
    let after: En1992Snapshot = semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after");
    let reached = replace(&before, &after).iter().fold(before.clone(), |document, mutation| {
        let raised = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(mutation, &document);
        assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
        protocol::apply_diff(raised.diff(), &document).expect("apply")
    });
    assert_eq!(reached, after);
}
