use super::*;
use crate::editor::sequence::modes::edit::windows::script::transient::{SequenceScriptWindowTransient, SequenceScriptWindowTransientMutation, SequenceScriptWindowTransientOwner};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

fn block_on_sequence_windows<F: std::future::Future>(future: F) -> F::Output {
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
fn sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows() {
    std::thread::Builder::new()
        .name("sequence-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_sequence_windows(async {
            use crate::editor::sequence::commands::node_graph::set_viewport;
            use crate::editor::sequence::commands::playback::run_command;
            use crate::editor::sequence::commands::step::add_step;
            use crate::editor::sequence::{SequenceCommand, SEQUENCE_PLAY_BODY_MAIN, SEQUENCE_PLAY_BODY_SCRIPT};
            use crate::editor::sequence::unit_tests::context::{new_app_with_registry_wired, SequenceApp};
            use semio_framework_plugin::{artifact_app_laws, ActionMeta, PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner, WindowTransientOwner};

            async fn dispatch(app: &mut SequenceApp, command: SequenceCommand, view: &ViewModel) -> Result<(usize, usize, usize), String> {
                let command_id = command.command_id();
                let meta = ActionMeta { instance_id: 83, view_state: Some(view.clone()), ..artifact_app_laws::meta("sequence-window-ownership") };
                app.dispatch_typed(command, &meta).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                let receipt = artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                let lanes = (
                    receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count(),
                    receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowTransient).count(),
                    receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Child).count(),
                );
                Ok(lanes)
            }
            async fn graph_scene(app: &mut SequenceApp, view: &ViewModel) -> Result<semio_framework_plugin::NodeGraphScene, String> {
                let tree = app.render(SEQUENCE_PLAY_BODY_MAIN, None, view).await.map_err(|error| format!("{error:?}"))?;
                let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
            }
            async fn script_scene(app: &mut SequenceApp, view: &ViewModel) -> Result<semio_framework_plugin::TextEditorScene, String> {
                let tree = app.render(SEQUENCE_PLAY_BODY_SCRIPT, None, view).await.map_err(|error| format!("{error:?}"))?;
                let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                artifact_app_laws::decode_fixture_scene(&json).map_err(str::to_string)
            }

            let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
            let ids: Vec<&str> = fixture["windowInstances"].as_array().unwrap().iter().map(|row| row["id"].as_str().unwrap()).collect();
            let view = ViewModel {
                window_instances: vec![
                    ViewWindowInstance { id: ids[0].into(), window_kind_id: SequenceMainWindowConfigOwner::WINDOW_KIND_ID.into() },
                    ViewWindowInstance { id: ids[1].into(), window_kind_id: SequenceMainWindowConfigOwner::WINDOW_KIND_ID.into() },
                    ViewWindowInstance { id: ids[2].into(), window_kind_id: SequenceScriptWindowTransientOwner::WINDOW_KIND_ID.into() },
                    ViewWindowInstance { id: ids[3].into(), window_kind_id: SequenceScriptWindowTransientOwner::WINDOW_KIND_ID.into() },
                ],
                ..Default::default()
            };
            let main_left = view.for_window_instance(ids[0]).unwrap();
            let main_right = view.for_window_instance(ids[1]).unwrap();
            let script_left = view.for_window_instance(ids[2]).unwrap();
            let script_right = view.for_window_instance(ids[3]).unwrap();
            let mut app = Box::new(new_app_with_registry_wired().await);
            app.bind_instance_id(83).await;
            let outcome: Result<(), String> = async {
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let mut lanes = (0, 0, 0);
                for (context, camera) in [
                    (&main_left, SequenceCamera { x: 13.0, y: -5.0, zoom: 2.0 }),
                    (&main_right, SequenceCamera { x: -8.0, y: 21.0, zoom: 0.5 }),
                ] {
                    let next = dispatch(&mut app, SequenceCommand::SetViewport(set_viewport::SetViewport { camera }), context).await?;
                    lanes.0 += next.0;
                    lanes.1 += next.1;
                    lanes.2 += next.2;
                }
                let next = dispatch(&mut app, SequenceCommand::Run(run_command::Run {}), &script_left).await?;
                lanes.0 += next.0;
                lanes.1 += next.1;
                lanes.2 += next.2;
                if lanes != (2, 1, 0) { return Err(format!("Sequence exact-window lane count changed: {lanes:?}")); }
                let left_graph = graph_scene(&mut app, &main_left).await?.viewport.ok_or("left Sequence viewport missing")?;
                let right_graph = graph_scene(&mut app, &main_right).await?.viewport.ok_or("right Sequence viewport missing")?;
                if (left_graph.x, left_graph.y, left_graph.zoom) != (13.0, -5.0, 2.0)
                    || (right_graph.x, right_graph.y, right_graph.zoom) != (-8.0, 21.0, 0.5)
                {
                    return Err("Sequence camera crossed exact window partitions".into());
                }
                let left_script = script_scene(&mut app, &script_left).await?;
                let right_script = script_scene(&mut app, &script_right).await?;
                if !left_script.buffer.contains("# run result") || right_script.buffer.contains("# run result") {
                    return Err("Sequence run result crossed exact script window partitions".into());
                }
                let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != document_after.pack || document_before.spr != document_after.spr { return Err("Sequence window publication changed document bytes".into()); }
                let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                app.load_document_pack(&document_before).await.map_err(|error| format!("{error:?}"))?;
                let cleared = app.window_transient_snapshot(&script_left).map_err(|error| format!("{error:?}"))?.ok_or("Sequence transient owner missing after reload")?;
                if cleared.get::<SequenceScriptWindowTransientOwner>() != Some(&SequenceScriptWindowTransient::default()) { return Err("Sequence transient survived same-byte reload".into()); }
                for context in [&main_left, &main_right] {
                    if app.window_config_generation(context).await.map_err(|error| format!("{error:?}"))?.is_none() { return Err("Sequence config was lost during reload".into()); }
                }
                let cleared_left_script = script_scene(&mut app, &script_left).await?;
                if cleared_left_script.buffer.contains("# run result") { return Err("Sequence render retained cleared transient".into()); }
                let before_edit = graph_scene(&mut app, &main_left).await?;
                let edit = &fixture["childEditAfterReload"];
                let edit_lanes = dispatch(&mut app, SequenceCommand::AddStep(add_step::AddStep {
                    kind: edit["kind"].as_str().ok_or("Sequence child edit kind missing")?.into(),
                    x: edit["x"].as_f64().ok_or("Sequence child edit x missing")?,
                    y: edit["y"].as_f64().ok_or("Sequence child edit y missing")?,
                }), &main_left).await?;
                if edit_lanes != (0, 0, 1) { return Err(format!("Sequence child edit published wrong lanes after reload: {edit_lanes:?}")); }
                let after_edit = graph_scene(&mut app, &main_left).await?;
                let expected_delta = edit["expectedNodeDelta"].as_u64().ok_or("Sequence child edit delta missing")? as usize;
                if after_edit.nodes.len() != before_edit.nodes.len() + expected_delta { return Err("Sequence child edit was orphaned after reload".into()); }
                let parent_after_edit = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != parent_after_edit.pack || document_before.spr != parent_after_edit.spr { return Err("Sequence child edit changed parent document bytes".into()); }
                let mut reopened = Box::new(new_app_with_registry_wired().await);
                reopened.bind_instance_id(84).await;
                for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                let reopened_left = graph_scene(&mut reopened, &main_left).await?.viewport.ok_or("reopened left Sequence viewport missing")?;
                let reopened_right = graph_scene(&mut reopened, &main_right).await?.viewport.ok_or("reopened right Sequence viewport missing")?;
                artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                if reopened_left != left_graph || reopened_right != right_graph { return Err("Sequence persisted window config changed during restore".into()); }
                let stale = ViewModel { window_id: Some("lost-sequence-window".into()), window_instances: view.window_instances.clone(), ..Default::default() };
                if addressed(&stale, SequenceMainWindowConfig::default()).is_ok() { return Err("Sequence accepted stale window identity".into()); }
                if crate::editor::sequence::modes::edit::windows::script::transient::addressed(&main_left, SequenceScriptWindowTransient::default()).is_ok() {
                    return Err("Sequence accepted wrong-kind window identity".into());
                }
                Ok(())
            }.await;
            if let Err(error) = &outcome { eprintln!("[DEBUG] Sequence exact-window runtime failure before close: {error}"); }
            artifact_app_laws::close_registered_fixture_app(&mut *app);
            outcome.expect("Sequence exact-window runtime law");
            eprintln!("[DEBUG] Sequence runtime isolated two main cameras and two script results, restored config, cleared transient on reload, and preserved document bytes");
        }))
        .expect("spawn Sequence window ownership law")
        .join()
        .expect("Sequence window ownership law thread");
}

#[test]
fn sequence_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base_config: SequenceMainWindowConfig = dsl::json::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let base_transient: SequenceScriptWindowTransient = dsl::json::from_json_str(&fixture["baseTransient"].to_string()).unwrap();
    for row in fixture["configMutations"].as_array().unwrap() {
        let mutation: SequenceMainWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_config).diff().apply(&base_config).unwrap();
        let restored = mutation.inverse(&base_config).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_config);
        assert_eq!(SequenceMainWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceMainWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for row in fixture["transientMutations"].as_array().unwrap() {
        let mutation: SequenceScriptWindowTransientMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_transient).diff().apply(&base_transient).unwrap();
        let restored = mutation.inverse(&base_transient).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_transient);
        assert_eq!(SequenceScriptWindowTransientMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceScriptWindowTransientMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    eprintln!("[DEBUG] Sequence config/transient mutations matched neutral fixture inverse, text, and binary laws");
}
