use super::*;

#[semio_framework_async_macros::async_test]
async fn wires_pointer_move_uses_only_the_captured_canvas_and_publishes_document_positions() {
    use crate::editor::wires::commands::{canvas_pointer_down::CanvasPointerDown, canvas_pointer_move::CanvasPointerMove, canvas_pointer_up::CanvasPointerUp, delete_selection::DeleteSelection};
    use crate::editor::wires::{create_wires_app, ReasoningWiresPlayApp, WiresCommand, WIRES_PLAY_BODY_COMPOSITE, WIRES_PLAY_WINDOW_CANVAS};
    use semio_framework_plugin::{testkit, ActionMeta, App, Canvas2dScene, EditorApp, InteractionTarget, PluginApp, ViewModel, ViewWindowInstance, INTERACTION_SELECT_ACTION_ID};
    fn manifest() -> App {
        App { definition: create_wires_app(), examples: Vec::new() }
    }
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧫️fixtures/🖱️pointer-move.json")).unwrap();
    let mut app = testkit::new_app_with_registry::<EditorApp<ReasoningWiresPlayApp>>(manifest).await;
    app.bind_instance_id(1).await;
    let view = ViewModel { window_instances: ["left", "right"].into_iter().map(|id| ViewWindowInstance { id: id.into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }).collect(), ..Default::default() };
    let result: Result<(), String> = async {
        let mut seed = crate::empty_wires_snapshot();
        seed.content = crate::wires_content_child_with_owner(vec![dsl::DslValue::from(&vectors["initialNode"])], Vec::new());
        let envelope = store::create_document_envelope::<crate::WiresSnapshot, crate::WiresMutation>(crate::MINDMAP_WIRES_SCHEMA, "reasoning-wires", seed, None);
        let files = store::print_document_pack(&envelope).await;
        let mut retirement = store::retire_document_envelope(
            envelope,
            std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<crate::WiresSnapshot>::default()),
            std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<crate::WiresMutation>::default()),
        );
        for _ in 0..100_000 {
            if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?, store::SnapshotRetirementStep::Complete) {
                break;
            }
        }
        if !retirement.terminal_is_empty() {
            return Err("seed envelope did not finish bounded retirement".into());
        }
        let files = files.map_err(|error| format!("{error:?}"))?;
        app.load_document_pack(&files).await.map_err(|error| format!("{error:?}"))?;
        let config = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        for row in vectors["steps"].as_array().ok_or("missing gesture steps")? {
            let x = row["x"].as_f64().ok_or("missing x")?;
            let y = row["y"].as_f64().ok_or("missing y")?;
            let command = match row["command"].as_str().ok_or("missing command")? {
                "down" => WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some(vectors["node"].as_str().ok_or("missing node")?.into()), x, y }),
                "move" => WiresCommand::CanvasPointerMove(CanvasPointerMove { x, y }),
                "up" => WiresCommand::CanvasPointerUp(CanvasPointerUp {}),
                _ => return Err("unknown gesture command".into()),
            };
            let window = view.for_window_instance(row["window"].as_str().ok_or("missing window")?).ok_or("unknown window")?;
            app.dispatch_typed(command, &ActionMeta { view_state: Some(window), ..testkit::meta("gesture") }).await.map_err(|error| format!("{error:?}"))?;
            let receipt = testkit::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
            let document_publication = receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact);
            if document_publication != row["documentPublication"].as_bool().ok_or("missing document publication")? {
                return Err(format!("gesture publication lanes disagree with neutral fixture: {} {:?}", row["command"], receipt.lanes));
            }
            let snapshot = app.snapshot().map_err(|error| format!("{error:?}"))?;
            let node = crate::standards::v1::subsets::any::schema::inferences::find_board_node(&snapshot, vectors["node"].as_str().ok_or("missing node")?).ok_or("created node is absent")?;
            let (actual_x, actual_y) = crate::schema::node_position(&node);
            if serde_json::json!([actual_x, actual_y]) != row["position"] {
                return Err(format!("gesture position disagrees with neutral fixture: ({actual_x}, {actual_y}) != {}", row["position"]));
            }
            for (index, id) in ["left", "right"].into_iter().enumerate() {
                let window = view.for_window_instance(id).ok_or("window instance absent")?;
                let transient = app.window_transient_snapshot(&window).map_err(|error| format!("{error:?}"))?.ok_or("window transient absent")?;
                if Some(transient.generation()) != row["generations"][index].as_u64() {
                    return Err(format!("gesture generation crossed window boundary: {id}"));
                }
                let state = transient.get::<WiresCanvasTransientOwner>().ok_or("typed window transient absent")?;
                let preview = &row["previews"][id];
                if preview.is_null() {
                    if state.drag_node_id.is_some() {
                        return Err(format!("gesture preview remained captured in {id}"));
                    }
                } else if state.drag_node_id.as_deref() != vectors["node"].as_str() {
                    return Err(format!("gesture preview lost its captured node in {id}"));
                }
                let tree = app.render(WIRES_PLAY_BODY_COMPOSITE, None, &window).await.map_err(|error| format!("{error:?}"))?;
                let projection = testkit::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                let scene = testkit::decode_fixture_scene::<Canvas2dScene>(&projection).map_err(str::to_string)?;
                let layers: serde_json::Value = serde_json::from_str(&scene.layers_json).map_err(|error| error.to_string())?;
                let rendered = layers.as_array().and_then(|layers| layers.iter().find(|layer| layer["id"].as_str() == vectors["node"].as_str())).ok_or("rendered gesture node absent")?;
                let expected = if preview.is_null() { &row["position"] } else { preview };
                let actual = serde_json::json!([rendered["x"].as_f64().ok_or("rendered x absent")?, rendered["y"].as_f64().ok_or("rendered y absent")?]);
                if actual != *expected {
                    return Err(format!("gesture preview crossed document/window ownership in {id}: {actual} != {expected}"));
                }
            }
        }
        let released = app.snapshot().map_err(|error| format!("{error:?}"))?;
        let released_scene = released.content.require_local_owner::<crate::WiresWorkingScene>().map_err(|error| error.to_string())?;
        let first_party_handle = crate::wires_content_child_handle(&released_scene.nodes, &released_scene.edges);
        if released.content.child_id != first_party_handle.child_id || released.content.target != first_party_handle.target {
            return Err("bounded release writer disagrees with the first-party graph content handle".into());
        }
        app.handle_action("undo", None, &ActionMeta { view_state: view.for_window_instance("left"), ..testkit::meta("gesture-undo") }).await.map_err(|error| format!("{error:?}"))?;
        let undone = app.snapshot().map_err(|error| format!("{error:?}"))?;
        let node = crate::standards::v1::subsets::any::schema::inferences::find_board_node(&undone, vectors["node"].as_str().ok_or("missing node")?).ok_or("undone node is absent")?;
        if crate::schema::node_position(&node) != (0.0, 0.0) {
            return Err("one undo did not reverse the entire released drag".into());
        }
        let after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        if config.pack != after.pack || config.spr != after.spr {
            return Err("gesture persisted window state in app config".into());
        }
        let left = view.for_window_instance("left").ok_or("left window absent")?;
        for command in [WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 1.0, y: 1.0 }), WiresCommand::CanvasPointerMove(CanvasPointerMove { x: 5.0, y: 7.0 })] {
            app.dispatch_typed(command, &ActionMeta { view_state: Some(left.clone()), ..testkit::meta("missing-target-drag") }).await.map_err(|error| format!("{error:?}"))?;
            testkit::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        }
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: "node-1".into() }]).map_err(|error| error.to_string())?;
        app.handle_action(
            INTERACTION_SELECT_ACTION_ID,
            semio_framework_plugin::optional_json_to_dsl(Some(serde_json::json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" }))).as_ref(),
            &testkit::meta("missing-target-selection"),
        )
        .await
        .map_err(|error| format!("{error:?}"))?;
        app.dispatch_typed(WiresCommand::DeleteSelection(DeleteSelection {}), &ActionMeta { view_state: Some(left.clone()), ..testkit::meta("missing-target-delete") }).await.map_err(|error| format!("{error:?}"))?;
        testkit::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        app.dispatch_typed(WiresCommand::CanvasPointerUp(CanvasPointerUp {}), &ActionMeta { view_state: Some(left.clone()), ..testkit::meta("missing-target-release") }).await.map_err(|error| format!("{error:?}"))?;
        let missing_release = testkit::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        if missing_release.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact) {
            return Err("missing drag target published a durable move".into());
        }
        let cleared = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("missing-target transient absent")?;
        if cleared.get::<WiresCanvasTransientOwner>() != Some(&WiresCanvasTransient::default()) {
            return Err("missing drag target did not clear its exact window preview".into());
        }
        Ok(())
    }
    .await;
    if let Err(error) = &result {
        eprintln!("[DEBUG] Wires pointer move runtime failure before close: {error}");
    }
    testkit::close_registered_fixture_app(&mut app);
    result.expect("captured-canvas document gesture");
    eprintln!("[DEBUG] Wires pointer move: five neutral gesture steps isolate exact canvas previews, publish once, and undo in one step");
}

