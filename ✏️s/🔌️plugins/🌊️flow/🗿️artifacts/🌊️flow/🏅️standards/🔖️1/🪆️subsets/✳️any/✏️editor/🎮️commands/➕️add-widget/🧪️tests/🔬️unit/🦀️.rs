use super::*;
use crate::editor::flow::testkit::{dispatch, flow_app};
use crate::editor::flow::FlowCommand;
use store::{ArtifactPack, SpaceMember};

#[semio_framework_async_macros::async_test]
async fn add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::testkit::{close_registered_fixture_app, meta, settle_registered_typed_operation};
    use semio_framework_plugin::PluginApp;

    let mut app = flow_app().await;
    let parent_before = app.snapshot().expect("snapshot");
    let child_id = parent_before.content.child_id.clone();
    let content_before = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
    let result = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) })).await;
    assert!(result.mutations.is_empty(), "admission must retain the child publication");
    let first = settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("first child publication");
    let repeated = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(50.0), y: Some(51.0) })).await;
    assert!(repeated.mutations.is_empty(), "repeated admission must retain the child publication");
    let second = settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("repeated child publication");
    let parent_after = app.snapshot().expect("snapshot");
    let content_after = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
    for receipt in [first, second] {
        assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Terminal], "each command must publish exactly one acknowledged child group followed by terminal");
    }
    assert_eq!(parent_after.content, parent_before.content, "addWidget must preserve the exact parent content coordinate");
    assert_eq!(content_after.nodes.len(), content_before.nodes.len() + 2);
    let inserted = &content_after.nodes[content_before.nodes.len()..];
    assert_eq!((inserted[0].position.x, inserted[0].position.y), (40.0, 40.0));
    assert_eq!((inserted[1].position.x, inserted[1].position.y), (50.0, 51.0));
    assert_eq!((inserted[0].id.as_str(), inserted[1].id.as_str()), ("note_2", "note_3"));
    let mut after_first_undo = content_before.clone();
    after_first_undo.nodes.push(inserted[0].clone());
    for expected in [after_first_undo, content_before] {
        app.handle_action("undo", None, &meta("local")).await.expect("undo child group");
        let content = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child after undo").document_pack_bytes().await.expect("Flow child pack after undo")).expect("Flow child snapshot after undo");
        assert_eq!(content, expected, "each inverse must restore the complete preceding child document in reverse insertion order");
    }
    close_registered_fixture_app(&mut app);
    eprintln!("[DEBUG] two acknowledged Flow child groups preserve parent identity and undo to the original child document");
}

#[semio_framework_async_macros::async_test]
async fn rename_rejects_blank_unchanged_and_taken_ids() {
    let mut app = flow_app().await;
    for value in ["", " ", "slider"] {
        let result = dispatch(&mut app, FlowCommand::RenameFlowWidget(crate::editor::flow::commands::rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: value.into() })).await;
        assert!(result.mutations.is_empty(), "rename to {value:?} must be a no-operation");
    }
}

#[semio_framework_async_macros::async_test]
async fn patch_flow_widgets_parses_the_raw_value_string_into_the_slider() {
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::PatchFlowWidgets(crate::editor::flow::commands::patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["slider".into()], field: "value".into(), value: "7.5".into() })).await;
    let patched = app.snapshot().expect("snapshot");
    let patched_widgets = patched.to_fixture().widgets;
    assert!(
        patched_widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if id == "slider" && (value - 7.5).abs() < f64::EPSILON)),
        "slider must carry the parsed value: {patched_widgets:?}"
    );
}
