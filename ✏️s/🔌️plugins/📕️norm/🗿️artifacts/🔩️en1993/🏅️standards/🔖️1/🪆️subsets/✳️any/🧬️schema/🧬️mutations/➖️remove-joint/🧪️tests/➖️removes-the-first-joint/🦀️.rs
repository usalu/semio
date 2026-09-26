use crate::mutations::{remove_joint::RemoveJoint, insert_joint::InsertJoint};
use crate::{SteelJoint, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = SteelJoint { id: "joint-new".into(), kind: "bolted".into(), member_id: "member-b1".into(), bolt_class: "8.8".into(), bolt_diameter: 0.016, bolt_rows: 1, bolts_per_row: 2, pitch: 0.05, gauge: 0.05, end_distance: 0.03, edge_distance: 0.03, shear_planes: 1, plate_thickness: 0.008, plate_fu: 510e6, weld_throat: 0.0, weld_length: 0.0, weld_fu: 510e6, weld_grade: "S355".into() ,
                actions: vec![crate::JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 40_000.0, tension: 0.0 }], category: "A".into(),
                friction_mu: 0.50,
                preload_force: 0.0,
                slip_factor_ks: 1.0,
                friction_surfaces: 1,
            };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertJoint { index: 0, joint: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.joints.len();
    let out = protocol::MutationKind::diff(&RemoveJoint { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.joints.len(), before_len - 1);
}
