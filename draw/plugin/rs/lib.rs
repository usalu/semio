//! ✏️ Draw plugin — declarative draw app bundled as a hot-swappable WASM component.

use draw::{
    apply_draw_edit_op, create_draw_boolean_layer, create_draw_path_layer, create_layer_by_kind, default_draw_document,
    draw_layer_world_bounds, draw_play_boolean_child_row_id, draw_play_layer_id_from_tree_row_id, draw_play_layers_tree_row_id,
    empty_draw_projection, find_draw_layer, find_draw_layer_location, flatten_draw_document_to_scene_nodes,
    flatten_draw_layers, layer_base, layer_id, layer_kind_label, mutate_draw_layer, patch_layer_field, rgba_to_hex,
    DrawDocument, DrawLayerNode, DrawOp, PathSegment, DRAW_BLEND_MODES, DRAW_BOOLEAN_OPS, DRAW_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_select, ui_inspector_mixed_slider, ui_inspector_mixed_text, ui_inspector_mixed_toggle,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, Canvas2dScene, CommandDescriptor, PanelGroup, UiControlNode,
    UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
    layout::WindowEngagementStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

const DRAW_PLAY_APP_ID: &str = "draw-play";
const DRAW_PLAY_CONTROLLER_ID: &str = "draw-play";
const DRAW_PLAY_SURFACE_ID: &str = "draw.play.composite";
const DRAW_PLAY_BODY_COMPOSITE: &str = "draw.play.composite";
const DRAW_PLAY_BODY_LAYERS: &str = "draw.play.layers";
const DRAW_PLAY_BODY_CATALOGUE: &str = "draw.play.catalogue";
const DRAW_PLAY_BODY_PROPERTIES: &str = "draw.play.properties";
const DRAW_LAYER_KIND_DRAG_MIME: &str = "application/x-semio-draw-layer-kind";
const DRAW_PLAY_EXAMPLE_DEFAULT_ID: &str = "semio";
const SEMIO_DRAW_EXAMPLE_JSON: &str = include_str!("../../example/semio.draw.json");

//#region 🔖Interaction
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrawInteractionState {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    hovered_id: Option<String>,
    #[serde(default)]
    engagement_input: String,
    #[serde(default)]
    drawing_path_id: Option<String>,
}

fn draw_play_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: DRAW_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn parse_interaction(view_state: &ViewState) -> DrawInteractionState {
    if let Some(selection_json) = &view_state.selection_json {
        if let Ok(ids) = serde_json::from_str::<Vec<String>>(selection_json) {
            return DrawInteractionState { selected_ids: ids, ..Default::default() };
        }
        if let Ok(value) = serde_json::from_str::<DrawInteractionState>(selection_json) {
            return value;
        }
    }
    view_state
        .panel_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_default()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrawPlayEnvelope {
    #[serde(flatten)]
    document: DrawDocument,
    #[serde(default)]
    undo_stack: Vec<DrawDocument>,
    #[serde(default)]
    redo_stack: Vec<DrawDocument>,
    #[serde(default)]
    interaction: DrawInteractionState,
}

fn parse_envelope(document_json: &str) -> DrawPlayEnvelope {
    if let Ok(envelope) = serde_json::from_str::<DrawPlayEnvelope>(document_json) {
        return envelope;
    }
    let document: DrawDocument = serde_json::from_str(document_json).unwrap_or_else(|_| empty_draw_projection());
    DrawPlayEnvelope {
        document,
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
        interaction: DrawInteractionState::default(),
    }
}

fn set_document_op(envelope: &DrawPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn interaction_state(envelope: &DrawPlayEnvelope, view_state: &ViewState) -> DrawInteractionState {
    if !envelope.interaction.selected_ids.is_empty()
        || envelope.interaction.hovered_id.is_some()
        || !envelope.interaction.engagement_input.is_empty()
    {
        return envelope.interaction.clone();
    }
    parse_interaction(view_state)
}

fn push_undo(play: &mut DrawPlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}

fn canvas_point_to_world(camera: &draw::DrawCamera, x: f64, y: f64, viewport_w: f64, viewport_h: f64) -> (f64, f64) {
    let zoom = camera.zoom.max(0.01);
    (
        (x - viewport_w * 0.5) / zoom - camera.x,
        (y - viewport_h * 0.5) / zoom - camera.y,
    )
}

fn pick_layer_at(document: &DrawDocument, world_x: f64, world_y: f64) -> Option<String> {
    flatten_draw_layers(&document.layers).into_iter().rev().find_map(|layer| {
        let (x, y, width, height) = draw_layer_world_bounds(layer)?;
        if world_x >= x && world_x <= x + width && world_y >= y && world_y <= y + height {
            Some(layer_id(layer).to_string())
        } else {
            None
        }
    })
}

fn append_pen_point(document: &DrawDocument, path_id: &str, point: [f64; 2], start: bool) -> DrawDocument {
    mutate_draw_layer(document, path_id, |layer| {
        if let DrawLayerNode::Path(path) = layer {
            if start {
                path.segments = vec![
                    PathSegment::Move { to: point },
                    PathSegment::Line { to: point },
                ];
            } else {
                path.segments.push(PathSegment::Line { to: point });
            }
        }
    })
}

fn begin_pen_path(document: &DrawDocument, point: [f64; 2]) -> (DrawDocument, String) {
    let layer = create_draw_path_layer("Pen path", vec![PathSegment::Move { to: point }, PathSegment::Line { to: point }]);
    let id = layer_id(&layer).to_string();
    let next = apply_draw_edit_op(document, &DrawOp::AddLayer { parent_id: None, index: None, layer });
    (next, id)
}
//#endregion 🔖Interaction

//#region 🔖DrawApp
struct DrawApp;

impl semio_framework_plugin::PluginApp for DrawApp {
    fn app_id(&self) -> &str {
        DRAW_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&DrawPlayEnvelope {
            document: default_draw_document("empty", None),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            interaction: DrawInteractionState::default(),
        })
        .expect("draw document json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let interaction = interaction_state(&play, view_state);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<DrawPlayEnvelope>(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                    if let Ok(parsed) = serde_json::from_value::<DrawDocument>(next.clone()) {
                        play.document = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setSelection" => {
                play.interaction.selected_ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                return vec![set_document_op(&play)];
            }
            "setHover" => {
                play.interaction.hovered_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&play)];
            }
            "undo" => {
                if let Some(previous) = play.undo_stack.pop() {
                    play.redo_stack.push(play.document.clone());
                    play.document = previous;
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.redo_stack.pop() {
                    play.undo_stack.push(play.document.clone());
                    play.document = next;
                    return vec![set_document_op(&play)];
                }
            }
            "selectAll" => {
                play.interaction.selected_ids = flatten_draw_layers(&play.document.layers)
                    .into_iter()
                    .map(|layer| layer_id(layer).to_string())
                    .collect();
                return vec![set_document_op(&play)];
            }
            "clearSelection" => {
                play.interaction.selected_ids.clear();
                return vec![set_document_op(&play)];
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    push_undo(&mut play);
                    play.document = apply_draw_edit_op(&play.document, &DrawOp::SetActiveTool { tool: tool.into() });
                    return vec![set_document_op(&play)];
                }
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(camera) = serde_json::from_value(camera.clone()) {
                        push_undo(&mut play);
                        play.document = apply_draw_edit_op(&play.document, &DrawOp::SetCamera { camera });
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setCameraZoom" => {
                let zoom = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let mut camera = play.document.camera.clone();
                camera.zoom = zoom;
                push_undo(&mut play);
                play.document = apply_draw_edit_op(&play.document, &DrawOp::SetCamera { camera });
                return vec![set_document_op(&play)];
            }
            "setSelectedOpacity" => {
                let opacity = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let mut next = play.document.clone();
                for layer_id in &interaction.selected_ids {
                    next = apply_draw_edit_op(&next, &DrawOp::SetLayerOpacity { layer_id: layer_id.clone(), opacity });
                }
                push_undo(&mut play);
                play.document = next;
                return vec![set_document_op(&play)];
            }
            "engagementInput" => {
                play.interaction.engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(&interaction.engagement_input)
                    .into();
                return vec![set_document_op(&play)];
            }
            "engagementSubmit" => {
                let value = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(&interaction.engagement_input)
                    .trim();
                if value.is_empty() || interaction.selected_ids.len() != 1 {
                    return Vec::new();
                }
                push_undo(&mut play);
                play.document = apply_draw_edit_op(
                    &play.document,
                    &DrawOp::SetLayerName {
                        layer_id: interaction.selected_ids[0].clone(),
                        name: value.into(),
                    },
                );
                return vec![set_document_op(&play)];
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                push_undo(&mut play);
                if example_id == "empty" || example_id.is_empty() {
                    play.document = default_draw_document("empty", None);
                } else if example_id == DRAW_PLAY_EXAMPLE_DEFAULT_ID {
                    play.document = serde_json::from_str(SEMIO_DRAW_EXAMPLE_JSON).unwrap_or_else(|_| empty_draw_projection());
                }
                play.interaction.selected_ids.clear();
                return vec![set_document_op(&play)];
            }
            "setFixtureJson" => {
                let json_text = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()).unwrap_or("");
                if json_text.contains(DRAW_DOCUMENT_SCHEMA) {
                    if let Ok(parsed) = serde_json::from_str(json_text) {
                        push_undo(&mut play);
                        play.document = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "addLayer" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("path");
                let layer = create_layer_by_kind(kind);
                let select_id = layer_id(&layer).to_string();
                push_undo(&mut play);
                play.document = apply_draw_edit_op(
                    &play.document,
                    &DrawOp::AddLayer {
                        parent_id: None,
                        index: Some(play.document.layers.len()),
                        layer,
                    },
                );
                play.interaction.selected_ids = vec![select_id.clone()];
                return vec![set_document_op(&play), json!({ "op": "selectLayer", "layerId": select_id }).to_string()];
            }
            "dropLayerKind" | "moveLayer" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                let layer_id_arg = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str());
                let target_row_id = args
                    .and_then(|value| value.get("targetRowId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("draw-play-layers");
                let drop_position = args
                    .and_then(|value| value.get("dropPosition"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("inside");
                if command == "dropLayerKind" {
                    if let Some(kind) = kind {
                        let layer = create_layer_by_kind(kind);
                        let select_id = layer_id(&layer).to_string();
                        let (parent_id, index) = resolve_reorder_target(&play.document, target_row_id, drop_position);
                        push_undo(&mut play);
                        play.document = apply_draw_edit_op(
                            &play.document,
                            &DrawOp::AddLayer { parent_id, index: Some(index), layer },
                        );
                        play.interaction.selected_ids = vec![select_id.clone()];
                        return vec![set_document_op(&play), json!({ "op": "selectLayer", "layerId": select_id }).to_string()];
                    }
                } else if let Some(layer_id) = layer_id_arg {
                    let (parent_id, index) = resolve_reorder_target(&play.document, target_row_id, drop_position);
                    push_undo(&mut play);
                    play.document = apply_draw_edit_op(
                        &play.document,
                        &DrawOp::ReorderLayer {
                            layer_id: layer_id.into(),
                            parent_id,
                            index,
                        },
                    );
                    return vec![set_document_op(&play)];
                }
            }
            "deleteLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                if !layer_id.is_empty() {
                    push_undo(&mut play);
                    play.document = apply_draw_edit_op(&play.document, &DrawOp::RemoveLayer { layer_id: layer_id.into() });
                    play.interaction.selected_ids.retain(|id| id != layer_id);
                    return vec![set_document_op(&play)];
                }
            }
            "duplicateLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                if !layer_id.is_empty() {
                    push_undo(&mut play);
                    play.document = apply_draw_edit_op(&play.document, &DrawOp::DuplicateLayer { layer_id: layer_id.into() });
                    return vec![set_document_op(&play)];
                }
            }
            "toggleLayerVisible" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(layer) = find_draw_layer(&play.document, layer_id) {
                    let visible = !layer_base(layer).visible;
                    push_undo(&mut play);
                    play.document = apply_draw_edit_op(
                        &play.document,
                        &DrawOp::SetLayerVisible {
                            layer_id: layer_id.into(),
                            visible,
                        },
                    );
                    return vec![set_document_op(&play)];
                }
            }
            "combineBoolean" => {
                let op = args.and_then(|value| value.get("op")).and_then(|value| value.as_str()).unwrap_or("union");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect::<Vec<_>>())
                    .filter(|values: &Vec<String>| !values.is_empty())
                    .unwrap_or_else(|| interaction.selected_ids.clone());
                if ids.len() >= 2 {
                    let layer = create_draw_boolean_layer("Boolean", op, ids);
                    let select_id = layer_id(&layer).to_string();
                    push_undo(&mut play);
                    play.document = apply_draw_edit_op(
                        &play.document,
                        &DrawOp::AddLayer {
                            parent_id: None,
                            index: Some(play.document.layers.len()),
                            layer,
                        },
                    );
                    play.interaction.selected_ids = vec![select_id.clone()];
                    return vec![set_document_op(&play), json!({ "op": "selectLayer", "layerId": select_id }).to_string()];
                }
            }
            "patchLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if !layer_id.is_empty() && !field.is_empty() {
                    push_undo(&mut play);
                    play.document = patch_layer_field(&play.document, layer_id, field, &value);
                    return vec![set_document_op(&play)];
                }
            }
            "patchLayers" => {
                let layer_ids: Vec<String> = args
                    .and_then(|value| value.get("layerIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if !field.is_empty() {
                    push_undo(&mut play);
                    for layer_id in layer_ids {
                        play.document = patch_layer_field(&play.document, &layer_id, field, &value);
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "commitDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<DrawDocument>(next.clone()) {
                        push_undo(&mut play);
                        play.document = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "canvasPointerDown" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                let viewport_w = args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()).unwrap_or(800.0);
                let viewport_h = args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()).unwrap_or(600.0);
                let extend = args.and_then(|value| value.get("extend")).and_then(|value| value.as_bool()).unwrap_or(false);
                if let (Some(x), Some(y)) = (x, y) {
                    let (world_x, world_y) = canvas_point_to_world(&play.document.camera, x, y, viewport_w, viewport_h);
                    let point = [world_x, world_y];
                    if play.document.active_tool.as_deref() == Some("pen") {
                        push_undo(&mut play);
                        let (next, path_id) = begin_pen_path(&play.document, point);
                        play.document = next;
                        play.interaction.drawing_path_id = Some(path_id.clone());
                        play.interaction.selected_ids = vec![path_id];
                        return vec![set_document_op(&play)];
                    }
                    if let Some(picked) = pick_layer_at(&play.document, world_x, world_y) {
                        if extend {
                            if play.interaction.selected_ids.iter().any(|id| id == &picked) {
                                play.interaction.selected_ids.retain(|id| id != &picked);
                            } else {
                                play.interaction.selected_ids.push(picked);
                            }
                        } else {
                            play.interaction.selected_ids = vec![picked];
                        }
                    } else if !extend {
                        play.interaction.selected_ids.clear();
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "canvasPointerMove" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                let viewport_w = args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()).unwrap_or(800.0);
                let viewport_h = args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()).unwrap_or(600.0);
                if let (Some(x), Some(y)) = (x, y) {
                    let (world_x, world_y) = canvas_point_to_world(&play.document.camera, x, y, viewport_w, viewport_h);
                    if play.document.active_tool.as_deref() == Some("pen") {
                        if let Some(path_id) = play.interaction.drawing_path_id.clone() {
                            play.document = append_pen_point(&play.document, &path_id, [world_x, world_y], false);
                            return vec![set_document_op(&play)];
                        }
                    }
                    play.interaction.hovered_id = pick_layer_at(&play.document, world_x, world_y);
                    return vec![set_document_op(&play)];
                }
            }
            "canvasPointerUp" => {
                if play.interaction.drawing_path_id.take().is_some() {
                    return vec![set_document_op(&play)];
                }
            }
            "canvasWheel" => {}
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let interaction = interaction_state(&play, view_state);
        match body_key {
            DRAW_PLAY_BODY_COMPOSITE => render_canvas(&play.document),
            DRAW_PLAY_BODY_LAYERS => render_layers_panel(&play.document, &interaction),
            DRAW_PLAY_BODY_CATALOGUE => render_catalogue_panel(&play.document, &interaction),
            DRAW_PLAY_BODY_PROPERTIES => render_properties_panel(&play.document, &interaction),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖DrawApp

//#region 🔖Canvas
fn render_canvas(document: &DrawDocument) -> UiNode {
    let scene_nodes = flatten_draw_document_to_scene_nodes(document);
    build_canvas_2d_scene(
        DRAW_PLAY_SURFACE_ID,
        DRAW_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x: document.camera.x,
            camera_y: document.camera.y,
            zoom: document.camera.zoom,
            layers_json: serde_json::to_string(&scene_nodes).unwrap_or_else(|_| "[]".into()),
        },
    )
}
//#endregion 🔖Canvas

//#region 🔖LayersPanel
fn layer_icon(layer: &draw::DrawLayerNode) -> &str {
    match layer {
        draw::DrawLayerNode::Group(_) => "folder",
        draw::DrawLayerNode::Boolean(_) => "combine",
        draw::DrawLayerNode::Trace(_) => "scan-line",
        draw::DrawLayerNode::Path(_) => "pen-tool",
        draw::DrawLayerNode::Shape(_) => "square",
        draw::DrawLayerNode::Text(_) => "type",
        draw::DrawLayerNode::Image(_) => "image",
    }
}

fn layer_tree_item(doc: &DrawDocument, layer: &draw::DrawLayerNode) -> UiTreeItemNode {
    let row_id = draw_play_layers_tree_row_id(layer);
    let base = layer_base(layer);
    let nested_items = match layer {
        draw::DrawLayerNode::Group(group) => Some(group.children.iter().map(|child| layer_tree_item(doc, child)).collect()),
        draw::DrawLayerNode::Boolean(boolean) => Some(
            boolean
                .children
                .iter()
                .map(|child_id| boolean_child_item(doc, &boolean.base.id, child_id))
                .collect(),
        ),
        _ => None,
    };
    let mut drag_data = HashMap::new();
    drag_data.insert("application/x-semio-draw-layer-id".into(), base.id.clone());
    UiTreeItemNode {
        id: row_id,
        label: base.name.clone(),
        description: Some(match layer {
            draw::DrawLayerNode::Boolean(boolean) => boolean.op.clone(),
            _ => base.blend_mode.clone(),
        }),
        icon_id: Some(layer_icon(layer).into()),
        selected: None,
        default_open: Some(matches!(layer, draw::DrawLayerNode::Group(_))),
        command: Some(draw_play_cmd("setSelection", Some(json!({ "ids": [base.id] })))),
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: Some(true),
        drag_data: Some(drag_data),
        items: nested_items,
        control: None,
        is_hidden: if base.visible { None } else { Some(true) },
    }
}

fn boolean_child_item(doc: &DrawDocument, boolean_id: &str, child_id: &str) -> UiTreeItemNode {
    let row_id = draw_play_boolean_child_row_id(boolean_id, child_id);
    if let Some(child) = find_draw_layer(doc, child_id) {
        return UiTreeItemNode {
            id: row_id,
            label: layer_base(child).name.clone(),
            description: Some(layer_kind_label(child)),
            icon_id: Some(layer_icon(child).into()),
            selected: None,
            default_open: None,
            command: Some(draw_play_cmd("setSelection", Some(json!({ "ids": [child_id] })))),
        hover_command: None,
        unhover_command: None,
        actions: None,
            draggable: Some(false),
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        };
    }
    UiTreeItemNode {
        id: row_id,
        label: format!("{child_id} (missing)"),
        description: None,
        icon_id: Some("alert-circle".into()),
        selected: None,
        default_open: None,
        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: Some(false),
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn render_layers_panel(document: &DrawDocument, interaction: &DrawInteractionState) -> UiNode {
    let toolbar_items = vec![
        tree_button("draw-play-layers.add.path", "Add Path", "pen-tool", "addLayer", json!({ "kind": "path" })),
        tree_button("draw-play-layers.add.rect", "Add Rectangle", "square", "addLayer", json!({ "kind": "shape:rect" })),
        tree_button("draw-play-layers.add.text", "Add Text", "type", "addLayer", json!({ "kind": "text" })),
        tree_button("draw-play-layers.add.group", "Add Group", "folder-plus", "addLayer", json!({ "kind": "group" })),
        tree_button("draw-play-layers.add.boolean", "Add Boolean", "combine", "addLayer", json!({ "kind": "boolean" })),
    ];
    let layer_items = if document.layers.is_empty() {
        vec![UiTreeItemNode {
            id: "draw-play-layers.empty".into(),
            label: "Drop layers here".into(),
            description: None,
            icon_id: Some("pen-tool".into()),
            selected: None,
            default_open: None,
            command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }]
    } else {
        document.layers.iter().map(|layer| layer_tree_item(document, layer)).collect()
    };
    let selected_tree_ids: Vec<String> = interaction
        .selected_ids
        .iter()
        .filter_map(|id| find_draw_layer(document, id).map(draw_play_layers_tree_row_id))
        .collect();
    let highlighted_ids: Vec<String> = interaction
        .hovered_id
        .as_ref()
        .and_then(|id| find_draw_layer(document, id).map(draw_play_layers_tree_row_id))
        .into_iter()
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "draw-play-layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: toolbar_items.into_iter().chain(layer_items).collect(),
        }],
        selected_ids: Some(selected_tree_ids),
        highlighted_ids: if highlighted_ids.is_empty() { None } else { Some(highlighted_ids) },
        selection_change: Some(draw_play_cmd("setSelection", None)),
    })
}

