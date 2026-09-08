
use super::*;
use crate::editor::flow::testkit::{dispatch, dispatch_with_registry, flow_app_with_registry, select_graph};
use crate::editor::flow::{FLOW_PLAY_BODY_MAIN, FlowCommand};

#[semio_framework_async_macros::async_test]
async fn delete_selection_deletes_the_widgets_picked_via_interaction_select() {
    let mut app = flow_app_with_registry().await;
    select_graph(&mut app, &["slider"], &[]).await;
    let result = dispatch(&mut app, FlowCommand::DeleteSelection(DeleteSelection {})).await;
    assert!(!result.mutations.is_empty(), "deleteSelection must emit operations for a picked widget");
    assert!(!app.snapshot().expect("snapshot").to_fixture().widgets.iter().any(|widget| crate::schema::widget_id(widget) == "slider"), "slider must be deleted");
}

#[semio_framework_async_macros::async_test]
async fn delete_selection_action_removes_selected_synapses() {
    let mut app = flow_app_with_registry().await;
    let before = app.snapshot().expect("snapshot").to_fixture().synapses.len();
    select_graph(&mut app, &[], &["s1"]).await;
    let result = dispatch_with_registry(&mut app, FlowCommand::DeleteSelection(DeleteSelection {})).await;
    let after = app.snapshot().expect("snapshot").to_fixture();
    assert!(!result.mutations.is_empty(), "deleteSelection must emit operations for an edge");
    assert!(!after.synapses.iter().any(|synapse| synapse.id == "s1"), "synapse s1 must be removed");
    assert_eq!(after.synapses.len(), before - 1);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `contextMenuAt` no longer sets
/// selection (framework-owned now) — kept only because the shared `NodeGraph` canvas renderer
/// (framework layer, unmigrated this wave) still dispatches it on right-click; a blank id (or any
/// id) is a genuine no-operation.
#[semio_framework_async_macros::async_test]
async fn context_menu_at_is_a_no_operation() {
    use crate::editor::flow::commands::context_menu_at;
    let mut app = flow_app_with_registry().await;
    let result = dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: String::new() })).await;
    assert!(result.mutations.is_empty());
    assert!(!crate::editor::flow::testkit::render(&mut app, FLOW_PLAY_BODY_MAIN).await.contains(r#""selection":["#));
}
