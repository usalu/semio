//! ✏️ Draw plugin — declarative draw app bundled as a hot-swappable WASM component.

use draw::{
    apply_draw_edit_op, create_draw_boolean_layer, create_draw_path_layer, create_draw_trace_layer, create_layer_by_kind,
    default_draw_document, default_layer_base, draw_layer_descendant_leaf_ids, draw_layer_world_bounds,
    draw_play_boolean_child_row_id, draw_play_layer_id_from_tree_row_id, draw_play_layers_tree_row_id, draw_transform_to_matrix,
    empty_draw_projection, find_draw_layer, find_draw_layer_location, flatten_draw_document_to_scene_nodes,
    flatten_draw_layers, layer_base, layer_id, layer_kind_label, layer_to_path_segments, patch_layer_field,
    rgba_to_hex, DrawDocument, DrawLayerNode, DrawOp, PathSegment, DRAW_BLEND_MODES, DRAW_BOOLEAN_OPS, DRAW_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{SurfaceKind,
    build_canvas_2d_scene, create_default_layout, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_select, ui_inspector_mixed_slider, ui_inspector_mixed_text, ui_inspector_mixed_toggle,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, tool_collection, tool_toggle, App, Canvas2dScene,
    ActionDescriptor, PanelGroup, ToolCategory, ToolNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiSelectItem,
    UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement,
    WindowEngagementInput,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum DrawDragState {
    Marquee {
        method: String,
        start: [f64; 2],
        cursor: [f64; 2],
        merge: String,
        active: bool,
    },
    Shape {
        tool: String,
        start: [f64; 2],
        cursor: [f64; 2],
    },
    Draft {
        tool: String,
        points: Vec<[f64; 2]>,
        cursor: [f64; 2],
    },
}

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
    drag: Option<DrawDragState>,
}

fn draw_play_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: DRAW_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
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
        || envelope.interaction.drag.is_some()
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
        (x - viewport_w * 0.5) / zoom + camera.x,
        (y - viewport_h * 0.5) / zoom + camera.y,
    )
}
//#endregion 🔖Interaction

//#region 🔖ToolStateMachine
const DRAW_MARQUEE_THRESHOLD_PX: f64 = 4.0;
const DRAW_PICK_TOLERANCE_PX: f64 = 8.0;

fn matrix_transform_point(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

/// 🎯 Maps shift/ctrl/meta modifiers to a `SelectionMergeMode` (matches `@semio-tech/ui-react`'s `marqueeModeFromModifiers`).
fn selection_merge_mode(shift: bool, ctrl: bool, meta: bool) -> &'static str {
    let ctrl_or_meta = ctrl || meta;
    if shift && ctrl_or_meta {
        "invertive"
    } else if shift {
        "additive"
    } else if ctrl_or_meta {
        "subtractive"
    } else {
        "default"
    }
}

fn merge_selection(mode: &str, current: &[String], incoming: &[String]) -> Vec<String> {
    match mode {
        "default" => {
            let mut out = Vec::new();
            for id in incoming {
                if !out.contains(id) {
                    out.push(id.clone());
                }
            }
            out
        }
        "additive" => {
            let mut out = current.to_vec();
            for id in incoming {
                if !out.contains(id) {
                    out.push(id.clone());
                }
            }
            out
        }
        "subtractive" => current.iter().filter(|id| !incoming.contains(id)).cloned().collect(),
        _ => {
            let mut out = current.to_vec();
            for id in incoming {
                if let Some(position) = out.iter().position(|existing| existing == id) {
                    out.remove(position);
                } else {
                    out.push(id.clone());
                }
            }
            out
        }
    }
}

struct DrawPickTarget {
    generality: i32,
    layer_id: String,
}

fn draw_pick_generality(layer: &DrawLayerNode) -> i32 {
    match layer {
        DrawLayerNode::Group(_) => 0,
        DrawLayerNode::Boolean(_) | DrawLayerNode::Trace(_) => 1,
        _ => 2,
    }
}

fn ancestor_group_ids(doc: &DrawDocument, target_id: &str) -> Vec<String> {
    fn walk(layers: &[DrawLayerNode], target_id: &str, ancestors: &mut Vec<String>) -> bool {
        for layer in layers {
            if layer_id(layer) == target_id {
                return true;
            }
            if let DrawLayerNode::Group(group) = layer {
                ancestors.push(group.base.id.clone());
                if walk(&group.children, target_id, ancestors) {
                    return true;
                }
                ancestors.pop();
            }
        }
        false
    }
    let mut ancestors = Vec::new();
    walk(&doc.layers, target_id, &mut ancestors);
    ancestors
}

fn segment_control_points(segment: &PathSegment) -> Vec<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } => vec![*to],
        PathSegment::Quad { ctrl, to } => vec![*ctrl, *to],
        PathSegment::Cubic { ctrl1, ctrl2, to } => vec![*ctrl1, *ctrl2, *to],
        PathSegment::Arc { to, .. } => vec![*to],
        PathSegment::Close => Vec::new(),
    }
}

/// 🎯 All pick targets under a world point (groups win by default, control points win over everything when enabled).
fn resolve_pick_targets_at(doc: &DrawDocument, world: [f64; 2], tolerance_world: f64, include_control_points: bool) -> Vec<DrawPickTarget> {
    let mut hits = Vec::new();
    for layer in flatten_draw_layers(&doc.layers).into_iter().rev() {
        let base = layer_base(layer);
        if !base.visible || base.locked {
            continue;
        }
        let Some((x, y, width, height)) = draw_layer_world_bounds(layer) else { continue };
        if !(world[0] >= x && world[0] <= x + width && world[1] >= y && world[1] <= y + height) {
            continue;
        }
        let id = layer_id(layer).to_string();
        hits.push(DrawPickTarget { generality: draw_pick_generality(layer), layer_id: id.clone() });
        if matches!(layer, DrawLayerNode::Group(_)) {
            continue;
        }
        for group_id in ancestor_group_ids(doc, &id) {
            if !hits.iter().any(|target| target.layer_id == group_id) {
                hits.push(DrawPickTarget { generality: 0, layer_id: group_id });
            }
        }
        if include_control_points && matches!(layer, DrawLayerNode::Path(_) | DrawLayerNode::Shape(_)) {
            let matrix = draw_transform_to_matrix(&base.transform);
            for segment in layer_to_path_segments(layer) {
                for local in segment_control_points(&segment) {
                    let world_point = matrix_transform_point(matrix, local);
                    let dx = world[0] - world_point[0];
                    let dy = world[1] - world_point[1];
                    if (dx * dx + dy * dy).sqrt() <= tolerance_world {
                        hits.push(DrawPickTarget { generality: 4, layer_id: id.clone() });
                    }
                }
            }
        }
    }
    hits
}

fn best_pick_layer_id(targets: &[DrawPickTarget]) -> Option<String> {
    targets.iter().max_by_key(|target| target.generality).map(|target| target.layer_id.clone())
}

