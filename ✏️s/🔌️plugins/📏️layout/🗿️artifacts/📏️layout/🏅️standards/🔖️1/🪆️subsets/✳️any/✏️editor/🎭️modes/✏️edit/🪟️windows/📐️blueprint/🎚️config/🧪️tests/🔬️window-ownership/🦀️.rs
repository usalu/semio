use super::*;
use crate::editor::layout::modes::edit::windows::blueprint::transient::{LayoutWindowTransient, LayoutWindowTransientMutation};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn block_on_layout_windows<F: std::future::Future>(future: F) -> F::Output {
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
fn layout_window_ownership_runtime_isolates_restores_and_resets_exact_windows() {
    std::thread::Builder::new()
        .name("layout-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_layout_windows(async {
            use crate::editor::layout::commands::{engagement_input, set_camera};
            use crate::editor::layout::{create_layout_app, LayoutCommand, LayoutPlayApp, LAYOUT_PLAY_BODY_BLUEPRINT};
            use crate::editor::layout::modes::edit::windows::blueprint::transient::LayoutBlueprintWindowTransientOwner;
            use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

            async fn dispatch(app: &mut VcsArtifactApp<EditorApp<LayoutPlayApp>>, command: LayoutCommand, view: &ViewModel) -> Result<(usize, usize), String> {
                let meta = ActionMeta { instance_id: 81, view_state: Some(view.clone()), ..artifact_app_laws::meta("layout-window-ownership") };
                app.dispatch_typed(command, &meta).await.map_err(|error| format!("{error:?}"))?;
                let receipt = artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{error:?}"))?;
                Ok((
                    receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count(),
                    receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowTransient).count(),
                ))
            }

            async fn scene(app: &mut VcsArtifactApp<EditorApp<LayoutPlayApp>>, view: &ViewModel) -> Result<semio_framework_plugin::Canvas2dScene, String> {
                let tree = app.render(LAYOUT_PLAY_BODY_BLUEPRINT, None, view).await.map_err(|error| format!("{error:?}"))?;
                let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
            }

            let manifest = || App { definition: create_layout_app(), examples: Vec::new() };
            let view = ViewModel {
                window_instances: vec![
                    ViewWindowInstance { id: "layout-left".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() },
                    ViewWindowInstance { id: "layout-right".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() },
                    ViewWindowInstance { id: "layout-preview".into(), window_kind_id: crate::editor::layout::LAYOUT_PLAY_WINDOW_PREVIEW.into() },
                ],
                ..Default::default()
            };
            let left = view.for_window_instance("layout-left").unwrap();
            let right = view.for_window_instance("layout-right").unwrap();
            let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<LayoutPlayApp>>(manifest).await);
            app.bind_instance_id(81).await;
            let outcome: Result<(), String> = async {
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let mut lanes = (0, 0);
                for (context, camera) in [
                    (&left, LayoutCamera { x: 12.0, y: -8.0, zoom: 2.0 }),
                    (&right, LayoutCamera { x: -21.0, y: 5.0, zoom: 0.75 }),
                ] {
                    let next = dispatch(&mut app, LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: None, camera }), context).await?;
                    lanes.0 += next.0;
                    lanes.1 += next.1;
                }
                let next = dispatch(&mut app, LayoutCommand::EngagementInput(engagement_input::EngagementInput { value: "export png".into() }), &left).await?;
                lanes.0 += next.0;
                lanes.1 += next.1;
                if lanes != (2, 1) { return Err(format!("Layout exact-window lane count changed: {lanes:?}")); }
                let left_scene = scene(&mut app, &left).await?;
                let right_scene = scene(&mut app, &right).await?;
                if (left_scene.camera_x, left_scene.camera_y, left_scene.zoom) != (12.0, -8.0, 2.0)
                    || (right_scene.camera_x, right_scene.camera_y, right_scene.zoom) != (-21.0, 5.0, 0.75)
                {
                    return Err("Layout camera crossed exact window partitions".into());
                }
                let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != document_after.pack || document_before.spr != document_after.spr { return Err("Layout window publication changed document bytes".into()); }
                let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                app.load_document_pack(&document_before).await.map_err(|error| format!("{error:?}"))?;
                let cleared = app.window_transient_snapshot(&left).map_err(|error| format!("{error:?}"))?.ok_or("Layout transient owner missing after reload")?;
                if cleared.get::<LayoutBlueprintWindowTransientOwner>() != Some(&LayoutWindowTransient::default()) { return Err("Layout transient survived same-byte reload".into()); }
                for context in [&left, &right] {
                    if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))?.is_none() { return Err("Layout config was lost during reload".into()); }
                }
                let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<LayoutPlayApp>>(manifest).await);
                reopened.bind_instance_id(82).await;
                for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                let reopened_left = scene(&mut reopened, &left).await?;
                let reopened_right = scene(&mut reopened, &right).await?;
                artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                if reopened_left.camera_x != left_scene.camera_x || reopened_right.camera_x != right_scene.camera_x { return Err("Layout config changed during restore".into()); }
                let stale = ViewModel { window_id: Some("lost-layout".into()), window_instances: view.window_instances.clone(), ..Default::default() };
                if addressed(&stale, LayoutWindowConfig::default()).is_ok() { return Err("Layout accepted stale window identity".into()); }
                let wrong = view.for_window_instance("layout-preview").unwrap();
                let mutation = addressed(&wrong, LayoutWindowConfig::default()).map_err(|error| format!("{error:?}"))?;
                if mutation.window_id() != "layout-preview" { return Err("Layout preview config lost exact identity".into()); }
                Ok(())
            }.await;
            if let Err(error) = &outcome { eprintln!("[DEBUG] Layout exact-window runtime failure before close: {error}"); }
            artifact_app_laws::close_registered_fixture_app(&mut *app);
            outcome.expect("Layout exact-window runtime law");
            eprintln!("[DEBUG] Layout runtime isolated two Blueprint windows, restored persisted config, cleared transient on reload, and preserved document bytes");
        }))
        .expect("spawn Layout window ownership law")
        .join()
        .expect("Layout window ownership law thread");
}

#[test]
fn layout_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base_config: LayoutWindowConfig = dsl::json::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let base_transient: LayoutWindowTransient = dsl::json::from_json_str(&fixture["baseTransient"].to_string()).unwrap();
    for row in fixture["configMutations"].as_array().unwrap() {
        let mutation: LayoutWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_config).diff().apply(&base_config).unwrap();
        let restored = mutation.inverse(&base_config).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_config);
        assert_eq!(LayoutWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(LayoutWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for row in fixture["transientMutations"].as_array().unwrap() {
        let mutation: LayoutWindowTransientMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_transient).diff().apply(&base_transient).unwrap();
        let restored = mutation.inverse(&base_transient).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_transient);
        assert_eq!(LayoutWindowTransientMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(LayoutWindowTransientMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    eprintln!("[DEBUG] Layout config/transient mutations matched neutral fixture inverse, text, and binary laws");
}
