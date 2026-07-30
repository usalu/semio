//! ⚙️ Puzzle 2d app — headless compute (constitutional: engine).
#![allow(clippy::missing_errors_doc, reason = "Puzzle board bundle is internal to puzzle 2d.")]

use puzzle_2d::Puzzle2dProjection;

pub use cavas::{CubicBez, Point, Vec2};
pub use graph::cavas;
pub use graph::{
    apply_edge_handle_snap_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value, apply_normal_undirected_redraw_layout_to_fixture_v1_json,
    apply_redraw_layout_to_fixture_v1_json as apply_ported_redraw_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_value, GraphExtension,
};
pub use infinite_board_port_directed_normal::{self as graph, *};
pub use reasoning_mindmap as mindmap;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn is_undirected_fixture_schema(schema: &str) -> bool {
    matches!(schema, "reasoning.mindmap.fixture" | "reasoning.wires.fixture")
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn redraw_layout_fixture_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
    let fixture: serde_json::Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
    let schema = fixture.get("schema").and_then(|v| v.as_str()).unwrap_or("");
    let opts: serde_json::Value = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
    let mode = opts.get("mode").and_then(|v| v.as_str()).unwrap_or("force-graph");
    if mode == "force-graph" && is_undirected_fixture_schema(schema) {
        apply_normal_undirected_redraw_layout_to_fixture_v1_json(fixture_json, options_json).map_err(|e| e.to_string())
    } else {
        apply_ported_redraw_layout_to_fixture_v1_json(fixture_json, options_json)
    }
}

mod board_metabolism_icons {
    include!(concat!(env!("OUT_DIR"), "/board_metabolism_icon_match.rs"));
}

fn resolve_node_icon_svg_from_encoding(encoded: &str) -> Option<String> {
    let t = encoded.trim();
    if t.is_empty() {
        return None;
    }
    if let Some(s) = board_metabolism_icons::board_metabolism_icon_svg(t) {
        return Some(s.to_string());
    }
    let lower = t.to_ascii_lowercase();
    if lower.starts_with("<?xml") || lower.contains("<svg") {
        return Some(t.to_string());
    }
    None
}

mod board_icon_codec {
    pub use crate::cavas::icon_codec::{board_resolve_icon_kind as resolve_with_lookup, board_typst_markup_to_svg, BoardResolvedIcon, ThemedSvgLookup};
    pub fn board_resolve_icon_kind(encoded: &str) -> BoardResolvedIcon {
        resolve_with_lookup(encoded, super::puzzle_themed_icon_lookup)
    }
}

pub fn puzzle_themed_icon_lookup(key: &str) -> Option<&'static str> {
    board_metabolism_icons::board_metabolism_icon_svg(key)
}

pub fn puzzle_board_host() -> BoardHost {
    let mut h = BoardHost::new();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}

pub fn puzzle_board_host_normal() -> BoardHost {
    let mut h = BoardHost::new_normal();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}

// #region 🔖Puzzle2dExtension
/// 🧩 Puzzle 2d domain extension over the property graph canvas.
#[derive(Clone, Debug, Default)]
pub struct Puzzle2dExtension;

impl cavas::CanvasExtension for Puzzle2dExtension {
    fn extension_id(&self) -> &str {
        "puzzle.2d"
    }
}

impl graph::GraphExtension for Puzzle2dExtension {}
// #endregion 🔖Puzzle2dExtension

// #region 🔖WasmHost
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
#[cfg(target_arch = "wasm32")]
use mathematical_geometry::ray_from_origin_to_axis_aligned_rectangle_edge;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardComputeEdgeBezier)]
pub fn board_compute_edge_bezier(source_px: f64, source_py: f64, source_cx: f64, source_cy: f64, target_px: f64, target_py: f64, target_cx: f64, target_cy: f64) -> Vec<f64> {
    let c = compute_edge_bezier_points(Point::new(source_px, source_py), Point::new(target_px, target_py), Point::new(source_cx, source_cy), Point::new(target_cx, target_cy));
    vec![c.p0.x, c.p0.y, c.p1.x, c.p1.y, c.p2.x, c.p2.y, c.p3.x, c.p3.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardDistancePointCubic)]
pub fn board_distance_point_cubic(px: f64, py: f64, p0x: f64, p0y: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64, p3x: f64, p3y: f64, steps: u32) -> f64 {
    let curve = CubicBez::new(Point::new(p0x, p0y), Point::new(p1x, p1y), Point::new(p2x, p2y), Point::new(p3x, p3y));
    distance_point_to_cubic_bezier(Point::new(px, py), curve, steps.max(1) as usize)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRayRectEdge)]
pub fn board_ray_rect_edge(hw: f64, hh: f64, ux: f64, uy: f64) -> Vec<f64> {
    let p = ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionCircle)]
pub fn board_handle_position_circle(cx: f64, cy: f64, radius: f64, angle: f64) -> Vec<f64> {
    let p = handle_position_on_circle(Point::new(cx, cy), radius, angle);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionRectangle)]
pub fn board_handle_position_rectangle(cx: f64, cy: f64, width: f64, height: f64, angle: f64) -> Vec<f64> {
    let p = handle_position_on_rectangle(Point::new(cx, cy), width, height, angle);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawLayoutFixtureJson)]
pub fn board_redraw_layout_fixture_json(fixture_json: &str, options_json: &str) -> Result<String, JsValue> {
    redraw_layout_fixture_json(fixture_json, options_json).map_err(|e| JsValue::from_str(&e))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawHandlesFixtureJson)]
pub fn board_redraw_handles_fixture_json(fixture_json: &str) -> Result<String, JsValue> {
    graph::apply_edge_handle_snap_to_fixture_v1_json(fixture_json).map_err(|e| JsValue::from_str(&e))
}

/// 🔤 Parses `.puzzle2d` DSL text (`Puzzle2dProjection`'s `dsl::DslDocument` grammar) into the same camelCase JSON shape callers previously got from a hand-authored `*.2d.json` fixture — lets non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the DSL grammar.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = puzzle2dParseDslJson)]
pub fn puzzle2d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use store::DocumentDsl;
    let projection = Puzzle2dProjection::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&projection).map_err(|error| JsValue::from_str(&error.to_string()))
}

// #region 🔖WasmSession
/// 🖥️ Single WASM entry: one {@link BoardHost}, optional WebGPU surface bound via {@link BoardSession::attach_canvas}.
#[cfg(target_arch = "wasm32")]
struct BoardSessionInner {
    host: BoardHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
}

