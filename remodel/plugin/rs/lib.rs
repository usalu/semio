//! 🏺 Remodel plugin — photogrammetry play app (video → watertight mesh) bundled as a hot-swappable WASM component.

use remodel_document::{default_remodel_scene, RemodelScene};
use semio_framework_plugin::{
    build_world_3d_scene, mesh_from_kind, ui_stack_vertical, ui_text, world3d_camera_json, world3d_scene,
    world3d_selection_json, App, MeshData, PanelGroup, PluginApp, SurfaceKind, ToolNode, UiNode, ViewState,
    WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖Constants
const REMODEL_PLAY_APP_ID: &str = "remodel-play";
const REMODEL_PLAY_SURFACE_MAIN: &str = "remodel.play";
const REMODEL_PLAY_BODY_MAIN: &str = "remodel.play.main";
const REMODEL_PLAY_BODY_DOCUMENT: &str = "remodel.play.document";
const REMODEL_PLAY_WINDOW_MAIN: &str = "remodel-main";
const REMODEL_MESH_ID: &str = "remodel-result";
//#endregion 🔖Constants

//#region 🔖Document
fn parse_scene(document_json: &str) -> RemodelScene {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_remodel_scene())
}

fn set_document_op(scene: &RemodelScene) -> String {
    json!({ "op": "setDocument", "document": scene }).to_string()
}

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

fn build_document_panel(scene: &RemodelScene) -> UiNode {
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
    ui_stack_vertical(vec![ui_text(video_label), ui_text(job_label), ui_text(mesh_label)])
}
//#endregion 🔖Document

//#region 🔖RemodelPlayApp
#[derive(Default)]
struct RemodelPlayApp;

impl PluginApp for RemodelPlayApp {
    fn app_id(&self) -> &str {
        REMODEL_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_remodel_scene()).expect("remodel default scene json")
    }

    fn tools(&self, _document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        Vec::new()
    }

    fn window_measures(
        &self,
        _document_json: &str,
        _view_state: &ViewState,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::new()
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut scene = parse_scene(document_json);
        match action {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<RemodelScene>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    scene.active_tool = Some(tool.into());
                    return vec![set_document_op(&scene)];
                }
            }
            "setSelection" => {
                let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("face");
                let ids: Vec<u32> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                scene.selection.mode = mode.into();
                scene.selection.ids = ids;
                return vec![set_document_op(&scene)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        scene.camera = parsed;
                        return vec![set_document_op(&scene)];
                    }
                }
            }
            "resetPlaceholderMesh" => {
                scene.result = Some(remodel_document::RemodelMesh {
                    mesh: mesh_from_kind("box"),
                    source: remodel_document::MeshSource::Placeholder,
                });
                return vec![set_document_op(&scene)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let scene = parse_scene(document_json);
        match body_key {
            REMODEL_PLAY_BODY_MAIN => build_world_3d_scene(
                REMODEL_PLAY_SURFACE_MAIN,
                REMODEL_PLAY_APP_ID,
                world3d_scene(
                    world3d_camera_json(scene.camera.position, scene.camera.target, scene.camera.fov),
                    world_meshes_json(&scene),
                    world_instances_json(&scene),
                    world3d_selection_json("none", &[], None),
                    &WorldSunConfig::default(),
                ),
            ),
            REMODEL_PLAY_BODY_DOCUMENT => build_document_panel(&scene),
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
            .window_kind(REMODEL_PLAY_WINDOW_MAIN, "Model", REMODEL_PLAY_BODY_MAIN, SurfaceKind::World3d)
            .default_layout(semio_framework_plugin::create_default_layout(
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
            ),
    )
    .example("default", "Default", &default_example)
    .program("remodel", "Remodel", "mesh")
}

fn remodel_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let scene: RemodelScene = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    Ok(scene.result.map(|result| result.mesh).unwrap_or_else(|| mesh_from_kind("box")))
}

fn register_remodel_exports() {
    semio_framework_os::register_mesh_export_handlers("3d.remodel", "remodel", remodel_mesh_from_document);
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

    #[test]
    fn initial_document_renders_placeholder_mesh_in_main_window() {
        let mut app = RemodelPlayApp;
        let document_json = app.initial_document_json();
        let view_state = ViewState::default();
        let node = app.render(REMODEL_PLAY_BODY_MAIN, &document_json, &view_state);
        let scene_json = serde_json::to_value(&node).expect("ui node json");
        let meshes_json = scene_json
            .pointer("/component/world3d/meshesJson")
            .and_then(|value| value.as_str())
            .expect("meshes_json present");
        assert!(meshes_json.contains(REMODEL_MESH_ID));
        let _ = app.handle_action_patch_ops("resetPlaceholderMesh", None, &document_json, &view_state);
    }
}
//#endregion 🧪Tests
