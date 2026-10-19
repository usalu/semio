//! 🎚️ Engagement submit, commit and probe tests of the CAD editor unit tests: the state confirm, the committing event, the interaction scope, the render cost probe and the solid export. See [`super`].

use super::*;
#[cfg(test)] use semio_s_artifact_cad_cad_modes as edit;

//#region 🔖️EngagementSubmit
/// ⏎️ The shell's Enter/Space on an empty action line during a session is the state's `confirm`:
/// box's `first_corner_height` accepts the typed height, which reaches the `ready` commit state (the spec's
/// `commit.fromStates` entry, so it commits at once). Without a session the empty line stays the idle no-op.
#[semio_framework_async_macros::async_test]
async fn empty_submit_during_a_session_fires_the_state_confirm_and_commits() {
    let models = CadPaneModels(vec![(CadPaneId::Shape, "shape-model-test".into(), semio_s_artifact_cad_cad_model::geometry::semio_model_snapshot_from_objects(&[]))]);
    let mut runtime = CadPlayRuntime::default();
    assert!(start_interaction_session(&mut runtime, CadPaneId::Shape, "primitive.box"));
    {
        let session = runtime.engagement_session.as_mut().expect("session");
        assert!(apply_event(session, "pointer.down", Some(&semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(0.0), semio_framework_value::DslValue::float(0.0), semio_framework_value::DslValue::float(0.0)]))));
        assert!(apply_event(session, "pointer.down", Some(&semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(2.0), semio_framework_value::DslValue::float(3.0), semio_framework_value::DslValue::float(0.0)]))));
        assert_eq!(session.state, "first_corner_height");
    }
    runtime.engagement_input = "2".into();
    assert!(engagement_submit_entries(&models, &mut runtime, CadPaneId::Shape).is_empty(), "the typed height is a scalar entry, not a commit");
    assert_eq!(runtime.engagement_session.as_ref().map(|session| session.state.as_str()), Some("first_corner_height"));
    assert!(runtime.engagement_input.is_empty(), "a consumed scalar entry clears the published line");
    let entries = engagement_submit_entries(&models, &mut runtime, CadPaneId::Shape);
    assert_eq!(entries.len(), 1, "accepting the height reaches `ready` and commits exactly one box: {entries:?}");
    assert!(matches!(entries[0], CadToolEntry::Create { pane: CadPaneId::Shape, .. }));
    assert!(runtime.engagement_session.is_none(), "the committed session is closed");
    assert_eq!(runtime.engagement_step, "Committed 1 object(s)");
    assert!(engagement_submit_entries(&models, &mut runtime, CadPaneId::Shape).is_empty());
    assert_eq!(runtime.engagement_step, "Idle");
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️EngagementCommit
/// 🧵️ One engagement is at most one history item (design §17.4): every non-committing step (keystroke, start, pointer move,
/// pick) is window-transient state only, and the committing step — the empty line, which is the state's `confirm` into the
/// box's `ready` commit state — is the one stamped edit: an `insert-element` on the shape pane's model child, with no
/// parent mutation and no config edit (design §20.15).
#[semio_framework_async_macros::async_test]
async fn engagement_steps_are_window_state_until_the_one_committing_edit() {
    use semio_framework_pack_json::json;
    let mut mounted = new_app().await;
    let scene = mounted.snapshot().expect("snapshot");
    let children = mounted.test_child_content_view();
    let shape_child = semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&scene, CadPaneId::Shape).expect("the demo document composes a shape child").child_id.clone();
    let config = CadConfig::default();
    let history = empty_history();
    let operation = semio_framework_plugin::AppOperationContext { app_instance_id: TEST_INSTANCE, parent_document_id: "cad-test-document".into(), operation_id: 1, generation: 1, canonical_base_revision: [0; 32], authoring_seed: "seed-engagement".into() };
    let mut transient = CadWorldWindowTransient::default();
    let mut step = |action: &str, args: Value, what: &str| {
        let doc = ArtifactView::with_children(&scene, &history, children.clone()).bound_to_operation(operation.clone());
        let cfg = ConfigView { snapshot: &config, window: None };
        let mut ctx = CadDispatchCtx::new(CadInteractionSnapshot::default(), None, transient.clone());
        let emit = command_from_action(action, Some(&args)).dispatch(&doc, &cfg, &mut ctx).unwrap_or_else(|fault| panic!("{what}: {fault:?}"));
        if let Some(next) = ctx.next_window_transient {
            transient = next;
        }
        emit
    };
    let window_only = |emit: &Emit<CadMutation, CadConfigMutation>, what: &str| {
        assert!(emit.config_mutations.is_empty() && emit.transaction.is_none(), "{what} is no config edit and opens no transaction");
        assert!(emit.artifact_mutations.is_empty() && emit.child_preparations.is_empty(), "{what} touches no document");
    };
    for (index, character) in ["B", "Bo", "Box"].iter().enumerate() {
        window_only(&step("engagementInput", json!({ "pane": "shape", "value": character }), "keystroke"), &format!("keystroke {index}"));
    }
    window_only(&step("engagementPossibleSelect", json!({ "pane": "shape", "possibleId": "primitive.box" }), "start"), "starting the interaction");
    window_only(&step("worldPointerMove", json!({ "position": [0.5, 0.5, 0.0] }), "move"), "a pointer move");
    for point in [[0.0, 0.0, 0.0], [2.0, 3.0, 0.0]] {
        window_only(&step("worldPointerDown", json!({ "pane": "shape", "position": point }), "pick"), "a non-committing pick");
    }
    window_only(&step("engagementInput", json!({ "pane": "shape", "value": "2" }), "height"), "the height keystroke");
    window_only(&step("engagementSubmit", json!({ "pane": "shape" }), "apply"), "applying the typed height");
    let mut emit = step("engagementSubmit", json!({ "pane": "shape" }), "commit");
    drop(step);
    assert!(emit.transaction.as_ref().is_some_and(|transaction| transaction.tool == "s.cad.cad@1/*#editor#primitive.box"), "the commit is its own stamped edit: {:?}", emit.transaction);
    assert!(emit.config_mutations.is_empty() && emit.artifact_mutations.is_empty(), "the commit edits neither the config nor the parent document");
    let mut ready = false;
    for _ in 0..4096 {
        match emit.prepare_child_one(1, 65_536).expect("bounded child preparation") {
            semio_framework_plugin::app::ChildEmitPreparationStep::Ready => {
                ready = true;
                break;
            }
            semio_framework_plugin::app::ChildEmitPreparationStep::Pending => {}
            semio_framework_plugin::app::ChildEmitPreparationStep::Refused(fault) => panic!("child preparation refused: {}", fault.message),
        }
    }
    assert!(ready, "the commit's child edit prepares within its bound");
    assert_eq!(emit.child_emits.len(), 1, "one pane child receives the box");
    assert_eq!((emit.child_emits[0].slot.as_str(), emit.child_emits[0].child_id.as_str(), emit.child_emits[0].ops.len()), ("shapeModel", shape_child.as_str(), 1), "one `insert-element` on the shape pane's child");
    let runtime = cad_runtime_from(&config, &transient);
    assert!(runtime.engagement_session.is_none());
    assert_eq!(runtime.engagement_step, "Committed 1 object(s)");
    drop(emit);
    drop(children);
    close(&mut mounted);
}
//#endregion 🔖️EngagementCommit

