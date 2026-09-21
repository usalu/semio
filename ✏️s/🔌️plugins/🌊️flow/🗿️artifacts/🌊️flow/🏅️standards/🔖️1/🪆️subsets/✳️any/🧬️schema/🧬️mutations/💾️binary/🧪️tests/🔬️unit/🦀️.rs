use super::*;
use crate::editor::flow::commands::{duplicate_widget, move_media_node};
use crate::editor::flow::modes::edit::windows::main::FLOW_PLAY_WINDOW_MAIN;
use crate::editor::flow::unit_tests::context::{flow_app_with_registry, FlowApp};
use crate::editor::flow::FlowCommand;
use protocol::Identified;
use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, settle_registered_typed_operation};
use semio_framework_plugin::{ActionMeta, PluginApp, ViewModel, ViewWindowInstance};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use store::{ArtifactPack, SpaceMember};

fn sample_move_widgets_operation() -> FlowMutation {
    FlowMutation::MoveWidgets(crate::schema::mutations::move_widgets::MoveWidgets {
        entries: vec![semio_framework_artifact_flow_flow::FlowLayoutEntry { id: "slider".into(), layout: Some(semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 }) }],
    })
}

fn flow_main_window_meta() -> ActionMeta {
    ActionMeta {
        view_state: Some(ViewModel {
            window_id: Some(FLOW_PLAY_WINDOW_MAIN.into()),
            active_window_kind_id: Some(FLOW_PLAY_WINDOW_MAIN.into()),
            window_instances: vec![ViewWindowInstance { id: FLOW_PLAY_WINDOW_MAIN.into(), window_kind_id: FLOW_PLAY_WINDOW_MAIN.into() }],
            ..Default::default()
        }),
        ..meta("local")
    }
}

async fn settle_flow_command(app: &mut FlowApp, command: FlowCommand) {
    app.dispatch_typed(command, &flow_main_window_meta()).await.expect("real Flow editor command admission");
    settle_registered_typed_operation(app, 1).await.expect("real Flow editor command publication and acknowledgement");
}

async fn drive_document_archive_load(app: &mut FlowApp, operation: u64) -> protocol::DocumentArchiveLoadStatus {
    for _ in 0..1_000_000 {
        let status = Box::pin(PluginApp::poll_document_archive_load(app, operation)).await.expect("Flow document archive status");
        if matches!(status.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            return status;
        }
        let _ = PluginApp::maintenance_step(app, 1, store::OWNED_SCHEMA_DECODE_PAGE_BYTES).expect("Flow document archive maintenance");
        semio_framework_async::yield_once().await;
    }
    panic!("Flow document archive exceeded its public maintenance authority")
}

async fn content_snapshot(app: &FlowApp) -> SemioFlowSnapshot {
    let parent = app.snapshot().expect("Flow parent snapshot");
    SemioFlowSnapshot::decode_pack(
        &app.child_store("content", &parent.content.child_id)
            .await
            .expect("Flow content child")
            .document_pack_bytes()
            .await
            .expect("Flow content child pack"),
    )
    .expect("Flow content child snapshot")
}