fn tree_button(id: &str, label: &str, icon: &str, command: &str, args: Value) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: Some(icon.into()),
        selected: None,
        default_open: None,
        command: Some(draw_play_cmd(command, Some(args))),
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}
//#endregion 🔖LayersPanel

//#region 🔖CataloguePanel
fn render_catalogue_panel(_document: &DrawDocument, interaction: &DrawInteractionState) -> UiNode {
    let catalogue_kinds = [
        ("path", "Path", "pen-tool"),
        ("shape:rect", "Rectangle", "square"),
        ("shape:ellipse", "Ellipse", "circle"),
        ("shape:line", "Line", "minus"),
        ("shape:polygon", "Polygon", "pentagon"),
        ("text", "Text", "type"),
        ("image", "Image", "image"),
        ("group", "Group", "folder"),
        ("boolean", "Boolean", "combine"),
        ("trace", "Trace", "scan-line"),
    ];
    let mut items: Vec<UiTreeItemNode> = catalogue_kinds
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(DRAW_LAYER_KIND_DRAG_MIME.into(), json!({ "kind": kind }).to_string());
            UiTreeItemNode {
                id: format!("draw-play-catalogue.{kind}"),
                label: label.into(),
                description: None,
                icon_id: Some(icon.into()),
                selected: None,
                default_open: None,
                command: None,
                hover_command: None,
                unhover_command: None,
                actions: None,
                draggable: Some(true),
                drag_data: Some(drag_data),
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    for op in DRAW_BOOLEAN_OPS {
        items.push(UiTreeItemNode {
            id: format!("draw-play-catalogue.bool.{op}"),
            label: format!("Boolean {op}"),
            description: None,
            icon_id: Some("combine".into()),
            selected: None,
            default_open: None,
            command: Some(draw_play_cmd(
                "combineBoolean",
                Some(json!({ "op": op, "ids": interaction.selected_ids })),
            )),
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        });
    }
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "draw-play-catalogue".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}
//#endregion 🔖CataloguePanel

//#region 🔖InspectorPanel
fn inspector_patch(layer_ids: &[String], field: &str) -> CommandDescriptor {
    draw_play_cmd("patchLayers", Some(json!({ "layerIds": layer_ids, "field": field })))
}

fn inspector_number_field(layer_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
            commit: None,
            on_change: inspector_patch(layer_ids, field),
        }),
    })
}

