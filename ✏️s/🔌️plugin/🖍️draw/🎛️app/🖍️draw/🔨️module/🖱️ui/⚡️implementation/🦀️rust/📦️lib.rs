//! 🖥️ Draw app — DocumentApp impl, render, manifest (constitutional: ui). B1: `DrawPlayApp` keeps
//! only ephemeral in-process gesture scratch (`gesture`/`preview_seq` `RefCell`s, never undo-tracked
//! — see `//#region 🔖️UtilityPreviewContract` in the framework plugin crate); every former
//! `DrawInteractionState` field (selection, hover, engagement-input draft, the session-only free
//! canvas camera) now lives in `draw_engine::DrawConfig`, written via `draw_op::DrawConfigOperation`s
//! (real `backwards`, no ad hoc inverse). Every action dispatches through the single typed
//! `draw_protocol::DrawCommand` channel via `DocumentApp::handle` — mirrors `shooting_ui`'s B1 pilot.

use draw::{DrawArtboard, DrawDocument, DrawLayerNode, PathSegment, DRAW_BLEND_MODES, DRAW_BOOLEAN_OPERATIONS, DRAW_DOCUMENT_SCHEMA};
use draw_engine::{
    create_draw_boolean_layer, create_draw_path_layer, create_draw_trace_layer, create_layer_by_kind, default_draw_document, default_layer_base, draw_io, draw_layer_descendant_leaf_ids, draw_layer_world_bounds, draw_play_boolean_child_row_id,
    draw_play_layer_id_from_tree_row_id, draw_play_layers_tree_row_id, draw_transform_to_matrix, draw_vector_media, empty_draw_projection, find_draw_layer, find_draw_layer_location, flatten_draw_document_to_scene_nodes, flatten_draw_layers,
    layer_base, layer_id, layer_kind_label, layer_to_path_segments, resolve_draw_artboard, rgba_to_hex, semio_draw_example_document, semio_draw_example_json, DrawConfig,
};
use draw_op::{draw_op_for_layer_field, DrawConfigOperation, DrawOperation};
use draw_protocol::DrawCommand;
use semio_framework_plugin::kernel::HostEffect;
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, selection_ids, tree_item, tree_item_with_action, tree_item_with_action_draggable, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_select, ui_inspector_mixed_slider,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, AppLabels, ArtifactKindSpec, Canvas2dScene, ConfigView, DocumentApp,
    DocumentView, Emit, Label, Locale, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder, SurfaceKind, Terminology, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemNode, UtilityCategory, UtilityDefinition, WindowEngagement, WindowEngagementInput, WindowEngagementStatus,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_UTILITY_ACTION_ID, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use store::DocumentPack;

//#region 🔖️Constants
const DRAW_PLAY_APP_ID: &str = "draw-play";
const DRAW_PLAY_CONTROLLER_ID: &str = "draw-play";
const DRAW_PLAY_WINDOW_CANVAS: &str = "draw-composite";
/// 🧰️ The utility the canvas returns to after committing a shape/draft/trace (first UtilityRef default).
const DRAW_DEFAULT_UTILITY: &str = "selectDirect";
const DRAW_PLAY_SURFACE_ID: &str = "draw.play.composite";
const DRAW_PLAY_BODY_COMPOSITE: &str = "draw.play.composite";
const DRAW_PLAY_BODY_LAYERS: &str = "draw.play.layers";
const DRAW_PLAY_BODY_CATALOGUE: &str = "draw.play.catalogue";
const DRAW_PLAY_BODY_PROPERTIES: &str = "draw.play.properties";
const DRAW_LAYER_KIND_DRAG_MIME: &str = "application/x-semio-draw-layer-kind";
const DRAW_PLAY_EXAMPLE_DEFAULT_ID: &str = "semio";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s identical helpers.
fn is_de_locale(cfg: &DrawConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn draw_locale(cfg: &DrawConfig) -> Locale {
    if is_de_locale(cfg) {
        Locale::De
    } else {
        Locale::En
    }
}

/// 🗣️ `AppLabels::labels` (was the deleted `LocaleLabels::locale_labels_en/de`).
fn resolve_labels<L: AppLabels>(cfg: &DrawConfig) -> &'static L {
    L::labels(draw_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn draw_play_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: DRAW_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

fn canvas_point_to_world(camera: &draw::DrawCamera, x: f64, y: f64, viewport_w: f64, viewport_h: f64) -> (f64, f64) {
    let zoom = camera.zoom.max(0.01);
    ((x - viewport_w * 0.5) / zoom + camera.x, (y - viewport_h * 0.5) / zoom + camera.y)
}

//#region 🔖️GestureMachine
const DRAW_MARQUEE_THRESHOLD_PX: f64 = 4.0;
const DRAW_PICK_TOLERANCE_PX: f64 = 8.0;

fn matrix_transform_point(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

/// 🎯️ Maps shift/ctrl/meta modifiers to a `SelectionMergeMode` (matches `@semio-tech/ui-react`'s `marqueeModeFromModifiers`).
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

/// 🎯️ All pick targets under a world point (groups win by default, control points win over everything when enabled).
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

fn apply_point_pick(interaction: &mut DrawConfig, doc: &DrawDocument, world: [f64; 2], shift: bool, ctrl: bool, meta: bool, include_control_points: bool) {
    let tolerance = DRAW_PICK_TOLERANCE_PX / interaction.camera.zoom.max(1e-6);
    let targets = resolve_pick_targets_at(doc, world, tolerance, include_control_points);
    let picked = best_pick_layer_id(&targets);
    let mode = selection_merge_mode(shift, ctrl, meta);
    interaction.selected_ids = match picked {
        Some(id) => merge_selection(mode, &interaction.selected_ids, &[id]),
        None if mode == "default" => Vec::new(),
        None => interaction.selected_ids.clone(),
    };
}

/// ⬚️ Marquee/lasso layer hits — reduces the lasso gesture to its bounding box, matching the premigration behaviour.
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

fn shape_preview_segments(utility: &str, start: [f64; 2], end: [f64; 2]) -> Vec<PathSegment> {
    if utility == "shapeLine" {
        return vec![PathSegment::Move { to: start }, PathSegment::Line { to: end }];
    }
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if utility == "shapeRect" {
        return vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close];
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

fn draft_preview_segments(utility: &str, points: &[[f64; 2]], cursor: [f64; 2]) -> Vec<PathSegment> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut segments = vec![PathSegment::Move { to: points[0] }];
    for point in points.iter().skip(1) {
        segments.push(PathSegment::Line { to: *point });
    }
    segments.push(PathSegment::Line { to: cursor });
    if utility == "shapePolygon" && points.len() > 1 {
        segments.push(PathSegment::Close);
    }
    segments
}

/// 🔷️ Emits the operations that commit a shape drag (add the shape layer + return to direct-select) and
/// records the new layer as the current selection; empty when the drag is too small to commit.
fn commit_shape_drag(interaction: &mut DrawConfig, doc: &DrawDocument, utility: &str, start: [f64; 2], end: [f64; 2]) -> Vec<DrawOperation> {
    let x = start[0].min(end[0]);
    let y = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    if width < 1.0 && height < 1.0 {
        return Vec::new();
    }
    let layer = DrawLayerNode::Shape(draw::DrawShapeBody {
        base: default_layer_base(match utility {
            "shapeLine" => "Line",
            "shapeEllipse" => "Ellipse",
            _ => "Rectangle",
        }),
        shape_kind: match utility {
            "shapeLine" => "line",
            "shapeEllipse" => "ellipse",
            _ => "rect",
        }
        .into(),
        rect: if utility == "shapeRect" { Some(draw::DrawRect { x, y, width, height }) } else { None },
        ellipse: if utility == "shapeEllipse" { Some(draw::DrawEllipse { cx: x + width / 2.0, cy: y + height / 2.0, rx: width / 2.0, ry: height / 2.0 }) } else { None },
        circle: None,
        line: if utility == "shapeLine" { Some(draw::DrawLine { x1: start[0], y1: start[1], x2: end[0], y2: end[1] }) } else { None },
        polygon: None,
    });
    let select_id = layer_id(&layer).to_string();
    interaction.selected_ids = vec![select_id];
    vec![DrawOperation::AddLayer { parent_id: None, index: Some(doc.layers.len()), layer: Box::new(layer) }]
}

/// ✒️ Emits the operations that commit a freehand/polygon draft into a path or polygon layer and records it
/// as the current selection; empty when the draft has too few points to form a shape.
fn commit_draft(interaction: &mut DrawConfig, doc: &DrawDocument, utility: &str, points: &[[f64; 2]]) -> Vec<DrawOperation> {
    if points.len() < 2 {
        return Vec::new();
    }
    let layer = if utility == "pen" {
        let mut segments = vec![PathSegment::Move { to: points[0] }];
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
        create_draw_path_layer("Path", segments)
    } else {
        DrawLayerNode::Shape(draw::DrawShapeBody { base: default_layer_base("Polygon"), shape_kind: "polygon".into(), rect: None, ellipse: None, circle: None, line: None, polygon: Some(draw::DrawPolygon { points: points.to_vec() }) })
    };
    let select_id = layer_id(&layer).to_string();
    interaction.selected_ids = vec![select_id];
    vec![DrawOperation::AddLayer { parent_id: None, index: Some(doc.layers.len()), layer: Box::new(layer) }]
}

/// 🖍️ Emits the operations that add a trace layer over the picked image (or first asset) and records it as
/// the current selection; empty when no bitmap source is available.
fn commit_trace_at(interaction: &mut DrawConfig, doc: &DrawDocument, world: [f64; 2]) -> Vec<DrawOperation> {
    let tolerance = DRAW_PICK_TOLERANCE_PX / interaction.camera.zoom.max(1e-6);
    let hit_layer_id = best_pick_layer_id(&resolve_pick_targets_at(doc, world, tolerance, false));
    let source_key = hit_layer_id
        .and_then(|id| find_draw_layer(doc, &id).cloned())
        .and_then(|layer| match layer {
            DrawLayerNode::Image(image) => Some(image.image_key),
            _ => None,
        })
        .or_else(|| doc.assets.as_ref().and_then(|assets| assets.keys().next().cloned()));
    let Some(source_key) = source_key else { return Vec::new() };
    let layer = create_draw_trace_layer("Trace", &source_key);
    let select_id = layer_id(&layer).to_string();
    interaction.selected_ids = vec![select_id];
    vec![DrawOperation::AddLayer { parent_id: None, index: Some(doc.layers.len()), layer: Box::new(layer) }]
}

/// 🧰️ Wraps a committed gesture's `operations` as a single described edit plus the host effect that returns
/// the canvas to the default select utility (the active utility is host-owned, never a document operation).
fn commit_with_utility_reset(operations: Vec<DrawOperation>, description: &str) -> Emit<DrawOperation, DrawConfigOperation> {
    if operations.is_empty() {
        return Emit::default();
    }
    let mut emit = Emit::commit(operations, description);
    emit.effects.push(HostEffect::SetActiveUtility { window_id: DRAW_PLAY_WINDOW_CANVAS.into(), utility_id: DRAW_DEFAULT_UTILITY.into() });
    emit
}

/// 🧮️ B1: appends a `DrawConfigOperation::SetSelection` config edit to a gesture's document-side
/// `Emit` iff the gesture actually changed the selection (`apply_point_pick`/`commit_shape_drag`/…
/// mutate `config.selected_ids` in place) — keeps document operations (shape/draft/trace commits) and
/// the selection change that rode along with them in exactly one dispatch's `Emit`.
fn finish_gesture_emit(mut emit: Emit<DrawOperation, DrawConfigOperation>, before: &DrawConfig, after: &DrawConfig) -> Emit<DrawOperation, DrawConfigOperation> {
    if after.selected_ids != before.selected_ids {
        emit.config_operations.push(DrawConfigOperation::SetSelection { ids: after.selected_ids.clone() });
    }
    emit
}

/// 🩹️ Parses a `DrawCommand::PatchLayer`/`PatchLayers` wire `value` as JSON text (falling back to a
/// plain JSON string when it isn't valid JSON) so one `String` wire field covers every heterogeneous
/// `draw_op_for_layer_field` value type (bool/number/string) — mirrors
/// `shooting_protocol::ShootingCommand`'s `PatchShots`/`PatchAssets` shape.
fn patch_value_json(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()))
}

//#region 🔖️GestureContext
/// 🎛️ Per-gesture scratch geometry threaded through the shared `fsm` statechart below — one flat
/// struct (XState convention: context is machine-global, never per-state) mirroring the fields the
/// old hand-rolled `DrawDragState` enum kept per-variant.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GestureContext {
    method: String,
    merge: String,
    utility: String,
    start: [f64; 2],
    cursor: [f64; 2],
    points: Vec<[f64; 2]>,
    active: bool,
}

