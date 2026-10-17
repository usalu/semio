//! 🎮️ Engagement and import tests of the CAD editor unit tests: the interaction sessions, window transient state and the spatial and OBJ imports. See [`super`].

use super::*;

//#region 🔖️Engagement
/// 🫧️ A window whose action line reads `line`.
fn typed(line: &str) -> CadWorldWindowTransient {
    CadWorldWindowTransient { engagement_input: line.into(), ..CadWorldWindowTransient::default() }
}

fn spatial_scene_import_args() -> Value {
    let file_text = json!({
        "schema": "spatial.model",
        "revision": 1,
        "modelDefinitionId": "spatial.shape",
        "objects": [{
            "id": "object-engagement-reset",
            "label": "Engagement reset",
            "typology": "spatial.shape.primitive.box",
            "visible": true,
            "locked": false,
            "origin": [0.0, 0.0, 0.0],
            "primitives": []
        }]
    })
    .to_string();
    json!({ "payload": file_text, "name": "engagement-reset.spatial.json" })
}

#[semio_framework_async_macros::async_test]
async fn engagement_starts_box_interaction_session() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let (emit, transient) = drive_engagement(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &CadConfig::default(), &typed("b")).expect("submit");
    assert!(emit.config_mutations.is_empty(), "starting an interaction is no config edit");
    assert!(cad_runtime_from(&CadConfig::default(), &transient).engagement_session.is_some(), "the window's transient holds the live session");
}

#[semio_framework_async_macros::async_test]
async fn world_pointer_move_updates_live_preview_without_committing_or_emitting_mutations() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let (_, started) = drive_engagement(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config, &typed("b")).expect("submit");
    let (emit, moved) = drive_engagement(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config, &started).expect("move");
    assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.transaction.is_none(), "a pointer move is window state only");
    let runtime = cad_runtime_from(&config, &moved);
    let session = runtime.engagement_session.as_ref().expect("session still active");
    assert_eq!(session.state, "first_corner", "pointer.move must not change state");
    assert_eq!(session.context.0.get("cursor"), Some(&semio_framework_pack_json::to_dsl_value(&json!([3.0, 4.0, 0.0]))));
}

/// 🫧️ LAW (design §17.4): every engagement step that commits nothing — a keystroke, starting the interaction, a pointer
/// move, an abort — is the addressed window's transient alone: no config edit, no coalesce key, no document edit; a scene
/// import and an example switch reset the window's engagement.
#[semio_framework_async_macros::async_test]
async fn every_engagement_step_is_window_transient_state_never_config() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let config = CadConfig::default();
    let window_only = |emit: &Emit<CadMutation, CadConfigMutation>, what: &str| {
        assert!(emit.config_mutations.is_empty(), "{what} is no config edit");
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "{what} touches no document");
    };
    let (emit, typed_line) = drive_engagement(&app, &scene, "engagementInput", Some(json!({ "pane": "shape", "value": "b" })), &config, &CadWorldWindowTransient::default()).expect("keystroke");
    window_only(&emit, "a keystroke");
    assert_eq!(typed_line.engagement_input, "b");
    let (emit, started) = drive_engagement(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config, &typed_line).expect("submit");
    window_only(&emit, "starting the interaction");
    assert!(started.engagement_session_json.is_some());
    let (emit, moved) = drive_engagement(&app, &scene, "worldPointerMove", Some(json!({ "position": [1.0, 2.0, 0.0] })), &config, &started).expect("move");
    window_only(&emit, "a pointer move");
    assert_ne!(moved.engagement_session_json, started.engagement_session_json, "the move updates the live session");
    let (emit, aborted) = drive_engagement(&app, &scene, "engagementAbort", None, &config, &moved).expect("abort");
    window_only(&emit, "an abort");
    assert!(aborted.engagement_session_json.is_none() && aborted.engagement_step == "Idle");
    let (_, imported) = drive_engagement(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &config, &started).expect("import");
    assert!(imported.engagement_session_json.is_none(), "a scene import resets the window's engagement");
    let (emit, switched) = drive_engagement(&app, &scene, "setActiveExample", Some(json!({ "exampleId": "" })), &config, &started).expect("example switch");
    assert!(switched.engagement_session_json.is_none(), "an example switch resets the window's engagement");
    assert_eq!(emit.config_mutations.len(), 1, "the example switch records the active example in config");
}

