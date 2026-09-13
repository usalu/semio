use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_plugin::WindowConfigOwner;
use store::{ArtifactDsl, ArtifactPack};

fn block_on_generation2d_window_ownership<F: std::future::Future>(mut future: std::pin::Pin<Box<F>>) -> F::Output {
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

macro_rules! assert_window_config_codecs {
    ($state:ty, $mutation:ty, $base:expr, $next:expr, $operation:expr) => {{
        let base: $state = $base;
        let next: $state = $next;
        let operation: $mutation = $operation;
        let applied = operation.diff(&base).diff().apply(&base).expect("window-config diff applies");
        assert_eq!(applied, next);
        let restored = operation.inverse(&base).into_iter().fold(applied, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("window-config inverse applies"));
        assert_eq!(restored, base);
        assert_eq!(<$state>::parse_dsl(&base.print_dsl()).expect("window-config DSL round-trip"), base);
        assert_eq!(<$state>::decode_pack(&base.encode_pack()).expect("window-config Pack round-trip"), base);
        assert_eq!(<$mutation>::parse_op(&operation.print_op()).expect("window-config text-op round-trip"), operation);
        assert_eq!(<$mutation>::decode_op(&operation.encode_op().expect("window-config binary-op encoding")).expect("window-config binary-op round-trip"), operation);

        let mut hostile_state: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&base)).expect("window-config state JSON");
        hostile_state.as_object_mut().expect("window-config state object").insert("foreign".into(), serde_json::Value::Bool(true));
        assert!(dsl::json::from_json_str::<$state>(&hostile_state.to_string()).is_err(), "window-config state must reject unknown fields");
        let mut hostile_mutation: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&operation)).expect("window-config mutation JSON");
        hostile_mutation.as_object_mut().expect("window-config mutation object").insert("foreign".into(), serde_json::Value::Bool(true));
        assert!(dsl::json::from_json_str::<$mutation>(&hostile_mutation.to_string()).is_err(), "window-config mutation must reject unknown fields");

        let valid_pack = base.encode_pack();
        let (_, body) = store::semio_format::unwrap_binary(&valid_pack).expect("window-config Pack envelope");
        assert!(<$state>::decode_pack(&body).is_err(), "window-config Pack must reject a missing envelope");
        let foreign = store::semio_format::SemioEnvelope::from_envelope_id("procedural.generation2d.foreignwindowconfig", store::semio_format::Component::Pack, 1).expect("foreign window-config envelope");
        assert!(<$state>::decode_pack(&store::semio_format::wrap_binary(&foreign, &body)).is_err(), "window-config Pack must reject a foreign owner");
        let wrong_component = store::semio_format::SemioEnvelope::from_envelope_id(<$state as ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("wrong-component window-config envelope");
        assert!(<$state>::decode_pack(&store::semio_format::wrap_binary(&wrong_component, &body)).is_err(), "window-config Pack must reject a foreign component");
        let wrong_version = store::semio_format::SemioEnvelope::from_envelope_id(<$state as ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 2).expect("wrong-version window-config envelope");
        assert!(<$state>::decode_pack(&store::semio_format::wrap_binary(&wrong_version, &body)).is_err(), "window-config Pack must reject a foreign version");
    }};
}