/// 🎇️ Document-touching side effects the gesture machine requests but never executes — `fsm`'s
/// guards/actions only ever see `(&Context, Option<&Event>)`, never the `DrawDocument` tree, so every
/// hit-test/commit that needs the document is deferred to `DrawPlayApp::step_gesture` as an effect.
#[derive(Clone, Debug)]
pub enum GestureEffect {
    CommitMarquee { start: [f64; 2], end: [f64; 2], active: bool, merge: String, shift: bool, ctrl: bool, meta: bool },
    CommitShape { utility: String, start: [f64; 2], end: [f64; 2] },
    CommitDraft { utility: String, points: Vec<[f64; 2]> },
    CommitTrace { world: [f64; 2] },
    PickPoint { world: [f64; 2], shift: bool, ctrl: bool, meta: bool },
}

fn gesture_context_from_input(_input: ()) -> GestureContext {
    GestureContext::default()
}
//#endregion 🔖️GestureContext

//#region 🔖️GesturePreview
/// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": a pure, JSON-serializable
/// snapshot of the gesture machine's live, uncommitted scratch geometry — shaped as the exact payload
/// `framework_sync::SyncSession::publish_preview(key, seq, payload)` expects, ready to hand off the
/// instant a transport exists. `None` while `draw_gesture` is `idle` (no live gesture to preview); this
/// function only ever reads `GestureContext`, never `DrawDocument`/`DrawOperation` — a preview can never
/// become persistent state.
///
/// 🚧️ Deliberately unwired beyond `DrawPlayApp::gesture_preview` below: `framework/sync` (home of
/// `SyncSession::publish_preview`) documents itself as host-only — "WASI-P2 plugins never link this
/// crate" — and this crate compiles as a WASI-P2 component (`crate-type = ["cdylib", "rlib"]`,
/// `semio:draw`), so it cannot reach that API directly. The one cross-sandbox channel this crate can
/// reach, `store::BackboneMessage` (via `host_backbone_send`), has no preview-shaped variant yet — adding
/// one is a `vcs`/WIT/`framework/plugin/host` change outside this ticket's plugin-crate-only file
/// ownership. See `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw7-preview-law.txt`.
fn draw_gesture_preview_payload(ctx: &GestureContext, is_idle: bool) -> Option<Value> {
    if is_idle {
        return None;
    }
    Some(json!({
        "method": ctx.method,
        "utility": ctx.utility,
        "start": ctx.start,
        "cursor": ctx.cursor,
        "points": ctx.points,
        "active": ctx.active,
    }))
}
//#endregion 🔖️GesturePreview

//#region 🔖️GestureGuards
fn utility_is_marquee(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if utility == "selectMarquee" || utility == "selectLasso")
}

fn utility_is_shape(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if matches!(utility.as_str(), "shapeRect" | "shapeEllipse" | "shapeLine"))
}

fn utility_is_draft(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if utility == "pen" || utility == "shapePolygon")
}

fn utility_is_trace(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if utility == "trace")
}

/// 🖊️ Drafting self-loop: the same pen/polygon utility is still active, so the pointer-down appends a point.
fn utility_is_draft_same(ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if (utility == "pen" || utility == "shapePolygon") && utility == &ctx.utility)
}

/// 🖊️ Drafting restart: a different pen/polygon utility switched in without going through `UtilityChanged` first.
fn utility_is_draft_different(ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerDown { utility, .. }) if (utility == "pen" || utility == "shapePolygon") && utility != &ctx.utility)
}

fn utility_is_select_direct(_ctx: &GestureContext, event: Option<&draw_gesture::Event>) -> bool {
    matches!(event, Some(draw_gesture::Event::PointerUp { utility, .. }) if utility == "selectDirect")
}
//#endregion 🔖️GestureGuards

//#region 🔖️GestureActions
fn gesture_start_marquee(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, shift, ctrl, meta }) = event {
        ctx.method = if utility == "selectLasso" { "lasso".into() } else { "rectangle".into() };
        ctx.start = *world;
        ctx.cursor = *world;
        ctx.merge = selection_merge_mode(*shift, *ctrl, *meta).into();
        ctx.active = false;
    }
}

fn gesture_start_shape(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, .. }) = event {
        ctx.utility = utility.clone();
        ctx.start = *world;
        ctx.cursor = *world;
    }
}

fn gesture_start_draft(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { utility, world, .. }) = event {
        ctx.utility = utility.clone();
        ctx.points = vec![*world];
        ctx.cursor = *world;
    }
}

fn gesture_append_draft_point(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { world, .. }) = event {
        ctx.points.push(*world);
        ctx.cursor = *world;
    }
}

fn gesture_update_marquee_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerMove { world, marquee_threshold_world }) = event {
        let distance = ((world[0] - ctx.start[0]).powi(2) + (world[1] - ctx.start[1]).powi(2)).sqrt();
        ctx.active = ctx.active || distance >= *marquee_threshold_world;
        ctx.cursor = *world;
    }
}

fn gesture_update_shape_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerMove { world, .. }) = event {
        ctx.cursor = *world;
    }
}

fn gesture_update_draft_cursor(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, _sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerMove { world, .. }) = event {
        ctx.cursor = *world;
    }
}

fn gesture_commit_marquee(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerUp { world, shift, ctrl, meta, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitMarquee { start: ctx.start, end: *world, active: ctx.active, merge: ctx.merge.clone(), shift: *shift, ctrl: *ctrl, meta: *meta }));
    }
}

fn gesture_commit_shape(ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerUp { world, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitShape { utility: ctx.utility.clone(), start: ctx.start, end: *world }));
    }
}

fn gesture_commit_draft(ctx: &mut GestureContext, _event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    sink.push(fsm::Command::Effect(GestureEffect::CommitDraft { utility: ctx.utility.clone(), points: ctx.points.clone() }));
}

fn gesture_commit_trace(_ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerDown { world, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::CommitTrace { world: *world }));
    }
}

fn gesture_pick_point(_ctx: &mut GestureContext, event: Option<&draw_gesture::Event>, sink: &mut dyn fsm::CommandSink<draw_gesture::DrawGesture>) {
    if let Some(draw_gesture::Event::PointerUp { world, shift, ctrl, meta, .. }) = event {
        sink.push(fsm::Command::Effect(GestureEffect::PickPoint { world: *world, shift: *shift, ctrl: *ctrl, meta: *meta }));
    }
}
//#endregion 🔖️GestureActions

