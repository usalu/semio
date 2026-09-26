use crate::mutations::insert_crane_runway::InsertCraneRunway;
use crate::{CraneRunway, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = CraneRunway { id: "crane-new".into(), member_id: "member-b1".into(), wheel_contact_length: 0.08, dispersion: 0.04, web_thickness: 0.01, phi: 1.1, fy: 355e6, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "q-imposed".into(), force: 40_000.0 }] };
    let payload = InsertCraneRunway { index: base.crane_runways.len(), crane_runway: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.crane_runways.len(), base.crane_runways.len() + 1);
}
