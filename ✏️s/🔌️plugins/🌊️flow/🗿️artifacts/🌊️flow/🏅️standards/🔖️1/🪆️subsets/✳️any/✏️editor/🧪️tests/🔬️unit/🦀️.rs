use crate::editor::flow::commands::{add_widget, connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, evaluate, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport, open_spotlight, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_active_example, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode, set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::NoConfig;
use crate::editor::flow::modes::edit::windows::compiled;
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use crate::editor::flow::modes::{edit, generate};
use crate::editor::flow::base::{flow_graph_edge_target_id, flow_graph_node_target_id, FLOW_INTERACTION_GRAPH, FLOW_PLAY_WINDOW_MAIN};
use crate::editor::flow::base::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
use crate::editor::flow::host::flow_removal_leaves;
use crate::editor::flow::host::{host_from_snapshot, sync_host_selection_domains};
use crate::{FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
use flow::FlowEvalSession;
use semio_framework_artifact_flow_flow::{CameraJson, Widget};
use semio_framework_artifact_flow_flow::{SynapseSpec, WidgetLayout};
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuRequest;
use semio_framework_plugin::Fault;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::WindowMeasure;
use semio_framework_test_fixtures::fixture_text;
use serde_json::json;
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use serde_json::Value;
#[allow(unused_imports)]
use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main;
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
#[allow(unused_imports)]
use crate::editor::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
#[allow(unused_imports)]
use crate::editor::flow::presence::{FlowPresence, FlowPresenceMutation};
#[allow(unused_imports)]
use crate::editor::flow::terminology::{flow_play_labels, FlowPlayLabels};
#[allow(unused_imports)]
use crate::editor::flow::base::{flow_graph_selection_domains, FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_PLAY_APP_ID, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_SCENE_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
#[allow(unused_imports)]
use crate::editor::flow::host::{cold_grant, flow_content_leaves_emit, focus_selection_camera, seed_host_catalogue, value_fault};
#[allow(unused_imports)]
use crate::FlowMutation;
#[allow(unused_imports)]
use flow::{flow_host_with_session, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
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
use semio_framework_plugin::AppDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::CommandDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuItemSpec;
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
use semio_framework_plugin::GranularityDefinition;
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
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::mutations::{insert_edge, insert_node};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, PortRef};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowNode;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::flow::{FLOW_DIRECT_STORE_TOOL_IDS, FLOW_CHILD_GROUP_TOOL_IDS, FlowChildGroupJobFactory, FLOW_HOST_ONLY_TOOL_IDS, FLOW_GRAPH_OPERATION_TOOL_IDS, FLOW_CONTRIBUTIONS_TOOL_IDS, retained};

use crate::editor::flow::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, flow_app_with_registry, settle, FlowApp};
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{EditorApp, PluginApp};

#[test]
fn retained_content_child_factory_is_exact_and_legacy_closed() {
    let factory = FlowChildGroupJobFactory::new("s.flow.flow@1/*#editor");
    let expected = ["addWidget", "moveMediaNode", "nodeGraphEdit", "spotlightCommit"];
    let keys = semio_framework::ToolJobFactory::keys(&factory);
    assert_eq!(
        keys,
        expected.map(|tool_id| semio_framework::ToolFactoryKey::new("s.flow.flow@1/*#editor", tool_id)).as_slice()
    );
    assert_eq!(semio_framework::ToolJobFactory::payload_schema_id(&factory), FLOW_DOCUMENT_SCHEMA);
    assert_eq!(semio_framework::ToolJobFactory::classification(&factory), semio_framework_plugin::InteractiveJobClassification::Migrated);
    assert_eq!(semio_framework::ToolJobFactory::execution_contract(&factory), semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),);
    assert_eq!(<FlowChildGroupJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::TOOL_IDS, &expected);
    let publication = <FlowChildGroupJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(publication.len(), expected.len());
    for (contract, tool_id) in publication.iter().zip(expected) {
        assert_eq!(contract.tool_id, tool_id);
        assert_eq!(contract.lanes, &[semio_framework_plugin::ArtifactToolPublicationLane::Child]);
    }
    let proofs = <FlowPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs();
    assert_eq!(proofs.len(), FLOW_DIRECT_STORE_TOOL_IDS.len() + FLOW_HOST_ONLY_TOOL_IDS.len() + FLOW_CHILD_GROUP_TOOL_IDS.len() + FLOW_GRAPH_OPERATION_TOOL_IDS.len() + FLOW_CONTRIBUTIONS_TOOL_IDS.len());
    assert_eq!(FLOW_CHILD_GROUP_TOOL_IDS, &expected);
}

#[semio_framework_async_macros::async_test]
async fn retained_add_widget_dispatches_one_acknowledged_child_group_and_retires() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    use store::{pack::snapshot::ArtifactPack, SpaceMember};

    let fixture: Value = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧵️add-widget-retained/🔣️.json")).expect("retained addWidget fixture");
    let mut app = flow_app_with_registry().await;
    PluginApp::bind_instance_id(&mut *app, 1).await;
    let parent_before = app.snapshot().expect("Flow parent before retained addWidget");
    let child_id = parent_before.content.child_id.clone();
    let child_before = semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot::decode_pack(
        &app.child_store("content", &child_id).await.expect("Flow child before retained addWidget").document_pack_bytes().await.expect("Flow child pack before retained addWidget"),
    )
    .expect("decode Flow child before retained addWidget");
    let accepted = &fixture["accepted"][0]["command"];
    let started = dispatch(
        &mut app,
        FlowCommand::AddWidget(add_widget::AddWidget { kind: accepted["kind"].as_str().expect("kind").to_string(), neuron_kind: accepted["neuronKind"].as_str().map(str::to_string), x: accepted["x"].as_f64(), y: accepted["y"].as_f64(), label: None, action: None, format: None }),
    )
    .await;
    assert!(started.mutations.is_empty(), "retained addWidget must not publish through its immediate invocation result");
    let lanes = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("retained addWidget publication and exact ACK").lanes;
    assert_eq!(
        lanes,
        [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal],
        "one acknowledged child group, the emit's own UI scope, then terminal — the framework publishes a Ui page for every successful emit (`ui_pending`, `🔌️plugin/🦀️.rs:27709`), which its own contract fixture declares as `artifactPublications: 1, uiPublications: 1, terminalReceipts: 1` (`🔌️plugin/🧫️fixtures/⏳️completion/🔣️.json:5`)"
    );
    assert!(!PluginApp::has_pending_typed_operations(&*app));
    let parent_after = app.snapshot().expect("Flow parent after retained addWidget");
    assert_eq!(parent_after.content, parent_before.content, "retained addWidget must preserve the exact parent content coordinate");
    let child_after = semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot::decode_pack(
        &app.child_store("content", &child_id).await.expect("Flow child after retained addWidget").document_pack_bytes().await.expect("Flow child pack after retained addWidget"),
    )
    .expect("decode Flow child after retained addWidget");
    assert_eq!(child_after.nodes.len(), child_before.nodes.len() + 1);
    assert_eq!(child_after.edges, child_before.edges);
    let expected = &fixture["accepted"][0]["expected"];
    let inserted_expected = &fixture["accepted"][0]["inserted"];
    let inserted = child_after.nodes.last().expect("retained addWidget inserted node");
    assert_eq!(inserted.kind, inserted_expected["kind"].as_str().expect("inserted kind"));
    assert_eq!(inserted.position.x, inserted_expected["x"].as_f64().expect("inserted x"));
    assert_eq!(inserted.position.y, inserted_expected["y"].as_f64().expect("inserted y"));
    assert_eq!(expected["parentMutations"], 0);
    assert_eq!(expected["childGroups"], 1);
    for _ in 0..100_000 {
        let demand = PluginApp::close_retirement_demands(&*app, 16_384).expect("Flow app close quote");
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(16_384), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        if matches!(PluginApp::close_step(&mut *app, grant).expect("retained addWidget app close"), semio_framework_plugin::PluginLifecycleStep::Complete(_)) {
            break;
        }
    }
    assert!(PluginApp::close_terminal_is_empty(&*app));
}

