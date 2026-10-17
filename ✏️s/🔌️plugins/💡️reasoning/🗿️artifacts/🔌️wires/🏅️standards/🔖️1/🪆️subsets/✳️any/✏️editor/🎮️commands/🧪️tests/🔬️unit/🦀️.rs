//! 🧪️ Aggregated unit tests of the `commands` leaves, one module per leaf.

mod add_node {
use super::super::add_node::*;
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;
use crate::schema::{board_node, board_snapshot_nodes};

#[semio_framework_async_macros::async_test]
async fn add_node_appends_and_selects() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(AddNode { kind: "identity".into() })).await;
    let board = crate::editor::wires::unit_tests::context::board(&app);
    assert_eq!(board_snapshot_nodes(&board).len(), 1);
    assert!(board_node(&board, "node-1").is_some());
}
}

mod add_relationship {
use crate::schema::entity_id;
use super::super::add_relationship::*;
use crate::editor::wires::commands::add_node;
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;
use crate::schema::board_snapshot_edges;

/// 🔗️ `addRelationship` connects the NAMED endpoints: two freshly added nodes, related `node-2 → node-1`
/// (the reverse of creation order, so a hardcoded `node-1 → node-2` cannot pass).
#[semio_framework_async_macros::async_test]
async fn add_relationship_connects_the_named_nodes_and_selects_the_edge() {
    let mut app = new_app().await;
    for _ in 0..2 {
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    }
    dispatch(&mut app, WiresCommand::AddRelationship(AddRelationship { kind: "owns".into(), source_id: "node-2".into(), target_id: "node-1".into() })).await;
    let board = crate::editor::wires::unit_tests::context::board(&app);
    let edges = board_snapshot_edges(&board);
    assert_eq!(edges.len(), 1);
    assert_eq!(entity_id(&edges[0], "source"), Some("node-2"));
    assert_eq!(entity_id(&edges[0], "target"), Some("node-1"));
    assert_eq!(entity_id(&edges[0], "edgeKind"), Some("wires.owns"));
}

/// 🛑️ Missing, unknown and identical endpoints are refused by name — never a silent success and never
/// a guessed pair of nodes.
#[semio_framework_async_macros::async_test]
async fn add_relationship_refuses_missing_unknown_and_identical_endpoints_by_name() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    for (payload, code) in [
        (AddRelationship { kind: "owns".into(), source_id: String::new(), target_id: String::new() }, "wires.relationship.endpoints-missing"),
        (AddRelationship { kind: "owns".into(), source_id: "node-1".into(), target_id: String::new() }, "wires.relationship.endpoint-missing"),
        (AddRelationship { kind: "owns".into(), source_id: "node-1".into(), target_id: "node-9".into() }, "mutation.target-missing"),
        (AddRelationship { kind: "owns".into(), source_id: "node-1".into(), target_id: "node-1".into() }, "wires.relationship.self"),
    ] {
        app.dispatch_typed(WiresCommand::AddRelationship(payload), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("admitted");
        let fault = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.err().expect("refused");
        assert!(fault.message.contains(code) || fault.code.0 == code, "expected {code}, got {fault:?}");
    }
    let board = crate::editor::wires::unit_tests::context::board(&app);
    assert!(board_snapshot_edges(&board).is_empty(), "no refused invocation wrote an edge");
}
}

