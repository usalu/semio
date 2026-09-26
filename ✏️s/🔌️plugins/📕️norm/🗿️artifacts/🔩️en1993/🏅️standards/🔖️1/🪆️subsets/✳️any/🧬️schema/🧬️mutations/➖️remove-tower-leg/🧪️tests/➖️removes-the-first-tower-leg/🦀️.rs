use crate::mutations::{remove_tower_leg::RemoveTowerLeg, insert_tower_leg::InsertTowerLeg};
use crate::{TowerLeg, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = TowerLeg { id: "tw-new".into(), member_id: "member-b1".into(), force_coefficient: 1.2,
        dynamic_factor: 1.1, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 200_000.0 }] };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertTowerLeg { index: 0, tower_leg: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.tower_legs.len();
    let out = protocol::MutationKind::diff(&RemoveTowerLeg { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.tower_legs.len(), before_len - 1);
}
