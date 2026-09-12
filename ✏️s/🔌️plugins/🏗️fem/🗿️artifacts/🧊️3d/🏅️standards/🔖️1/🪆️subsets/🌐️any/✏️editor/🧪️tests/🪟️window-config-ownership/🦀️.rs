use super::*;

fn renderer_camera_command() -> crate::editor::fem3d::commands::set_camera::SetCamera {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json")).unwrap();
    let gesture = fixture["gestures"].as_array().unwrap().iter().find(|row| row["id"] == "orbit-completes-into-one-setcamera").unwrap();
    let payload = serde_json::json!({ "windowId": fixture["scene"]["windowInstanceId"], "camera": gesture["expect"]["camera"] });
    dsl::json::from_json_str(&payload.to_string()).expect("native FEM command decodes the renderer gesture envelope")
}

#[test]
fn fem3d_window_config_camera_uses_the_shared_renderer_pose_contract() {
    let command = renderer_camera_command();
    assert_eq!(command.camera, crate::Viewport3dOrbit { position: [8.0, -3.0, 5.0], target: [0.0; 3], zoom: 1.25, up: None });
    for invalid in [
        serde_json::json!({ "position": [8, -3, 5], "target": [0, 0, 0], "zoom": 1.25 }),
        serde_json::json!({ "camera": { "json": "{}" } }),
        serde_json::json!({ "camera": { "position": [8, -3], "target": [0, 0, 0], "zoom": 1.25 } }),
        serde_json::json!({ "camera": { "position": [8, -3, 5], "target": [0, 0, 0], "zoom": 1.25, "projection": "orthographic" } }),
    ] {
        assert!(dsl::json::from_json_str::<crate::editor::fem3d::commands::set_camera::SetCamera>(&invalid.to_string()).is_err());
    }
    eprintln!("[DEBUG] FEM native camera command admits the actual nested renderer pose and rejects opaque, flat, short-vector and projection-mixed payloads");
}


#[test]
fn fem3d_window_config_document_admission_rejects_window_and_os_fields() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🪟️window-config-contract/🧫️fixtures/🧬️document-admission/🔣️.json")).expect("document admission fixture");
    let row = fixture["cases"].as_array().expect("document cases").iter().find(|row| row["dimension"] == "3d").expect("dimension");
    let base = row["document"].to_string();
    let _ = dsl::json::from_json_str::<crate::standards::v1::subsets::any::schema::Fem3dArtifact>(&base).expect("neutral FEM artifact admission");
    let _ = dsl::json::from_json_str::<crate::Fem3dSnapshot>(&base).expect("neutral FEM snapshot admission");
    for field in row["foreignFields"].as_array().expect("foreign fields") {
        let key = field["key"].as_str().expect("field key");
        let mut candidate = row["document"].clone();
        candidate[key] = field["value"].clone();
        let text = candidate.to_string();
        assert!(dsl::json::from_json_str::<crate::standards::v1::subsets::any::schema::Fem3dArtifact>(&text).is_err(), "artifact admitted {key}");
        assert!(dsl::json::from_json_str::<crate::Fem3dSnapshot>(&text).is_err(), "snapshot admitted {key}");
    }
    eprintln!("[DEBUG] FEM 3d artifact and snapshot reject all three foreign owner fields");
}

