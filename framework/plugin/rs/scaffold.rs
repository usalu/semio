//! 🧰 Helpers for scaffolding standard technology plugins.

use crate::{
    build_canvas_2d_scene, build_node_graph_scene, build_raster_scene, build_table_scene,
    build_text_editor_scene, build_world_3d_scene, default_world3d_selection, ui_stack_vertical,
    ui_text, world3d_default_meshes_json, App, Canvas2dScene, NodeGraphScene, PluginApp,
    RasterScene, TableScene, TextEditorScene, UiNode, ViewState, World3dScene,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
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
    pub hierarchy: &'static [&'static str],
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

fn hierarchy_body_key(body_key: &str) -> String {
    body_key.replace(".composite", ".hierarchy")
}

fn properties_body_key(body_key: &str) -> String {
    body_key.replace(".composite", ".properties")
}

fn json_field(document: &Value, keys: &[&str]) -> Option<Value> {
    keys.iter().find_map(|key| document.get(*key)).cloned()
}

fn canvas_layers_json(document: &Value, fallback: &str) -> String {
    json_field(
        document,
        &["layers", "tiles", "blocks", "features", "cells", "nodes"],
    )
    .map(|value| value.to_string())
    .unwrap_or_else(|| fallback.to_string())
}

fn world_instances_json(document: &Value, fallback: &str) -> String {
    json_field(
        document,
        &["instances", "entities", "meshes", "tiles", "cells", "parts"],
    )
    .map(|value| value.to_string())
    .unwrap_or_else(|| fallback.to_string())
}

fn node_graph_payload(document: &Value, fallback: &str) -> (String, String) {
    if let Some(nodes) = document.get("nodes") {
        let edges = document
            .get("edges")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        return (nodes.to_string(), edges.to_string());
    }
    if let Some(flow) = document.get("flow") {
        let nodes = flow
            .get("components")
            .or_else(|| flow.get("nodes"))
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        let edges = flow
            .get("edges")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        return (nodes.to_string(), edges.to_string());
    }
    if let Some(steps) = document.get("steps") {
        return (steps.to_string(), "[]".into());
    }
    (fallback.into(), "[]".into())
}

fn text_editor_payload(document: &Value, fallback: &str) -> (String, Option<String>) {
    if let Some(text) = document
        .get("text")
        .or_else(|| document.get("source"))
        .and_then(|value| value.as_str())
    {
        let language = document
            .get("language")
            .and_then(|value| value.as_str())
            .map(str::to_string);
        return (text.into(), language);
    }
    if document.is_string() {
        return (
            document.as_str().unwrap_or(fallback).into(),
            Some("plain".into()),
        );
    }
    (fallback.into(), Some("plain".into()))
}

fn table_payload(document: &Value, fallback: &str) -> (String, String) {
    let rows = json_field(document, &["rows", "edits", "records"])
        .map(|value| value.to_string())
        .unwrap_or_else(|| fallback.to_string());
    let columns = document
        .get("columns")
        .map(|value| value.to_string())
        .unwrap_or_else(|| r#"[{"id":"id","label":"Id"}]"#.into());
    (columns, rows)
}

fn raster_payload(document: &Value, fallback: &str) -> RasterScene {
    if let Ok(scene) = serde_json::from_value::<RasterScene>(document.clone()) {
        return scene;
    }
    let parsed: Value = serde_json::from_str(fallback).unwrap_or(Value::Null);
    RasterScene {
        width: document
            .get("width")
            .or_else(|| parsed.get("width"))
            .and_then(|value| value.as_u64())
            .unwrap_or(256) as u32,
        height: document
            .get("height")
            .or_else(|| parsed.get("height"))
            .and_then(|value| value.as_u64())
            .unwrap_or(256) as u32,
        pixels_base64: document
            .get("pixelsBase64")
            .or_else(|| document.get("pixels_base64"))
            .and_then(|value| value.as_str())
            .unwrap_or("")
            .into(),
    }
}

pub fn scene_kind_component_tag(kind: SceneKind) -> &'static str {
    match kind {
        SceneKind::Canvas2d => "canvas-2d",
        SceneKind::World3d => "world-3d",
        SceneKind::NodeGraph => "node-graph",
        SceneKind::TextEditor => "text-editor",
        SceneKind::Table => "table",
        SceneKind::Raster => "raster",
    }
}

