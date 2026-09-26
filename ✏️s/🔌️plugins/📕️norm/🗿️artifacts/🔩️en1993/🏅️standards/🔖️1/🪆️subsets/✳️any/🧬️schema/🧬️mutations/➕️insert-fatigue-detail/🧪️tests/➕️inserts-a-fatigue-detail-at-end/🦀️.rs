use crate::mutations::insert_fatigue_detail::InsertFatigueDetail;
use crate::{FatigueDetail, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = FatigueDetail { id: "fat-new".into(), member_id: "member-b1".into(), category: 80, method: "damage_tolerant".into(), spectrum: vec![crate::FatigueBand { id: "fb".into(), delta_sigma: 40e6, cycles: 1e6 }] };
    let payload = InsertFatigueDetail { index: base.fatigue_details.len(), fatigue_detail: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.fatigue_details.len(), base.fatigue_details.len() + 1);
}