//#region 🔖️GestureStatechart
/// 🎭️ Pointer-gesture control flow — states/events/guards straight off the old hand-rolled
/// `DrawDragState` match arms, now compiled by `fsm`'s `statechart!` DSL (`fsm/rs`, `fsm/macros/rs`)
/// into dense static tables instead of an ad-hoc enum + inline `match` transition ladder.
fsm::statechart! {
    machine draw_gesture {
        context: GestureContext;
        event Event {
            PointerDown { utility: String, world: [f64; 2], shift: bool, ctrl: bool, meta: bool },
            PointerMove { world: [f64; 2], marquee_threshold_world: f64 },
            PointerUp { utility: String, world: [f64; 2], shift: bool, ctrl: bool, meta: bool },
            CommitDraft,
            Escape,
            UtilityChanged,
        }
        input: ();
        output: ();
        effect: GestureEffect;
        context_from_input: gesture_context_from_input;
        initial: idle;

        state idle {
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_draft => drafting do gesture_start_draft;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on PointerUp if utility_is_select_direct => idle do gesture_pick_point;
        }
        state marqueeing {
            on PointerMove => marqueeing do gesture_update_marquee_cursor;
            on PointerUp => idle do gesture_commit_marquee;
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_draft => drafting do gesture_start_draft;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on Escape => idle;
            on UtilityChanged => idle;
        }
        state shape_dragging {
            on PointerMove => shape_dragging do gesture_update_shape_cursor;
            on PointerUp => idle do gesture_commit_shape;
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_draft => drafting do gesture_start_draft;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on Escape => idle;
            on UtilityChanged => idle;
        }
        state drafting {
            on PointerMove => drafting do gesture_update_draft_cursor;
            on PointerDown if utility_is_draft_same => drafting do gesture_append_draft_point;
            on PointerDown if utility_is_draft_different => drafting do gesture_start_draft;
            on PointerDown if utility_is_marquee => marqueeing do gesture_start_marquee;
            on PointerDown if utility_is_shape => shape_dragging do gesture_start_shape;
            on PointerDown if utility_is_trace => idle do gesture_commit_trace;
            on CommitDraft => idle do gesture_commit_draft;
            on Escape => idle;
            on UtilityChanged => idle;
        }
    }
}
//#endregion 🔖️GestureStatechart
//#endregion 🔖️GestureMachine

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
                let index = if drop_position == "before" { location.index } else { location.index + 1 };
                return (location.parent_id, index);
            }
        }
    }
    (None, document.layers.len())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the draw app; one field per label makes every locale combination compile-checked.
    struct DrawPlayLabels {
        window_canvas: native_en "Canvas", native_de "Leinwand", reuse_en "Canvas", reuse_de "Leinwand";
        mode_edit: native_en "Edit", native_de "Bearbeiten", reuse_en "Edit", reuse_de "Bearbeiten";
        add_path: native_en "Add Path", native_de "Pfad hinzufügen", reuse_en "Add Path", reuse_de "Pfad hinzufügen";
        add_rectangle: native_en "Add Rectangle", native_de "Rechteck hinzufügen", reuse_en "Add Rectangle", reuse_de "Rechteck hinzufügen";
        add_text: native_en "Add Text", native_de "Text hinzufügen", reuse_en "Add Text", reuse_de "Text hinzufügen";
        add_group: native_en "Add Group", native_de "Gruppe hinzufügen", reuse_en "Add Group", reuse_de "Gruppe hinzufügen";
        add_boolean: native_en "Add Boolean", native_de "Boolean hinzufügen", reuse_en "Add Boolean", reuse_de "Boolean hinzufügen";
        empty_state: native_en "Drop layers here", native_de "Ebenen hier ablegen", reuse_en "Drop layers here", reuse_de "Ebenen hier ablegen";
        kind_path: native_en "Path", native_de "Pfad", reuse_en "Path", reuse_de "Pfad";
        kind_rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        kind_ellipse: native_en "Ellipse", native_de "Ellipse", reuse_en "Ellipse", reuse_de "Ellipse";
        kind_line: native_en "Line", native_de "Linie", reuse_en "Line", reuse_de "Linie";
        kind_polygon: native_en "Polygon", native_de "Polygon", reuse_en "Polygon", reuse_de "Polygon";
        kind_text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        kind_image: native_en "Image", native_de "Bild", reuse_en "Image", reuse_de "Bild";
        kind_group: native_en "Group", native_de "Gruppe", reuse_en "Group", reuse_de "Gruppe";
        kind_boolean: native_en "Boolean", native_de "Boolean", reuse_en "Boolean", reuse_de "Boolean";
        kind_trace: native_en "Trace", native_de "Nachzeichnung", reuse_en "Trace", reuse_de "Nachzeichnung";
        boolean_operation: native_en "Boolean Operation", native_de "Boolean-Operation", reuse_en "Boolean Operation", reuse_de "Boolean-Operation";
        children: native_en "Children", native_de "Kinder", reuse_en "Children", reuse_de "Kinder";
        trace_threshold: native_en "Trace Threshold", native_de "Trace-Schwellenwert", reuse_en "Trace Threshold", reuse_de "Trace-Schwellenwert";
        simplify: native_en "Simplify", native_de "Vereinfachen", reuse_en "Simplify", reuse_de "Vereinfachen";
        source_key: native_en "Source Key", native_de "Quellschlüssel", reuse_en "Source Key", reuse_de "Quellschlüssel";
        width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        content: native_en "Content", native_de "Inhalt", reuse_en "Content", reuse_de "Inhalt";
        size: native_en "Size", native_de "Größe", reuse_en "Size", reuse_de "Größe";
        segment_count: native_en "Segment Count", native_de "Segmentanzahl", reuse_en "Segment Count", reuse_de "Segmentanzahl";
        children_count: native_en "Children Count", native_de "Kinderanzahl", reuse_en "Children Count", reuse_de "Kinderanzahl";
        appearance: native_en "Appearance", native_de "Erscheinungsbild", reuse_en "Appearance", reuse_de "Erscheinungsbild";
        fill: native_en "Fill", native_de "Füllung", reuse_en "Fill", reuse_de "Füllung";
        fill_alpha: native_en "Fill Alpha", native_de "Füllung Alpha", reuse_en "Fill Alpha", reuse_de "Füllung Alpha";
        stroke_width: native_en "Stroke Width", native_de "Strichstärke", reuse_en "Stroke Width", reuse_de "Strichstärke";
        layer: native_en "Layer", native_de "Ebene", reuse_en "Layer", reuse_de "Ebene";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        opacity: native_en "Opacity", native_de "Deckkraft", reuse_en "Opacity", reuse_de "Deckkraft";
        blend_mode: native_en "Blend Mode", native_de "Mischmodus", reuse_en "Blend Mode", reuse_de "Mischmodus";
        visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        orientation: native_en "Orientation", native_de "Ausrichtung", reuse_en "Orientation", reuse_de "Ausrichtung";
        position_x: native_en "Position X", native_de "Position X", reuse_en "Position X", reuse_de "Position X";
        position_y: native_en "Position Y", native_de "Position Y", reuse_en "Position Y", reuse_de "Position Y";
        scale_x: native_en "Scale X", native_de "Skalierung X", reuse_en "Scale X", reuse_de "Skalierung X";
        scale_y: native_en "Scale Y", native_de "Skalierung Y", reuse_en "Scale Y", reuse_de "Skalierung Y";
        rotation: native_en "Rotation", native_de "Rotation", reuse_en "Rotation", reuse_de "Rotation";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
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
        draw::DrawLayerNode::Boolean(boolean) => Some(boolean.children.iter().map(|child_id| boolean_child_item(doc, &boolean.base.id, child_id)).collect()),
        _ => None,
    };
    let description = Some(match layer {
        draw::DrawLayerNode::Boolean(boolean) => boolean.operation.clone(),
        _ => base.blend_mode.clone(),
    });
    let action = draw_play_action("setSelection", Some(json!({ "ids": [base.id] })));
    let drag_data = json!({ "application/x-semio-draw-layer-id": base.id });
    UiTreeItemNode {
        icon_id: Some(layer_icon(layer).into()),
        default_open: Some(matches!(layer, draw::DrawLayerNode::Group(_))),
        items: nested_items,
        dimmed: if base.visible { None } else { Some(true) },
        ..tree_item_with_action_draggable(row_id, Label::data(base.name.clone()), description, action, &drag_data)
    }
}

fn boolean_child_item(doc: &DrawDocument, boolean_id: &str, child_id: &str) -> UiTreeItemNode {
    let row_id = draw_play_boolean_child_row_id(boolean_id, child_id);
    match find_draw_layer(doc, child_id) {
        Some(child) => UiTreeItemNode { draggable: Some(false), ..tree_item_with_action(row_id, Label::data(layer_base(child).name.clone()), Some(layer_kind_label(child)), draw_play_action("setSelection", Some(json!({ "ids": [child_id] })))) },
        None => UiTreeItemNode { icon_id: Some("alert-circle".into()), draggable: Some(false), ..tree_item(row_id, Label::data(format!("{child_id} (missing)"))) },
    }
}

fn render_layers_panel(document: &DrawDocument, interaction: &DrawConfig, labels: &DrawPlayLabels) -> UiNode {
    let action_items = vec![
        tree_button("draw-play-layers.add.path", labels.add_path, "pen-tool", "addLayer", json!({ "kind": "path" })),
        tree_button("draw-play-layers.add.rect", labels.add_rectangle, "square", "addLayer", json!({ "kind": "shape:rect" })),
        tree_button("draw-play-layers.add.text", labels.add_text, "type", "addLayer", json!({ "kind": "text" })),
        tree_button("draw-play-layers.add.group", labels.add_group, "folder-plus", "addLayer", json!({ "kind": "group" })),
        tree_button("draw-play-layers.add.boolean", labels.add_boolean, "combine", "addLayer", json!({ "kind": "boolean" })),
    ];
    let layer_items = if document.layers.is_empty() {
        vec![UiTreeItemNode { icon_id: Some("pen-tool".into()), menu: None, ..tree_item("draw-play-layers.empty", labels.empty_state) }]
    } else {
        document.layers.iter().map(|layer| layer_tree_item(document, layer)).collect()
    };
    let selected_tree_ids: Vec<String> = interaction.selected_ids.iter().filter_map(|id| find_draw_layer(document, id).map(draw_play_layers_tree_row_id)).collect();
    let highlighted_ids: Vec<String> = interaction.hovered_id.as_ref().and_then(|id| find_draw_layer(document, id).map(draw_play_layers_tree_row_id)).into_iter().collect();
    let builder = PanelTreeBuilder::new("draw-play-layers")
        .section("draw-play-layers", Some(Label::data(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)), true, action_items.into_iter().chain(layer_items).collect())
        .selected(selected_tree_ids)
        .selection_change(draw_play_action("setSelection", None));
    if highlighted_ids.is_empty() {
        builder.build()
    } else {
        builder.highlighted(highlighted_ids).build()
    }
}

fn tree_button(id: &str, label: impl Into<Label>, icon: &str, action: &str, args: Value) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon.into()), menu: None, ..tree_item_with_action(id, label, None, draw_play_action(action, Some(args))) }
}

fn render_catalogue_panel(_document: &DrawDocument, interaction: &DrawConfig, labels: &DrawPlayLabels) -> UiNode {
    let catalogue_kinds = [
        ("path", labels.kind_path, "pen-tool"),
        ("shape:rect", labels.kind_rectangle, "square"),
        ("shape:ellipse", labels.kind_ellipse, "circle"),
        ("shape:line", labels.kind_line, "minus"),
        ("shape:polygon", labels.kind_polygon, "hexagon"),
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
            UiTreeItemNode { icon_id: Some(icon.into()), draggable: Some(true), drag_data: Some(drag_data), ..tree_item(format!("draw-play-catalogue.{kind}"), label) }
        })
        .collect();
    for operation in DRAW_BOOLEAN_OPERATIONS {
        items.push(UiTreeItemNode {
            icon_id: Some("combine".into()),
            ..tree_item_with_action(
                format!("draw-play-catalogue.bool.{operation}"),
                Label::data(format!("{} {operation}", labels.kind_boolean.as_str())),
                None,
                draw_play_action("combineBoolean", Some(json!({ "operation": operation, "ids": interaction.selected_ids }))),
            )
        });
    }
    PanelTreeBuilder::new("draw-play-catalogue").section("draw-play-catalogue", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, items).build()
}

fn inspector_patch(layer_ids: &[String], field: &str) -> ActionDescriptor {
    draw_play_action("patchLayers", Some(json!({ "layerIds": layer_ids, "field": field })))
}

