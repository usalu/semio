use crate::mutations::insert_cold_formed_member::InsertColdFormedMember;
use crate::{ColdFormedMember, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = ColdFormedMember { id: "cf-new".into(), b_bar: 0.08, thickness: 0.002, k_sigma: 4.0, psi: 1.0, fy: 355e6, gross_resistance: 40_000.0, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 10_000.0 }] };
    let payload = InsertColdFormedMember { index: base.cold_formed_members.len(), cold_formed_member: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.cold_formed_members.len(), base.cold_formed_members.len() + 1);
}