fn apply_point_pick(play: &mut DrawPlayEnvelope, world: [f64; 2], shift: bool, ctrl: bool, meta: bool, include_control_points: bool) {
    let tolerance = DRAW_PICK_TOLERANCE_PX / play.document.camera.zoom.max(1e-6);
    let targets = resolve_pick_targets_at(&play.document, world, tolerance, include_control_points);
    let picked = best_pick_layer_id(&targets);
    let mode = selection_merge_mode(shift, ctrl, meta);
    play.interaction.selected_ids = match picked {
        Some(id) => merge_selection(mode, &play.interaction.selected_ids, &[id]),
        None if mode == "default" => Vec::new(),
        None => play.interaction.selected_ids.clone(),
    };
}

/// ⬚ Marquee/lasso layer hits — reduces the lasso gesture to its bounding box, matching the premigration behaviour.
fn marquee_layer_hits(doc: &DrawDocument, start: [f64; 2], end: [f64; 2], crossing: bool) -> Vec<String> {
    let rect_x = start[0].min(end[0]);
    let rect_y = start[1].min(end[1]);
    let rect_w = (end[0] - start[0]).abs();
    let rect_h = (end[1] - start[1]).abs();
    let mut hits = Vec::new();
    for layer in flatten_draw_layers(&doc.layers) {
        let base = layer_base(layer);
        if !base.visible || matches!(layer, DrawLayerNode::Group(_)) {
            continue;
        }
        let Some((x, y, width, height)) = draw_layer_world_bounds(layer) else { continue };
        let intersects = rect_x <= x + width && rect_x + rect_w >= x && rect_y <= y + height && rect_y + rect_h >= y;
        let contains = x >= rect_x && y >= rect_y && x + width <= rect_x + rect_w && y + height <= rect_y + rect_h;
        if if crossing { intersects } else { contains } {
            hits.push(layer_id(layer).to_string());
        }
    }
    hits
}

fn shape_preview_segments(tool: &str, start: [f64; 2], end: [f64; 2]) -> Vec<PathSegment> {
    if tool == "shapeLine" {
        return vec![PathSegment::Move { to: start }, PathSegment::Line { to: end }];
    }
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if tool == "shapeRect" {
        return vec![
            PathSegment::Move { to: [x, y] },
            PathSegment::Line { to: [x + width, y] },
            PathSegment::Line { to: [x + width, y + height] },
            PathSegment::Line { to: [x, y + height] },
            PathSegment::Close,
        ];
    }
    let cx = x + width / 2.0;
    let cy = y + height / 2.0;
    let rx = width / 2.0;
    let ry = height / 2.0;
    let k = 0.552_284_749_8;
    vec![
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + rx * k, cy - ry], ctrl2: [cx + rx, cy - ry * k], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + ry * k], ctrl2: [cx + rx * k, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - rx * k, cy + ry], ctrl2: [cx - rx, cy + ry * k], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - ry * k], ctrl2: [cx - rx * k, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ]
}

fn draft_preview_segments(tool: &str, points: &[[f64; 2]], cursor: [f64; 2]) -> Vec<PathSegment> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut segments = vec![PathSegment::Move { to: points[0] }];
    for point in points.iter().skip(1) {
        segments.push(PathSegment::Line { to: *point });
    }
    segments.push(PathSegment::Line { to: cursor });
    if tool == "shapePolygon" && points.len() > 1 {
        segments.push(PathSegment::Close);
    }
    segments
}

fn commit_shape_drag(play: &mut DrawPlayEnvelope, tool: &str, start: [f64; 2], end: [f64; 2]) -> Option<String> {
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if width < 1.0 && height < 1.0 {
        return None;
    }
    let layer = DrawLayerNode::Shape(draw::DrawShapeBody {
        base: default_layer_base(match tool {
            "shapeLine" => "Line",
            "shapeEllipse" => "Ellipse",
            _ => "Rectangle",
        }),
        shape_kind: match tool {
            "shapeLine" => "line",
            "shapeEllipse" => "ellipse",
            _ => "rect",
        }
        .into(),
        rect: if tool == "shapeRect" { Some(draw::DrawRect { x, y, width, height }) } else { None },
        ellipse: if tool == "shapeEllipse" {
            Some(draw::DrawEllipse { cx: x + width / 2.0, cy: y + height / 2.0, rx: width / 2.0, ry: height / 2.0 })
        } else {
            None
        },
        circle: None,
        line: if tool == "shapeLine" { Some(draw::DrawLine { x1: start[0], y1: start[1], x2: end[0], y2: end[1] }) } else { None },
        polygon: None,
    });
    let select_id = layer_id(&layer).to_string();
    push_undo(play);
    play.document = apply_draw_edit_op(&play.document, &DrawOp::AddLayer { parent_id: None, index: Some(play.document.layers.len()), layer });
    play.document = apply_draw_edit_op(&play.document, &DrawOp::SetActiveTool { tool: "selectDirect".into() });
    play.interaction.selected_ids = vec![select_id.clone()];
    Some(select_id)
}

fn commit_draft(play: &mut DrawPlayEnvelope, tool: &str, points: &[[f64; 2]]) -> Option<String> {
    if points.len() < 2 {
        return None;
    }
    let layer = if tool == "pen" {
        let mut segments = vec![PathSegment::Move { to: points[0] }];
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
        create_draw_path_layer("Path", segments)
    } else {
        DrawLayerNode::Shape(draw::DrawShapeBody {
            base: default_layer_base("Polygon"),
            shape_kind: "polygon".into(),
            rect: None,
            ellipse: None,
            circle: None,
            line: None,
            polygon: Some(draw::DrawPolygon { points: points.to_vec() }),
        })
    };
    let select_id = layer_id(&layer).to_string();
    push_undo(play);
    play.document = apply_draw_edit_op(&play.document, &DrawOp::AddLayer { parent_id: None, index: Some(play.document.layers.len()), layer });
    play.document = apply_draw_edit_op(&play.document, &DrawOp::SetActiveTool { tool: "selectDirect".into() });
    play.interaction.selected_ids = vec![select_id.clone()];
    Some(select_id)
}