fn inspector_text_field(layer_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: None,
            on_change: inspector_patch(layer_ids, field),
        }),
    })
}

fn uniform_layers<'a>(layers: &[&'a draw::DrawLayerNode]) -> Option<Vec<&'a draw::DrawLayerNode>> {
    if layers.is_empty() {
        return None;
    }
    let kind = layer_kind_label(layers[0]);
    if layers.iter().all(|layer| layer_kind_label(layer) == kind) {
        Some(layers.to_vec())
    } else {
        None
    }
}

fn inspector_kind_group(doc: &DrawDocument, layers: &[&draw::DrawLayerNode]) -> Option<UiInspectorFieldGroup> {
    let uniform = uniform_layers(layers)?;
    let layer = uniform[0];
    let layer_ids: Vec<String> = uniform.iter().map(|entry| layer_id(entry).to_string()).collect();
    let mut fields: Vec<UiNode> = Vec::new();
    match layer {
        draw::DrawLayerNode::Boolean(boolean) => {
            let ops: Vec<String> = uniform
                .iter()
                .filter_map(|entry| match entry {
                    draw::DrawLayerNode::Boolean(entry) => Some(entry.op.clone()),
                    _ => None,
                })
                .collect();
            let op_mixed = ui_inspector_mixed_select(&ops);
            fields.push(UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.boolean-op".into(),
                label: "Boolean Op".into(),
                child: UiControlNode::Select(UiSelectNode {
                    id: "draw-play-inspector.boolean-op.select".into(),
                    value: op_mixed.value,
                    placeholder: op_mixed.placeholder,
                    items: DRAW_BOOLEAN_OPS.iter().map(|op| UiSelectItem { value: (*op).into(), label: (*op).into() }).collect(),
                    on_change: inspector_patch(&layer_ids, "booleanOp"),
                }),
            }));
            let child_labels = boolean
                .children
                .iter()
                .filter_map(|child_id| find_draw_layer(doc, child_id).map(|child| layer_base(child).name.clone()))
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.boolean-children",
                "Children",
                if child_labels.is_empty() { "—".into() } else { child_labels },
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.boolean".into(),
                label: "Boolean".into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Trace(trace) => {
            let thresholds: Vec<f64> = uniform
                .iter()
                .filter_map(|entry| match entry {
                    draw::DrawLayerNode::Trace(entry) => Some(entry.params.threshold),
                    _ => None,
                })
                .collect();
            let simplifies: Vec<f64> = uniform
                .iter()
                .filter_map(|entry| match entry {
                    draw::DrawLayerNode::Trace(entry) => Some(entry.params.simplify_epsilon),
                    _ => None,
                })
                .collect();
            let threshold_mixed = ui_inspector_mixed_slider(&thresholds);
            let simplify_mixed = ui_inspector_mixed_slider(&simplifies);
            fields.push(UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.trace-threshold".into(),
                label: "Trace Threshold".into(),
                child: UiControlNode::Slider(UiSliderNode {
                    id: "draw-play-inspector.trace-threshold.slider".into(),
                    value: if threshold_mixed.uniform { threshold_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "traceThreshold"),
                }),
            }));
            fields.push(UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.trace-simplify".into(),
                label: "Simplify".into(),
                child: UiControlNode::Slider(UiSliderNode {
                    id: "draw-play-inspector.trace-simplify.slider".into(),
                    value: if simplify_mixed.uniform { simplify_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    on_change: inspector_patch(&layer_ids, "traceSimplify"),
                }),
            }));
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.trace-source",
                "Source Key",
                trace.source_key.clone(),
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.trace".into(),
                label: "Trace".into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Shape(shape) if shape.shape_kind == "rect" => {
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-width",
                "Width",
                &uniform.iter().filter_map(|entry| match entry {
                    draw::DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.width),
                    _ => None,
                }).collect::<Vec<_>>(),
                "rectWidth",
            ));
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-height",
                "Height",
                &uniform.iter().filter_map(|entry| match entry {
                    draw::DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.height),
                    _ => None,
                }).collect::<Vec<_>>(),
                "rectHeight",
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.rect".into(),
                label: "Rectangle".into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Text(_) => {
            fields.push(inspector_text_field(
                &layer_ids,
                "draw-play-inspector.text-content",
                "Content",
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        draw::DrawLayerNode::Text(entry) => Some(entry.content.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "textContent",
            ));
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.text-size",
                "Size",
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        draw::DrawLayerNode::Text(entry) => Some(entry.size),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "textSize",
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.text".into(),
                label: "Text".into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Path(path) => {
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.path-segments",
                "Segment Count",
                path.segments.len().to_string(),
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.path".into(),
                label: "Path".into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Group(group) => {
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.group-children",
                "Children Count",
                group.children.len().to_string(),
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.group".into(),
                label: "Group".into(),
                default_open: None,
                fields,
            });
        }
        _ => {}
    }
    None
}

