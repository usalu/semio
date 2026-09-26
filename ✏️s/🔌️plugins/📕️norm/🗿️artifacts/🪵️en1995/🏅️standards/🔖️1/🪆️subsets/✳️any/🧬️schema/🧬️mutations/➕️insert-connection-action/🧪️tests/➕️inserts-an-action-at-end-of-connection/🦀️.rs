//! 🧪️ `insert-connection-action` — `inserts-an-action-at-end-of-connection`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::insert_connection_action::InsertConnectionAction;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn inserts_an_action_at_end_of_connection() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = InsertConnectionAction { connection_id: base.connections[0].id.clone(), index: 99, action: crate::ConnectionAction { id: "w".into(), ..base.connections[0].actions[0].clone() } };
    let outcome = <InsertConnectionAction as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.connections[0].actions.len(), base.connections[0].actions.len() + 1);
    assert_eq!(next.connections[0].actions.last().unwrap().id, "w");
    let inverse = <InsertConnectionAction as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::InsertConnectionAction(payload));
}
