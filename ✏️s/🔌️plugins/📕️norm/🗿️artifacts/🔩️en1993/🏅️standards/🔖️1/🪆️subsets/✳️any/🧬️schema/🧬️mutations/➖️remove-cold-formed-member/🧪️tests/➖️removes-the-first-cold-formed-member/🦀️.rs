use crate::mutations::{remove_cold_formed_member::RemoveColdFormedMember, insert_cold_formed_member::InsertColdFormedMember};
use crate::{ColdFormedMember, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = ColdFormedMember { id: "cf-new".into(), b_bar: 0.08, thickness: 0.002, k_sigma: 4.0, psi: 1.0, fy: 355e6, gross_resistance: 40_000.0, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 10_000.0 }] };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertColdFormedMember { index: 0, cold_formed_member: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.cold_formed_members.len();
    let out = protocol::MutationKind::diff(&RemoveColdFormedMember { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.cold_formed_members.len(), before_len - 1);
}
