use super::*;
use crate::editor::flow::testkit::{dispatch, flow_app};
use crate::editor::flow::FlowCommand;
use store::{ArtifactPack, SpaceMember};

#[semio_framework_async_macros::async_test]
async fn add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content() {
    let mut app = flow_app().await;
    let parent_before = app.snapshot().expect("snapshot");
    let child_id = parent_before.content.child_id.clone();
    let content_before = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
    let result = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) })).await;
    let repeated = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(50.0), y: Some(51.0) })).await;
    let parent_after = app.snapshot().expect("snapshot");
    let content_after = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
    assert_eq!(result.mutations.len(), 1, "addWidget must expose one child kernel mutation");
    assert_eq!(repeated.mutations.len(), 1, "a reconstructed host must expose one child kernel mutation for a repeated kind");
    assert_eq!(result.inverse_group.member_edits.len(), 1, "addWidget must dispatch one typed child edit");
    assert_eq!(repeated.inverse_group.member_edits.len(), 1, "a repeated kind must dispatch one typed child edit");
    assert_eq!(parent_after.content, parent_before.content, "addWidget must preserve the exact parent content coordinate");
    assert_eq!(content_after.nodes.len(), content_before.nodes.len() + 2);
    let inserted = &content_after.nodes[content_before.nodes.len()..];
    assert_eq!((inserted[0].position.x, inserted[0].position.y), (40.0, 40.0));
    assert_eq!((inserted[1].position.x, inserted[1].position.y), (50.0, 51.0));
    assert_eq!((inserted[0].id.as_str(), inserted[1].id.as_str()), ("note_2", "note_3"));
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