fn commit_trace_at(play: &mut DrawPlayEnvelope, world: [f64; 2]) -> Option<String> {
    let tolerance = DRAW_PICK_TOLERANCE_PX / play.document.camera.zoom.max(1e-6);
    let hit_layer_id = best_pick_layer_id(&resolve_pick_targets_at(&play.document, world, tolerance, false));
    let source_key = hit_layer_id
        .and_then(|id| find_draw_layer(&play.document, &id).cloned())
        .and_then(|layer| match layer {
            DrawLayerNode::Image(image) => Some(image.image_key),
            _ => None,
        })
        .or_else(|| play.document.assets.as_ref().and_then(|assets| assets.keys().next().cloned()));
    let source_key = source_key?;
    let layer = create_draw_trace_layer("Trace", &source_key);
    let select_id = layer_id(&layer).to_string();
    push_undo(play);
    play.document = apply_draw_edit_op(&play.document, &DrawOp::AddLayer { parent_id: None, index: Some(play.document.layers.len()), layer });
    play.document = apply_draw_edit_op(&play.document, &DrawOp::SetActiveTool { tool: "selectDirect".into() });
    play.interaction.selected_ids = vec![select_id.clone()];
    Some(select_id)
}
//#endregion 🔖ToolStateMachine

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the draw app; one field per label makes every locale combination compile-checked.
struct DrawLabels {
    add_path: &'static str,
    add_rectangle: &'static str,
    add_text: &'static str,
    add_group: &'static str,
    add_boolean: &'static str,
    empty_state: &'static str,
    kind_path: &'static str,
    kind_rectangle: &'static str,
    kind_ellipse: &'static str,
    kind_line: &'static str,
    kind_polygon: &'static str,
    kind_text: &'static str,
    kind_image: &'static str,
    kind_group: &'static str,
    kind_boolean: &'static str,
    kind_trace: &'static str,
    boolean_op: &'static str,
    children: &'static str,
    trace_threshold: &'static str,
    simplify: &'static str,
    source_key: &'static str,
    width: &'static str,
    height: &'static str,
    content: &'static str,
    size: &'static str,
    segment_count: &'static str,
    children_count: &'static str,
    appearance: &'static str,
    fill: &'static str,
    fill_alpha: &'static str,
    stroke_width: &'static str,
    layer: &'static str,
    name: &'static str,
    opacity: &'static str,
    blend_mode: &'static str,
    visible: &'static str,
    locked: &'static str,
    orientation: &'static str,
    position_x: &'static str,
    position_y: &'static str,
    scale_x: &'static str,
    scale_y: &'static str,
    rotation: &'static str,
    tool_group_select: &'static str,
    tool_group_draw: &'static str,
    tool_group_combine: &'static str,
    tool_group_view: &'static str,
    tool_marquee_select: &'static str,
    tool_lasso_select: &'static str,
    tool_direct_select: &'static str,
    tool_pen: &'static str,
    tool_pan: &'static str,
}

const DRAW_LABELS_NATIVE_EN: DrawLabels = DrawLabels {
    add_path: "Add Path",
    add_rectangle: "Add Rectangle",
    add_text: "Add Text",
    add_group: "Add Group",
    add_boolean: "Add Boolean",
    empty_state: "Drop layers here",
    kind_path: "Path",
    kind_rectangle: "Rectangle",
    kind_ellipse: "Ellipse",
    kind_line: "Line",
    kind_polygon: "Polygon",
    kind_text: "Text",
    kind_image: "Image",
    kind_group: "Group",
    kind_boolean: "Boolean",
    kind_trace: "Trace",
    boolean_op: "Boolean Op",
    children: "Children",
    trace_threshold: "Trace Threshold",
    simplify: "Simplify",
    source_key: "Source Key",
    width: "Width",
    height: "Height",
    content: "Content",
    size: "Size",
    segment_count: "Segment Count",
    children_count: "Children Count",
    appearance: "Appearance",
    fill: "Fill",
    fill_alpha: "Fill Alpha",
    stroke_width: "Stroke Width",
    layer: "Layer",
    name: "Name",
    opacity: "Opacity",
    blend_mode: "Blend Mode",
    visible: "Visible",
    locked: "Locked",
    orientation: "Orientation",
    position_x: "Position X",
    position_y: "Position Y",
    scale_x: "Scale X",
    scale_y: "Scale Y",
    rotation: "Rotation",
    tool_group_select: "Select",
    tool_group_draw: "Draw",
    tool_group_combine: "Combine",
    tool_group_view: "View",
    tool_marquee_select: "Marquee Select",
    tool_lasso_select: "Lasso Select",
    tool_direct_select: "Direct Select",
    tool_pen: "Pen",
    tool_pan: "Pan",
};