pub fn assert_standard_app_renders(spec: StandardApp) {
    let app = StandardPluginApp { spec };
    let node = app.render(spec.body_key, spec.initial_document_json, &ViewState::default());
    let json = serde_json::to_string(&node).expect("ui json");
    let tag = scene_kind_component_tag(spec.scene_kind);
    assert!(json.contains(tag), "expected {tag} in {json}");
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
        let hierarchy_key = hierarchy_body_key(self.spec.body_key);
        let properties_key = properties_body_key(self.spec.body_key);
        if body_key == hierarchy_key {
            return render_hierarchy_panel(self.spec.label, document_json);
        }
        if body_key == properties_key {
            return render_properties_panel(self.spec.label, document_json);
        }
        if body_key != self.spec.body_key {
            return ui_text(format!("Unknown body: {body_key}"));
        }
        let document: Value =
            serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
        match self.spec.scene_kind {
            SceneKind::Canvas2d => build_canvas_2d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                Canvas2dScene {
                    camera_x: document
                        .get("camera")
                        .and_then(|camera| camera.get("x"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(0.0),
                    camera_y: document
                        .get("camera")
                        .and_then(|camera| camera.get("y"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(0.0),
                    zoom: document
                        .get("camera")
                        .and_then(|camera| camera.get("zoom"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(1.0),
                    layers_json: canvas_layers_json(&document, document_json),
                },
            ),
            SceneKind::World3d => build_world_3d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                World3dScene {
                    camera_json: document
                        .get("camera")
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| r#"{"x":0,"y":0,"z":5}"#.into()),
                    meshes_json: document
                        .get("meshes")
                        .map(|value| value.to_string())
                        .unwrap_or_else(world3d_default_meshes_json),
                    instances_json: world_instances_json(&document, document_json),
                    selection_json: document
                        .get("selection")
                        .map(|value| value.to_string())
                        .unwrap_or_else(default_world3d_selection),
                    vortices_json: None,
                    attractions_json: None,
                    target_volumes_json: None,
                    references_json: None,
                    brush_preview_json: None,
                    interaction_json: None,
                },
            ),
            SceneKind::NodeGraph => {
                let (nodes_json, edges_json) = node_graph_payload(&document, document_json);
                build_node_graph_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    NodeGraphScene::base(
                        nodes_json,
                        edges_json,
                        document
                            .get("viewport")
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| r#"{"x":0,"y":0,"zoom":1}"#.into()),
                    ),
                )
            }
            SceneKind::TextEditor => {
                let (buffer, language) = text_editor_payload(&document, document_json);
                build_text_editor_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    TextEditorScene::base(buffer, language, None),
                )
            }
            SceneKind::Table => {
                let (columns_json, rows_json) = table_payload(&document, document_json);
                build_table_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    TableScene {
                        columns_json,
                        rows_json,
                    },
                )
            }
            SceneKind::Raster => build_raster_scene(
                self.spec.surface_id,
                self.spec.app_id,
                raster_payload(&document, document_json),
            ),
        }
    }
}

fn render_hierarchy_panel(label: &str, document_json: &str) -> UiNode {
    let document: Value =
        serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
    let schema = document
        .get("schema")
        .and_then(|value| value.as_str())
        .unwrap_or(label);
    let count = document
        .get("layers")
        .or_else(|| document.get("nodes"))
        .or_else(|| document.get("rows"))
        .or_else(|| document.get("entities"))
        .and_then(|value| value.as_array())
        .map(|rows| rows.len())
        .unwrap_or(0);
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {schema}")),
        ui_text(format!("Items: {count}")),
    ])
}

fn render_properties_panel(label: &str, document_json: &str) -> UiNode {
    let document: Value =
        serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
    let id = document
        .get("id")
        .and_then(|value| value.as_str())
        .unwrap_or(label);
    ui_stack_vertical(vec![
        ui_text(format!("App: {label}")),
        ui_text(format!("Id: {id}")),
    ])
}

pub fn standard_app(spec: StandardApp) -> App {
    let hierarchy_key = hierarchy_body_key(spec.body_key);
    let properties_key = properties_body_key(spec.body_key);
    let app = App::from_builder(
        App::builder(spec.app_id, spec.label)
            .hierarchy(spec.hierarchy.iter().copied())
            .icon_id(spec.app_id)
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind("main", "Main", spec.body_key)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                &hierarchy_key,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                &properties_key,
            ),
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

pub fn register_standard_app(bundle: crate::PluginBundle, spec: StandardApp) -> crate::PluginBundle {
    let app = standard_app(spec);
    bundle.register_app(app, move || standard_factory(spec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-canvas",
            label: "Canvas",
            hierarchy: &["semio", "test", "canvas"],
            program_id: None,
            yields: None,
            surface_id: "test.canvas",
            body_key: "test.canvas.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"test","id":"test","layers":[]}"#,
        });
    }

    #[test]
    fn node_graph_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-graph",
            label: "Graph",
            hierarchy: &["semio", "test", "graph"],
            program_id: None,
            yields: None,
            surface_id: "test.graph",
            body_key: "test.graph.composite",
            scene_kind: SceneKind::NodeGraph,
            initial_document_json: r#"{"nodes":[],"edges":[]}"#,
        });
    }

    #[test]
    fn world_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-world",
            label: "World",
            hierarchy: &["semio", "test", "world"],
            program_id: None,
            yields: None,
            surface_id: "test.world",
            body_key: "test.world.composite",
            scene_kind: SceneKind::World3d,
            initial_document_json: r#"{"schema":"test","id":"test","entities":[]}"#,
        });
    }

    #[test]
    fn text_editor_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-text",
            label: "Text",
            hierarchy: &["semio", "test", "text"],
            program_id: None,
            yields: None,
            surface_id: "test.text",
            body_key: "test.text.composite",
            scene_kind: SceneKind::TextEditor,
            initial_document_json: r#"{"schema":"test","id":"test","source":""}"#,
        });
    }

    #[test]
    fn table_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-table",
            label: "Table",
            hierarchy: &["semio", "test", "table"],
            program_id: None,
            yields: None,
            surface_id: "test.table",
            body_key: "test.table.composite",
            scene_kind: SceneKind::Table,
            initial_document_json: r#"{"schema":"test","id":"test","rows":[]}"#,
        });
    }

    #[test]
    fn raster_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-raster",
            label: "Raster",
            hierarchy: &["semio", "test", "raster"],
            program_id: None,
            yields: None,
            surface_id: "test.raster",
            body_key: "test.raster.composite",
            scene_kind: SceneKind::Raster,
            initial_document_json: r#"{"schema":"raster.document","id":"raster","width":64,"height":64,"pixelsBase64":""}"#,
        });
    }
}
