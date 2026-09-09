use super::*;
use crate::editor::writer::modes::edit::windows::main::transient::{WriterMainWindowTransient, WriterMainWindowTransientMutation, WriterMainWindowTransientOwner};
use dsl::os_pack as pack;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn block_on_writer_window_state<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn writer_window_state_retained_publications_isolate_two_windows_and_reload_only_config() {
    std::thread::Builder::new()
        .name("writer-window-state-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            block_on_writer_window_state(async {
                use crate::editor::writer::commands::{engagement_input, lint_document, set_camera, set_editor_selection, set_font_px};
                use crate::editor::writer::{create_writer_app, WriterCommand, WriterPlayApp, WRITER_PLAY_BODY_MAIN};
                use crate::WriterCamera;
                use semio_framework_plugin::{testkit, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

                fn manifest() -> App {
                    App { definition: create_writer_app(), examples: Vec::new() }
                }
                async fn render(app: &mut VcsArtifactApp<EditorApp<WriterPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::TextEditorScene, String> {
                    let tree = app.render(WRITER_PLAY_BODY_MAIN, None, view).await.map_err(|error| format!("{error:?}"))?;
                    let json = testkit::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                    testkit::decode_fixture_scene::<semio_framework_plugin::TextEditorScene>(&json).map_err(str::to_string)
                }
                fn scene_json(text: Option<&String>, field: &str) -> Result<serde_json::Value, String> {
                    let text = text.ok_or_else(|| format!("missing Writer scene field {field}"))?;
                    serde_json::from_str(text).map_err(|error| error.to_string())
                }
                async fn drain(app: &mut VcsArtifactApp<EditorApp<WriterPlayApp>>) -> Result<(usize, usize), String> {
                    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
                    let (mut config_receipts, mut transient_receipts) = (0, 0);
                    while app.has_pending_typed_operations() {
                        if std::time::Instant::now() >= deadline {
                            return Err("Writer window operations did not finish".into());
                        }
                        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
                        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
                        if let Some(page) = app.take_typed_operation_result_page(1) {
                            let lane = page.lane;
                            let bytes = page.bytes().to_vec();
                            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
                            match lane {
                                semio_framework_plugin::app::TypedOperationResultLane::WindowConfig => config_receipts += 1,
                                semio_framework_plugin::app::TypedOperationResultLane::WindowTransient => transient_receipts += 1,
                                semio_framework_plugin::app::TypedOperationResultLane::Fault => return Err(format!("Writer window publication failed: {bytes:?}")),
                                _ => {}
                            }
                        }
                        app.take_typed_operation_effect();
                        app.take_typed_operation_event();
                        app.take_typed_operation_ui_scope();
                        std::thread::yield_now();
                    }
                    Ok((config_receipts, transient_receipts))
                }

                let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-state-ownership/🔣️.json")).unwrap();
                let left_id = fixture["leftWindowId"].as_str().unwrap();
                let right_id = fixture["rightWindowId"].as_str().unwrap();
                let view = ViewModel { window_instances: [left_id, right_id].into_iter().map(|id| ViewWindowInstance { id: id.into(), window_kind_id: WriterMainWindowConfigOwner::WINDOW_KIND_ID.into() }).collect(), ..Default::default() };
                let left = view.for_window_instance(left_id).unwrap();
                let right = view.for_window_instance(right_id).unwrap();
                let mut app = Box::new(testkit::new_app_with_registry::<EditorApp<WriterPlayApp>>(manifest).await);
                let mut reopened = Box::new(testkit::new_app_with_registry::<EditorApp<WriterPlayApp>>(manifest).await);
                app.bind_instance_id(1).await;
                reopened.bind_instance_id(2).await;
                let outcome: Result<(), String> = async {
                    let document = app.snapshot().map_err(|error| format!("{error:?}"))?;
                    let app_config = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    for (context, command) in [
                        (&left, WriterCommand::SetCamera(set_camera::SetCamera { camera: WriterCamera { x: 12.0, y: -4.0, zoom: 1.5 } })),
                        (&right, WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 8 })),
                        (&right, WriterCommand::SetFontPx(set_font_px::SetFontPx { value: 16 })),
                        (&left, WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "format".into() })),
                    ] {
                        app.dispatch_typed(command, &ActionMeta { view_state: Some(context.clone()), ..testkit::meta("writer-window-state") }).await.map_err(|error| format!("{error:?}"))?;
                    }
                    let (mut config_receipts, mut transient_receipts) = drain(&mut app).await?;
                    app.dispatch_typed(WriterCommand::LintDocument(lint_document::LintDocument {}), &ActionMeta { view_state: Some(left.clone()), ..testkit::meta("writer-window-state") }).await.map_err(|error| format!("{error:?}"))?;
                    let (next_config_receipts, next_transient_receipts) = drain(&mut app).await?;
                    config_receipts += next_config_receipts;
                    transient_receipts += next_transient_receipts;
                    if (config_receipts, transient_receipts) != (2, 3) {
                        return Err(format!("expected 2 config and 3 transient receipts, got {config_receipts} and {transient_receipts}"));
                    }
                    if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
                        return Err("Writer view publications changed the document".into());
                    }
                    let app_config_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    if app_config.pack != app_config_after.pack || app_config.spr != app_config_after.spr {
                        return Err("Writer view publications changed app configuration".into());
                    }
                    for context in [&left, &right] {
                        if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))? != Some(1) {
                            return Err(format!("Writer config generation was not isolated for {:?}", context.window_id));
                        }
                    }
                    if app.window_transient_generation(&left).map_err(|error| format!("{error:?}"))? != Some(2) || app.window_transient_generation(&right).map_err(|error| format!("{error:?}"))? != Some(1) {
                        return Err("Writer transient generations were not isolated".into());
                    }
                    let left_transient = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.and_then(|snapshot| snapshot.get::<WriterMainWindowTransientOwner>().cloned()).ok_or("missing left Writer transient")?;
                    let right_transient = app.window_transient_snapshot(&right).map_err(|error| format!("{error:?}"))?.and_then(|snapshot| snapshot.get::<WriterMainWindowTransientOwner>().cloned()).ok_or("missing right Writer transient")?;
                    if left_transient.engagement_input != "format" || left_transient.lint_generation != 1 || left_transient.editor_selection.is_some() {
                        return Err(format!("left Writer transient crossed partitions: {left_transient:?}"));
                    }
                    if right_transient.editor_selection != Some(crate::WriterEditorSelection { start: 3, end: 8 }) || right_transient.lint_generation != 0 || !right_transient.engagement_input.is_empty() {
                        return Err(format!("right Writer transient crossed partitions: {right_transient:?}"));
                    }
                    let left_tree = render(&mut app, &left).await?;
                    let right_tree = render(&mut app, &right).await?;
                    if scene_json(left_tree.camera_json.as_ref(), "cameraJson")? != serde_json::json!({ "x": 12.0, "y": -4.0, "zoom": 1.5 }) {
                        return Err("left Writer camera did not render".into());
                    }
                    let default_camera = serde_json::to_value(WriterCamera::default()).map_err(|error| error.to_string())?;
                    if scene_json(right_tree.camera_json.as_ref(), "cameraJson")? != default_camera {
                        return Err("left Writer camera contaminated right render".into());
                    }
                    if scene_json(right_tree.selection_json.as_ref(), "selectionJson")? != serde_json::json!({ "start": 3, "end": 8 }) {
                        return Err("right Writer selection did not render".into());
                    }
                    if scene_json(right_tree.settings_json.as_ref(), "settingsJson")?["fontPx"] != serde_json::json!(16) {
                        return Err("right Writer settings did not render".into());
                    }
                    let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if packs.len() != 2 {
                        return Err(format!("expected two Writer window config packs, got {}", packs.len()));
                    }
                    for pack in packs {
                        reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?;
                    }
                    let reopened_left = render(&mut reopened, &left).await?;
                    let reopened_right = render(&mut reopened, &right).await?;
                    if scene_json(reopened_left.camera_json.as_ref(), "cameraJson")? != scene_json(left_tree.camera_json.as_ref(), "cameraJson")?
                        || scene_json(reopened_right.settings_json.as_ref(), "settingsJson")? != scene_json(right_tree.settings_json.as_ref(), "settingsJson")?
                    {
                        return Err("Writer persisted window config changed during reload".into());
                    }
                    if scene_json(reopened_right.selection_json.as_ref(), "selectionJson")? != serde_json::json!({ "start": 0, "end": 0 }) {
                        return Err("Writer ephemeral selection survived config reload".into());
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = &outcome {
                    eprintln!("[DEBUG] Writer exact-window runtime failure before close: {error}");
                }
                testkit::close_registered_fixture_app(reopened.as_mut());
                testkit::close_registered_fixture_app(app.as_mut());
                outcome.expect("retained Writer exact-window ownership and persistence");
                eprintln!("[DEBUG] two Writer windows published config/transient state independently, preserved document/app config, and reloaded only persisted config");
            })
        })
        .expect("spawn Writer window-state law")
        .join()
        .expect("Writer window-state law thread");
}