#[semio_framework_async_macros::async_test]
async fn wires_pointer_move_document_replacement_clears_only_successful_reload_previews() {
    use crate::editor::wires::commands::{canvas_pointer_down::CanvasPointerDown, canvas_pointer_move::CanvasPointerMove, node_graph_viewport::NodeGraphViewport};
    use crate::editor::wires::{create_wires_app, ReasoningWiresPlayApp, WiresCommand, WIRES_PLAY_BODY_COMPOSITE, WIRES_PLAY_WINDOW_CANVAS};
    use semio_framework_plugin::{testkit, ActionMeta, App, Canvas2dScene, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    fn manifest() -> App {
        App { definition: create_wires_app(), examples: Vec::new() }
    }
    async fn dispatch_gesture(app: &mut VcsArtifactApp<EditorApp<ReasoningWiresPlayApp>>, command: WiresCommand, window: &ViewModel) -> Result<(), String> {
        app.dispatch_typed(command, &ActionMeta { view_state: Some(window.clone()), ..testkit::meta("reload-gesture") }).await.map_err(|error| format!("{error:?}"))?;
        testkit::settle_registered_typed_operation(app, 1).await.map_err(|error| format!("{error:?}"))?;
        Ok(())
    }
    async fn scene(app: &mut VcsArtifactApp<EditorApp<ReasoningWiresPlayApp>>, window: &ViewModel) -> Result<Canvas2dScene, String> {
        let tree = app.render(WIRES_PLAY_BODY_COMPOSITE, None, window).await.map_err(|error| format!("{error:?}"))?;
        let projection = testkit::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
        testkit::decode_fixture_scene(&projection).map_err(str::to_string)
    }
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧫️fixtures/🖱️pointer-move.json")).unwrap();
    let mut app = testkit::new_app_with_registry::<EditorApp<ReasoningWiresPlayApp>>(manifest).await;
    app.bind_instance_id(1).await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "left".into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }], ..Default::default() };
    let left = view.for_window_instance("left").unwrap();
    let result: Result<(), String> = async {
        let mut seed = crate::empty_wires_snapshot();
        seed.content = crate::wires_content_child_with_owner(vec![dsl::DslValue::from(&vectors["initialNode"])], Vec::new());
        let envelope = store::create_document_envelope::<crate::WiresSnapshot, crate::WiresMutation>(crate::MINDMAP_WIRES_SCHEMA, "reasoning-wires", seed, None);
        let pack = store::print_document_pack(&envelope).await.map_err(|error| format!("{error:?}"))?;
        let text = store::print_document_text(&envelope).await.map_err(|error| format!("{error:?}"))?;
        app.load_document_pack(&pack).await.map_err(|error| format!("{error:?}"))?;
        let mut retirement = store::retire_document_envelope(
            envelope,
            std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<crate::WiresSnapshot>::default()),
            std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<crate::WiresMutation>::default()),
        );
        for _ in 0..100_000 {
            if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?, store::SnapshotRetirementStep::Complete) {
                break;
            }
        }
        if !retirement.terminal_is_empty() {
            return Err("reload seed envelope did not retire".into());
        }
        app.dispatch_typed(
            WiresCommand::NodeGraphViewport(NodeGraphViewport { camera: crate::editor::wires::modes::edit::windows::canvas::config::WiresCanvasCamera { x: 12.0, y: -4.0, zoom: 2.0 } }),
            &ActionMeta { view_state: Some(left.clone()), ..testkit::meta("reload-camera") },
        )
        .await
        .map_err(|error| format!("{error:?}"))?;
        testkit::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        let config_generation = app.window_config_generation(&left).await.map_err(|error| format!("{error:?}"))?.ok_or("window config generation absent")?;
        dispatch_gesture(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }), &left).await?;
        dispatch_gesture(&mut app, WiresCommand::CanvasPointerMove(CanvasPointerMove { x: 16.0, y: 28.0 }), &left).await?;
        app.load_document_pack(&pack).await.map_err(|error| format!("{error:?}"))?;
        let cleared = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("cleared transient absent")?;
        if cleared.get::<WiresCanvasTransientOwner>() != Some(&WiresCanvasTransient::default()) {
            return Err("identical pack reload preserved an old drag preview".into());
        }
        let reloaded_scene = scene(&mut app, &left).await?;
        if (reloaded_scene.camera_x, reloaded_scene.camera_y, reloaded_scene.zoom) != (12.0, -4.0, 2.0) || app.window_config_generation(&left).await.map_err(|error| format!("{error:?}"))? != Some(config_generation) {
            return Err("document reload changed the concrete canvas camera".into());
        }
        dispatch_gesture(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 4.0, y: 5.0 }), &left).await?;
        dispatch_gesture(&mut app, WiresCommand::CanvasPointerMove(CanvasPointerMove { x: 7.0, y: 9.0 }), &left).await?;
        let preview = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.and_then(|snapshot| snapshot.get::<WiresCanvasTransientOwner>().cloned()).ok_or("active preview absent")?;
        let preview_generation = app.window_transient_generation(&left).map_err(|error| format!("{error:?}"))?.ok_or("preview generation absent")?;
        let mut malformed = pack.clone();
        malformed.pack.truncate(4);
        if app.load_document_pack(&malformed).await.is_ok() {
            return Err("malformed pack unexpectedly replaced the document".into());
        }
        let preserved = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("preserved transient absent")?;
        if preserved.get::<WiresCanvasTransientOwner>() != Some(&preview)
            || app.window_transient_generation(&left).map_err(|error| format!("{error:?}"))? != Some(preview_generation)
            || app.window_config_generation(&left).await.map_err(|error| format!("{error:?}"))? != Some(config_generation)
        {
            return Err("rejected pack changed preview or camera ownership".into());
        }
        app.load_document_text(&text).await.map_err(|error| format!("{error:?}"))?;
        let cleared = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("text-cleared transient absent")?;
        if cleared.get::<WiresCanvasTransientOwner>() != Some(&WiresCanvasTransient::default()) || app.window_config_generation(&left).await.map_err(|error| format!("{error:?}"))? != Some(config_generation) {
            return Err("text reload did not clear the preview while preserving the camera".into());
        }
        Ok(())
    }
    .await;
    if let Err(error) = &result {
        eprintln!("[DEBUG] Wires document replacement runtime failure before close: {error}");
    }
    testkit::close_registered_fixture_app(&mut app);
    result.expect("Wires document replacement ownership");
    eprintln!("[DEBUG] Wires reload: valid pack/text clear exact-window previews, malformed pack preserves them, and camera config survives");
}

