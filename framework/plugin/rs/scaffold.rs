//! 🧰 Helpers for scaffolding standard technology plugins.

use crate::{
    build_canvas_2d_scene, build_node_graph_scene, build_table_scene, build_text_editor_scene,
    build_world_3d_scene, ui_stack_vertical, ui_text, App, Canvas2dScene, NodeGraphScene,
    PluginApp, TableScene, TextEditorScene, UiNode, ViewState, World3dScene,
};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneKind {
    Canvas2d,
    World3d,
    NodeGraph,
    TextEditor,
    Table,
    Raster,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StandardApp {
    pub app_id: &'static str,
    pub label: &'static str,
    pub program_id: Option<&'static str>,
    pub yields: Option<&'static str>,
    pub surface_id: &'static str,
    pub body_key: &'static str,
    pub scene_kind: SceneKind,
    pub initial_document_json: &'static str,
}

pub struct StandardPluginApp {
    pub spec: StandardApp,
}

impl PluginApp for StandardPluginApp {
    fn app_id(&self) -> &str {
        self.spec.app_id
    }

    fn initial_document_json(&self) -> String {
        self.spec.initial_document_json.to_string()
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        if command == "setDocument" {
            if let Some(document) = args.and_then(|value| value.get("document")) {
                return vec![serde_json::json!({ "op": "setDocument", "document": document }).to_string()];
            }
        }
        if command == "patch" {
            if let Some(patch) = args.and_then(|value| value.get("patch")) {
                return vec![serde_json::json!({ "op": "patch", "patch": patch }).to_string()];
            }
        }
        let _ = document_json;
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        if body_key != self.spec.body_key {
            return ui_text(format!("Unknown body: {body_key}"));
        }
        match self.spec.scene_kind {
            SceneKind::Canvas2d => build_canvas_2d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                Canvas2dScene {
                    camera_x: 0.0,
                    camera_y: 0.0,
                    zoom: 1.0,
                    layers_json: document_json.to_string(),
                },
            ),
            SceneKind::World3d => build_world_3d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                World3dScene {
                    camera_json: r#"{"x":0,"y":0,"z":5}"#.into(),
                    instances_json: document_json.to_string(),
                },
            ),
            SceneKind::NodeGraph => build_node_graph_scene(
                self.spec.surface_id,
                self.spec.app_id,
                NodeGraphScene {
                    nodes_json: document_json.to_string(),
                    edges_json: "[]".into(),
                    viewport_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
                },
            ),
            SceneKind::TextEditor => build_text_editor_scene(
                self.spec.surface_id,
                self.spec.app_id,
                TextEditorScene {
                    buffer: document_json.to_string(),
                    language: Some("plain".into()),
                    selection_json: None,
                },
            ),
            SceneKind::Table => build_table_scene(
                self.spec.surface_id,
                self.spec.app_id,
                TableScene {
                    columns_json: r#"[{"id":"id","label":"Id"}]"#.into(),
                    rows_json: document_json.to_string(),
                },
            ),
            SceneKind::Raster => ui_stack_vertical(vec![ui_text("Raster viewport"), ui_text(document_json)]),
        }
    }
}

pub fn standard_app(spec: StandardApp) -> App {
    let app = App::from_builder(
        App::builder(spec.app_id, spec.label)
            .icon_id(spec.app_id)
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind("main", "Main", spec.body_key),
    );
    if let (Some(program_id), Some(yields)) = (spec.program_id, spec.yields) {
        app.program(program_id, spec.label, yields)
    } else {
        app
    }
}

pub fn standard_factory(spec: StandardApp) -> Box<dyn PluginApp> {
    Box::new(StandardPluginApp { spec })
}

pub fn register_standard_app(mut bundle: crate::PluginBundle, spec: StandardApp) -> crate::PluginBundle {
    let app = standard_app(spec);
    bundle.register_app(app, move || standard_factory(spec))
}
