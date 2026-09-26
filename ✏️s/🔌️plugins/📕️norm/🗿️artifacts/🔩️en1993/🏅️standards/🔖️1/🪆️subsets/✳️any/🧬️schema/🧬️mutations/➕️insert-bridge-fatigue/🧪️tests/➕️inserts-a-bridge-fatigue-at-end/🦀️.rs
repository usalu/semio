use crate::mutations::insert_bridge_fatigue::InsertBridgeFatigue;
use crate::{BridgeFatigue, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = BridgeFatigue { id: "br-new".into(), member_id: "member-b1".into(), lambda: 1.0, phi2: 1.0, delta_sigma_p: 25e6, category: 71, method: "damage_tolerant".into() };
    let payload = InsertBridgeFatigue { index: base.bridge_fatigue.len(), bridge_fatigue_item: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.bridge_fatigue.len(), base.bridge_fatigue.len() + 1);
}