async fn assert_document_archive_round_trip(source: &mut FlowApp, operation: u64) {
    let expected_parent = source.snapshot().expect("source Flow parent snapshot");
    let expected_content = Box::pin(content_snapshot(source)).await;
    let expected = Box::pin(PluginApp::document_archive(&*source)).await.expect("source Flow document archive");
    assert_eq!(expected.members.len(), 1, "Flow archive must carry its exact content child closure");
    let member = &expected.members[0];
    assert_eq!(member.ordinal, 0);
    assert_eq!(member.owner.slot, "content");
    assert_eq!(member.owner.child_id, expected_parent.content.child_id);
    assert_eq!(member.reference.artifact_id, expected_parent.content.child_id);

    let mut restored = flow_app_with_registry().await;
    PluginApp::begin_document_archive_load(&mut *restored, operation, expected.clone()).expect("Flow document archive admission");
    let status = Box::pin(drive_document_archive_load(&mut *restored, operation)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "Flow composed archive load fault: {:?}", status.fault);
    PluginApp::acknowledge_document_archive_load(&mut *restored, operation).expect("Flow document archive acknowledgement");
    assert_eq!(restored.snapshot().expect("restored Flow parent snapshot"), expected_parent);
    assert_eq!(Box::pin(content_snapshot(&restored)).await, expected_content);
    assert_eq!(Box::pin(PluginApp::document_archive(&*restored)).await.expect("restored Flow document archive"), expected);
    close_registered_fixture_app(&mut *restored);

    let mut missing_member = expected.clone();
    missing_member.members.clear();
    let mut refused = flow_app_with_registry().await;
    let retained = Box::pin(PluginApp::document_archive(&*refused)).await.expect("pre-refusal Flow document archive");
    let refusal_operation = operation.checked_add(1).expect("archive operation id");
    PluginApp::begin_document_archive_load(&mut *refused, refusal_operation, missing_member).expect("missing-member Flow archive admission");
    let status = Box::pin(drive_document_archive_load(&mut *refused, refusal_operation)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Fault, "a parent that references an absent content member must be refused");
    PluginApp::acknowledge_document_archive_load(&mut *refused, refusal_operation).expect("missing-member Flow archive acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(&*refused)).await.expect("Flow archive after refused replacement"), retained);
    close_registered_fixture_app(&mut *refused);
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = sample_move_widgets_operation();
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn flow_document_text_round_trips_store_with_applied_operation() {
    let mut app = flow_app_with_registry().await;
    let content_id = app.snapshot().expect("Flow parent before move").content.child_id.clone();
    Box::pin(settle_flow_command(
        &mut *app,
        FlowCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "slider".into(), x: 1.0, y: 2.0 }),
    ))
    .await;
    let snapshot = app.snapshot().expect("moved Flow parent snapshot");
    assert_eq!(snapshot.content.child_id, content_id, "move must retain the exact composed content member");
    let content = Box::pin(content_snapshot(&app)).await;
    let moved = content.nodes.iter().find(|node| node.id == "slider").expect("moved slider content node");
    assert_eq!((moved.position.x, moved.position.y), (1.0, 2.0));
    Box::pin(assert_document_archive_round_trip(&mut *app, 71)).await;
    close_registered_fixture_app(&mut *app);
}

/// 🌉️ The composite pilot's own op text/binary law, plus proof it survives the REAL
/// `ArtifactStore::dispatch` path — `replay_mutations` calls `Op::encode_op()` on every applied
/// mutation before it even reaches history, so a composite whose codec only worked in isolation
/// would still fail here.
#[semio_framework_async_macros::async_test]
async fn duplicate_widget_composite_round_trips_through_op_codecs_and_a_real_store_dispatch() {
    let duplicate = FlowMutation::DuplicateWidget(crate::schema::mutations::duplicate_widget::mutation::DuplicateWidget {
        source_id: "note-1".into(),
        new_id: "note-2".into(),
        synapse_id: "note-1-to-note-2".into(),
        from_port: "out".into(),
        to_port: "in".into(),
    });
    store::os_store::test_support::assert_op_text_binary_equivalence(&duplicate);
    let bytes = encode_op(&duplicate).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), duplicate);

    let mut app = flow_app_with_registry().await;
    Box::pin(settle_flow_command(&mut *app, FlowCommand::DuplicateWidget(duplicate_widget::DuplicateWidget { widget_id: "slider".into() }))).await;
    let content = Box::pin(content_snapshot(&app)).await;
    assert!(content.nodes.iter().any(|node| node.id == "slider-copy"));
    assert!(content.edges.iter().any(|edge| edge.id == "slider-to-slider-copy"));
    Box::pin(assert_document_archive_round_trip(&mut *app, 81)).await;
    close_registered_fixture_app(&mut *app);
}
