//! 🧪️ `insert-member-action` — `inserts-an-action-at-end-of-member`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::insert_member_action::InsertMemberAction;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn inserts_an_action_at_end_of_member() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = InsertMemberAction { member_id: base.members[0].id.clone(), index: 99, action: crate::CharacteristicAction { id: "w".into(), ..base.members[0].actions[0].clone() } };
    let outcome = <InsertMemberAction as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.members[0].actions.len(), base.members[0].actions.len() + 1);
    assert_eq!(next.members[0].actions.last().unwrap().id, "w");
    let inverse = <InsertMemberAction as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::InsertMemberAction(payload));
}
