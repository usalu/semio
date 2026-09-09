use super::*;
use crate::editor::flow::testkit::{dispatch, flow_app, flow_app_with_registry, FlowApp};
use semio_framework_plugin::testkit::meta;
use semio_framework_plugin::{EditorApp, PluginApp};

#[test]
fn retained_add_widget_factory_is_exact_child_only_and_legacy_closed() {
    let factory = FlowChildGroupJobFactory::new("s.flow.flow@1/*#editor");
    let keys = semio_framework::ToolJobFactory::keys(&factory);
    assert_eq!(keys, &[semio_framework::ToolFactoryKey::new("s.flow.flow@1/*#editor", "addWidget")]);
    assert_eq!(semio_framework::ToolJobFactory::payload_schema_id(&factory), FLOW_DOCUMENT_SCHEMA);
    assert_eq!(semio_framework::ToolJobFactory::classification(&factory), semio_framework_plugin::InteractiveJobClassification::Migrated);
    assert_eq!(semio_framework::ToolJobFactory::execution_contract(&factory), semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),);
    assert_eq!(<FlowChildGroupJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::TOOL_IDS, &["addWidget"]);
    let publication = <FlowChildGroupJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(publication.len(), 1);
    assert_eq!(publication[0].tool_id, "addWidget");
    assert_eq!(publication[0].lanes, &[semio_framework_plugin::ArtifactToolPublicationLane::Child]);
    let proofs = <FlowPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs();
    assert_eq!(proofs.len(), FLOW_DIRECT_STORE_TOOL_IDS.len() + FLOW_HOST_ONLY_TOOL_IDS.len() + FLOW_CHILD_GROUP_TOOL_IDS.len());
    assert!(FLOW_CHILD_GROUP_TOOL_IDS.contains(&"addWidget"));
    eprintln!("[DEBUG] retained addWidget factory owns one key, one exact proof and one Child-only publication lane");
}

#[semio_framework_async_macros::async_test]
async fn retained_add_widget_dispatches_one_acknowledged_child_group_and_retires() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    use store::{ArtifactPack, SpaceMember};

    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️add-widget-retained/🔣️.json")).expect("retained addWidget fixture");
    let mut app = flow_app_with_registry().await;
    PluginApp::bind_instance_id(&mut app, 1).await;
    let parent_before = app.snapshot().expect("Flow parent before retained addWidget");
    let child_id = parent_before.content.child_id.clone();
    let child_before = semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot::decode_pack(
        &app.child_store("content", &child_id).await.expect("Flow child before retained addWidget").document_pack_bytes().await.expect("Flow child pack before retained addWidget"),
    )
    .expect("decode Flow child before retained addWidget");
    let accepted = &fixture["accepted"][0]["command"];
    let started = dispatch(
        &mut app,
        FlowCommand::AddWidget(add_widget::AddWidget { kind: accepted["kind"].as_str().expect("kind").to_string(), neuron_kind: accepted["neuronKind"].as_str().map(str::to_string), x: accepted["x"].as_f64(), y: accepted["y"].as_f64() }),
    )
    .await;
    assert!(started.mutations.is_empty(), "retained addWidget must not publish through its immediate invocation result");
    let lanes = semio_framework_plugin::testkit::settle_registered_typed_operation(&mut app, 1).await.expect("retained addWidget publication and exact ACK").lanes;
    assert_eq!(lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Terminal]);
    assert!(!PluginApp::has_pending_typed_operations(&app));
    let parent_after = app.snapshot().expect("Flow parent after retained addWidget");
    assert_eq!(parent_after.content, parent_before.content, "retained addWidget must preserve the exact parent content coordinate");
    let child_after = semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot::decode_pack(
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
        if PluginApp::close_step(&mut app, 1, 16_384).expect("retained addWidget app close") == semio_framework_plugin::PluginCloseStep::Complete {
            break;
        }
    }
    assert!(PluginApp::close_terminal_is_empty(&app));
    eprintln!("[DEBUG] retained addWidget published one typed child mutation, preserved parent identity and closed every app owner");
}