fn inspector_number_field(layer_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
            commit: None,
            on_change: inspector_patch(layer_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

fn inspector_text_field(layer_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: None,
            on_change: inspector_patch(layer_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
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

fn inspector_kind_group(doc: &DrawDocument, layers: &[&draw::DrawLayerNode], labels: &DrawPlayLabels) -> Option<UiInspectorFieldGroup> {
    let uniform = uniform_layers(layers)?;
    let layer = uniform[0];
    let layer_ids: Vec<String> = uniform.iter().map(|entry| layer_id(entry).to_string()).collect();
    let mut fields: Vec<UiNode> = Vec::new();
    match layer {
        draw::DrawLayerNode::Boolean(boolean) => {
            let operations: Vec<String> = uniform
                .iter()
                .filter_map(|entry| match entry {
                    draw::DrawLayerNode::Boolean(entry) => Some(entry.operation.clone()),
                    _ => None,
                })
                .collect();
            let op_mixed = ui_inspector_mixed_select(&operations);
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.boolean-operation".into(),
                label: labels.boolean_operation.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.boolean-operation.select".into(),
                    value: op_mixed.value,
                    placeholder: op_mixed.placeholder.map(Label::data),
                    items: DRAW_BOOLEAN_OPERATIONS.iter().map(|operation| UiSelectItem { value: (*operation).into(), label: Label::data(*operation) }).collect(),
                    on_change: inspector_patch(&layer_ids, "booleanOperation"),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
            let child_labels = boolean.children.iter().filter_map(|child_id| find_draw_layer(doc, child_id).map(|child| layer_base(child).name.clone())).collect::<Vec<_>>().join(", ");
            fields.push(ui_inspector_readonly_field("draw-play-inspector.boolean-children", labels.children, if child_labels.is_empty() { "—".into() } else { child_labels }));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.boolean".into(), label: labels.kind_boolean.into(), default_open: None, fields });
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
                presence: UiPresence::default(),
                id: "draw-play-inspector.trace-threshold".into(),
                label: labels.trace_threshold.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.trace-threshold.slider".into(),
                    value: if threshold_mixed.uniform { threshold_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "traceThreshold"),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.trace-simplify".into(),
                label: labels.simplify.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.trace-simplify.slider".into(),
                    value: if simplify_mixed.uniform { simplify_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    on_change: inspector_patch(&layer_ids, "traceSimplify"),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }));
            fields.push(ui_inspector_readonly_field("draw-play-inspector.trace-source", labels.source_key, trace.source_key.clone()));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.trace".into(), label: labels.kind_trace.into(), default_open: None, fields });
        }
        draw::DrawLayerNode::Shape(shape) if shape.shape_kind == "rect" => {
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-width",
                labels.width,
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        draw::DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.width),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "rectWidth",
            ));
            fields.push(inspector_number_field(
                &layer_ids,
                "draw-play-inspector.rect-height",
                labels.height,
                &uniform
                    .iter()
                    .filter_map(|entry| match entry {
                        draw::DrawLayerNode::Shape(entry) => entry.rect.as_ref().map(|rect| rect.height),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                "rectHeight",
            ));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.rect".into(), label: labels.kind_rectangle.into(), default_open: None, fields });
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
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.text".into(), label: labels.kind_text.into(), default_open: None, fields });
        }
        draw::DrawLayerNode::Path(path) => {
            fields.push(ui_inspector_readonly_field("draw-play-inspector.path-segments", labels.segment_count, path.segments.len().to_string()));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.path".into(), label: labels.kind_path.into(), default_open: None, fields });
        }
        draw::DrawLayerNode::Group(group) => {
            fields.push(ui_inspector_readonly_field("draw-play-inspector.group-children", labels.children_count, group.children.len().to_string()));
            return Some(UiInspectorFieldGroup { presence: UiPresence::default(), id: "draw-play-inspector.kind.group".into(), label: labels.kind_group.into(), default_open: None, fields });
        }
        _ => {}
    }
    None
}

fn inspector_appearance_group(layers: &[&draw::DrawLayerNode], labels: &DrawPlayLabels) -> UiInspectorFieldGroup {
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
    let stroke_widths: Vec<f64> = layers.iter().map(|entry| layer_base(entry).attributes.stroke.as_ref().map(|stroke| stroke.width).unwrap_or(1.0)).collect();
    let fill_alpha_mixed = ui_inspector_mixed_slider(&fill_alphas);
    UiInspectorFieldGroup {
        id: "draw-play-inspector.appearance".into(),
        label: labels.appearance.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.fill", labels.fill, &fill_colors, "fillColor"),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.fill-alpha".into(),
                label: labels.fill_alpha.into(),
                child: Box::new(UiNode::Slider(UiSliderNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.fill-alpha.slider".into(),
                    value: if fill_alpha_mixed.uniform { fill_alpha_mixed.value } else { 0.0 },
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: inspector_patch(&layer_ids, "fillAlpha"),
                    unit: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            inspector_number_field(&layer_ids, "draw-play-inspector.stroke-width", labels.stroke_width, &stroke_widths, "strokeWidth"),
        ],
    }
}

fn inspector_layer_group(layers: &[&draw::DrawLayerNode], labels: &DrawPlayLabels) -> UiInspectorFieldGroup {
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
        presence: UiPresence::default(),
        fields: vec![
            inspector_text_field(&layer_ids, "draw-play-inspector.name", labels.name, &names, "name"),
            inspector_number_field(&layer_ids, "draw-play-inspector.opacity", labels.opacity, &opacities, "opacity"),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.blend-mode".into(),
                label: labels.blend_mode.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "draw-play-inspector.blend-mode.select".into(),
                    value: blend_mixed.value,
                    placeholder: blend_mixed.placeholder.map(Label::data),
                    items: DRAW_BLEND_MODES.iter().map(|mode| UiSelectItem { value: (*mode).into(), label: Label::data(*mode) }).collect(),
                    on_change: inspector_patch(&layer_ids, "blendMode"),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.visible".into(),
                label: labels.visible.into(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.visible.toggle".into(),
                    icon_id: "eye".into(),
                    text: None,
                    on_change: inspector_patch(&layer_ids, "visible"),
                    presence: UiPresence::selected(visible_mixed.uniform && visible_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "draw-play-inspector.locked".into(),
                label: labels.locked.into(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: "draw-play-inspector.locked.toggle".into(),
                    icon_id: "lock".into(),
                    text: None,
                    on_change: inspector_patch(&layer_ids, "locked"),
                    presence: UiPresence::selected(locked_mixed.uniform && locked_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ],
    }
}

fn inspector_orientation_group(layers: &[&draw::DrawLayerNode], labels: &DrawPlayLabels) -> UiInspectorFieldGroup {
    let layer_ids: Vec<String> = layers.iter().map(|entry| layer_id(entry).to_string()).collect();
    UiInspectorFieldGroup {
        id: "draw-play-inspector.orientation".into(),
        label: labels.orientation.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-x", labels.position_x, &layers.iter().map(|entry| layer_base(entry).transform.x).collect::<Vec<_>>(), "transformX"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-y", labels.position_y, &layers.iter().map(|entry| layer_base(entry).transform.y).collect::<Vec<_>>(), "transformY"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-scale-x", labels.scale_x, &layers.iter().map(|entry| layer_base(entry).transform.scale_x).collect::<Vec<_>>(), "transformScaleX"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-scale-y", labels.scale_y, &layers.iter().map(|entry| layer_base(entry).transform.scale_y).collect::<Vec<_>>(), "transformScaleY"),
            inspector_number_field(&layer_ids, "draw-play-inspector.transform-rotation", labels.rotation, &layers.iter().map(|entry| layer_base(entry).transform.rotation).collect::<Vec<_>>(), "transformRotation"),
        ],
    }
}

fn render_properties_panel(document: &DrawDocument, interaction: &DrawConfig, labels: &DrawPlayLabels, active_utility: &str) -> UiNode {
    let selected_layers: Vec<&draw::DrawLayerNode> = interaction.selected_ids.iter().filter_map(|id| find_draw_layer(document, id)).collect();
    if selected_layers.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(Label::data(format!("Schema: {}", DRAW_DOCUMENT_SCHEMA))),
            ui_text(Label::data(format!("Utility: {active_utility}"))),
            ui_text(Label::data(format!("Layers: {}", flatten_draw_layers(&document.layers).len()))),
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
//#endregion 🔖️Panels

//#region 🔖️Render
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
const DRAW_ARTBOARD_FILL: [f64; 4] = [0.969, 0.953, 0.890, 1.0];
const DRAW_ARTBOARD_STROKE: [f64; 4] = [0.198, 0.223, 0.205, 0.55];
const DRAW_ARTBOARD_LABEL: [f64; 4] = [0.198, 0.223, 0.205, 0.92];

/// 📐️ Formats one artboard edge length for the dimension label (integers stay bare).
fn format_artboard_dimension(value: f64) -> String {
    if (value - value.round()).abs() < 1e-6 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.2}")
    }
}

/// 🖼️ Artboard paper + `W × H` dimension label — drawn under document content.
fn artboard_scene_records(document: &DrawDocument) -> Vec<Value> {
    let artboard = resolve_draw_artboard(document).unwrap_or(DrawArtboard { width: 1024.0, height: 1024.0 });
    let width = artboard.width.max(1.0);
    let height = artboard.height.max(1.0);
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [width, 0.0] }, PathSegment::Line { to: [width, height] }, PathSegment::Line { to: [0.0, height] }, PathSegment::Close];
    let label = format!("{} × {}", format_artboard_dimension(width), format_artboard_dimension(height));
    let label_size = 12.0_f64;
    let label_x = (width * 0.5) - (label.len() as f64 * label_size * 0.28);
    vec![
        overlay_record("artboard:frame".into(), [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_ARTBOARD_FILL), DRAW_ARTBOARD_STROKE, 1.0),
        json!({
            "id": "artboard:dimensions",
            "role": "overlay",
            "transform": [1.0, 0.0, 0.0, 1.0, label_x, height + label_size * 0.35],
            "segments": [],
            "fill": { "kind": "solid", "color": DRAW_ARTBOARD_LABEL },
            "opacity": 1.0,
            "blendMode": "normal",
            "visible": true,
            "text": { "content": label, "size": label_size },
        }),
    ]
}

fn render_canvas(document: &DrawDocument, interaction: &DrawConfig, gesture: &draw_gesture::Snapshot, active_utility: &str) -> UiNode {
    let scene_nodes = flatten_draw_document_to_scene_nodes(document);
    let artboard_records = artboard_scene_records(document);
    let mut records: Vec<Value> = Vec::with_capacity(scene_nodes.len() + artboard_records.len() + 4);
    records.push(json!({
        "id": "meta:utility",
        "role": "meta",
        "utility": active_utility,
    }));
    records.extend(artboard_records);
    for node in &scene_nodes {
        records.push(serde_json::to_value(node).unwrap_or(Value::Null));
    }
    let node_by_id: HashMap<&str, &draw_engine::DrawSceneNode> = scene_nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let selected_leaf_ids: Vec<String> = interaction.selected_ids.iter().filter_map(|id| find_draw_layer(document, id)).flat_map(draw_layer_descendant_leaf_ids).collect();
    for leaf_id in &selected_leaf_ids {
        if let Some(node) = node_by_id.get(leaf_id.as_str()) {
            records.push(overlay_record(format!("overlay:sel:{leaf_id}"), node.transform, &node.segments, Some(DRAW_OVERLAY_SELECTION_FILL), DRAW_OVERLAY_SELECTION_STROKE, 2.0));
        }
    }
    if let Some(hovered_id) = &interaction.hovered_id {
        if !selected_leaf_ids.iter().any(|id| id == hovered_id) {
            if let Some(layer) = find_draw_layer(document, hovered_id) {
                for leaf_id in draw_layer_descendant_leaf_ids(layer) {
                    if let Some(node) = node_by_id.get(leaf_id.as_str()) {
                        records.push(overlay_record(format!("overlay:hover:{leaf_id}"), node.transform, &node.segments, None, DRAW_OVERLAY_HOVER_STROKE, 1.5));
                    }
                }
            }
        }
    }
    if gesture.matches("marqueeing") {
        let ctx = &gesture.context;
        let x = ctx.start[0].min(ctx.cursor[0]);
        let y = ctx.start[1].min(ctx.cursor[1]);
        let width = (ctx.cursor[0] - ctx.start[0]).abs();
        let height = (ctx.cursor[1] - ctx.start[1]).abs();
        let segments = vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close];
        records.push(overlay_record("overlay:marquee".into(), [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_OVERLAY_MARQUEE_FILL), DRAW_OVERLAY_MARQUEE_STROKE, 1.0));
    } else if gesture.matches("shape_dragging") {
        let ctx = &gesture.context;
        let segments = shape_preview_segments(&ctx.utility, ctx.start, ctx.cursor);
        records.push(overlay_record("overlay:preview".into(), [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_OVERLAY_SELECTION_FILL), DRAW_OVERLAY_SELECTION_STROKE, 1.5));
    } else if gesture.matches("drafting") {
        let ctx = &gesture.context;
        let segments = draft_preview_segments(&ctx.utility, &ctx.points, ctx.cursor);
        records.push(overlay_record("overlay:preview".into(), [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &segments, Some(DRAW_OVERLAY_SELECTION_FILL), DRAW_OVERLAY_SELECTION_STROKE, 1.5));
    }
    build_canvas_2d_scene(
        DRAW_PLAY_SURFACE_ID,
        DRAW_PLAY_CONTROLLER_ID,
        Canvas2dScene { camera_x: interaction.camera.x, camera_y: interaction.camera.y, zoom: interaction.camera.zoom, layers_json: serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()) },
    )
}
//#endregion 🔖️Render

