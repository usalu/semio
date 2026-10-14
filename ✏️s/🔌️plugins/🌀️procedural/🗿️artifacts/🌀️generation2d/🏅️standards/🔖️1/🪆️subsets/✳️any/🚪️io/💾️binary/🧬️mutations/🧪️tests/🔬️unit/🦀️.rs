use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use crate::mutations::*;
use semio_s_artifact_procedural_2d_mut_document::change_schema;
use semio_s_artifact_procedural_2d_mut_synapses::connect_synapse;
use semio_s_artifact_procedural_2d_mut_generations::create_generation;
use semio_s_artifact_procedural_2d_mut_widgets::create_widget;
use semio_s_artifact_procedural_2d_mut_widgets::delete_widget;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::OpText;
use semio_framework_artifact_flow_flow::{SynapseSpec, Widget};
use semio_framework_os_kernel::os_store::test_support;
use store::os_store::store_schema::ArtifactCommand;

//#region 🔖️OpTextTests
#[test]
fn op_text_round_trip_create_widget() {
    test_support::assert_op_line_round_trip(&create_widget(2, Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() }));
}

#[test]
fn op_text_round_trip_delete_widget() {
    test_support::assert_op_line_round_trip(&delete_widget("note-9".into()));
}

#[test]
fn op_text_round_trip_connect_synapse() {
    test_support::assert_op_line_round_trip(&connect_synapse(1, SynapseSpec { id: "s1".into(), from: "rect".into(), to: "fill".into(), from_port: "draw.drawing".into(), to_port: String::new() }));
}

#[test]
fn op_text_round_trip_change_schema() {
    test_support::assert_op_line_round_trip(&change_schema("flow.host_snapshot".into()));
}

#[test]
fn op_text_round_trip_create_generation() {
    let generation = semio_framework_artifact_playbook_playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() };
    test_support::assert_op_line_round_trip(&create_generation(generation.clone()));
    test_support::assert_op_line_round_trip(&Generation2dMutation::CreateGeneration(semio_s_artifact_procedural_2d_mut_generations::create_generation::CreateGeneration { generation, index: Some(2) }));
}

#[test]
fn op_binary_round_trip_create_generation_keeps_its_index_and_select_generation() {
    let generation = semio_framework_artifact_playbook_playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() };
    let mut cases = vec![
        semio_s_artifact_procedural_2d_mut_generations::select_generation(Some("generation-1".into())),
        semio_s_artifact_procedural_2d_mut_generations::select_generation(None),
    ];
    for index in [None, Some(0), Some(3)] {
        cases.push(Generation2dMutation::CreateGeneration(semio_s_artifact_procedural_2d_mut_generations::create_generation::CreateGeneration { generation: generation.clone(), index }));
    }
    for mutation in cases {
        assert_eq!(decode_op(&encode_op(&mutation).expect("encode")).expect("decode"), mutation);
        test_support::assert_op_line_round_trip(&mutation);
    }
}
//#endregion 🔖️OpTextTests

//#region 🔖️OpTextErrorTests
#[test]
fn op_text_parse_rejects_unknown_operation() {
    let error = Generation2dMutation::parse_op("bogus-op id=\"x\"").unwrap_err();
    assert!(error.message.contains("unknown operation"), "unexpected error: {}", error.message);
}

#[test]
fn op_text_parse_rejects_non_integer_index() {
    let error = Generation2dMutation::parse_op("create-widget index=abc note text=\"\" id=\"x\"").unwrap_err();
    assert!(error.message.contains("expected Int"), "unexpected error: {}", error.message);
}
//#endregion 🔖️OpTextErrorTests

#[test]
fn op_binary_round_trips_via_wrapper_fns() {
    let operation = change_schema("flow.host_snapshot".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_with_operation_applied() {
    let mut store = semio_s_artifact_procedural_2d_testkit::store_fixture::document_store(Generation2dSnapshot::default()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![create_widget(3, Widget::InputNote { id: "note-9".into(), text: String::new() })], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
    semio_s_artifact_procedural_2d_testkit::store_fixture::close(store);
}