const DRAW_LABELS_NATIVE_DE: DrawLabels = DrawLabels {
    add_path: "Pfad hinzufügen",
    add_rectangle: "Rechteck hinzufügen",
    add_text: "Text hinzufügen",
    add_group: "Gruppe hinzufügen",
    add_boolean: "Boolean hinzufügen",
    empty_state: "Ebenen hier ablegen",
    kind_path: "Pfad",
    kind_rectangle: "Rechteck",
    kind_ellipse: "Ellipse",
    kind_line: "Linie",
    kind_polygon: "Polygon",
    kind_text: "Text",
    kind_image: "Bild",
    kind_group: "Gruppe",
    kind_boolean: "Boolean",
    kind_trace: "Nachzeichnung",
    boolean_op: "Boolean-Operation",
    children: "Kinder",
    trace_threshold: "Trace-Schwellenwert",
    simplify: "Vereinfachen",
    source_key: "Quellschlüssel",
    width: "Breite",
    height: "Höhe",
    content: "Inhalt",
    size: "Größe",
    segment_count: "Segmentanzahl",
    children_count: "Kinderanzahl",
    appearance: "Erscheinungsbild",
    fill: "Füllung",
    fill_alpha: "Füllung Alpha",
    stroke_width: "Strichstärke",
    layer: "Ebene",
    name: "Name",
    opacity: "Deckkraft",
    blend_mode: "Mischmodus",
    visible: "Sichtbar",
    locked: "Gesperrt",
    orientation: "Ausrichtung",
    position_x: "Position X",
    position_y: "Position Y",
    scale_x: "Skalierung X",
    scale_y: "Skalierung Y",
    rotation: "Rotation",
    tool_group_select: "Auswahl",
    tool_group_draw: "Zeichnen",
    tool_group_combine: "Kombinieren",
    tool_group_view: "Ansicht",
    tool_marquee_select: "Rahmenauswahl",
    tool_lasso_select: "Lasso-Auswahl",
    tool_direct_select: "Direktauswahl",
    tool_pen: "Stift",
    tool_pan: "Verschieben",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
fn draw_labels(view_state: &ViewState) -> &'static DrawLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &DRAW_LABELS_NATIVE_DE } else { &DRAW_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖DrawApp
#[derive(Default)]
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

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let interaction = interaction_state(&play, view_state);
        match action {
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
                        play.document = apply_draw_edit_op(&play.document, &DrawOp::SetCamera { camera });
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setCameraZoom" => {
                let zoom = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let mut camera = play.document.camera.clone();
                camera.zoom = zoom;
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
                if action == "dropLayerKind" {
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
                let shift = args.and_then(|value| value.get("shift")).and_then(|value| value.as_bool()).unwrap_or(false);
                let ctrl = args.and_then(|value| value.get("ctrl")).and_then(|value| value.as_bool()).unwrap_or(false);
                let meta = args.and_then(|value| value.get("meta")).and_then(|value| value.as_bool()).unwrap_or(false);
                let (Some(x), Some(y)) = (x, y) else { return Vec::new() };
                let (world_x, world_y) = canvas_point_to_world(&play.document.camera, x, y, viewport_w, viewport_h);
                let world = [world_x, world_y];
                let tool = play.document.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
                match tool.as_str() {
                    "selectMarquee" | "selectLasso" => {
                        play.interaction.drag = Some(DrawDragState::Marquee {
                            method: if tool == "selectLasso" { "lasso".into() } else { "rectangle".into() },
                            start: world,
                            cursor: world,
                            merge: selection_merge_mode(shift, ctrl, meta).into(),
                            active: false,
                        });
                        return vec![set_document_op(&play)];
                    }
                    "shapeRect" | "shapeEllipse" | "shapeLine" => {
                        play.interaction.drag = Some(DrawDragState::Shape { tool: tool.clone(), start: world, cursor: world });
                        return vec![set_document_op(&play)];
                    }
                    "pen" | "shapePolygon" => {
                        let matches_existing = matches!(&play.interaction.drag, Some(DrawDragState::Draft { tool: existing, .. }) if existing == &tool);
                        if matches_existing {
                            if let Some(DrawDragState::Draft { points, cursor, .. }) = &mut play.interaction.drag {
                                points.push(world);
                                *cursor = world;
                            }
                        } else {
                            play.interaction.drag = Some(DrawDragState::Draft { tool: tool.clone(), points: vec![world], cursor: world });
                        }
                        return vec![set_document_op(&play)];
                    }
                    "trace" => {
                        if let Some(select_id) = commit_trace_at(&mut play, world) {
                            return vec![set_document_op(&play), json!({ "op": "selectLayer", "layerId": select_id }).to_string()];
                        }
                        return Vec::new();
                    }
                    _ => {}
                }
            }
            "canvasPointerMove" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                let viewport_w = args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()).unwrap_or(800.0);
                let viewport_h = args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()).unwrap_or(600.0);
                let (Some(x), Some(y)) = (x, y) else { return Vec::new() };
                let (world_x, world_y) = canvas_point_to_world(&play.document.camera, x, y, viewport_w, viewport_h);
                let world = [world_x, world_y];
                if let Some(drag) = &mut play.interaction.drag {
                    match drag {
                        DrawDragState::Marquee { start, cursor, active, .. } => {
                            let distance = ((world[0] - start[0]).powi(2) + (world[1] - start[1]).powi(2)).sqrt();
                            let threshold_world = DRAW_MARQUEE_THRESHOLD_PX / play.document.camera.zoom.max(1e-6);
                            *active = *active || distance >= threshold_world;
                            *cursor = world;
                        }
                        DrawDragState::Shape { cursor, .. } | DrawDragState::Draft { cursor, .. } => {
                            *cursor = world;
                        }
                    }
                    return vec![set_document_op(&play)];
                }
                let tool = play.document.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
                let include_control_points = tool == "selectDirect";
                let tolerance = DRAW_PICK_TOLERANCE_PX / play.document.camera.zoom.max(1e-6);
                let next_hover = best_pick_layer_id(&resolve_pick_targets_at(&play.document, world, tolerance, include_control_points));
                if next_hover == play.interaction.hovered_id {
                    return Vec::new();
                }
                play.interaction.hovered_id = next_hover;
                return vec![set_document_op(&play)];
            }
            "canvasPointerUp" => {
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                let viewport_w = args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()).unwrap_or(800.0);
                let viewport_h = args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()).unwrap_or(600.0);
                let shift = args.and_then(|value| value.get("shift")).and_then(|value| value.as_bool()).unwrap_or(false);
                let ctrl = args.and_then(|value| value.get("ctrl")).and_then(|value| value.as_bool()).unwrap_or(false);
                let meta = args.and_then(|value| value.get("meta")).and_then(|value| value.as_bool()).unwrap_or(false);
                let (Some(x), Some(y)) = (x, y) else { return Vec::new() };
                let (world_x, world_y) = canvas_point_to_world(&play.document.camera, x, y, viewport_w, viewport_h);
                let world = [world_x, world_y];
                match play.interaction.drag.clone() {
                    Some(DrawDragState::Draft { .. }) => {
                        return Vec::new();
                    }
                    Some(DrawDragState::Marquee { start, merge, active, .. }) => {
                        play.interaction.drag = None;
                        if active {
                            let crossing = world[0] < start[0];
                            let hits = marquee_layer_hits(&play.document, start, world, crossing);
                            play.interaction.selected_ids = merge_selection(&merge, &play.interaction.selected_ids, &hits);
                        } else {
                            apply_point_pick(&mut play, world, shift, ctrl, meta, false);
                        }
                        return vec![set_document_op(&play)];
                    }
                    Some(DrawDragState::Shape { tool, start, .. }) => {
                        play.interaction.drag = None;
                        if let Some(select_id) = commit_shape_drag(&mut play, &tool, start, world) {
                            return vec![set_document_op(&play), json!({ "op": "selectLayer", "layerId": select_id }).to_string()];
                        }
                        return vec![set_document_op(&play)];
                    }
                    None => {
                        let tool = play.document.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
                        if tool == "selectDirect" {
                            apply_point_pick(&mut play, world, shift, ctrl, meta, true);
                            return vec![set_document_op(&play)];
                        }
                    }
                }
            }
            "canvasDoubleClick" => {
                if let Some(DrawDragState::Draft { tool, points, .. }) = play.interaction.drag.clone() {
                    play.interaction.drag = None;
                    if let Some(select_id) = commit_draft(&mut play, &tool, &points) {
                        return vec![set_document_op(&play), json!({ "op": "selectLayer", "layerId": select_id }).to_string()];
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "canvasEscape" => {
                if play.interaction.drag.is_some() {
                    play.interaction.drag = None;
                    return vec![set_document_op(&play)];
                }
            }
            "canvasCommitDraft" => {
                if let Some(DrawDragState::Draft { tool, points, .. }) = play.interaction.drag.clone() {
                    play.interaction.drag = None;
                    if let Some(select_id) = commit_draft(&mut play, &tool, &points) {
                        return vec![set_document_op(&play), json!({ "op": "selectLayer", "layerId": select_id }).to_string()];
                    }
                    return vec![set_document_op(&play)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let interaction = interaction_state(&play, view_state);
        let labels = draw_labels(view_state);
        match body_key {
            DRAW_PLAY_BODY_COMPOSITE => render_canvas(&play.document, &interaction),
            DRAW_PLAY_BODY_LAYERS => render_layers_panel(&play.document, &interaction, labels),
            DRAW_PLAY_BODY_CATALOGUE => render_catalogue_panel(&play.document, &interaction, labels),
            DRAW_PLAY_BODY_PROPERTIES => render_properties_panel(&play.document, &interaction, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn tools(&self, document_json: &str, view_state: &ViewState) -> Vec<ToolNode> {
        let play = parse_envelope(document_json);
        let active = play.document.active_tool.clone().unwrap_or_else(|| "selectDirect".into());
        let labels = draw_labels(view_state);
        let toggle = |id: &str, icon: &str, label: &str| {
            tool_toggle(
                format!("draw-play-tools.{id}"),
                icon,
                label,
                active == id,
                draw_play_action("setActiveTool", Some(json!({ "tool": id }))),
            )
        };
        vec![
            tool_collection(
                "draw-play-tools-select",
                "mouse-pointer-2",
                labels.tool_group_select,
                vec![
                    toggle("selectMarquee", "square-dashed", labels.tool_marquee_select),
                    toggle("selectLasso", "lasso", labels.tool_lasso_select),
                    toggle("selectDirect", "mouse-pointer-2", labels.tool_direct_select),
                ],
            )
            .with_category(ToolCategory::Selection),
            tool_collection(
                "draw-play-tools-draw",
                "pen-tool",
                labels.tool_group_draw,
                vec![
                    toggle("pen", "pen-tool", labels.tool_pen),
                    toggle("shapeRect", "square", labels.kind_rectangle),
                    toggle("shapeEllipse", "circle", labels.kind_ellipse),
                    toggle("shapeLine", "minus", labels.kind_line),
                    toggle("shapePolygon", "pentagon", labels.kind_polygon),
                ],
            )
            .with_category(ToolCategory::Tools),
            tool_collection(
                "draw-play-tools-combine",
                "combine",
                labels.tool_group_combine,
                vec![toggle("booleanCombine", "combine", labels.kind_boolean), toggle("trace", "scan-line", labels.kind_trace)],
            )
            .with_category(ToolCategory::Tools),
            tool_collection("draw-play-tools-view", "move", labels.tool_group_view, vec![toggle("transformMove", "move", labels.tool_pan)]).with_category(ToolCategory::Tools),
        ]
    }
}
//#endregion 🔖DrawApp

//#region 🔖Canvas
fn overlay_record(id: String, transform: [f64; 6], segments: &[PathSegment], fill: Option<[f64; 4]>, stroke_color: [f64; 4], stroke_width: f64) -> Value {
    json!({
        "id": id,
        "role": "overlay",
        "transform": transform,
        "segments": segments,
        "fill": fill.map(|color| json!({ "kind": "solid", "color": color })),
        "stroke": { "color": stroke_color, "width": stroke_width, "cap": "round", "join": "round" },
        "opacity": 1.0,
        "blendMode": "normal",
        "visible": true,
        "fillRule": "evenodd",
    })
}

const DRAW_OVERLAY_SELECTION_STROKE: [f64; 4] = [0.98, 0.75, 0.14, 0.95];
const DRAW_OVERLAY_SELECTION_FILL: [f64; 4] = [0.98, 0.75, 0.14, 0.16];
const DRAW_OVERLAY_HOVER_STROKE: [f64; 4] = [0.56, 0.78, 0.98, 0.9];
const DRAW_OVERLAY_MARQUEE_STROKE: [f64; 4] = [0.36, 0.65, 0.98, 0.9];
const DRAW_OVERLAY_MARQUEE_FILL: [f64; 4] = [0.36, 0.65, 0.98, 0.12];

fn render_canvas(document: &DrawDocument, interaction: &DrawInteractionState) -> UiNode {
    let scene_nodes = flatten_draw_document_to_scene_nodes(document);
    let mut records: Vec<Value> = Vec::with_capacity(scene_nodes.len() + 4);
    records.push(json!({
        "id": "meta:tool",
        "role": "meta",
        "tool": document.active_tool.clone().unwrap_or_else(|| "selectDirect".into()),
    }));
    for node in &scene_nodes {
        records.push(serde_json::to_value(node).unwrap_or(Value::Null));
    }
    let node_by_id: HashMap<&str, &draw::DrawSceneNode> = scene_nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let selected_leaf_ids: Vec<String> = interaction
        .selected_ids
        .iter()
        .filter_map(|id| find_draw_layer(document, id))
        .flat_map(draw_layer_descendant_leaf_ids)
        .collect();
    for leaf_id in &selected_leaf_ids {
        if let Some(node) = node_by_id.get(leaf_id.as_str()) {
            records.push(overlay_record(
                format!("overlay:sel:{leaf_id}"),
                node.transform,
                &node.segments,
                Some(DRAW_OVERLAY_SELECTION_FILL),
                DRAW_OVERLAY_SELECTION_STROKE,
                2.0,
            ));
        }
    }
    if let Some(hovered_id) = &interaction.hovered_id {
        if !selected_leaf_ids.iter().any(|id| id == hovered_id) {
            if let Some(layer) = find_draw_layer(document, hovered_id) {
                for leaf_id in draw_layer_descendant_leaf_ids(layer) {
                    if let Some(node) = node_by_id.get(leaf_id.as_str()) {
                        records.push(overlay_record(
                            format!("overlay:hover:{leaf_id}"),
                            node.transform,
                            &node.segments,
                            None,
                            DRAW_OVERLAY_HOVER_STROKE,
                            1.5,
                        ));
                    }
                }
            }
        }
    }
    if let Some(drag) = &interaction.drag {
        match drag {
            DrawDragState::Marquee { start, cursor, .. } => {
                let x = start[0].min(cursor[0]);
                let y = start[1].min(cursor[1]);
                let width = (cursor[0] - start[0]).abs();
                let height = (cursor[1] - start[1]).abs();
                let segments = vec![
                    PathSegment::Move { to: [x, y] },
                    PathSegment::Line { to: [x + width, y] },
                    PathSegment::Line { to: [x + width, y + height] },
                    PathSegment::Line { to: [x, y + height] },
                    PathSegment::Close,
                ];
                records.push(overlay_record(
                    "overlay:marquee".into(),
                    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                    &segments,
                    Some(DRAW_OVERLAY_MARQUEE_FILL),
                    DRAW_OVERLAY_MARQUEE_STROKE,
                    1.0,
                ));
            }
            DrawDragState::Shape { tool, start, cursor } => {
                let segments = shape_preview_segments(tool, *start, *cursor);
                records.push(overlay_record(
                    "overlay:preview".into(),
                    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                    &segments,
                    Some(DRAW_OVERLAY_SELECTION_FILL),
                    DRAW_OVERLAY_SELECTION_STROKE,
                    1.5,
                ));
            }
            DrawDragState::Draft { tool, points, cursor } => {
                let segments = draft_preview_segments(tool, points, *cursor);
                records.push(overlay_record(
                    "overlay:preview".into(),
                    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                    &segments,
                    Some(DRAW_OVERLAY_SELECTION_FILL),
                    DRAW_OVERLAY_SELECTION_STROKE,
                    1.5,
                ));
            }
        }
    }
    build_canvas_2d_scene(
        DRAW_PLAY_SURFACE_ID,
        DRAW_PLAY_CONTROLLER_ID,
        Canvas2dScene {
            camera_x: document.camera.x,
            camera_y: document.camera.y,
            zoom: document.camera.zoom,
            layers_json: serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()),
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
        action: Some(draw_play_action("setSelection", Some(json!({ "ids": [base.id] })))),
        hover_action: None,
        unhover_action: None,
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
            action: Some(draw_play_action("setSelection", Some(json!({ "ids": [child_id] })))),
        hover_action: None,
        unhover_action: None,
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
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: Some(false),
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn render_layers_panel(document: &DrawDocument, interaction: &DrawInteractionState, labels: &DrawLabels) -> UiNode {
    let toolbar_items = vec![
        tree_button("draw-play-layers.add.path", labels.add_path, "pen-tool", "addLayer", json!({ "kind": "path" })),
        tree_button("draw-play-layers.add.rect", labels.add_rectangle, "square", "addLayer", json!({ "kind": "shape:rect" })),
        tree_button("draw-play-layers.add.text", labels.add_text, "type", "addLayer", json!({ "kind": "text" })),
        tree_button("draw-play-layers.add.group", labels.add_group, "folder-plus", "addLayer", json!({ "kind": "group" })),
        tree_button("draw-play-layers.add.boolean", labels.add_boolean, "combine", "addLayer", json!({ "kind": "boolean" })),
    ];
    let layer_items = if document.layers.is_empty() {
        vec![UiTreeItemNode {
            id: "draw-play-layers.empty".into(),
            label: labels.empty_state.into(),
            description: None,
            icon_id: Some("pen-tool".into()),
            selected: None,
            default_open: None,
            action: None,
        hover_action: None,
        unhover_action: None,
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
        selection_change: Some(draw_play_action("setSelection", None)),
        drop_action: None,
    })
}

fn tree_button(id: &str, label: &str, icon: &str, action: &str, args: Value) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: Some(icon.into()),
        selected: None,
        default_open: None,
        action: Some(draw_play_action(action, Some(args))),
        hover_action: None,
        unhover_action: None,
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
fn render_catalogue_panel(_document: &DrawDocument, interaction: &DrawInteractionState, labels: &DrawLabels) -> UiNode {
    let catalogue_kinds = [
        ("path", labels.kind_path, "pen-tool"),
        ("shape:rect", labels.kind_rectangle, "square"),
        ("shape:ellipse", labels.kind_ellipse, "circle"),
        ("shape:line", labels.kind_line, "minus"),
        ("shape:polygon", labels.kind_polygon, "pentagon"),
        ("text", labels.kind_text, "type"),
        ("image", labels.kind_image, "image"),
        ("group", labels.kind_group, "folder"),
        ("boolean", labels.kind_boolean, "combine"),
        ("trace", labels.kind_trace, "scan-line"),
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
                action: None,
                hover_action: None,
                unhover_action: None,
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
            label: format!("{} {op}", labels.kind_boolean),
            description: None,
            icon_id: Some("combine".into()),
            selected: None,
            default_open: None,
            action: Some(draw_play_action(
                "combineBoolean",
                Some(json!({ "op": op, "ids": interaction.selected_ids })),
            )),
            hover_action: None,
            unhover_action: None,
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
        drop_action: None,
    })
}
//#endregion 🔖CataloguePanel

//#region 🔖InspectorPanel
fn inspector_patch(layer_ids: &[String], field: &str) -> ActionDescriptor {
    draw_play_action("patchLayers", Some(json!({ "layerIds": layer_ids, "field": field })))
}

fn inspector_number_field(layer_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
            commit: None,
            on_change: inspector_patch(layer_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_text_field(layer_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: None,
            on_change: inspector_patch(layer_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
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

fn inspector_kind_group(doc: &DrawDocument, layers: &[&draw::DrawLayerNode], labels: &DrawLabels) -> Option<UiInspectorFieldGroup> {
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
                label: labels.boolean_op.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "draw-play-inspector.boolean-op.select".into(),
                    value: op_mixed.value,
                    placeholder: op_mixed.placeholder,
                    items: DRAW_BOOLEAN_OPS.iter().map(|op| UiSelectItem { value: (*op).into(), label: (*op).into() }).collect(),
                    on_change: inspector_patch(&layer_ids, "booleanOp"),
                })),
                description: None,
                required: None,
                error: None,
            }));
            let child_labels = boolean
                .children
                .iter()
                .filter_map(|child_id| find_draw_layer(doc, child_id).map(|child| layer_base(child).name.clone()))
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.boolean-children",
                labels.children,
                if child_labels.is_empty() { "—".into() } else { child_labels },
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.boolean".into(),
                label: labels.kind_boolean.into(),
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
                label: labels.trace_threshold.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    id: "draw-play-inspector.trace-threshold.slider".into(),
                    value: if threshold_mixed.uniform { threshold_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "traceThreshold"),
                    unit: None,
                })),
                description: None,
                required: None,
                error: None,
            }));
            fields.push(UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.trace-simplify".into(),
                label: labels.simplify.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    id: "draw-play-inspector.trace-simplify.slider".into(),
                    value: if simplify_mixed.uniform { simplify_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    on_change: inspector_patch(&layer_ids, "traceSimplify"),
                    unit: None,
                })),
                description: None,
                required: None,
                error: None,
            }));
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.trace-source",
                labels.source_key,
                trace.source_key.clone(),
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.trace".into(),
                label: labels.kind_trace.into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Shape(shape) if shape.shape_kind == "rect" => {
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-width",
                labels.width,
                &uniform.iter().filter_map(|entry| match entry {
                    draw::DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.width),
                    _ => None,
                }).collect::<Vec<_>>(),
                "rectWidth",
            ));
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-height",
                labels.height,
                &uniform.iter().filter_map(|entry| match entry {
                    draw::DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.height),
                    _ => None,
                }).collect::<Vec<_>>(),
                "rectHeight",
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.rect".into(),
                label: labels.kind_rectangle.into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Text(_) => {
            fields.push(inspector_text_field(
                &layer_ids,
                "draw-play-inspector.text-content",
                labels.content,
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
                labels.size,
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
                label: labels.kind_text.into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Path(path) => {
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.path-segments",
                labels.segment_count,
                path.segments.len().to_string(),
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.path".into(),
                label: labels.kind_path.into(),
                default_open: None,
                fields,
            });
        }
        draw::DrawLayerNode::Group(group) => {
            fields.push(ui_inspector_readonly_field(
                "draw-play-inspector.group-children",
                labels.children_count,
                group.children.len().to_string(),
            ));
            return Some(UiInspectorFieldGroup {
                id: "draw-play-inspector.kind.group".into(),
                label: labels.kind_group.into(),
                default_open: None,
                fields,
            });
        }
        _ => {}
    }
    None
}

