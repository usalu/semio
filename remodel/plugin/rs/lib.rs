//! 🏺 Remodel plugin — photogrammetry play app (video → watertight mesh) bundled as a hot-swappable WASM component.

use remodel_document::{
    default_active_tool, default_remodel_scene, CameraState, RemodelMesh, RemodelOp, RemodelScene, SelectionState,
    SourceVideo, MeshSource, REMODEL_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, mesh_from_kind, ui_stack_vertical, ui_text, world3d_camera_json,
    world3d_scene, world3d_selection_json, ActionEmit, App, DocumentApp, DocumentView, MeshData, PanelGroup, SurfaceKind,
    UiNode, ViewState, WorldSunConfig, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde_json::{json, Value};

//#region 🔖Constants
const REMODEL_PLAY_APP_ID: &str = "remodel-play";
const REMODEL_PLAY_SURFACE_MAIN: &str = "remodel.play";
const REMODEL_PLAY_BODY_MAIN: &str = "remodel.play.main";
const REMODEL_PLAY_BODY_DOCUMENT: &str = "remodel.play.document";
const REMODEL_PLAY_WINDOW_MAIN: &str = "remodel-main";
const REMODEL_MESH_ID: &str = "remodel-result";
//#endregion 🔖Constants

//#region 🔖Runtime
/// 🎛️ Ephemeral viewport state (orbit camera, face/vertex selection, active transform tool) — lives in
/// the app struct, never in the document, so panning the camera or picking a face never lands in undo
/// history nor syncs to peers.
#[derive(Clone, Debug, PartialEq)]
struct RemodelPlayRuntime {
    camera: CameraState,
    selection: SelectionState,
    active_tool: String,
}

impl Default for RemodelPlayRuntime {
    fn default() -> Self {
        Self {
            camera: CameraState::default(),
            selection: SelectionState::default(),
            active_tool: default_active_tool(),
        }
    }
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

fn build_document_panel(scene: &RemodelScene, runtime: &RemodelPlayRuntime) -> UiNode {
    let video_label = scene
        .source_video
        .as_ref()
        .map(|video| format!("Source video: {} ({} frames, {:.1} fps)", video.filename, video.frame_count, video.fps))
        .unwrap_or_else(|| "No source video imported yet".into());
    let job_label = format!(
        "Reconstruction: {:?} ({:.0}%){}",
        scene.job.stage,
        scene.job.progress_0_1 * 100.0,
        scene
            .job
            .error
            .as_ref()
            .map(|error| format!(" — error: {error}"))
            .unwrap_or_default()
    );
    let mesh_label = scene
        .result
        .as_ref()
        .map(|result| {
            format!(
                "Mesh: {:?}, {} vertices, {} triangles",
                result.source,
                result.mesh.vertex_count(),
                result.mesh.triangle_count()
            )
        })
        .unwrap_or_else(|| "Mesh: none".into());
    let tool_label = format!("Tool: {} · selection: {} ({})", runtime.active_tool, runtime.selection.mode, runtime.selection.ids.len());
    ui_stack_vertical(vec![ui_text(video_label), ui_text(job_label), ui_text(mesh_label), ui_text(tool_label)])
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
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    self.runtime.active_tool = tool.into();
                }
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
                match args.and_then(|value| value.get("params")).and_then(|value| serde_json::from_value(value.clone()).ok()) {
                    Some(params) => ActionEmit::ops(vec![RemodelOp::SetParams { params }]),
                    None => ActionEmit::default(),
                }
            }
            "resetPlaceholderMesh" => ActionEmit::ops(vec![RemodelOp::SetResult { result: Some(placeholder_result()) }]),
            "clearResult" => ActionEmit::ops(vec![RemodelOp::SetResult { result: None }]),
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RemodelScene>, _view_state: &ViewState) -> UiNode {
        let scene = doc.projection;
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
            REMODEL_PLAY_BODY_DOCUMENT => build_document_panel(scene, &self.runtime),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖RemodelPlayApp

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
            .operation("importVideo", "Import Video")
            .operation("setParams", "Set Params")
            .operation("resetPlaceholderMesh", "Reset Placeholder Mesh")
            .operation("clearResult", "Clear Result")
            .view_action("setActiveTool", "Set Active Tool")
            .view_action("setSelection", "Set Selection")
            .view_action("setCamera", "Set Camera"),
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
        let result = app
            .handle_action("setActiveTool", Some(&json!({ "tool": "sculpt" })), &ViewState::default(), &meta("local"))
            .expect("set tool");
        assert!(result.operations.is_empty(), "active tool is ephemeral view state");
        app.handle_action("setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "fov": 60.0 } })), &ViewState::default(), &meta("local")).expect("set camera");
        let node = app.render(REMODEL_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let camera: Value = serde_json::from_str(payload["world3d"]["cameraJson"].as_str().unwrap()).unwrap();
        assert_eq!(camera["position"], json!([1.0, 2.0, 3.0]));
        let document = app.render(REMODEL_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render doc");
        assert!(serde_json::to_string(&document).unwrap().contains("sculpt"));
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