//#region 🔖️InteractionScope
/// 🎯️ A hover on the `cad` domain repaints the four world bodies and nothing else — never the
/// framework's `Full` fallback (every window body, every panel, the rails) on pointer motion; a
/// selection adds the Inspection and Artifact panels plus the HUD count; a verb on a foreign domain
/// is handed back to the framework.
#[semio_framework_async_macros::async_test]
async fn cad_interaction_scope_keeps_hover_to_the_world_bodies() {
    use semio_framework::kernel::UiDirtyScope;
    use semio_framework_plugin::InteractionVerb;
    let Some(UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels }) = cad_interaction_scope(InteractionVerb::Hover, &[CAD_INTERACTION_DOMAIN]) else {
        panic!("hover answers a partial scope");
    };
    assert_eq!(window_bodies, CAD_WORLD_BODY_KEYS.map(str::to_string).to_vec());
    assert!(panel_bodies.is_empty() && !utilities && !tools && !engagements && !measures && !labels);
    let Some(UiDirtyScope::Partial { panel_bodies, engagements, .. }) = cad_interaction_scope(InteractionVerb::Select, &[CAD_INTERACTION_DOMAIN]) else {
        panic!("select answers a partial scope");
    };
    assert_eq!(panel_bodies, vec![inspection::CAD_PLAY_BODY_PROPERTIES.to_string(), document::CAD_PLAY_BODY_ARTIFACT.to_string()]);
    assert!(engagements);
    assert_eq!(cad_interaction_scope(InteractionVerb::SetGranularity, &[CAD_INTERACTION_DOMAIN]), Some(UiDirtyScope::None));
    assert_eq!(cad_interaction_scope(InteractionVerb::Hover, &["vortex"]), None);
    assert_eq!(cad_interaction_scope(InteractionVerb::Hover, &[]), None);
}
//#endregion 🔖️InteractionScope