fn inspector_appearance_group(layers: &[&draw::DrawLayerNode], labels: &DrawLabels) -> UiInspectorFieldGroup {
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
        label: labels.appearance.into(),
        default_open: None,
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.fill", labels.fill, &fill_colors, "fillColor"),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.fill-alpha".into(),
                label: labels.fill_alpha.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    id: "draw-play-inspector.fill-alpha.slider".into(),
                    value: if fill_alpha_mixed.uniform { fill_alpha_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "fillAlpha"),
                    unit: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
            inspector_number_field(&layer_ids, "draw-play-inspector.stroke-width", labels.stroke_width, &stroke_widths, "strokeWidth"),
        ],
    }
}

fn inspector_layer_group(layers: &[&draw::DrawLayerNode], labels: &DrawLabels) -> UiInspectorFieldGroup {
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
        label: labels.layer.into(),
        default_open: None,
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.name", labels.name, &names, "name"),
            inspector_number_field(&layer_ids, "draw-play-inspector.opacity", labels.opacity, &opacities, "opacity"),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.blend-mode".into(),
                label: labels.blend_mode.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "draw-play-inspector.blend-mode.select".into(),
                    value: blend_mixed.value,
                    placeholder: blend_mixed.placeholder,
                    items: DRAW_BLEND_MODES
                        .iter()
                        .map(|mode| UiSelectItem { value: (*mode).into(), label: (*mode).into() })
                        .collect(),
                    on_change: inspector_patch(&layer_ids, "blendMode"),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.visible".into(),
                label: labels.visible.into(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.visible.toggle".into(),
                    icon_id: "eye".into(),
                    pressed: visible_mixed.uniform && visible_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&layer_ids, "visible"),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "draw-play-inspector.locked".into(),
                label: labels.locked.into(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.locked.toggle".into(),
                    icon_id: "lock".into(),
                    pressed: locked_mixed.uniform && locked_mixed.pressed,
                    text: None,
                    on_change: inspector_patch(&layer_ids, "locked"),
                })),
                description: None,
                required: None,
                error: None,
            }),
        ],
    }
}

