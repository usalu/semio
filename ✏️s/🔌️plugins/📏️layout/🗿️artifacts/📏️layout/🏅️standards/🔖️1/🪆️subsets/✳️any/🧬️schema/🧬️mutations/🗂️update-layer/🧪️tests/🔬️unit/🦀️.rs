use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_layer_renames_and_locks_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::UpdateLayer(UpdateLayer { page_id: "page-1".into(), layer_id: "layer-1".into(), name: "Art".into(), visible: false, locked: true });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("layer applies");
    let layer = next.pages.iter().find(|page| page.id == "page-1").unwrap().layers.iter().find(|layer| layer.id == "layer-1").unwrap();
    assert_eq!(layer.name, "Art");
    assert!(!layer.visible);
    assert!(layer.locked);
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse applies");
    let layer = restored.pages.iter().find(|page| page.id == "page-1").unwrap().layers.iter().find(|layer| layer.id == "layer-1").unwrap();
    assert_eq!(layer.name, "Content");
    assert!(layer.visible);
    assert!(!layer.locked);
}