/// ↩️ Nonadjacent severed edges regain their exact original indices and large authored content.
#[test]
fn delete_cascade_inverse_restores_exact_edge_order_and_label() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧹️delete-cascade/🔣️.json")).unwrap();
    let mut scene = fixture["scene"].clone();
    let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
    assert_eq!(label.len(), fixture["label"]["expectedBytes"].as_u64().unwrap() as usize);
    scene["widgets"][1]["label"] = Value::String(label);
    let (widgets, synapses, layout) = crate::schema::mutations::decode_flow_scene_json(&scene.to_string()).unwrap();
    let base = FlowSnapshot { content: crate::flow_content_child_handle(&widgets, &synapses, &layout), ..FlowSnapshot::default() };
    let mutation = FlowMutation::DeleteWidget(DeleteWidget { id: fixture["targetId"].as_str().unwrap().into() });
    let ordinary_inverse = crate::schema::mutations::inverse_flow_mutation(&base, &mutation);
    let (post, prepared_inverse, _) = prepare_flow_artifact(&base, mutation).unwrap();
    let forward = crate::flow_working_scene(&post);
    assert_eq!(serde_json::to_value(forward.synapses.iter().map(|edge| &edge.id).collect::<Vec<_>>()).unwrap(), fixture["expectedForwardSynapses"]);
    for inverses in [ordinary_inverse, prepared_inverse] {
        let indices: Vec<_> = inverses
            .iter()
            .filter_map(|inverse| match inverse {
                FlowMutation::ConnectWidgets(value) => Some(value.index),
                _ => None,
            })
            .collect();
        assert_eq!(serde_json::to_value(indices).unwrap(), fixture["expectedInverseIndices"]);
        let mut restored = post.clone();
        for inverse in inverses {
            crate::schema::mutations::apply_flow_mutation(&mut restored, &inverse).unwrap();
        }
        assert_eq!(crate::schema::mutations::encode_flow_projection_json(&restored), crate::schema::mutations::encode_flow_projection_json(&base));
    }
}

/// 🗂️ Serde is the independent Rust JSON oracle for the language-agnostic Flow/Note route census.
#[test]
fn action_cohort_fixtures_match_the_exact_route_census() {
    let flow: Value = serde_json::from_str(include_str!("../../../../../../../../../🧫️fixtures/🎬️action-cohort/🔣️.json")).expect("Flow action-cohort fixture must be valid JSON");
    let note: Value = serde_json::from_str(include_str!("../../../../../../../../../../🗒️note/🧫️fixtures/🧪️action-cohort/🔣️.json")).expect("Note action-cohort fixture must be valid JSON");
    for (fixture, owner, total, framework_owned) in [(&flow, "FlowPlayApp", 37_u64, 0_usize), (&note, "NotePlayApp", 36_u64, 0_usize)] {
        assert_eq!(fixture["owner"], owner);
        assert_eq!(fixture["routeCount"].as_u64(), Some(total));
        assert!(fixture["retainedRoutes"].as_array().is_some_and(Vec::is_empty));
        assert_eq!(fixture["frameworkOwnedRoutes"].as_array().map(Vec::len), Some(framework_owned));
        let routes: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
        let mut unique = routes.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(routes.len() + framework_owned, total as usize);
        assert_eq!(unique.len(), routes.len());
    }
}

async fn context_menu_items(app: &mut FlowApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
    let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface, window_instance_id: None, point: None };
    serde_json::to_value(app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await).unwrap_or(Value::Null)
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

/// ⚖️ The two rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
/// bytes captured from the pre-merge `flow_protocol` crate. A regression here is a real format break,
/// not a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(FlowCommand, &str, &str); 3] = [
        (FlowCommand::AddWidget(add_widget::AddWidget { kind: "neuron".into(), neuron_kind: Some("math.add".into()), x: None, y: None }), "add-widget kind=neuron neuron-kind=math.add", "010002086d6174682e616464066e6575726f6e02000601010600"),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `SetGridVisible`'s binary
        // ordinal shifted 24 (0x18) → 18 (0x12) — seven rows ahead of it in `FlowCommand`
        // (`setSelection`/`clearSelection`/`selectAll`/`selectNode`/`nodeGraphSelect`/
        // `nodeGraphHover`/`graphPointerDown`) were deleted (framework-injected now), a real,
        // documented wire-format break (row order IS the ordinal — deleting from the middle is not
        // the safe "append only" case the row-order doc comment calls out).
        (FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: None }), "set-grid-visible", "01120000"),
        (FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(true) }), "set-grid-visible pressed=true", "011200010002"),
    ];
    for (command, text, hex) in cases {
        let encoded = protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(protocol::OpText::print_op(&command), text, "text for {command:?}");
        assert_eq!(encoded, hex, "hex for {command:?}");
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<FlowCommand> {
    use semio_framework_artifact_flow_flow::CameraJson;
    vec![
        FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None }),
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
                node_graph_edit::FlowNodeGraphEditOp::SetSnapshot { snapshot_json: "{}".into() },
                node_graph_edit::FlowNodeGraphEditOp::DeleteSelection,
                node_graph_edit::FlowNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
            ],
        }),
        FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit { operations: vec![spotlight_commit::FlowNodeGraphEditOp::DeleteSelection] }),
        FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }),
        FlowCommand::Evaluate(evaluate::Evaluate {}),
        FlowCommand::FocusSelection(focus_selection::FocusSelection {}),
        FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 } }),
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
        FlowCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
        FlowCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
        FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_flow_app()).expect("app definition json");
    for id in [main::FLOW_PLAY_WINDOW_MAIN, compiled::FLOW_PLAY_WINDOW_COMPILED, generations::FLOW_PLAY_WINDOW_GENERATIONS, form::FLOW_PLAY_WINDOW_GENERATE_FORM, preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::FLOW_PLAY_MODE_EDIT, generate::FLOW_PLAY_MODE_GENERATE, generate::FLOW_PLAY_LAYOUT_GENERATE] {
        assert!(json.contains(id), "mode/layout {id} missing from the manifest");
    }
    for body in [FLOW_PLAY_BODY_DOCUMENT, FLOW_PLAY_BODY_CATALOGUE, FLOW_PLAY_BODY_INSPECTOR] {
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
    let main_window = definition.window_kinds.iter().find(|window| window.id == main::FLOW_PLAY_WINDOW_MAIN).expect("main window kind declared");
    assert!(main_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == FLOW_INTERACTION_GRAPH), "main window must reference the graph interaction domain");
}