#[test]
fn generation2d_window_camera_ownership_matches_neutral_fixture_and_exact_codecs() {
    use crate::editor::generation2d::modes::edit::windows::{flow, preview as edit_preview};
    use crate::editor::generation2d::modes::generate::windows::preview as generate_preview;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("neutral Generation2d window fixture");
    let main_base: flow::config::Generation2dMainWindowConfig = dsl::json::from_json_str(&fixture["baseConfigs"][flow::GENERATION2D_PLAY_WINDOW_MAIN].to_string()).expect("neutral main config");
    let main_next: flow::config::Generation2dMainWindowConfig = dsl::json::from_json_str(&fixture["expected"]["main-left"].to_string()).expect("neutral main next config");
    let main_mutation = flow::config::Generation2dMainWindowConfigMutation::Snapshot { config: Box::new(main_next.clone()) };
    assert_window_config_codecs!(flow::config::Generation2dMainWindowConfig, flow::config::Generation2dMainWindowConfigMutation, main_base, main_next, main_mutation);

    let edit_base: edit_preview::config::Generation2dEditPreviewWindowConfig = dsl::json::from_json_str(&fixture["baseConfigs"][edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW].to_string()).expect("neutral edit-preview config");
    let edit_next: edit_preview::config::Generation2dEditPreviewWindowConfig = dsl::json::from_json_str(&fixture["expected"]["edit-left"].to_string()).expect("neutral edit-preview next config");
    let edit_mutation = edit_preview::config::Generation2dEditPreviewWindowConfigMutation::Snapshot { config: Box::new(edit_next.clone()) };
    assert_window_config_codecs!(edit_preview::config::Generation2dEditPreviewWindowConfig, edit_preview::config::Generation2dEditPreviewWindowConfigMutation, edit_base, edit_next, edit_mutation);

    let generate_base: generate_preview::config::Generation2dGeneratePreviewWindowConfig = dsl::json::from_json_str(&fixture["baseConfigs"][generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW].to_string()).expect("neutral generate-preview config");
    let generate_next: generate_preview::config::Generation2dGeneratePreviewWindowConfig = dsl::json::from_json_str(&fixture["expected"]["generate-left"].to_string()).expect("neutral generate-preview next config");
    let generate_mutation = generate_preview::config::Generation2dGeneratePreviewWindowConfigMutation::Snapshot { config: Box::new(generate_next.clone()) };
    assert_window_config_codecs!(generate_preview::config::Generation2dGeneratePreviewWindowConfig, generate_preview::config::Generation2dGeneratePreviewWindowConfigMutation, generate_base, generate_next, generate_mutation);

    assert_ne!(<flow::config::Generation2dMainWindowConfig as ArtifactDsl>::envelope_id(), <edit_preview::config::Generation2dEditPreviewWindowConfig as ArtifactDsl>::envelope_id());
    assert_ne!(<edit_preview::config::Generation2dEditPreviewWindowConfig as ArtifactDsl>::envelope_id(), <generate_preview::config::Generation2dGeneratePreviewWindowConfig as ArtifactDsl>::envelope_id());
    let main_pack = flow::config::Generation2dMainWindowConfig::default().encode_pack();
    assert!(edit_preview::config::Generation2dEditPreviewWindowConfig::decode_pack(&main_pack).is_err(), "a main owner Pack must not decode as edit preview");
    assert!(generate_preview::config::Generation2dGeneratePreviewWindowConfig::decode_pack(&main_pack).is_err(), "a main owner Pack must not decode as generate preview");
}

#[test]
fn generation2d_window_camera_raw_handlers_are_explicit_no_ops() {
    use crate::editor::generation2d::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, node_graph_viewport};
    use crate::editor::generation2d::unit_tests::context::{empty_history_view, retire_flow_eval_session};

    let snapshot = Generation2dSnapshot::default();
    let history = empty_history_view();
    let config = Generation2dConfig::default();
    let document = ArtifactView::new(&snapshot, &history);
    let config_view = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let viewport = node_graph_viewport::handle(&node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 7.0, y: -4.0, zoom: 2.0 } }, &document, &config_view, &mut session).expect("raw NodeGraph viewport handler");
    assert!(viewport.artifact_mutations.is_empty() && viewport.config_mutations.is_empty() && viewport.window_config_mutations.is_empty() && viewport.draft_mutations.is_empty() && viewport.effects.is_empty() && viewport.events.is_empty());
    let canvas = [
        canvas_pointer_down::handle(&canvas_pointer_down::CanvasPointerDown {}, &document, &config_view, &mut session).expect("raw Canvas pointer-down handler"),
        canvas_pointer_move::handle(&canvas_pointer_move::CanvasPointerMove {}, &document, &config_view, &mut session).expect("raw Canvas pointer-move handler"),
        canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp {}, &document, &config_view, &mut session).expect("raw Canvas pointer-up handler"),
        canvas_wheel::handle(&canvas_wheel::CanvasWheel {}, &document, &config_view, &mut session).expect("raw Canvas wheel handler"),
    ];
    assert!(canvas.iter().all(|emit| emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.window_config_mutations.is_empty() && emit.draft_mutations.is_empty() && emit.effects.is_empty() && emit.events.is_empty()));
    retire_flow_eval_session(session);
}

