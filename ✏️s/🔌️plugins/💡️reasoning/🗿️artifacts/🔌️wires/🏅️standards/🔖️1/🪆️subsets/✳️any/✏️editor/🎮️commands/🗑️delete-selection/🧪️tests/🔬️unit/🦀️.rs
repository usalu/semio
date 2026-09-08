
use super::*;
use crate::editor::wires::WiresCommand;
use crate::editor::wires::commands::add_node;
use crate::editor::wires::testkit::{app_with_registry, dispatch, new_app};
use crate::schema::fixture_nodes;
use semio_framework_plugin::{INTERACTION_SELECT_ACTION_ID, InteractionTarget, PluginApp, testkit::meta};
use serde_json::json;

/// 🕹️ `handle`'s macro-only path treats the selection as empty (no `InteractionView` reachable) —
/// nothing gets deleted.
#[semio_framework_async_macros::async_test]
async fn handle_alone_deletes_nothing_without_a_live_selection() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    dispatch(&mut app, WiresCommand::DeleteSelection(DeleteSelection {})).await;
    assert_eq!(fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).len(), 1);
}

/// 🕹️ End-to-end proof the "graph" domain's live selection actually drives `deleteSelection` —
/// spawns a node, selects it via the framework's real `interactionSelect` action (the only way a
/// downstream crate can populate a genuine `InteractionView`), then confirms `deleteSelection`
/// removes exactly that node.
#[semio_framework_async_macros::async_test]
async fn delete_selection_removes_the_live_selected_node() {
    let mut app = app_with_registry().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: "node-1".into() }]).expect("targets");
    app.handle_action(INTERACTION_SELECT_ACTION_ID, semio_framework_plugin::optional_json_to_dsl(Some(json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" }))).as_ref(), &meta("local"))
        .await
        .expect("interactionSelect");
    dispatch(&mut app, WiresCommand::DeleteSelection(DeleteSelection {})).await;
    assert!(fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).is_empty());
}
