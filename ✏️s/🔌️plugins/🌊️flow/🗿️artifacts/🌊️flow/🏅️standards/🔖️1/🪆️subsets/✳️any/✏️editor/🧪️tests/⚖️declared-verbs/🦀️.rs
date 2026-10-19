//! ⚖️ Every verb the flow editor declares honours its declaration — the framework's declared-verb law
//! (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, booted on the demo
//! example with the invocations of `🧫️fixtures/⚖️declared-verb-examples.json`.

use crate::editor::flow::commands::{add_widget, evaluate, remove_widget};
use semio_framework_test_fixtures::fixture_text;
use serde_json::Value;
#[allow(unused_imports)]
use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
#[allow(unused_imports)]
use crate::editor::flow::commands::{connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport, open_spotlight, patch_flow_widgets, rename_flow_widget, reorganize, replace_image, run_extension_action, set_active_example, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode, set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension};
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
#[allow(unused_imports)]
use semio_framework_plugin::{NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::{compiled, main};
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
#[allow(unused_imports)]
use crate::editor::flow::modes::{edit, generate};
#[allow(unused_imports)]
use crate::editor::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
#[allow(unused_imports)]
use crate::editor::flow::presence::{FlowPresence, FlowPresenceMutation};
#[allow(unused_imports)]
use crate::editor::flow::terminology::{flow_play_labels, FlowPlayLabels};
#[allow(unused_imports)]
use crate::editor::flow::editor_vocabulary::{flow_graph_edge_target_id, flow_graph_node_target_id, flow_graph_selection_domains, FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_INTERACTION_GRAPH, FLOW_PLAY_APP_ID, FLOW_PLAY_WINDOW_MAIN, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_SCENE_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
#[allow(unused_imports)]
use crate::editor::flow::editor_vocabulary::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
#[allow(unused_imports)]
use crate::editor::flow::host::{cold_grant, flow_content_leaves_emit, flow_removal_leaves, focus_selection_camera, seed_host_catalogue, value_fault};
#[allow(unused_imports)]
use crate::editor::flow::host::{host_from_snapshot, sync_host_selection_domains};
#[allow(unused_imports)]
use crate::{FlowMutation, FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
#[allow(unused_imports)]
use semio_framework_artifact_flow_flow::{CameraJson, Widget};
#[allow(unused_imports)]
use semio_framework_artifact_flow_flow::{SynapseSpec, WidgetLayout};
#[allow(unused_imports)]
use semio_framework_os_infinite_directed_presentation::DagDrawLod;
#[allow(unused_imports)]
use semio_framework_plugin::app::{ChildEmitPreparation, InteractionView};
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::AppActionRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::AppDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::CommandDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuItemSpec;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuRequest;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DomainTopology;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::Effect;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::GranularityDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTopology;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MergeMode;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_framework_plugin::TopologyNode;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use serde_json::json;
#[allow(unused_imports)]
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, PortRef, SemioFlowSnapshot};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowNode;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::flow::*;
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
    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_flow_app, Some(fixture_text!("../../../🧫️fixtures/⚖️declared-verb-examples.json")), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
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