#[cfg(target_arch = "wasm32")]
impl BoardSessionInner {
    fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let scene = self.host.build_vector_scene();
        let clear = self.host.canvas_theme.raster_clear;
        self.gpu.render_frame(&scene, clear)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct BoardSession {
    state: Rc<RefCell<BoardSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BoardSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(BoardSessionInner { host: puzzle_board_host(), gpu: cavas::gpu_session::CanvasGpuSession::default() })) }
    }

    /// 🧠 Construct a normal-graph session (no handles; edges connect node ids).
    #[wasm_bindgen(js_name = newNormal)]
    pub fn new_normal() -> Self {
        Self { state: Rc::new(RefCell::new(BoardSessionInner { host: puzzle_board_host_normal(), gpu: cavas::gpu_session::CanvasGpuSession::default() })) }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = isDraggingAreaSelect)]
    pub fn is_dragging_area_select(&self) -> bool {
        self.state.borrow().host.is_dragging_area_select()
    }

    #[wasm_bindgen(js_name = defersDescriptorSyncFromJs)]
    pub fn defers_descriptor_sync_from_js(&self) -> bool {
        self.state.borrow().host.defers_descriptor_sync_from_js()
    }

    /// @emoji 🌊 Binds WebGPU presentation to `canvas` once; `logical_w`/`logical_h` are CSS pixels, `dpr` scales the swapchain backing store; uses `future_to_promise` so wasm-bindgen does not hold `&mut BoardSession` across `await` (avoids `borrow_fail` vs `setSize` during GPU setup).
    #[wasm_bindgen(js_name = attach_canvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> Promise {
        let inner = self.state.clone();
        if inner.borrow().gpu.gpu_ready() {
            return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
        }
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let canvas = canvas.clone();
        future_to_promise(async move {
            let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.host.set_size(lw, lh, dpr);
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let mut inner = self.state.borrow_mut();
        inner.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = setSelectionScreenPreview)]
    pub fn set_selection_screen_preview(&mut self, flat_xy: &[f64]) {
        let mut inner = self.state.borrow_mut();
        if flat_xy.len() < 4 || flat_xy.len() % 2 != 0 {
            inner.host.set_selection_screen_preview(None);
            return;
        }
        let mut pts = Vec::with_capacity(flat_xy.len() / 2);
        for chunk in flat_xy.chunks_exact(2) {
            pts.push(Point::new(chunk[0], chunk[1]));
        }
        inner.host.set_selection_screen_preview(Some(pts));
    }

    #[wasm_bindgen(js_name = clearSelectionScreenPreview)]
    pub fn clear_selection_screen_preview(&mut self) {
        self.state.borrow_mut().host.set_selection_screen_preview(None);
    }

    #[wasm_bindgen(js_name = syncDescriptorJson)]
    pub fn sync_descriptor_json(&mut self, json: &str) -> Result<(), JsValue> {
        let mut raw: serde_json::Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        normalize_board_descriptor_hidden_to_visible(&mut raw);
        let desc: SceneDescriptorJson = serde_json::from_value(raw).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.state.borrow_mut().host.sync_descriptor(&desc).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = setNodePositionsJson)]
    pub fn set_node_positions_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_node_positions_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setKindCatalogsJson)]
    pub fn set_board_kind_catalogs_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_board_kind_catalogs_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = clearIconVectorCache)]
    pub fn clear_icon_vector_cache_wasm(&mut self) {
        self.state.borrow_mut().host.clear_icon_vector_cache();
    }

    #[wasm_bindgen(js_name = parseFixtureJson)]
    pub fn parse_fixture_json(&mut self, json: &str) -> bool {
        let raw: serde_json::Value = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(_) => return false,
        };
        self.state.borrow_mut().host.parse_fixture_v1(&raw)
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera_wasm(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    /// 🎥 Sets the camera without enqueuing a `camera` event — for re-applying the framing camera after a fixture re-parse without echoing it back to the program.
    #[wasm_bindgen(js_name = setCameraSilent)]
    pub fn set_camera_silent_wasm(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera_silent(x, y, zoom);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen_wasm(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
    pub fn pick_targets_at_screen_json_wasm(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
    }

    #[wasm_bindgen(js_name = pointerLeaveScreen)]
    pub fn pointer_leave_screen_wasm(&mut self, alt: bool) {
        self.state.borrow_mut().host.pointer_leave_screen(alt);
    }

    #[wasm_bindgen(js_name = cancelAreaSelect)]
    pub fn cancel_area_select_wasm(&mut self) -> bool {
        self.state.borrow_mut().host.cancel_area_select()
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen_wasm(&mut self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active_wasm(&mut self, active: bool) {
        self.state.borrow_mut().host.set_wheel_zoom_active(active);
    }

    #[wasm_bindgen(js_name = deleteSelection)]
    pub fn delete_selection_wasm(&mut self) {
        self.state.borrow_mut().host.delete_selection();
    }

    #[wasm_bindgen(js_name = drainEventsJson)]
    pub fn drain_events_json_wasm(&mut self) -> String {
        self.state.borrow_mut().host.drain_events_json()
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        let inner = self.state.borrow();
        serde_json::json!({
            "x": inner.host.camera.x,
            "y": inner.host.camera.y,
            "zoom": inner.host.camera.zoom,
        })
        .to_string()
    }

    #[wasm_bindgen(js_name = overlayPaintStateJson)]
    pub fn overlay_paint_state_json_wasm(&self) -> String {
        self.state.borrow().host.overlay_paint_state_json()
    }

    #[wasm_bindgen(js_name = setSelectionOptions)]
    pub fn set_selection_options_wasm(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
        self.state.borrow_mut().host.set_selection_options(method, mode, select_nodes, select_edges, select_handles);
    }

    #[wasm_bindgen(js_name = setHandleLinkCompatJson)]
    pub fn set_handle_link_compat_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_handle_link_compat_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setWorldRasterTiling)]
    pub fn set_world_raster_tiling_wasm(&mut self, mode: &str) {
        self.state.borrow_mut().host.set_world_raster_tiling(mode);
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json_wasm(&self) -> String {
        puzzle_2d_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setGridSnapEnabled)]
    pub fn set_grid_snap_enabled_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_grid_snap_enabled(enabled);
    }

    #[wasm_bindgen(js_name = setGridFactor)]
    pub fn set_grid_factor_wasm(&mut self, v: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_grid_factor(v).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setActiveUtility)]
    pub fn set_active_utility_wasm(&mut self, label: &str) {
        self.state.borrow_mut().host.set_active_utility(label);
    }

    #[wasm_bindgen(js_name = setSuggestionOffset)]
    pub fn set_suggestion_offset_wasm(&mut self, distance: f64) {
        self.state.borrow_mut().host.set_suggestion_offset(distance);
    }

    #[wasm_bindgen(js_name = setBrushKindWeights)]
    pub fn set_brush_kind_weights_wasm(&mut self, json: &str) {
        self.state.borrow_mut().host.set_brush_kind_weights(json);
    }

    #[wasm_bindgen(js_name = setBrushNodeSize)]
    pub fn set_brush_node_size_wasm(&mut self, size: f64) {
        self.state.borrow_mut().host.set_brush_node_size(size);
    }

    #[wasm_bindgen(js_name = brushCycleCandidate)]
    pub fn brush_cycle_candidate_wasm(&mut self, forward: bool) {
        self.state.borrow_mut().host.brush_cycle_candidate(forward);
    }

    #[wasm_bindgen(js_name = brushSetCandidateIndex)]
    pub fn brush_set_candidate_index_wasm(&mut self, index: u32) {
        self.state.borrow_mut().host.brush_set_candidate_index(index as usize);
    }

    #[wasm_bindgen(js_name = brushOpenSlot)]
    pub fn brush_open_slot_wasm(&mut self, handle_id: &str) {
        self.state.borrow_mut().host.brush_open_slot(handle_id);
    }

    #[wasm_bindgen(js_name = brushCommitSlot)]
    pub fn brush_commit_slot_wasm(&mut self) {
        self.state.borrow_mut().host.brush_commit_slot();
    }

    #[wasm_bindgen(js_name = brushCancelSlot)]
    pub fn brush_cancel_slot_wasm(&mut self) {
        self.state.borrow_mut().host.brush_cancel_slot();
    }

    #[wasm_bindgen(js_name = brushFillJson)]
    pub fn brush_fill_json_wasm(&self, max_count: u32, seed: u32) -> String {
        self.state.borrow().host.brush_fill_json(max_count, u64::from(seed))
    }

    #[wasm_bindgen(js_name = brushFillSessionBegin)]
    pub fn brush_fill_session_begin_wasm(&mut self, max_count: u32, seed: u32) {
        self.state.borrow_mut().host.brush_fill_session_begin(max_count, u64::from(seed));
    }

    #[wasm_bindgen(js_name = brushFillSessionStep)]
    pub fn brush_fill_session_step_wasm(&mut self, chunk_budget: u32) -> String {
        self.state.borrow_mut().host.brush_fill_session_step(chunk_budget)
    }

    #[wasm_bindgen(js_name = brushFillSessionClear)]
    pub fn brush_fill_session_clear_wasm(&mut self) {
        self.state.borrow_mut().host.brush_fill_session_clear();
    }

    #[wasm_bindgen(js_name = setBrushSessionJson)]
    pub fn set_brush_session_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_brush_session_mirror_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearBrushSessionJson)]
    pub fn clear_brush_session_json_wasm(&mut self) {
        let _ = self.state.borrow_mut().host.set_brush_session_mirror_json("");
    }

    #[wasm_bindgen(js_name = setFixtureDropPreviewJson)]
    pub fn set_fixture_drop_preview_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_fixture_drop_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearFixtureDropPreview)]
    pub fn clear_fixture_drop_preview_wasm(&mut self) {
        let _ = self.state.borrow_mut().host.set_fixture_drop_preview_json("");
    }

    #[wasm_bindgen(js_name = setLinkSessionJson)]
    pub fn set_link_session_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_external_link_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clearLinkSessionJson)]
    pub fn clear_link_session_json_wasm(&mut self) {
        self.state.borrow_mut().host.clear_external_link_preview();
    }

    #[wasm_bindgen(js_name = setOriginalElementStyle)]
    pub fn set_original_element_style_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_original_element_style(enabled);
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label_wasm(&mut self, label: &str) {
        self.state.borrow_mut().host.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = setSelectionIdsJson)]
    pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_selection_ids(&ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setSelectionIdsJsonSilent)]
    pub fn set_selection_ids_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_selection_ids_silent(&ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setPreselectStateJsonSilent)]
    pub fn set_preselect_state_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
        #[derive(serde::Deserialize)]
        struct PreselectSync {
            ids: Vec<String>,
            #[serde(default, rename = "removedIds")]
            removed_ids: Vec<String>,
        }
        let body: PreselectSync = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_preselect_state_silent(&body.ids, &body.removed_ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setHoveredIdSilent)]
    pub fn set_hovered_id_silent_wasm(&mut self, id: Option<String>) {
        self.state.borrow_mut().host.set_hovered_id_silent(id);
    }

    #[wasm_bindgen(js_name = setHoveredKindSilent)]
    pub fn set_hovered_kind_silent_wasm(&mut self, domain: Option<String>, kind_id: Option<String>) {
        self.state.borrow_mut().host.set_hovered_kind_silent(domain, kind_id);
    }

    #[wasm_bindgen(js_name = encodedSceneHint)]
    pub fn encoded_scene_hint_wasm(&self) -> usize {
        self.state.borrow().host.encoded_scene_hint()
    }

    /// @emoji 🎨 Presents one frame when a GPU surface is attached; otherwise no-operation `Ok`.
    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }
}
// #endregion 🔖WasmSession


//#region 🔖DocumentHelpers
pub fn empty_puzzle2d_projection() -> Puzzle2dProjection {
    Puzzle2dProjection::default()
}
//#endregion 🔖DocumentHelpers

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cavas::Point;

    #[test]
    fn computes_handle_positions_and_edge_curves() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let curve = engine.edge_curve(100).expect("edge curve should exist");
        let p0 = curve.p0();
        let p1 = curve.p1();
        let p2 = curve.p2();
        let p3 = curve.p3();
        let cap = 8.0;
        assert!((p0.x() - (40.0 + cap)).abs() < 0.001);
        assert!(p0.y().abs() < 0.001);
        assert!((p3.x() - (260.0 - cap)).abs() < 0.001);
        assert!(p3.y().abs() < 0.001);
        let source_radial = p0 - Point::ZERO;
        let arm0 = p1 - p0;
        let align0 = normalize_or_zero(source_radial).dot(normalize_or_zero(arm0));
        let target_approach = Point::new(300.0, 0.0) - p3;
        let arm1 = p3 - p2;
        let align1 = normalize_or_zero(target_approach).dot(normalize_or_zero(arm1));
        assert!(align0 > 0.99);
        assert!(align1 > 0.99);
    }

    #[test]
    fn drags_nodes_without_rebuilding_the_scene_catalog() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 30.0, true);

        engine.pointer_down(0.0, 0.0, false);
        engine.pointer_move(60.0, 25.0);
        engine.pointer_up(60.0, 25.0);

        let node = engine.nodes.get(&1).expect("node should remain in the engine");
        assert_eq!(node.center, Point::new(60.0, 25.0));

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { node_ids, .. } if node_ids == &vec![1])));
        assert!(events.iter().any(|event| matches!(event, BoardEvent::NodeMoved { id: 1, x, y } if (*x - 60.0).abs() < 0.001 && (*y - 25.0).abs() < 0.001)));
    }

    #[test]
    fn hit_tests_handles_before_nodes_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 200.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let handle_point = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(handle_point.x, handle_point.y, false);

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { handle_ids, .. } if handle_ids == &vec![10])));
    }

    #[test]
    fn renders_snapshot_for_nodes_handles_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 10.0, 20.0, 18.0, true);
        engine.create_node(2, 120.0, 20.0, 18.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let snapshot = engine.render_snapshot();
        assert_eq!(snapshot.nodes.len(), 2);
        assert_eq!(snapshot.handles.len(), 2);
        assert_eq!(snapshot.edges.len(), 1);
    }

    #[test]
    fn engine_extend_pick_keeps_node_when_adding_handle() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        engine.pointer_down(0.0, 0.0, false);
        let _ = engine.drain_events();
        let hp = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(hp.x, hp.y, true);
        let events = engine.drain_events();
        let last = events.iter().rev().find_map(|event| match event {
            BoardEvent::SelectionChanged { node_ids, handle_ids, edge_ids } => Some((node_ids.clone(), handle_ids.clone(), edge_ids.clone())),
            _ => None,
        });
        let Some((node_ids, handle_ids, edge_ids)) = last else {
            panic!("expected SelectionChanged");
        };
        assert!(node_ids.contains(&1));
        assert!(handle_ids.contains(&10));
        assert!(edge_ids.is_empty());
    }
}

#[cfg(test)]
mod host_tests {
    use super::{
        compute_edge_bezier_points, distance_between, handle_position_on_circle, handle_position_on_rectangle, BoardElementStyleKind, BoardHost, EdgeDescJson, EdgeStrokePattern, EdgeTipGeometry, GraphPortMode, HandleDescJson, Interaction,
        NodeDescJson, NodeShape, SceneDescriptorJson, WireDescJson,
    };
    use crate::cavas::geom_sel::cubic_bezier_point;
    use crate::cavas::Point;
    use serde_json::json;