#[semio_framework_async_macros::async_test]
async fn wires_pointer_move_pending_release_cancels_and_retires_with_small_or_zero_grants() {
    use crate::editor::wires::commands::{canvas_pointer_down::CanvasPointerDown, canvas_pointer_move::CanvasPointerMove, canvas_pointer_up::CanvasPointerUp};
    use crate::editor::wires::{create_wires_app, ReasoningWiresPlayApp, WiresCommand, WIRES_PLAY_WINDOW_CANVAS};
    use semio_framework_plugin::{testkit, ActionMeta, App, EditorApp, PluginApp, ViewModel, ViewWindowInstance};
    fn manifest() -> App {
        App { definition: create_wires_app(), examples: Vec::new() }
    }
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧫️fixtures/🖱️pointer-move.json")).unwrap();
    let mut app = testkit::new_app_with_registry::<EditorApp<ReasoningWiresPlayApp>>(manifest).await;
    app.bind_instance_id(1).await;
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "left".into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }], ..Default::default() };
    let left = view.for_window_instance("left").unwrap();
    let result: Result<(), String> = async {
        let mut seed = crate::empty_wires_snapshot();
        seed.content = crate::wires_content_child_with_owner(vec![dsl::DslValue::from(&vectors["initialNode"])], Vec::new());
        let envelope = store::create_document_envelope::<crate::WiresSnapshot, crate::WiresMutation>(crate::MINDMAP_WIRES_SCHEMA, "reasoning-wires", seed, None);
        let pack = store::print_document_pack(&envelope).await.map_err(|error| format!("{error:?}"))?;
        app.load_document_pack(&pack).await.map_err(|error| format!("{error:?}"))?;
        let mut retirement = store::retire_document_envelope(
            envelope,
            std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<crate::WiresSnapshot>::default()),
            std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<crate::WiresMutation>::default()),
        );
        for _ in 0..100_000 {
            if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?, store::SnapshotRetirementStep::Complete) {
                break;
            }
        }
        if !retirement.terminal_is_empty() {
            return Err("cancellation seed envelope did not retire".into());
        }
        for command in [WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 0.0, y: 0.0 }), WiresCommand::CanvasPointerMove(CanvasPointerMove { x: 11.0, y: 13.0 })] {
            app.dispatch_typed(command, &ActionMeta { view_state: Some(left.clone()), ..testkit::meta("cancel-release") }).await.map_err(|error| format!("{error:?}"))?;
            testkit::settle_registered_typed_operation(&mut app, 1).await.map_err(|error| format!("{error:?}"))?;
        }
        app.dispatch_typed(WiresCommand::CanvasPointerUp(CanvasPointerUp {}), &ActionMeta { view_state: Some(left), ..testkit::meta("cancel-release") }).await.map_err(|error| format!("{error:?}"))?;
        if !app.has_pending_typed_operations() {
            return Err("released drag did not enter retained publication".into());
        }
        app.maintenance_step(0, 0).map_err(|error| format!("{error:?}"))?;
        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
        if !app.has_pending_typed_operations() {
            return Err("zero grant unexpectedly completed retained publication".into());
        }
        app.maintenance_step(1, 1).map_err(|error| format!("{error:?}"))?;
        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
        if !app.has_pending_typed_operations() {
            return Err("one-byte grant unexpectedly completed retained publication".into());
        }
        Ok(())
    }
    .await;
    if let Err(error) = &result {
        eprintln!("[DEBUG] Wires cancellation runtime failure before close: {error}");
    }
    testkit::close_registered_fixture_app(&mut app);
    result.expect("bounded Wires publication cancellation");
    eprintln!("[DEBUG] Wires cancellation: zero/small grants leave release pending and app close reaches terminal-empty ownership");
}