fn inspector_appearance_group(layers: &[&draw::DrawLayerNode]) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| layer_id(entry).to_string()).collect();
    let fill_colors: Vec<String> = layers
        .iter()
        .map(|entry| {
            layer_base(entry)
                .attributes
                .fill
                .as_ref()
                .and_then(|fill| match fill {
                    draw::FillStyle::Solid { color } => Some(rgba_to_hex(*color)),
                    draw::FillStyle::LinearGradient { .. } | draw::FillStyle::RadialGradient { .. } => Some("#000000".into()),
                })
                .unwrap_or_else(|| "#000000".into())
        })
        .collect();
    let fill_alphas: Vec<f64> = layers
        .iter()
        .map(|entry| {
            layer_base(entry)
                .attributes
                .fill
                .as_ref()
                .and_then(|fill| match fill {
                    draw::FillStyle::Solid { color } => Some(color[3]),
                    draw::FillStyle::LinearGradient { .. } | draw::FillStyle::RadialGradient { .. } => Some(1.0),
                })
                .unwrap_or(1.0)
        })
        .collect();
    let stroke_widths: Vec<f64> = layers
        .iter()
        .map(|entry| layer_base(entry).attributes.stroke.as_ref().map(|stroke| stroke.width).unwrap_or(1.0))
        .collect();
    let fill_alpha_mixed = ui_inspector_mixed_slider(&fill_alphas);
    UiInspectorFieldGroup {
        id: "draw-play-inspector.appearance".into(),
        label: "Appearance".into(),
        default_open: None,
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.fill", "Fill", &fill_colors, "fillColor"),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.fill-alpha".into(),
                label: "Fill Alpha".into(),
                child: UiControlNode::Slider(UiSliderNode {
                    id: "draw-play-inspector.fill-alpha.slider".into(),
                    value: if fill_alpha_mixed.uniform { fill_alpha_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "fillAlpha"),
                }),
            }),
            inspector_number_field(&layer_ids, "draw-play-inspector.stroke-width", "Stroke Width", &stroke_widths, "strokeWidth"),
        ],
    }
}

