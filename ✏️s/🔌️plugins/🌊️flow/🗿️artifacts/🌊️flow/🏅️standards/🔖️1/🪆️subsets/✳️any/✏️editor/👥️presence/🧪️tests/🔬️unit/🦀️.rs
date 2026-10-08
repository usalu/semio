use super::*;
use protocol::os_spr::protocol_laws::{assert_mutation_inverse_sum_law};
use protocol::{DiffAlgebra, Mutation};

fn busy() -> FlowPresence {
    FlowPresence { preview_off_node_ids: vec!["a".into(), "b".into(), "c".into()], camera: CameraJson { x: 3.0, y: -2.0, zoom: 2.0 } }
}

#[semio_framework_async_macros::async_test]
async fn a_set_names_only_the_slot_it_owns_and_an_unchanged_value_is_an_empty_diff() {
    let base = busy();
    let outcome = FlowPresenceMutation::SetCamera(CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }).diff(&base);
    assert_eq!(outcome.diff(), &FlowPresenceDiff { camera: Some(CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }), ..Default::default() });
    assert!(FlowPresenceMutation::SetPreviewOffNodeIds(base.preview_off_node_ids.clone()).diff(&base).diff().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn every_set_obeys_the_inverse_sum_law_including_removing_a_middle_row() {
    for base in [busy(), FlowPresence::default()] {
        assert_mutation_inverse_sum_law(&FlowPresenceMutation::SetPreviewOffNodeIds(vec!["a".into(), "c".into()]), &base).await;
        assert_mutation_inverse_sum_law(&FlowPresenceMutation::SetPreviewOffNodeIds(Vec::new()), &base).await;
        assert_mutation_inverse_sum_law(&FlowPresenceMutation::SetCamera(CameraJson { x: 9.0, y: 9.0, zoom: 4.0 }), &base).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn presence_and_operations_round_trip_text_and_binary() {
    store::os_store::test_support::assert_dsl_round_trip(&busy());
    store::os_store::test_support::assert_dsl_pack_equivalence(&busy());
    store::os_store::test_support::assert_op_line_round_trip(&FlowPresenceMutation::SetPreviewOffNodeIds(vec!["a".into()]));
    store::os_store::test_support::assert_op_line_round_trip(&FlowPresenceMutation::SetCamera(CameraJson { x: 1.0, y: 2.0, zoom: 3.0 }));
}
