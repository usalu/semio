//! 🧪️ `change-connection-label-de` — `sets-labelDe`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::change_connection_label_de::ChangeConnectionLabelDe;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn sets_label_de() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = ChangeConnectionLabelDe { connection_id: base.connections[0].id.clone(), new_value: "Schraubverbund (überarbeitet)".into() };
    let outcome = <ChangeConnectionLabelDe as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_eq!(next.connections[0].label_de, payload.new_value.clone());
    assert_ne!(next, base);
    let inverse = <ChangeConnectionLabelDe as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::ChangeConnectionLabelDe(payload));
}
