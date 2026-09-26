use crate::mutations::{remove_tension_component::RemoveTensionComponent, insert_tension_component::InsertTensionComponent};
use crate::{TensionComponent, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = TensionComponent { id: "ten-new".into(), f_uk: 400_000.0, f_k: 300_000.0, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 200_000.0 }] };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertTensionComponent { index: 0, tension_component: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.tension_components.len();
    let out = protocol::MutationKind::diff(&RemoveTensionComponent { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.tension_components.len(), before_len - 1);
}
