use super::*;
use crate::results_window_config::{addressed, ChangeSelectedCheckIndex, NormResultsWindowConfig, NormResultsWindowConfigMutation};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn block_on_norm_results_windows<F: std::future::Future>(future: F) -> F::Output {
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

/// 🧹️ A registered runtime fixture that always drives its exact retained close witness.
struct ClosingFixtureApp<A: semio_framework_plugin::PluginApp>(Box<A>);

impl<A: semio_framework_plugin::PluginApp> std::ops::Deref for ClosingFixtureApp<A> {
    type Target = A;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<A: semio_framework_plugin::PluginApp> std::ops::DerefMut for ClosingFixtureApp<A> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<A: semio_framework_plugin::PluginApp> Drop for ClosingFixtureApp<A> {
    fn drop(&mut self) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *self.0);
    }
}

#[test]
fn norm_results_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🪟️results/🎚️config/🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base: NormResultsWindowConfig = dsl::json::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let mut windows = std::collections::BTreeMap::<String, NormResultsWindowConfig>::new();
    for row in fixture["windowInstances"].as_array().unwrap() {
        if row["windowKindId"] == WINDOW_RESULTS {
            windows.insert(row["id"].as_str().unwrap().to_string(), base.clone());
        }
    }
    for row in fixture["mutations"].as_array().unwrap() {
        let id = row["windowId"].as_str().or_else(|| row["focusedWindowId"].as_str()).unwrap().to_string();
        let before = windows.get(&id).cloned().unwrap();
        let mutation: NormResultsWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&before).diff().apply(&before).unwrap();
        let restored = mutation.inverse(&before).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, before);
        assert_eq!(NormResultsWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(NormResultsWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        windows.insert(id, after);
    }
    for (id, expected) in fixture["expected"].as_object().unwrap() {
        let expected: NormResultsWindowConfig = dsl::json::from_json_str(&expected.to_string()).unwrap();
        assert_eq!(windows.get(id), Some(&expected));
        assert_eq!(NormResultsWindowConfig::parse_dsl(&expected.print_dsl()).unwrap(), expected);
        assert_eq!(NormResultsWindowConfig::decode_pack(&expected.encode_pack()).unwrap(), expected);
    }
    for row in fixture["undoMutations"].as_array().unwrap() {
        let id = row["windowId"].as_str().or_else(|| row["focusedWindowId"].as_str()).unwrap().to_string();
        let before = windows.get(&id).cloned().unwrap();
        let mutation: NormResultsWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        windows.insert(id, mutation.diff(&before).diff().apply(&before).unwrap());
    }
    assert!(windows.values().all(|config| config == &base));
    for row in fixture["redoMutations"].as_array().unwrap() {
        let id = row["windowId"].as_str().or_else(|| row["focusedWindowId"].as_str()).unwrap().to_string();
        let before = windows.get(&id).cloned().unwrap();
        let mutation: NormResultsWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        windows.insert(id, mutation.diff(&before).diff().apply(&before).unwrap());
    }
    for (id, expected) in fixture["expected"].as_object().unwrap() {
        let expected: NormResultsWindowConfig = dsl::json::from_json_str(&expected.to_string()).unwrap();
        assert_eq!(windows.get(id), Some(&expected));
    }
    eprintln!("[DEBUG] Norm Results-window config matched neutral fixture, inverse, text, Pack, and two-instance undo/redo laws");
}

