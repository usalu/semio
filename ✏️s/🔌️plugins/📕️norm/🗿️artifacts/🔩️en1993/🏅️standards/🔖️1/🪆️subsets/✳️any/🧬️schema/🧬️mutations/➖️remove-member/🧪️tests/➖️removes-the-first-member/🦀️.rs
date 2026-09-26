use crate::mutations::{remove_member::RemoveMember, insert_member::InsertMember};
use crate::{SteelMember, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = SteelMember { id: "member-new".into(), label: "New".into(), member_type: "beam".into(), section_id: "sec-heb240".into(), material_id: "mat-s355".into(), length: 3.0, buckling_length_y: 3.0, buckling_length_z: 3.0, ltb_length: 3.0, ltb_restraint_spacing: 1.5, load_application: "shearCenter".into(), end_moment_ratio_psi: -1.0, moment_diagram: "linear".into(), deflection_limit_ratio: 300.0 };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertMember { index: 0, member: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.members.len();
    let out = protocol::MutationKind::diff(&RemoveMember { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.members.len(), before_len - 1);
}