//#region 🔖️DrawPlayApp
/// 🧪️ B1: unit-shaped struct — every former `DrawConfig` field (selection, hover, camera, engagement
/// draft, active utility, locale) now lives in `draw_engine::DrawConfig` (see `DocumentApp::Config`),
/// written through `draw_op::DrawConfigOperation`s. `gesture`/`preview_seq` stay app-runtime `RefCell`
/// scratch — genuinely ephemeral, never undo-tracked, matching the "scratch + commit" preview pattern
/// (`Emit::commit`'s doc, `UtilityPreviewContract`).
pub struct DrawPlayApp {
    /// 🎭️ Live `fsm` snapshot driving pointer gestures — see `//#region 🔖️GestureMachine`.
    gesture: RefCell<draw_gesture::Snapshot>,
    /// 👻️ Per-`key` monotone counter for `gesture_preview` — see `//#region 🔖️GesturePreview`.
    preview_seq: RefCell<u64>,
}

impl Default for DrawPlayApp {
    fn default() -> Self {
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        Self { gesture: RefCell::new(fsm::init::<draw_gesture::DrawGesture>((), &mut sink)), preview_seq: RefCell::new(0) }
    }
}

impl DrawPlayApp {
    /// 🎭️ Feeds one gesture event through the shared `fsm` statechart, then drains and executes any
    /// requested `GestureEffect`s against the live document — the only place gesture control-flow
    /// (owned by `fsm`) meets document-mutating logic (owned by `draw`). `config` is the caller's
    /// working copy (mutated in place for selection changes the gesture makes); the returned `Emit`
    /// carries only DOCUMENT operations (shape/draft/trace commits) — the caller (`handle`) diffs
    /// `config` before/after via `finish_gesture_emit` to fold in any selection change.
    fn step_gesture(&self, event: draw_gesture::Event, document: &DrawDocument, config: &mut DrawConfig) -> Emit<DrawOperation, DrawConfigOperation> {
        let mut sink: Vec<fsm::Command<draw_gesture::DrawGesture>> = Vec::new();
        fsm::macrostep(&mut *self.gesture.borrow_mut(), event, &mut sink, &mut fsm::NullInspector);
        let next_seq = self.preview_seq.borrow().wrapping_add(1);
        *self.preview_seq.borrow_mut() = next_seq;
        let mut operations = Vec::new();
        let mut commit_description: Option<&'static str> = None;
        for command in sink {
            let fsm::Command::Effect(effect) = command else { continue };
            match effect {
                GestureEffect::CommitMarquee { start, end, active, merge, shift, ctrl, meta } => {
                    if active {
                        let crossing = end[0] < start[0];
                        let hits = marquee_layer_hits(document, start, end, crossing);
                        config.selected_ids = merge_selection(&merge, &config.selected_ids, &hits);
                    } else {
                        apply_point_pick(config, document, end, shift, ctrl, meta, false);
                    }
                }
                GestureEffect::CommitShape { utility, start, end } => {
                    operations.extend(commit_shape_drag(config, document, &utility, start, end));
                    commit_description = Some("Add shape");
                }
                GestureEffect::CommitDraft { utility, points } => {
                    operations.extend(commit_draft(config, document, &utility, &points));
                    commit_description = Some("Commit draft");
                }
                GestureEffect::CommitTrace { world } => {
                    operations.extend(commit_trace_at(config, document, world));
                    commit_description = Some("Trace image");
                }
                GestureEffect::PickPoint { world, shift, ctrl, meta } => {
                    apply_point_pick(config, document, world, shift, ctrl, meta, true);
                }
            }
        }
        match commit_description {
            Some(description) => commit_with_utility_reset(operations, description),
            None => Emit::default(),
        }
    }

    /// 👻️ See `//#region 🔖️GesturePreview` — the `(key, seq, payload)` tuple already shaped as
    /// `SyncSession::publish_preview`'s exact argument list once a host bridge can carry it out of
    /// this sandboxed plugin. `#[allow(dead_code)]`: no caller exists inside this crate today (see the
    /// doc on `draw_gesture_preview_payload`); exercised by `🧪️Tests` only until that bridge lands.
    #[allow(dead_code)]
    fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let gesture = self.gesture.borrow();
        let payload = draw_gesture_preview_payload(&gesture.context, gesture.matches("idle"))?;
        Some(("gesture", *self.preview_seq.borrow(), serde_json::to_vec(&payload).ok()?))
    }
}

impl DocumentApp for DrawPlayApp {
    type Projection = DrawDocument;
    type Operation = DrawOperation;
    type Config = DrawConfig;
    type ConfigOperation = DrawConfigOperation;
    type Command = DrawCommand;