//#region 🔖️ConfigTests

/// 🔁️ B1 dsl/pack round-trip law for `WiresCanvasTransient` — a non-default fixture exercising every field.
#[semio_framework_async_macros::async_test]
async fn wires_window_transient_dsl_pack_round_trip() {
    let config = WiresCanvasTransient { drag_node_id: Some("node-1".into()), drag_start_x: 1.5, drag_start_y: 2.5, drag_last_x: 12.5, drag_last_y: -7.25, drag_zoom: 2.0 };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}
//#endregion 🔖️ConfigTests

//#region 🔖️ConfigOperationTests
#[semio_framework_async_macros::async_test]
async fn window_transient_drag_op_text_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&WiresCanvasTransientMutation::SetDrag(SetDrag { node_id: Some("node-1".into()), start_x: 1.5, start_y: 2.5, last_x: 12.5, last_y: -7.25, zoom: 2.0 }));
    store::os_store::test_support::assert_op_line_round_trip(&WiresCanvasTransientMutation::SetDrag(SetDrag { node_id: None, start_x: 0.0, start_y: 0.0, last_x: 0.0, last_y: 0.0, zoom: 1.0 }));
}

/// ⏪️ `backwards()` returns the SAME variant re-addressed at the pre-op field value — a targeted,
/// in-kind inverse, not a whole-config replace.
#[semio_framework_async_macros::async_test]
async fn window_transient_backwards_restores_the_same_field_from_base() {
    let base = WiresCanvasTransient { drag_node_id: Some("node-1".into()), drag_last_x: 1.0, drag_last_y: 2.0, ..Default::default() };
    let forward = WiresCanvasTransientMutation::SetDrag(SetDrag { node_id: Some("node-2".into()), start_x: 3.0, start_y: 4.0, last_x: 5.0, last_y: 6.0, zoom: 2.0 });
    let inverse = forward.inverse(&base);
    assert_eq!(inverse, vec![WiresCanvasTransientMutation::SetDrag(SetDrag { node_id: base.drag_node_id.clone(), start_x: base.drag_start_x, start_y: base.drag_start_y, last_x: base.drag_last_x, last_y: base.drag_last_y, zoom: base.drag_zoom })]);
    assert_eq!(forward.diff(&base).diff().clone(), WiresCanvasTransient { drag_node_id: Some("node-2".into()), drag_start_x: 3.0, drag_start_y: 4.0, drag_last_x: 5.0, drag_last_y: 6.0, drag_zoom: 2.0 });
}
//#endregion 🔖️ConfigOperationTests

