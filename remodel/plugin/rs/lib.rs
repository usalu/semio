//! 🏺 Remodel plugin — photogrammetry play app (video → watertight mesh) bundled as a hot-swappable WASM component.

use remodel_document::{
    default_remodel_scene, CameraState, DenseResolution, ReconstructionParams, RemodelMesh, RemodelOp, RemodelScene,
    SelectionState, SourceVideo, MeshSource, REMODEL_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, mesh_from_kind, ui_stack_vertical, ui_text, world3d_camera_json,
    world3d_scene, world3d_selection_json, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App,
    DocumentApp, DocumentView, MeshData, PanelGroup, SurfaceKind, UtilityCategory, UtilityDefinition, UiNode, ViewState,
    WorldSunConfig, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde_json::{json, Value};

//#region 🔖Constants
const REMODEL_PLAY_APP_ID: &str = "remodel-play";
const REMODEL_PLAY_SURFACE_MAIN: &str = "remodel.play";
const REMODEL_PLAY_BODY_MAIN: &str = "remodel.play.main";
const REMODEL_PLAY_BODY_DOCUMENT: &str = "remodel.play.document";
const REMODEL_PLAY_WINDOW_MAIN: &str = "remodel-main";
const REMODEL_MESH_ID: &str = "remodel-result";
/// 🧰 The utility active when the host has not yet set `view_state.active_utility_id` (first UtilityRef default).
const REMODEL_DEFAULT_UTILITY: &str = "select";
//#endregion 🔖Constants

//#region 🔖Runtime
/// 🎛️ Ephemeral viewport state (orbit camera, face/vertex selection) — lives in the app struct, never
/// in the document, so panning the camera or picking a face never lands in undo history nor syncs to
/// peers. The active utility is host-owned session state (`view_state.active_utility_id`), not stored here.
#[derive(Clone, Debug, Default, PartialEq)]
struct RemodelPlayRuntime {
    camera: CameraState,
    selection: SelectionState,
}
//#endregion 🔖Runtime

//#region 🔖Document
fn world_meshes_json(scene: &RemodelScene) -> String {
    let meshes: Vec<Value> = scene
        .result
        .as_ref()
        .map(|result| vec![json!({ "id": REMODEL_MESH_ID, "data": result.mesh })])
        .unwrap_or_default();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn world_instances_json(scene: &RemodelScene) -> String {
    let instances: Vec<Value> = if scene.result.is_some() {
        vec![json!({
            "id": REMODEL_MESH_ID,
            "meshId": REMODEL_MESH_ID,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "selected": false,
            "hovered": false,
        })]
    } else {
        Vec::new()
    };
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn build_document_panel(scene: &RemodelScene, runtime: &RemodelPlayRuntime, active_utility: &str, labels: &RemodelLabels) -> UiNode {
    let video_label = scene
        .source_video
        .as_ref()
        .map(|video| format!("{}: {} ({} {}, {:.1} fps)", labels.source_video, video.filename, video.frame_count, labels.frames, video.fps))
        .unwrap_or_else(|| labels.no_source_video.into());
    let job_label = format!(
        "{}: {:?} ({:.0}%){}",
        labels.reconstruction,
        scene.job.stage,
        scene.job.progress_0_1 * 100.0,
        scene
            .job
            .error
            .as_ref()
            .map(|error| format!(" — {}: {error}", labels.error))
            .unwrap_or_default()
    );
    let mesh_label = scene
        .result
        .as_ref()
        .map(|result| {
            format!(
                "{}: {:?}, {} {}, {} {}",
                labels.mesh,
                result.source,
                result.mesh.vertex_count(),
                labels.vertices,
                result.mesh.triangle_count(),
                labels.triangles
            )
        })
        .unwrap_or_else(|| format!("{}: {}", labels.mesh, labels.mesh_none));
    let utility_label = format!("{}: {} · {}: {} ({})", labels.utility, active_utility, labels.selection, runtime.selection.mode, runtime.selection.ids.len());
    ui_stack_vertical(vec![ui_text(video_label), ui_text(job_label), ui_text(mesh_label), ui_text(utility_label)])
}

fn placeholder_result() -> RemodelMesh {
    RemodelMesh {
        mesh: mesh_from_kind("box"),
        source: MeshSource::Placeholder,
    }
}
//#endregion 🔖Document

//#region 🔖RemodelPlayApp
#[derive(Default)]
struct RemodelPlayApp {
    runtime: RemodelPlayRuntime,
}

impl DocumentApp for RemodelPlayApp {
    type Projection = RemodelScene;
    type Op = RemodelOp;

    fn app_id(&self) -> &str {
        REMODEL_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        REMODEL_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> RemodelScene {
        default_remodel_scene()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        _doc: &DocumentView<'_, RemodelScene>,
        _view_state: &ViewState,
    ) -> ActionEmit<RemodelOp> {
        match action {
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Host-owned utility switch: remodel keeps no in-progress gesture scratch, so emit nothing.
                ActionEmit::default()
            }
            "setSelection" => {
                let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str());
                if let Some(mode) = mode {
                    self.runtime.selection.mode = mode.into();
                }
                self.runtime.selection.ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                ActionEmit::default()
            }
            "setCamera" => {
                if let Some(parsed) = args.and_then(|value| value.get("camera")).and_then(|value| serde_json::from_value(value.clone()).ok()) {
                    self.runtime.camera = parsed;
                }
                ActionEmit::default()
            }
            "importVideo" => {
                let video: Option<SourceVideo> = args
                    .and_then(|value| value.get("video"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok());
                if video.is_none() {
                    return ActionEmit::default();
                }
                ActionEmit::ops(vec![RemodelOp::SetSourceVideo { video }])
            }
            "setParams" => {
                // 📝 Staged typed args (defaults materialized host-side) rebuild the whole params register.
                let mut params = ReconstructionParams::default();
                if let Some(value) = args.and_then(|value| value.get("frameSampleStride")).and_then(|value| value.as_u64()) {
                    params.frame_sample_stride = value as u32;
                }
                if let Some(value) = args.and_then(|value| value.get("maxFrames")).and_then(|value| value.as_u64()) {
                    params.max_frames = value as u32;
                }
                if let Some(value) = args.and_then(|value| value.get("featureTargetCount")).and_then(|value| value.as_u64()) {
                    params.feature_target_count = value as u32;
                }
                if let Some(resolution) = args
                    .and_then(|value| value.get("denseMvsResolution"))
                    .and_then(|value| serde_json::from_value::<DenseResolution>(value.clone()).ok())
                {
                    params.dense_mvs_resolution = resolution;
                }
                if let Some(value) = args.and_then(|value| value.get("tsdfVoxelSizeMm")).and_then(|value| value.as_f64()) {
                    params.tsdf_voxel_size_mm = value as f32;
                }
                ActionEmit::ops(vec![RemodelOp::SetParams { params }])
            }
            "resetPlaceholderMesh" => ActionEmit::ops(vec![RemodelOp::SetResult { result: Some(placeholder_result()) }]),
            "clearResult" => ActionEmit::ops(vec![RemodelOp::SetResult { result: None }]),
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RemodelScene>, view_state: &ViewState) -> UiNode {
        let scene = doc.projection;
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(REMODEL_DEFAULT_UTILITY);
        match body_key {
            REMODEL_PLAY_BODY_MAIN => build_world_3d_scene(
                REMODEL_PLAY_SURFACE_MAIN,
                REMODEL_PLAY_APP_ID,
                world3d_scene(
                    world3d_camera_json(self.runtime.camera.position, self.runtime.camera.target, self.runtime.camera.fov),
                    world_meshes_json(scene),
                    world_instances_json(scene),
                    world3d_selection_json(&self.runtime.selection.mode, &[], None),
                    &WorldSunConfig::default(),
                ),
            ),
            REMODEL_PLAY_BODY_DOCUMENT => build_document_panel(scene, &self.runtime, active_utility, remodel_labels(view_state)),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = remodel_labels(view_state);
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([(REMODEL_PLAY_WINDOW_MAIN.to_string(), labels.model.to_string())]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::from([("model".to_string(), labels.model.to_string())]),
            action_labels: remodel_action_labels(is_de),
            utility_labels: remodel_utility_labels(is_de),
            example_labels: std::collections::HashMap::from([("default".to_string(), labels.default_example.to_string())]),
            action_arg_labels: std::collections::HashMap::new(),
            dialog_labels: std::collections::HashMap::new(),
            introduction_labels: std::collections::HashMap::new(),
        }
    }
}
//#endregion 🔖RemodelPlayApp

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the remodel play app; one field per label makes every locale
/// combination compile-checked. Native-only (no reuse-terminology variant): remodel's domain nouns
/// (video/reconstruction/mesh/vertices/triangles) do not map onto the Object/Vortex/Attraction
/// reuse vocabulary.
struct RemodelLabels {
    model: &'static str,
    set_params: &'static str,
    reset_placeholder_mesh: &'static str,
    clear_result: &'static str,
    import_video: &'static str,
    set_selection: &'static str,
    set_camera: &'static str,
    select: &'static str,
    sculpt: &'static str,
    default_example: &'static str,
    source_video: &'static str,
    no_source_video: &'static str,
    reconstruction: &'static str,
    error: &'static str,
    mesh: &'static str,
    mesh_none: &'static str,
    vertices: &'static str,
    triangles: &'static str,
    utility: &'static str,
    selection: &'static str,
    frames: &'static str,
}

const REMODEL_LABELS_NATIVE_EN: RemodelLabels = RemodelLabels {
    model: "Model",
    set_params: "Set Params",
    reset_placeholder_mesh: "Reset Placeholder Mesh",
    clear_result: "Clear Result",
    import_video: "Import Video",
    set_selection: "Set Selection",
    set_camera: "Set Camera",
    select: "Select",
    sculpt: "Sculpt",
    default_example: "Default",
    source_video: "Source video",
    no_source_video: "No source video imported yet",
    reconstruction: "Reconstruction",
    error: "error",
    mesh: "Mesh",
    mesh_none: "none",
    vertices: "vertices",
    triangles: "triangles",
    utility: "Utility",
    selection: "selection",
    frames: "frames",
};
const REMODEL_LABELS_NATIVE_DE: RemodelLabels = RemodelLabels {
    model: "Modell",
    set_params: "Parameter festlegen",
    reset_placeholder_mesh: "Platzhalter-Mesh zuruecksetzen",
    clear_result: "Ergebnis loeschen",
    import_video: "Video importieren",
    set_selection: "Auswahl festlegen",
    set_camera: "Kamera festlegen",
    select: "Auswaehlen",
    sculpt: "Formen",
    default_example: "Standard",
    source_video: "Quellvideo",
    no_source_video: "Noch kein Quellvideo importiert",
    reconstruction: "Rekonstruktion",
    error: "Fehler",
    mesh: "Mesh",
    mesh_none: "keine",
    vertices: "Vertices",
    triangles: "Dreiecke",
    utility: "Werkzeug",
    selection: "Auswahl",
    frames: "Frames",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unrecognized locales fall back to English.
fn remodel_labels(view_state: &ViewState) -> &'static RemodelLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &REMODEL_LABELS_NATIVE_DE } else { &REMODEL_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_remodel_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the builder chain.
fn remodel_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    let labels = if is_de { &REMODEL_LABELS_NATIVE_DE } else { &REMODEL_LABELS_NATIVE_EN };
    std::collections::HashMap::from([
        ("setParams".to_string(), labels.set_params.to_string()),
        ("resetPlaceholderMesh".to_string(), labels.reset_placeholder_mesh.to_string()),
        ("clearResult".to_string(), labels.clear_result.to_string()),
        ("importVideo".to_string(), labels.import_video.to_string()),
        ("setSelection".to_string(), labels.set_selection.to_string()),
        ("setCamera".to_string(), labels.set_camera.to_string()),
    ])
}

/// 🗣️ (utility id) -> localized toolbar-button label, for every `.utility(...)` declared in `create_remodel_app`.
fn remodel_utility_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    let labels = if is_de { &REMODEL_LABELS_NATIVE_DE } else { &REMODEL_LABELS_NATIVE_EN };
    std::collections::HashMap::from([
        ("select".to_string(), labels.select.to_string()),
        ("sculpt".to_string(), labels.sculpt.to_string()),
    ])
}
//#endregion 🔖CommandLabels

//#region 🔖Manifest
fn create_remodel_app() -> App {
    let default_example = serde_json::to_string(&default_remodel_scene()).expect("remodel default example");
    App::from_builder(
        App::builder(REMODEL_PLAY_APP_ID, "Remodel")
            .document(["semio", "remodel"])
            .icon_id("scan")
            .mode("model", "Model")
            .default_mode_id("model")
            .window_kind(REMODEL_PLAY_WINDOW_MAIN, "Model", REMODEL_PLAY_BODY_MAIN, SurfaceKind::World3d)
            .default_layout(create_default_layout(
                &[REMODEL_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Model".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                REMODEL_PLAY_BODY_DOCUMENT,
            )
            .operation("setParams", "Set Params")
            .operation("resetPlaceholderMesh", "Reset Placeholder Mesh")
            .operation("clearResult", "Clear Result")
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("importVideo", "Import Video", ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setSelection", "Set Selection", ActionKind::View) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", "Set Camera", ActionKind::View) })
            // 📝 Staged argument form for the P1 params action — enumerable, stable reconstruction fields.
            .action_args("setParams", vec![
                ActionArgDef::number("frameSampleStride", "Frame Sample Stride").default_value(5),
                ActionArgDef::number("maxFrames", "Max Frames").default_value(200),
                ActionArgDef::number("featureTargetCount", "Feature Target Count").default_value(4000),
                ActionArgDef::select("denseMvsResolution", "Dense MVS Resolution", vec![
                    ActionArgOption::new("low", "Low"),
                    ActionArgOption::new("medium", "Medium"),
                    ActionArgOption::new("high", "High"),
                ]).default_value("medium"),
                ActionArgDef::slider("tsdfVoxelSizeMm", "TSDF Voxel Size (mm)", 1.0, 20.0).default_value(5.0),
            ])
            // 🧰 Mesh-editing utility group — an exclusive per-window set (active utility is host-owned).
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", "Select", "mouse-pointer-2") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Tools), ..UtilityDefinition::new("sculpt", "Sculpt", "brush") })
            .window_kind_utilities(REMODEL_PLAY_WINDOW_MAIN, vec!["select".into(), "sculpt".into()]),
    )
    .example("default", "Default", &default_example)
    .program("remodel", "Remodel", "mesh")
}

fn remodel_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let scene: RemodelScene = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    Ok(scene.result.map(|result| result.mesh).unwrap_or_else(|| mesh_from_kind("box")))
}

fn register_remodel_exports() {
    semio_framework_os::register_mesh_exporter("3d.remodel", "remodel", remodel_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.remodel", "remodel", remodel_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.remodel", "remodel", remodel_mesh_from_document);
}

semio_framework_plugin::semio_plugin! {
    id: "remodel", label: "Remodel", version: "0.1.0",
    setup: register_remodel_exports,
    apps: [ create_remodel_app => RemodelPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
    use vcs::{Backbone, MemoryBackbone};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<RemodelPlayApp> {
        VcsDocumentApp::new(RemodelPlayApp::default())
    }

    #[test]
    fn initial_document_carries_placeholder_mesh_into_world3d_json() {
        let scene = default_remodel_scene();
        assert!(scene.result.is_some());
        assert!(world_meshes_json(&scene).contains(REMODEL_MESH_ID));
        assert!(world_instances_json(&scene).contains(REMODEL_MESH_ID));
    }

    #[test]
    fn render_does_not_panic_for_known_body_keys() {
        let mut app = new_app();
        let _ = app.render(REMODEL_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render main");
        let _ = app.render(REMODEL_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render document");
    }

    #[test]
    fn clear_and_reset_result_round_trip_through_ops() {
        let mut app = new_app();
        let result = app.handle_action("clearResult", None, &ViewState::default(), &meta("local")).expect("clear");
        assert_eq!(result.operations.len(), 1);
        assert!(app.projection().expect("materialize projection").result.is_none());
        app.handle_action("resetPlaceholderMesh", None, &ViewState::default(), &meta("local")).expect("reset");
        assert_eq!(app.projection().expect("materialize projection").result.map(|r| r.source), Some(MeshSource::Placeholder));
    }

    #[test]
    fn view_actions_mutate_runtime_without_emitting_ops() {
        let mut app = new_app();
        app.handle_action("setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "fov": 60.0 } })), &ViewState::default(), &meta("local")).expect("set camera");
        let node = app.render(REMODEL_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let camera: Value = serde_json::from_str(payload["world3d"]["cameraJson"].as_str().unwrap()).unwrap();
        assert_eq!(camera["position"], json!([1.0, 2.0, 3.0]));
    }

    #[test]
    fn set_active_utility_switches_host_view_state_without_ops_or_history() {
        let mut app = new_app();
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "sculpt" })), &ViewState::default(), &meta("local"))
            .expect("switch utility");
        assert!(result.operations.is_empty(), "utility switch is host-owned view state, never a document op");
        let view_state = ViewState { active_utility_id: Some("sculpt".into()), ..ViewState::default() };
        let document = app.render(REMODEL_PLAY_BODY_DOCUMENT, None, &view_state).expect("render doc");
        assert!(serde_json::to_string(&document).unwrap().contains("sculpt"), "active utility comes from view_state.active_utility_id");
    }

    #[test]
    fn set_params_arg_form_materializes_typed_args_into_ops() {
        let mut app = new_app();
        let result = app
            .handle_action(
                "setParams",
                Some(&json!({
                    "frameSampleStride": 7,
                    "maxFrames": 321,
                    "featureTargetCount": 5000,
                    "denseMvsResolution": "high",
                    "tsdfVoxelSizeMm": 3.5,
                })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("set params");
        assert_eq!(result.operations.len(), 1, "typed args produce one SetParams op");
        let params = app.projection().expect("materialize projection").params;
        assert_eq!(params.frame_sample_stride, 7);
        assert_eq!(params.max_frames, 321);
        assert_eq!(params.feature_target_count, 5000);
        assert_eq!(params.dense_mvs_resolution, DenseResolution::High);
        assert_eq!(params.tsdf_voxel_size_mm, 3.5);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        app.handle_action("clearResult", None, &ViewState::default(), &meta("local")).expect("clear");
        assert!(app.projection().expect("materialize projection").result.is_none());
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert!(app.projection().expect("materialize projection").result.is_some());
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert!(app.projection().expect("materialize projection").result.is_none());
    }

    /// 🧪 The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT field edits (A imports a source video, B clears the result mesh), and exchanging ops
    /// over a `MemoryBackbone` converges both sides to contain BOTH edits — impossible under a
    /// whole-document `setDocument` snapshot, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://remodel-convergence", "mem://remodel-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a
            .handle_action(
                "importVideo",
                Some(&json!({ "video": { "assetId": "vid-1", "filename": "scan.mp4", "frameCount": 120, "fps": 30.0 } })),
                &ViewState::default(),
                &meta("actor-a"),
            )
            .expect("a imports video");
        instance_b.handle_action("clearResult", None, &ViewState::default(), &meta("actor-b")).expect("b clears result");

        // A neutral history action pumps inbound ops without touching applied_edit_ids the way undo
        // would (RemodelOp does not override Operation::author_id).
        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("materialize projection");
        let projection_b = instance_b.projection().expect("materialize projection");

        assert_eq!(projection_a.source_video.as_ref().map(|v| v.filename.clone()).as_deref(), Some("scan.mp4"), "A keeps its own video import");
        assert_eq!(projection_b.source_video.as_ref().map(|v| v.filename.clone()).as_deref(), Some("scan.mp4"), "B converges on A's remote video import");
        assert!(projection_a.result.is_none(), "A converges on B's remote result clear");
        assert!(projection_b.result.is_none(), "B keeps its own result clear");
    }
}
//#endregion 🧪Tests
