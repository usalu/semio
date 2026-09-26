use crate::mutations::insert_joint::InsertJoint;
use crate::{SteelJoint, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = SteelJoint { id: "joint-new".into(), kind: "bolted".into(), member_id: "member-b1".into(), bolt_class: "8.8".into(), bolt_diameter: 0.016, bolt_rows: 1, bolts_per_row: 2, pitch: 0.05, gauge: 0.05, end_distance: 0.03, edge_distance: 0.03, shear_planes: 1, plate_thickness: 0.008, plate_fu: 510e6, weld_throat: 0.0, weld_length: 0.0, weld_fu: 510e6, weld_grade: "S355".into() ,
                actions: vec![crate::JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 40_000.0, tension: 0.0 }], category: "A".into(),
                friction_mu: 0.50,
                preload_force: 0.0,
                slip_factor_ks: 1.0,
                friction_surfaces: 1,
            };
    let payload = InsertJoint { index: base.joints.len(), joint: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.joints.len(), base.joints.len() + 1);
}
