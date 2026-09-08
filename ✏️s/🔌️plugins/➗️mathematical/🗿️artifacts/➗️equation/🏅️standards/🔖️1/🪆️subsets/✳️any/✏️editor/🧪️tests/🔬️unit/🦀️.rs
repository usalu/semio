
use super::*;
use crate::editor::equation::testkit::{math_app, math_app_with_registry};

//#region 🔖️RetainedCommands
fn retained_operation(generation: u64) -> AppOperationContext {
    AppOperationContext { app_instance_id: 7, parent_document_id: "equation-retained-test".into(), operation_id: 11, generation, canonical_base_revision: [17; 32] }
}

fn graph_with_shape(node_count: usize, edge_count: usize) -> EquationGraph {
    let nodes = (0..node_count).map(|index| crate::EquationNode { id: format!("n{index}"), label: format!("N{index}"), x: index as f64, y: -(index as f64) }).collect();
    let edges = (0..edge_count).map(|index| crate::EquationEdge { id: format!("e{index}"), source: format!("n{}", index % node_count.max(1)), target: format!("n{}", (index + 1) % node_count.max(1)) }).collect();
    EquationGraph { directed: true, nodes, edges, algorithm: "bfs".into(), algorithm_seed: Some("n0".into()) }
}

fn drive_retained(work: &mut EquationRetainedCommandWork, command: &EquationCommand, snapshot: &EquationSnapshot, operation: &AppOperationContext) -> protocol::DslValue {
    let config = EquationConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    loop {
        match work.step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation }).expect("retained Equation turn") {
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Progress { .. } => {}
            // 🌱️ `ToValue`/`DslValue` in place of the old `serde_json::to_value` oracle: `DslValue`
            // already implements `PartialEq`, so the two runs compare directly with no JSON text
            // round trip needed.
            ArtifactCommandWorkStep::Complete(emit) => return protocol::ToValue::to_value(&emit.artifact_mutations),
            ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => panic!("Equation commands do not publish ephemeral state"),
        }
    }
}

#[test]
fn retained_schema_contract_and_factory_identity_are_exact() {
    let fixture: Value = json::parse(include_str!("../../../../../../../🧪️fixtures/⚖️equation-retained-command-law.json")).expect("language-neutral retained fixture");
    assert_eq!(fixture["contract"]["workItems"], 65_536);
    assert_eq!(fixture["contract"]["maximumStepMillis"], 8);
    assert_eq!(fixture["actions"], json::array(EQUATION_TOOL_IDS.iter().map(|id| Value::from(*id))));
    assert_eq!(fixture["hostileCases"].as_array().map(|values| values.len()), Some(14));
    let factory = EquationCommandJobFactory::new("s.mathematical.equation@1/*#editor");
    let keys = <EquationCommandJobFactory as semio_framework::ToolJobFactory>::keys(&factory);
    assert_eq!(keys.len(), EQUATION_TOOL_IDS.len());
    for (key, tool_id) in keys.iter().zip(EQUATION_TOOL_IDS) {
        assert_eq!(key.controller_id, "s.mathematical.equation@1/*#editor");
        assert_eq!(key.tool_id, *tool_id);
    }
    assert_eq!(<EquationPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 7);
}

