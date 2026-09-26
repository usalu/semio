//! 🧪️ `insert-connection` — `inserts-a-connection-at-end`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::insert_connection::InsertConnection;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn inserts_a_connection_at_end() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = InsertConnection { index: 99, connection: crate::TimberConnection { id: "conn-C9".into(), ..base.connections[0].clone() } };
    let outcome = <InsertConnection as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.connections.len(), base.connections.len() + 1);
    assert_eq!(next.connections.last().unwrap().id, "conn-C9");
    let inverse = <InsertConnection as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::InsertConnection(payload));
}