//#region 🔖️RenderCostProbe
/// 🩺️ Where a warm window render spends its time (a probe, printed with `--nocapture`; the only
/// assertion is that a warm render is cheaper than a cold one).
#[semio_framework_async_macros::async_test]
async fn world_scene_render_cost_probe() {
    let view = forest_view();
    let options = CadDislocateOptions::default();
    let cold = std::time::Instant::now();
    let _ = edit::build_world_scene_for_pane(&view, CadPaneId::Building, "cad.play.scene3d/building", None, options).expect("scene");
    let cold = cold.elapsed();
    let warm = std::time::Instant::now();
    for _ in 0..10 {
        let _ = edit::build_world_scene_for_pane(&view, CadPaneId::Building, "cad.play.scene3d/building", None, options).expect("scene");
    }
    let warm = warm.elapsed() / 10;
    let composed = view.panes.pane(CadPaneId::Building);
    let lane = std::time::Instant::now();
    let meshes = edit::world_meshes_json_cached(CadPaneId::Building, composed.genesis.as_ref(), &composed.objects, composed.geometry(CadPaneId::Building));
    let lane = lane.elapsed();
    let carrier = std::time::Instant::now();
    let _ = semio_framework_plugin::paged_text_carrier("meshes", &meshes);
    let carrier = carrier.elapsed();
    assert!(warm < cold);
}
//#endregion 🔖️RenderCostProbe

//#region 🔖️PaneSolidExport
/// ⚖️ A pane's current export writes the pane's real geometry: the forest's Building pane has world-space solids and
/// exports as STEP, OBJ and STL files — never the empty spatial fallback (io-matrix 27 19:5x measured
/// `cad.current.spatial.dsl` with `objects: []` for all three). The files' parsers (ISO 10303-21 structure, three.js
/// OBJ/STL loaders) judge them live in `verify io`.
#[semio_framework_async_macros::async_test]
async fn current_pane_exports_its_real_solids() {
    use semio_s_artifact_cad_cad_io::{CAD_SOLID_EXPORT_DIALECT_OBJ, CAD_SOLID_EXPORT_DIALECT_STL};
    let view = forest_view();
    let mut kernel = cad_brep_kernel();
    assert!(!collect_pane_solids(&mut kernel, &view, CadPaneId::Building).is_empty(), "the Building pane has solids");
    for (format, extension) in [(CAD_SOLID_EXPORT_DIALECT_STEP, ".step"), (CAD_SOLID_EXPORT_DIALECT_OBJ, ".obj"), (CAD_SOLID_EXPORT_DIALECT_STL, ".stl")] {
        let export = export_solid_for_pane(&view, CadPaneId::Building, format).expect("a solid export");
        assert!(export.filename.ends_with(extension), "{} names its format", export.filename);
    }
}
//#endregion 🔖️PaneSolidExport