#[test]
fn writer_window_state_mutations_are_exact_reversible_and_codec_stable() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-state-ownership/🔣️.json")).unwrap();
    let base_config: WriterMainWindowConfig = pack::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let base_transient: WriterMainWindowTransient = pack::from_json_str(&fixture["baseTransient"].to_string()).unwrap();
    let ids = [fixture["leftWindowId"].as_str().unwrap(), fixture["rightWindowId"].as_str().unwrap()];
    let mut configs = std::collections::BTreeMap::from(ids.map(|id| (id.to_string(), base_config.clone())));
    let mut transients = std::collections::BTreeMap::from(ids.map(|id| (id.to_string(), base_transient.clone())));
    for step in fixture["steps"].as_array().unwrap() {
        let id = step["windowId"].as_str().unwrap();
        if step["lane"] == "config" {
            let mutation: WriterMainWindowConfigMutation = pack::from_json_str(&step["mutation"].to_string()).unwrap();
            let before = configs[id].clone();
            let after = mutation.diff(&before).diff().apply(&before).unwrap();
            let restored = mutation.inverse(&before).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
            assert_eq!(restored, before);
            assert_eq!(WriterMainWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            assert_eq!(WriterMainWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
            configs.insert(id.into(), after);
        } else {
            let mutation: WriterMainWindowTransientMutation = pack::from_json_str(&step["mutation"].to_string()).unwrap();
            let before = transients[id].clone();
            let after = mutation.diff(&before).diff().apply(&before).unwrap();
            let restored = mutation.inverse(&before).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
            assert_eq!(restored, before);
            assert_eq!(WriterMainWindowTransientMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            assert_eq!(WriterMainWindowTransientMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
            transients.insert(id.into(), after);
        }
    }
    for (id, state) in configs {
        let expected: WriterMainWindowConfig = pack::from_json_str(&fixture["expectedConfigs"][id].to_string()).unwrap();
        assert_eq!(state, expected);
    }
    for (id, state) in transients {
        let expected: WriterMainWindowTransient = pack::from_json_str(&fixture["expectedTransients"][id].to_string()).unwrap();
        assert_eq!(state, expected);
    }
    eprintln!("[DEBUG] Writer exact-window config/transient mutations matched the shared neutral fixture and exact inverse/codec laws");
}