mod canvas_pointer_down {
use super::super::canvas_pointer_down::*;
use crate::editor::wires::commands::{add_node, canvas_pointer_move, canvas_pointer_up};
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;
use crate::schema::board_node;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{artifact_app_laws, PluginApp, INTERACTION_SELECT_ACTION_ID};

/// 🖱️ A drag is a CANVAS gesture: `WiresWindowDragWork::extent` measures it against the dispatching
/// canvas window's transient and config owners, so the gesture carries that window as its view state
/// and one `nodeGraphViewport` mints the window config first (the recipe of
/// `pointer_down_selects_the_hit_node_inline`). The move is a window-transient preview; the release
/// publishes the ONE durable `move-node`, and one settled framework undo restores the origin.
#[semio_framework_async_macros::async_test]
async fn pointer_drag_translates_node_by_screen_delta() {
    use crate::editor::wires::commands::node_graph_viewport::NodeGraphViewport;
    use crate::editor::wires::unit_tests::context::settle;
    use crate::editor::wires::WIRES_PLAY_WINDOW_CANVAS;
    use semio_framework_plugin::{ActionMeta, ViewModel, ViewWindowInstance};
    let mut app = new_app().await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "left".into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let meta = ActionMeta { view_state: view.for_window_instance("left"), ..artifact_app_laws::meta("local") };
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    for command in [
        WiresCommand::NodeGraphViewport(NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }),
        WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 100.0, y: 100.0 }),
        WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 140.0, y: 130.0, samples: Vec::new() }),
        WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: false }),
    ] {
        app.dispatch_typed(command, &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("dispatch");
        settle(&mut app).await;
    }
    let node = board_node(&crate::editor::wires::unit_tests::context::board(&app), "node-1").expect("node-1").clone();
    assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(40.0));
    assert_eq!(node.get("y").and_then(|value| value.as_f64()), Some(30.0));
    artifact_app_laws::settle_history_verb(&mut *app, "undo", artifact_app_laws::meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    let node = board_node(&crate::editor::wires::unit_tests::context::board(&app), "node-1").expect("node-1").clone();
    assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(0.0));
}

