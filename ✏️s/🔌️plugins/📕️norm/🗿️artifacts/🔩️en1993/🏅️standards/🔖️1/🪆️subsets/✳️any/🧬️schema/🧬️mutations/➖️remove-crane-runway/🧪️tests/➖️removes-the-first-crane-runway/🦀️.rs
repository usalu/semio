use crate::mutations::{remove_crane_runway::RemoveCraneRunway, insert_crane_runway::InsertCraneRunway};
use crate::{CraneRunway, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = CraneRunway { id: "crane-new".into(), member_id: "member-b1".into(), wheel_contact_length: 0.08, dispersion: 0.04, web_thickness: 0.01, phi: 1.1, fy: 355e6, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "q-imposed".into(), force: 40_000.0 }] };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertCraneRunway { index: 0, crane_runway: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.crane_runways.len();
    let out = protocol::MutationKind::diff(&RemoveCraneRunway { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.crane_runways.len(), before_len - 1);
}
