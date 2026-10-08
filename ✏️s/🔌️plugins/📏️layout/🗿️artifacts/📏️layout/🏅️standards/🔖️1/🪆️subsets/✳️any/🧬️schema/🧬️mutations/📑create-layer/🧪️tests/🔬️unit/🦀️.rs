use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn create_layer_appends_an_empty_layer_and_inverse_removes_it() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    let mutation = LayoutMutation::CreateLayer(CreateLayer { page_id: "page-1".into(), id: "layer-2".into(), name: "Notes".into(), remove: false, index: None });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("layer");
    assert_eq!(next.pages[0].layers.last().unwrap().name, "Notes");
    let restored = protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse");
    assert_eq!(restored.pages[0].layers.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn removing_a_locked_middle_layer_restores_its_position_and_flags() {
    let mut base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    for (id, locked) in [("layer-mid", true), ("layer-top", false)] {
        base.pages[0].layer_ids.push(id.into());
        base.pages[0].layers.push(crate::Layer { id: id.into(), name: id.into(), visible: true, locked, object_ids: Vec::new() });
    }
    let mutation = LayoutMutation::CreateLayer(CreateLayer { page_id: "page-1".into(), id: "layer-mid".into(), name: "layer-mid".into(), remove: true, index: None });
    let removed = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("remove");
    let mut restored = removed;
    for step in mutation.inverse(&base).expect("valid retained mutation inverse fixture") {
        restored = protocol::apply_diff(step.diff(&restored).diff(), &restored).expect("inverse step");
    }
    assert_eq!(restored, base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
