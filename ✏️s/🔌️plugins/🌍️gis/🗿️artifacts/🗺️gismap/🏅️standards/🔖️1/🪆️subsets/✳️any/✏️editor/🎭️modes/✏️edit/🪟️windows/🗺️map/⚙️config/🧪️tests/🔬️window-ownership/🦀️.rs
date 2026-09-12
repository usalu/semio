use super::*;
use super::mutations as map_config_mutations;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn block_on_gis_map_windows<F: std::future::Future>(future: F) -> F::Output {
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
fn gis_map_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base: MapWindowConfig = dsl::json::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let mut windows = std::collections::BTreeMap::<String, MapWindowConfig>::new();
    for row in fixture["windowInstances"].as_array().unwrap() {
        if row["windowKindId"] == "gis2d-main" {
            windows.insert(row["id"].as_str().unwrap().to_string(), base.clone());
        }
    }
    for row in fixture["mutations"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap().to_string();
        let before = windows.get(&id).cloned().unwrap();
        let mutation: MapWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&before).diff().apply(&before).unwrap();
        let restored = mutation.inverse(&before).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, before);
        assert_eq!(MapWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(MapWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        windows.insert(id, after);
    }
    for (id, expected) in fixture["expected"].as_object().unwrap() {
        let expected: MapWindowConfig = dsl::json::from_json_str(&expected.to_string()).unwrap();
        assert_eq!(windows.get(id), Some(&expected));
    }
    for row in fixture["undoMutations"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap().to_string();
        let before = windows.get(&id).cloned().unwrap();
        let mutation: MapWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        windows.insert(id, mutation.diff(&before).diff().apply(&before).unwrap());
    }
    assert!(windows.values().all(|config| config == &base));
    for row in fixture["redoMutations"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap().to_string();
        let before = windows.get(&id).cloned().unwrap();
        let mutation: MapWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        windows.insert(id, mutation.diff(&before).diff().apply(&before).unwrap());
    }
    for (id, expected) in fixture["expected"].as_object().unwrap() {
        let expected: MapWindowConfig = dsl::json::from_json_str(&expected.to_string()).unwrap();
        assert_eq!(windows.get(id), Some(&expected));
    }
    eprintln!("[DEBUG] GIS Map window config matched neutral fixture inverse, text, binary, left-only undo/redo, and two-instance isolation laws");
}

