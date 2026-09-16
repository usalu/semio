use super::*;
use crate::editor::wires::commands::{add_node, canvas_pointer_move, canvas_pointer_up};
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{artifact_app_laws, PluginApp, INTERACTION_SELECT_ACTION_ID};

#[semio_framework_async_macros::async_test]
async fn pointer_drag_translates_node_by_screen_delta() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 100.0, y: 100.0 })).await;
    dispatch(&mut app, WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 140.0, y: 130.0, samples: Vec::new() })).await;
    let node = find_board_node(&app.snapshot().expect("snapshot"), "node-1").expect("node-1").clone();
    assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(40.0));
    assert_eq!(node.get("y").and_then(|value| value.as_f64()), Some(30.0));
    dispatch(&mut app, WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: false })).await;
    // A coalesced drag collapses to a single undo step restoring the origin.
    app.handle_action("undo", None, &artifact_app_laws::meta("local")).await.expect("undo");
    let node = find_board_node(&app.snapshot().expect("snapshot"), "node-1").expect("node-1").clone();
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
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🖱️pointer-move.json")).expect("pointer-move fixture");
    let mut app = crate::editor::wires::unit_tests::context::app_with_registry().await;
    app.bind_instance_id(1).await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "left".into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }], ..Default::default() };
    let left = view.for_window_instance("left").expect("canvas window instance");
    let result: Result<(), String> = async {
        let mut seed = crate::empty_wires_snapshot();
        seed.content = crate::wires_content_child_with_owner(vec![dsl::DslValue::from(&vectors["initialNode"])], Vec::new());
        let envelope = store::create_document_envelope::<crate::WiresSnapshot, crate::WiresMutation>(crate::MINDMAP_WIRES_SCHEMA, "reasoning-wires", seed, None);
        let pack = store::print_document_pack(&envelope).await.map_err(|error| format!("{error:?}"))?;
        app.load_document_pack(&pack).await.map_err(|error| format!("{error:?}"))?;
        crate::editor::wires::unit_tests::context::retire_envelope(envelope);
        let meta = ActionMeta { view_state: Some(left.clone()), ..artifact_app_laws::meta("inline-pick") };
        app.dispatch_typed(WiresCommand::NodeGraphViewport(NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }), &meta).await.map_err(|error| format!("{error:?}"))?;
        artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        let admitted = app.dispatch_typed(WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }), &meta).await.map_err(|error| format!("{error:?}"))?;
        if !admitted.mutations.is_empty() {
            return Err("a pointer down never mutates the document directly".into());
        }
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
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
    artifact_app_laws::close_registered_fixture_app(&mut app);
    result.expect("inline pick");
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_on_empty_space_requests_no_select_effect() {
    let mut app = new_app().await;
    let result = dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: None, x: 0.0, y: 0.0 })).await;
    assert!(result.requested_effects.is_empty());
    assert!(result.mutations.is_empty());
}
