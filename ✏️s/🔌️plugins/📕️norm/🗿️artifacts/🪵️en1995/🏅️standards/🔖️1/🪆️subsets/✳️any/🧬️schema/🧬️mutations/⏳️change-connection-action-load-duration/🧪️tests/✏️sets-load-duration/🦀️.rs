//! 🧪️ `change-connection-action-load-duration` — `sets-load-duration`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::change_connection_action_load_duration::ChangeConnectionActionLoadDuration;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn sets_load_duration() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = ChangeConnectionActionLoadDuration { connection_id: base.connections[0].id.clone(), action_id: base.connections[0].actions[0].id.clone(), new_value: "short".into() };
    let outcome = <ChangeConnectionActionLoadDuration as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.connections[0].actions[0].load_duration, payload.new_value.clone());
    assert_ne!(next, base);
    let inverse = <ChangeConnectionActionLoadDuration as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::ChangeConnectionActionLoadDuration(payload));
}
