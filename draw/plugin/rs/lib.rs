//! ✏️ Draw plugin — declarative draw app bundled as a hot-swappable WASM component.

use draw::{empty_draw_projection, DrawDocument, DRAW_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ui_stack_vertical, ui_text, App, Canvas2dScene, PluginApp, PluginBundle, UiNode, ViewState,
    build_canvas_2d_scene,
};
use serde_json::Value;

const DRAW_PLAY_APP_ID: &str = "draw-play";
const DRAW_PLAY_SURFACE_ID: &str = "draw.play.composite";
const DRAW_PLAY_BODY_COMPOSITE: &str = "draw.play.composite";
const DRAW_PLAY_BODY_LAYERS: &str = "draw.play.layers";
const DRAW_PLAY_BODY_CATALOGUE: &str = "draw.play.catalogue";
const DRAW_PLAY_BODY_PROPERTIES: &str = "draw.play.properties";

struct DrawApp;

impl PluginApp for DrawApp {
    fn app_id(&self) -> &str {
        DRAW_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&empty_draw_projection()).expect("draw document json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut document: DrawDocument = serde_json::from_str(document_json).unwrap_or_else(|_| empty_draw_projection());
        match command {
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    document.active_tool = Some(tool.into());
                    return vec![serde_json::json!({ "op": "setDocument", "document": document }).to_string()];
                }
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(camera) = serde_json::from_value(camera.clone()) {
                        document.camera = Some(camera);
                        return vec![serde_json::json!({ "op": "setDocument", "document": document }).to_string()];
                    }
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let document: DrawDocument = serde_json::from_str(document_json).unwrap_or_else(|_| empty_draw_projection());
        match body_key {
            DRAW_PLAY_BODY_COMPOSITE => render_canvas(&document),
            DRAW_PLAY_BODY_LAYERS => render_layers_panel(&document),
            DRAW_PLAY_BODY_CATALOGUE => ui_stack_vertical(vec![ui_text("Catalogue")]),
            DRAW_PLAY_BODY_PROPERTIES => render_properties_panel(&document),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn render_canvas(document: &DrawDocument) -> UiNode {
    let camera = document.camera.clone().unwrap_or(draw::DrawCamera { x: 0.0, y: 0.0, zoom: 1.0 });
    build_canvas_2d_scene(
        DRAW_PLAY_SURFACE_ID,
        DRAW_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            layers_json: serde_json::to_string(&document.layers).unwrap_or_else(|_| "[]".into()),
        },
    )
}

fn render_layers_panel(document: &DrawDocument) -> UiNode {
    if document.layers.is_empty() {
        return ui_stack_vertical(vec![ui_text("No layers yet.")]);
    }
    ui_stack_vertical(
        document
            .layers
            .iter()
            .map(|layer| ui_text(layer_label(layer)))
            .collect(),
    )
}

fn render_properties_panel(document: &DrawDocument) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {}", DRAW_DOCUMENT_SCHEMA)),
        ui_text(format!("Tool: {}", document.active_tool.clone().unwrap_or_else(|| "selectMarquee".into()))),
    ])
}

fn layer_label(layer: &draw::DrawLayerNode) -> String {
    match layer {
        draw::DrawLayerNode::Shape(shape) => shape.base.name.clone(),
        draw::DrawLayerNode::Group(group) => group.base.name.clone(),
    }
}

fn create_draw_app() -> App {
    App::from_builder(
        App::builder(DRAW_PLAY_APP_ID, "Draw")
            .icon_id("draw")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind("draw-composite", "Canvas", DRAW_PLAY_BODY_COMPOSITE)
            .panel_tab("framework.panel.hierarchy", "Hierarchy", "workbench", DRAW_PLAY_BODY_LAYERS)
            .panel_tab("framework.panel.catalogue", "Catalogue", "workbench", DRAW_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", "Inspection", "details", DRAW_PLAY_BODY_PROPERTIES)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("empty", "Empty", serde_json::to_string(&empty_draw_projection()).unwrap())
    .program("draw", "Draw", "image")
}

fn draw_bundle() -> PluginBundle {
    PluginBundle::new("draw", "Draw", "0.1.0").register_app(create_draw_app(), || Box::new(DrawApp))
}

static _PLUGIN_INIT: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
    semio_framework_plugin::install_plugin_bundle(draw_bundle());
});

semio_framework_plugin::wasm_plugin_exports!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_canvas_scene() {
        let app = DrawApp;
        let document = serde_json::to_string(&empty_draw_projection()).unwrap();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }
}
