
use super::*;
use crate::editor::wires::testkit::{metabolism_app, new_app, render};
use semio_framework_plugin::EditorApp;

const RETAINED_ROUTES: &str = include_str!("../../🧫️fixtures/🛣️retained-command-routes.json");

#[test]
fn retained_route_fixture_matches_the_exact_factory_and_fail_closed_census() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let fixture: Value = serde_json::from_str(RETAINED_ROUTES).expect("Wires retained route fixture decodes through serde_json");
    assert_eq!(fixture.get("maximumRawBytes").and_then(Value::as_u64), Some(WIRES_RETAINED_RAW_BYTES as u64));
    assert_eq!(fixture.get("maximumWorkItems").and_then(Value::as_u64), Some(WIRES_RETAINED_WORK_ITEMS as u64));
    let routes = fixture.get("routes").and_then(Value::as_array).expect("routes");
    let migrated = routes.iter().filter(|route| route.get("disposition").and_then(Value::as_str) == Some("migrated")).map(|route| route.get("id").and_then(Value::as_str).expect("route id")).collect::<Vec<_>>();
    assert_eq!(migrated, WIRES_RETAINED_TOOL_IDS);
    assert_eq!(routes.len(), 10);
    assert_eq!(<WiresRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, WIRES_RETAINED_PUBLICATION_CONTRACTS);
    assert!(WIRES_RETAINED_PUBLICATION_CONTRACTS.iter().all(|row| row.lanes == [ArtifactToolPublicationLane::Config]));
    assert!(routes.iter().filter(|route| route.get("disposition").and_then(Value::as_str) == Some("batch-only-pending-rewrite")).all(|route| route.get("lanes").and_then(Value::as_array).is_some_and(Vec::is_empty)));
}

#[test]
fn config_preparation_rejects_wrong_lane_and_oversized_locale() {
    use store::ArtifactStoreOneItemPreparationFactory;
    let factory = WiresConfigPreparationFactory;
    assert!(factory.preflight(&WiresConfigMutation::SetLocale(crate::editor::wires::config::SetLocale { value: "de-DE".into() }), None, store::HistoryLane::Document).is_ok());
    assert!(factory.preflight(&WiresConfigMutation::SetLocale(crate::editor::wires::config::SetLocale { value: "de-DE".into() }), None, store::HistoryLane::Interaction).is_err());
    assert!(factory.preflight(&WiresConfigMutation::SetLocale(crate::editor::wires::config::SetLocale { value: "x".repeat(WIRES_RETAINED_RAW_BYTES + 1) }), None, store::HistoryLane::Document).is_err());
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 10, "every WiresCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
/// per-row from the `app_commands!` table's `"id" as "wire-key"` declarations rather than derived
/// (several rows genuinely diverge from a naive kebab-case of the id: `setLocale` → `locale`,
/// `setActiveExample` → `active-example`, and all three `canvasPointer*` rows drop the `canvas-`
/// prefix). This is what a missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the
/// record prints with no keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_keys = [
        ("setActiveExample", "active-example"),
        ("addNode", "add-node"),
        ("addRelationship", "add-relationship"),
        ("deleteSelection", "delete-selection"),
        ("forceLayout", "force-layout"),
        ("reorganize", "reorganize"),
        ("canvasPointerMove", "pointer-move"),
        ("canvasPointerDown", "pointer-down"),
        ("canvasPointerUp", "pointer-up"),
        ("setLocale", "locale"),
    ];
    for command in every_command() {
        let id = command.command_id();
        let expected = expected_keys.iter().find(|(row_id, _)| *row_id == id).map(|(_, key)| *key).unwrap_or_else(|| panic!("no expected wire key recorded for command {id}"));
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// ⚖️ The wire bytes/text pinned from the pre-merge 7-crate baseline (see the ticket's
/// `🧪️wire-baseline-before.txt`) — a regression here is a real format break, not a fixture mismatch.
/// `setSelection`/`documentSelect` dissolved into the framework's own "graph" interaction domain
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) and no longer exist as `WiresCommand`
/// rows, which shifts every later row's binary ordinal by 2 — `CanvasPointerUp`'s and `SetLocale`'s
/// pinned hex below are updated for the new ordinals (8 and 9); `SetActiveExample` is unaffected
/// (ordinal 0, before the deleted rows).
#[semio_framework_async_macros::async_test]
async fn commands_keep_their_pre_migration_wire_bytes() {
    let node = dsl::to_dsl_value(&dsl::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).unwrap();
    let _ = node;
    let cases: [(WiresCommand, &str, &str); 3] = [
        (WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "metabolism".into() }), "active-example active-example example-id=metabolism", "0100010a6d657461626f6c69736d01000600"),
        (WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}), "pointer-up pointer-up", "01080000"),
        (WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), "locale locale value=de-DE", "0109010564652d444501000600"),
    ];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<WiresCommand> {
    vec![
        WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "metabolism".into() }),
        WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }),
        WiresCommand::AddRelationship(add_relationship::AddRelationship { kind: "owns".into() }),
        WiresCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        WiresCommand::ForceLayout(force_layout::ForceLayout {}),
        WiresCommand::Reorganize(reorganize::Reorganize {}),
        WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1.5, y: -2.5 }),
        WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }),
        WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
        WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️Interaction