/// 🔁️ LAW: repeat-last restarts the interaction the window last finalized. `📦️box.json` defaults to `point` mode, so the
/// line `d` selects the two-corner diagonal flow; `set.height` only records the height, and `Confirm` reaches `ready`, the
/// box's commit state.
#[semio_framework_async_macros::async_test]
async fn engagement_repeat_last_restarts_the_last_finalized_interaction() {
    let app = CadPlayApp::default();
    let mut scene = default_document();
    let config = CadConfig::default();
    let step = |scene: &CadSnapshot, action: &str, args: Option<Value>, transient: &CadWorldWindowTransient| drive_engagement(&app, scene, action, args, &config, transient).unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    let (_, mut transient) = step(&scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &typed("b"));
    assert!(cad_runtime_from(&config, &transient).engagement_session.is_some());
    transient.engagement_input = "d".into();
    (_, transient) = step(&scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &transient);
    for position in [json!([0.0, 0.0, 0.0]), json!([2.0, 3.0, 0.0])] {
        let (emit, next) = step(&scene, "worldPointerDown", Some(json!({ "pane": "shape", "position": position })), &transient);
        scene = apply_mutations(&scene, &emit.artifact_mutations);
        transient = next;
    }
    transient.engagement_input = "SetHeight2.5".into();
    (_, transient) = step(&scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &transient);
    transient.engagement_input = "Confirm".into();
    let (emit, committed) = step(&scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &transient);
    scene = apply_mutations(&scene, &emit.artifact_mutations);
    let runtime = cad_runtime_from(&config, &committed);
    assert!(runtime.engagement_session.is_none(), "box should have committed");
    assert_eq!(runtime.last_finalized_interaction_id.as_deref(), Some("primitive.box"));
    let (_, repeated) = step(&scene, "engagementRepeatLast", Some(json!({ "pane": "shape" })), &committed);
    let runtime = cad_runtime_from(&config, &repeated);
    let session = runtime.engagement_session.as_ref().expect("repeat-last should start a session");
    assert_eq!(session.interaction_id, "primitive.box");
}
//#endregion 🔖️Engagement

//#region 🔖️Import
#[semio_framework_async_macros::async_test]
async fn import_spatial_modelspace_round_trips() {
    let payload = json!({
        "schema": "spatial.modelspace",
        "revision": 1,
        "activeModelDefinitionId": "spatial.shape",
        "models": [{
            "id": "spatial.shape",
            "model": {
                "schema": "spatial.model",
                "revision": 1,
                "objects": [{
                    "id": "object-imported",
                    "label": "Imported",
                    "typology": "spatial.shape.primitive.box",
                    "visible": true,
                    "locked": false,
                    "origin": [1.0, 2.0, 3.0],
                    "primitives": []
                }]
            }
        }]
    });
    let scene = scene_from_spatial_payload(&semio_framework_pack_json::to_dsl_value(&payload)).expect("scene");
    assert!(scene.shape_model.is_some(), "a real imported object must mint a shape-model child");
}

#[semio_framework_async_macros::async_test]
async fn import_cad_file_action_accepts_spatial_json_text_string_payload() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let file_text = json!({
        "schema": "spatial.model",
        "revision": 1,
        "modelDefinitionId": "spatial.shape",
        "objects": [{
            "id": "object-loaded",
            "label": "Loaded",
            "typology": "spatial.shape.primitive.box",
            "visible": true,
            "locked": false,
            "origin": [1.0, 2.0, 3.0],
            "primitives": []
        }]
    })
    .to_string();
    let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": file_text, "name": "cad.spatial.json" })));
    // 🌱️ Whole-document replace is not an in-history mutation (SEMANTIC-MUTATIONS-OVERHAUL
    // retired `SetSnapshot`) — a spatial JSON string payload now surfaces as a
    // `Effect::LoadDocument` carrying the replacement document's pack bytes.
    let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("importCadFile must emit a LoadDocument effect for a spatial JSON string payload") else {
        panic!("expected a LoadDocument effect");
    };
    let next = <CadSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(next.shape_model.is_some(), "a real imported object must mint a shape-model child");
}

/// 📥️ A single object file lands as ONE `insert-element` on the shape pane's model child (design §20.15); where no model
/// child is composed the import is refused by name, never dropped. Selecting the imported object stays the host's
/// follow-up `interactionSelect` (selection is framework-owned).
#[semio_framework_async_macros::async_test]
async fn import_cad_file_action_imports_obj_by_extension() {
    let obj_text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
    let obj_data_url = format!("data:model/obj;base64,{}", base64_codec::base64_standard_encode(obj_text));
    let fault = drive_result(&CadPlayApp::default(), &empty_cad_snapshot(), "importCadFile", Some(json!({ "payload": obj_data_url.clone(), "name": "triangle.obj" })), &CadConfig::default()).err().expect("an object import into a pane without a model child is refused");
    assert_eq!(fault.code.0, "cad.import-object-refused");
    let mut app = new_app().await;
    let parent = app.snapshot().expect("snapshot");
    let before = pane_objects(&app, CadPaneId::Shape).await.len();
    app.dispatch_typed(CadCommand::ImportCadFile(import_cad_file::ImportCadFile { name: "triangle.obj".into(), payload: obj_data_url }), &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("import dispatch");
    settle(&mut app).await;
    assert_eq!(pane_objects(&app, CadPaneId::Shape).await.len(), before + 1, "the imported object is one more element of the shape pane's model child");
    assert_eq!(app.snapshot().expect("snapshot"), parent, "an object import never moves the parent document");
    close(&mut app);
}
//#endregion 🔖️Import