fn inspector_layer_group(layers: &[&draw::DrawLayerNode]) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| layer_id(entry).to_string()).collect();
    let names: Vec<String> = layers.iter().map(|entry| layer_base(entry).name.clone()).collect();
    let opacities: Vec<f64> = layers.iter().map(|entry| layer_base(entry).opacity).collect();
    let blend_modes: Vec<String> = layers.iter().map(|entry| layer_base(entry).blend_mode.clone()).collect();
    let visibles: Vec<bool> = layers.iter().map(|entry| layer_base(entry).visible).collect();
    let locked: Vec<bool> = layers.iter().map(|entry| layer_base(entry).locked).collect();
    let blend_mixed = ui_inspector_mixed_select(&blend_modes);
    let visible_mixed = ui_inspector_mixed_toggle(&visibles);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    UiInspectorFieldGroup {
        id: "draw-play-inspector.layer".into(),
        label: "Layer".into(),
        default_open: None,
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.name", "Name", &names, "name"),
            inspector_number_field(&layer_ids, "draw-play-inspector.opacity", "Opacity", &opacities, "opacity"),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.blend-mode".into(),
                label: "Blend Mode".into(),
                child: UiControlNode::Select(UiSelectNode {
                    id: "draw-play-inspector.blend-mode.select".into(),
                    value: blend_mixed.value,
                    placeholder: blend_mixed.placeholder,
                    items: DRAW_BLEND_MODES
                        .iter()
                        .map(|mode| UiSelectItem { value: (*mode).into(), label: (*mode).into() })
                        .collect(),
                    on_change: inspector_patch(&layer_ids, "blendMode"),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.visible".into(),
                label: "Visible".into(),
                child: UiControlNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.visible.toggle".into(),
                    icon_id: "eye".into(),
                    pressed: visible_mixed.uniform && visible_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&layer_ids, "visible"),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.locked".into(),
                label: "Locked".into(),
                child: UiControlNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.locked.toggle".into(),
                    icon_id: "lock".into(),
                    pressed: locked_mixed.uniform && locked_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&layer_ids, "locked"),
                }),
            }),
        ],
    }
}

