//! 🧪️ `remove-connection` — `removes-the-first-connection`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::remove_connection::RemoveConnection;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn removes_the_first_connection() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = RemoveConnection { index: 0 };
    let outcome = <RemoveConnection as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.connections.len(), base.connections.len() - 1);
    let inverse = <RemoveConnection as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::RemoveConnection(payload));
}