#[test]
fn gis_map_window_ownership_runtime_isolates_renders_and_reopens_two_map_windows() {
    std::thread::Builder::new()
        .name("gis-map-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_gis_map_windows(async {
            use crate::editor::gis2d::commands::example::set_active_example;
            use crate::editor::gis2d::commands::view::{fit_world, set_camera, set_layer_stroke_scale, set_lod_mode, set_render_mode, set_vector_style, toggle_layer_visibility};
            use crate::editor::gis2d::{create_gis2d_app, Gis2dCommand, Gis2dPlayApp};
            use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner, WindowMeasure};

            async fn dispatch(
                app: &mut VcsArtifactApp<EditorApp<Gis2dPlayApp>>,
                command: Gis2dCommand,
                view: &ViewModel,
            ) -> Result<usize, String> {
                let command_id = command.command_id();
                let meta = ActionMeta { instance_id: 86, view_state: Some(view.clone()), ..artifact_app_laws::meta("gis-map-window-ownership") };
                app.dispatch_typed(command, &meta).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                let receipt = artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                Ok(receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count())
            }

            async fn render(app: &mut VcsArtifactApp<EditorApp<Gis2dPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::TiledMapScene, String> {
                let tree = app.render(crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_BODY_COMPOSITE, None, view).await.map_err(|error| format!("{error:?}"))?;
                let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
            }

            async fn measures(app: &mut VcsArtifactApp<EditorApp<Gis2dPlayApp>>, view: &ViewModel) -> Vec<WindowMeasure> {
                app.window_measures(view).await.remove(view.window_id.as_deref().unwrap_or_default()).unwrap_or_default()
            }

            fn same_window_config(left: &semio_framework_plugin::TiledMapScene, right: &semio_framework_plugin::TiledMapScene) -> bool {
                let same_json = |left: &str, right: &str| {
                    serde_json::from_str::<serde_json::Value>(left).ok() == serde_json::from_str::<serde_json::Value>(right).ok()
                };
                left.camera_json == right.camera_json
                    && left.render_mode == right.render_mode
                    && left.vector_style == right.vector_style
                    && left.lod_mode == right.lod_mode
                    && same_json(&left.layer_visibility_json, &right.layer_visibility_json)
                    && same_json(&left.layer_stroke_scale_json, &right.layer_stroke_scale_json)
            }

            fn select_value(rows: &[WindowMeasure], id: &str) -> Option<String> {
                rows.iter().find_map(|row| match row {
                    WindowMeasure::Select { id: row_id, value, .. } if row_id == id => Some(value.clone()),
                    _ => None,
                })
            }

            fn toggle_value(rows: &[WindowMeasure], id: &str) -> Option<bool> {
                rows.iter().find_map(|row| match row {
                    WindowMeasure::Toggle { id: row_id, pressed, .. } if row_id == id => Some(*pressed),
                    WindowMeasure::Group { children, .. } => toggle_value(children, id),
                    _ => None,
                })
            }

            fn slider_value(rows: &[WindowMeasure], id: &str) -> Option<f64> {
                rows.iter().find_map(|row| match row {
                    WindowMeasure::Slider { id: row_id, value, .. } if row_id == id => Some(*value),
                    WindowMeasure::Group { children, .. } => slider_value(children, id),
                    _ => None,
                })
            }

            let manifest = || App { definition: create_gis2d_app(), examples: Vec::new() };
            let roster = vec![
                ViewWindowInstance { id: "gis-map-left".into(), window_kind_id: MapWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "gis-map-right".into(), window_kind_id: MapWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "gis-map-other".into(), window_kind_id: "other-window".into() },
            ];
            let view = ViewModel { window_instances: roster, ..Default::default() };
            let left = view.for_window_instance("gis-map-left").unwrap();
            let right = view.for_window_instance("gis-map-right").unwrap();
            let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Gis2dPlayApp>>(manifest).await);
            app.bind_instance_id(86).await;
            let outcome: Result<(), String> = async {
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let publications =
                    dispatch(&mut app, Gis2dCommand::FitWorld(fit_world::FitWorld {}), &left).await?
                    + dispatch(&mut app, Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }), &left).await?
                    + dispatch(&mut app, Gis2dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "city".into() }), &left).await?
                    + dispatch(&mut app, Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "positions".into(), value: 2.5 }), &left).await?
                    + dispatch(&mut app, Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":-21,"y":5,"zoom":0.75}"#.into() }), &right).await?
                    + dispatch(&mut app, Gis2dCommand::SetVectorStyle(set_vector_style::SetVectorStyle { value: "figureGround".into() }), &right).await?
                    + dispatch(&mut app, Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() }), &right).await?;
                if publications != 7 { return Err(format!("GIS Map exact-window lane count changed: {publications}")); }
                let document_after_view = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != document_after_view.pack || document_before.spr != document_after_view.spr { return Err("GIS Map view commands changed document bytes".into()); }
                let left_render = render(&mut app, &left).await?;
                let right_render = render(&mut app, &right).await?;
                let left_strokes: serde_json::Value = serde_json::from_str(&left_render.layer_stroke_scale_json).map_err(|error| error.to_string())?;
                let right_visibility: serde_json::Value = serde_json::from_str(&right_render.layer_visibility_json).map_err(|error| error.to_string())?;
                if left_render.render_mode != "vector"
                    || left_render.lod_mode != "city"
                    || left_strokes["positions"] != 2.5
                    || right_render.vector_style != "figureGround"
                    || right_visibility["water"] != false
                    || left_render.camera_json == right_render.camera_json
                {
                    return Err("GIS Map render did not consume all isolated exact-window fields".into());
                }
                let left_measures = measures(&mut app, &left).await;
                let right_measures = measures(&mut app, &right).await;
                if select_value(&left_measures, "gis2d-play-window.render-mode").as_deref() != Some("vector")
                    || select_value(&left_measures, "gis2d-play-window.lod-mode").as_deref() != Some("city")
                    || slider_value(&left_measures, "gis2d-play-window.weight.positions") != Some(2.5)
                    || select_value(&right_measures, "gis2d-play-window.vector-style").as_deref() != Some("figureGround")
                    || toggle_value(&right_measures, "gis2d-play-window.layer.water") != Some(false)
                {
                    return Err("GIS Map chrome did not consume isolated exact-window state".into());
                }
                let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                if packs.len() != 2 { return Err(format!("GIS Map persisted {} window packs instead of two", packs.len())); }
                let persisted_payloads = packs
                    .iter()
                    .map(|pack| (pack.window_id.clone(), pack.window_kind_id.clone(), pack.files.pack.clone(), pack.files.spr.clone()))
                    .collect::<Vec<_>>();
                let persisted_left = artifact_app_laws::capture_fixture_window_config::<MapWindowConfigOwner, _, _>(&mut *app, &left)
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .ok_or_else(|| "GIS Map left config disappeared before persistence".to_string())?;
                let persisted_right = artifact_app_laws::capture_fixture_window_config::<MapWindowConfigOwner, _, _>(&mut *app, &right)
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .ok_or_else(|| "GIS Map right config disappeared before persistence".to_string())?;
                dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }), &left).await?;
                if !app.snapshot().map_err(|error| format!("{error:?}"))?.positions.is_empty() { return Err("GIS Map empty example did not clear the document".into()); }
                dispatch(&mut app, Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":999,"y":999,"zoom":9}"#.into() }), &left).await?;
                let camera_before_example = render(&mut app, &left).await?.camera_json;
                if dispatch(&mut app, Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "reuse-map".into() }), &left).await? != 1 {
                    return Err("GIS Map example did not publish its fitted camera to the exact window".into());
                }
                if app.snapshot().map_err(|error| format!("{error:?}"))?.positions.is_empty() { return Err("GIS Map reuse example did not restore document positions".into()); }
                if render(&mut app, &left).await?.camera_json == camera_before_example { return Err("GIS Map example camera was not captured by its caller window".into()); }
                let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Gis2dPlayApp>>(manifest).await);
                reopened.bind_instance_id(87).await;
                for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                let reopened_payloads = reopened
                    .window_config_packs()
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .into_iter()
                    .map(|pack| (pack.window_id, pack.window_kind_id, pack.files.pack, pack.files.spr))
                    .collect::<Vec<_>>();
                let reopened_left_config = artifact_app_laws::capture_fixture_window_config::<MapWindowConfigOwner, _, _>(&mut *reopened, &left)
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .ok_or_else(|| "GIS Map reopened left config is absent".to_string())?;
                let reopened_right_config = artifact_app_laws::capture_fixture_window_config::<MapWindowConfigOwner, _, _>(&mut *reopened, &right)
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .ok_or_else(|| "GIS Map reopened right config is absent".to_string())?;
                let reopened_left = render(&mut reopened, &left).await?;
                let reopened_right = render(&mut reopened, &right).await?;
                artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                if reopened_payloads != persisted_payloads
                    || reopened_left_config != persisted_left
                    || reopened_right_config != persisted_right
                    || !same_window_config(&reopened_left, &left_render)
                    || !same_window_config(&reopened_right, &right_render)
                {
                    return Err("GIS Map persisted exact-window config changed during reopen".into());
                }
                let stale = ViewModel { window_id: Some("gis-map-missing".into()), window_instances: view.window_instances.clone(), ..Default::default() };
                if addressed(&stale, MapWindowConfigMutation::SetCamera(map_config_mutations::SetCamera { camera_json: "{}".into() })).is_ok() { return Err("GIS Map accepted stale window identity".into()); }
                let wrong = view.for_window_instance("gis-map-other").unwrap();
                if addressed(&wrong, MapWindowConfigMutation::SetCamera(map_config_mutations::SetCamera { camera_json: "{}".into() })).is_ok() { return Err("GIS Map accepted the wrong concrete window kind".into()); }
                Ok(())
            }.await;
            if let Err(error) = &outcome { eprintln!("[DEBUG] GIS Map exact-window runtime failure before close: {error}"); }
            artifact_app_laws::close_registered_fixture_app(&mut *app);
            outcome.expect("GIS Map exact-window runtime law");
            eprintln!("[DEBUG] GIS Map retained commands isolated, rendered, measured, and reopened two concrete Map windows");
        }))
        .expect("spawn GIS Map window ownership law")
        .join()
        .expect("GIS Map window ownership law thread");
}