#[test]
fn norm_results_window_ownership_runtime_isolates_focused_inspection_and_reopens() {
    std::thread::Builder::new()
        .name("norm-results-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_norm_results_windows(async {
            use crate::editor::en1996::commands::selected_check;
            use crate::editor::en1996::{create_en1996_app, En1996Command, En1996PlayApp};
            use crate::editor::en1996::modes::edit::windows::inputs;
            use crate::editor::en1996::panels::inspection;
            use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

            type NormApp = VcsArtifactApp<EditorApp<En1996PlayApp>>;

            async fn dispatch(app: &mut NormApp, instance_id: u32, index: Option<u32>, view: &ViewModel) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, String> {
                let meta = ActionMeta { instance_id, view_state: Some(view.clone()), ..artifact_app_laws::meta("norm-results-window-ownership") };
                app.dispatch_typed(En1996Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index }), &meta).await.map_err(|error| format!("{error:?}"))?;
                artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{error:?}"))
            }

            async fn render_inspection(app: &mut NormApp, view: &ViewModel) -> Result<String, String> {
                let tree = app.render(inspection::BODY_INSPECTION, None, view).await.map_err(|error| format!("{error:?}"))?;
                artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)
            }

            fn panel_view(roster: &[ViewWindowInstance], focused_window_id: &str) -> ViewModel {
                ViewModel { window_id: None, focused_window_id: Some(focused_window_id.into()), window_instances: roster.to_vec(), ..Default::default() }
            }

            let manifest = || App { definition: create_en1996_app(), examples: Vec::new() };
            let roster = vec![
                ViewWindowInstance { id: "norm-results-left".into(), window_kind_id: ResultsWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "norm-results-right".into(), window_kind_id: ResultsWindowConfigOwner::WINDOW_KIND_ID.into() },
                ViewWindowInstance { id: "norm-inputs".into(), window_kind_id: inputs::WINDOW_INPUTS.into() },
            ];
            let all = ViewModel { window_instances: roster.clone(), ..Default::default() };
            let left = all.for_window_instance("norm-results-left").expect("left Results window");
            let right = all.for_window_instance("norm-results-right").expect("right Results window");
            let left_panel = panel_view(&roster, "norm-results-left");
            let right_panel = panel_view(&roster, "norm-results-right");
            let mut app = ClosingFixtureApp(Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<En1996PlayApp>>(manifest).await));
            app.bind_instance_id(204).await;
            let outcome: Result<(), String> = async {
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let config_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                let left_receipt = dispatch(&mut app, 204, Some(1), &left_panel).await?;
                let left_render = render_inspection(&mut app, &left_panel).await?;
                let packs_after_left = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                if packs_after_left.len() != 1 || packs_after_left[0].window_id != "norm-results-left" {
                    return Err(format!("Norm left selection eagerly allocated another window config: {} packs", packs_after_left.len()));
                }
                let default_right_render = render_inspection(&mut app, &right_panel).await?;
                let default_right = artifact_app_laws::capture_fixture_window_config::<ResultsWindowConfigOwner, _, _>(&mut *app, &right)
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .ok_or_else(|| "missing lazily rendered right Results config".to_string())?;
                if default_right.selected_check_index.is_some() || default_right_render == left_render || default_right_render.contains("No checks") {
                    return Err(format!("Norm untouched Results window did not render the default check at index zero: config={default_right:?}"));
                }
                let right_receipt = dispatch(&mut app, 204, Some(2), &right).await?;
                for receipt in [&left_receipt, &right_receipt] {
                    use semio_framework_plugin::app::TypedOperationResultLane;
                    if receipt.lanes.iter().filter(|lane| **lane == TypedOperationResultLane::WindowConfig).count() != 1
                        || receipt.lanes.iter().any(|lane| matches!(lane, TypedOperationResultLane::Artifact | TypedOperationResultLane::Config | TypedOperationResultLane::Draft | TypedOperationResultLane::Presence | TypedOperationResultLane::Transient | TypedOperationResultLane::WindowTransient | TypedOperationResultLane::Child | TypedOperationResultLane::Interaction | TypedOperationResultLane::Effect | TypedOperationResultLane::Download | TypedOperationResultLane::Fault))
                    {
                        return Err(format!("Norm selection published a missing or forbidden ownership lane: {:?}", receipt.lanes));
                    }
                }
                let left_config = artifact_app_laws::capture_fixture_window_config::<ResultsWindowConfigOwner, _, _>(&mut *app, &left).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing left Results config".to_string())?;
                let right_config = artifact_app_laws::capture_fixture_window_config::<ResultsWindowConfigOwner, _, _>(&mut *app, &right).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing right Results config".to_string())?;
                if left_config.selected_check_index != Some(1) || right_config.selected_check_index != Some(2) {
                    return Err(format!("Norm selection crossed exact windows: left={left_config:?}, right={right_config:?}"));
                }
                let right_render = render_inspection(&mut app, &right_panel).await?;
                if left_render == right_render || left_render.contains("No checks") || right_render.contains("No checks") {
                    return Err("Norm inspection did not consume isolated focused Results config".into());
                }
                let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let config_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != document_after.pack || document_before.spr != document_after.spr || config_before.pack != config_after.pack || config_before.spr != config_after.spr {
                    return Err("Norm Results-window selection changed document or app config bytes".into());
                }
                let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                if packs.len() != 2 { return Err(format!("Norm persisted {} Results packs instead of two", packs.len())); }
                let mut reopened = ClosingFixtureApp(Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<En1996PlayApp>>(manifest).await));
                reopened.bind_instance_id(205).await;
                let reopened_outcome: Result<(NormResultsWindowConfig, NormResultsWindowConfig), String> = async {
                    for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                    if render_inspection(&mut reopened, &left_panel).await? != left_render || render_inspection(&mut reopened, &right_panel).await? != right_render {
                        return Err("Norm focused inspection changed during Results-window reopen".into());
                    }
                    dispatch(&mut reopened, 205, Some(3), &left_panel).await?;
                    let reopened_left = artifact_app_laws::capture_fixture_window_config::<ResultsWindowConfigOwner, _, _>(&mut *reopened, &left).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing reopened left config".to_string())?;
                    let reopened_right = artifact_app_laws::capture_fixture_window_config::<ResultsWindowConfigOwner, _, _>(&mut *reopened, &right).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing reopened right config".to_string())?;
                    Ok((reopened_left, reopened_right))
                }
                .await;
                if let Err(error) = &reopened_outcome { eprintln!("[DEBUG] Norm Results reopened-app failure before close: {error}"); }
                drop(reopened);
                let (reopened_left, reopened_right) = reopened_outcome?;
                if reopened_left.selected_check_index != Some(3) || reopened_right != right_config { return Err("Norm edit after reload lost same-kind instance isolation".into()); }
                let stale = ViewModel { window_id: Some("norm-results-missing".into()), window_instances: roster.clone(), ..Default::default() };
                let wrong = all.for_window_instance("norm-inputs").expect("Inputs window");
                let absent = ViewModel { window_instances: roster, ..Default::default() };
                let mutation = || NormResultsWindowConfigMutation::ChangeSelectedCheckIndex(ChangeSelectedCheckIndex { index: Some(4) });
                if addressed::<ResultsWindowConfigOwner>(&stale, mutation()).is_ok() || addressed::<ResultsWindowConfigOwner>(&wrong, mutation()).is_ok() || addressed::<ResultsWindowConfigOwner>(&absent, mutation()).is_ok() {
                    return Err("Norm Results-window address admitted stale, wrong-kind, or absent identity".into());
                }
                Ok(())
            }
            .await;
            if let Err(error) = &outcome { eprintln!("[DEBUG] Norm Results exact-window runtime failure before close: {error}"); }
            drop(app);
            outcome.expect("Norm Results exact-window ownership runtime law");
            eprintln!("[DEBUG] Norm Results selection isolated and reopened two same-kind windows, rendered focused Inspection, preserved document/app config bytes, rejected invalid identities, and closed on an 8 MiB stack");
        }))
        .expect("spawn Norm Results window ownership law")
        .join()
        .expect("Norm Results window ownership law thread");
}