/// 🌳️ `interaction_topology` registers every widget/synapse as a root at its own granularity —
/// the same row-id-prefixed targets the document panel tree renders (see
/// `document_panel::render`'s doc comment).
#[semio_framework_async_macros::async_test]
async fn interaction_topology_registers_every_widget_and_synapse_as_a_root() {
    let document = FlowSnapshot::default();
    let config = FlowMainWindowConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = FlowPlayApp::interaction_topology(&doc, &cfg);
    let graph = topology.domains.get(FLOW_INTERACTION_GRAPH).expect("graph domain present in topology");
    let live = document.to_fixture();
    assert_eq!(graph.ordered.len(), live.widgets.len() + live.synapses.len());
    assert!(graph.ordered.iter().all(|node| node.parent.is_none()), "every node/edge is a root — no real group membership at this level");
    assert!(graph.ordered.iter().any(|node| node.id == flow_graph_node_target_id("slider") && node.granularity == "node"));
    assert!(graph.ordered.iter().any(|node| node.id == flow_graph_edge_target_id("s1") && node.granularity == "edge"));
}
//#endregion 🔖️Interaction

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn undo_restores_fixture_after_add_widget() {
    let mut app = flow_app().await;
    let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
    dispatch(&mut app, FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) })).await;
    assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before + 1);
    app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before);
    app.handle_action("redo", None, &meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_three_surfaces() {
    let mut app = flow_app().await;
    use crate::editor::flow::testkit::render;
    assert!(render(&mut app, FLOW_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
    assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
    assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW).await.contains("text-editor"));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::flow::testkit::render;
    let mut app = flow_app().await;
    assert!(render(&mut app, "flow.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn host_from_snapshot_deletes_edge_selected_by_synapse_domain() {
    let config = FlowMainWindowConfig::default();
    let fixture = FlowSnapshot::default();
    let session = FlowEvalSession::new();
    let mut host = host_from_snapshot(&fixture, &config, &session);
    sync_host_selection_domains(&mut host, &[], &["s1".into()], &[]);
    assert!(host.has_selection(), "s1 must resolve through host_from_snapshot edge map");
    host.delete_selection().expect("deleteSelection");
    assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_on_disjoint_edits() {
    use crate::schema::widget_id;
    use semio_framework_plugin::testkit::paired_apps;
    let (mut instance_a, mut instance_b) = paired_apps::<EditorApp<FlowPlayApp>>("mem://flow-convergence").await;

    instance_a.dispatch_typed(FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: "input".into() }), &meta("actor-a")).await.expect("a renames slider");
    instance_b.dispatch_typed(FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(10.0), y: Some(10.0) }), &meta("actor-b")).await.expect("b adds a note");

    // A neutral history action always dispatches through the store, which pumps inbound operations first.
    instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");

    let projection_a = instance_a.snapshot().expect("snapshot a").to_fixture();
    let projection_b = instance_b.snapshot().expect("snapshot b").to_fixture();
    assert!(projection_a.widgets.iter().any(|widget| widget_id(widget) == "input"), "A keeps its rename");
    assert!(projection_a.widgets.iter().any(|widget| matches!(widget, Widget::InputNote { .. })), "A absorbs B's note");
    assert_eq!(projection_a.widgets.len(), projection_b.widgets.len(), "both instances converge to the same widget set");
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
    assert!(!menu_json.contains(r#""id":"delete-selection""#), "empty canvas must omit delete: {menu_json}");
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
    assert!(!before.contains(r#""id":"delete-selection""#), "preview starts without delete: {before}");
    dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: "slider".into() })).await;
    let after = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).await.to_string();
    assert!(after.contains("setPreviewOff"), "menu keeps preview: {after}");
    assert!(after.contains(r#""ids":["slider"]"#) || after.contains("slider"), "preview args target the clicked node: {after}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_annotates_mixed_selection_counts_and_omits_delete_without_selection() {
    let mut app = flow_app_with_registry().await;
    let empty = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "main".into(), kind: "nodeGraph".into(), hits: vec![], selection: vec![], text: None })).await.to_string();
    assert!(!empty.contains(r#""id":"delete-selection""#), "empty must omit delete: {empty}");

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
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("deleteSelection");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
}
//#endregion 🔖️ContextMenu
