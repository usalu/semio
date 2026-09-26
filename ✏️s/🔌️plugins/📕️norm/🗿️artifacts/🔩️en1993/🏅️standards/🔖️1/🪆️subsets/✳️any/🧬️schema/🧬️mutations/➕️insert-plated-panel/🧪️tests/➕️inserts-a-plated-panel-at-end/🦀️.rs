use crate::mutations::insert_plated_panel::InsertPlatedPanel;
use crate::{PlatedPanel, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = PlatedPanel { id: "pl-new".into(), a: 1.0, b: 0.4, thickness: 0.012, fy: 355e6, k_sigma: 4.0, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 150e6 }] };
    let payload = InsertPlatedPanel { index: base.plated_panels.len(), plated_panel: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.plated_panels.len(), base.plated_panels.len() + 1);
}
