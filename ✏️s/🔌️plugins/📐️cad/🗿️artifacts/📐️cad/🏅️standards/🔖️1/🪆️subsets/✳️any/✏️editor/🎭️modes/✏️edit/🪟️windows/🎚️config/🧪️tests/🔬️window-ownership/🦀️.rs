use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn block_on_cad_window_ownership<F: std::future::Future>(future: F) -> F::Output {
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
fn cad_document_contract_world_window_config_matches_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json")).expect("CAD world-window fixture");
    let base: CadWorldWindowConfig = dsl::json::from_json_str(&fixture["valid"][0].to_string()).expect("neutral config");
    assert_eq!(base, CadWorldWindowConfig::default());
    let mut next = base.clone();
    next.camera.zoom = fixture["patchedZoom"].as_f64().expect("patched zoom");
    let mutation = CadWorldWindowConfigMutation::Snapshot { config: Box::new(next.clone()) };
    let after = mutation.diff(&base).diff().apply(&base).expect("window config diff");
    assert_eq!(after, next);
    let restored = mutation.inverse(&base).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("window config inverse"));
    assert_eq!(restored, base);
    assert_eq!(CadWorldWindowConfigMutation::parse_op(&mutation.print_op()).expect("text mutation"), mutation);
    assert_eq!(CadWorldWindowConfigMutation::decode_op(&mutation.encode_op().expect("binary mutation")).expect("decoded mutation"), mutation);
    let text = store::ArtifactDsl::print_dsl(&next);
    assert_eq!(<CadWorldWindowConfig as store::ArtifactDsl>::parse_dsl(&text).expect("window config text"), next);
    let bytes = store::ArtifactPack::encode_pack(&next);
    assert_eq!(<CadWorldWindowConfig as store::ArtifactPack>::decode_pack(&bytes).expect("window config pack"), next);
    eprintln!("[DEBUG] CAD world-window config matched neutral fixture, inverse, text, and binary laws");
}