/// 🕹️ A hit emits `interactionSelect` for the "graph" domain's "node" granularity
/// (`Effect::DispatchAction`) instead of mutating config directly — and since ticket
/// 26/09/16/INPUT-CAUSALITY-LEDGER §2 C the typed-operation ladder folds that verb INLINE, so the
/// effect never reaches the host and the witness is the selection snapshot once the retained
/// publication settles. The recipe is `🫧️transient`'s: a seeded document (`addNode` is still
/// `BatchOnlyPendingRewrite`, so no node can be added by dispatch), a `ViewModel` naming the canvas
/// window, one `NodeGraphViewport` to mint the window config the gesture's `extent` requires, and
/// the host's settle protocol. Fails-before: the effect rode the receipt's `effects` to the host
/// and the selection landed one guest round trip later.
#[semio_framework_async_macros::async_test]
async fn pointer_down_selects_the_hit_node_inline() {
    use crate::editor::wires::commands::node_graph_viewport::NodeGraphViewport;
    use crate::editor::wires::{WIRES_INTERACTION_GRAPH, WIRES_PLAY_WINDOW_CANVAS};
    use semio_framework_plugin::{ActionMeta, ViewModel, ViewWindowInstance};
    let vectors: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../🧪️tests/🧫️fixtures/🖱️pointer-move.json")).expect("pointer-move fixture");
    let mut app = crate::editor::wires::unit_tests::context::app_with_registry().await;
    app.bind_instance_id(1).await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "left".into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let left = view.for_window_instance("left").expect("canvas window instance");
    let result: Result<(), String> = async {
        let node = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&vectors["initialNode"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral full graph node");
        crate::editor::wires::unit_tests::context::load_graph_snapshot(&mut app, vec![node], Vec::new()).await.map_err(|error| format!("{error:?}"))?;
        let meta = ActionMeta { view_state: Some(left.clone()), ..artifact_app_laws::meta("inline-pick") };
        app.dispatch_typed(WiresCommand::NodeGraphViewport(NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        artifact_app_laws::settle_registered_typed_operation(&mut *app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        let admitted = app.dispatch_typed(WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        if !admitted.mutations.is_empty() {
            return Err("a pointer down never mutates the document directly".into());
        }
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut *app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        if receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)) {
            return Err(format!("interactionSelect is folded in-reactor, never handed to the host: {:?}", receipt.effects));
        }
        let selected = app.interaction_state().await.selection.get(WIRES_INTERACTION_GRAPH).map(|selection| (selection.granularity.clone(), selection.ids.clone()));
        if selected != Some(("node".to_string(), vec!["node-1".to_string()])) {
            return Err(format!("the hit node is selected inside the carrying operation, got {selected:?}"));
        }
        Ok(())
    }
    .await;
    app.close();
    result.expect("inline pick");
}

/// 🖱️ A press on empty canvas clears the drag: no document publication and no `interactionSelect`.
#[semio_framework_async_macros::async_test]
async fn pointer_down_on_empty_space_requests_no_select_effect() {
    use crate::editor::wires::commands::node_graph_viewport::NodeGraphViewport;
    use crate::editor::wires::unit_tests::context::settle;
    use crate::editor::wires::{WIRES_INTERACTION_GRAPH, WIRES_PLAY_WINDOW_CANVAS};
    use semio_framework_plugin::{ActionMeta, ViewModel, ViewWindowInstance};
    let mut app = new_app().await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "left".into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let meta = ActionMeta { view_state: view.for_window_instance("left"), ..artifact_app_laws::meta("local") };
    app.dispatch_typed(WiresCommand::NodeGraphViewport(NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("viewport");
    settle(&mut app).await;
    app.dispatch_typed(WiresCommand::CanvasPointerDown(CanvasPointerDown { id: None, x: 0.0, y: 0.0 }), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("pointer down");
    let receipt = settle(&mut app).await;
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "an empty-space press never publishes the document");
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)));
    assert!(app.interaction_state().await.selection.get(WIRES_INTERACTION_GRAPH).is_none_or(|selection| selection.ids.is_empty()));
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod set_active_example {
use super::super::set_active_example::*;
use crate::editor::wires::unit_tests::context::{dispatch_receipt, metabolism_app, new_app};
use crate::editor::wires::WiresCommand;
use crate::schema::board_snapshot_nodes;

/// 🧬️ Whole-document replace is not an in-history mutation (a whole-snapshot variant is banned
/// outright), so `setActiveExample` now surfaces as a `Effect::LoadDocument` carrying the
/// example document's pack bytes rather than an `artifact_mutations` entry — `dispatch`'s
/// in-process harness never applies `effects` to its own store, so this asserts on the emitted
/// effect directly (mirrors `🎮️commands/📚️example`-style facets elsewhere in this ticket).
///
/// 🔁️ Read through `dispatch_receipt`: the publication receipt belongs to the ONE settle that drains
/// the dispatched operation. `dispatch(..)` + a second `settle(..)` answered an empty receipt (nothing
/// pending any more) and this law reported "setActiveExample must emit a LoadDocument effect" against
/// a command that emitted one all along.
#[semio_framework_async_macros::async_test]
async fn set_active_example_metabolism_loads_seven_nodes() {
    use semio_framework_plugin::Effect;
    let mut app = new_app().await;
    let (result, receipt) = dispatch_receipt(&mut app, WiresCommand::SetActiveExample(SetActiveExample { example_id: WIRES_PLAY_EXAMPLE_METABOLISM_ID.into() })).await;
    assert!(result.mutations.is_empty(), "setActiveExample replaces the whole document via an effect, not in-history mutations");
    let Effect::LoadDocument { pack, .. } = receipt.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let document = <crate::WiresSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    let child_pack = crate::genesis_wires_child_pack(&document, crate::WIRES_CONTENT_SLOT, &document.content.child_id).expect("declared example child pack");
    let content = <crate::SemioGraphSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&child_pack).expect("full example graph child");
    let board = crate::wires_composed(&document, &content).board;
    assert_eq!(board_snapshot_nodes(&board).len(), 7);
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_unknown_id_loads_empty_document() {
    use semio_framework_plugin::Effect;
    let mut app = metabolism_app().await;
    let (_, receipt) = dispatch_receipt(&mut app, WiresCommand::SetActiveExample(SetActiveExample { example_id: "nope".into() })).await;
    let Effect::LoadDocument { pack, .. } = receipt.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let document = <crate::WiresSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    let child_pack = crate::genesis_wires_child_pack(&document, crate::WIRES_CONTENT_SLOT, &document.content.child_id).expect("declared example child pack");
    let content = <crate::SemioGraphSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&child_pack).expect("full example graph child");
    let board = crate::wires_composed(&document, &content).board;
    assert!(board_snapshot_nodes(&board).is_empty());
}
}