    fn app_id(&self) -> &str {
        DRAW_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        DRAW_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> DrawDocument {
        default_draw_document("empty", None)
    }

    fn io(&self) -> Option<AppIo> {
        Some(draw_io())
    }

    /// 🎞️ `vector:out` (see `draw_engine::draw_vector_media`) plus the inherited `document:out`
    /// default (the pack of `doc.projection`, replicated inline — overriding `export_media` shadows
    /// the trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, DrawDocument>) -> Result<Media, MediaError> {
        match port {
            "vector:out" => draw_vector_media(doc.projection),
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(&self, projection: DrawDocument) -> Option<DrawOperation> {
        Some(DrawOperation::SetDocument { document: projection })
    }

    /// 🏷️ Maps each `DrawCommand` variant back to the action id it was declared under in
    /// `create_draw_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &DrawCommand) -> &str {
        match command {
            DrawCommand::SetDocument { .. } => "setDocument",
            DrawCommand::CommitDocument { .. } => "commitDocument",
            DrawCommand::SetFixtureJson { .. } => "setFixtureJson",
            DrawCommand::SetActiveExample { .. } => "setActiveExample",
            DrawCommand::SetSelectedOpacity { .. } => "setSelectedOpacity",
            DrawCommand::EngagementSubmit { .. } => "engagementSubmit",
            DrawCommand::AddLayer { .. } => "addLayer",
            DrawCommand::DropLayerKind { .. } => "dropLayerKind",
            DrawCommand::MoveLayer { .. } => "moveLayer",
            DrawCommand::DeleteLayer { .. } => "deleteLayer",
            DrawCommand::DuplicateLayer { .. } => "duplicateLayer",
            DrawCommand::ToggleLayerVisible { .. } => "toggleLayerVisible",
            DrawCommand::CombineBoolean { .. } => "combineBoolean",
            DrawCommand::PatchLayer { .. } => "patchLayer",
            DrawCommand::PatchLayers { .. } => "patchLayers",
            DrawCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
            DrawCommand::SetCamera { .. } => "setCamera",
            DrawCommand::SetCameraZoom { .. } => "setCameraZoom",
            DrawCommand::SetSelection { .. } => "setSelection",
            DrawCommand::SetHover { .. } => "setHover",
            DrawCommand::SelectAll => "selectAll",
            DrawCommand::ClearSelection => "clearSelection",
            DrawCommand::EngagementInput { .. } => "engagementInput",
            DrawCommand::SetLocale { .. } => "setLocale",
            DrawCommand::CanvasPointerDown { .. } => "canvasPointerDown",
            DrawCommand::CanvasPointerMove { .. } => "canvasPointerMove",
            DrawCommand::CanvasPointerUp { .. } => "canvasPointerUp",
            DrawCommand::CanvasDoubleClick => "canvasDoubleClick",
            DrawCommand::CanvasCommitDraft => "canvasCommitDraft",
            DrawCommand::CanvasEscape => "canvasEscape",
        }
    }

    fn handle(&self, command: &DrawCommand, doc: &DocumentView<'_, DrawDocument>, cfg: &ConfigView<'_, DrawConfig>) -> Emit<DrawOperation, DrawConfigOperation> {
        let document = doc.projection;
        let mut config = cfg.projection.clone();
        match command {
            //#region 🔖️ContentOperations
            DrawCommand::SetDocument { document: next } | DrawCommand::CommitDocument { document: next } => Emit::operations(vec![DrawOperation::SetDocument { document: next.clone() }]),
            DrawCommand::SetFixtureJson { json } => {
                if json.contains(DRAW_DOCUMENT_SCHEMA) {
                    if let Ok(document) = serde_json::from_str::<DrawDocument>(json) {
                        return Emit::operations(vec![DrawOperation::SetDocument { document }]);
                    }
                }
                Emit::default()
            }
            DrawCommand::SetActiveExample { example_id } => {
                let next = if example_id.is_empty() {
                    Some(default_draw_document("empty", None))
                } else if example_id == DRAW_PLAY_EXAMPLE_DEFAULT_ID {
                    Some(semio_draw_example_document())
                } else {
                    None
                };
                match next {
                    Some(document) => Emit { document_operations: vec![DrawOperation::SetDocument { document }], config_operations: vec![DrawConfigOperation::SetSelection { ids: Vec::new() }], ..Default::default() },
                    None => Emit::default(),
                }
            }
            DrawCommand::SetSelectedOpacity { value } => {
                let operations: Vec<DrawOperation> = config.selected_ids.iter().filter(|id| find_draw_layer(document, id).is_some()).map(|id| DrawOperation::SetLayerOpacity { layer_id: id.clone(), opacity: *value }).collect();
                if operations.is_empty() {
                    return Emit::default();
                }
                Emit::amend(operations, "opacity")
            }
            DrawCommand::EngagementSubmit { value } => {
                let value = value.clone().unwrap_or_else(|| config.engagement_input.clone());
                let value = value.trim();
                if value.is_empty() || config.selected_ids.len() != 1 {
                    return Emit::default();
                }
                Emit::operations(vec![DrawOperation::SetLayerName { layer_id: config.selected_ids[0].clone(), name: value.into() }])
            }
            DrawCommand::AddLayer { kind } => {
                let layer = create_layer_by_kind(kind);
                let select_id = layer_id(&layer).to_string();
                Emit {
                    document_operations: vec![DrawOperation::AddLayer { parent_id: None, index: Some(document.layers.len()), layer: Box::new(layer) }],
                    config_operations: vec![DrawConfigOperation::SetSelection { ids: vec![select_id] }],
                    ..Default::default()
                }
            }
            DrawCommand::DropLayerKind { kind, target_row_id, drop_position } => {
                let layer = create_layer_by_kind(kind);
                let (parent_id, index) = resolve_reorder_target(document, target_row_id, drop_position);
                let select_id = layer_id(&layer).to_string();
                Emit { document_operations: vec![DrawOperation::AddLayer { parent_id, index: Some(index), layer: Box::new(layer) }], config_operations: vec![DrawConfigOperation::SetSelection { ids: vec![select_id] }], ..Default::default() }
            }
            DrawCommand::MoveLayer { layer_id: target_id, target_row_id, drop_position } => {
                let (parent_id, index) = resolve_reorder_target(document, target_row_id, drop_position);
                Emit::operations(vec![DrawOperation::ReorderLayer { layer_id: target_id.clone(), parent_id, index }])
            }
            DrawCommand::DeleteLayer { layer_id: target_id } => {
                if target_id.is_empty() || find_draw_layer(document, target_id).is_none() {
                    return Emit::default();
                }
                let remaining: Vec<String> = config.selected_ids.iter().filter(|id| *id != target_id).cloned().collect();
                Emit { document_operations: vec![DrawOperation::RemoveLayer { layer_id: target_id.clone() }], config_operations: vec![DrawConfigOperation::SetSelection { ids: remaining }], ..Default::default() }
            }
            DrawCommand::DuplicateLayer { layer_id: target_id } => {
                if target_id.is_empty() {
                    return Emit::default();
                }
                Emit::operations(vec![DrawOperation::DuplicateLayer { layer_id: target_id.clone() }])
            }
            DrawCommand::ToggleLayerVisible { layer_id: target_id } => match find_draw_layer(document, target_id) {
                Some(layer) => {
                    let visible = !layer_base(layer).visible;
                    Emit::operations(vec![DrawOperation::SetLayerVisible { layer_id: target_id.clone(), visible }])
                }
                None => Emit::default(),
            },
            DrawCommand::CombineBoolean { operation, ids } => {
                let ids: Vec<String> = if ids.is_empty() { config.selected_ids.clone() } else { ids.clone() };
                if ids.len() < 2 {
                    return Emit::default();
                }
                let layer = create_draw_boolean_layer("Boolean", operation, ids);
                let select_id = layer_id(&layer).to_string();
                Emit {
                    document_operations: vec![DrawOperation::AddLayer { parent_id: None, index: Some(document.layers.len()), layer: Box::new(layer) }],
                    config_operations: vec![DrawConfigOperation::SetSelection { ids: vec![select_id] }],
                    ..Default::default()
                }
            }
            DrawCommand::PatchLayer { layer_id: target_id, field, value } => {
                let json_value = patch_value_json(value);
                match draw_op_for_layer_field(document, target_id, field, &json_value) {
                    Some(operation) => Emit::operations(vec![operation]),
                    None => Emit::default(),
                }
            }
            DrawCommand::PatchLayers { layer_ids, field, value } => {
                let json_value = patch_value_json(value);
                let operations: Vec<DrawOperation> = layer_ids.iter().filter_map(|id| draw_op_for_layer_field(document, id, field, &json_value)).collect();
                if operations.is_empty() {
                    return Emit::default();
                }
                Emit::operations(operations)
            }
            //#endregion 🔖️ContentOperations
            //#region 🔖️ConfigOnly
            DrawCommand::SetActiveUtility { utility_id } => {
                // 🧰️ Host-owned utility switch: clear any in-progress gesture scratch (discarding any
                // document-op the FSM would produce — `UtilityChanged` never carries one).
                self.step_gesture(draw_gesture::Event::UtilityChanged, document, &mut config);
                Emit::config(vec![DrawConfigOperation::SetActiveUtility { utility_id: utility_id.clone() }])
            }
            // 📷️ Camera — session-only runtime pose, never a document operation.
            DrawCommand::SetCamera { camera } => Emit::config(vec![DrawConfigOperation::SetCamera { camera: camera.clone() }]),
            DrawCommand::SetCameraZoom { value } => {
                let camera = draw::DrawCamera { zoom: *value, ..config.camera };
                Emit::config(vec![DrawConfigOperation::SetCamera { camera }])
            }
            DrawCommand::SetSelection { ids } => Emit::config(vec![DrawConfigOperation::SetSelection { ids: ids.clone() }]),
            DrawCommand::SetHover { id } => Emit::config(vec![DrawConfigOperation::SetHovered { id: id.clone() }]),
            DrawCommand::SelectAll => {
                let ids = flatten_draw_layers(&document.layers).into_iter().map(|layer| layer_id(layer).to_string()).collect();
                Emit::config(vec![DrawConfigOperation::SetSelection { ids }])
            }
            DrawCommand::ClearSelection => Emit::config(vec![DrawConfigOperation::SetSelection { ids: Vec::new() }]),
            DrawCommand::EngagementInput { value } => Emit::config(vec![DrawConfigOperation::SetEngagementInput { value: value.clone() }]),
            DrawCommand::SetLocale { value } => Emit::config(vec![DrawConfigOperation::SetLocale { value: value.clone() }]),
            //#endregion 🔖️ConfigOnly
            //#region 🔖️CanvasGestures
            DrawCommand::CanvasPointerDown { x, y, width, height, shift, ctrl, meta } => {
                let (world_x, world_y) = canvas_point_to_world(&config.camera, *x, *y, *width, *height);
                let active_utility = config.active_utility_id.clone();
                let emit = self.step_gesture(draw_gesture::Event::PointerDown { utility: active_utility, world: [world_x, world_y], shift: *shift, ctrl: *ctrl, meta: *meta }, document, &mut config);
                finish_gesture_emit(emit, cfg.projection, &config)
            }
            DrawCommand::CanvasPointerMove { x, y, width, height } => {
                let (world_x, world_y) = canvas_point_to_world(&config.camera, *x, *y, *width, *height);
                let world = [world_x, world_y];
                if self.gesture.borrow().matches("idle") {
                    let include_control_points = config.active_utility_id == "selectDirect";
                    let tolerance = DRAW_PICK_TOLERANCE_PX / config.camera.zoom.max(1e-6);
                    let hovered_id = best_pick_layer_id(&resolve_pick_targets_at(document, world, tolerance, include_control_points));
                    if hovered_id == config.hovered_id {
                        return Emit::default();
                    }
                    return Emit::config(vec![DrawConfigOperation::SetHovered { id: hovered_id }]);
                }
                let marquee_threshold_world = DRAW_MARQUEE_THRESHOLD_PX / config.camera.zoom.max(1e-6);
                let emit = self.step_gesture(draw_gesture::Event::PointerMove { world, marquee_threshold_world }, document, &mut config);
                finish_gesture_emit(emit, cfg.projection, &config)
            }
            DrawCommand::CanvasPointerUp { x, y, width, height, shift, ctrl, meta } => {
                let (world_x, world_y) = canvas_point_to_world(&config.camera, *x, *y, *width, *height);
                let active_utility = config.active_utility_id.clone();
                let emit = self.step_gesture(draw_gesture::Event::PointerUp { utility: active_utility, world: [world_x, world_y], shift: *shift, ctrl: *ctrl, meta: *meta }, document, &mut config);
                finish_gesture_emit(emit, cfg.projection, &config)
            }
            DrawCommand::CanvasDoubleClick | DrawCommand::CanvasCommitDraft => {
                let emit = self.step_gesture(draw_gesture::Event::CommitDraft, document, &mut config);
                finish_gesture_emit(emit, cfg.projection, &config)
            }
            DrawCommand::CanvasEscape => {
                let emit = self.step_gesture(draw_gesture::Event::Escape, document, &mut config);
                finish_gesture_emit(emit, cfg.projection, &config)
            } //#endregion 🔖️CanvasGestures
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, DrawDocument>, cfg: &ConfigView<'_, DrawConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let gesture = self.gesture.borrow();
        let labels = resolve_labels::<DrawPlayLabels>(config);
        let active_utility = config.active_utility_id.as_str();
        match body_key {
            DRAW_PLAY_BODY_COMPOSITE => render_canvas(document, config, &gesture, active_utility),
            DRAW_PLAY_BODY_LAYERS => render_layers_panel(document, config, labels),
            DRAW_PLAY_BODY_CATALOGUE => render_catalogue_panel(document, config, labels),
            DRAW_PLAY_BODY_PROPERTIES => render_properties_panel(document, config, labels, active_utility),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️DrawPlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector-bound vocabulary
/// that is dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn draw_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn draw_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

pub fn create_draw_app() -> App {
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
        status: Some(vec![WindowEngagementStatus { id: "draw-layer-count".into(), text: "0 layers · 0 selected".into() }]),
        possible_engagements: None,
    };
    App::from_builder(
        App::builder(DRAW_PLAY_APP_ID, LocalizedLabel::native("Draw", "Zeichnen")).document(["semio", "draw"])
            .artifact_kind(ArtifactKindSpec {
                id: "2d.drawing".into(),
                name: "2D Drawing".into(),
                source_format: "draw.document".into(),
                component_kind: "draw".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                schema: "draw.document".into(),
                export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
            })
            .icon_id("draw")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(DRAW_PLAY_WINDOW_CANVAS, LocalizedLabel::native("Canvas", "Leinwand"), DRAW_PLAY_BODY_COMPOSITE, SurfaceKind::Canvas2d, engagement, "pen-tool")
            .panel_tab("framework.panel.document", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, DRAW_PLAY_BODY_LAYERS)
            .panel_tab("framework.panel.catalogue", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, DRAW_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, DRAW_PLAY_BODY_PROPERTIES)
            // ✏️ Palette-visible content operations.
            .operation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            .operation("combineBoolean", LocalizedLabel::native("Combine Boolean", "Boolean kombinieren"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🔧️ Internal content operations — inspector/layer-panel/import-bound, not palette commands.
            .action_with(draw_internal_action("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Operation))
            .action_with(draw_internal_action("commitDocument", LocalizedLabel::native("Commit Document", "Dokument übernehmen"), ActionKind::Operation))
            .action_with(draw_internal_action("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Operation))
            .action_with(draw_internal_action("setSelectedOpacity", LocalizedLabel::native("Set Selected Opacity", "Deckkraft der Auswahl festlegen"), ActionKind::Operation))
            .action_with(draw_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Operation))
            .action_with(draw_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Operation))
            .action_with(draw_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Operation))
            .action_with(draw_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Operation))
            .action_with(draw_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Operation))
            .action_with(draw_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Operation))
            .action_with(draw_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Operation))
            .action_with(draw_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Operation))
            // 🖱️ Internal pointer/gesture vocabulary — commit-time handlers emit operations, the rest are pure View.
            .action_with(draw_internal_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::Operation))
            .action_with(draw_internal_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::Operation))
            .action_with(draw_internal_action("canvasDoubleClick", LocalizedLabel::native("Canvas Double Click", "Leinwand-Doppelklick"), ActionKind::Operation))
            .action_with(draw_internal_action("canvasCommitDraft", LocalizedLabel::native("Canvas Commit Draft", "Leinwand-Entwurf übernehmen"), ActionKind::Operation))
            .action_with(draw_internal_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View))
            .action_with(draw_internal_action("canvasEscape", LocalizedLabel::native("Canvas Escape", "Leinwand abbrechen"), ActionKind::View))
            // 👁️ Ephemeral view state.
            .view_action("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .action_with(draw_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(draw_internal_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"), ActionKind::View))
            .action_with(draw_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(draw_internal_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), ActionKind::View))
            // 📷️ Camera — session-only runtime pose, never a document operation.
            .action_with(draw_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(draw_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            // 🧰️ Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(draw_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(draw_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", "Select", UtilityCategory::Selection))
            .utility(draw_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", "Select", UtilityCategory::Selection))
            .utility(draw_utility("pen", LocalizedLabel::native("Pen", "Stift"), "pen-tool", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeRect", LocalizedLabel::native("Rectangle", "Rechteck"), "rectangle-tool", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeEllipse", LocalizedLabel::native("Ellipse", "Ellipse"), "circle", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeLine", LocalizedLabel::native("Line", "Linie"), "minus", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapePolygon", LocalizedLabel::native("Polygon", "Polygon"), "hexagon", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("booleanCombine", LocalizedLabel::native("Boolean", "Boolean"), "combine", "Combine", UtilityCategory::Utilities))
            .utility(draw_utility("trace", LocalizedLabel::native("Trace", "Nachzeichnen"), "scan-line", "Combine", UtilityCategory::Utilities))
            .utility(draw_utility("transformMove", LocalizedLabel::native("Pan", "Verschieben"), "move", "View", UtilityCategory::Utilities))
            .window_kind_utilities(DRAW_PLAY_WINDOW_CANVAS, vec![
                "selectMarquee".into(), "selectLasso".into(), "selectDirect".into(),
                "pen".into(), "shapeRect".into(), "shapeEllipse".into(), "shapeLine".into(), "shapePolygon".into(),
                "booleanCombine".into(), "trace".into(), "transformMove".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+a", "selectAll")
            .keybinding("escape", "canvasEscape")
            .keybinding("enter", "canvasCommitDraft")
            .default_layout(create_default_layout(
                &[DRAW_PLAY_WINDOW_CANVAS.into()],
                "row",
                Some(&[100.0]),
                Some(&["Canvas".into()]),
            )),
    )
    .example(DRAW_PLAY_EXAMPLE_DEFAULT_ID, LocalizedLabel::native("Semio", "Semio"), semio_draw_example_json(), "sparkles")
    .workflow("draw", "Draw", "2d.drawing")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
/// 🌉️ Generic `DocumentStore` aliases used only by the WASM bridge below — see the recipe's
/// circular-dependency note: these need both `DrawDocument` (`draw`) and `DrawOperation` (`draw_op`),
/// which is exactly the dependency shape `ui` already has (unlike `engine`, which `draw_op` itself
/// depends on and therefore can never depend back on `draw_op`).
pub type DrawEnvelope = store::DocumentEnvelope<DrawDocument, DrawOperation>;
pub type DrawStore = store::DocumentStore<DrawDocument, DrawOperation>;

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DrawDocumentVcs {
        store: RefCell<DrawStore>,
    }

    #[wasm_bindgen]
    impl DrawDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<DrawDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: DrawEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    DrawStore::new(envelope)
                }
                None => DrawStore::new(create_document_envelope(DRAW_DOCUMENT_SCHEMA, "draw", empty_draw_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use draw_engine::create_draw_shape_layer_rect;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, ViewState};

    fn new_app() -> VcsDocumentApp<DrawPlayApp> {
        testkit::new_app::<DrawPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    fn new_app_with_registry() -> VcsDocumentApp<DrawPlayApp> {
        testkit::new_app_with_registry::<DrawPlayApp>(create_draw_app)
    }

    /// 🧰️ Sets the config's host-owned active utility to `utility` (replaces the deleted
    /// `ViewState.active_utility_id` seam — B1 moved it onto `DrawConfig`).
    fn set_utility(app: &mut VcsDocumentApp<DrawPlayApp>, utility: &str) {
        app.dispatch_typed(DrawCommand::SetActiveUtility { utility_id: utility.into() }, &testkit::meta("local")).expect("set active utility");
    }

    fn first_layer_id(app: &VcsDocumentApp<DrawPlayApp>) -> String {
        layer_id(&app.projection().expect("materialize projection").layers[0]).to_string()
    }

    fn last_layer_id(app: &VcsDocumentApp<DrawPlayApp>) -> String {
        let projection = app.projection().expect("materialize projection");
        layer_id(projection.layers.last().expect("layer")).to_string()
    }

    #[test]
    fn renders_canvas_scene_with_segments() {
        let mut app = new_app();
        let example_json = semio_draw_example_json();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, Some(example_json.as_str()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
        let value = serde_json::to_value(&node).unwrap();
        let layers_json = value.pointer("/canvas2d/layersJson").and_then(|v| v.as_str()).expect("layersJson string");
        assert!(layers_json.contains("segments"));
        let records: Vec<Value> = serde_json::from_str(layers_json).unwrap();
        assert!(records.iter().any(|record| record.get("role").and_then(|value| value.as_str()) == Some("meta")));
        assert!(records.iter().any(|record| record.get("id").and_then(|value| value.as_str()) == Some("artboard:frame")), "canvas must show the document artboard frame");
        assert!(
            records.iter().any(|record| { record.get("id").and_then(|value| value.as_str()) == Some("artboard:dimensions") && record.pointer("/text/content").and_then(|value| value.as_str()).is_some_and(|label| label.contains('×')) }),
            "canvas must show document dimension label"
        );
        assert!(layers_json.contains("200 × 200"), "example artboard dimensions must be visible");
    }

    #[test]
    fn default_document_exposes_artboard_dimensions_on_canvas() {
        let mut app = new_app();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let value = serde_json::to_value(&node).unwrap();
        let layers_json = value.pointer("/canvas2d/layersJson").and_then(|v| v.as_str()).expect("layersJson string");
        assert!(layers_json.contains("1024 × 1024"), "blank documents show default artboard dimensions");
    }

    #[test]
    fn layers_panel_lists_default_layer() {
        let mut app = new_app();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-layers.add.path"));
        assert!(json.contains("Layer 1"));
    }

    #[test]
    fn catalogue_panel_lists_boolean_operations() {
        let mut app = new_app();
        let node = app.render(DRAW_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-catalogue.path"));
        assert!(json.contains("Boolean union"));
    }

    #[test]
    fn add_layer_action_emits_op_and_appends_path() {
        let mut app = new_app();
        let before = app.projection().unwrap().layers.len();
        let result = app.dispatch_typed(DrawCommand::AddLayer { kind: "shape:rect".into() }, &testkit::meta("local")).expect("add layer");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().unwrap();
        assert_eq!(projection.layers.len(), before + 1);
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    }

    #[test]
    fn patch_layers_opacity_emits_granular_operation() {
        let mut app = new_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::PatchLayers { layer_ids: vec![id], field: "opacity".into(), value: "0.5".into() }, &testkit::meta("local")).expect("patch");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().unwrap();
        assert!((layer_base(&projection.layers[0]).opacity - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn patch_layer_name_emits_op_and_changes_projection() {
        let mut app = new_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::PatchLayer { layer_id: id, field: "name".into(), value: "Renamed".into() }, &testkit::meta("local")).expect("patch");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(layer_base(&app.projection().unwrap().layers[0]).name, "Renamed");
    }

    #[test]
    fn set_selection_view_action_emits_no_ops_and_drives_inspector() {
        let mut app = new_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::SetSelection { ids: vec![id] }, &testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "selection is ephemeral view state, not a document operation");
        let node = app.render(DRAW_PLAY_BODY_PROPERTIES, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Orientation"));
        assert!(json.contains("Position X"));
    }

    #[test]
    fn set_active_utility_clears_scratch_and_emits_no_history_entry() {
        let mut app = new_app_with_registry();
        set_utility(&mut app, "shapeRect");
        // Begin a shape gesture so there is scratch to clear.
        app.dispatch_typed(DrawCommand::CanvasPointerDown { x: 10.0, y: 10.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("down");
        let before = app.projection().unwrap();
        // Switching utilities is the framework View action: no document operations, nothing to sync/undo.
        let result = app.dispatch_typed(DrawCommand::SetActiveUtility { utility_id: "pen".into() }, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.projection().unwrap(), before, "utility switching does not mutate the document");
        // The cleared scratch means a follow-up pointer up commits nothing.
        let up = app.dispatch_typed(DrawCommand::CanvasPointerUp { x: 40.0, y: 40.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("up");
        assert!(up.operations.is_empty(), "the in-progress shape draft was cleared on utility switch");
    }

    //#region 🔖️GesturePreview
    /// 🔬️ CW7 preview-law seam: `DrawPlayApp::gesture_preview` reads live `GestureContext` only, never
    /// `DrawDocument`/`DrawOperation` — exercised directly against `DrawPlayApp` (bypassing the
    /// `VcsDocumentApp` wrapper) since `step_gesture` is the natural per-tick gesture handler.
    #[test]
    fn gesture_preview_is_none_while_idle() {
        let app = DrawPlayApp::default();
        assert!(app.gesture_preview().is_none(), "no live gesture, nothing to preview");
    }

    #[test]
    fn gesture_preview_reflects_live_shape_drag_and_clears_on_commit() {
        let mut app = DrawPlayApp::default();
        let document = default_draw_document("empty", None);
        let mut config = DrawConfig { active_utility_id: "shapeRect".into(), ..Default::default() };

        let down = app.step_gesture(draw_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [10.0, 10.0], shift: false, ctrl: false, meta: false }, &document, &mut config);
        assert!(down.document_operations.is_empty(), "pointer-down starts a scratch drag, not a document operation");
        let (key, seq_after_down, payload) = app.gesture_preview().expect("shape drag is live after pointer-down");
        assert_eq!(key, "gesture");
        let value: Value = serde_json::from_slice(&payload).expect("payload is valid json");
        assert_eq!(value["start"], json!([10.0, 10.0]));
        assert_eq!(value["cursor"], json!([10.0, 10.0]));

        let moved = app.step_gesture(draw_gesture::Event::PointerMove { world: [40.0, 30.0], marquee_threshold_world: 4.0 }, &document, &mut config);
        assert!(moved.document_operations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let (_, seq_after_move, payload) = app.gesture_preview().expect("shape drag is still live mid-drag");
        let value: Value = serde_json::from_slice(&payload).expect("payload is valid json");
        assert_eq!(value["cursor"], json!([40.0, 30.0]), "preview tracks the live cursor, not the drag start");
        assert!(seq_after_move > seq_after_down, "seq is monotone per tick, for staleness detection on the receiving end");

        let up = app.step_gesture(draw_gesture::Event::PointerUp { utility: "shapeRect".into(), world: [40.0, 30.0], shift: false, ctrl: false, meta: false }, &document, &mut config);
        assert_eq!(up.document_operations.len(), 1, "pointer-up commits the shape as one real DrawOperation");
        assert!(app.gesture_preview().is_none(), "the gesture returned to idle: nothing left to preview, and the commit above already carried the real operation");
    }

    #[test]
    fn gesture_preview_is_a_pure_read_never_mutating_gesture_context() {
        let mut app = DrawPlayApp::default();
        let document = default_draw_document("empty", None);
        let mut config = DrawConfig { active_utility_id: "shapeRect".into(), ..Default::default() };
        app.step_gesture(draw_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [1.0, 2.0], shift: false, ctrl: false, meta: false }, &document, &mut config);
        let context_before = app.gesture.borrow().context.clone();
        let _ = app.gesture_preview();
        let _ = app.gesture_preview();
        assert_eq!(app.gesture.borrow().context, context_before, "gesture_preview must never mutate the live gesture scratch it reads");
    }
    //#endregion 🔖️GesturePreview

    #[test]
    fn combine_boolean_creates_boolean_layer() {
        let mut app = new_app();
        let first_id = first_layer_id(&app);
        app.dispatch_typed(DrawCommand::AddLayer { kind: "shape:rect".into() }, &testkit::meta("local")).expect("add rect");
        let second_id = last_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::CombineBoolean { operation: "union".into(), ids: vec![first_id, second_id] }, &testkit::meta("local")).expect("combine");
        assert_eq!(result.operations.len(), 1);
        assert!(app.projection().unwrap().layers.iter().any(|layer| matches!(layer, DrawLayerNode::Boolean(_))));
    }

    #[test]
    fn canvas_point_to_world_matches_host_formula() {
        let camera = draw::DrawCamera { x: 100.0, y: 50.0, zoom: 2.0 };
        let (world_x, world_y) = canvas_point_to_world(&camera, 420.0, 310.0, 800.0, 600.0);
        assert!((world_x - 110.0).abs() < 1e-9);
        assert!((world_y - 55.0).abs() < 1e-9);
    }

    #[test]
    fn shape_rect_drag_commits_one_layer_and_requests_utility_reset() {
        // Under the real registry: canvasPointerUp is an Operation-kind pointer handler, so emitting
        // the AddLayer operation is allowed; the return-to-select is a HostEffect, not a document operation.
        let mut app = new_app_with_registry();
        set_utility(&mut app, "shapeRect");
        app.dispatch_typed(DrawCommand::CanvasPointerDown { x: 500.0, y: 400.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("down");
        app.dispatch_typed(DrawCommand::CanvasPointerMove { x: 600.0, y: 500.0, width: 1000.0, height: 800.0 }, &testkit::meta("local")).expect("move");
        let result = app.dispatch_typed(DrawCommand::CanvasPointerUp { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("up");
        assert_eq!(result.operations.len(), 1, "a shape drag commits as one edit adding exactly the layer");
        let projection = app.projection().unwrap();
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Shape(shape) if shape.shape_kind == "rect")));
        assert!(
            matches!(
                result.requested_effects.as_slice(),
                [HostEffect::SetActiveUtility { window_id, utility_id }] if window_id == DRAW_PLAY_WINDOW_CANVAS && utility_id == "selectDirect"
            ),
            "the canvas returns to select-direct via a host effect, not a document operation"
        );
    }

    #[test]
    fn pen_draft_commits_path_layer_on_enter() {
        let mut app = new_app();
        set_utility(&mut app, "pen");
        app.dispatch_typed(DrawCommand::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("p1");
        app.dispatch_typed(DrawCommand::CanvasPointerDown { x: 500.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("p2");
        let result = app.dispatch_typed(DrawCommand::CanvasCommitDraft, &testkit::meta("local")).expect("commit");
        assert_eq!(result.operations.len(), 1, "the draft commits as exactly one AddLayer edit");
        let projection = app.projection().unwrap();
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Path(path) if !path.segments.is_empty())));
        assert!(matches!(result.requested_effects.as_slice(), [HostEffect::SetActiveUtility { utility_id, .. }] if utility_id == "selectDirect"));
    }

    #[test]
    fn canvas_escape_cancels_draft_without_committing() {
        let mut app = new_app();
        let before = app.projection().unwrap().layers.len();
        set_utility(&mut app, "pen");
        app.dispatch_typed(DrawCommand::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("p1");
        let result = app.dispatch_typed(DrawCommand::CanvasEscape, &testkit::meta("local")).expect("escape");
        assert!(result.operations.is_empty());
        assert_eq!(app.projection().unwrap().layers.len(), before);
    }

    #[test]
    fn marquee_select_covers_contained_layer_only() {
        let mut app = new_app();
        set_utility(&mut app, "selectMarquee");
        let mut document = default_draw_document("marquee-test", None);
        document.layers.clear();
        let mut rect_a = create_draw_shape_layer_rect("A");
        if let DrawLayerNode::Shape(shape) = &mut rect_a {
            shape.rect = Some(draw::DrawRect { x: 10.0, y: 10.0, width: 20.0, height: 20.0 });
        }
        let rect_a_id = layer_id(&rect_a).to_string();
        let mut rect_b = create_draw_shape_layer_rect("B");
        if let DrawLayerNode::Shape(shape) = &mut rect_b {
            shape.rect = Some(draw::DrawRect { x: 200.0, y: 200.0, width: 20.0, height: 20.0 });
        }
        let rect_b_id = layer_id(&rect_b).to_string();
        document.layers.push(rect_a);
        document.layers.push(rect_b);
        app.dispatch_typed(DrawCommand::SetDocument { document: document.clone() }, &testkit::meta("local")).expect("load");
        // `default_draw_document` centers the camera on its 1024x1024 artboard (512,512 @ 0.75 zoom);
        // pin it to an identity camera so screen and world coordinates coincide for this drag.
        app.dispatch_typed(DrawCommand::SetCamera { camera: draw::DrawCamera { x: 0.0, y: 0.0, zoom: 1.0 } }, &testkit::meta("local")).expect("camera");
        app.dispatch_typed(DrawCommand::CanvasPointerDown { x: 400.0, y: 300.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("down");
        app.dispatch_typed(DrawCommand::CanvasPointerMove { x: 460.0, y: 360.0, width: 800.0, height: 600.0 }, &testkit::meta("local")).expect("move");
        app.dispatch_typed(DrawCommand::CanvasPointerUp { x: 460.0, y: 360.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }, &testkit::meta("local")).expect("up");
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&format!("overlay:sel:{rect_a_id}")), "the contained rect A is selected");
        assert!(!json.contains(&format!("overlay:sel:{rect_b_id}")), "the outside rect B is not selected");
    }

    #[test]
    fn set_camera_writes_runtime_and_emits_no_operations() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        let result = app.dispatch_typed(DrawCommand::SetCamera { camera: draw::DrawCamera { x: 5.0, y: 5.0, zoom: 2.0 } }, &testkit::meta("local")).expect("camera");
        assert!(result.operations.is_empty(), "camera is a view action and emits no operations");
        assert_eq!(app.projection().expect("projection"), before, "camera never mutates the document");
        let json = serde_json::to_string(&app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains(r#""zoom":2.0"#), "composite scene camera reflects runtime state: {json}");
        assert!(json.contains(r#""cameraX":5.0"#), "composite scene camera reflects runtime state: {json}");
    }

    #[test]
    fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
        let mut app = new_app();
        app.dispatch_typed(DrawCommand::SetCamera { camera: draw::DrawCamera { x: 4.0, y: 5.0, zoom: 1.0 } }, &testkit::meta("local")).expect("set camera");
        let result = app.dispatch_typed(DrawCommand::SetCameraZoom { value: 3.0 }, &testkit::meta("local")).expect("set camera zoom");
        assert!(result.operations.is_empty(), "camera zoom is a view action and emits no operations");
        let json = serde_json::to_string(&app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains(r#""zoom":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#""cameraX":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    #[test]
    fn add_layer_undo_round_trip_through_wrapper() {
        let mut app = new_app();
        let before = app.projection().unwrap().layers.len();
        testkit::assert_undo_redo_round_trip(&mut app, DrawCommand::AddLayer { kind: "path".into() }, |app| app.projection().unwrap().layers.len(), before, before + 1);
    }

    #[test]
    fn utility_registry_declares_all_canvas_utilities_scoped_to_the_window() {
        let definition = create_draw_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"],);
        // Selection utilities carry the Selection category; the rest are Tools.
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee", "selectLasso", "selectDirect"]);
        let scene = definition.window_kinds.iter().find(|window| window.id == DRAW_PLAY_WINDOW_CANVAS).expect("canvas window");
        assert_eq!(scene.utilities.len(), definition.utilities.len(), "every utility is scoped to the canvas window kind");
        // The framework auto-injects the setActiveUtility View action once utilities are declared.
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        // The retired document operation vocabulary is gone.
        assert!(!definition.actions.iter().any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }

    #[test]
    fn render_canvas_emits_selection_overlay() {
        let mut app = new_app();
        app.dispatch_typed(DrawCommand::AddLayer { kind: "shape:rect".into() }, &testkit::meta("local")).expect("add");
        let id = last_layer_id(&app);
        app.dispatch_typed(DrawCommand::SetSelection { ids: vec![id.clone()] }, &testkit::meta("local")).expect("select");
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&format!("overlay:sel:{id}")));
    }

    #[test]
    fn draw_labels_resolve_native_by_default() {
        let mut app = new_app();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Add Path"));
        assert!(json.contains("Add Rectangle"));
        assert!(!json.contains("Pfad hinzufügen"));
    }

    #[test]
    fn draw_labels_translate_panels_in_german() {
        let mut app = new_app();
        app.dispatch_typed(DrawCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let layers_node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render");
        let layers_json = serde_json::to_string(&layers_node).unwrap();
        assert!(layers_json.contains("Pfad hinzufügen"));
        assert!(layers_json.contains("Rechteck hinzufügen"));
        assert!(!layers_json.contains("Add Path"));
        let catalogue_node = app.render(DRAW_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("\"Ellipse\""));
        assert!(catalogue_json.contains("Nachzeichnung"));
    }

    #[test]
    fn draw_io_declares_vector_out_and_export_media_covers_both_ports() {
        let mut app = new_app();
        app.dispatch_typed(DrawCommand::AddLayer { kind: "shape:rect".into() }, &testkit::meta("local")).expect("add");
        let projection = app.projection().expect("projection");
        let doc = DocumentView { projection: &projection, history: &semio_framework_plugin::HistoryView::empty() };
        let app_impl = DrawPlayApp::default();
        let vector = app_impl.export_media("vector:out", &doc).expect("vector:out");
        let MediaPayload::Structured { schema, json } = vector.payload else { panic!("expected structured svg payload") };
        assert_eq!(schema, "2d.drawing");
        assert!(json.starts_with("<svg"));
        assert!(app_impl.export_media("document:out", &doc).is_ok());
        assert!(matches!(app_impl.export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
    }
}
//#endregion 🧪️Tests