#[semio_framework_async_macros::async_test]
async fn wires_window_transient_retained_pointer_lifecycle_is_partitioned() {
    use crate::editor::wires::commands::{canvas_pointer_down::CanvasPointerDown, canvas_pointer_up::CanvasPointerUp};
    use crate::editor::wires::{create_wires_app, ReasoningWiresPlayApp, WiresCommand, WIRES_PLAY_WINDOW_CANVAS};
    use semio_framework_plugin::{testkit, ActionMeta, App, EditorApp, PluginApp, ViewModel, ViewWindowInstance};
    fn manifest() -> App {
        App { definition: create_wires_app(), examples: Vec::new() }
    }
    let mut app = testkit::new_app_with_registry::<EditorApp<ReasoningWiresPlayApp>>(manifest).await;
    app.bind_instance_id(1).await;
    let view = ViewModel { window_instances: ["canvas-left", "canvas-right"].into_iter().map(|id| ViewWindowInstance { id: id.into(), window_kind_id: WIRES_PLAY_WINDOW_CANVAS.into() }).collect(), ..Default::default() };
    let left = view.for_window_instance("canvas-left").unwrap();
    let right = view.for_window_instance("canvas-right").unwrap();
    let result: Result<(), String> = async {
        let document = app.snapshot().map_err(|error| format!("{error:?}"))?;
        let config = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        for command in [WiresCommand::CanvasPointerDown(CanvasPointerDown { id: None, x: 12.0, y: 24.0 }), WiresCommand::CanvasPointerUp(CanvasPointerUp {})] {
            app.dispatch_typed(command, &ActionMeta { view_state: Some(left.clone()), ..testkit::meta("canvas") }).await.map_err(|error| format!("{error:?}"))?;
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
            while app.has_pending_typed_operations() {
                if std::time::Instant::now() >= deadline {
                    return Err("canvas lifecycle did not finish".into());
                }
                app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
                app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
                if let Some(page) = app.take_typed_operation_result_page(1) {
                    let lane = page.lane;
                    let bytes = page.bytes().to_vec();
                    app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
                    if lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                        return Err(format!("canvas lifecycle fault: {bytes:?}"));
                    }
                }
                app.take_typed_operation_effect();
                app.take_typed_operation_event();
                app.take_typed_operation_ui_scope();
                std::thread::yield_now();
            }
        }
        let local = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("left owner absent")?;
        let other = app.window_transient_snapshot(&right).map_err(|error| format!("{error:?}"))?.ok_or("right owner absent")?;
        if local.generation() != 2 || other.generation() != 0 {
            return Err("canvas transient generations were shared or did not advance".into());
        }
        if local.get::<WiresCanvasTransientOwner>() != Some(&WiresCanvasTransient::default()) || other.get::<WiresCanvasTransientOwner>() != Some(&WiresCanvasTransient::default()) {
            return Err("pointer release did not leave exact empty drag projections".into());
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
            return Err("pointer-only input changed document content".into());
        }
        let after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        if config.pack != after.pack || config.spr != after.spr {
            return Err("pointer-only input changed the app config envelope".into());
        }
        Ok(())
    }
    .await;
    if let Err(error) = &result {
        eprintln!("[DEBUG] Wires window transient runtime failure before close: {error}");
    }
    testkit::close_registered_fixture_app(&mut app);
    result.expect("concrete canvas transient ownership");
    eprintln!("[DEBUG] Wires retained pointer lifecycle advanced only its concrete canvas and preserved document/config envelopes");
}
