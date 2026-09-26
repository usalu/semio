//! 🧪️ `remove-connection-action` — `removes-the-first-action-of-connection`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::remove_connection_action::RemoveConnectionAction;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn removes_the_first_action_of_connection() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = RemoveConnectionAction { connection_id: base.connections[0].id.clone(), index: 0 };
    let outcome = <RemoveConnectionAction as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.connections[0].actions.len(), base.connections[0].actions.len() - 1);
    let inverse = <RemoveConnectionAction as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::RemoveConnectionAction(payload));
}
