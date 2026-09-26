//! 🧪️ `change-member-fire-duration` — `sets-fireDurationS`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::change_member_fire_duration::ChangeMemberFireDuration;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn sets_fire_duration_s() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = ChangeMemberFireDuration { member_id: base.members[0].id.clone(), new_value: 1800.0 };
    let outcome = <ChangeMemberFireDuration as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert!((next.members[0].fire_duration_s - payload.new_value).abs() < 1e-12);
    assert_ne!(next, base);
    let inverse = <ChangeMemberFireDuration as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::ChangeMemberFireDuration(payload));
}
