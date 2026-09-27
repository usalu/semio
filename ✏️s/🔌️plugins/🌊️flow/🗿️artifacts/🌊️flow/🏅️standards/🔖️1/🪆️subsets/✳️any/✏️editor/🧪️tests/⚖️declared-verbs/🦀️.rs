//! ⚖️ Every verb the flow editor declares honours its declaration — the framework's declared-verb law
//! (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, booted on the demo
//! example with the invocations of `🧫️fixtures/⚖️declared-verb-examples.json`.

use super::*;
use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences};

/// ⚖️ LAW: all 28 declared verbs honour their declarations, probed with the first-party extensions installed (their
/// operators give `add` the ports `connectMediaPorts` wires). The agent lane refuses by name every verb that edits the
/// composed content child — an agent transaction carries parent operations only, so a child group is
/// `interactive-job.agent-lane-uncarried` until the MCP gateway commits owned children (routed to G10, ticket
/// 26/09/23 `wp-p8.md` § routed) — `evaluate`, whose host-only job the agent lane cannot preview, and
/// `focusSelection`, which frames the camera of the main window the agent address does not name. The list is pinned so
/// a carrier that lands turns this law red until the pin is removed.
#[semio_framework_async_macros::async_test]
async fn every_declared_flow_verb_honours_its_declaration() {
    crate::editor::flow::unit_tests::context::install_first_party_light_flow_extensions_for_tests();
    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_flow_app, Some(include_str!("../../🧫️fixtures/⚖️declared-verb-examples.json"))).await;
    assert_eq!(probes.len(), 28, "declared verbs");
    assert_eq!(
        declared_verb_agent_divergences(&probes),
        ["addWidget", "removeWidget", "duplicateWidget", "disconnect", "connectMediaPorts", "moveMediaNode", "reorganize", "patchFlowWidgets", "renameFlowWidget", "setActiveExample", "evaluate", "focusSelection"],
        "agent-lane divergences"
    );
}

/// ⚖️ LAW: the content child is the one scene. After a child edit (`addWidget`) the window renders the new widget and
/// a parent-routed verb (`removeWidget`) finds it; after that verb the content child still exists at the same
/// coordinate, so the next child edit lands instead of being refused before any capacity is measured.
#[semio_framework_async_macros::async_test]
async fn the_content_child_is_the_one_scene_every_verb_and_window_reads() {
    use crate::editor::flow::unit_tests::context::{composed_scene, dispatch, flow_app, render, settle};
    use crate::editor::flow::commands::{add_widget::AddWidget, remove_widget::RemoveWidget};
    let mut app = flow_app().await;
    let coordinate = app.snapshot().expect("snapshot").content.child_id.clone();
    dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) })).await;
    settle(&mut app).await;
    let added = composed_scene(&app).await;
    let note = added.widgets.iter().map(|widget| crate::schema::widget_id(widget).to_string()).find(|id| id.starts_with("note")).expect("addWidget lands one note in the content child");
    added.retire_cold();
    let main_window = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&render(&mut app, FLOW_PLAY_BODY_MAIN).await).expect("main window node-graph scene");
    assert!(main_window.nodes.iter().any(|node| node.id == note), "the main window renders the widget the child edit added");
    dispatch(&mut app, FlowCommand::RemoveWidget(RemoveWidget { widget_id: note.clone() })).await;
    settle(&mut app).await;
    let removed = composed_scene(&app).await;
    assert!(!removed.widgets.iter().any(|widget| crate::schema::widget_id(widget) == note), "removeWidget finds and removes the child-added widget");
    removed.retire_cold();
    assert_eq!(app.snapshot().expect("snapshot").content.child_id, coordinate, "no verb re-points the parent's content coordinate");
    dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(80.0), y: Some(80.0) })).await;
    settle(&mut app).await;
}
