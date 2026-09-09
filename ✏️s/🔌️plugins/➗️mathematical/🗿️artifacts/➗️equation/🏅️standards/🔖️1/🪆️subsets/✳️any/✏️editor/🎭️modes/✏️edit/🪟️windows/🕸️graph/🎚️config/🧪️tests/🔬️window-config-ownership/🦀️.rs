//! 🧪️ Exact Equation graph-window configuration publication and persistence laws.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_plugin::WindowConfigOwner;

fn block_on_equation_window_config<F: std::future::Future>(future: F) -> F::Output {
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
fn equation_graph_window_config_retained_publications_isolate_and_reload_two_windows() {
    std::thread::Builder::new()
        .name("equation-window-config-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            block_on_equation_window_config(async {
                use crate::editor::equation::commands::node_graph_viewport;
                use crate::editor::equation::testkit::{equation_app_manifest_for_testkit, MathApp};
                use crate::editor::equation::{EquationCommand, EquationPlayApp, MATH_PLAY_BODY_GRAPH};
                use semio_framework_plugin::{testkit, ActionMeta, EditorApp, PluginApp, ViewModel, ViewWindowInstance};

                async fn render(app: &mut MathApp, view: &ViewModel) -> Result<semio_framework_plugin::NodeGraphViewport, String> {
                    let tree = app.render(MATH_PLAY_BODY_GRAPH, None, view).await.map_err(|error| format!("{error:?}"))?;
                    let json = testkit::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                    let scene = testkit::decode_fixture_scene::<semio_framework_plugin::NodeGraphScene>(&json).map_err(str::to_string)?;
                    scene.viewport.ok_or_else(|| "Equation graph render omitted viewport".into())
                }

                async fn drain(app: &mut MathApp) -> Result<usize, String> {
                    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
                    let mut receipts = 0;
                    while app.has_pending_typed_operations() {
                        if std::time::Instant::now() >= deadline {
                            return Err("Equation window operations did not finish".into());
                        }
                        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
                        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
                        if let Some(page) = app.take_typed_operation_result_page(1) {
                            let lane = page.lane;
                            let bytes = page.bytes().to_vec();
                            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
                            match lane {
                                semio_framework_plugin::app::TypedOperationResultLane::WindowConfig => receipts += 1,
                                semio_framework_plugin::app::TypedOperationResultLane::Fault => return Err(format!("Equation window publication failed: {bytes:?}")),
                                _ => {}
                            }
                        }
                        app.take_typed_operation_effect();
                        app.take_typed_operation_event();
                        app.take_typed_operation_ui_scope();
                        std::thread::yield_now();
                    }
                    Ok(receipts)
                }

                let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
                let left_id = fixture["leftWindowId"].as_str().unwrap();
                let right_id = fixture["rightWindowId"].as_str().unwrap();
                let view = ViewModel { window_instances: [left_id, right_id].into_iter().map(|id| ViewWindowInstance { id: id.into(), window_kind_id: EquationGraphWindowConfigOwner::WINDOW_KIND_ID.into() }).collect(), ..Default::default() };
                let left = view.for_window_instance(left_id).unwrap();
                let right = view.for_window_instance(right_id).unwrap();
                let mut app = Box::new(testkit::new_app_with_registry::<EditorApp<EquationPlayApp>>(equation_app_manifest_for_testkit).await);
                let mut reopened = Box::new(testkit::new_app_with_registry::<EditorApp<EquationPlayApp>>(equation_app_manifest_for_testkit).await);
                app.bind_instance_id(1).await;
                reopened.bind_instance_id(2).await;
                let outcome: Result<(), String> = async {
                    let document = app.snapshot().map_err(|error| format!("{error:?}"))?;
                    let app_config = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    for row in fixture["cases"].as_array().unwrap() {
                        let context = view.for_window_instance(row["windowId"].as_str().unwrap()).unwrap();
                        let camera: EquationCamera = serde_json::from_value(row["mutation"]["camera"].clone()).map_err(|error| error.to_string())?;
                        app.dispatch_typed(EquationCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera }), &ActionMeta { view_state: Some(context), ..testkit::meta("equation-window-config") })
                            .await
                            .map_err(|error| format!("{error:?}"))?;
                    }
                    if drain(&mut app).await? != 2 {
                        return Err("Equation did not publish one config receipt per exact window".into());
                    }
                    if app.snapshot().map_err(|error| format!("{error:?}"))? != document {
                        return Err("Equation window config changed the document".into());
                    }
                    let app_config_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    if app_config.pack != app_config_after.pack || app_config.spr != app_config_after.spr {
                        return Err("Equation window config changed app config".into());
                    }
                    let left_viewport = render(&mut app, &left).await?;
                    let right_viewport = render(&mut app, &right).await?;
                    for (viewport, expected) in [(&left_viewport, &fixture["cases"][1]["expected"][left_id]["camera"]), (&right_viewport, &fixture["cases"][1]["expected"][right_id]["camera"])] {
                        if viewport.x != expected["x"].as_f64().unwrap() || viewport.y != expected["y"].as_f64().unwrap() || viewport.zoom != expected["zoom"].as_f64().unwrap() {
                            return Err(format!("Equation exact window rendered the wrong viewport: {viewport:?}"));
                        }
                    }
                    for context in [&left, &right] {
                        if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))? != Some(1) {
                            return Err(format!("Equation window {:?} did not own generation one", context.window_id));
                        }
                    }
                    let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if packs.len() != 2 {
                        return Err(format!("expected two Equation window packs, got {}", packs.len()));
                    }
                    for pack in packs {
                        reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?;
                    }
                    let reopened_left = render(&mut reopened, &left).await?;
                    let reopened_right = render(&mut reopened, &right).await?;
                    if reopened_left.x != left_viewport.x
                        || reopened_left.y != left_viewport.y
                        || reopened_left.zoom != left_viewport.zoom
                        || reopened_right.x != right_viewport.x
                        || reopened_right.y != right_viewport.y
                        || reopened_right.zoom != right_viewport.zoom
                    {
                        return Err("Equation exact window config changed during reload".into());
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = &outcome {
                    eprintln!("[DEBUG] Equation exact-window runtime failure before close: {error}");
                }
                testkit::close_registered_fixture_app(&mut reopened);
                testkit::close_registered_fixture_app(&mut app);
                outcome.expect("Equation exact-window config publication and persistence");
                eprintln!("[DEBUG] two Equation graph windows published, rendered, and reloaded independent persisted camera state");
            })
        })
        .expect("spawn Equation window-config law")
        .join()
        .expect("Equation window-config law thread");
}

#[test]
fn equation_graph_window_config_mutations_follow_the_neutral_trace_and_restore() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let base: EquationGraphWindowConfig = dsl::json::from_json_str(&fixture["base"].to_string()).unwrap();
    let mut windows = std::collections::BTreeMap::from([(fixture["leftWindowId"].as_str().unwrap().to_string(), base.clone()), (fixture["rightWindowId"].as_str().unwrap().to_string(), base)]);
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap();
        let mutation: EquationGraphWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let before = windows[id].clone();
        let after = mutation.diff(&before).diff().apply(&before).unwrap();
        let restored = mutation.inverse(&before).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, before);
        assert_eq!(EquationGraphWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(EquationGraphWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        windows.insert(id.into(), after);
        for (window_id, state) in &windows {
            let expected: EquationGraphWindowConfig = dsl::json::from_json_str(&row["expected"][window_id].to_string()).unwrap();
            assert_eq!(state, &expected);
        }
    }
    eprintln!("[DEBUG] Equation graph-window config matched the neutral exact-window trace, inverse, text, and binary laws");
}
