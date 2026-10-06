use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn create_layer_appends_an_empty_layer_and_inverse_removes_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::CreateLayer(CreateLayer { page_id: "page-1".into(), id: "layer-2".into(), name: "Notes".into(), remove: false });
    let next = mutation.diff(&base).diff().apply(&base).expect("layer");
    assert_eq!(next.pages[0].layers.last().unwrap().name, "Notes");
    let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff().apply(&next).expect("inverse");
    assert_eq!(restored.pages[0].layers.len(), 1);
}