fn block_on_fem_window_ownership<F: std::future::Future>(mut future: std::pin::Pin<Box<F>>) -> F::Output {
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
fn fem3d_window_config_runtime_isolates_same_kind_instances_and_restores_packs() {
    std::thread::Builder::new()
        .name("fem3d-window-ownership-law".into())
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            block_on_fem_window_ownership(Box::pin(async {
                use crate::editor::fem3d::commands::{set_active_example, set_result_display};
                use crate::editor::fem3d::modes::edit::windows::{model, results};
                use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

                fn manifest() -> App {
                    App { definition: create_fem3d_app(), examples: Vec::new() }
                }

                async fn dispatch(app: &mut VcsArtifactApp<EditorApp<Fem3dPlayApp>>, view: &ViewModel, command: Fem3dCommand, expected_windows: usize) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, String> {
                    let command_id = command.command_id();
                    let meta = ActionMeta { instance_id: 71, view_state: Some(view.clone()), ..artifact_app_laws::meta("fem3d-window-ownership") };
                    let dispatch = app.dispatch_typed(command, &meta);
                    Box::pin(dispatch).await.map_err(|error| format!("{error:?}"))?;
                    let receipt = Box::pin(artifact_app_laws::settle_registered_typed_operation(app, 71)).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                    let actual = receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count();
                    if actual != expected_windows {
                        return Err(format!("expected {expected_windows} FEM window publications, got {actual}"));
                    }
                    Ok(receipt)
                }

                let all = ViewModel {
                    window_instances: vec![
                        ViewWindowInstance { id: "model-left".into(), window_kind_id: model::FEM3D_WINDOW_MODEL.into() },
                        ViewWindowInstance { id: "model-right".into(), window_kind_id: model::FEM3D_WINDOW_MODEL.into() },
                        ViewWindowInstance { id: "results-left".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() },
                        ViewWindowInstance { id: "results-right".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() },
                        ViewWindowInstance { id: "foreign".into(), window_kind_id: "fem-panel".into() },
                    ],
                    ..Default::default()
                };
                let model_left = all.for_window_instance("model-left").expect("model-left");
                let results_left = all.for_window_instance("results-left").expect("results-left");
                let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Fem3dPlayApp>>(manifest).await);
                app.bind_instance_id(71).await;
                let outcome: Result<(), String> = Box::pin(async {
                    let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    dispatch(&mut app, &model_left, Fem3dCommand::SetCamera(renderer_camera_command()), 1).await?;
                    dispatch(&mut app, &results_left, Fem3dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 }), 1).await?;
                    let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    if document_before.pack != document_after.pack || document_before.spr != document_after.spr {
                        return Err("FEM window commands changed document bytes".into());
                    }
                    if app_before.pack != app_after.pack || app_before.spr != app_after.spr {
                        return Err("FEM window commands changed app-config bytes".into());
                    }
                    let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if packs.len() != 2 || packs.iter().any(|pack| pack.window_id.ends_with("right")) {
                        return Err("FEM window state crossed into a same-kind sibling".into());
                    }
                    let model_state = artifact_app_laws::capture_fixture_window_config::<model::config::Fem3dModelWindowConfigOwner, _, _>(&mut *app, &model_left).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing exact FEM model state".to_string())?;
                    if model_state.camera != (crate::Viewport3dOrbit { position: [8.0, -3.0, 5.0], target: [0.0; 3], zoom: 1.25, up: None }) {
                        return Err("FEM model camera did not persist on its exact owner".into());
                    }
                    let rendered = app.render(model::FEM3D_BODY_MODEL, None, &model_left).await.map_err(|error| format!("{error:?}"))?;
                    let rendered = artifact_app_laws::project_and_retire_fixture_tree(rendered).map_err(|error| format!("{error:?}"))?;
                    let scene = artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::World3dScene>(&rendered).map_err(|error| format!("{error:?}"))?;
                    let echoed = dsl::json::from_json_str::<crate::Viewport3dOrbit>(&scene.camera_json).map_err(|error| format!("{error:?}"))?;
                    if echoed != model_state.camera { return Err("FEM renderer echo lost the exact window pose".into()); }
                    let result_state = artifact_app_laws::capture_fixture_window_config::<results::config::Fem3dResultsWindowConfigOwner, _, _>(&mut *app, &results_left).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing exact FEM results state".to_string())?;
                    if result_state.result_source_id.as_deref() != Some("dead") || result_state.result_mode != crate::app_surface::ResultMode::Modal || result_state.result_mode_index != 2 {
                        return Err("FEM result display did not persist on its exact owner".into());
                    }
                    let before_example: Vec<(String, Vec<u8>)> = packs.iter().map(|pack| (pack.window_id.clone(), pack.files.pack.clone())).collect();
                    let reset = dispatch(&mut app, &model_left, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() }), 0).await?;
                    let [semio_framework::kernel::Effect::LoadDocument { pack, spr }] = reset.effects.as_slice() else {
                        return Err("FEM example command did not emit one host-owned document load".into());
                    };
                    app.load_document_pack(&store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() }).await.map_err(|error| format!("{error:?}"))?;
                    let after_example = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    let after_example: Vec<(String, Vec<u8>)> = after_example.iter().map(|pack| (pack.window_id.clone(), pack.files.pack.clone())).collect();
                    if before_example != after_example {
                        return Err("FEM example replacement changed exact window preferences".into());
                    }
                    if artifact_app_laws::capture_fixture_window_config::<model::config::Fem3dModelWindowConfigOwner, _, _>(&mut *app, &model_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&model_state)
                        || artifact_app_laws::capture_fixture_window_config::<results::config::Fem3dResultsWindowConfigOwner, _, _>(&mut *app, &results_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&result_state) {
                        return Err("FEM window values changed during document reset".into());
                    }
                    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Fem3dPlayApp>>(manifest).await);
                    reopened.bind_instance_id(72).await;
                    let expected: std::collections::BTreeMap<_, _> = packs.iter().map(|pack| ((pack.window_id.clone(), pack.window_kind_id.clone()), (pack.files.pack.clone(), pack.files.spr.clone()))).collect();
                    for pack in packs {
                        reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?;
                    }
                    let restored = reopened.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if restored.len() != 2 || restored.iter().any(|pack| pack.window_id.ends_with("right")) {
                        return Err("FEM window packs did not restore on exact owners".into());
                    }
                    let restored: std::collections::BTreeMap<_, _> = restored.into_iter().map(|pack| ((pack.window_id, pack.window_kind_id), (pack.files.pack, pack.files.spr))).collect();
                    if restored != expected {
                        return Err("FEM exact window Pack and SPR bytes changed during reload".into());
                    }
                    if artifact_app_laws::capture_fixture_window_config::<model::config::Fem3dModelWindowConfigOwner, _, _>(&mut *reopened, &model_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&model_state)
                        || artifact_app_laws::capture_fixture_window_config::<results::config::Fem3dResultsWindowConfigOwner, _, _>(&mut *reopened, &results_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&result_state) {
                        return Err("FEM window values changed during exact config reload".into());
                    }
                    artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                    let stale = ViewModel { window_id: Some("missing".into()), window_instances: all.window_instances.clone(), ..Default::default() };
                    if model::config::addressed(&stale, Default::default()).is_ok() {
                        return Err("FEM accepted a stale model window id".into());
                    }
                    let foreign = all.for_window_instance("foreign").expect("foreign");
                    if results::config::addressed(&foreign, Default::default()).is_ok() {
                        return Err("FEM accepted a foreign results window kind".into());
                    }
                    Ok(())
                })
                .await;
                artifact_app_laws::close_registered_fixture_app(&mut *app);
                outcome.expect("FEM exact-window ownership runtime law");
                eprintln!("[DEBUG] FEM fem3d isolated two same-kind instances per window kind, restored exact packs, and preserved app/document bytes");
            }))
        })
        .expect("spawn FEM window ownership law")
        .join()
        .expect("FEM window ownership law thread");
}
