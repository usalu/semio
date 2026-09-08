
use super::*;
use crate::{GENERATION_3D_SCHEMA, Generation3dSnapshot};
use semio_framework_artifact_flow_flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use semio_framework_os_kernel::os_store::test_support;
use store::{ArtifactCommand, create_document_envelope};

#[test]
fn op_text_round_trip_create_widget() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::CreateWidget(CreateWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } }));
}

#[test]
fn op_text_round_trip_delete_widget() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::DeleteWidget(DeleteWidget { id: "note-9".into() }));
}

#[test]
fn op_text_round_trip_connect_synapse() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::ConnectSynapse(ConnectSynapse {
        index: 1,
        synapse: SynapseSpec { id: "e1".into(), from: "height".into(), to: "extrude".into(), from_port: "number".into(), to_port: String::new() },
    }));
}

#[test]
fn op_text_round_trip_disconnect_synapse() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "e1".into() }));
}

#[test]
fn op_text_round_trip_move_widget() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } }));
}

#[test]
fn op_text_round_trip_delete_widget_position() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "extrude".into() }));
}

#[test]
fn op_text_round_trip_update_camera() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } }));
}

#[test]
fn op_text_round_trip_change_schema() {
    test_support::assert_op_line_round_trip(&Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture".into() }));
}

#[test]
fn op_text_round_trip_create_generation() {
    let generation = semio_framework_artifact_playbook_playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: std::collections::HashMap::new() };
    test_support::assert_op_line_round_trip(&Generation3dMutation::CreateGeneration(CreateGeneration { generation }));
}

#[test]
fn op_text_parse_rejects_unknown_operation() {
    let error = <Generation3dMutation as protocol::OpText>::parse_op("bogus-op id=\"w-1\"").expect_err("unknown operation must fail to parse");
    assert!(error.to_string().contains("unknown operation"), "unexpected error: {error}");
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_with_operation_applied() {
    let mut store = store::ArtifactStore::<Generation3dSnapshot, Generation3dMutation>::new(create_document_envelope(GENERATION_3D_SCHEMA, "generation3d", Generation3dSnapshot::default(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![Generation3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], description: None }).await.expect("apply");
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
}