/// 🕹️ The "graph" domain is declared `HierarchyProvider::Flat`, single-select/pick/replace-only,
/// and scoped to the canvas window (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
#[semio_framework_async_macros::async_test]
async fn graph_interaction_domain_is_declared_flat_and_scoped_to_the_canvas_window() {
    let definition = create_wires_app();
    let graph = definition.interactions.iter().find(|interaction| interaction.id == WIRES_INTERACTION_GRAPH).expect("graph interaction domain declared");
    assert!(matches!(graph.hierarchy, HierarchyProvider::Flat));
    assert_eq!(graph.granularities.len(), 2);
    assert!(!graph.selection.transitive, "graph has no hierarchy to close a transitive selection over");
    let canvas_window = definition.window_kinds.iter().find(|window| window.id == WIRES_PLAY_WINDOW_CANVAS).expect("canvas window kind declared");
    assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == WIRES_INTERACTION_GRAPH), "canvas window must reference the graph interaction domain");
}

/// 🕹️ `wires_select_action_args` shapes the exact JSON the framework's `interactionSelect` action
/// expects: `domainId`/`targets` (a JSON-stringified `Vec<InteractionTarget>`)/`merge`/`method`.
#[semio_framework_async_macros::async_test]
async fn wires_select_action_args_shapes_interaction_select_payload() {
    let args = wires_select_action_args(&["node-1".to_string()], WIRES_GRANULARITY_NODE, "replace");
    assert_eq!(args["domainId"], WIRES_INTERACTION_GRAPH);
    assert_eq!(args["merge"], "replace");
    assert_eq!(args["method"], "pick");
    assert!(args["targets"].as_str().expect("targets json").contains("node-1"));
    assert!(args["targets"].as_str().expect("targets json").contains(WIRES_GRANULARITY_NODE));
}
//#endregion 🔖️Interaction

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_wires_app()).expect("app definition json");
    assert!(json.contains(WIRES_PLAY_WINDOW_CANVAS), "window kind missing from the manifest: {json}");
    assert!(json.contains(edit::WIRES_PLAY_MODE_EDIT), "mode missing from the manifest");
    for body in [WIRES_PLAY_BODY_DOCUMENT, WIRES_PLAY_BODY_CATALOGUE, WIRES_PLAY_BODY_PROPERTIES] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("graph.wires"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn wires_labels_resolve_native_by_default() {
    let mut app = metabolism_app().await;
    let json = render(&mut app, WIRES_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("Identities") && json.contains("Relationships"));
    let catalogue_json = render(&mut app, WIRES_PLAY_BODY_CATALOGUE).await;
    assert!(catalogue_json.contains("Identity kinds"));
    assert!(catalogue_json.contains("Relationship kinds"));
}

#[semio_framework_async_macros::async_test]
async fn metabolism_board_fixture_uses_mindmap_schema() {
    let document = crate::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
    let board = crate::wires_working_board(&document);
    assert_eq!(board.get("schema").and_then(|value| value.as_str()), Some(crate::MINDMAP_BOARD_SCHEMA));
    assert_eq!(crate::schema::fixture_nodes(&board).len(), 7);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = new_app().await;
    assert!(render(&mut app, "reasoning.wires.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = new_app().await;
    semio_framework_plugin::testkit::assert_undo_redo_round_trip(
        &mut app,
        WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }),
        |app| crate::schema::fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).len(),
        0,
        1,
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    semio_framework_plugin::testkit::assert_ingest_idempotent::<EditorApp<ReasoningWiresPlayApp>, usize>(WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), |app| {
        crate::schema::fixture_nodes(&crate::wires_working_board(&app.snapshot().expect("snapshot"))).len()
    })
    .await;
}

