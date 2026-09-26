use crate::mutations::{remove_plated_panel::RemovePlatedPanel, insert_plated_panel::InsertPlatedPanel};
use crate::{PlatedPanel, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = PlatedPanel { id: "pl-new".into(), a: 1.0, b: 0.4, thickness: 0.012, fy: 355e6, k_sigma: 4.0, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 150e6 }] };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertPlatedPanel { index: 0, plated_panel: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.plated_panels.len();
    let out = protocol::MutationKind::diff(&RemovePlatedPanel { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.plated_panels.len(), before_len - 1);
}
