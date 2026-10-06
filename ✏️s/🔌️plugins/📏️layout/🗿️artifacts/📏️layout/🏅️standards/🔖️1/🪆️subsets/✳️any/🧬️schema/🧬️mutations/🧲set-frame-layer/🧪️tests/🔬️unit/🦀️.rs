use super::*;
use crate::mutations::create_layer::CreateLayer;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn set_frame_layer_moves_a_frame_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let created = LayoutMutation::CreateLayer(CreateLayer { page_id: "page-1".into(), id: "layer-2".into(), name: "Notes".into(), remove: false }).diff(&base).diff().apply(&base).expect("layer");
    let mutation = LayoutMutation::SetFrameLayer(SetFrameLayer { page_id: "page-1".into(), frame_id: "frame-1".into(), layer_id: "layer-2".into() });
    let next = mutation.diff(&created).diff().apply(&created).expect("move");
    let frame = next.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
    assert_eq!(frame.layer_id(), "layer-2");
    assert!(next.pages[0].layers.iter().find(|layer| layer.id == "layer-2").unwrap().object_ids.iter().any(|id| id == "frame-1"));
    assert!(next.pages[0].layers.iter().find(|layer| layer.id == "layer-1").unwrap().object_ids.iter().all(|id| id != "frame-1"));
    let restored = mutation.inverse(&created).expect("valid retained mutation inverse fixture")[0].diff(&next).diff().apply(&next).expect("inverse");
    assert_eq!(restored.pages[0].frames.iter().find(|frame| frame.id() == "frame-1").unwrap().layer_id(), "layer-1");
}