/// ↩️ LAW (design §20.15): deleting a node is child-lane `remove-edge` leaves for every edge touching it (child order) and
/// then its `remove-node`; undoing them through their own inverses restores every node and edge by id, large authored
/// content included.
#[test]
fn a_node_delete_is_child_removal_leaves_whose_inverses_restore_the_content() {
    #[derive(semio_framework_value::FromValue)]
    struct Scene {
        widgets: Vec<Widget>,
        synapses: Vec<SynapseSpec>,
        layout: flow::OrderedMap<WidgetLayout>,
    }
    let fixture: Value = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧹️delete-cascade/🔣️.json")).unwrap();
    let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
    assert_eq!(label.len(), fixture["label"]["expectedBytes"].as_u64().unwrap() as usize);
    let mut scene = fixture["scene"].clone();
    scene["widgets"][1]["label"] = Value::String(label.clone());
    let Scene { widgets, synapses, layout } = semio_framework_value::FromValue::from_value(scene.into()).unwrap();
    let working = FlowWorkingScene { widgets, synapses, layout };
    let base = crate::flow_content_snapshot_from_working(&working.widgets, &working.synapses, &working.layout);
    drop(working);
    let target = fixture["targetId"].as_str().unwrap().to_string();
    let leaves = flow_removal_leaves(&base, std::slice::from_ref(&target), &[]);
    let named: Vec<Value> = leaves
        .iter()
        .map(|leaf| match leaf {
            SemioFlowMutation::RemoveEdge(value) => json!({ "kind": "remove-edge", "id": value.id }),
            SemioFlowMutation::RemoveNode(value) => json!({ "kind": "remove-node", "id": value.id }),
            other => panic!("unexpected removal leaf {other:?}"),
        })
        .collect();
    assert_eq!(Value::Array(named), fixture["expectedLeaves"]);
    let (forward, inverses) = leaves.iter().fold((base.clone(), Vec::new()), |(state, mut inverses), leaf| {
        inverses.push(<SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::inverse(leaf, &state).unwrap());
        let (diff, messages) = <SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::diff(leaf, &state).into_parts();
        assert!(messages.is_empty(), "{leaf:?}: {messages:?}");
        (protocol::apply_diff(&diff, &state).unwrap(), inverses)
    });
    assert_eq!(serde_json::to_value(forward.edges.iter().map(|edge| &edge.id).collect::<Vec<_>>()).unwrap(), fixture["expectedForwardSynapses"]);
    assert!(forward.nodes.iter().all(|node| node.id != target));
    let restored = inverses.iter().rev().flatten().fold(forward, |state, inverse| {
        let diff = <SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::diff(inverse, &state).into_parts().0;
        protocol::apply_diff(&diff, &state).unwrap()
    });
    let by_id = |mut snapshot: SemioFlowSnapshot| {
        snapshot.nodes.sort_by(|a, b| a.id.cmp(&b.id));
        snapshot.edges.sort_by(|a, b| a.id.cmp(&b.id));
        snapshot
    };
    let restored = by_id(restored);
    assert_eq!(restored, by_id(base));
    assert_eq!(restored.nodes.iter().find(|node| node.id == target).map(|node| node.label.as_str()), Some(label.as_str()));
}

