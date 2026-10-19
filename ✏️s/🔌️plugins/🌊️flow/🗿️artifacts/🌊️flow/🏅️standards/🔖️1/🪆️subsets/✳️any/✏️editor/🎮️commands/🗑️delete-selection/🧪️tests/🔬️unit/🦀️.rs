#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use crate::editor::flow::edit_rules::ContentEdit;
#[allow(unused_imports)]
use crate::editor::flow::host::{flow_composed_content, flow_content_leaves_emit};
#[allow(unused_imports)]
use crate::editor::flow::editor_vocabulary::{flow_graph_selection_domains, FLOW_INTERACTION_GRAPH};
#[allow(unused_imports)]
use crate::{FlowMutation, FlowSnapshot};
#[allow(unused_imports)]
use flow::FlowEvalSession;
#[allow(unused_imports)]
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::flow::commands::delete_selection::*;
use crate::editor::flow::unit_tests::context::{composed_scene, dispatch, dispatch_with_registry, flow_app_with_registry, select_graph, settle};
use crate::editor::flow::{FlowCommand, FLOW_PLAY_BODY_MAIN};

#[semio_framework_async_macros::async_test]
async fn delete_selection_deletes_the_widgets_picked_via_interaction_select() {
    let mut app = flow_app_with_registry().await;
    select_graph(&mut app, &["slider"], &[]).await;
    let result = dispatch(&mut app, FlowCommand::DeleteSelection(DeleteSelection {})).await;
    assert!(result.mutations.is_empty(), "retained deleteSelection admission cannot publish document operations synchronously");
    settle(&mut app).await;
    let after = composed_scene(&app).await;
    assert!(!after.widgets.iter().any(|widget| crate::schema::widget_id(widget) == "slider"), "slider must be deleted");
    after.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn delete_selection_action_removes_selected_synapses() {
    let mut app = flow_app_with_registry().await;
    let before_scene = composed_scene(&app).await;
    let before = before_scene.synapses.len();
    before_scene.retire_cold();
    select_graph(&mut app, &[], &["s1"]).await;
    let result = dispatch_with_registry(&mut app, FlowCommand::DeleteSelection(DeleteSelection {})).await;
    assert!(result.mutations.is_empty(), "retained deleteSelection admission cannot publish document operations synchronously");
    settle(&mut app).await;
    let after = composed_scene(&app).await;
    assert!(!after.synapses.iter().any(|synapse| synapse.id == "s1"), "synapse s1 must be removed");
    assert_eq!(after.synapses.len(), before - 1);
    after.retire_cold();
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
    settle(&mut app).await;
    assert!(!crate::editor::flow::unit_tests::context::render(&mut app, FLOW_PLAY_BODY_MAIN).await.contains(r#""selection":["#));
}