#[test]
fn cad_document_contract_world_window_runtime_isolates_commands_and_restores_exact_owner() {
    std::thread::Builder::new()
        .name("cad-world-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            block_on_cad_window_ownership(async {
                use crate::editor::cad::commands::camera::{set_camera, set_projection, set_projection_param};
                use crate::editor::cad::commands::sun::{set_sun_azimuth, toggle_sun};
                use crate::editor::cad::commands::utility::set_dislocate_option;
                use crate::editor::cad::{create_cad_app, CadCommand, CadPlayApp};
                use crate::CadCamera;
                use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner, WindowMeasure};

                fn manifest() -> App {
                    App { definition: create_cad_app(), examples: Vec::new() }
                }

                async fn drain(app: &mut VcsArtifactApp<EditorApp<CadPlayApp>>) -> Result<usize, String> {
                    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
                    let mut window_config_pages = 0;
                    while app.has_pending_typed_operations() {
                        if std::time::Instant::now() >= deadline {
                            return Err("CAD window publication timed out".into());
                        }
                        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
                        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
                        if let Some(page) = app.take_typed_operation_result_page(1) {
                            let lane = page.lane;
                            let bytes = page.bytes().to_vec();
                            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
                            match lane {
                                semio_framework_plugin::app::TypedOperationResultLane::WindowConfig => window_config_pages += 1,
                                semio_framework_plugin::app::TypedOperationResultLane::Fault => return Err(format!("CAD window publication fault: {}", String::from_utf8_lossy(&bytes))),
                                _ => {}
                            }
                        }
                        app.take_typed_operation_effect();
                        app.take_typed_operation_event();
                        app.take_typed_operation_ui_scope();
                        std::thread::yield_now();
                    }
                    Ok(window_config_pages)
                }

                async fn dispatch(app: &mut VcsArtifactApp<EditorApp<CadPlayApp>>, view: &ViewModel, command: CadCommand) -> Result<(), String> {
                    app.dispatch_typed(command, &ActionMeta { instance_id: 91, view_state: Some(view.clone()), ..artifact_app_laws::meta("cad-window-ownership") }).await.map_err(|error| format!("{error:?}"))?;
                    if drain(app).await? != 1 {
                        return Err("CAD command did not publish exactly one exact-window config result".into());
                    }
                    Ok(())
                }

                async fn scene(app: &mut VcsArtifactApp<EditorApp<CadPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::World3dScene, String> {
                    let tree = app.render(shape::BODY_KEY, None, view).await.map_err(|error| format!("{error:?}"))?;
                    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                    artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
                }

                fn toggle(measures: &[WindowMeasure], id: &str) -> Option<bool> {
                    for measure in measures {
                        match measure {
                            WindowMeasure::Toggle { id: candidate, pressed, .. } if candidate == id => return Some(*pressed),
                            WindowMeasure::Group { children, .. } => {
                                if let Some(value) = toggle(children, id) { return Some(value) }
                            }
                            _ => {}
                        }
                    }
                    None
                }

                fn slider(measures: &[WindowMeasure], id: &str) -> Option<f64> {
                    for measure in measures {
                        match measure {
                            WindowMeasure::Slider { id: candidate, value, .. } if candidate == id => return Some(*value),
                            WindowMeasure::Group { children, .. } => {
                                if let Some(value) = slider(children, id) { return Some(value) }
                            }
                            _ => {}
                        }
                    }
                    None
                }

                async fn assert_exact_state(app: &mut VcsArtifactApp<EditorApp<CadPlayApp>>, left: &ViewModel, right: &ViewModel, expected: &serde_json::Value) -> Result<(), String> {
                    let left_scene = scene(app, left).await?;
                    let right_scene = scene(app, right).await?;
                    let left_camera: serde_json::Value = serde_json::from_str(&left_scene.camera_json).map_err(|error| error.to_string())?;
                    let right_camera: serde_json::Value = serde_json::from_str(&right_scene.camera_json).map_err(|error| error.to_string())?;
                    let left_target: [f64; 3] = serde_json::from_value(left_camera["target"].clone()).map_err(|error| error.to_string())?;
                    let expected_target: [f64; 3] = serde_json::from_value(expected["cameraTarget"].clone()).map_err(|error| error.to_string())?;
                    if left_target != expected_target
                        || left_camera["zoom"].as_f64() != expected["cameraZoom"].as_f64()
                        || left_camera["projection"]["mode"]["kind"].as_str() != expected["projectionKind"].as_str()
                        || left_camera["projection"]["mode"]["fov"].as_f64() != expected["projectionFov"].as_f64()
                    {
                        return Err(format!("CAD left camera/projection did not retain the exact command state: {left_camera}"));
                    }
                    if right_camera["zoom"].as_f64() != Some(1.0) || right_camera["projection"]["mode"]["kind"].as_str() != Some("threePoint") {
                        return Err(format!("CAD left camera crossed into its same-kind sibling: {right_camera}"));
                    }
                    let left_measures = app.window_measures(left).await;
                    let right_measures = app.window_measures(right).await;
                    let left_rows = left_measures.get(left.window_id.as_deref().expect("left id")).map_or(&[][..], Vec::as_slice);
                    let right_rows = right_measures.get(right.window_id.as_deref().expect("right id")).map_or(&[][..], Vec::as_slice);
                    if toggle(left_rows, "cad-measure-sun-enabled") != Some(true)
                        || slider(left_rows, "cad-measure-sun-azimuth") != expected["sunAzimuth"].as_f64()
                        || toggle(left_rows, "cad-dislocate-rotate") != expected["rotateEnabled"].as_bool()
                    {
                        return Err("CAD left sun or utility state was not supplied by its exact WindowConfigSnapshot".into());
                    }
                    if toggle(right_rows, "cad-measure-sun-enabled") != Some(false)
                        || slider(right_rows, "cad-measure-sun-azimuth") != Some(45.0)
                        || toggle(right_rows, "cad-dislocate-rotate") != Some(true)
                    {
                        return Err("CAD left sun or utility state crossed into its same-kind sibling".into());
                    }
                    Ok(())
                }

                let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json")).expect("CAD window fixture");
                let expected = &fixture["ownership"];
                let left_id = expected["leftId"].as_str().expect("left id");
                let right_id = expected["rightId"].as_str().expect("right id");
                let wrong_id = expected["wrongId"].as_str().expect("wrong id");
                let view = ViewModel {
                    window_instances: vec![
                        ViewWindowInstance { id: left_id.into(), window_kind_id: shape::config::CadShapeWindowConfigOwner::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: right_id.into(), window_kind_id: shape::config::CadShapeWindowConfigOwner::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: wrong_id.into(), window_kind_id: building::config::CadBuildingWindowConfigOwner::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: "cad-panel".into(), window_kind_id: "cad-document-panel".into() },
                    ],
                    ..Default::default()
                };
                let left = view.for_window_instance(left_id).expect("left window");
                let right = view.for_window_instance(right_id).expect("right window");
                let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<CadPlayApp>>(manifest).await);
                app.bind_instance_id(91).await;
                let outcome: Result<(), String> = async {
                    let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_config_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    let position: [f64; 3] = serde_json::from_value(expected["cameraPosition"].clone()).map_err(|error| error.to_string())?;
                    let target: [f64; 3] = serde_json::from_value(expected["cameraTarget"].clone()).map_err(|error| error.to_string())?;
                    dispatch(&mut app, &left, CadCommand::SetCamera(set_camera::SetCamera {
                        pane: Some("cad.play.scene3d/building".into()),
                        camera: CadCamera { position, target, zoom: expected["cameraZoom"].as_f64().expect("zoom"), fov: 50.0, ..CadCamera::default() },
                    })).await?;
                    dispatch(&mut app, &left, CadCommand::SetProjection(set_projection::SetProjection {
                        pane: Some("cad.play.scene3d/energy".into()),
                        field: Some("perspectiveKind".into()),
                        value_str: Some(expected["projectionKind"].as_str().expect("projection kind").into()),
                        value_num: None,
                        param: None,
                    })).await?;
                    dispatch(&mut app, &left, CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam {
                        pane: Some("cad.play.scene3d/structure-classic".into()),
                        field: None,
                        value_str: None,
                        value_num: expected["projectionFov"].as_f64(),
                        param: Some("fov".into()),
                    })).await?;
                    dispatch(&mut app, &left, CadCommand::ToggleSun(toggle_sun::ToggleSun {})).await?;
                    dispatch(&mut app, &left, CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: expected["sunAzimuth"].as_f64().expect("sun azimuth") })).await?;
                    dispatch(&mut app, &left, CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption {
                        pane: Some("building".into()),
                        option: "rotate".into(),
                        pressed: expected["rotateEnabled"].as_bool(),
                    })).await?;
                    assert_exact_state(&mut app, &left, &right, expected).await?;
                    let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_config_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    if document_before.pack != document_after.pack || document_before.spr != document_after.spr {
                        return Err("CAD window commands changed document bytes".into());
                    }
                    if app_config_before.pack != app_config_after.pack || app_config_before.spr != app_config_after.spr {
                        return Err("CAD window commands changed app configuration bytes".into());
                    }
                    let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if packs.len() != 1 || packs[0].window_id != left_id || packs[0].window_kind_id != shape::WINDOW_KIND_ID {
                        return Err("CAD exact-window pack ownership changed".into());
                    }
                    app.load_document_pack(&document_before).await.map_err(|error| format!("{error:?}"))?;
                    assert_exact_state(&mut app, &left, &right, expected).await?;
                    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<CadPlayApp>>(manifest).await);
                    reopened.bind_instance_id(92).await;
                    for pack in packs {
                        reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?;
                    }
                    let restored = assert_exact_state(&mut reopened, &left, &right, expected).await;
                    artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                    restored?;
                    let stale = ViewModel { window_id: Some("cad-missing".into()), window_instances: view.window_instances.clone(), ..Default::default() };
                    if addressed(&stale, CadWorldWindowConfig::default()).is_ok() {
                        return Err("CAD accepted a stale window identity".into());
                    }
                    let panel = view.for_window_instance("cad-panel").expect("panel");
                    if addressed(&panel, CadWorldWindowConfig::default()).is_ok() {
                        return Err("CAD accepted a non-world window kind".into());
                    }
                    let building = view.for_window_instance(wrong_id).expect("building");
                    let building_mutation = addressed(&building, CadWorldWindowConfig::default()).map_err(|error| format!("{error:?}"))?;
                    if building_mutation.window_id() != wrong_id || building_mutation.window_kind_id() != building::WINDOW_KIND_ID {
                        return Err("CAD concrete owner dispatch lost its exact kind".into());
                    }
                    Ok(())
                }.await;
                if let Err(error) = &outcome {
                    eprintln!("[DEBUG] CAD exact-window runtime failure before close: {error}");
                }
                artifact_app_laws::close_registered_fixture_app(&mut *app);
                outcome.expect("CAD exact-window ownership runtime law");
                eprintln!("[DEBUG] CAD runtime isolated camera, projection, sun, and utility state across two same-kind windows, restored the exact pack, and preserved document/app bytes");
            })
        })
        .expect("spawn CAD window ownership law")
        .join()
        .expect("CAD window ownership law thread");
}
