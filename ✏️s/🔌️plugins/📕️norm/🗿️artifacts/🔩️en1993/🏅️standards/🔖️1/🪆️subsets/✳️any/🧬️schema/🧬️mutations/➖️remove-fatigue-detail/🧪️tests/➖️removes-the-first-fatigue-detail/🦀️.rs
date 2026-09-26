use crate::mutations::{remove_fatigue_detail::RemoveFatigueDetail, insert_fatigue_detail::InsertFatigueDetail};
use crate::{FatigueDetail, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = FatigueDetail { id: "fat-new".into(), member_id: "member-b1".into(), category: 80, method: "damage_tolerant".into(), spectrum: vec![crate::FatigueBand { id: "fb".into(), delta_sigma: 40e6, cycles: 1e6 }] };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertFatigueDetail { index: 0, fatigue_detail: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.fatigue_details.len();
    let out = protocol::MutationKind::diff(&RemoveFatigueDetail { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.fatigue_details.len(), before_len - 1);
}
