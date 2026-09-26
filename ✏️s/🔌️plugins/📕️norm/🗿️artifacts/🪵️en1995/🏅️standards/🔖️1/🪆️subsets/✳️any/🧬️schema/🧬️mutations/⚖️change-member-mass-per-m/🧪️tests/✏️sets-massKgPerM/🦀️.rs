//! 🧪️ `change-member-mass-per-m` — `sets-massKgPerM`: applies, checks the written field, then replays the inverse back to BASE.
use crate::mutations::change_member_mass_per_m::ChangeMemberMassPerM;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;
#[test]
fn sets_mass_kg_per_m() {
    let base = En1995Snapshot::compliant_building_beam();
    let payload = ChangeMemberMassPerM { member_id: base.members[0].id.clone(), new_value: 140.0 };
    let outcome = <ChangeMemberMassPerM as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);
    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());
    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert!((next.members[0].mass_kg_per_m - payload.new_value).abs() < 1e-12);
    assert_ne!(next, base);
    let inverse = <ChangeMemberMassPerM as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);
    assert_eq!(inverse.len(), 1);
    let mut restored = next.clone();
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);
        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::ChangeMemberMassPerM(payload));
}