#[test]
fn generation2d_window_camera_ownership_runtime_isolates_routes_renders_and_reopens() {
    std::thread::Builder::new()
        .name("generation2d-window-camera-ownership-law".into())
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            block_on_generation2d_window_ownership(Box::pin(async {
                use crate::editor::generation2d::commands::{add_generation, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, node_graph_viewport};
                use crate::editor::generation2d::modes::edit::windows::{flow, preview as edit_preview};
                use crate::editor::generation2d::modes::generate::windows::preview as generate_preview;
                use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigPack};

                type TestApp = VcsArtifactApp<EditorApp<Generation2dPlayApp>>;
                fn manifest() -> App { crate::editor::generation2d::unit_tests::context::generation2d_manifest_for_tests() }

                async fn load_exact<O: WindowConfigOwner>(app: &mut TestApp, window_id: &str, state: O::State) -> Result<(), String> {
                    let envelope_id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
                    let envelope = store::create_config_envelope::<O::State, O::Mutation>(O::SCHEMA, &envelope_id, state, None).await;
                    let files = store::print_document_pack(&envelope).await.map_err(|error| format!("{error:?}"))?;
                    drop(envelope.into_owners());
                    app.load_window_config_pack(WindowConfigPack { window_id: window_id.into(), window_kind_id: O::WINDOW_KIND_ID.into(), files }).await.map_err(|error| format!("{error:?}"))
                }

                async fn dispatch(app: &mut TestApp, view: Option<&ViewModel>, command: Generation2dCommand) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, String> {
                    let meta = ActionMeta { instance_id: 71, view_state: view.cloned(), ..artifact_app_laws::meta("generation2d-window-camera-ownership") };
                    Box::pin(app.dispatch_typed(command, &meta)).await.map_err(|error| format!("{error:?}"))?;
                    Box::pin(artifact_app_laws::settle_registered_typed_operation(app, 71)).await.map_err(|error| format!("{error:?}"))
                }

                async fn main_scene(app: &mut TestApp, view: &ViewModel) -> Result<semio_framework_plugin::NodeGraphScene, String> {
                    let tree = Box::pin(app.render(flow::GENERATION2D_PLAY_BODY_MAIN, None, view)).await.map_err(|error| format!("{error:?}"))?;
                    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                    artifact_app_laws::decode_fixture_scene(&json).map_err(|error| format!("{error}: {json}"))
                }

                async fn canvas_scene(app: &mut TestApp, body: &str, view: &ViewModel) -> Result<semio_framework_plugin::Canvas2dScene, String> {
                    let tree = Box::pin(app.render(body, None, view)).await.map_err(|error| format!("{error:?}"))?;
                    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;
                    artifact_app_laws::decode_fixture_scene(&json).map_err(|error| format!("{error}: {json}"))
                }

                async fn retained_rejection(view: Option<ViewModel>) -> (Box<TestApp>, String) {
                    let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Generation2dPlayApp>>(manifest).await);
                    app.bind_instance_id(71).await;
                    let meta = ActionMeta { instance_id: 71, view_state: view, ..artifact_app_laws::meta("generation2d-window-rejection") };
                    let command = Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 3.0 } });
                    let observed = match Box::pin(app.dispatch_typed(command, &meta)).await {
                        Err(error) => format!("{error:?}"),
                        Ok(_) => match Box::pin(artifact_app_laws::settle_registered_typed_operation(&mut *app, 71)).await {
                            Err(error) => format!("{error:?}"),
                            Ok(receipt) => format!("unexpected receipt: {:?}", receipt.lanes),
                        },
                    };
                    (app, observed)
                }

                let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("neutral runtime fixture");
                let all = ViewModel {
                    window_instances: fixture["windowInstances"].as_array().expect("window instances").iter().map(|row| ViewWindowInstance {
                        id: row["id"].as_str().expect("window id").into(),
                        window_kind_id: row["windowKindId"].as_str().expect("window kind").into(),
                    }).collect(),
                    ..Default::default()
                };
                let main_left = all.for_window_instance("main-left").expect("main-left");
                let main_right = all.for_window_instance("main-right").expect("main-right");
                let edit_left = all.for_window_instance("edit-left").expect("edit-left");
                let edit_right = all.for_window_instance("edit-right").expect("edit-right");
                let generate_left = all.for_window_instance("generate-left").expect("generate-left");
                let generate_right = all.for_window_instance("generate-right").expect("generate-right");
                let foreign = all.for_window_instance("foreign").expect("foreign");

                let main_left_value = semio_framework_os_kernel::Viewport2d { x: 11.0, y: 12.0, zoom: 1.1 };
                let main_right_value = semio_framework_os_kernel::Viewport2d { x: 21.0, y: 22.0, zoom: 1.2 };
                let edit_left_value = semio_framework_os_kernel::Viewport2d { x: 31.0, y: 32.0, zoom: 1.3 };
                let edit_right_value = semio_framework_os_kernel::Viewport2d { x: 41.0, y: 42.0, zoom: 1.4 };
                let generate_left_value = semio_framework_os_kernel::Viewport2d { x: 51.0, y: 52.0, zoom: 1.5 };
                let generate_right_value = semio_framework_os_kernel::Viewport2d { x: 61.0, y: 62.0, zoom: 1.6 };

                let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Generation2dPlayApp>>(manifest).await);
                app.bind_instance_id(71).await;
                let outcome: Result<(), String> = Box::pin(async {
                    load_exact::<edit_preview::config::Generation2dEditPreviewWindowConfigOwner>(&mut app, "edit-left", edit_preview::config::Generation2dEditPreviewWindowConfig { viewport: edit_left_value }).await?;
                    load_exact::<edit_preview::config::Generation2dEditPreviewWindowConfigOwner>(&mut app, "edit-right", edit_preview::config::Generation2dEditPreviewWindowConfig { viewport: edit_right_value }).await?;
                    load_exact::<generate_preview::config::Generation2dGeneratePreviewWindowConfigOwner>(&mut app, "generate-left", generate_preview::config::Generation2dGeneratePreviewWindowConfig { viewport: generate_left_value }).await?;
                    load_exact::<generate_preview::config::Generation2dGeneratePreviewWindowConfigOwner>(&mut app, "generate-right", generate_preview::config::Generation2dGeneratePreviewWindowConfig { viewport: generate_right_value }).await?;
                    eprintln!("[DEBUG] Generation2d exact-window law loaded four preview owners");

                    let preview_receipt = dispatch(&mut app, Some(&generate_left), Generation2dCommand::AddGeneration(add_generation::AddGeneration {})).await?;
                    if preview_receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Transient).count() != 1 { return Err("AddGeneration did not populate the app preview transient".into()); }
                    drop(preview_receipt);
                    let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;

                    for (view, viewport) in [(&main_left, main_left_value), (&main_right, main_right_value)] {
                        let receipt = dispatch(&mut app, Some(view), Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport })).await?;
                        let windows = receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count();
                        let configs = receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Config).count();
                        if windows != 1 || configs != 0 { return Err(format!("NodeGraph retained route published windows={windows} appConfigs={configs}")); }
                    }

                    for view in [&edit_left, &generate_left] {
                        for command in [
                            Generation2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}),
                            Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}),
                            Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
                            Generation2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {}),
                        ] {
                            let receipt = dispatch(&mut app, Some(view), command).await?;
                            if receipt.lanes
                                != [
                                    semio_framework_plugin::app::TypedOperationResultLane::Ui,
                                    semio_framework_plugin::app::TypedOperationResultLane::Terminal,
                                ]
                            {
                                return Err(format!("Canvas no-op published non-lifecycle lanes: {:?}", receipt.lanes));
                            }
                        }
                    }
                    eprintln!("[DEBUG] Generation2d exact-window law settled camera and Canvas command lanes");

                    let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    if document_before.pack != document_after.pack || document_before.spr != document_after.spr { return Err("Generation2d exact-window actions changed document Pack or SPR".into()); }
                    if app_before.pack != app_after.pack || app_before.spr != app_after.spr { return Err("Generation2d exact-window actions changed app-config Pack or SPR".into()); }

                    let left_graph = main_scene(&mut app, &main_left).await?.viewport.ok_or("main-left viewport missing")?;
                    let right_graph = main_scene(&mut app, &main_right).await?.viewport.ok_or("main-right viewport missing")?;
                    eprintln!("[DEBUG] Generation2d exact-window law rendered two main scenes");
                    if left_graph != main_left_value || right_graph != main_right_value { return Err("Generation2d main renderer crossed exact instances".into()); }
                    {
                        let edit_scenes = [canvas_scene(&mut app, edit_preview::GENERATION2D_PLAY_BODY_PREVIEW, &edit_left).await?, canvas_scene(&mut app, edit_preview::GENERATION2D_PLAY_BODY_PREVIEW, &edit_right).await?];
                        eprintln!("[DEBUG] Generation2d exact-window law rendered two edit-preview scenes");
                        if (edit_scenes[0].camera_x, edit_scenes[0].camera_y, edit_scenes[0].zoom) != (edit_left_value.x, edit_left_value.y, edit_left_value.zoom)
                            || (edit_scenes[1].camera_x, edit_scenes[1].camera_y, edit_scenes[1].zoom) != (edit_right_value.x, edit_right_value.y, edit_right_value.zoom) { return Err("Generation2d edit-preview renderer crossed exact instances".into()); }
                    }
                    {
                        let generate_scenes = [canvas_scene(&mut app, generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW, &generate_left).await?, canvas_scene(&mut app, generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW, &generate_right).await?];
                        eprintln!("[DEBUG] Generation2d exact-window law rendered two generate-preview scenes");
                        if (generate_scenes[0].camera_x, generate_scenes[0].camera_y, generate_scenes[0].zoom) != (generate_left_value.x, generate_left_value.y, generate_left_value.zoom)
                            || (generate_scenes[1].camera_x, generate_scenes[1].camera_y, generate_scenes[1].zoom) != (generate_right_value.x, generate_right_value.y, generate_right_value.zoom) { return Err("Generation2d generate-preview renderer crossed exact instances".into()); }
                    }

                    let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if packs.len() != 6 { return Err(format!("expected six exact Generation2d packs, got {}", packs.len())); }
                    let expected: std::collections::BTreeMap<_, _> = packs.iter().map(|pack| ((pack.window_id.clone(), pack.window_kind_id.clone()), (pack.files.pack.clone(), pack.files.spr.clone()))).collect();
                    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Generation2dPlayApp>>(manifest).await);
                    reopened.bind_instance_id(71).await;
                    reopened.load_document_pack(&document_after).await.map_err(|error| format!("{error:?}"))?;
                    reopened.load_config_pack(&app_after).await.map_err(|error| format!("{error:?}"))?;
                    drop(document_before);
                    drop(app_before);
                    drop(document_after);
                    drop(app_after);
                    for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                    let restored = reopened.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    eprintln!("[DEBUG] Generation2d exact-window law reopened six owner packs");
                    let restored: std::collections::BTreeMap<_, _> = restored.into_iter().map(|pack| ((pack.window_id, pack.window_kind_id), (pack.files.pack, pack.files.spr))).collect();
                    if restored != expected { return Err("Generation2d exact window Pack or SPR bytes changed during reopen".into()); }
                    let reopened_left = main_scene(&mut reopened, &main_left).await?.viewport.ok_or("reopened main-left viewport missing")?;
                    let reopened_edit = canvas_scene(&mut reopened, edit_preview::GENERATION2D_PLAY_BODY_PREVIEW, &edit_right).await?;
                    if reopened_left != main_left_value || (reopened_edit.camera_x, reopened_edit.camera_y, reopened_edit.zoom) != (edit_right_value.x, edit_right_value.y, edit_right_value.zoom) { return Err("Generation2d reopened renderer lost exact viewports".into()); }
                    let second_preview = dispatch(&mut reopened, Some(&generate_right), Generation2dCommand::AddGeneration(add_generation::AddGeneration {})).await?;
                    if second_preview.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Transient).count() != 1 { return Err("reopened Generation2d preview transient did not repopulate".into()); }
                    drop(second_preview);
                    let reopened_generate = canvas_scene(&mut reopened, generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW, &generate_right).await?;
                    eprintln!("[DEBUG] Generation2d exact-window law rendered reopened scenes");
                    if (reopened_generate.camera_x, reopened_generate.camera_y, reopened_generate.zoom) != (generate_right_value.x, generate_right_value.y, generate_right_value.zoom) { return Err("Generation2d reopened generate-preview lost exact viewport".into()); }
                    drop(reopened_generate);
                    drop(restored);
                    drop(expected);
                    eprintln!("[DEBUG] Generation2d exact-window law closing reopened app");
                    artifact_app_laws::close_registered_fixture_app(&mut *reopened);
                    eprintln!("[DEBUG] Generation2d exact-window law closed reopened app");
                    let reopened_terminal_empty = reopened.close_terminal_is_empty();
                    drop(reopened);
                    if !reopened_terminal_empty { return Err("reopened Generation2d app was not terminal-empty after close".into()); }

                    let exact_rejections = [
                        (flow::config::addressed(&ViewModel { window_id: None, window_instances: all.window_instances.clone(), ..Default::default() }, Default::default()), "generation2d-main-window-required"),
                        (flow::config::addressed(&ViewModel { window_id: Some("missing".into()), window_instances: all.window_instances.clone(), ..Default::default() }, Default::default()), "generation2d-main-window-stale"),
                        (flow::config::addressed(&foreign, Default::default()), "generation2d-main-window-kind-required"),
                    ];
                    for (observed, expected) in exact_rejections {
                        if observed.is_ok() || !format!("{observed:?}").contains(expected) { return Err(format!("exact address did not reject with {expected}: {observed:?}")); }
                    }
                    let mut rejection_faults = Vec::with_capacity(6);
                    for (view, case, public_fault) in [
                        (None, "generation2d-main-window-view-required", "registered fixture typed operation fault: retained command reducer rejected operation"),
                        (Some(ViewModel { window_id: Some("missing".into()), window_instances: all.window_instances.clone(), ..Default::default() }), "generation2d-main-window-stale", "window-config.window-context"),
                        (Some(foreign.clone()), "generation2d-main-window-kind-required", "registered fixture typed operation fault: retained command reducer rejected operation"),
                    ] {
                        let (mut rejected, observed) = Box::pin(retained_rejection(view)).await;
                        let rejected_as_expected = !observed.starts_with("unexpected receipt:") && observed.contains(public_fault);
                        eprintln!("[DEBUG] Generation2d exact-window law observed and is closing rejection app for {case}");
                        artifact_app_laws::close_registered_fixture_app(&mut *rejected);
                        eprintln!("[DEBUG] Generation2d exact-window law closed rejection app for {case}");
                        let terminal_empty = rejected.close_terminal_is_empty();
                        drop(rejected);
                        if !terminal_empty { rejection_faults.push(format!("rejection app for {case} was not terminal-empty after close")); }
                        if !rejected_as_expected { rejection_faults.push(format!("expected retained rejection for {case}, observed {observed}")); }
                    }
                    if !rejection_faults.is_empty() { return Err(rejection_faults.join("; ")); }
                    Ok(())
                }).await;
                if let Err(error) = &outcome { eprintln!("[DEBUG] Generation2d exact-window runtime failure before primary close: {error}"); }
                eprintln!("[DEBUG] Generation2d exact-window law closing primary app");
                artifact_app_laws::close_registered_fixture_app(&mut *app);
                eprintln!("[DEBUG] Generation2d exact-window law closed primary app");
                let primary_terminal_empty = app.close_terminal_is_empty();
                drop(app);
                assert!(primary_terminal_empty, "Generation2d primary app was not terminal-empty after close");
                outcome.expect("Generation2d exact-window ownership runtime law");
                eprintln!("[DEBUG] Generation2d isolated six exact camera owners, verified retained/raw route separation, rendered/reopened every kind, preserved document/app Pack+SPR, rejected invalid contexts, and closed on a 2 MiB stack");
            }))
        })
        .expect("spawn Generation2d window ownership law")
        .join()
        .expect("Generation2d window ownership law thread");
}
