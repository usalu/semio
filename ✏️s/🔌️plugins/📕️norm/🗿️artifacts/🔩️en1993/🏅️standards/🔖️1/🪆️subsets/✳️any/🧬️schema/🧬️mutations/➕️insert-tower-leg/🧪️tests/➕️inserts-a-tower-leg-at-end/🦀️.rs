use crate::mutations::insert_tower_leg::InsertTowerLeg;
use crate::{TowerLeg, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = TowerLeg { id: "tw-new".into(), member_id: "member-b1".into(), force_coefficient: 1.2,
        dynamic_factor: 1.1, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 200_000.0 }] };
    let payload = InsertTowerLeg { index: base.tower_legs.len(), tower_leg: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.tower_legs.len(), base.tower_legs.len() + 1);
}