    fn set_detail_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 0.0, 2.0);
    }

    fn set_micro_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 60.0, 4.5);
    }

    fn set_overview_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 0.0, 0.25);
    }

    fn sample_scene() -> SceneDescriptorJson {
        SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: None,
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![
                HandleDescJson {
                    id: "a:h0".into(),
                    node_id: "a".into(),
                    angle: 0.0,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("port".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
                HandleDescJson {
                    id: "b:h0".into(),
                    node_id: "b".into(),
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("port".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
            ],
            edges: vec![EdgeDescJson { id: "e1".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None }],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        }
    }

    /// 🔗 Keeps the runtime kind-catalog JSON shape in sync with the compile-time `puzzle2d-default` manifest.
    #[test]
    fn puzzle2d_default_manifest_satisfies_board_host_validation() {
        let manifest: serde_json::Value = serde_json::from_str(include_str!("../../../../2d/manifest/default.manifest.json")).unwrap();
        let handle_kinds: Vec<serde_json::Value> = manifest["portKinds"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| json!({ "id": row["id"], "name": row["name"], "color": row["presentation"]["color"], "defaultWireKind": row["presentation"]["defaultWireKind"] }))
            .collect();
        let wire_kinds: Vec<serde_json::Value> = manifest["wireKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "defaultEdgeKind": row["presentation"]["defaultEdgeKind"] })).collect();
        let edge_kinds: Vec<serde_json::Value> = manifest["edgeKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"] })).collect();
        let catalogs_json = json!({ "handleKinds": handle_kinds, "wireKinds": wire_kinds, "edgeKinds": edge_kinds }).to_string();

        let mut host = BoardHost::new();
        host.set_board_kind_catalogs_from_json(&catalogs_json).expect("catalog json derived from the manifest must be valid");
        host.validate_against_manifest_id("puzzle2d-default").expect("runtime catalog must satisfy the compile-time puzzle2d-default manifest");
    }

    #[test]
    fn board_host_defers_descriptor_sync_while_panning() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let _ = h.drain_events_json();
        h.pointer_down_screen(10.0, 10.0, 1, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        h.pointer_move_screen(80.0, 60.0, false, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        let _ = h.drain_events_json();
        h.pointer_up_screen(80.0, 60.0, false, false, false);
        assert!(!h.defers_descriptor_sync_from_js());
    }

    #[test]
    fn board_host_defers_descriptor_sync_while_dragging_nodes() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let _ = h.drain_events_json();
        let start = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(start.x, start.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { .. }));
        h.pointer_move_screen(start.x + 40.0, start.y, false, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"));
        h.pointer_up_screen(start.x + 40.0, start.y, false, false, false);
        assert!(!h.defers_descriptor_sync_from_js());
        let end = h.drain_events_json();
        assert!(end.contains("nodeDragEnd"));
    }

    #[test]
    fn board_host_set_node_positions_updates_existing_nodes_only() {
        let mut h = BoardHost::new();
        h.set_size(400, 300, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        h.set_node_positions(&[("a".into(), 12.0, 34.0), ("missing".into(), 1.0, 2.0), ("a".into(), f64::NAN, 0.0)]);
        let node = h.nodes.get("a").expect("node a should remain");
        assert!((node.x - 12.0).abs() < 0.001);
        assert!((node.y - 34.0).abs() < 0.001);
        assert!(h.test_content_scene_generation() > gen_before, "moving nodes must invalidate cached world content");
        h.set_node_positions_json(r#"[{"id":"a","x":90.0,"y":110.0}]"#).unwrap();
        let node = h.nodes.get("a").expect("node a should remain");
        assert!((node.x - 90.0).abs() < 0.001);
        assert!((node.y - 110.0).abs() < 0.001);
    }

    #[test]
    fn board_host_overlay_paint_state_json_matches_host_camera_lod_and_node_centers() {
        let mut h = BoardHost::new();
        h.set_size(640, 480, 2.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_camera_silent(12.0, -8.0, 0.2);
        if let Some(n) = h.nodes.get_mut("a") {
            n.x = 33.0;
            n.y = 44.0;
        }
        let raw: serde_json::Value = serde_json::from_str(&h.overlay_paint_state_json()).expect("overlay paint state json");
        assert!((raw["camera"]["x"].as_f64().unwrap() - 12.0).abs() < 1e-9);
        assert!((raw["camera"]["y"].as_f64().unwrap() - (-8.0)).abs() < 1e-9);
        assert!((raw["camera"]["zoom"].as_f64().unwrap() - 0.2).abs() < 1e-9);
        assert_eq!(raw["lod"].as_str(), Some("overview"));
        let nodes = raw["nodes"].as_array().expect("nodes array");
        let a = nodes.iter().find(|row| row["id"].as_str() == Some("a")).expect("node a row");
        assert!((a["x"].as_f64().unwrap() - 33.0).abs() < 1e-9);
        assert!((a["y"].as_f64().unwrap() - 44.0).abs() < 1e-9);
    }

    #[test]
    fn board_host_node_drag_invalidates_cached_world_content() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 80.0, s.y + 40.0, false, false, false);
        assert!(h.test_content_scene_generation() > gen_before, "node drag must rebuild cached nodes/handles, not only edges");
        let node = h.nodes.get("a").expect("dragged node");
        assert!(node.x.abs() > 1.0 || node.y.abs() > 1.0, "pointer move should translate node a away from origin");
    }

    #[test]
    fn board_host_manual_lod_follow_zoom_still_encodes_graph() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let with_automatic = h.encoded_scene_hint();
        assert!(with_automatic > 0, "sample scene should encode vector paths");
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("");
        let manual_follow_zoom = h.encoded_scene_hint();
        assert!(manual_follow_zoom > 0, "manual follow-zoom LOD must still draw nodes/edges (hint={manual_follow_zoom})");
        h.set_forced_draw_lod_label("overview");
        let pinned_overview = h.encoded_scene_hint();
        assert!(pinned_overview > 0, "pinned overview LOD must still draw graph");
        h.set_automatic_lod(true);
        let automatic_restored = h.encoded_scene_hint();
        assert_eq!(with_automatic, automatic_restored);
    }

    #[test]
    fn board_host_pick_selection_never_sets_exit_highlight() {
        let mut h = BoardHost::new();
        h.set_size(400, 300, 1.0);
        let mut d = sample_scene();
        d.selection_exit_highlight_ids = vec!["a".into(), "ghost".into()];
        h.sync_descriptor(&d).unwrap();
        let _ = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let ev = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        assert!(ev.contains("\"exitHighlightIds\":[]"));
        h.set_selection_ids(&["e1".into()]);
        let ev2 = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        assert!(ev2.contains("\"exitHighlightIds\":[]"));
    }

    #[test]
    fn board_host_canvas_theme_keeps_explicit_element_state_colors() {
        let mut h = BoardHost::new();
        h.set_canvas_theme_from_json(
            r#"{
				"nodeStrokeHovered": [1, 2, 3, 255],
				"edgeStrokeHovered": [4, 5, 6, 255],
				"handleStrokeHovered": [7, 8, 9, 255],
				"wireStrokeHovered": [10, 11, 12, 255]
			}"#,
        )
        .unwrap();
        assert_eq!(h.canvas_theme.node_stroke_hovered.to_rgba8(), crate::cavas::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
        assert_eq!(h.canvas_theme.edge_stroke_hovered.to_rgba8(), crate::cavas::Color::from_rgba8(4, 5, 6, 255).to_rgba8());
        assert_eq!(h.canvas_theme.handle_stroke_hovered.to_rgba8(), crate::cavas::Color::from_rgba8(7, 8, 9, 255).to_rgba8());
        assert_eq!(h.canvas_theme.wire_stroke_hovered.to_rgba8(), crate::cavas::Color::from_rgba8(10, 11, 12, 255).to_rgba8());
    }

    #[test]
    fn board_host_cancel_area_select_restores_initial_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        h.pointer_down_screen(5.0, 5.0, 0, false, false);
        assert!(!h.is_dragging_area_select());
        h.pointer_move_screen(20.0, 5.0, false, false, false);
        assert!(h.is_dragging_area_select());
        let _ = h.drain_events_json();
        assert!(h.cancel_area_select());
        assert!(!h.is_dragging_area_select());
        let ev = h.drain_events_json();
        assert!(ev.contains("preselectCancel"));
        assert!(!ev.contains("\"select\""));
        assert_eq!(h.selection.len(), 2);
        assert!(h.selection.contains("a") && h.selection.contains("b"));
        assert!(h.preselect.is_empty());
    }

    #[test]
    fn board_host_syncs_descriptor_and_hit_tests_handle_before_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hit = h.resolve_hit_world(hp);
        assert_eq!(hit.as_deref(), Some("a:h0"));
        assert!(h.encoded_scene_hint() > 10);
    }

    #[test]
    fn board_host_cached_content_includes_edge_vector_paths_at_overview_zoom() {
        let mut h = BoardHost::new();
        h.set_size(1200, 800, 1.0);
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        h.set_camera_silent(0.0, 0.0, 0.21);
        let with_edges = h.encoded_scene_hint();
        let mut without = link_test_scene_no_edge();
        h.sync_descriptor(&without).unwrap();
        let without_edges = h.encoded_scene_hint();
        assert!(with_edges > without_edges, "overview cached draw must encode edges (with={with_edges}, without={without_edges})");
    }

    #[test]
    fn board_host_world_clip_changes_vector_encoding() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 600.0,
            y: 400.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_world_raster_tiling("none");
        let monolithic = h.encoded_scene_hint();
        h.set_world_raster_tiling("world-clip");
        let tiled = h.encoded_scene_hint();
        assert!(tiled >= monolithic);
    }

    #[test]
    fn board_host_silent_selection_keeps_cached_world_content_warm() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen_before, "selection chrome must paint via dynamic fill/stroke layers without rebuilding cached icons");
        assert_ne!(h.encoded_scene_hint(), neutral_hint, "selected node fill appears in overlay fill layer at normal LOD");
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected));
    }

    #[test]
    fn board_host_selected_node_keeps_selected_style_when_hovered() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_selection_ids(&["a".into()]);
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected), "committed selection chrome should beat hover while pointer is over the node");
        h.set_selection_ids(&[]);
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Hovered), "unselected nodes should still use hover chrome");
    }

    #[test]
    fn board_host_dragging_selected_node_keeps_selected_style_at_detail_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_selection_ids(&["a".into()]);
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { .. }));
        assert_eq!(h.hovered_id.as_deref(), Some("a"));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected), "node drag should keep primary selected paint at detail LOD");
    }

    #[test]
    fn board_host_drag_emits_node_move() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let w = Point::new(0.0, 0.0);
        let s = h.world_to_screen(w);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"));
    }

    #[test]
    fn board_host_compact_discrete_hit_selects_and_drags_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.5);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
        assert!(h.resolve_hit_world(Point::new(150.0, 0.0)).is_none());
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "compact discrete node hit should drag, got: {ev}");
    }

    #[test]
    fn board_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("minimap");
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let gap = Point::new(150.0, 0.0);
        assert!(h.resolve_hit_world(gap).is_none());
        let s = h.world_to_screen(gap);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "expected bounded drag nodeMove, got: {ev}");
        let zoom = 0.1;
        let dx = 50.0 / zoom;
        let dy = 30.0 / zoom;
        let a = h.nodes.get("a").unwrap();
        let b = h.nodes.get("b").unwrap();
        assert!((a.x - dx).abs() < 1e-3 && (a.y - dy).abs() < 1e-3);
        assert!((b.x - (300.0 + dx)).abs() < 1e-3 && (b.y - dy).abs() < 1e-3);
    }

    #[test]
    fn board_host_overview_bounded_drag_moves_selection_inside_union_bounds() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("overview");
        set_overview_lod(&mut h);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let gap = Point::new(150.0, 0.0);
        assert!(h.resolve_hit_world(gap).is_none());
        let s = h.world_to_screen(gap);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 40.0, s.y + 20.0, false, false, false);
        h.pointer_up_screen(s.x + 40.0, s.y + 20.0, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "expected overview bounded drag, got: {ev}");
    }

    #[test]
    fn board_host_detail_lod_resolves_direct_handle_hit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 2.0, hp.y);
        assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_multi_select_drag_moves_each_selected_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 100.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_options("rectangle", "additive", true, true, true);
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let w = Point::new(0.0, 0.0);
        let s = h.world_to_screen(w);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 10.0, s.y + 5.0, false, false, false);
        h.pointer_up_screen(s.x + 10.0, s.y + 5.0, false, false, false);
        let a = h.nodes.get("a").expect("node a");
        let b = h.nodes.get("b").expect("node b");
        assert!((a.x - 10.0).abs() < 1e-6);
        assert!((a.y - 5.0).abs() < 1e-6);
        assert!((b.x - 110.0).abs() < 1e-6);
        assert!((b.y - 5.0).abs() < 1e-6);
        let sorted: Vec<_> = h.selection.iter().cloned().collect();
        assert_eq!(sorted, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn board_host_selection_target_edges_skips_node_geometry() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "invertive", false, true, false);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let inside_node_a = Point::new(0.0, 0.0);
        assert!(h.resolve_hit_world(inside_node_a).is_none());
        let on_edge = Point::new(150.0, 0.0);
        assert_eq!(h.resolve_hit_world(on_edge).as_deref(), Some("e1"));
    }

    #[test]
    fn board_host_additive_click_merges_edge_into_existing_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "additive", true, true, true);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        let on_edge = Point::new(150.0, 0.0);
        let s = h.world_to_screen(on_edge);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        let mut got: Vec<_> = h.selection.iter().cloned().collect();
        got.sort();
        assert_eq!(got, vec!["a".to_string(), "e1".to_string()]);
    }

    #[test]
    fn board_host_selection_change_does_not_bump_content_scene_generation() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen);
        let selected_hint = h.encoded_scene_hint();
        assert_ne!(selected_hint, neutral_hint);
        h.set_selection_ids_silent(&[]);
        assert_eq!(h.test_content_scene_generation(), gen);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[test]
    fn board_host_hover_change_does_not_bump_content_scene_generation() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_content_scene_generation(), gen, "hover must paint via dynamic overlay chrome without rebuilding cached icons");
        let hovered_hint = h.encoded_scene_hint();
        assert_ne!(hovered_hint, neutral_hint);
        h.set_hovered_id_silent(None);
        assert_eq!(h.test_content_scene_generation(), gen);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[test]
    fn board_host_background_click_deselect_skips_preselect_events() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let desc = sample_scene();
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let _ = h.drain_events_json();
        let away = Point::new(5000.0, 5000.0);
        let s = h.world_to_screen(away);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        assert!(!h.is_dragging_area_select());
        h.pointer_move_screen(s.x + 1.0, s.y, false, false, false);
        let mid = h.drain_events_json();
        assert!(!mid.contains("preselect"), "background click path must not emit preselect");
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        assert!(h.selection.contains("a"));
        h.pointer_up_screen(s.x, s.y, false, false, false);
        assert!(h.selection.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        let fin = h.drain_events_json();
        assert!(fin.contains("select"));
        assert!(!fin.contains("preselect"));
        assert!(fin.contains("\"exitHighlightIds\":[]"));
    }

    #[test]
    fn board_host_background_click_without_drag_clears_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let away = Point::new(5000.0, 5000.0);
        let s = h.world_to_screen(away);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_up_screen(s.x, s.y, false, false, false);
        assert!(h.selection.is_empty());
    }

    #[test]
    fn board_host_rectangle_area_select_includes_handles_with_nodes() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "invertive", true, true, true);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let w0 = Point::new(-90.0, -70.0);
        let w1 = Point::new(90.0, 90.0);
        let s0 = h.world_to_screen(w0);
        let s1 = h.world_to_screen(w1);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let mut got: Vec<_> = h.selection.iter().cloned().collect();
        got.sort();
        assert!(got.contains(&"a".to_string()));
        assert!(got.contains(&"a:h0".to_string()));
    }

    #[test]
    fn board_host_area_select_preselect_matches_selected_chrome() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        let w_down = Point::new(350.0, -50.0);
        let w_mid = Point::new(270.0, 50.0);
        let w_end = Point::new(265.0, 48.0);
        let s_down = h.world_to_screen(w_down);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        assert!(!h.is_dragging_area_select());
        let _ = h.drain_events_json();
        let s_mid = h.world_to_screen(w_mid);
        let s_end = h.world_to_screen(w_end);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
        assert!(h.is_dragging_area_select());
        let _ = h.drain_events_json();
        assert!(h.preselect.contains("b"), "preview should include node b");
        assert!(h.preselect_removed.contains("a"));
        assert!(h.selection_exit_highlight.is_empty());
        assert!(!h.selection.contains("b"), "committed selection unchanged during preselect");
        let frozen = h.preselect_removed.clone();
        h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
        let _ = h.drain_events_json();
        assert_eq!(frozen, h.preselect_removed);
        h.pointer_up_screen(s_end.x, s_end.y, false, false, false);
        let _ = h.drain_events_json();
        assert!(h.selection.contains("b"));
        assert!(!h.selection.contains("a"));
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
    }

    #[test]
    fn board_host_area_select_from_empty_keeps_selection_until_commit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&[]);
        let _ = h.drain_events_json();
        let w_down = Point::new(350.0, -50.0);
        let w_mid = Point::new(270.0, 50.0);
        let s_down = h.world_to_screen(w_down);
        let s_mid = h.world_to_screen(w_mid);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
        let _ = h.drain_events_json();
        assert!(h.is_dragging_area_select());
        assert!(h.preselect.contains("b"));
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection.is_empty());
        h.pointer_up_screen(s_mid.x, s_mid.y, false, false, false);
        let _ = h.drain_events_json();
        assert!(h.selection.contains("b"));
        assert!(h.preselect.is_empty());
    }

    #[test]
    fn board_host_minimap_pointer_move_hovers_node_under_cursor() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        h.set_forced_draw_lod_label("minimap");
        h.sync_descriptor(&sample_scene()).unwrap();
        let center = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(center.x, center.y, false, false, false);
        assert_eq!(h.hovered_id.as_deref(), Some("a"));
        let away = h.world_to_screen(Point::new(5000.0, 5000.0));
        h.pointer_move_screen(away.x, away.y, false, false, false);
        assert!(h.hovered_id.is_none());
    }

    #[test]
    fn board_host_minimap_preselect_matches_selected_chrome() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids(&["b".into()]);
        let _ = h.drain_events_json();
        let selected_hint = h.encoded_scene_hint();
        assert!(selected_hint > neutral_hint, "minimap selected chrome should add visible vector encoding over neutral state");
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        let w_down = Point::new(350.0, -50.0);
        let w_end = Point::new(265.0, 48.0);
        let s_down = h.world_to_screen(w_down);
        let s_end = h.world_to_screen(w_end);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        h.pointer_move_screen(s_end.x, s_end.y, false, false, false);
        assert!(h.is_dragging_area_select());
        assert!(h.preselect.contains("b"));
        h.set_selection_screen_preview(None);
        let preselect_hint = h.encoded_scene_hint();
        assert!(preselect_hint > neutral_hint, "minimap preselect should add visible selected chrome over neutral minimap rendering");
    }

    #[test]
    fn board_host_silent_preselect_applies_selected_chrome_without_area_drag() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let neutral_hint = h.encoded_scene_hint();
        assert!(!matches!(h.interaction, Interaction::Selection { .. }));
        h.set_preselect_state_silent(&["b".into()], &[]);
        assert!(h.nodes.get("b").is_some_and(|n| n.selected));
        assert!(h.nodes.get("a").is_some_and(|n| !n.selected));
        let preselect_hint = h.encoded_scene_hint();
        assert!(preselect_hint > neutral_hint, "silent minimap preselect should paint selected chrome without an active area-select interaction");
    }

    #[test]
    fn board_host_hover_tracks_visible_wires() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.edges.clear();
        desc.wires.push(WireDescJson { id: "w1".into(), source: "a:h0".into(), target: None, end_x: Some(220.0), end_y: Some(0.0), selected: None, style: None, wire_kind: None, user_data: None, visible: None, locked: None });
        h.sync_descriptor(&desc).unwrap();
        let source = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let curve = compute_edge_bezier_points(source, Point::new(220.0, 0.0), Point::new(0.0, 0.0), Point::new(220.0, 0.0));
        let probe = cubic_bezier_point(curve, 0.5);
        h.update_hover_from_world(probe);
        assert_eq!(h.hovered_id.as_deref(), Some("w1"));
    }

    fn link_test_scene_no_edge() -> SceneDescriptorJson {
        SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    root: None,
                    shape: Some("circle".into()),
                    radius: Some(40.0),
                    width: None,
                    height: None,
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 280.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    root: None,
                    shape: Some("circle".into()),
                    radius: Some(40.0),
                    width: None,
                    height: None,
                    scale: None,
                },
            ],
            handles: vec![
                HandleDescJson {
                    id: "a:h0".into(),
                    node_id: "a".into(),
                    angle: 0.0,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("parent".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
                HandleDescJson {
                    id: "b:h0".into(),
                    node_id: "b".into(),
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("child".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        }
    }

    fn link_test_scene_no_edge_non_draggable_nodes() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        for n in &mut s.nodes {
            n.draggable = Some(false);
        }
        s
    }

    fn link_test_scene_node_a_two_free_handles() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.handles.push(HandleDescJson {
            id: "a:h1".into(),
            node_id: "a".into(),
            angle: std::f64::consts::FRAC_PI_2,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("parent".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s
    }

    fn link_test_scene_b_two_free_child_handles() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.handles.push(HandleDescJson {
            id: "b:h1".into(),
            node_id: "b".into(),
            angle: 0.0,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("child".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s
    }

    fn link_test_scene_target_b_handle_busy() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.nodes.push(NodeDescJson {
            id: "c".into(),
            x: 560.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        s.handles.push(HandleDescJson {
            id: "c:h0".into(),
            node_id: "c".into(),
            angle: std::f64::consts::PI,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("child".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s.edges.push(EdgeDescJson { id: "e-bc".into(), source: "b:h0".into(), target: "c:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
        s
    }

    fn link_test_scene_a_to_b_linked() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.edges.push(EdgeDescJson { id: "e-ab".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None, source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None });
        s
    }

    fn link_test_scene_node_a_two_handles_one_busy() -> SceneDescriptorJson {
        let mut s = link_test_scene_a_to_b_linked();
        s.handles.push(HandleDescJson {
            id: "a:h1".into(),
            node_id: "a".into(),
            angle: std::f64::consts::FRAC_PI_2,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("parent".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            locked: None,
            scale: None,
        });
        s
    }

    #[test]
    fn board_host_node_drag_proximity_connect_overlapping_compatible_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let center_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
        let overlap = h.world_to_screen(Point::new(60.0, 0.0));
        h.pointer_move_screen(overlap.x, overlap.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { proximity_pair: Some(_), .. }), "expected proximity preview wire while overlapping compatible nodes");
        h.pointer_up_screen(overlap.x, overlap.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
        assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
        assert!(ev.contains("b:h0"));
        assert!(ev.contains("a:h0"));
    }

    #[test]
    fn board_host_node_drag_skips_proximity_when_moving_node_has_incident_edge() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        let _ = h.drain_events_json();
        let center_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
        let overlap = h.world_to_screen(Point::new(60.0, 0.0));
        h.pointer_move_screen(overlap.x, overlap.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { proximity_pair: None, .. }), "connected moving node must not preview node-drag proximity");
        h.pointer_up_screen(overlap.x, overlap.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("proximityConnect"), "expected no proximityConnect, got: {ev}");
    }

    #[test]
    fn board_host_link_drag_snap_emits_edge_create() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
        let created: Vec<_> = h.edges.keys().filter(|k| k.starts_with("edge-link-")).cloned().collect();
        assert_eq!(created.len(), 1);
    }

    #[test]
    fn board_host_link_drag_snap_micro_zoom_rectangle_compatible_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_micro_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id":"core.rect.bottom","name":"B","color":"#112233","defaultWireKind":"link.w"},
                    {"id":"core.rect.top","name":"T","color":"#112233","defaultWireKind":"link.w"}
                ],
                "wireKinds": [{"id":"link.w","name":"W","defaultEdgeKind":"link.e"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(r#"[{"source":"core.rect.bottom","target":"core.rect.top","specificity":"handle"}]"#).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
                    y: 100.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    root: None,
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(100.0),
                    height: Some(56.0),
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 0.0,
                    y: 20.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    root: None,
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(100.0),
                    height: Some(56.0),
                    scale: None,
                },
            ],
            handles: vec![
                HandleDescJson {
                    id: "a:h0".into(),
                    node_id: "a".into(),
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("core.rect.bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
                HandleDescJson {
                    id: "b:h0".into(),
                    node_id: "b".into(),
                    angle: 0.0,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("core.rect.top".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    scale: None,
                },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let pa = handle_position_on_rectangle(Point::new(0.0, 100.0), 100.0, 56.0, std::f64::consts::PI);
        let pb = handle_position_on_rectangle(Point::new(0.0, 20.0), 100.0, 56.0, 0.0);
        let s0 = h.world_to_screen(pa);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let mid = Point::new(0.0, 60.0);
        let s_mid = h.world_to_screen(mid);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(pb);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::LinkDragSnap { ref target_id, .. } if target_id.as_deref() == Some("b:h0")), "expected drag snap onto b:h0 at micro zoom");
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
        assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
    }

    #[test]
    fn board_host_link_drag_snap_proximity_connect_in_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge_non_draggable_nodes()).unwrap();
        let _ = h.drain_events_json();
        let center_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(center_a.x, center_a.y, 0, false, false);
        h.pointer_up_screen(center_a.x, center_a.y, false, false, false);
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate at overview LOD, got: {ev}");
        assert!(ev.contains("proximityConnect") || ev.contains("indirectConnect"), "expected proximityConnect or indirectConnect, got: {ev}");
    }

    #[test]
    fn board_host_parses_mindmap_fixture_without_handles() {
        let mut h = BoardHost::new_normal();
        let fixture = json!({
            "schema": "reasoning.mindmap.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "width": 48.0, "height": 48.0, "shape": "rectangle", "root": true },
                { "id": "b", "x": 120.0, "y": 0.0, "width": 40.0, "height": 40.0, "shape": "rectangle" }
            ],
            "edges": [
                { "id": "e1", "source": "a", "target": "b", "edgeKind": "wires.owns" }
            ]
        });
        assert!(h.parse_fixture_v1(&fixture));
        assert_eq!(h.port_mode, GraphPortMode::Normal);
        assert!(h.handles.is_empty());
        assert_eq!(h.edges.len(), 1);
        assert_eq!(h.edges.get("e1").unwrap().source, "a");
        assert_eq!(h.edges.get("e1").unwrap().target, "b");
        h.set_size(800, 600, 1.0);
        let scene = h.build_vector_scene();
        assert!(scene.path_count() > 0);
    }

    #[test]
    fn board_host_ingests_edge_and_node_kind_catalog_visual_fields() {
        let mut h = BoardHost::new_normal();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "edgeKinds": [
                    {"id":"wires.owns","name":"Owns","color":"#ff0000","stroke":"3","pattern":"dashed","targetTip":"filled-diamond","directed":false},
                    {"id":"wires.is","name":"Is","color":"#00ff00","pattern":"dotted","targetTip":"filled-arrow","directed":false}
                ],
                "nodeKinds": [
                    {"id":"capsule","name":"Capsule","shape":"circle","color":"#aabbcc"}
                ]
            })
            .to_string(),
        )
        .unwrap();
        let owns = h.edge_kinds.get("wires.owns").expect("owns edge kind");
        assert_eq!(owns.stroke_width, 3.0);
        assert_eq!(owns.pattern, EdgeStrokePattern::Dashed);
        assert_eq!(owns.target_tip.as_deref(), Some("filled-diamond"));
        assert!(!owns.directed);
        assert!(owns.color.is_some());
        let is = h.edge_kinds.get("wires.is").expect("is edge kind");
        assert_eq!(is.pattern, EdgeStrokePattern::Dotted);
        assert_eq!(is.target_tip.as_deref(), Some("filled-arrow"));
        assert!(!is.directed);
        let diamond = h.edge_tips.get("filled-diamond").expect("filled-diamond tip");
        assert_eq!(diamond.geometry, EdgeTipGeometry::Diamond);
        assert!(diamond.filled);
        let capsule = h.node_kinds.get("capsule").expect("capsule node kind");
        assert_eq!(capsule.shape, NodeShape::Circle);
        assert!(capsule.color_fill.is_some());
    }

    #[test]
    fn board_host_sync_descriptor_normal_graph_node_id_edges() {
        let mut h = BoardHost::new_normal();
        let desc = SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    root: Some(true),
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(48.0),
                    height: Some(48.0),
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 120.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    locked: None,
                    root: None,
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(40.0),
                    height: Some(40.0),
                    scale: None,
                },
            ],
            handles: vec![],
            edges: vec![EdgeDescJson { id: "e1".into(), source: "a".into(), target: "b".into(), edge_kind: Some("wires.owns".into()), source_tip: None, target_tip: None, selected: None, style: None, user_data: None, visible: None, locked: None }],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        assert!(h.handles.is_empty());
        assert_eq!(h.edges.get("e1").unwrap().source, "a");
        assert_eq!(h.edges.get("e1").unwrap().target, "b");
        h.set_size(800, 600, 1.0);
        let scene = h.build_vector_scene();
        assert!(scene.path_count() > 0);
    }

    #[test]
    fn board_host_hidden_handle_blocks_proximity_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port", "hidden": true }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "hidden handle should block connect, got: {ev}");
        assert!(h.edges.is_empty());
    }

    #[test]
    fn board_host_hidden_node_blocks_indirect_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "parent" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "hidden": true,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "child" }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
        let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(inside_b.x, inside_b.y, false, false, false);
        h.pointer_up_screen(inside_b.x, inside_b.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "hidden node should block indirect connect, got: {ev}");
        assert!(matches!(h.interaction, Interaction::None));
        assert!(h.edges.is_empty());
    }

    #[test]
    fn board_host_locked_node_blocks_hit_select() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes[0].locked = Some(true);
        h.sync_descriptor(&desc).unwrap();
        assert!(h.resolve_hit_world(Point::new(0.0, 0.0)).is_none());
        h.update_hover_from_world(Point::new(0.0, 0.0));
        assert_ne!(h.hovered_id.as_deref(), Some("a"));
    }

    #[test]
    fn board_host_locked_handle_blocks_proximity_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port", "locked": true }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "locked handle should block connect, got: {ev}");
        assert!(h.edges.is_empty());
    }

    #[test]
    fn board_host_overview_lod_omits_direct_handle_resolve_hit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 3.0, hp.y);
        assert_ne!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_link_rejects_incompatible_handle_kind_pairs() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
    }

    #[test]
    fn board_host_link_accepts_matching_handle_kind_pair() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[test]
    fn board_host_normal_lod_prefers_node_at_center_and_handle_off_rim() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 2.0, hp.y);
        assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_indirect_ring_resolve_skips_connected_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_handles_one_busy()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring_busy = h.indirect_handle_world_pos(ha0).unwrap();
        assert_ne!(h.resolve_hit_world(ring_busy).as_deref(), Some("a:h0"));
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a:h1"));
    }

    #[test]
    fn board_host_indirect_sole_compatible_drop_creates_edge_immediately() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkAtSourceHandle { ref source_id, .. } if source_id == "a:h0"
        ));
        let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(inside_b.x, inside_b.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::LinkDragSnap { .. }));
        h.pointer_up_screen(inside_b.x, inside_b.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("indirectConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
    }

    #[test]
    fn board_host_indirect_two_compatible_child_handles_on_target_require_ring_pick() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(sb.x, sb.y, false, false, false);
        h.pointer_up_screen(sb.x, sb.y, false, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkTargetNode { ref target_node_id, .. } if target_node_id == "b"
        ));
        let b0 = h.handles.get("b:h0").unwrap();
        let ring0 = h.indirect_handle_world_pos(b0).unwrap();
        let s1 = h.world_to_screen(ring0);
        h.pointer_down_screen(s1.x, s1.y, 0, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("indirectConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
    }

    #[test]
    fn board_host_indirect_target_click_elsewhere_stops_wire() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let target_center = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(target_center.x, target_center.y, false, false, false);
        h.pointer_up_screen(target_center.x, target_center.y, false, false, false);
        assert!(matches!(h.interaction, Interaction::LinkTargetNode { .. }));
        h.pointer_down_screen(20.0, 20.0, 0, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        assert!(h.edges.is_empty());
    }

    #[test]
    fn board_host_indirect_ring_shown_when_node_has_two_free_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_indirect_ring_paints_without_rebuilding_world_cache() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("a:h0"));
        assert!(h.encoded_scene_hint() > neutral_hint, "indirect ring must paint in the live overlay, not only in stale cached geometry");
        h.set_selection_ids_silent(&[]);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[test]
    fn board_host_link_drag_emits_compatible_nodes_and_target_ring_events() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(sb.x, sb.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("linkCompatibleNodes"), "got: {ev}");
        assert!(ev.contains(r#""nodeIds":["b"]"#) || ev.contains(r#""nodeIds": ["b"]"#), "got: {ev}");
        assert!(ev.contains("linkTargetRing"), "got: {ev}");
        assert!(ev.contains("b:h0") && ev.contains("b:h1"), "got: {ev}");
        let ring = h.indirect_handle_world_pos(h.handles.get("b:h1").unwrap()).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("b:h1"));
        h.pointer_up_screen(20.0, 20.0, false, false, false);
        let ev_end = h.drain_events_json();
        assert!(ev_end.contains("linkCompatibleNodes"));
        assert!(ev_end.contains(r#""nodeIds":[]"#) || ev_end.contains(r#""nodeIds": []"#));
        assert!(ev_end.contains("linkTargetRing"));
    }

    #[test]
    fn board_host_indirect_ring_gap_scales_with_node_across_zoom() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let ha = h.handles.get("a:h0").unwrap().clone();
        let node_r = 40.0_f64;
        let body = || handle_position_on_circle(Point::new(0.0, 0.0), node_r, 0.0);
        let gap_ratio = |host: &BoardHost| {
            let ring = host.indirect_handle_world_pos(&ha).unwrap();
            let gap_px = distance_between(host.world_to_screen(ring), host.world_to_screen(body()));
            gap_px / (node_r * host.camera.zoom)
        };
        h.set_camera(0.0, 0.0, 1.0);
        let ratio_z1 = gap_ratio(&h);
        let gap_px_z1 = node_r * ratio_z1;
        h.set_camera(0.0, 0.0, 4.25);
        let ratio_z2 = gap_ratio(&h);
        let gap_px_z2 = node_r * 4.25 * ratio_z2;
        assert!((ratio_z1 - ratio_z2).abs() < 1e-6, "rim-to-ring ratios differ: {ratio_z1} vs {ratio_z2}");
        assert!((ratio_z1 - 0.7).abs() < 1e-6);
        assert!((gap_px_z2 - gap_px_z1 * 4.25).abs() < 0.6, "screen gap should scale with zoom: {gap_px_z1} vs {gap_px_z2}");
    }

    #[test]
    fn board_host_indirect_handle_marker_radius_scales_with_node_extent() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let ha = h.handles.get("a:h0").unwrap();
        assert!((h.indirect_handle_marker_radius_world(ha) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn board_host_handle_scale_combines_node_and_kind_scales() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"slot-a","name":"Slot A","color":"#112233","scale":2.0}],
                "nodeKinds": [{"id":"kind-a","name":"Kind A","scale":1.5}],
            })
            .to_string(),
        )
        .unwrap();
        let mut desc = link_test_scene_no_edge();
        desc.nodes[0].node_kind = Some("kind-a".into());
        desc.nodes[0].scale = Some(2.0);
        desc.handles[0].handle_kind = Some("slot-a".into());
        desc.handles[0].scale = Some(0.5);
        h.sync_descriptor(&desc).unwrap();
        let ha = h.handles.get("a:h0").unwrap();
        assert_eq!(h.resolve_hit_world(Point::new(120.0, 0.0)).as_deref(), Some("a:h0"));
        assert!((h.indirect_handle_marker_radius_world(ha) - 96.0).abs() < 1e-6);
    }

    #[test]
    fn board_host_link_wire_specificity_allows_when_handle_row_absent() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],
                "wireKinds": [{"id":"flow.wire","name":"W","defaultEdgeKind":"flow.edge"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(r#"[{"source":"flow.wire","target":"child","specificity":"wire"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[test]
    fn board_host_kind_catalog_accepts_modern_hsl_handle_colors() {
        let mut h = BoardHost::new();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id":"space","name":"S","color":"hsl(206 52% 48%)"},
                    {"id":"comma","name":"C","color":"hsl(206, 52%, 48%)"},
                    {"id":"slash","name":"Sl","color":"hsl(206 52% 48% / 0.5)"},
                ],
            })
            .to_string(),
        )
        .unwrap();
        let c_space = h.handle_kinds.get("space").expect("space").color;
        let c_comma = h.handle_kinds.get("comma").expect("comma").color;
        let c_slash = h.handle_kinds.get("slash").expect("slash").color;
        assert_eq!(c_space, c_comma);
        assert_ne!(c_space, c_slash);
    }

    #[test]
    fn board_host_rejects_kind_catalog_rows_with_legacy_label() {
        let mut h = BoardHost::new();
        let err = h.set_board_kind_catalogs_from_json(&serde_json::json!({"handleKinds":[{"id":"h","label":"legacy","color":"#112233"}]}).to_string()).unwrap_err();
        assert!(err.to_string().contains("legacy label"));
    }

    #[test]
    fn board_host_link_important_pair_overrides_lower_specificity_filter() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],
                "wireKinds": [{"id":"flow.wire","name":"W"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(
            r#"[
				{"source":"flow.wire","target":"nope","specificity":"wire"},
				{"source":"parent","target":"child","specificity":"general","important":true}
			]"#,
        )
        .unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[test]
    fn board_host_link_drag_does_not_snap_when_target_handle_busy() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
        assert_eq!(h.edges.len(), 1);
        assert!(h.edges.contains_key("e-bc"));
    }

    #[test]
    fn board_host_link_does_not_start_from_busy_source_handle() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        assert!(!h.drain_events_json().contains("edgeCreate"));
    }

    #[test]
    fn board_host_indirect_does_not_commit_on_busy_target_handle() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let target_center = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(target_center.x, target_center.y, false, false, false);
        h.pointer_up_screen(target_center.x, target_center.y, false, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkTargetNode {
                ref source_id,
                ref target_node_id
            } if source_id == "a:h0" && target_node_id == "b"
        ));
        let _ = h.drain_events_json();
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(sb.x, sb.y, 0, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
        assert_eq!(h.edges.len(), 1);
        assert!(matches!(h.interaction, Interaction::None));
    }

    #[test]
    fn board_host_link_short_drag_does_not_emit_edge_create() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        h.pointer_move_screen(s0.x + 2.0, s0.y, false, false, false);
        h.pointer_up_screen(s0.x + 2.0, s0.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
    }

    #[test]
    fn board_host_brush_slot_emits_preview_and_place_on_leave() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        let catalogs = json!({
            "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
            "nodeKinds": [{
                "id": "brush.kind",
                "name": "Brush Kind",
                "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }]
            }]
        });
        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("a.kind".into()),
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![HandleDescJson {
                id: "a:h0".into(),
                node_id: "a".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some("port".into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let s = h.world_to_screen(slot);
        h.pointer_move_screen(s.x, s.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview, got: {ev}");
        h.pointer_leave_screen(true);
        let ev2 = h.drain_events_json();
        assert!(ev2.contains("brushPlace"), "expected brushPlace on leave with Alt, got: {ev2}");
        assert!(ev2.contains("brush.kind"));
        assert!(ev2.contains("a:h0"));
        assert!(ev2.contains("nodeId"));
        assert!(ev2.contains("edgeId"));
    }

    #[test]
    fn board_host_brush_open_slot_suggestions_commit_and_cancel() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 2.0);
        h.set_active_utility("select");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        let catalogs = json!({
            "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
            "nodeKinds": [{
                "id": "brush.kind",
                "name": "Brush Kind",
                "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }]
            }]
        });
        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("a.kind".into()),
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![HandleDescJson {
                id: "a:h0".into(),
                node_id: "a".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some("port".into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        h.brush_open_slot("a:h0");
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        assert!(ev.contains("brushPreview"), "expected brushPreview, got: {ev}");
        assert!(ev.contains("\"id\":\"a:h0\""), "expected hovered source handle, got: {ev}");
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let expected_x = hp.x + (hp.x - 0.0) * (40.0 / 40.0);
        assert!(ev.contains(&format!("\"x\":{expected_x}")), "preview should flush along handle normal, got: {ev}");
        h.brush_commit_slot();
        let ev_commit = h.drain_events_json();
        assert!(ev_commit.contains("brushPlace"), "expected brushPlace on commit, got: {ev_commit}");
        h.brush_open_slot("a:h0");
        let _ = h.drain_events_json();
        h.brush_cancel_slot();
        let ev_cancel = h.drain_events_json();
        assert!(!ev_cancel.contains("brushPlace"), "cancel should not place, got: {ev_cancel}");
    }

    #[test]
    fn board_host_brush_slot_commit_survives_pointer_move_out_of_slot() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let _ = h.drain_events_json();
        assert_eq!(h.nodes.len(), 2);
        let far = h.world_to_screen(Point::new(500.0, 500.0));
        h.pointer_move_screen(far.x, far.y, false, false, true);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPlace"), "expected brushPlace when leaving slot with Alt, got: {ev}");
    }

    #[test]
    fn board_host_brush_slot_skips_place_on_leave_without_alt() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{"handleKind": "child", "angle": 3.141592653589793}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let _ = h.drain_events_json();
        h.pointer_leave_screen(false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"), "expected no brushPlace without Alt, got: {ev}");
    }

    #[test]
    fn board_host_brush_fill_frontier_deterministic_and_collision_limited() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [
                        { "handleKind": "child", "angle": 0.0 },
                        { "handleKind": "child", "angle": 3.141592653589793 }
                    ]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let first = h.brush_fill_json(3, 42);
        let second = h.brush_fill_json(3, 42);
        assert_eq!(first, second, "fill must be deterministic for the same seed");
        let v: serde_json::Value = serde_json::from_str(&first).unwrap();
        let placements = v.get("placements").and_then(|x| x.as_array()).unwrap();
        assert!(!placements.is_empty(), "expected at least one fill placement");
        assert!(placements.len() <= 3);
        let many = h.brush_fill_json(1000, 99);
        let many_v: serde_json::Value = serde_json::from_str(&many).unwrap();
        let many_n = many_v.get("placements").and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0);
        assert!(many_n < 1000, "collision should cap fill before 1000 on a tight scene");
    }

    #[test]
    fn board_host_brush_fill_session_step_matches_brush_fill_json() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [
                        { "handleKind": "child", "angle": 0.0 },
                        { "handleKind": "child", "angle": 3.141592653589793 }
                    ]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&h.brush_fill_json(12, 77)).unwrap();
        h.brush_fill_session_begin(12, 77);
        let mut stepped: Vec<serde_json::Value> = Vec::new();
        let mut done = false;
        while !done {
            let chunk: serde_json::Value = serde_json::from_str(&h.brush_fill_session_step(4)).unwrap();
            done = chunk.get("done").and_then(|x| x.as_bool()).unwrap_or(true);
            if let Some(rows) = chunk.get("placements").and_then(|x| x.as_array()) {
                stepped.extend(rows.iter().cloned());
            }
        }
        h.brush_fill_session_clear();
        assert_eq!(stepped, expected.get("placements").and_then(|x| x.as_array()).cloned().unwrap_or_default());
    }

    #[test]
    fn board_host_fixture_drop_preview_json_paints_while_select_utility_active() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("select");
        h.set_fixture_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":200.0,"screenY":150.0,"shape":"circle","radius":20.0,"iconKind":"capsule_J"}"#).unwrap();
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
        h.set_fixture_drop_preview_json("").unwrap();
        assert!(h.encoded_scene_hint() > 0);
    }

    #[test]
    fn board_host_fixture_drop_preview_uses_catalog_shape_and_icon_at_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.05);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "nodeKinds": [{
                    "id": "capsule_J",
                    "name": "Capsule J",
                    "scale": 2.0,
                    "shape": "circle",
                    "icon": "capsule_J",
                    "handles": [{"handleKind": "door", "angle": 0.0}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.set_fixture_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":120.0,"screenY":90.0,"shape":"circle","radius":10.0,"iconKind":"capsule_J"}"#).unwrap();
        let hint_with_preview = h.encoded_scene_hint();
        assert!(hint_with_preview > 0);
        h.set_fixture_drop_preview_json("").unwrap();
        let hint_cleared = h.encoded_scene_hint();
        assert!(hint_cleared != hint_with_preview || hint_with_preview > 0);
    }

    #[test]
    fn board_host_brush_session_mirror_json_shows_preview_without_pointer() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_utility("brush");
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id": "parent", "name": "Parent", "color": "#888888"}],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{"handleKind": "parent", "angle": 3.141592653589793}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let session = serde_json::json!({
            "sourceHandleId": "a:h0",
            "candidates": ["brush.kind"],
            "index": 0,
            "preview": {
                "node": {
                    "nodeKind": "brush.kind",
                    "x": 120.0,
                    "y": 0.0,
                    "shape": "circle",
                    "radius": 20.0,
                    "handles": [{"handleKind": "parent", "angle": 3.141592653589793}]
                },
                "edge": { "sourceHandleId": "a:h0", "targetHandleIndex": 0 }
            }
        });
        h.set_brush_session_mirror_json(&session.to_string()).unwrap();
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
    }

    #[test]
    fn board_host_brush_candidates_sorted_by_handle_proximity() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [
                    {
                        "id": "light",
                        "name": "Light",
                        "handles": [
                            {"handleKind": "child", "angle": 0.0},
                            {"handleKind": "child", "angle": 3.141592653589793}
                        ]
                    },
                    {
                        "id": "heavy",
                        "name": "Heavy",
                        "handles": [{"handleKind": "child", "angle": 3.141592653589793}]
                    }
                ]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let candidates =
            v.as_array().and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).and_then(|c| c.as_array()).cloned());
        assert_eq!(candidates.as_ref().map(|rows| rows.len()), Some(3));
        let first_kind = candidates.as_ref().and_then(|rows| rows.first()).and_then(|row| row.get("nodeKind")).and_then(|x| x.as_str());
        assert_eq!(first_kind, Some("heavy"));
    }

    #[test]
    fn board_host_brush_lists_every_compatible_handle_per_node_kind() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "dual",
                    "name": "Dual",
                    "handles": [
                        {"handleKind": "child", "angle": 0.0},
                        {"handleKind": "child", "angle": 3.141592653589793}
                    ]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.drain_events_json();
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let candidates = v
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).and_then(|c| c.as_array()).cloned())
            .unwrap_or_default();
        assert_eq!(candidates.len(), 2, "expected one row per compatible handle, got: {ev}");
        let indices: Vec<u64> = candidates.iter().filter_map(|row| row.get("targetHandleIndex").and_then(|i| i.as_u64())).collect();
        assert!(indices.contains(&0));
        assert!(indices.contains(&1));
    }

    #[test]
    fn board_host_fill_base_core_rectangular_excludes_cylindric_tambour() {
        const BASE_KIND: &str = "Base";
        const CYLINDRIC_TAMBOUR_KIND: &str = "Cylindric Tambour";
        const FIRST_STOREY_KIND: &str = "First Storey Tambour";
        let mut h = BoardHost::new();
        h.set_suggestion_offset(80.0);
        h.set_brush_node_size(40.0);
        use store::DocumentDsl;
        let fixture: serde_json::Value = serde_json::to_value(<puzzle_2d::Puzzle2dProjection as store::DocumentDsl>::parse_dsl(include_str!("../../../../2d/example/nakagin-capsule-tower.puzzle2d")).unwrap()).unwrap();
        let compat_str = fixture.get("meta").and_then(|m| m.get("kindCompatibility")).map(|v| v.to_string()).unwrap_or_else(|| "[]".to_string());
        h.set_handle_link_compat_from_json(&compat_str).unwrap();
        let catalogs_str = fixture
            .get("meta")
            .and_then(|m| m.get("kindCatalogs"))
            .map(|kc| {
                serde_json::json!({
                    "handleKinds": kc.get("handles"),
                    "nodeKinds": kc.get("nodes"),
                })
                .to_string()
            })
            .unwrap_or_else(|| "{}".to_string());
        h.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "base".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: Some("base".into()),
                node_kind: Some(BASE_KIND.into()),
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(20.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![
                HandleDescJson {
                    id: "base:c0".into(),
                    node_id: "base".into(),
                    angle: -2.3561944901923453,
                    radius: Some(3.0),
                    scale: None,
                    selected: None,
                    visible: None,
                    locked: None,
                    style: None,
                    handle_kind: Some("core rectangular bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                },
                HandleDescJson {
                    id: "base:c1".into(),
                    node_id: "base".into(),
                    angle: -0.7853981633974483,
                    radius: Some(3.0),
                    scale: None,
                    selected: None,
                    visible: None,
                    locked: None,
                    style: None,
                    handle_kind: Some("core rectangular bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let out: serde_json::Value = serde_json::from_str(&h.brush_fill_json(1, 7)).unwrap();
        let placements = out.get("placements").and_then(|x| x.as_array()).unwrap();
        assert_eq!(placements.len(), 1, "expected one fill placement on base");
        let node_kind = placements[0].get("nodeKind").and_then(|x| x.as_str()).unwrap_or("");
        assert_ne!(node_kind, CYLINDRIC_TAMBOUR_KIND, "cylindric tambour must not stack on rectangular core");
        assert_eq!(node_kind, FIRST_STOREY_KIND, "first storey tambour matches rectangular core stack");
    }

    #[test]
    fn board_host_brush_door_tambour_left_excludes_capital_with_metabolism_compat_rules() {
        const DOOR_TAMBOUR_LEFT: &str = "door tambour left";
        const CAPITAL_KIND: &str = "Capital";
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        use store::DocumentDsl;
        let fixture: serde_json::Value = serde_json::to_value(<puzzle_2d::Puzzle2dProjection as store::DocumentDsl>::parse_dsl(include_str!("../../../../2d/example/nakagin-capsule-tower.puzzle2d")).unwrap()).unwrap();
        let compat_str = fixture.get("meta").and_then(|m| m.get("kindCompatibility")).map(|v| v.to_string()).unwrap_or_else(|| "[]".to_string());
        h.set_handle_link_compat_from_json(&compat_str).unwrap();
        let catalogs_str = fixture
            .get("meta")
            .and_then(|m| m.get("kindCatalogs"))
            .map(|kc| {
                serde_json::json!({
                    "handleKinds": kc.get("handles"),
                    "nodeKinds": kc.get("nodes"),
                })
                .to_string()
            })
            .unwrap_or_else(|| "{}".to_string());
        h.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "tambour".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("Tambour".into()),
                user_data: None,
                visible: None,
                locked: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![HandleDescJson {
                id: "tambour:h0".into(),
                node_id: "tambour".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                locked: None,
                style: None,
                handle_kind: Some(DOOR_TAMBOUR_LEFT.into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let slot_screen = h.world_to_screen(slot);
        h.pointer_move_screen(slot_screen.x, slot_screen.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let candidates = v
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates")).and_then(|row| row.get("payload")).and_then(|p| p.get("candidates")).cloned())
            .and_then(|c| c.as_array().cloned())
            .unwrap_or_default();
        let ids: Vec<String> = candidates.iter().filter_map(|x| x.as_str().map(str::to_string)).collect();
        assert!(!ids.iter().any(|id| id == CAPITAL_KIND), "door tambour left must not suggest Capital, got: {ids:?}");
    }

    #[test]
    fn board_host_brush_slot_accepts_pointer_on_node_body_at_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview when hovering node body at overview LOD, got: {ev}");
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
    }

    #[test]
    fn board_host_brush_slot_accepts_pointer_on_indirect_ring_anchor() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_utility("brush");
        h.set_suggestion_offset(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        let s = h.world_to_screen(ring);
        h.pointer_move_screen(s.x, s.y, false, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview on indirect ring anchor, got: {ev}");
    }
}

#[cfg(test)]

#[cfg(test)]
mod force_graph_tests {
    use crate::graph::apply_edge_handle_snap_to_fixture_v1_json;
    use crate::{apply_force_graph_layout_to_fixture_v1_json, apply_normal_undirected_redraw_layout_to_fixture_v1_json};
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn force_graph_spreads_two_linked_circles_along_x() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 1.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 8000.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected horizontal separation, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_pins_locked_node_positions() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 40.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "iterations": 180,
            "idealEdgeLength": 160.0,
            "repulsionStrength": 7500.0,
            "springStrength": 0.045,
            "gravity": 0.0,
            "randomSeed": 101,
            "lockedNodeIds": ["a"]
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let ay = nodes[0]["y"].as_f64().unwrap();
        assert!((ax - 0.0).abs() < 1e-9 && (ay - 0.0).abs() < 1e-9);
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - 40.0).abs() > 25.0, "expected free node to move, bx={bx}");
    }

    #[test]
    fn redraw_force_graph_top_level_locked_node_ids_pins() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 40.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "lockedNodeIds": ["a"],
            "randomSeed": 101,
            "redrawHandlesAfter": false,
            "forceGraph": {
                "iterations": 180,
                "idealEdgeLength": 160.0,
                "repulsionStrength": 7500.0,
                "springStrength": 0.045,
                "gravity": 0.0
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        assert!((nodes[0]["x"].as_f64().unwrap() - 0.0).abs() < 1e-9);
        assert!((nodes[0]["y"].as_f64().unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn redraw_force_graph_mindmap_schema_uses_undirected_layout() {
        let fixture = json!({
            "schema": "reasoning.mindmap.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0 },
                { "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0 }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 0.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = apply_normal_undirected_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected mindmap undirected springs, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_normal_mode_node_id_edges_apply_spring_forces() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0, "handles": [] },
                { "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0, "handles": [] }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 0.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected node-id edge springs to spread nodes, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_rejects_bad_schema() {
        let err = apply_force_graph_layout_to_fixture_v1_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();
        assert!(err.contains("schema"));
    }

    #[test]
    fn force_graph_barnes_hut_many_bodies_yields_finite_coordinates() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for k in 0..64 {
            let id = format!("n{k}");
            nodes.push(json!({
                "id": id,
                "x": (k % 8) as f64 * 12.0,
                "y": (k / 8) as f64 * 12.0,
                "radius": 8.0,
                "handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]
            }));
            if k > 0 {
                let prev = format!("n{}", k - 1);
                edges.push(json!({
                    "id": format!("e{k}"),
                    "source": format!("{prev}:h0"),
                    "target": format!("{id}:h0")
                }));
            }
        }
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": nodes,
            "edges": edges
        });
        let opts = json!({
            "iterations": 180,
            "idealEdgeLength": 90.0,
            "repulsionStrength": 6000.0,
            "springStrength": 0.05,
            "gravity": 0.01,
            "randomSeed": 91,
            "barnesHutTheta": 0.72,
            "pairwiseRepulsionMaxBodies": 12
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for row in parsed["nodes"].as_array().unwrap() {
            let x = row["x"].as_f64().unwrap();
            let y = row["y"].as_f64().unwrap();
            assert!(x.is_finite() && y.is_finite());
        }
        let xs: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["x"].as_f64().unwrap()).collect();
        let ys: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["y"].as_f64().unwrap()).collect();
        let x_span = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) - xs.iter().copied().fold(f64::INFINITY, f64::min);
        let y_span = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) - ys.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(x_span > 40.0 && y_span > 35.0, "expected BH layout to spread graph, x_span={x_span} y_span={y_span}");
    }

    #[test]
    fn force_graph_bh_layout_is_deterministic_for_fixed_seed() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for k in 0..36 {
            let id = format!("n{k}");
            nodes.push(json!({
                "id": id,
                "x": (k % 6) as f64 * 9.0,
                "y": (k / 6) as f64 * 9.0,
                "radius": 6.5,
                "handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]
            }));
            if k > 0 {
                let prev = format!("n{}", k - 1);
                edges.push(json!({
                    "id": format!("e{k}"),
                    "source": format!("{prev}:h0"),
                    "target": format!("{id}:h0")
                }));
            }
        }
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": nodes,
            "edges": edges
        });
        let opts = json!({
            "iterations": 120,
            "idealEdgeLength": 88.0,
            "repulsionStrength": 5400.0,
            "springStrength": 0.047,
            "gravity": 0.013,
            "randomSeed": 4041,
            "barnesHutTheta": 0.55,
            "pairwiseRepulsionMaxBodies": 8
        });
        let s = fixture.to_string();
        let o = opts.to_string();
        let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        assert_eq!(out_a, out_b, "BH path must be bitwise reproducible for identical inputs");
    }

    #[test]
    fn force_graph_pairwise_layout_is_deterministic_for_fixed_seed() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 30.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },
                { "id": "b", "x": 3.0, "y": 1.0, "radius": 30.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },
                { "id": "c", "x": -2.0, "y": 4.0, "radius": 28.0, "handles": [{ "id": "c:h0", "angle": 1.0, "handleKind": "port" }] }
            ],
            "edges": [
                { "id": "e1", "source": "a:h0", "target": "b:h0" },
                { "id": "e2", "source": "b:h0", "target": "c:h0" }
            ]
        });
        let opts = json!({
            "iterations": 90,
            "idealEdgeLength": 110.0,
            "repulsionStrength": 6200.0,
            "springStrength": 0.042,
            "gravity": 0.011,
            "randomSeed": 909,
            "pairwiseRepulsionMaxBodies": 80
        });
        let s = fixture.to_string();
        let o = opts.to_string();
        let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        assert_eq!(out_a, out_b);
    }

    #[test]
    fn force_graph_clamped_barnes_hut_theta_runs_without_error() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },
                { "id": "b", "x": 5.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },
                { "id": "c", "x": 2.0, "y": 8.0, "radius": 18.0, "handles": [{ "id": "c:h0", "angle": 0.0, "handleKind": "port" }] }
            ],
            "edges": [
                { "id": "e1", "source": "a:h0", "target": "b:h0" },
                { "id": "e2", "source": "b:h0", "target": "c:h0" }
            ]
        });
        let opts = json!({
            "iterations": 40,
            "idealEdgeLength": 100.0,
            "repulsionStrength": 5000.0,
            "springStrength": 0.05,
            "gravity": 0.01,
            "randomSeed": 3,
            "barnesHutTheta": 500.0,
            "pairwiseRepulsionMaxBodies": 2
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for row in parsed["nodes"].as_array().unwrap() {
            assert!(row["x"].as_f64().unwrap().is_finite());
            assert!(row["y"].as_f64().unwrap().is_finite());
        }
    }

    #[test]
    fn redraw_force_graph_wraps_flat_options() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 1.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 8000.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0);
    }

    #[test]
    fn edge_handle_snap_sets_circle_handle_angles_on_center_line() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 200.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let out = crate::graph::apply_edge_handle_snap_to_fixture_v1_json(&fixture.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
        let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
        assert!((ang_a - 0.0).abs() < 1e-6, "expected east on a, got {ang_a}");
        assert!((ang_b - std::f64::consts::PI).abs() < 1e-6, "expected west on b, got {ang_b}");
    }

    #[test]
    fn redraw_force_graph_with_snap_sets_handle_angles() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 200.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "redrawHandlesAfter": true,
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 8000.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
        let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        let ay = nodes[0]["y"].as_f64().unwrap();
        let by = nodes[1]["y"].as_f64().unwrap();
        let exp_a = f64::atan2(by - ay, bx - ax);
        let exp_b = f64::atan2(ay - by, ax - bx);
        let wrap_diff = |a: f64, b: f64| {
            let mut d = (a - b).rem_euclid(std::f64::consts::TAU);
            if d > std::f64::consts::PI {
                d -= std::f64::consts::TAU;
            }
            d.abs()
        };
        assert!(wrap_diff(ang_a, exp_a) < 0.03, "a angle {ang_a} vs exp {exp_a}");
        assert!(wrap_diff(ang_b, exp_b) < 0.03, "b angle {ang_b} vs exp {exp_b}");
    }

    #[test]
    fn force_graph_accepts_logical_nodes_without_xy() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "centerX": 0.0,
            "centerY": 0.0,
            "randomSeed": 3,
            "forceGraph": { "iterations": 120, "idealEdgeLength": 160.0, "gravity": 0.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for n in parsed["nodes"].as_array().unwrap() {
            assert!(n["x"].as_f64().unwrap().is_finite());
            assert!(n["y"].as_f64().unwrap().is_finite());
        }
    }

    #[test]
    fn hierarchical_tree_normal_mode_node_id_edges_stacks_by_depth() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "r", "root": true, "radius": 18.0, "handles": [] },
                { "id": "c1", "radius": 18.0, "handles": [] },
                { "id": "c2", "radius": 18.0, "handles": [] }
            ],
            "edges": [
                { "id": "e1", "source": "r", "target": "c1" },
                { "id": "e2", "source": "r", "target": "c2" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        let c2y = *ys.get("c2").unwrap();
        assert!((c1y - ry).abs() > 40.0, "expected child below root");
        assert!((c2y - ry).abs() > 40.0);
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
    }

    #[test]
    fn hierarchical_tree_stacks_by_depth() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c2",
                    "radius": 18.0,
                    "handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [
                { "id": "e1", "source": "r:h", "target": "c1:h" },
                { "id": "e2", "source": "r:h", "target": "c2:h" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        let c2y = *ys.get("c2").unwrap();
        assert!((c1y - ry).abs() > 40.0, "expected child below root");
        assert!((c2y - ry).abs() > 40.0);
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
    }

    #[test]
    fn hierarchical_tree_pins_locked_root_coordinates() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "x": 120.0,
                    "y": -33.0,
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c2",
                    "x": 5.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [
                { "id": "e1", "source": "r:h", "target": "c1:h" },
                { "id": "e2", "source": "r:h", "target": "c2:h" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "lockedNodeIds": ["r"],
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
        }
        let (rx, ry) = *by_id.get("r").unwrap();
        assert!((rx - 120.0).abs() < 1e-3 && (ry + 33.0).abs() < 1e-3, "locked root moved: {rx},{ry}");
        let (_c1x, c1y) = *by_id.get("c1").unwrap();
        let (_c2x, c2y) = *by_id.get("c2").unwrap();
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
        assert!((c1y - ry).abs() > 40.0, "children laid relative to tree, root stayed pinned");
    }

    #[test]
    fn redraw_hierarchical_tree_nested_locked_node_ids_pins() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "x": 77.0,
                    "y": 12.0,
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": {
                "direction": "downwards",
                "layerSpacing": 90.0,
                "siblingGap": 12.0,
                "lockedNodeIds": ["r"]
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
        }
        let (rx, ry) = *by_id.get("r").unwrap();
        assert!((rx - 77.0).abs() < 1e-3 && (ry - 12.0).abs() < 1e-3, "nested locked list ignored: {rx},{ry}");
    }

    #[test]
    fn hierarchical_tree_right_places_children_larger_x_than_root() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "right", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut xs: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            xs.insert(id, n["x"].as_f64().unwrap());
        }
        let rx = *xs.get("r").unwrap();
        let c1x = *xs.get("c1").unwrap();
        assert!(c1x > rx + 40.0, "expected child to the right of root");
    }

    #[test]
    fn hierarchical_tree_upwards_places_children_smaller_y_than_root() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "upwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        assert!(c1y < ry - 40.0, "expected child above root (smaller y)");
    }

    #[test]
    fn hierarchical_tree_rejects_unknown_direction() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": []
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "hierarchicalTree": { "direction": "sideways" }
        });
        let err = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap_err();
        assert!(err.contains("unknown hierarchical tree direction"));
    }

    #[test]
    fn redraw_rejects_unknown_mode() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [],
            "edges": []
        });
        let err = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), r#"{"mode":"nope"}"#).unwrap_err();
        assert!(err.contains("unknown redraw mode"));
    }

    #[test]
    fn svg_icon_append_smoke() {
        let mut scene = crate::cavas::Scene::new();
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="#ffffff"/><path d="M0 0 L10 10" stroke="#000000" stroke-width="1"/></svg>"##;
        crate::cavas::svg_icon::append_svg_str(&mut scene, svg).expect("parse svg");
        let fg = crate::cavas::Color::from_rgba8(200, 10, 10, 255);
        let bg = crate::cavas::Color::from_rgba8(10, 200, 10, 255);
        let mut scene2 = crate::cavas::Scene::new();
        crate::cavas::svg_icon::append_svg_str_themed(&mut scene2, svg, fg, bg).expect("parse themed");
    }

    #[test]
    fn board_icon_codec_resolves_catalog_key_via_themed_lookup() {
        let r = super::board_icon_codec::board_resolve_icon_kind("capsule_J");
        match r {
            crate::cavas::icon_codec::BoardResolvedIcon::SvgThemed(s) => {
                assert!(s.contains("<svg"), "catalog metabolism key should resolve via themed lookup");
            }
            other => panic!("unexpected resolution for catalog capsule_J: {other:?}"),
        }
    }

    #[test]
    fn board_icon_codec_resolves_typst_math_to_svg_plain() {
        let r = super::board_icon_codec::board_resolve_icon_kind("typst:$x^2$");
        match r {
            super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
            }
            other => panic!("unexpected resolution: {other:?}"),
        }
    }

    #[test]
    fn board_icon_codec_resolves_emoji_prefix_without_tofu() {
        let r = super::board_icon_codec::board_resolve_icon_kind("emoji:☺");
        match r {
            super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
                assert!(!s.contains('\u{fffd}'), "expected no U+FFFD replacement in emoji SVG, got {}", &s[..s.len().min(400)]);
            }
            other => panic!("unexpected resolution: {other:?}"),
        }
    }

    #[test]
    fn svg_icon_content_bounds_follows_nested_group_translate() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200"><g transform="translate(72 88)"><rect width="12" height="12" fill="rgb(8,8,8)"/></g></svg>"#;
        let (x, y, w, h) = crate::cavas::svg_icon::svg_icon_content_bounds_from_str(svg).expect("parse");
        assert!(x >= 70.0 && x <= 74.0, "expected translated art near x≈72, got {x}");
        assert!(y >= 86.0 && y <= 90.0, "expected translated art near y≈88, got {y}");
        assert!(w > 10.0 && w < 14.0 && h > 10.0 && h < 14.0, "expected ~12×12 bbox, got {w}×{h}");
    }

    #[test]
    fn svg_icon_content_bounds_includes_visible_image_abs_box() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><image href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==" x="30" y="40" width="50" height="50"/></svg>"##;
        let (x, y, w, h) = crate::cavas::svg_icon::svg_icon_content_bounds_from_str(svg).expect("parse");
        assert!((x - 30.0).abs() < 2.0, "expected image bbox near x=30, got {x}");
        assert!((y - 40.0).abs() < 2.0, "expected image bbox near y=40, got {y}");
        assert!((w - 50.0).abs() < 2.0 && (h - 50.0).abs() < 2.0, "expected ~50×50 bbox, got {w}×{h}");
    }
}
// #endregion 🔖Tests
