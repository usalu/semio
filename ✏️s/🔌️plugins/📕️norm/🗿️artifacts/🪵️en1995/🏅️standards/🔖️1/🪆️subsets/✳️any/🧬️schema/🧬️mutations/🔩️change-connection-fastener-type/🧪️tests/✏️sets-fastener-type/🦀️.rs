//! 🧪️ `change-connection-fastener-type` — `sets-fastener-type`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::change_connection_fastener_type::ChangeConnectionFastenerType;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn sets_fastener_type() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = ChangeConnectionFastenerType { connection_id: base.connections[0].id.clone(), new_value: "screw".into() };
    let outcome = <ChangeConnectionFastenerType as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.connections[0].fastener_type, payload.new_value.clone());
    assert_ne!(next, base);
    let inverse = <ChangeConnectionFastenerType as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::ChangeConnectionFastenerType(payload));
}