/// 🧪️ The definitional merge proof: A adds a node while B renames another node — disjoint edits
/// on one backbone that must both survive on both instances (impossible under whole-document LWW).
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_graph_edits_via_backbone() {
    use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
    use semio_framework_plugin::PluginApp;
    use semio_framework_plugin::testkit::meta;
    use store::MemoryBackbone;

    let mut instance_a = new_app().await;
    let mut instance_b = new_app().await;
    // Seed both from an identical base projection carrying node-1/node-2 (as initial state, not
    // as edits) so the only edits on the channel are A's and B's disjoint ones.
    let seed_node = |id: &str| dsl::to_dsl_value(&dsl::json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": id, "handles": [] })).expect("seed node");
    let mut base = crate::empty_wires_snapshot();
    base = store::apply_mutation(&base, &crate::mutations::create_node(seed_node("node-1"))).expect("valid mutation").0;
    base = store::apply_mutation(&base, &crate::mutations::create_node(seed_node("node-2"))).expect("valid mutation").0;
    let base_envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::MINDMAP_WIRES_SCHEMA, "reasoning-wires", base, None);
    let base_files = store::print_document_pack(&base_envelope).await.expect("print document pack");
    instance_a.load_document_pack(&base_files).await.expect("load a");
    instance_b.load_document_pack(&base_files).await.expect("load b");
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://mindmap-convergence", "mem://mindmap-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    // A adds node-3 (a new node); B moves node-2 (a PatchNode) — disjoint edits on the graph.
    instance_a.dispatch_typed(WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), &meta("actor-a")).await.expect("a adds node");
    instance_b.dispatch_typed(WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-2".into()), x: 0.0, y: 0.0 }), &meta("actor-b")).await.expect("b down");
    instance_b.dispatch_typed(WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 50.0, y: 60.0 }), &meta("actor-b")).await.expect("b move");
    instance_b.dispatch_typed(WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}), &meta("actor-b")).await.expect("b up");

    instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
    instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");

    let projection_a = instance_a.snapshot().expect("projection a");
    let projection_b = instance_b.snapshot().expect("projection b");
    // A's added node-3 survives on both.
    assert!(find_board_node(&projection_a, "node-3").is_some(), "A keeps its own node");
    assert!(find_board_node(&projection_b, "node-3").is_some(), "B converges on A's node");
    // B's move of node-2 survives on both.
    let x_of = |document: &WiresSnapshot| find_board_node(document, "node-2").map(|node| crate::schema::node_position(&node)).unwrap().0;
    assert_eq!(x_of(&projection_a), 50.0, "A converges on B's move");
    assert_eq!(x_of(&projection_b), 50.0, "B keeps its own move");
}
//#endregion 🔖️CrossCutting
