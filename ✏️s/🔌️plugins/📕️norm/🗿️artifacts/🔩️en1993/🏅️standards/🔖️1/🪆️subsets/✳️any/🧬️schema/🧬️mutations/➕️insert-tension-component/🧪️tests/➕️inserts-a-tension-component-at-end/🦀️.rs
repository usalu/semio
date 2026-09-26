use crate::mutations::insert_tension_component::InsertTensionComponent;
use crate::{TensionComponent, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = TensionComponent { id: "ten-new".into(), f_uk: 400_000.0, f_k: 300_000.0, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 200_000.0 }] };
    let payload = InsertTensionComponent { index: base.tension_components.len(), tension_component: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.tension_components.len(), base.tension_components.len() + 1);
}