#[semio_framework_async_macros::async_test]
async fn retained_semantic_maxima_accept_exact_and_reject_maximum_plus_one() {
    let command = EquationCommand::SetDirected(set_directed::SetDirected { directed: false });
    let maximum_nodes = crate::equation_snapshot_with_state(graph_with_shape(EQUATION_MAX_NODES, 0), EquationGeometry::default());
    let excessive_nodes = crate::equation_snapshot_with_state(graph_with_shape(EQUATION_MAX_NODES + 1, 0), EquationGeometry::default());
    assert!(equation_command_extent(&command, &maximum_nodes).is_some());
    assert!(equation_command_extent(&command, &excessive_nodes).is_none());
    let maximum_edges = crate::equation_snapshot_with_state(graph_with_shape(2, EQUATION_MAX_EDGES), EquationGeometry::default());
    let excessive_edges = crate::equation_snapshot_with_state(graph_with_shape(2, EQUATION_MAX_EDGES + 1), EquationGeometry::default());
    assert!(equation_command_extent(&command, &maximum_edges).is_some());
    assert!(equation_command_extent(&command, &excessive_edges).is_none());

    let snapshot = crate::equation_snapshot_with_state(EquationGraph::default(), EquationGeometry::default());
    let point = crate::EquationPoint { x: 1.0, y: 2.0 };
    let maximum_points = EquationCommand::SetPoints(set_points::SetPoints { geometry: EquationGeometry { points: vec![point.clone(); EQUATION_MAX_POINTS] } });
    let excessive_points = EquationCommand::SetPoints(set_points::SetPoints { geometry: EquationGeometry { points: vec![point; EQUATION_MAX_POINTS + 1] } });
    assert!(equation_command_extent(&maximum_points, &snapshot).is_some());
    assert!(equation_command_extent(&excessive_points, &snapshot).is_none());
    let maximum_text = "a".repeat(EQUATION_MAX_TEXT_BYTES);
    let excessive_text = "a".repeat(EQUATION_MAX_TEXT_BYTES + 1);
    assert!(equation_command_extent(&EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: maximum_text, seed: None }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: excessive_text, seed: None }), &snapshot).is_none());
    assert!(equation_command_extent(&EquationCommand::SetLocale(set_locale::SetLocale { value: "d".repeat(EQUATION_MAX_LOCALE_BYTES) }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::SetLocale(set_locale::SetLocale { value: "d".repeat(EQUATION_MAX_LOCALE_BYTES + 1) }), &snapshot).is_none());

    let operations = |count: usize| json::to_string(&json::array(std::iter::repeat(json::object([])).take(count)));
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: operations(EQUATION_MAX_EDIT_OPERATIONS) }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: operations(EQUATION_MAX_EDIT_OPERATIONS + 1) }), &snapshot).is_none());
    let delete = |count: usize| json::to_string(&json::array([json::object([("operation".to_string(), Value::from("deleteSelection")), ("nodeIds".to_string(), json::array((0..count).map(|index| Value::from(format!("n{index}")))))])]));
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: delete(EQUATION_MAX_DELETE_IDS) }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: delete(EQUATION_MAX_DELETE_IDS + 1) }), &snapshot).is_none());
    let exact_json = format!("[{}]", " ".repeat(EQUATION_MAX_EDIT_JSON_BYTES - 2));
    let excessive_json = format!("[{}]", " ".repeat(EQUATION_MAX_EDIT_JSON_BYTES - 1));
    assert_eq!(exact_json.len(), EQUATION_MAX_EDIT_JSON_BYTES);
    assert_eq!(excessive_json.len(), EQUATION_MAX_EDIT_JSON_BYTES + 1);
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: exact_json }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: excessive_json }), &snapshot).is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_interruption_replay_aba_cancel_and_repeated_close_are_exact() {
    let graph = graph_with_shape(8, 12);
    let snapshot = crate::equation_snapshot_with_state(graph, EquationGeometry::default());
    let command = EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
        operations_json: json::to_string(&json::array([
            json::object([("operation".to_string(), Value::from("move")), ("nodeId".to_string(), Value::from("n7")), ("x".to_string(), Value::from(41.0)), ("y".to_string(), Value::from(42.0))]),
            json::object([("operation".to_string(), Value::from("deleteSelection")), ("nodeIds".to_string(), json::array([Value::from("n1"), Value::from("n3")]))]),
            json::object([("operation".to_string(), Value::from("addNode")), ("x".to_string(), Value::from(5.0)), ("y".to_string(), Value::from(6.0))]),
        ])),
    });
    let operation = retained_operation(13);
    let extent = equation_command_extent(&command, &snapshot).expect("retained extent");
    let identity = equation_operation_identity("nodeGraphEdit", &operation);
    let mut uninterrupted = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    let config = EquationConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    for _ in 0..9 {
        assert!(matches!(
            uninterrupted
                .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
                .expect("checkpoint prefix"),
            ArtifactCommandWorkStep::Progress { .. }
        ));
    }
    let mut checkpoint = [0_u8; 40];
    assert_eq!(uninterrupted.checkpoint(&mut checkpoint).expect("checkpoint"), 40);
    let aba_operation = retained_operation(14);
    let mut stale_aba = EquationRetainedCommandWork::new("nodeGraphEdit", equation_operation_identity("nodeGraphEdit", &aba_operation), extent);
    assert!(stale_aba.restore(&checkpoint).is_err());
    let mut wrong_action = EquationRetainedCommandWork::new("setDirected", equation_operation_identity("setDirected", &operation), extent);
    assert!(wrong_action.restore(&checkpoint).is_err());

    let mut replayed = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    replayed.restore(&checkpoint).expect("interrupted restore");
    let uninterrupted_output = drive_retained(&mut uninterrupted, &command, &snapshot, &operation);
    let replayed_output = drive_retained(&mut replayed, &command, &snapshot, &operation);
    assert_eq!(uninterrupted_output, replayed_output, "the DslValue-encoded mutation output must observe exact replay output");

    let mut cancelled_before = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Blocked);
    cancelled_before.begin_close();
    assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
    assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
    let mut cancelled_after = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    assert!(matches!(
        cancelled_after
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
            .expect("cancel after admission"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    cancelled_after.begin_close();
    assert!(matches!(cancelled_after.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }));
    while !cancelled_after.terminal_is_empty() {
        let _ = cancelled_after.close_step(1, usize::MAX);
    }
    assert_eq!(cancelled_after.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
    assert_eq!(cancelled_after.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
}

#[semio_framework_async_macros::async_test]
async fn retained_maximum_microturns_stay_below_eight_milliseconds() {
    let graph = graph_with_shape(EQUATION_MAX_NODES, EQUATION_MAX_EDGES);
    let snapshot = crate::equation_snapshot_with_state(graph, EquationGeometry::default());
    let ids = (0..EQUATION_MAX_DELETE_IDS).map(|index| format!("n{index}")).collect::<Vec<_>>();
    let mut operations = vec![json::object([("operation".to_string(), Value::from("deleteSelection")), ("nodeIds".to_string(), json::array(ids.iter().map(|id| Value::from(id.as_str()))))])];
    operations.resize(EQUATION_MAX_EDIT_OPERATIONS, json::object([]));
    let command = EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: json::to_string(&json::array(operations)) });
    let operation = retained_operation(23);
    let extent = equation_command_extent(&command, &snapshot).expect("maximum retained extent");
    let mut work = EquationRetainedCommandWork::new("nodeGraphEdit", equation_operation_identity("nodeGraphEdit", &operation), extent);
    let config = EquationConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    loop {
        let started = std::time::Instant::now();
        let step = work
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
            .expect("maximum retained turn");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Equation microturn exceeded 8 ms");
        if matches!(step, ArtifactCommandWorkStep::Complete(_)) {
            break;
        }
    }
    work.begin_close();
    while !work.terminal_is_empty() {
        let started = std::time::Instant::now();
        let _ = work.close_step(1, usize::MAX);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Equation close turn exceeded 8 ms");
    }
}
//#endregion 🔖️RetainedCommands

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_the_full_row_set_is_covered() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 7, "every EquationCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// kebab-cased command id, except for the two documented divergences: `setLocale` → `locale`
/// (an undeclared host-pushed command) and `setDocument` → `set-artifact` (the `app_commands!`
/// row's own `"setDocument" as "set-artifact" => set_artifact::SetArtifact` explicitly pins a
/// non-kebab wire keyword, matching `SetArtifact`'s own `#[dsl(keyword = "set-artifact")]`).
/// **Pre-existing bug, independently traced**: `git log -1 --date=iso -- 🎮️commands/🗿️set-artifact/
/// 🦀️.rs` shows `SetArtifact`'s explicit `set-artifact` keyword predates this ticket's
/// own edits to this file (which only touched `render`/`export_media`); this test's hardcoded
/// exception list simply never accounted for the second declared divergence. Fixed outright
/// per this ticket's own "trivial, safe, unambiguous" guidance rather than left unresolved.
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected = match id {
            "setLocale" => "locale".to_string(),
            "setDocument" => "set-artifact".to_string(),
            _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
        };
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<EquationCommand> {
    vec![
        EquationCommand::SetArtifact(set_artifact::SetArtifact { graph: crate::document_dsl::math_graph_to_dsl(&EquationGraph::default()), geometry: EquationGeometry::default() }),
        EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }),
        EquationCommand::SetDirected(set_directed::SetDirected { directed: true }),
        EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: r#"[{"operation":"addNode","x":12.0,"y":34.0}]"#.into() }),
        EquationCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: crate::EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 } }),
        EquationCommand::SetPoints(set_points::SetPoints { geometry: EquationGeometry::default() }),
        EquationCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
    ]
}

