/// 📜️ Manifest and cross-cutting laws of the generation2d editor routes: the manifest stitch, the declared action bridge, the canvas wire defaults and the widget undo and convergence laws.

//#region 🔖️ManifestSanity
#[cfg(test)] use semio_s_artifact_procedural_2d_mode_edit::windows::flow as flow_window;
#[cfg(test)] use semio_s_artifact_procedural_2d_mode_edit::windows::preview as edit_preview;
#[cfg(test)]#[cfg(test)] use semio_s_artifact_procedural_2d_editor_panels::document as document_panel;
#[cfg(test)] use semio_s_artifact_procedural_2d_editor_panels::catalogue as catalogue_panel;
#[cfg(test)] use semio_s_artifact_procedural_2d_editor_panels::inspection as inspection_panel;
#[cfg(test)]#[test]
fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_generation2d_app()).expect("app definition json");
    for id in [
        flow_window::GENERATION2D_PLAY_WINDOW_MAIN,
        edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW,
        generations::GENERATION2D_PLAY_WINDOW_GENERATIONS,
        form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM,
        generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW,
    ] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::GENERATION2D_PLAY_MODE_EDIT, generate::GENERATION2D_PLAY_MODE_GENERATE] {
        assert!(json.contains(id), "mode {id} missing from the manifest");
    }
    for body in [document_panel::GENERATION2D_PLAY_BODY_ARTIFACT, catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE, inspection_panel::GENERATION2D_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("2d.generation"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn declared_actions_bridge_to_commands() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Generation2dPlayApp>>(context::generation2d_manifest_for_tests).await;
}

/// 🧵️ LAW (design L4 / §2 D): a legacy `{x, y}` `canvasPointerMove` wire folds into one sample, a
/// batched wire keeps every `[x, y]` pair in order, and `canvasPointerUp` without `cancelled` is a
/// real release. Generation2d keeps no canvas drag gesture, so both handlers stay no-ops either way.
#[test]
fn canvas_pointer_wire_defaults_samples_and_cancelled() {
    let bridge = |action: &str, args: semio_framework_value::DslValue| <Generation2dPlayApp as ArtifactEditor>::command_from_action(action, Some((args).clone()), None).expect("declared pointer action bridges");
    let f = semio_framework_value::DslValue::float;
    let legacy = semio_framework_value::DslValue::object([("x".into(), f(5.0)), ("y".into(), f(6.0))]);
    assert_eq!(bridge("canvasPointerMove", legacy.clone()), Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: vec![[5.0, 6.0]] }), "an absent `samples` is the single (x, y)");
    assert_eq!(bridge("canvasPointerMove", semio_framework_value::DslValue::Object(Vec::new())), Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: Vec::new() }), "no coordinates at all is an empty batch");
    let pair = |x: f64, y: f64| semio_framework_value::DslValue::Array(vec![f(x), f(y)]);
    let batched = semio_framework_value::DslValue::object([("x".into(), f(3.0)), ("y".into(), f(4.0)), ("samples".into(), semio_framework_value::DslValue::Array(vec![pair(1.0, 1.5), pair(3.0, 4.0)]))]);
    let Generation2dCommand::CanvasPointerMove(moved) = bridge("canvasPointerMove", batched) else { panic!("move") };
    assert_eq!(moved.samples, vec![[1.0, 1.5], [3.0, 4.0]]);
    assert_eq!(moved.last_sample(), Some([3.0, 4.0]));
    assert_eq!(bridge("canvasPointerUp", legacy), Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: false }), "an absent `cancelled` is a real release");
    assert_eq!(bridge("canvasPointerUp", semio_framework_value::DslValue::object([("cancelled".into(), semio_framework_value::DslValue::Bool(true))])), Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: true }));
    let document = Generation2dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&document, &history);
    let config = Generation2dConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::default();
    let emit = canvas_pointer_up::handle(&canvas_pointer_up::CanvasPointerUp { cancelled: true }, &view, &cfg, &mut session).expect("cancel");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty() && emit.config_mutations.is_empty(), "a cancel never selects or commits");
    document.retire_cold();
    session.retire_cold();
}