/// 🗂️ Serde is the independent Rust JSON oracle for the language-agnostic Flow/Note route census, read exactly as the
/// cohort schema (`🎬️action-cohort/🧬️schema` `routeCount`) defines it: the owner's own command rows, each classified in
/// exactly one group. Flow's census is additionally the exact row set of its typed command table.
#[test]
fn action_cohort_fixtures_match_the_exact_route_census() {
    let flow: Value = serde_json::from_str(fixture_text!("../../../../../../../../../../../🧪️tests/🧫️fixtures/🎬️action-cohort/🔣️.json")).expect("Flow action-cohort fixture must be valid JSON");
    let note: Value = serde_json::from_str(fixture_text!("../../../../../../../../../../../../🗒️note/🧪️tests/🧫️fixtures/🧪️action-cohort/🔣️.json")).expect("Note action-cohort fixture must be valid JSON");
    for (fixture, owner) in [(&flow, "FlowPlayApp"), (&note, "NotePlayApp")] {
        assert_eq!(fixture["owner"], owner);
        let routes: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
        let retained: Vec<&str> = fixture["retainedRoutes"].as_array().expect("retained routes").iter().map(|route| route.as_str().expect("retained route id")).collect();
        let migrated: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().filter(|group| group["status"] == "migrated").flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
        assert_eq!(retained, migrated, "{owner}: the retained route index must exactly name the migrated groups");
        let mut unique = routes.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), routes.len(), "{owner}: every route is classified in exactly one group");
        assert_eq!(fixture["routeCount"].as_u64(), Some(routes.len() as u64), "{owner}: routeCount is exactly the classified command rows");
    }
    let mut classified: Vec<&str> = flow["groups"].as_array().expect("groups").iter().flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
    classified.sort_unstable();
    let mut declared = FlowCommand::TOOL_JOB_IDS.to_vec();
    declared.sort_unstable();
    assert_eq!(classified, declared, "the Flow census names exactly FlowCommand's rows");
}