fn inspector_orientation_group(layers: &[&draw::DrawLayerNode], labels: &DrawLabels) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| layer_id(entry).to_string()).collect();
    UiInspectorFieldGroup {
        id: "draw-play-inspector.orientation".into(),
        label: labels.orientation.into(),
        default_open: None,
        fields: vec![
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-x",
                labels.position_x,
                &layers.iter().map(|entry| layer_base(entry).transform.x).collect::<Vec<_>>(),
                "transformX",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-y",
                labels.position_y,
                &layers.iter().map(|entry| layer_base(entry).transform.y).collect::<Vec<_>>(),
                "transformY",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-scale-x",
                labels.scale_x,
                &layers.iter().map(|entry| layer_base(entry).transform.scale_x).collect::<Vec<_>>(),
                "transformScaleX",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-scale-y",
                labels.scale_y,
                &layers.iter().map(|entry| layer_base(entry).transform.scale_y).collect::<Vec<_>>(),
                "transformScaleY",
            ),
            inspector_number_field(
                &layer_ids,
                "draw-play-inspector.transform-rotation",
                labels.rotation,
                &layers.iter().map(|entry| layer_base(entry).transform.rotation).collect::<Vec<_>>(),
                "transformRotation",
            ),
        ],
    }
}

fn render_properties_panel(document: &DrawDocument, interaction: &DrawInteractionState, labels: &DrawLabels) -> UiNode {
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
    if let Some(kind_group) = inspector_kind_group(document, &selected_layers, labels) {
        groups.push(kind_group);
    }
    groups.push(inspector_orientation_group(&selected_layers, labels));
    groups.push(inspector_appearance_group(&selected_layers, labels));
    groups.push(inspector_layer_group(&selected_layers, labels));
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
            on_change: Some(draw_play_action("engagementInput", None)),
            on_submit: Some(draw_play_action("engagementSubmit", None)),
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
            .window_kind_with_engagement("draw-composite", "Canvas", DRAW_PLAY_BODY_COMPOSITE, SurfaceKind::Canvas2d, engagement)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, DRAW_PLAY_BODY_LAYERS)
            .panel_tab("framework.panel.catalogue", "Catalogue", PanelGroup::Workbench, DRAW_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, DRAW_PLAY_BODY_PROPERTIES)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+a", "selectAll")
            .keybinding("escape", "canvasEscape")
            .keybinding("enter", "canvasCommitDraft")
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
    semio_framework_os::register_2d_export_handlers("2d.drawing", "draw", draw::draw_document_json_to_svg);
    semio_framework_os::register_os_media_export_handler("2d.drawing", semio_framework_os::OsMediaFormat::Dwg, |doc| {
        let bytes = draw::draw_document_json_to_dwg_bytes(doc)?;
        Ok(semio_framework_os::OsMediaExportResult {
            data: { use base64::Engine; base64::engine::general_purpose::STANDARD.encode(bytes) },
            mime_type: semio_framework_os::OsMediaFormat::Dwg.mime_type().into(),
            file_name: "draw.dwg".into(),
            encoding: Some("base64".into()),
        })
    });
    semio_framework_os::register_dwg_import_handler("2d.drawing", draw::draw_document_json_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "draw",
    label: "Draw",
    version: "0.1.0",
    setup: register_draw_exports,
    apps: [ create_draw_app => DrawApp ],
}
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
        let value = serde_json::to_value(&node).unwrap();
        let layers_json = value.pointer("/canvas2d/layersJson").and_then(|v| v.as_str()).expect("layersJson string");
        assert!(layers_json.contains("segments"));
        let records: Vec<Value> = serde_json::from_str(layers_json).unwrap();
        assert!(records.iter().any(|record| record.get("role").and_then(|value| value.as_str()) == Some("meta")));
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
    fn add_layer_action_appends_path() {
        let mut app = DrawApp;
        let document = serde_json::to_string(&empty_draw_projection()).unwrap();
        let ops = app.handle_action_patch_ops("addLayer", Some(&json!({ "kind": "path" })), &document, &ViewState::default());
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
        let ops = app.handle_action_patch_ops(
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
        let ops = app.handle_action_patch_ops("setActiveTool", Some(&json!({ "tool": "pen" })), &document, &ViewState::default());
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
        let ops = app.handle_action_patch_ops(
            "combineBoolean",
            Some(&json!({ "op": "union", "ids": [first_id, second_id] })),
            &document_json,
            &ViewState::default(),
        );
        let next = apply_ops(&document, &ops);
        assert!(next.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Boolean(_))));
    }

    #[test]
    fn canvas_point_to_world_matches_host_formula() {
        let camera = draw::DrawCamera { x: 100.0, y: 50.0, zoom: 2.0 };
        let (world_x, world_y) = canvas_point_to_world(&camera, 420.0, 310.0, 800.0, 600.0);
        assert!((world_x - 110.0).abs() < 1e-9);
        assert!((world_y - 55.0).abs() < 1e-9);
    }

    fn dispatch(app: &mut DrawApp, document_json: &str, action: &str, args: Option<Value>, view_state: &ViewState) -> String {
        let ops = app.handle_action_patch_ops(action, args.as_ref(), document_json, view_state);
        for op_json in ops.iter().rev() {
            if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                if op.get("op").and_then(|value| value.as_str()) == Some("setDocument") {
                    if let Some(document) = op.get("document") {
                        return document.to_string();
                    }
                }
            }
        }
        document_json.to_string()
    }

    fn envelope_json(document: DrawDocument) -> String {
        serde_json::to_string(&DrawPlayEnvelope {
            document,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            interaction: DrawInteractionState::default(),
        })
        .unwrap()
    }

    #[test]
    fn shape_rect_drag_commits_rectangle_layer() {
        let mut app = DrawApp;
        let mut document = default_draw_document("shape-test", None);
        document.active_tool = Some("shapeRect".into());
        document.layers.clear();
        let mut state = envelope_json(document);
        state = dispatch(&mut app, &state, "canvasPointerDown", Some(json!({ "x": 500.0, "y": 400.0, "width": 1000.0, "height": 800.0 })), &ViewState::default());
        state = dispatch(&mut app, &state, "canvasPointerMove", Some(json!({ "x": 600.0, "y": 500.0, "width": 1000.0, "height": 800.0 })), &ViewState::default());
        state = dispatch(
            &mut app,
            &state,
            "canvasPointerUp",
            Some(json!({ "x": 600.0, "y": 500.0, "width": 1000.0, "height": 800.0, "shift": false, "ctrl": false, "meta": false })),
            &ViewState::default(),
        );
        let envelope: DrawPlayEnvelope = serde_json::from_str(&state).unwrap();
        assert!(envelope.document.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Shape(shape) if shape.shape_kind == "rect")));
        assert_eq!(envelope.document.active_tool.as_deref(), Some("selectDirect"));
        assert_eq!(envelope.interaction.selected_ids.len(), 1);
    }

    #[test]
    fn pen_draft_commits_path_layer_on_enter() {
        let mut app = DrawApp;
        let mut document = default_draw_document("pen-test", None);
        document.active_tool = Some("pen".into());
        document.layers.clear();
        let mut state = envelope_json(document);
        state = dispatch(&mut app, &state, "canvasPointerDown", Some(json!({ "x": 400.0, "y": 300.0, "width": 800.0, "height": 600.0 })), &ViewState::default());
        state = dispatch(&mut app, &state, "canvasPointerDown", Some(json!({ "x": 500.0, "y": 300.0, "width": 800.0, "height": 600.0 })), &ViewState::default());
        state = dispatch(&mut app, &state, "canvasCommitDraft", None, &ViewState::default());
        let envelope: DrawPlayEnvelope = serde_json::from_str(&state).unwrap();
        assert!(envelope.document.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Path(_))));
        assert!(envelope.interaction.drag.is_none());
        assert_eq!(envelope.document.active_tool.as_deref(), Some("selectDirect"));
    }

    #[test]
    fn canvas_escape_cancels_draft_without_committing() {
        let mut app = DrawApp;
        let mut document = default_draw_document("escape-test", None);
        document.active_tool = Some("pen".into());
        document.layers.clear();
        let mut state = envelope_json(document);
        state = dispatch(&mut app, &state, "canvasPointerDown", Some(json!({ "x": 400.0, "y": 300.0, "width": 800.0, "height": 600.0 })), &ViewState::default());
        state = dispatch(&mut app, &state, "canvasEscape", None, &ViewState::default());
        let envelope: DrawPlayEnvelope = serde_json::from_str(&state).unwrap();
        assert!(envelope.document.layers.is_empty());
        assert!(envelope.interaction.drag.is_none());
    }

    #[test]
    fn marquee_select_covers_contained_layer_only() {
        let mut app = DrawApp;
        let mut document = default_draw_document("marquee-test", None);
        document.layers.clear();
        document.active_tool = Some("selectMarquee".into());
        let mut rect_a = create_draw_shape_layer_rect("A");
        if let DrawLayerNode::Shape(shape) = &mut rect_a {
            shape.rect = Some(draw::DrawRect { x: 10.0, y: 10.0, width: 20.0, height: 20.0 });
        }
        let rect_a_id = draw::layer_id(&rect_a).to_string();
        let mut rect_b = create_draw_shape_layer_rect("B");
        if let DrawLayerNode::Shape(shape) = &mut rect_b {
            shape.rect = Some(draw::DrawRect { x: 200.0, y: 200.0, width: 20.0, height: 20.0 });
        }
        document.layers.push(rect_a);
        document.layers.push(rect_b);
        let mut state = envelope_json(document);
        state = dispatch(&mut app, &state, "canvasPointerDown", Some(json!({ "x": 400.0, "y": 300.0, "width": 800.0, "height": 600.0 })), &ViewState::default());
        state = dispatch(&mut app, &state, "canvasPointerMove", Some(json!({ "x": 460.0, "y": 360.0, "width": 800.0, "height": 600.0 })), &ViewState::default());
        state = dispatch(
            &mut app,
            &state,
            "canvasPointerUp",
            Some(json!({ "x": 460.0, "y": 360.0, "width": 800.0, "height": 600.0, "shift": false, "ctrl": false, "meta": false })),
            &ViewState::default(),
        );
        let envelope: DrawPlayEnvelope = serde_json::from_str(&state).unwrap();
        assert_eq!(envelope.interaction.selected_ids, vec![rect_a_id]);
    }

    #[test]
    fn set_camera_does_not_push_undo() {
        let mut app = DrawApp;
        let document = serde_json::to_string(&empty_draw_projection()).unwrap();
        let ops = app.handle_action_patch_ops("setCamera", Some(&json!({ "camera": { "x": 5.0, "y": 5.0, "zoom": 2.0 } })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let op: Value = serde_json::from_str(&ops[0]).unwrap();
        let envelope: DrawPlayEnvelope = serde_json::from_value(op.get("document").unwrap().clone()).unwrap();
        assert!(envelope.undo_stack.is_empty());
        assert_eq!(envelope.document.camera.zoom, 2.0);
    }

    #[test]
    fn tools_marks_active_tool_pressed() {
        let app = DrawApp;
        let mut document = empty_draw_projection();
        document.active_tool = Some("pen".into());
        let document_json = serde_json::to_string(&document).unwrap();
        let tools = app.tools(&document_json, &ViewState::default());
        assert_eq!(tools.len(), 4);
        assert!(tools.iter().all(|tool| matches!(tool, ToolNode::Collection { .. })));
        let all_toggles: Vec<&ToolNode> = tools
            .iter()
            .flat_map(|tool| match tool {
                ToolNode::Collection { children, .. } => children.iter(),
                _ => [].iter(),
            })
            .collect();
        assert_eq!(all_toggles.len(), 11);
        let pen_pressed = all_toggles.iter().any(|tool| matches!(tool, ToolNode::Toggle { id, pressed, .. } if id == "draw-play-tools.pen" && *pressed == Some(true)));
        assert!(pen_pressed);
    }

    #[test]
    fn render_canvas_emits_selection_overlay() {
        let app = DrawApp;
        let mut document = default_draw_document("overlay-test", None);
        document.layers.clear();
        document.layers.push(create_draw_shape_layer_rect("Rect"));
        let layer_id = draw::layer_id(&document.layers[0]).to_string();
        let document_json = serde_json::to_string(&document).unwrap();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, &document_json, &view_with_selection(&[layer_id.as_str()]));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("overlay:sel:"));
    }

    #[test]
    fn draw_labels_resolve_native_by_default() {
        let app = DrawApp;
        let document = serde_json::to_string(&default_draw_document("test", None)).unwrap();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Add Path"));
        assert!(json.contains("Add Rectangle"));
        assert!(!json.contains("Pfad hinzufügen"));
    }

    #[test]
    fn draw_labels_translate_panels_in_german() {
        let app = DrawApp;
        let document = serde_json::to_string(&default_draw_document("test", None)).unwrap();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let layers_node = app.render(DRAW_PLAY_BODY_LAYERS, &document, &view_state);
        let layers_json = serde_json::to_string(&layers_node).unwrap();
        assert!(layers_json.contains("Pfad hinzufügen"));
        assert!(layers_json.contains("Rechteck hinzufügen"));
        assert!(!layers_json.contains("Add Path"));
        let catalogue_node = app.render(DRAW_PLAY_BODY_CATALOGUE, &document, &view_state);
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("\"Ellipse\""));
        assert!(catalogue_json.contains("Nachzeichnung"));
        let tools = app.tools(&document, &view_state);
        let tools_json = serde_json::to_string(&tools).unwrap();
        assert!(tools_json.contains("Rahmenauswahl"));
        assert!(tools_json.contains("Zeichnen"));
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
