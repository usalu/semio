//! 🧪️ `insert-member` — `inserts-a-member-at-end`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::insert_member::InsertMember;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn inserts_a_member_at_end() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = InsertMember { index: 99, member: crate::TimberMember { id: "beam-B9".into(), ..base.members[0].clone() } };
    let outcome = <InsertMember as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.members.len(), base.members.len() + 1);
    assert_eq!(next.members.last().unwrap().id, "beam-B9");
    let inverse = <InsertMember as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::InsertMember(payload));
}
