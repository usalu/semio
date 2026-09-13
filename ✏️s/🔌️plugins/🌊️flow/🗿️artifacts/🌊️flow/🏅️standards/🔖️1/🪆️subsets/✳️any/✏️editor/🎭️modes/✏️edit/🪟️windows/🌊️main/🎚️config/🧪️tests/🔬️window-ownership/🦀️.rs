use super::*;
use crate::editor::flow::modes::edit::windows::main::transient::{FlowGenerationsWindowTransientOwner, FlowWindowTransient, FlowWindowTransientMutation};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn block_on_flow_window_ownership<F: std::future::Future>(future: F) -> F::Output {
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
fn flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows() {
    std::thread::Builder::new()
        .name("flow-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            block_on_flow_window_ownership(async {
                use crate::editor::flow::commands::{node_graph_viewport, set_grid_factor, set_grid_visible};
                use crate::editor::flow::modes::generate::commands::add_generation;
                use crate::editor::flow::{create_flow_app, FlowCommand, FlowPlayApp, FLOW_PLAY_BODY_MAIN};
                use semio_framework_artifact_flow_flow::CameraJson;
                use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner, WindowTransientOwner};

                fn manifest() -> App {
                    App { definition: create_flow_app(), examples: Vec::new() }
                }
                async fn drain(app: &mut VcsArtifactApp<EditorApp<FlowPlayApp>>) -> Result<(usize, usize), String> {
                    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
                    let mut config = 0;
                    let mut transient = 0;
                    while app.has_pending_typed_operations() {
                        if std::time::Instant::now() >= deadline { return Err("Flow retained publications timed out".into()); }
                        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
                        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
                        if let Some(page) = app.take_typed_operation_result_page(1) {
                            let lane = page.lane;
                            let bytes = page.bytes().to_vec();
                            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
                            match lane {
                                semio_framework_plugin::app::TypedOperationResultLane::WindowConfig => config += 1,
                                semio_framework_plugin::app::TypedOperationResultLane::WindowTransient => transient += 1,
                                semio_framework_plugin::app::TypedOperationResultLane::Fault => return Err(format!("Flow retained publication fault: {bytes:?}")),
                                _ => {}
                            }
                        }
                        app.take_typed_operation_effect();
                        app.take_typed_operation_event();
                        app.take_typed_operation_ui_scope();
                        std::thread::yield_now();
                    }
                    Ok((config, transient))
                }
                async fn scene(app: &mut VcsArtifactApp<EditorApp<FlowPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::NodeGraphScene, String> {
                    let tree = app.render(FLOW_PLAY_BODY_MAIN, None, view).await.map_err(|error| format!("{error:?}"))?;
                    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                    artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
                }

                let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
                let left_id = fixture["windowInstances"][0]["id"].as_str().unwrap();
                let right_id = fixture["windowInstances"][1]["id"].as_str().unwrap();
                let generation_id = fixture["windowInstances"][3]["id"].as_str().unwrap();
                let view = ViewModel {
                    window_instances: vec![
                        ViewWindowInstance { id: left_id.into(), window_kind_id: FlowMainWindowConfigOwner::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: right_id.into(), window_kind_id: FlowMainWindowConfigOwner::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: generation_id.into(), window_kind_id: "flow-generations".into() },
                    ],
                    ..Default::default()
                };
                let left = view.for_window_instance(left_id).unwrap();
                let right = view.for_window_instance(right_id).unwrap();
                let generation = view.for_window_instance(generation_id).unwrap();
                let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<FlowPlayApp>>(manifest).await);
                app.bind_instance_id(71).await;
                let outcome: Result<(), String> = async {
                    let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    for (context, command) in [
                        (&left, FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 12.0, y: -8.0, zoom: 2.0 } })),
                        (&left, FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(false) })),
                        (&right, FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: -21.0, y: 5.0, zoom: 0.75 } })),
                        (&right, FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 20.0 })),
                    ] {
                        app.dispatch_typed(command, &ActionMeta { view_state: Some(context.clone()), ..artifact_app_laws::meta("flow-window-ownership") }).await.map_err(|error| format!("{error:?}"))?;
                    }
                    app.dispatch_typed(FlowCommand::AddGeneration(add_generation::AddGeneration {}), &ActionMeta { view_state: Some(generation.clone()), ..artifact_app_laws::meta("flow-window-ownership") }).await.map_err(|error| format!("{error:?}"))?;
                    if drain(&mut app).await? != (4, 1) { return Err("Flow exact-window publication lane count changed".into()); }
                    let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    if document_before.pack != document_after.pack || document_before.spr != document_after.spr { return Err("Flow window publications changed document bytes".into()); }
                    let left_scene = scene(&mut app, &left).await?;
                    let right_scene = scene(&mut app, &right).await?;
                    let left_viewport = left_scene.viewport.ok_or("left Flow viewport missing")?;
                    let right_viewport = right_scene.viewport.ok_or("right Flow viewport missing")?;
                    if (left_viewport.x, left_viewport.y, left_viewport.zoom) != (12.0, -8.0, 2.0) || (right_viewport.x, right_viewport.y, right_viewport.zoom) != (-21.0, 5.0, 0.75) {
                        return Err("Flow camera crossed exact window partitions".into());
                    }
                    fn grid(measures: &[semio_framework_plugin::WindowMeasure]) -> Option<(bool, f64)> {
                        let children = measures.iter().find_map(|measure| match measure {
                            semio_framework_plugin::WindowMeasure::Group { id, children, .. } if id == "flow-play-measures.grid" => Some(children),
                            _ => None,
                        })?;
                        let visible = children.iter().find_map(|measure| match measure {
                            semio_framework_plugin::WindowMeasure::Toggle { id, pressed, .. } if id == "flow-play-measures.grid-visible" => Some(*pressed),
                            _ => None,
                        })?;
                        let factor = children.iter().find_map(|measure| match measure {
                            semio_framework_plugin::WindowMeasure::Slider { id, value, .. } if id == "flow-play-measures.grid-factor" => Some(*value),
                            _ => None,
                        })?;
                        Some((visible, factor))
                    }
                    let left_measures = app.window_measures(&left).await;
                    let right_measures = app.window_measures(&right).await;
                    if grid(left_measures.get(left_id).map_or(&[][..], Vec::as_slice)) != Some((false, 10.0)) || grid(right_measures.get(right_id).map_or(&[][..], Vec::as_slice)) != Some((true, 20.0)) {
                        return Err("Flow settings crossed exact window partitions".into());
                    }
                    if app.window_transient_snapshot(&generation).map_err(|error| format!("{error:?}"))?.and_then(|snapshot| snapshot.get::<FlowGenerationsWindowTransientOwner>().cloned()).is_none() { return Err("Flow generation transient was not owned by its invoking window".into()); }
                    let config_packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    app.load_document_pack(&document_before).await.map_err(|error| format!("{error:?}"))?;
                    if app.window_transient_generation(&generation).map_err(|error| format!("{error:?}"))?.is_some() { return Err("Flow generation transient survived same-byte document reload".into()); }
                    for context in [&left, &right] {
                        if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))?.is_none() { return Err("Flow config was lost during same-byte document reload".into()); }
                    }
                    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<FlowPlayApp>>(manifest).await);
                    reopened.bind_instance_id(72).await;
                    for pack in config_packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                    let reopened_left = scene(&mut reopened, &left).await?.viewport.ok_or("reopened left Flow viewport missing")?;
                    let reopened_right = scene(&mut reopened, &right).await?.viewport.ok_or("reopened right Flow viewport missing")?;
                    artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                    if reopened_left != left_viewport || reopened_right != right_viewport { return Err("Flow persisted window config changed during restore".into()); }
                    let stale = ViewModel { window_id: Some("lost-flow-window".into()), window_instances: view.window_instances.clone(), ..Default::default() };
                    if addressed(&stale, FlowMainWindowConfig::default()).is_ok() { return Err("Flow accepted stale window identity".into()); }
                    let wrong = ViewModel { window_id: Some(generation_id.into()), window_instances: view.window_instances.clone(), ..Default::default() };
                    if addressed(&wrong, FlowMainWindowConfig::default()).is_ok() { return Err("Flow accepted wrong-kind window identity".into()); }
                    Ok(())
                }.await;
                if let Err(error) = &outcome { eprintln!("[DEBUG] Flow exact-window runtime failure before close: {error}"); }
                artifact_app_laws::close_registered_fixture_app(&mut *app);
                outcome.expect("Flow exact-window ownership runtime law");
                eprintln!("[DEBUG] Flow runtime isolated two same-kind cameras/settings, restored config, preserved document bytes, cleared transient on reload, and rejected stale/wrong windows");
            })
        })
        .expect("spawn Flow window ownership law")
        .join()
        .expect("Flow window ownership law thread");
}

#[test]
fn flow_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base_config: FlowMainWindowConfig = dsl::json::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let base_transient: FlowWindowTransient = dsl::json::from_json_str(&fixture["baseTransient"].to_string()).unwrap();
    for row in fixture["configMutations"].as_array().unwrap() {
        let mutation: FlowMainWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_config).diff().apply(&base_config).unwrap();
        let restored = mutation.inverse(&base_config).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_config);
        assert_eq!(FlowMainWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(FlowMainWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for row in fixture["transientMutations"].as_array().unwrap() {
        let mutation: FlowWindowTransientMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_transient).diff().apply(&base_transient).unwrap();
        let restored = mutation.inverse(&base_transient).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_transient);
        assert_eq!(FlowWindowTransientMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(FlowWindowTransientMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    eprintln!("[DEBUG] Flow config/transient mutations matched neutral fixture inverse, text, and binary laws");
}
