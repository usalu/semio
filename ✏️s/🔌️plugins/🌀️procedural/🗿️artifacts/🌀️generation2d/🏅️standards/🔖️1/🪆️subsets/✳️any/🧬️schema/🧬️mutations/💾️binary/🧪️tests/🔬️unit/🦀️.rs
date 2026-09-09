use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{change_schema, connect_synapse, create_generation, create_widget, delete_widget};
use crate::Generation2dSnapshot;
use protocol::OpText;
use semio_framework_artifact_flow_flow::{SynapseSpec, Widget};
use semio_framework_os_kernel::os_store::test_support;
use store::ArtifactCommand;

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
    test_support::assert_op_line_round_trip(&change_schema("flow.fixture".into()));
}

#[test]
fn op_text_round_trip_create_generation() {
    let generation = semio_framework_artifact_playbook_playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: Default::default() };
    test_support::assert_op_line_round_trip(&create_generation(generation));
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
    let operation = change_schema("flow.fixture".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_with_operation_applied() {
    let mut store = crate::store_fixture::document_store(Generation2dSnapshot::default()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![create_widget(3, Widget::InputNote { id: "note-9".into(), text: String::new() })], description: None }).await.expect("apply");
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
    crate::store_fixture::close(store);
}