async fn context_menu_items(app: &mut FlowApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
    let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface, window_instance_id: None, point: None };
    serde_json::to_value(app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await).unwrap_or(Value::Null)
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids, FlowCommand::TOOL_JOB_IDS, "every FlowCommand row must be covered in declaration order by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
/// silently breaks (the record prints with no keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected: String = id.chars().flat_map(|character| if character.is_ascii_uppercase() { vec!['-', character.to_ascii_lowercase()] } else { vec![character] }).collect();
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(crate) fn every_command() -> Vec<FlowCommand> {
    use semio_framework_artifact_flow_flow::CameraJson;
    vec![
        FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: Some("originalOperator雪".into()), x: Some(10.0), y: Some(20.0), label: Some("originalLabel🌊️".into()), action: Some("originalAction雪".into()), format: Some("originalFormat🌊️".into()) }),
        FlowCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "n1".into() }),
        FlowCommand::DuplicateWidget(duplicate_widget::DuplicateWidget { widget_id: "n1".into() }),
        FlowCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        FlowCommand::Disconnect(disconnect::Disconnect { synapse_id: "s1".into() }),
        FlowCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
        FlowCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
        FlowCommand::Reorganize(reorganize::Reorganize {}),
        FlowCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["n1".into(), "n2".into()], field: "value".into(), value: "5".into() }),
        FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: "n1".into(), value: "renamed".into() }),
        FlowCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
            operations: vec![
                node_graph_edit::FlowNodeGraphEditOp::Delete { node_ids: vec!["n1".into()], synapse_ids: vec!["s1".into()] },
                node_graph_edit::FlowNodeGraphEditOp::InsertPort { node_id: "n2".into(), side: "input".into(), index: 1 },
                node_graph_edit::FlowNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
                node_graph_edit::FlowNodeGraphEditOp::Move { gesture_id: "originalGesture雪".into(), node_ids: vec!["n1".into(), "n2".into()], dx: 2.0, dy: 3.0 },
                node_graph_edit::FlowNodeGraphEditOp::SetSlider { widget_id: "originalSlider🌊️".into(), value: 4.0 },
            ],
        }),
        FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit { operations: vec![spotlight_commit::FlowNodeGraphEditOp::Disconnect { synapse_id: "s1".into() }] }),
        FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }),
        FlowCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }),
        FlowCommand::Evaluate(evaluate::Evaluate {}),
        FlowCommand::FocusSelection(focus_selection::FocusSelection {}),
        FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 1.5 } }),
        FlowCommand::SetLodMode(set_lod_mode::SetLodMode { value: "micro".into() }),
        FlowCommand::SetProximityDistance(set_proximity_distance::SetProximityDistance { value: 48.0 }),
        FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(true) }),
        FlowCommand::SetGridSnapEnabled(set_grid_snap_enabled::SetGridSnapEnabled { pressed: None }),
        FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 10.0 }),
        FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: "n1".into() }),
        FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["n1".into()], value: true }),
        FlowCommand::OpenSpotlight(open_spotlight::OpenSpotlight {}),
        FlowCommand::ReplaceImage(replace_image::ReplaceImage { id: "n1".into() }),
        FlowCommand::SetCatalogueSections(set_catalogue_sections::SetCatalogueSections { sections_json: "[]".into() }),
        FlowCommand::ToggleExtension(toggle_extension::ToggleExtension { id: "auto-layout".into(), enabled: true }),
        FlowCommand::AddGeneration(add_generation::AddGeneration {}),
        FlowCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "g1".into() }),
        FlowCommand::SelectGeneration(select_generation::SelectGeneration { id: "g1".into() }),
        FlowCommand::RenameGeneration(rename_generation::RenameGeneration { id: "g1".into(), name: "Copy".into() }),
        FlowCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: semio_framework_value::DslValue::float(5.0) }),
        FlowCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: FLOW_PLAY_WINDOW_MAIN.into(), window_kind_id: FLOW_PLAY_WINDOW_MAIN.into() }),
        FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { window_id: FLOW_PLAY_WINDOW_MAIN.into(), node_hash: 42, output_json: Some(r#"{"done":true,"phase":"complete","unitsDone":1,"unitsTotal":1,"outputJson":"{}"}"#.into()) }),
        FlowCommand::SetContributions(set_contributions::SetContributions { json: "[]".into(), page: 0, page_count: 1 }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_flow_app()).expect("app definition json");
    for id in [FLOW_PLAY_WINDOW_MAIN, compiled::FLOW_PLAY_WINDOW_COMPILED, FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::FLOW_PLAY_MODE_EDIT, generate::FLOW_PLAY_MODE_GENERATE, generate::FLOW_PLAY_LAYOUT_GENERATE] {
        assert!(json.contains(id), "mode/layout {id} missing from the manifest");
    }
    for body in [FLOW_PLAY_BODY_ARTIFACT, FLOW_PLAY_BODY_CATALOGUE, FLOW_PLAY_BODY_INSPECTOR] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("computation.flow"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
/// 🕹️ The "graph" domain is declared `HierarchyProvider::Topology`, scoped to the main canvas window
/// kind, non-transitive (see the `.interaction(...)` doc comment for why), with node/edge/handle
/// granularities and all five merges.
#[semio_framework_async_macros::async_test]
async fn graph_interaction_domain_is_declared_topology_and_scoped_to_the_main_window() {
    let definition = create_flow_app();
    let graph = definition.interactions.iter().find(|interaction| interaction.id == FLOW_INTERACTION_GRAPH).expect("graph interaction domain declared");
    assert!(matches!(graph.hierarchy, HierarchyProvider::Topology));
    assert!(!graph.hover.transitive, "graph's outer widget list has no real group membership to walk transitively");
    assert!(!graph.selection.transitive);
    let granularity_ids: Vec<&str> = graph.granularities.iter().map(|granularity| granularity.id.as_str()).collect();
    assert_eq!(granularity_ids, ["node", "edge", "handle"]);
    let main_window = definition.window_kinds.iter().find(|window| window.id == FLOW_PLAY_WINDOW_MAIN).expect("main window kind declared");
    assert!(main_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == FLOW_INTERACTION_GRAPH), "main window must reference the graph interaction domain");
}

/// 🌳️ `interaction_topology` registers every widget/synapse as a root at its own granularity —
/// the same row-id-prefixed targets the document panel tree renders (see
/// `document_panel::render`'s doc comment).
#[semio_framework_async_macros::async_test]
async fn interaction_topology_registers_every_widget_and_synapse_as_a_root() {
    let document = FlowSnapshot::default();
    let config = semio_framework_plugin::NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = FlowPlayApp::interaction_topology(&doc, &cfg).expect("valid retained interaction fixture");
    let graph = topology.domains.get(FLOW_INTERACTION_GRAPH).expect("graph domain present in topology");
    let live = document.to_host_snapshot();
    assert_eq!(graph.ordered.len(), live.widgets.len() + live.synapses.len());
    assert!(graph.ordered.iter().all(|node| node.parent.is_none()), "every node/edge is a root — no real group membership at this level");
    assert!(graph.ordered.iter().any(|node| node.id == flow_graph_node_target_id("slider") && node.granularity == "node"));
    assert!(graph.ordered.iter().any(|node| node.id == flow_graph_edge_target_id("s1") && node.granularity == "edge"));
}
//#endregion 🔖️Interaction

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn undo_restores_fixture_after_add_widget() {
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, settle_history_verb, settle_registered_typed_operation};

    // 🧩️ Measured on the CHILD document, not on `to_host_snapshot()`. `addWidget` publishes on the
    // `Child` lane and its own law asserts that the parent's `content` coordinate is unchanged, and
    // `to_host_snapshot` reads exactly that coordinate's cached scene (`🌊️flow/🦀️.rs:271`) — so a
    // parent-side widget count can never move for this verb and read `3, expected 4` whatever the
    // command did. The child store is the surface the command writes and the window reads.
    let mut app = flow_app().await;
    let receiver = meta("local").instance_id;
    let child_id = app.snapshot().expect("snapshot").content.child_id.clone();
    let before = flow_child_node_count(&app, &child_id).await;
    dispatch(&mut app, FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0), label: None, action: None, format: None })).await;
    settle_registered_typed_operation(&mut *app, receiver, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("addWidget child publication");
    assert_eq!(flow_child_node_count(&app, &child_id).await, before + 1, "addWidget must land one node in the content child");
    settle_history_verb(&mut *app, "undo", receiver, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    assert_eq!(flow_child_node_count(&app, &child_id).await, before, "undo must retire the child-lane group");
    settle_history_verb(&mut *app, "redo", receiver, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    assert_eq!(flow_child_node_count(&app, &child_id).await, before + 1, "redo must reapply the child-lane group");
    close_registered_fixture_app(&mut *app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}

/// 🚚️ An absolute authored drop publishes one relative child edit and replays undo exactly.
#[semio_framework_async_macros::async_test]
async fn retained_move_media_node_preserves_absolute_drop_and_undo() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::artifact_app_laws::{settle_history_verb, settle_registered_typed_operation};
    use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
    use store::{pack::snapshot::ArtifactPack, SpaceMember};

    let mut app = flow_app_with_registry().await;
    let receiver = meta("local").instance_id;
    let parent = app.snapshot().expect("Flow parent");
    let child_id = parent.content.child_id.clone();
    let before = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("content child").document_pack_bytes().await.expect("content pack")).expect("content snapshot");
    let node = before.nodes.first().expect("genesis content node");
    let (x, y) = (node.position.x + 37.0, node.position.y - 23.0);
    dispatch(&mut app, FlowCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: node.id.clone(), x, y })).await;
    let lanes = settle_registered_typed_operation(&mut *app, receiver, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("moveMediaNode publication").lanes;
    assert_eq!(lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal]);
    let after = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("moved child").document_pack_bytes().await.expect("moved pack")).expect("moved snapshot");
    let moved = after.nodes.iter().find(|candidate| candidate.id == node.id).expect("addressed node");
    assert_eq!((moved.position.x, moved.position.y), (x, y));
    assert_eq!(app.snapshot().expect("parent after move").content, parent.content);
    settle_history_verb(&mut *app, "undo", receiver, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    let restored = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("restored child").document_pack_bytes().await.expect("restored pack")).expect("restored snapshot");
    assert_eq!(restored, before);
}

