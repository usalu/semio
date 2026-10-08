//! ⚖️ Every verb the flow editor declares honours its declaration — the framework's declared-verb law
//! (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, booted on the demo
//! example with the invocations of `🧫️fixtures/⚖️declared-verb-examples.json`.

use super::*;
use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, DeclaredVerbOutcome};

/// ⚖️ LAW: all 28 declared verbs honour their declarations, probed with the first-party extensions installed (their
/// operators give `add` the ports `connectMediaPorts` wires). The agent lane carries every verb that edits the composed
/// content child (owned-child op groups ride an agent transaction) and diverges from the shell lane on exactly two, pinned
/// so a change turns this law red: `evaluate`, whose shell run requests host effects an agent transaction cannot carry —
/// the fail-closed preview (P9, T6) refuses it `interactive-job.agent-lane-uncarried` by name instead of settling silently —
/// and `focusSelection`, which frames the camera of the main window the agent address does not name.
#[semio_framework_async_macros::async_test]
async fn every_declared_flow_verb_honours_its_declaration() {
    crate::editor::flow::unit_tests::context::install_first_party_light_flow_extensions_for_tests();
    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_flow_app, Some(include_str!("../../🧫️fixtures/⚖️declared-verb-examples.json"))).await;
    assert_eq!(probes.len(), 28, "declared verbs");
    assert_eq!(declared_verb_agent_divergences(&probes), ["evaluate", "focusSelection"], "agent-lane divergences");
    let evaluate = probes.iter().find(|probe| probe.verb == "evaluate").expect("evaluate is a declared verb");
    assert!(evaluate.windows.iter().all(|window| matches!(&window.staged, DeclaredVerbOutcome::Settled(effect) if !effect.is_silent())), "evaluate acts on the shell lane: {:?}", evaluate.windows.iter().map(|window| &window.staged).collect::<Vec<_>>());
    assert!(matches!(&evaluate.agent, Some(DeclaredVerbOutcome::Unreachable { code, .. }) if code == "interactive-job.agent-lane-uncarried"), "the agent lane refuses evaluate by name: {:?}", evaluate.agent);
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
    dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0), label: None, action: None, format: None })).await;
    settle(&mut app).await;
    let added = composed_scene(&app).await;
    let note = added.widgets.iter().map(|widget| crate::schema::widget_id(widget).to_string()).find(|id| id.starts_with("note")).expect("addWidget lands one note in the content child");
    added.retire_cold();
    let rendered = render(&mut app, FLOW_PLAY_BODY_MAIN).await;
    let spine = semio_framework_plugin::artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&rendered).expect("main window scene spine");
    let main_window = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&rendered).expect("assembled main window node-graph scene");
    let projection: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    let surface = semio_framework_plugin::artifact_app_laws::fixture_scene_node::<semio_framework_plugin::NodeGraphScene>(&projection).unwrap();
    let carrier = surface["children"].as_array().unwrap().iter().find(|child| child["key"] == "framework.scene.nodeGraph.nodes").expect("production node carrier");
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_plugin::artifact_app_laws::fixture_carrier_text(carrier)).unwrap();
    assert_eq!(main_window.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), oracle.as_array().unwrap().iter().map(|node| node["id"].as_str().unwrap()).collect::<Vec<_>>(), "assembled scene reads the actual serialized node carrier");
    eprintln!("[DEBUG] Flow current child={} added={} spine_ids={:?} rendered_ids={:?}; serde_json node carrier matched", coordinate, note, spine.nodes.iter().map(|node| &node.id).collect::<Vec<_>>(), main_window.nodes.iter().map(|node| &node.id).collect::<Vec<_>>());
    assert!(main_window.nodes.iter().any(|node| node.id == note), "the main window renders the widget the child edit added");
    dispatch(&mut app, FlowCommand::RemoveWidget(RemoveWidget { widget_id: note.clone() })).await;
    settle(&mut app).await;
    let removed = composed_scene(&app).await;
    assert!(!removed.widgets.iter().any(|widget| crate::schema::widget_id(widget) == note), "removeWidget finds and removes the child-added widget");
    removed.retire_cold();
    assert_eq!(app.snapshot().expect("snapshot").content.child_id, coordinate, "no verb re-points the parent's content coordinate");
    dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(80.0), y: Some(80.0), label: None, action: None, format: None })).await;
    settle(&mut app).await;
}