/// ⚖️ The row whose `Option` field makes `None`/`Some` distinct wire cases, pinned to the exact bytes
/// captured from the pre-merge `equation_protocol` crate (see the ticket's
/// `🧪️wire-baseline-before.txt`).
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(EquationCommand, &str, &str); 2] = [
        (EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "topo".into(), seed: None }), "set-algorithm algorithm=topo", "01010104746f706f01000600"),
        (EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }), "set-algorithm algorithm=bfs seed=a", "01010201610362667302000601010600"),
    ];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    // 🌱️ `AppDefinition` (`semio-framework-plugin`, framework-owned) has not itself gained
    // `ToValue` — `Debug` gives the same "does the manifest mention X" substring check without
    // needing `serde_json` for a framework type this batch does not own.
    let debug = format!("{:?}", create_equation_app());
    for id in [graph_window::MATH_PLAY_WINDOW_GRAPH, geometry_window::MATH_PLAY_WINDOW_GEOMETRY] {
        assert!(debug.contains(id), "window kind {id} missing from the manifest: {debug}");
    }
    assert!(debug.contains(edit::MATH_PLAY_MODE_EDIT), "mode missing from the manifest");
    assert!(debug.contains("computation.equation"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn equation_io_is_declared_on_the_manifest() {
    let app = create_equation_app();
    assert_eq!(app.io.artifact.id, "computation.equation");
    assert_eq!(app.io.ports.len(), 1);
    assert_eq!(app.io.ports[0].id, "result:out");
}

#[semio_framework_async_macros::async_test]
async fn create_equation_app_builds_a_definition_for_the_editor_role() {
    let def = create_equation_app();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, EQUATION_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<EquationPlayApp as ArtifactEditor>::DIALECT, EQUATION_DIALECT);
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::equation::testkit::render;
    let mut app = math_app().await;
    assert!(render(&mut app, "equation.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn command_surface_is_registry_clean() {
    let _app = math_app_with_registry().await;
}
//#endregion 🔖️CrossCutting

//#region 🔖️EquationIo
#[semio_framework_async_macros::async_test]
async fn equation_io_declares_result_out_with_the_computation_equation_kind() {
    let io = equation_io();
    assert_eq!(io.document_schema, "semio.equation/v1");
    assert_eq!(io.artifact.id, "computation.equation");
    assert_eq!(io.ports.len(), 1);
    let port = &io.ports[0];
    assert_eq!(port.id, "result:out");
    assert_eq!(port.kind_id.as_deref(), Some("computation.equation"));
    assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
    assert!(!port.required);
}
//#endregion 🔖️EquationIo

//#region 🔖️GraphAlgorithms
#[semio_framework_async_macros::async_test]
async fn topo_algorithm_overlay_orders_dag_nodes() {
    let graph = EquationGraph::default();
    let overlay = algorithm_overlay(&graph);
    assert!(overlay.get("a").unwrap().starts_with(" #0"));
    assert!(overlay.get("d").unwrap().starts_with(" #"));
}

#[semio_framework_async_macros::async_test]
async fn components_algorithm_overlay_groups_disconnected_node() {
    use crate::EquationNode;
    let mut graph = EquationGraph { algorithm: "components".into(), ..EquationGraph::default() };
    graph.nodes.push(EquationNode { id: "z".into(), label: "Z".into(), x: 0.0, y: 0.0 });
    let overlay = algorithm_overlay(&graph);
    assert_ne!(overlay.get("a"), overlay.get("z"));
}

#[semio_framework_async_macros::async_test]
async fn bfs_algorithm_overlay_reports_hop_distance() {
    let graph = EquationGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..EquationGraph::default() };
    let overlay = algorithm_overlay(&graph);
    assert_eq!(overlay.get("a").unwrap(), " d0");
    assert_eq!(overlay.get("b").unwrap(), " d1");
}

#[semio_framework_async_macros::async_test]
async fn workflow_json_round_trips_node_count() {
    let graph = EquationGraph::default();
    let (nodes, edges) = workflow_json(&graph);
    assert_eq!(nodes.len(), graph.nodes.len());
    assert_eq!(edges.len(), graph.edges.len());
}
//#endregion 🔖️GraphAlgorithms

//#region 🔖️Geometry
#[semio_framework_async_macros::async_test]
async fn geometry_layers_include_hull_and_centroid() {
    let geometry = EquationGeometry::default();
    let layers_json = geometry_layers_json(&geometry);
    assert!(layers_json.contains("\"hull\""));
    assert!(layers_json.contains("\"centroid\""));
}
//#endregion 🔖️Geometry