/// 🧩️ The `content` child's node count — the authoritative witness for every `Child`-lane verb.
async fn flow_child_node_count(app: &FlowApp, child_id: &str) -> usize {
    use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
    use store::{pack::snapshot::ArtifactPack, SpaceMember};

    SemioFlowSnapshot::decode_pack(&app.child_store("content", child_id).await.expect("Flow content child").document_pack_bytes().await.expect("Flow content child pack"))
        .expect("Flow content child snapshot")
        .nodes
        .len()
}

#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_three_surfaces() {
    let mut app = flow_app().await;
    use crate::editor::flow::unit_tests::context::render;
    assert!(render(&mut app, FLOW_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
    assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
    assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW).await.contains("text-editor"));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::flow::unit_tests::context::render;
    let mut app = flow_app().await;
    assert!(render(&mut app, "flow.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn host_from_snapshot_deletes_edge_selected_by_synapse_domain() {
    context::install_first_party_light_flow_extensions_for_tests();
    let config = FlowMainWindowConfig::default();
    let fixture = FlowSnapshot::default();
    let mut session = FlowEvalSession::new();
    let mut host = host_from_snapshot(&fixture, &config, &session);
    let edge_id = host.host_snapshot.synapses.first().expect("starter graph edge").id.clone();
    sync_host_selection_domains(&mut host, &[], std::slice::from_ref(&edge_id), &[]);
    assert!(host.has_selection(), "the starter edge must resolve through host_from_snapshot edge map");
    host.delete_selection().expect("deleteSelection");
    assert!(!host.host_snapshot.synapses.iter().any(|synapse| synapse.id == edge_id));
    host.retire_cold();
    session.retire_cold();
}

/// 🧬️ `artifact_app_laws::assert_two_registered_instances_converge` replayed over THIS crate's harness:
/// A adds an input note at (40, 41), B adds a button at (300, 301); both replicas must fold each other's
/// events onto the same `content` child. The framework law cannot host flow: its apps are member-less
/// and child-less, so `addWidget` (a child-lane tool) and every other document verb falls through to
/// the parent preparation ("Flow mutation requires its explicit batch-only recipe"), and it dispatches
/// without a window view, which every retained flow reducer requires (`flow-window-view-required`).
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_on_disjoint_edits() {
    use semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation;
    use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
    use store::{pack::snapshot::ArtifactPack, os_store::backbone::MemoryBackbone, SpaceMember};
    async fn probe(app: &FlowApp) -> Vec<(String, i64, i64)> {
        let child_id = app.snapshot().expect("Flow parent snapshot").content.child_id.clone();
        let child = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow content child").document_pack_bytes().await.expect("Flow content child pack")).expect("decode Flow content child");
        let mut nodes: Vec<(String, i64, i64)> = child.nodes.iter().map(|node| (node.kind.to_string(), node.position.x as i64, node.position.y as i64)).collect();
        nodes.sort();
        nodes
    }
    fn close(app: &mut FlowApp) {
        for _ in 0..100_000 {
            let demand = PluginApp::close_retirement_demands(&*app, 16_384).expect("Flow convergence app close quote");
            let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(16_384), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            if matches!(PluginApp::close_step(app, grant).expect("Flow convergence app close"), semio_framework_plugin::PluginLifecycleStep::Complete(_)) {
                return;
            }
        }
        panic!("Flow convergence app did not reach its close witness");
    }
    let mut instance_a = flow_app_with_registry().await;
    let mut instance_b = flow_app_with_registry().await;
    let metadata_a = semio_framework_plugin::ActionMeta { instance_id: 71, ..meta("actor-a") };
    let metadata_b = semio_framework_plugin::ActionMeta { instance_id: 72, ..meta("actor-b") };
    instance_a.bind_instance_id(metadata_a.instance_id).await;
    instance_b.bind_instance_id(metadata_b.instance_id).await;
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://flow-convergence", "mem://flow-convergence").await;
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");
    let genesis = probe(&instance_a).await;
    instance_a.dispatch_typed(FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(41.0), label: None, action: None, format: None }), &metadata_a, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("a applies its edit");
    settle_registered_typed_operation(&mut *instance_a, metadata_a.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("a's edit publishes");
    instance_b.dispatch_typed(FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(300.0), y: Some(301.0), label: None, action: None, format: None }), &metadata_b, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("b applies its disjoint edit");
    settle_registered_typed_operation(&mut *instance_b, metadata_b.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("b's edit publishes");
    instance_a.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("a folds b's events");
    instance_b.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("b folds a's events");
    let converged = probe(&instance_a).await;
    assert_eq!(converged, probe(&instance_b).await, "both instances must converge on the same content child");
    assert_eq!(converged.len(), genesis.len() + 2, "each instance holds both disjoint notes");
    let admitted = instance_a.handle_action("commitCheckpoint", None, &metadata_a, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("a commits a checkpoint");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut *instance_a, admitted).await.expect("a's checkpoint commit settles");
    settle_registered_typed_operation(&mut *instance_a, metadata_a.instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("a's checkpoint publication settles");
    instance_b.tick_backbone(&mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("b folds a's checkpoint");
    assert_eq!(probe(&instance_a).await, probe(&instance_b).await, "a replicated checkpoint keeps both instances converged");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
    close(&mut instance_a);
    close(&mut instance_b);
}
//#endregion 🔖️CrossCutting

//#region 🔖️ContextMenu
#[semio_framework_async_macros::async_test]
async fn context_menu_includes_select_all_when_empty() {
    let mut app = flow_app_with_registry().await;
    let menu = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "main".into(), kind: "nodeGraph".into(), hits: vec![], selection: vec![], text: None })).await;
    let menu_json = menu.to_string();
    assert!(menu_json.contains("selectAll"), "menu should be {menu_json}");
    assert!(menu_json.contains("Select All") || menu_json.contains("select-all"), "menu should be {menu_json}");
    assert!(menu_json.contains(r#""icon":"plus""#), "add-node icon: {menu_json}");
    assert!(menu_json.contains(r#""id":"delete-selection""#) && menu_json.contains(r#""reason":"Nothing selected""#) && !menu_json.contains(r#""action":"deleteSelection""#), "empty canvas keeps delete disabled with its reason: {menu_json}");
    assert!(!menu_json.contains("setPreviewOff"), "empty canvas must omit preview: {menu_json}");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection is framework-owned
/// `InteractionState` now and `ArtifactApp::context_menu` is not threaded an `InteractionView` this
/// wave (see `flow_context_menu_items`'s doc comment) — the request's own `surface.selection` groups
/// are the only way to feed a selection into the menu, mirroring what the real click caller carries.
fn node_selection_surface(node_ids: &[&str]) -> semio_framework_plugin::ContextMenuSurfaceTarget {
    semio_framework_plugin::ContextMenuSurfaceTarget {
        surface_id: "main".into(),
        kind: "nodeGraph".into(),
        hits: vec![],
        selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: node_ids.iter().map(|id| id.to_string()).collect() }],
        text: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn context_menu_includes_hide_preview_for_selection_and_set_preview_off_mutates_scene() {
    let mut app = flow_app_with_registry().await;
    let menu = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).await.to_string();
    assert!(menu.contains("setPreviewOff"), "menu should expose preview toggle: {menu}");
    assert!(menu.contains("Hide preview") || menu.contains("eye-off"), "menu should offer hide preview: {menu}");
    assert!(menu.contains("focusSelection"), "menu should expose zoom to selection: {menu}");
    assert!(menu.contains(r#""checked":true"#), "preview checked when visible: {menu}");
    dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: true })).await;
    settle(&mut app).await;
    let after_menu = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).await.to_string();
    assert!(after_menu.contains("Show preview") || after_menu.contains(r#""icon":"eye""#), "menu should offer show preview: {after_menu}");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `contextMenuAt` no longer sets
/// selection (a genuine no-operation now, see `context_menu_at::apply`'s doc comment) — the request's
/// own `surface.selection` groups carry the clicked target instead, mirroring what the real caller
/// (right-clicking a node) supplies alongside the `contextMenuAt` dispatch.
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_target_and_enables_preview() {
    let mut app = flow_app_with_registry().await;
    let before = context_menu_items(&mut app, None).await.to_string();
    assert!(before.contains(r#""reason":"Nothing selected""#) && !before.contains(r#""action":"deleteSelection""#), "preview starts with delete disabled: {before}");
    dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: "slider".into() })).await;
    settle(&mut app).await;
    let after = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).await.to_string();
    assert!(after.contains("setPreviewOff"), "menu keeps preview: {after}");
    assert!(after.contains(r#""ids":["slider"]"#) || after.contains("slider"), "preview args target the clicked node: {after}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_annotates_mixed_selection_counts_and_disables_delete_without_selection() {
    let mut app = flow_app_with_registry().await;
    let empty = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "main".into(), kind: "nodeGraph".into(), hits: vec![], selection: vec![], text: None })).await.to_string();
    assert!(empty.contains(r#""reason":"Nothing selected""#) && !empty.contains(r#""action":"deleteSelection""#), "empty keeps delete disabled with its reason: {empty}");

    let menu = context_menu_items(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
            selection: vec![
                semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: (1..=8).map(|i| format!("n{i}")).collect() },
                semio_framework_plugin::ContextMenuSelectionGroup { domain: "edge".into(), ids: (1..=13).map(|i| format!("e{i}")).collect() },
            ],
            text: None,
        }),
    )
    .await
    .to_string();
    assert!(menu.contains(r#""id":"delete-selection""#), "mixed selection must expose delete: {menu}");
    assert!(menu.contains("8 nodes and 13 edges"), "count phrase missing: {menu}");
    assert!(menu.contains("deleteSelection"), "delete action missing: {menu}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_for_edge_hit_uses_surface_edge_selection() {
    let mut app = flow_app_with_registry().await;
    let menu = context_menu_items(
        &mut app,
        Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "edge".into(), id: "syn-1".into(), label: None }],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "edge".into(), ids: vec!["syn-1".into()] }],
            text: None,
        }),
    )
    .await
    .to_string();
    assert!(menu.contains(r#""id":"delete-selection""#), "edge selection must expose delete: {menu}");
    assert!(menu.contains("1 edge") || menu.contains("1 Kante"), "edge count phrase missing: {menu}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
    let mut app = flow_app_with_registry().await;
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
            selection: vec![
                semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: (1..=8).map(|i| format!("n{i}")).collect() },
                semio_framework_plugin::ContextMenuSelectionGroup { domain: "edge".into(), ids: (1..=13).map(|i| format!("e{i}")).collect() },
            ],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("deleteSelection");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
}
//#endregion 🔖️ContextMenu