fn inspector_orientation_group(layers: &[&draw::DrawLayerNode]) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| layer_id(entry).to_string()).collect();
    UiInspectorFieldGroup {
        id: "draw-play-inspector.orientation".into(),
        label: "Orientation".into(),
        default_open: None,
        fields: vec![
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-x",
                "Position X",
                &layers.iter().map(|entry| layer_base(entry).transform.x).collect::<Vec<_>>(),
                "transformX",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-y",
                "Position Y",
                &layers.iter().map(|entry| layer_base(entry).transform.y).collect::<Vec<_>>(),
                "transformY",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-scale-x",
                "Scale X",
                &layers.iter().map(|entry| layer_base(entry).transform.scale_x).collect::<Vec<_>>(),
                "transformScaleX",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-scale-y",
                "Scale Y",
                &layers.iter().map(|entry| layer_base(entry).transform.scale_y).collect::<Vec<_>>(),
                "transformScaleY",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-rotation",
                "Rotation",
                &layers.iter().map(|entry| layer_base(entry).transform.rotation).collect::<Vec<_>>(),
                "transformRotation",
            ),
        ],
    }
}

fn render_properties_panel(document: &DrawDocument, interaction: &DrawInteractionState) -> UiNode {
    let selected_layers: Vec<&draw::DrawLayerNode> = interaction
        .selected_ids
        .iter()
        .filter_map(|id| find_draw_layer(document, id))
        .collect();
    if selected_layers.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", DRAW_DOCUMENT_SCHEMA)),
            ui_text(format!("Tool: {}", document.active_tool.clone().unwrap_or_else(|| "selectDirect".into()))),
            ui_text(format!("Layers: {}", flatten_draw_layers(&document.layers).len())),
        ]);
    }
    let mut groups = Vec::new();
    if let Some(kind_group) = inspector_kind_group(document, &selected_layers) {
        groups.push(kind_group);
    }
    groups.push(inspector_orientation_group(&selected_layers));
    groups.push(inspector_appearance_group(&selected_layers));
    groups.push(inspector_layer_group(&selected_layers));
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖InspectorPanel

