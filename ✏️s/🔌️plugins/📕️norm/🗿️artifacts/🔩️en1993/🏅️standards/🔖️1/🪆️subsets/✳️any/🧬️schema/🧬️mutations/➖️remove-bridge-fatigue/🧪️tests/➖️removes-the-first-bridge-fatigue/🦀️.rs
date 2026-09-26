use crate::mutations::{remove_bridge_fatigue::RemoveBridgeFatigue, insert_bridge_fatigue::InsertBridgeFatigue};
use crate::{BridgeFatigue, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = BridgeFatigue { id: "br-new".into(), member_id: "member-b1".into(), lambda: 1.0, phi2: 1.0, delta_sigma_p: 25e6, category: 71, method: "damage_tolerant".into() };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertBridgeFatigue { index: 0, bridge_fatigue_item: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.bridge_fatigue.len();
    let out = protocol::MutationKind::diff(&RemoveBridgeFatigue { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.bridge_fatigue.len(), before_len - 1);
}
