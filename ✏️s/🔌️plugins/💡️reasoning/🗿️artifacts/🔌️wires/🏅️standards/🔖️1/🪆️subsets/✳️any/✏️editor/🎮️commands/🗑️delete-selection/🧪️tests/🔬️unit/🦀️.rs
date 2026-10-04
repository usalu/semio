use super::*;
use crate::editor::wires::commands::add_node;
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;
use crate::schema::fixture_nodes;
use semio_framework_plugin::{artifact_app_laws::meta, InteractionTarget, PluginApp, INTERACTION_SELECT_ACTION_ID};

/// 🛑️ With no live "graph" selection the reducer refuses by name instead of answering an empty
/// success. Dispatching is no longer the way to reach that state: the retained route reads the live
/// `InteractionState`, and `addNode` itself selects the node it creates, so the reducer is measured
/// directly on the populated document.
#[semio_framework_async_macros::async_test]
async fn delete_selection_refuses_an_empty_selection_by_name() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    let board = crate::editor::wires::unit_tests::context::board(&app);
    assert_eq!(fixture_nodes(&board).len(), 1);
    let fault = semio_framework_plugin::artifact_app_laws::with_document_view(&mut *app, |view| delete_selected(view, &[])).await.expect("real registered parent view").err().expect("an empty selection is refused");
    assert_eq!(fault.code.0, "wires.selection.empty");
    let unknown = semio_framework_plugin::artifact_app_laws::with_document_view(&mut *app, |view| delete_selected(view, &["node-9".to_string()])).await.expect("real registered parent view").err().expect("a selection naming no live node is refused");
    assert_eq!(unknown.code.0, "wires.selection.empty");
}

/// 🕹️ End-to-end proof the "graph" domain's live selection actually drives `deleteSelection` —
/// spawns a node, selects it via the framework's real `interactionSelect` action (the only way a
/// downstream crate can populate a genuine `InteractionView`), then confirms `deleteSelection`
/// removes exactly that node.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_live_selected_node() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: "node-1".into() }]).expect("targets");
    app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(semio_framework_plugin::dsl_value!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" })).as_ref(), &meta("local"))
        .await
        .expect("interactionSelect");
    dispatch(&mut app, WiresCommand::DeleteSelection(DeleteSelection {})).await;
    assert!(fixture_nodes(&crate::editor::wires::unit_tests::context::board(&app)).is_empty());
}