/// 🎥️ LAW: `nodeGraphViewport` declares no args, so the shell stages nothing for it. An absent `viewport`
/// decodes to the identity camera instead of refusing the bridge, while a present but malformed one still
/// faults. The generic bridge law only exercises the absent half.
#[test]
fn node_graph_viewport_decodes_an_absent_viewport_as_identity_and_refuses_a_malformed_one() {
    let decode = |args: semio_framework_value::DslValue| <Generation2dPlayApp as ArtifactEditor>::command_from_action("nodeGraphViewport", Some((args).clone()), None);
    let identity = semio_framework_os_kernel::Viewport2d::default();
    assert_eq!(decode(semio_framework_value::DslValue::Object(Vec::new())).expect("an argless invocation bridges"), Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: identity }));
    let authored = semio_framework_os_kernel::Viewport2d { x: 12.0, y: -8.0, zoom: 2.0 };
    assert_eq!(
        decode(semio_framework_value::DslValue::object([("viewport".into(), semio_framework_value::ToValue::to_value(&authored))])).expect("an authored viewport bridges"),
        Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: authored })
    );
    let refused = decode(semio_framework_value::DslValue::object([("viewport".into(), semio_framework_value::DslValue::object([("x".into(), semio_framework_value::DslValue::float(1.0)), ("y".into(), semio_framework_value::DslValue::float(2.0)), ("zoom".into(), semio_framework_value::DslValue::float(0.0))]))]));
    assert!(refused.is_err(), "a zoom the graph cannot express must fault instead of being replaced by the identity camera");
}

/// 🧩️ `addWidget` is a MIGRATED interactive job now, so `dispatch_typed` only ENQUEUES it — the
/// document grows once the retained ladder is driven to its publication, exactly as the runtime
/// drives it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn add_widget_materializes_declared_kind_default_into_an_operation() {
    let mut app = app_with_registry().await;
    let result: Result<(usize, usize), String> = async {
        let before = snapshot_read(&app).host_snapshot.widgets.len();
        app.0.dispatch_typed(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, format: None, action: None, x: None, y: None }), &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())
            .await
            .map_err(|error| format!("{error:?}"))?;
        let (artifact, _, _) = drive_preview_operation(&mut app).await?;
        if artifact != 1 {
            return Err(format!("addWidget must publish its artifact lane exactly once, got {artifact}"));
        }
        Ok((before, snapshot_read(&app).host_snapshot.widgets.len()))
    }
    .await;
    close(app);
    let (before, after) = result.expect("Generation2d addWidget retained runtime");
    assert_eq!(after, before + 1);
}

#[semio_framework_async_macros::async_test]
async fn add_widget_undo_redo_round_trip() {
    let mut app = app().await;
    let before = snapshot_read(&app).host_snapshot.widgets.len();
    assert_undo_redo_round_trip(&mut app.0, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, format: None, action: None, x: None, y: None }), |app| snapshot_read(app).host_snapshot.widgets.len(), before, before + 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    close(app);
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_widget_moves() {
    let fixture = app().await;
    let widgets: Vec<String> = snapshot_read(&fixture).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget).to_string()).collect();
    close(fixture);
    assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
    let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
    semio_framework_plugin::artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<Generation2dPlayApp>, (Option<f64>, Option<f64>), _, _>(
        "mem://generation2d-convergence",
        || async { crate::app_fixture::generation2d_manifest_for_tests() },
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
        move |app| {
            let projection = snapshot_read(app);
            (projection.host_snapshot.layout.get(&w0).map(|entry| entry.x), projection.host_snapshot.layout.get(&w1).map(|entry| entry.x))
        },semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::app_fixture::render;
    let mut app = app().await;
    let rendered = render(&mut app, "generation2d.play.nope").await;
    close(app);
    assert!(rendered.contains("Unknown body"), "{rendered}");
}
//#endregion 🔖️CrossCutting
