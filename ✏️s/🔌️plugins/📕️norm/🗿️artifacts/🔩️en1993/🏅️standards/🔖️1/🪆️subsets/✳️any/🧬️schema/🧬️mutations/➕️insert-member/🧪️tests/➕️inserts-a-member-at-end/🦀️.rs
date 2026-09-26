use crate::mutations::insert_member::InsertMember;
use crate::{SteelMember, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = SteelMember { id: "member-new".into(), label: "New".into(), member_type: "beam".into(), section_id: "sec-heb240".into(), material_id: "mat-s355".into(), length: 3.0, buckling_length_y: 3.0, buckling_length_z: 3.0, ltb_length: 3.0, ltb_restraint_spacing: 1.5, load_application: "shearCenter".into(), end_moment_ratio_psi: -1.0, moment_diagram: "linear".into(), deflection_limit_ratio: 300.0 };
    let payload = InsertMember { index: base.members.len(), member: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.members.len(), base.members.len() + 1);
}