//#region 🔖Helpers
fn resolve_reorder_target(document: &DrawDocument, target_row_id: &str, drop_position: &str) -> (Option<String>, usize) {
    if target_row_id == "draw-play-layers" || target_row_id == "draw-play-layers.empty" {
        return (None, document.layers.len());
    }
    if let Some(layer_id) = draw_play_layer_id_from_tree_row_id(target_row_id) {
        if let Some(layer) = find_draw_layer(document, &layer_id) {
            if drop_position == "inside" {
                if let draw::DrawLayerNode::Group(group) = layer {
                    return (Some(group.base.id.clone()), group.children.len());
                }
            }
            if let Some(location) = find_draw_layer_location(document, &layer_id) {
                let index = if drop_position == "before" {
                    location.index
                } else {
                    location.index + 1
                };
                return (location.parent_id, index);
            }
        }
    }
    (None, document.layers.len())
}
//#endregion 🔖Helpers

//#region 🔖AppFactory
fn create_draw_app() -> App {
    let engagement = WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("draw-canvas-engagement".into()),
            value: Some(String::new()),
            placeholder: Some("Layer name".into()),
            on_change: Some(draw_play_cmd("engagementInput", None)),
            on_submit: Some(draw_play_cmd("engagementSubmit", None)),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "draw-layer-count".into(),
            text: "0 layers · 0 selected".into(),
        }]),
        possible_engagements: None,
    };
    App::from_builder(
        App::builder(DRAW_PLAY_APP_ID, "Draw").document(["semio", "draw"])
            .icon_id("draw")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement("draw-composite", "Canvas", DRAW_PLAY_BODY_COMPOSITE, engagement)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, DRAW_PLAY_BODY_LAYERS)
            .panel_tab("framework.panel.catalogue", "Catalogue", PanelGroup::Workbench, DRAW_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, DRAW_PLAY_BODY_PROPERTIES)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+a", "selectAll")
            .default_layout(create_default_layout(
                &["draw-composite".into()],
                "row",
                Some(&[100.0]),
                Some(&["Canvas".into()]),
            )),
    )
    .example("empty", "Empty", serde_json::to_string(&default_draw_document("empty", None)).unwrap())
    .example(
        DRAW_PLAY_EXAMPLE_DEFAULT_ID,
        "Semio",
        SEMIO_DRAW_EXAMPLE_JSON,
    )
    .program("draw", "Draw", "2d.drawing")
}

fn register_draw_exports() {
    semio_framework_os::register_2d_svg_png_export_handlers("2d.drawing", "draw", draw::draw_document_json_to_svg);
}

