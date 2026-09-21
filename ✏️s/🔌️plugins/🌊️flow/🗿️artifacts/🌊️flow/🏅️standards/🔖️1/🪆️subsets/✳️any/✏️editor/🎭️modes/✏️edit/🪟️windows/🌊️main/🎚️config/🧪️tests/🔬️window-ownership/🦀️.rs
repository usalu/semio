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
                /// 🪪️ The instance this law binds — the SAME id every result page is addressed to.
                const FLOW_WINDOW_OWNERSHIP_INSTANCE: u32 = 71;
                type FlowRuntime = VcsArtifactApp<EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
                /// 🧹️ Settles every retained publication through the framework's OWN settle ladder.
                /// A hand-rolled loop here took result pages for receiver `1` while this runtime binds
                /// instance `71` (so no page was ever presented or ACKed) and never drained
                /// `take_typed_operation_completion()` — `has_pending_typed_operations` counts both
                /// outboxes, so the loop spun to its 30 s deadline and the law failed with
                /// "Flow retained publications timed out" (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP).
                async fn drain(app: &mut FlowRuntime) -> Result<(usize, usize), String> {
                    use semio_framework_plugin::app::TypedOperationResultLane;
                    let receipt = artifact_app_laws::settle_registered_typed_operation(app, FLOW_WINDOW_OWNERSHIP_INSTANCE).await.map_err(|error| format!("{error:?}"))?;
                    eprintln!("[DEBUG] Flow exact-window settle lanes: {:?}", receipt.lanes);
                    let count = |wanted: TypedOperationResultLane| receipt.lanes.iter().filter(|lane| **lane == wanted).count();
                    Ok((count(TypedOperationResultLane::WindowConfig), count(TypedOperationResultLane::WindowTransient)))
                }
                async fn scene(app: &mut FlowRuntime, view: &ViewModel) -> Result<semio_framework_plugin::NodeGraphScene, String> {
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
                let mut app = Box::new(artifact_app_laws::new_app_with_registry_and_members::<EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(manifest).await);
                app.bind_instance_id(FLOW_WINDOW_OWNERSHIP_INSTANCE).await;
                let outcome: Result<(), String> = async {
                    let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let mut lanes = (0usize, 0usize);
                    for (context, command) in [
                        (&left, FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 12.0, y: -8.0, zoom: 2.0 } })),
                        (&left, FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(false) })),
                        (&right, FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: -21.0, y: 5.0, zoom: 0.75 } })),
                        (&right, FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 20.0 })),
                    ] {
                        app.dispatch_typed(command, &ActionMeta { instance_id: FLOW_WINDOW_OWNERSHIP_INSTANCE, view_state: Some(context.clone()), ..artifact_app_laws::meta("flow-window-ownership") }).await.map_err(|error| format!("{error:?}"))?;
                        lanes.0 += drain(&mut app).await?.0;
                    }
                    app.dispatch_typed(FlowCommand::AddGeneration(add_generation::AddGeneration {}), &ActionMeta { instance_id: FLOW_WINDOW_OWNERSHIP_INSTANCE, view_state: Some(generation.clone()), ..artifact_app_laws::meta("flow-window-ownership") }).await.map_err(|error| format!("{error:?}"))?;
                    lanes.1 += drain(&mut app).await?.1;
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
                    let left_grid = grid(left_measures.get(left_id).map_or(&[][..], Vec::as_slice));
                    let right_grid = grid(right_measures.get(right_id).map_or(&[][..], Vec::as_slice));
                    if left_grid != Some((false, 10.0)) || right_grid != Some((true, 20.0)) {
                        return Err(format!("Flow settings crossed exact window partitions: left {left_grid:?}, right {right_grid:?}"));
                    }
                    if app.window_transient_snapshot(&generation).map_err(|error| format!("{error:?}"))?.and_then(|snapshot| snapshot.get::<FlowGenerationsWindowTransientOwner>().cloned()).is_none() { return Err("Flow generation transient was not owned by its invoking window".into()); }
                    // 📄️ Checked AFTER the content laws above, so a lane-count change reports as itself
                    // instead of masking (or being masked by) a camera/settings regression.
                    // 📄️ FOUR window-config pages, one per dispatched config command, and one window
                    // transient page — checked AFTER the content laws above, so a lane-count change reports
                    // as itself instead of masking a camera/settings regression. Each command is settled
                    // before the next is dispatched (see the loop): dispatching two config commands for the
                    // SAME window back to back drops the second silently — no page, no fault, its amend lost
                    // (measured: lanes (2, 1), left grid stuck at the default `(true, 10.0)` instead of
                    // `(false, 10.0)`, right at `10.0` instead of `20.0`). That is a live production defect,
                    // recorded in 📓️flow.md §5; it is NOT this law's subject, which is exact-window partitioning.
                    if lanes != (4, 1) { return Err(format!("Flow exact-window publication lane count changed: {lanes:?} instead of (4, 1) window-config/window-transient pages")); }
                    let config_packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    app.load_document_pack(&document_before).await.map_err(|error| format!("{error:?}"))?;
                    // 🫧️ A same-byte reload is still a NEW document instance for every window: the whole
                    // window-transient registry is replaced at `document_generation + 1`, so each partition
                    // is reborn at generation 0 holding its own `Default` state. `capture` answers `Some`
                    // for every REGISTERED window kind (it materialises the partition on demand), so
                    // "the transient did not survive" is exactly "generation 0 and default state", never
                    // `None` — `None` only ever means "this window kind registers no transient owner at all".
                    if app.window_transient_generation(&generation).map_err(|error| format!("{error:?}"))? != Some(0) { return Err("Flow generation transient survived same-byte document reload".into()); }
                    if app.window_transient_snapshot(&generation).map_err(|error| format!("{error:?}"))?.and_then(|snapshot| snapshot.get::<FlowGenerationsWindowTransientOwner>().cloned()) != Some(FlowWindowTransient::default()) {
                        return Err("Flow generation transient state survived same-byte document reload".into());
                    }
                    for context in [&left, &right] {
                        if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))?.is_none() { return Err("Flow config was lost during same-byte document reload".into()); }
                    }
                    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry_and_members::<EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(manifest).await);
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