fn draw_bundle() -> semio_framework_plugin::PluginBundle {
    register_draw_exports();
    semio_framework_plugin::PluginBundle::new("draw", "Draw", "0.1.0").register_app(create_draw_app(), || Box::new(DrawApp))
}

static _PLUGIN_INIT: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
    semio_framework_plugin::install_plugin_bundle(draw_bundle());
});

semio_framework_plugin::plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use draw::{create_draw_shape_layer_rect, DrawLayerNode};
    use semio_framework_plugin::PluginApp;

    fn view_with_selection(ids: &[&str]) -> ViewState {
        ViewState {
            selection_json: Some(serde_json::to_string(&ids.iter().map(|id| id.to_string()).collect::<Vec<_>>()).unwrap()),
            ..Default::default()
        }
    }

    #[test]
    fn renders_canvas_scene_with_segments() {
        let app = DrawApp;
        let document = SEMIO_DRAW_EXAMPLE_JSON.to_string();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
        assert!(json.contains("segments") || json.contains("kernelPayload"));
    }

    #[test]
    fn layers_panel_lists_default_layer() {
        let app = DrawApp;
        let document = serde_json::to_string(&default_draw_document("test", None)).unwrap();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-layers.add.path"));
        assert!(json.contains("Layer 1"));
    }

    #[test]
    fn catalogue_panel_lists_boolean_ops() {
        let app = DrawApp;
        let document = serde_json::to_string(&empty_draw_projection()).unwrap();
        let node = app.render(DRAW_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-catalogue.path"));
        assert!(json.contains("Boolean union"));
    }

    #[test]
    fn add_layer_command_appends_path() {
        let mut app = DrawApp;
        let document = serde_json::to_string(&empty_draw_projection()).unwrap();
        let ops = app.handle_command_patch_ops("addLayer", Some(&json!({ "kind": "path" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 2);
        let next: DrawDocument = serde_json::from_str(&document).unwrap();
        let applied = apply_ops(&next, &ops);
        assert!(applied.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Path(_))));
    }

    #[test]
    fn patch_layers_opacity_updates_selection() {
        let mut app = DrawApp;
        let mut document = default_draw_document("patch", None);
        let layer_id = draw::layer_id(&document.layers[0]).to_string();
        let document_json = serde_json::to_string(&document).unwrap();
        let ops = app.handle_command_patch_ops(
            "patchLayers",
            Some(&json!({ "layerIds": [layer_id.clone()], "field": "opacity", "value": 0.5 })),
            &document_json,
            &view_with_selection(&[layer_id.as_str()]),
        );
        document = apply_ops(&document, &ops);
        assert!((layer_base(&document.layers[0]).opacity - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn inspector_renders_orientation_fields_for_selection() {
        let app = DrawApp;
        let document = default_draw_document("inspector", None);
        let layer_id = draw::layer_id(&document.layers[0]).to_string();
        let document_json = serde_json::to_string(&document).unwrap();
        let node = app.render(
            DRAW_PLAY_BODY_PROPERTIES,
            &document_json,
            &view_with_selection(&[layer_id.as_str()]),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Orientation"));
        assert!(json.contains("Position X"));
    }

    #[test]
    fn set_active_tool_updates_document() {
        let mut app = DrawApp;
        let document = serde_json::to_string(&empty_draw_projection()).unwrap();
        let ops = app.handle_command_patch_ops("setActiveTool", Some(&json!({ "tool": "pen" })), &document, &ViewState::default());
        let next: DrawDocument = apply_ops(&empty_draw_projection(), &ops);
        assert_eq!(next.active_tool.as_deref(), Some("pen"));
    }

    #[test]
    fn semio_example_fixture_parses() {
        let document: DrawDocument = serde_json::from_str(SEMIO_DRAW_EXAMPLE_JSON).expect("semio fixture");
        assert_eq!(document.id, "semio");
        assert_eq!(document.title.as_deref(), Some("Semio Emblem"));
        assert!(!document.layers.is_empty());
    }

    #[test]
    fn combine_boolean_requires_two_ids() {
        let mut app = DrawApp;
        let mut document = default_draw_document("bool", None);
        let second = create_draw_shape_layer_rect("Rect");
        let second_id = draw::layer_id(&second).to_string();
        document.layers.push(second);
        let first_id = draw::layer_id(&document.layers[0]).to_string();
        let document_json = serde_json::to_string(&document).unwrap();
        let ops = app.handle_command_patch_ops(
            "combineBoolean",
            Some(&json!({ "op": "union", "ids": [first_id, second_id] })),
            &document_json,
            &ViewState::default(),
        );
        let next = apply_ops(&document, &ops);
        assert!(next.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Boolean(_))));
    }

    fn apply_ops(document: &DrawDocument, ops: &[String]) -> DrawDocument {
        let mut play = DrawPlayEnvelope {
            document: document.clone(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            interaction: DrawInteractionState::default(),
        };
        for op_json in ops {
            if let Ok(op) = serde_json::from_str::<serde_json::Value>(op_json) {
                if op.get("op").and_then(|value| value.as_str()) == Some("setDocument") {
                    if let Some(document) = op.get("document") {
                        if let Ok(parsed) = serde_json::from_value::<DrawPlayEnvelope>(document.clone()) {
                            play = parsed;
                        } else if let Ok(parsed) = serde_json::from_value::<DrawDocument>(document.clone()) {
                            play.document = parsed;
                        }
                    }
                }
            }
        }
        play.document
    }
}
//#endregion 🧪Tests