/// 🎚️ Two retained window-config commands addressed at the SAME window inside ONE turn must BOTH
/// publish. Each command captures its window-config authority when it is DISPATCHED, and a window's
/// config partition is its own exact-base document, so the first command's publication leaves the
/// second command's captured base stale. The emission ladder popped the second mutation out of its
/// `Emit`, `begin_apply_batch` refused the stale base, and the retry the publication driver grants on
/// a fault then found nothing left to publish: the operation completed clean with NO page and NO
/// fault and the amend was lost. Measured on this exact pair before the fix
/// (`🗑️generated/fl2-window-config-before.txt`): lanes `[WindowConfig, Ui, Ui, Terminal, Terminal]` —
/// ONE window-config page for two commands — and the grid at `(false, 10.0)`, i.e. only the first
/// command landed (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP `📓️flow.md` §5.2).
///
/// 🪢️ What the FRAMEWORK owes, and what it does not. It owes both commands a publication, in dispatch
/// order — which is exactly what the NON-retained window-config route has always done (one
/// `WindowConfigOwnerRegistry::dispatch` per emitted mutation, in order). It does NOT owe the pair a
/// merge: flow's own `flow_direct_store_emit` emits a WHOLE-record `FlowMainWindowConfig` built from
/// the config it read at dispatch time, so the later record necessarily supersedes the earlier one and
/// the turn ends at the second command's full record. A flow-side field-level mutation (or a settle
/// between the two dispatches, as the law above does) is what makes both fields survive.
#[test]
fn flow_two_window_config_commands_in_one_turn_both_land() {
    std::thread::Builder::new()
        .name("flow-window-config-turn-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            block_on_flow_window_ownership(async {
                use crate::editor::flow::commands::{set_grid_factor, set_grid_visible};
                use crate::editor::flow::{create_flow_app, FlowCommand, FlowPlayApp};
                use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

                fn manifest() -> App {
                    App { definition: create_flow_app(), examples: Vec::new() }
                }
                const FLOW_ONE_TURN_INSTANCE: u32 = 73;
                type FlowRuntime = VcsArtifactApp<EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;

                let window_id = "flow-one-turn-window";
                let view = ViewModel {
                    window_instances: vec![ViewWindowInstance { id: window_id.into(), window_kind_id: FlowMainWindowConfigOwner::WINDOW_KIND_ID.into() }],
                    ..Default::default()
                };
                let window = view.for_window_instance(window_id).unwrap();
                let mut app: Box<FlowRuntime> = Box::new(artifact_app_laws::new_app_with_registry_and_members::<EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(manifest).await);
                app.bind_instance_id(FLOW_ONE_TURN_INSTANCE).await;
                let outcome: Result<(), String> = async {
                    let meta = ActionMeta { instance_id: FLOW_ONE_TURN_INSTANCE, view_state: Some(window.clone()), ..artifact_app_laws::meta("flow-one-turn") };
                    app.dispatch_typed(FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(false) }), &meta).await.map_err(|error| format!("{error:?}"))?;
                    app.dispatch_typed(FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 20.0 }), &meta).await.map_err(|error| format!("{error:?}"))?;
                    let receipt = artifact_app_laws::settle_registered_typed_operation(&mut *app, FLOW_ONE_TURN_INSTANCE).await.map_err(|error| format!("{error:?}"))?;
                    let pages = receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count();
                    eprintln!("[DEBUG] Flow one-turn window-config lanes: {:?}", receipt.lanes);
                    let measures = app.window_measures(&window).await;
                    let rows = measures.get(window_id).map_or(&[][..], Vec::as_slice);
                    let children = rows
                        .iter()
                        .find_map(|measure| match measure {
                            semio_framework_plugin::WindowMeasure::Group { id, children, .. } if id == "flow-play-measures.grid" => Some(children),
                            _ => None,
                        })
                        .ok_or("flow grid measures missing")?;
                    let visible = children
                        .iter()
                        .find_map(|measure| match measure {
                            semio_framework_plugin::WindowMeasure::Toggle { id, pressed, .. } if id == "flow-play-measures.grid-visible" => Some(*pressed),
                            _ => None,
                        })
                        .ok_or("flow grid-visible measure missing")?;
                    let factor = children
                        .iter()
                        .find_map(|measure| match measure {
                            semio_framework_plugin::WindowMeasure::Slider { id, value, .. } if id == "flow-play-measures.grid-factor" => Some(*value),
                            _ => None,
                        })
                        .ok_or("flow grid-factor measure missing")?;
                    if pages != 2 {
                        return Err(format!("two window-config commands in one turn published {pages} window-config pages instead of 2 — the second was dropped without a page or a fault"));
                    }
                    if (visible, factor) != (true, 20.0) {
                        return Err(format!("the turn did not end at the SECOND command's whole-record window config: grid {:?} instead of (true, 20.0)", (visible, factor)));
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = &outcome {
                    eprintln!("[DEBUG] Flow one-turn window-config failure before close: {error}");
                }
                artifact_app_laws::close_registered_fixture_app(&mut *app);
                outcome.expect("Flow two window-config commands in one turn");
                eprintln!("[DEBUG] Flow published both window-config commands dispatched in one turn, in order");
            })
        })
        .expect("spawn Flow one-turn window config law")
        .join()
        .expect("Flow one-turn window config law thread");
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
