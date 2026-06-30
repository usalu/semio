//! 🧩 Directed port graph normal leaf: `BoardHost`, puzzle.2d.fixture, WASM session paint.

pub mod board_host {
// #region board_host
//! 🕸️ Generic graph board host on infinite canvas.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]

use infinite_cavas::usvg;
use infinite_cavas::vello::kurbo::{Affine, Circle, CubicBez, Point, Rect, Stroke, Vec2};
use infinite_cavas::vello::peniko::{Blob, Color, Fill, ImageAlphaType, ImageBrush, ImageData, ImageFormat};
use infinite_cavas::vello::Scene;
use serde::Deserialize;
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    board_json_locked_option, board_json_visible_option, builtin_edge_tips, circle_handle_angle_toward, compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, fixture_edge_handle_ids_from_object,
    handle_exterior_cap_fill_path, handle_exterior_cap_stroke_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, merge_ids_into_selection, merge_pick_into_selection, normalize_or_zero,
    normalize_selection_mode, pick_merge_mode_for_modifiers, rectangle_handle_angle_toward, selection_drag_enclosing, selection_drag_shape, ActiveTool, BoardElementStyleKind, CachedIconBody, CompatSpecificity, EdgeData, EdgeDescJson,
    EdgeKindDef, EdgeStrokePattern, EdgeTipDef, EdgeTipGeometry, FixtureV1Json, GraphPortMode, HandleData, HandleDescJson, HandleKindDef, IconPaintCache, Interaction, LinkCompatRule, NodeData, NodeDescJson, NodeKindDef, NodeKindHandleTemplate,
    NodeShape, SceneDescriptorJson, SelectionOptions, VelloThemePalette, WireData, WireKindDef,
};
use infinite_cavas::camera::Camera;
use mathematical_graph_manifest::manifest_by_id;
use infinite_cavas::geom_sel::{
    cubic_bezier_axis_bounds, cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box,
    world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox,
};

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub use infinite_cavas::camera::{CANVAS_CAMERA_ZOOM_MAX as BOARD_CAMERA_ZOOM_MAX, CANVAS_CAMERA_ZOOM_MIN as BOARD_CAMERA_ZOOM_MIN};

use infinite_cavas::lod::{Lod, LodScale};

const GRID_WORLD_LARGE: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;
const GRID_WORLD_MEDIUM: f64 = ui_styling::metrics::board::GRID_WORLD_MEDIUM;
const GRID_WORLD_SMALL: f64 = ui_styling::metrics::board::GRID_WORLD_SMALL;
const GRID_WORLD_MICRO: f64 = ui_styling::metrics::board::GRID_WORLD_MICRO;
const GRID_FACTOR_DEFAULT: f64 = ui_styling::metrics::board::GRID_FACTOR_DEFAULT;
const WORLD_CLIP_TILE_WORLD: f64 = ui_styling::metrics::board::WORLD_CLIP_TILE_WORLD;
const MAX_WORLD_CLIP_TILES: u32 = ui_styling::metrics::board::MAX_WORLD_CLIP_TILES;
const EDGE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::EDGE_HIT_TOLERANCE_PX;
const HANDLE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::HANDLE_HIT_TOLERANCE_PX;
const INDIRECT_HANDLE_MARKER_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_MARKER_SCALE;
/// Radial offset from node rim to indirect-handle center, as a fraction of node half-extent (circle radius or half the shorter rectangle side).
const INDIRECT_HANDLE_RING_GAP_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_RING_GAP_SCALE;
const LINK_DRAG_MIN_DISTANCE_PX: f64 = ui_styling::metrics::board::LINK_DRAG_MIN_DISTANCE_PX;
const LINK_HANDLE_SNAP_EXTRA_PX: f64 = ui_styling::metrics::board::LINK_HANDLE_SNAP_EXTRA_PX;
const LINK_COMMIT_SNAP_TIGHT_PX: f64 = ui_styling::metrics::board::LINK_COMMIT_SNAP_TIGHT_PX;
const DEFAULT_SUGGESTION_OFFSET: f64 = ui_styling::metrics::board::SUGGESTION_OFFSET;
const DEFAULT_BRUSH_NODE_SIZE: f64 = ui_styling::metrics::board::BRUSH_NODE_SIZE;
const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_CLICK_MAX_DISTANCE_PX;
const BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;
const DEFAULT_WIRE_KIND_ID: &str = "wire.link";

const PUZZLE_2D_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-board silhouette; group selection and bounded drag only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Topology and indirect handle rings; no per-node picks.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Dense graph layout with simplified chrome.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Standard editing: nodes, edges, and handle rings.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Node icons and richer strokes.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Maximum fidelity including handle icons.", max_zoom: f64::INFINITY },
];

const PUZZLE_2D_LOD_SCALE: LodScale = LodScale { lods: PUZZLE_2D_LODS };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl BoardDrawLod {
    fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        Some(match id {
            "minimap" => Self::Minimap,
            "overview" => Self::Overview,
            "compact" => Self::Compact,
            "normal" => Self::Normal,
            "detail" => Self::Detail,
            "micro" => Self::Micro,
            _ => return None,
        })
    }

    fn from_scale_index(index: usize) -> Self {
        match index {
            0 => Self::Minimap,
            1 => Self::Overview,
            2 => Self::Compact,
            3 => Self::Normal,
            4 => Self::Detail,
            _ => Self::Micro,
        }
    }
}

pub fn puzzle_2d_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = PUZZLE_2D_LODS
        .iter()
        .map(|lod| {
            serde_json::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

/// @emoji 🎨 Whether drawable style resolves committed selection chrome or neutral cached geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StyleChromePass {
    CachedBase,
    InteractionOverlay,
}

/// @emoji 🎨 Which node/handle primitives to paint in a layered draw pass (fills behind icons/text).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeHandlePaintLayer {
    Full,
    Fill,
    Stroke,
    Icons,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BrushCandidate {
    node_kind_id: String,
    target_handle_index: usize,
}

#[derive(Clone, Debug)]
struct BrushPreviewSnapshot {
    source_handle_id: String,
    node_kind_id: String,
    x: f64,
    y: f64,
    shape: NodeShape,
    radius: f64,
    width: f64,
    height: f64,
    handles: Vec<NodeKindHandleTemplate>,
    target_handle_index: usize,
    icon_kind: Option<String>,
}

#[derive(Clone, Debug)]
struct FillVirtualNode {
    node_kind: String,
    x: f64,
    y: f64,
    shape: NodeShape,
    radius: f64,
    width: f64,
    height: f64,
}

#[derive(Clone, Debug)]
struct FillVirtualHandle {
    node_id: String,
    handle_kind: String,
    template: NodeKindHandleTemplate,
}

#[derive(Clone, Debug, Default)]
struct FillAccum {
    connected_handles: BTreeSet<String>,
    placements: Vec<(String, String, BrushPreviewSnapshot)>,
    virtual_nodes: HashMap<String, FillVirtualNode>,
    virtual_handles: HashMap<String, FillVirtualHandle>,
    virtual_bounds: Vec<WorldBox>,
    next_serial: u32,
}

#[derive(Clone, Debug)]
struct BrushFillSession {
    accum: FillAccum,
    state: u64,
    max_count: usize,
    stalled: bool,
}

#[derive(Clone, Debug)]
struct FixtureDropPreviewSnapshot {
    node_kind_id: String,
    x: f64,
    y: f64,
    shape: NodeShape,
    radius: f64,
    width: f64,
    height: f64,
    icon_kind: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
struct BoardPickTargetJson {
    domain: String,
    id: String,
    generality: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone)]
pub struct BoardHost {
    pub camera: Camera,
    pub nodes: BTreeMap<String, NodeData>,
    pub handles: BTreeMap<String, HandleData>,
    pub edges: BTreeMap<String, EdgeData>,
    pub wires: BTreeMap<String, WireData>,
    /// Catalog keyed by `handle_kind` id (see `set_board_kind_catalogs_from_json`).
    pub handle_kinds: BTreeMap<String, HandleKindDef>,
    pub wire_kinds: BTreeMap<String, WireKindDef>,
    pub node_kinds: BTreeMap<String, NodeKindDef>,
    pub edge_kinds: BTreeMap<String, EdgeKindDef>,
    /// @emoji 🔺 Registry of edge tip shapes keyed by catalog id (built-ins seeded at init).
    pub edge_tips: BTreeMap<String, EdgeTipDef>,
    /// @emoji 🔗 Kind-compatibility rules for link gestures; empty = unrestricted.
    pub link_compat_rules: Vec<LinkCompatRule>,
    pub selection: BTreeSet<String>,
    /// @emoji 👁️ Live rectangle/lasso preview ids (committed selection stays in `selection` until pointer-up).
    pub preselect: BTreeSet<String>,
    /// @emoji 💠 During preselect: anchor selection \\ `preselect` (secondary chrome while dragging).
    pub preselect_removed: BTreeSet<String>,
    /// @emoji 💠 After commit: ids dropped in the last `select` transition only.
    pub selection_exit_highlight: BTreeSet<String>,
    pub selection_options: SelectionOptions,
    pub hovered_id: Option<String>,
    /// @emoji 🖱️ Transitive same-kind hover `(domain, kind_id)` when hovering a kind row or derived from `hovered_id`.
    pub hovered_kind: Option<(String, String)>,
    pub interaction: Interaction,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
    pub world_raster_tiling: String,
    pub events: Vec<serde_json::Value>,
    /// Screen-space preview polygon (CSS pixels) while area-selecting; cleared when idle.
    pub selection_screen_preview: Option<Vec<Point>>,
    /// @emoji ↔️ True when area-select drag is crossing (right-to-left); drives dashed preview stroke.
    pub selection_preview_crossing: bool,
    /// Screen-space polyline preview (CSS px) while dragging a handle link before drop.
    pub link_screen_preview: Option<Vec<Point>>,
    pub vello_theme: VelloThemePalette,
    /// @emoji 📐 Positive multiplier for LOD world grid steps (`10` / `5` / `1` base world units per band).
    pub grid_factor: f64,
    /// @emoji 🧲 When true, node drags snap to the finest visible LOD grid (step scales with `grid_factor`).
    pub grid_snap_enabled: bool,
    pub preserve_original_element_style: bool,
    /// @emoji 📶 When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier when set.
    pub automatic_lod: bool,
    forced_draw_lod: Option<BoardDrawLod>,
    pub icon_paint_cache: IconPaintCache,
    /// @emoji 📡 Dedupes {@code linkCompatibleNodes} emissions while a link wire is active.
    link_compat_nodes_emit_key: Option<String>,
    /// @emoji 📡 Dedupes {@code linkTargetRing} emissions while a link wire is active.
    link_target_ring_emit_key: Option<String>,
    /// @emoji 📡 Dedupes `select` emissions when ids are unchanged but modifier merge mode changes mid‑gesture.
    last_select_emit_sig: Option<(Vec<String>, Option<String>)>,
    /// @emoji 📡 Dedupes `preselect` emissions during area-select drag.
    last_preselect_emit_sig: Option<(Vec<String>, Vec<String>, Option<String>)>,
    /// @emoji 🧿 Bumped when drawable content changes (not camera); keys {@link BoardHost.world_content_cache}.
    content_scene_generation: u64,
    /// @emoji 🎨 World-space Vello content reused across pan/zoom when generation and LOD match.
    world_content_cache: RefCell<Option<(u64, BoardDrawLod, Scene)>>,
    /// @emoji 🔍 True while the wheel zoom gesture is active (skip grid + per-tile rebuild hot paths).
    wheel_zoom_active: bool,
    /// @emoji 📶 LOD tier pinned for the active wheel gesture so pan/zoom does not rebuild {@link BoardHost.world_content_cache} on every band crossing.
    wheel_zoom_render_lod: Option<BoardDrawLod>,
    /// @emoji 🖌️ Active viewport tool (`select` suppresses brush slot logic).
    active_tool: ActiveTool,
    suggestion_offset: f64,
    brush_node_size: f64,
    brush_slot_source_id: Option<String>,
    brush_candidates: Vec<BrushCandidate>,
    brush_candidate_index: usize,
    brush_preview: Option<BrushPreviewSnapshot>,
    fixture_drop_preview: Option<FixtureDropPreviewSnapshot>,
    brush_candidates_emit_key: Option<String>,
    brush_preview_emit_key: Option<String>,
    brush_placement_serial: u64,
    brush_node_kind_weights: HashMap<String, f64>,
    brush_handle_kind_weights: HashMap<String, f64>,
    /// @emoji ⌥ Alt held while brushing — enables suggestion offset and commit-on-leave.
    brush_alt_pressed: bool,
    /// @emoji ✨ Suggestions menu opened a slot outside brush tool — use suggestion offset and highlight source handle.
    brush_slot_suggestions_active: bool,
    /// @emoji 🪣 Resumable greedy fill session for chunked WASM builds.
    brush_fill_session: Option<BrushFillSession>,
    pub port_mode: GraphPortMode,
}

impl Default for BoardHost {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            nodes: BTreeMap::new(),
            handles: BTreeMap::new(),
            edges: BTreeMap::new(),
            wires: BTreeMap::new(),
            handle_kinds: BTreeMap::new(),
            wire_kinds: BTreeMap::new(),
            node_kinds: BTreeMap::new(),
            edge_kinds: BTreeMap::new(),
            edge_tips: builtin_edge_tips(),
            link_compat_rules: Vec::new(),
            selection: BTreeSet::new(),
            preselect: BTreeSet::new(),
            preselect_removed: BTreeSet::new(),
            selection_exit_highlight: BTreeSet::new(),
            selection_options: SelectionOptions { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_edges: true, select_handles: true },
            hovered_id: None,
            hovered_kind: None,
            interaction: Interaction::None,
            width: 1,
            height: 1,
            dpr: 1.0,
            world_raster_tiling: "world-clip".into(),
            events: Vec::new(),
            selection_screen_preview: None,
            selection_preview_crossing: false,
            link_screen_preview: None,
            vello_theme: VelloThemePalette::default(),
            grid_factor: GRID_FACTOR_DEFAULT,
            grid_snap_enabled: false,
            preserve_original_element_style: false,
            automatic_lod: true,
            forced_draw_lod: None,
            icon_paint_cache: IconPaintCache::new(),
            link_compat_nodes_emit_key: None,
            link_target_ring_emit_key: None,
            last_select_emit_sig: None,
            last_preselect_emit_sig: None,
            content_scene_generation: 0,
            world_content_cache: RefCell::new(None),
            wheel_zoom_active: false,
            wheel_zoom_render_lod: None,
            active_tool: ActiveTool::Select,
            suggestion_offset: DEFAULT_SUGGESTION_OFFSET,
            brush_node_size: DEFAULT_BRUSH_NODE_SIZE,
            brush_slot_source_id: None,
            brush_candidates: Vec::new(),
            brush_candidate_index: 0,
            brush_preview: None,
            fixture_drop_preview: None,
            brush_candidates_emit_key: None,
            brush_preview_emit_key: None,
            brush_placement_serial: 0,
            brush_node_kind_weights: HashMap::new(),
            brush_handle_kind_weights: HashMap::new(),
            brush_alt_pressed: false,
            brush_slot_suggestions_active: false,
            brush_fill_session: None,
            port_mode: GraphPortMode::Ported,
        }
    }
}

impl BoardHost {
    /// @emoji 📶 Draw LOD used while building the vector scene (pins during wheel zoom).
    fn draw_lod_for_frame(&self) -> BoardDrawLod {
        if self.wheel_zoom_active {
            if let Some(pinned) = self.wheel_zoom_render_lod {
                return pinned;
            }
        }
        self.current_draw_lod()
    }

    fn board_draw_lod_label(lod: BoardDrawLod) -> &'static str {
        lod.label()
    }

    /// @emoji 🏷️ Camera, draw LOD, and visible node centers from the WASM host for the JS text overlay (must match the last GPU frame).
    pub fn overlay_paint_state_json(&self) -> String {
        let nodes: Vec<serde_json::Value> = self.nodes.values().filter(|n| n.visible).map(|n| serde_json::json!({ "id": n.id, "x": n.x, "y": n.y })).collect();
        serde_json::json!({
            "camera": { "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom },
            "lod": Self::board_draw_lod_label(self.draw_lod_for_frame()),
            "nodes": nodes,
        })
        .to_string()
    }

    fn bump_content_scene_generation(&mut self) {
        self.content_scene_generation = self.content_scene_generation.wrapping_add(1);
        *self.world_content_cache.borrow_mut() = None;
    }

    #[doc(hidden)]
    pub fn test_content_scene_generation(&self) -> u64 {
        self.content_scene_generation
    }

    fn viewport(&self) -> infinite_cavas::camera::Viewport {
        infinite_cavas::camera::Viewport { width: self.width, height: self.height, dpr: self.dpr }
    }

    fn camera_content_affine(&self) -> Affine {
        infinite_cavas::camera::camera_content_affine(&self.camera, &self.viewport())
    }
}

impl BoardHost {
    fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }
    fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }
    fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }
    fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// 🧠 Normal directed graph host: no handles, edges reference node ids.
    pub fn new_normal() -> Self {
        let mut host = Self::default();
        host.port_mode = GraphPortMode::Normal;
        host.selection_options.select_handles = false;
        host
    }

    fn has_ports(&self) -> bool {
        self.port_mode.has_ports()
    }

    fn node_rim_point_toward(&self, node: &NodeData, toward: Point) -> Option<Point> {
        let center = Point::new(node.x, node.y);
        match node.shape {
            NodeShape::Circle => {
                let radius = self.scaled_node_radius(node);
                let angle = circle_handle_angle_toward(center, toward);
                Some(handle_position_on_circle(center, radius, angle))
            }
            NodeShape::Rectangle => {
                let width = self.scaled_node_width(node);
                let height = self.scaled_node_height(node);
                let angle = rectangle_handle_angle_toward(center, width, height, toward);
                Some(handle_position_on_rectangle(center, width, height, angle))
            }
        }
    }

    fn current_draw_lod(&self) -> BoardDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        BoardDrawLod::from_scale_index(PUZZLE_2D_LOD_SCALE.resolve_index(self.camera.zoom))
    }

    fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
        match self.current_draw_lod() {
            BoardDrawLod::Minimap => None,
            BoardDrawLod::Overview | BoardDrawLod::Compact => Some(self.grid_step_large_world()),
            BoardDrawLod::Normal => Some(self.grid_step_medium_world()),
            BoardDrawLod::Detail => Some(self.grid_step_small_world()),
            BoardDrawLod::Micro => Some(self.grid_step_micro_world()),
        }
    }

    fn snap_world_scalar(&self, v: f64) -> f64 {
        if !self.grid_snap_enabled {
            return v;
        }
        let Some(step) = self.lod_visible_grid_snap_step_world() else {
            return v;
        };
        (v / step).round() * step
    }

    fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
        (self.snap_world_scalar(x), self.snap_world_scalar(y))
    }

    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let t = label.trim();
        if t.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = match BoardDrawLod::from_id(t) {
            Some(lod) => Some(lod),
            None => {
                self.forced_draw_lod = None;
                return;
            }
        };
    }

    pub fn set_grid_factor(&mut self, v: f64) -> Result<(), String> {
        if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
            return Err("gridFactor must be finite and in (0, 1e6]".into());
        }
        self.grid_factor = v;
        Ok(())
    }

    /// @emoji 🔗 Applies or clears a host-driven link preview session (cross-surface mirror).
    pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("setLinkSessionJson: {e}"))?;
        let source = v.get("source").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
        if source.is_empty() {
            if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
            }
            return Ok(());
        }
        let end_x = v.get("endX").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let end_y = v.get("endY").and_then(|y| y.as_f64()).unwrap_or(0.0);
        let compatible_node_ids: Vec<String> = v.get("compatiblePartIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        let ring_node_id = v.get("ringPartId").and_then(|n| n.as_str()).map(str::to_string);
        let ring_handle_ids: Vec<String> = v.get("ringAnchorIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        self.interaction = Interaction::ExternalLinkPreview { source_id: source.clone(), end_world: Point::new(end_x, end_y), compatible_node_ids, ring_node_id, ring_handle_ids };
        self.sync_link_gesture_events();
        Ok(())
    }

    /// @emoji 🔗 Clears host-driven link preview without touching local link drags.
    pub fn clear_external_link_preview(&mut self) {
        if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
            self.interaction = Interaction::None;
            self.clear_link_gesture_events();
        }
    }

    fn get_or_build_icon_paint(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<(f64, f64, f64, f64, CachedIconBody)> {
        self.icon_paint_cache.get_or_build(encoded, fg, bg, preserve_original_style)
    }

    pub fn clear_icon_vector_cache(&mut self) {
        self.icon_paint_cache.clear();
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.set_camera_internal(x, y, zoom, true);
    }

    /// @emoji 🔇 Updates viewport camera without enqueueing a `camera` drain row (wheel / imperative sync).
    pub fn set_camera_silent(&mut self, x: f64, y: f64, zoom: f64) {
        self.set_camera_internal(x, y, zoom, false);
    }

    fn set_camera_internal(&mut self, x: f64, y: f64, zoom: f64, emit_event: bool) {
        let zoom = infinite_cavas::camera::clamp_zoom(zoom);
        if (self.camera.x - x).abs() < 1e-9 && (self.camera.y - y).abs() < 1e-9 && (self.camera.zoom - zoom).abs() < 1e-9 {
            return;
        }
        self.camera.x = x;
        self.camera.y = y;
        self.camera.zoom = zoom;
        if emit_event {
            self.push_event("camera", json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
        }
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
        self.selection_options.method = method.into();
        self.selection_options.mode = normalize_selection_mode(mode);
        self.selection_options.select_nodes = select_nodes;
        self.selection_options.select_edges = select_edges;
        self.selection_options.select_handles = select_handles;
    }

    /// @emoji 🔗 JSON `[{ "source","target","bidirectional"?,"important"?,"specificity"? },…]` gates link gestures; empty clears restrictions.
    pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let arr = v.as_array().ok_or_else(|| "expected JSON array of compatibility objects".to_string())?;
        let mut next = Vec::new();
        for row in arr {
            let o = row.as_object().ok_or("compat row must be object")?;
            let source = o.get("source").and_then(|x| x.as_str()).ok_or_else(|| "compat row missing string source".to_string())?.trim().to_string();
            let target = o.get("target").and_then(|x| x.as_str()).ok_or_else(|| "compat row missing string target".to_string())?.trim().to_string();
            let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
            let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
            let spec_s = o.get("specificity").and_then(|x| x.as_str()).unwrap_or("handle");
            let specificity = Self::parse_compat_specificity(spec_s)?;
            next.push(LinkCompatRule { source, target, bidirectional, important, specificity });
        }
        self.link_compat_rules = next;
        Ok(())
    }

    fn parse_compat_specificity(raw: &str) -> Result<CompatSpecificity, String> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "general" => Ok(CompatSpecificity::General),
            "node" => Ok(CompatSpecificity::Node),
            "edge" => Ok(CompatSpecificity::Edge),
            "handle" | "vortex" => Ok(CompatSpecificity::Handle),
            "wire" => Ok(CompatSpecificity::Wire),
            _ => Err(format!("compat specificity must be general|node|edge|handle|wire|vortex, got {raw:?}")),
        }
    }

    fn reject_kind_catalog_row_legacy_label(row: &serde_json::Map<String, serde_json::Value>, slice: &str) -> Result<(), String> {
        if row.contains_key("label") {
            return Err(format!("{slice} kind row must use name, not legacy label"));
        }
        Ok(())
    }

    /// @emoji 🧩 JSON object `{ handleKinds?, wireKinds?, nodeKinds?, edgeKinds? }` replacing prior catalogs (omit arrays to clear that slice).
    pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let o = v.as_object().ok_or("kind catalogs root must be object")?;
        if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let ho = row.as_object().ok_or("handle kind row must be object")?;
                Self::reject_kind_catalog_row_legacy_label(ho, "handle")?;
                let id = ho.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("handle kind id missing")?;
                let name = ho.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color_s = ho.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("handle kind color missing")?;
                let color = Self::parse_css_color(color_s).ok_or_else(|| format!("invalid handle kind color {color_s:?}"))?;
                let default_wire_kind = ho.get("defaultWireKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                next.insert(id.to_string(), HandleKindDef { name, color, default_wire_kind, scale });
            }
            self.handle_kinds = next;
        }
        if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let wo = row.as_object().ok_or("wire kind row must be object")?;
                Self::reject_kind_catalog_row_legacy_label(wo, "wire")?;
                let id = wo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("wire kind id missing")?;
                let name = wo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let default_edge_kind = wo.get("defaultEdgeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
            }
            self.wire_kinds = next;
        }
        if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let no = row.as_object().ok_or("node kind row must be object")?;
                Self::reject_kind_catalog_row_legacy_label(no, "node")?;
                let id = no.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("node kind id missing")?;
                let name = no.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                let shape = match no.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                    Some("rectangle") => NodeShape::Rectangle,
                    _ => NodeShape::Circle,
                };
                let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
                if let Some(arr) = no.get("handles").and_then(|x| x.as_array()) {
                    for row in arr {
                        let ho = row.as_object().ok_or("node kind handle row must be object")?;
                        let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("node kind handle handleKind missing")?;
                        let angle = ho.get("angle").and_then(|x| x.as_f64()).filter(|x| x.is_finite()).ok_or("node kind handle angle missing")?;
                        let radius = ho.get("radius").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0);
                        handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                    }
                }
                let icon = no.get("icon").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let color_fill = no.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                next.insert(id.to_string(), NodeKindDef { name, scale, shape, handles, icon, color_fill });
            }
            self.node_kinds = next;
        }
        if let Some(arr) = o.get("edgeTips").and_then(|x| x.as_array()) {
            let mut tips = builtin_edge_tips();
            for row in arr {
                let eo = row.as_object().ok_or("edge tip row must be object")?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("edge tip id missing")?;
                let def = EdgeTipDef::from_catalog_row(eo).ok_or_else(|| format!("edge tip row {:?} invalid", id))?;
                tips.insert(id.to_string(), def);
            }
            self.edge_tips = tips;
        }
        if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let eo = row.as_object().ok_or("edge kind row must be object")?;
                Self::reject_kind_catalog_row_legacy_label(eo, "edge")?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("edge kind id missing")?;
                let name = eo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color = eo.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                let stroke_width = eo
                    .get("stroke")
                    .and_then(|x| x.as_f64())
                    .filter(|v| v.is_finite() && *v > 0.0)
                    .or_else(|| eo.get("stroke").and_then(|x| x.as_str()).and_then(|s| s.trim().parse::<f64>().ok()).filter(|v| v.is_finite() && *v > 0.0))
                    .unwrap_or(2.0);
                let pattern = match eo.get("pattern").and_then(|x| x.as_str()).map(str::trim) {
                    Some("dashed") => EdgeStrokePattern::Dashed,
                    Some("dotted") => EdgeStrokePattern::Dotted,
                    _ => EdgeStrokePattern::Solid,
                };
                let source_tip = Self::parse_catalog_tip_slot(eo.get("sourceTip").or_else(|| eo.get("source_tip")).and_then(|x| x.as_str()));
                let target_tip = Self::parse_catalog_tip_slot(eo.get("targetTip").or_else(|| eo.get("target_tip")).and_then(|x| x.as_str()).or_else(|| eo.get("marker").and_then(|x| x.as_str())));
                let directed = eo.get("directed").and_then(|x| x.as_bool()).unwrap_or(true);
                next.insert(id.to_string(), EdgeKindDef { name, color, stroke_width, pattern, source_tip, target_tip, directed });
            }
            self.edge_kinds = next;
        }
        Ok(())
    }

    /// @emoji 🛡️ Ensures runtime catalogs declare every kind from a compile-time manifest.
    pub fn validate_against_manifest_id(&self, manifest_id: &str) -> Result<(), String> {
        let gm = manifest_by_id(manifest_id).ok_or_else(|| format!("unknown manifest id {manifest_id}"))?;
        for row in &gm.port_kinds {
            let visual = row.presentation.as_ref().is_some_and(|p| p.get("color").is_some());
            if visual && !self.handle_kinds.contains_key(&row.id) {
                return Err(format!("catalog missing handle kind {:?}", row.id));
            }
        }
        for row in &gm.wire_kinds {
            if !self.wire_kinds.contains_key(&row.id) {
                return Err(format!("catalog missing wire kind {:?}", row.id));
            }
        }
        for row in &gm.edge_kinds {
            if row.presentation.is_some() && !self.edge_kinds.contains_key(&row.id) {
                return Err(format!("catalog missing edge kind {:?}", row.id));
            }
        }
        for row in &gm.node_kinds {
            if row.id == "Piece" {
                continue;
            }
            if !self.node_kinds.contains_key(&row.id) {
                return Err(format!("catalog missing node kind {:?}", row.id));
            }
        }
        Ok(())
    }

    fn parse_catalog_tip_slot(value: Option<&str>) -> Option<String> {
        let s = value?.trim();
        if s.is_empty() || s.eq_ignore_ascii_case("none") {
            Some(String::new())
        } else {
            Some(s.to_string())
        }
    }

    fn lookup_edge_tip<'a>(&'a self, id: &str) -> Option<&'a EdgeTipDef> {
        if id.is_empty() {
            return None;
        }
        self.edge_tips.get(id)
    }

    fn resolve_tip_slot<'a>(&'a self, slot: Option<&str>) -> Option<&'a EdgeTipDef> {
        match slot {
            Some("") => None,
            Some(id) => self.lookup_edge_tip(id),
            None => None,
        }
    }

    fn parse_css_hex_color(s: &str) -> Option<Color> {
        let s = s.trim();
        let hex = s.strip_prefix('#')?;
        match hex.len() {
            3 => {
                let mut full = String::new();
                for ch in hex.chars() {
                    full.push(ch);
                    full.push(ch);
                }
                let v = u32::from_str_radix(&full, 16).ok()?;
                let r = ((v >> 16) & 0xff) as u8;
                let g = ((v >> 8) & 0xff) as u8;
                let b = (v & 0xff) as u8;
                Some(Color::from_rgba8(r, g, b, 255))
            }
            6 => {
                let v = u32::from_str_radix(hex, 16).ok()?;
                let r = ((v >> 16) & 0xff) as u8;
                let g = ((v >> 8) & 0xff) as u8;
                let b = (v & 0xff) as u8;
                Some(Color::from_rgba8(r, g, b, 255))
            }
            8 => {
                let v = u32::from_str_radix(hex, 16).ok()?;
                let r = ((v >> 24) & 0xff) as u8;
                let g = ((v >> 16) & 0xff) as u8;
                let b = ((v >> 8) & 0xff) as u8;
                let a = (v & 0xff) as u8;
                Some(Color::from_rgba8(r, g, b, a))
            }
            _ => None,
        }
    }

    /// @emoji 🎨 Accepts `#rgb`/`#rrggbb`/`#rrggbbaa` or CSS `hsl()` / `hsla()` (comma or space syntax, optional `/` alpha).
    fn parse_css_color(s: &str) -> Option<Color> {
        if let Some(c) = Self::parse_css_hex_color(s) {
            return Some(c);
        }
        Self::parse_css_hsl_color(s)
    }

    fn parse_css_hsl_color(s: &str) -> Option<Color> {
        let low = s.trim().to_ascii_lowercase();
        let (legacy_alpha_form, inner) = if let Some(inner) = low.strip_prefix("hsla(").and_then(|x| x.strip_suffix(')')) {
            (true, inner)
        } else if let Some(inner) = low.strip_prefix("hsl(").and_then(|x| x.strip_suffix(')')) {
            (false, inner)
        } else {
            return None;
        };
        let inner = inner.trim();
        let (main, alpha_slash) = inner.split_once('/').map(|(a, b)| (a.trim(), Some(b.trim()))).unwrap_or((inner, None));
        let normalized = main.replace(',', " ");
        let parts: Vec<&str> = normalized.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }
        let h = Self::parse_css_hsl_hue(parts[0])?;
        let sat = Self::parse_css_hsl_sl(parts[1])?;
        let light = Self::parse_css_hsl_sl(parts[2])?;
        let alpha = if let Some(a) = alpha_slash {
            Self::parse_css_alpha_channel(a)?
        } else if legacy_alpha_form && parts.len() >= 4 {
            Self::parse_css_alpha_channel(parts[3])?
        } else {
            1.0
        };
        let (r, g, b) = Self::hsl_to_rgb_bytes(h, sat, light);
        let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
        Some(Color::from_rgba8(r, g, b, a))
    }

    fn parse_css_hsl_hue(tok: &str) -> Option<f64> {
        let t = tok.trim();
        let n = t.strip_suffix("deg").map(str::trim).unwrap_or(t);
        let v: f64 = n.parse().ok()?;
        v.is_finite().then_some(v)
    }

    fn parse_css_hsl_sl(tok: &str) -> Option<f64> {
        let t = tok.trim();
        if let Some(p) = t.strip_suffix('%') {
            let v: f64 = p.trim().parse().ok()?;
            Some((v / 100.0).clamp(0.0, 1.0))
        } else {
            let v: f64 = t.parse().ok()?;
            Some(v.clamp(0.0, 1.0))
        }
    }

    fn parse_css_alpha_channel(tok: &str) -> Option<f64> {
        Self::parse_css_hsl_sl(tok)
    }

    fn board_hsl_hue_to_rgb_component(p: f64, q: f64, mut t: f64) -> f64 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    }

    fn hsl_to_rgb_bytes(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
        let h_norm = ((h % 360.0 + 360.0) % 360.0) / 360.0;
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);
        if s <= f64::EPSILON {
            let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
            return (v, v, v);
        }
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        let r = Self::board_hsl_hue_to_rgb_component(p, q, h_norm + 1.0 / 3.0);
        let g = Self::board_hsl_hue_to_rgb_component(p, q, h_norm);
        let b = Self::board_hsl_hue_to_rgb_component(p, q, h_norm - 1.0 / 3.0);
        ((r * 255.0).round().clamp(0.0, 255.0) as u8, (g * 255.0).round().clamp(0.0, 255.0) as u8, (b * 255.0).round().clamp(0.0, 255.0) as u8)
    }

    fn explicit_style_kind(style: Option<&str>) -> Option<BoardElementStyleKind> {
        match style.map(str::trim).filter(|s| !s.is_empty()) {
            Some("original") => Some(BoardElementStyleKind::Original),
            Some("neutral") => Some(BoardElementStyleKind::Neutral),
            Some("hovered") => Some(BoardElementStyleKind::Hovered),
            Some("selected") => Some(BoardElementStyleKind::Selected),
            Some("highlighted") => Some(BoardElementStyleKind::Highlighted),
            Some("disabled") => Some(BoardElementStyleKind::Disabled),
            _ => None,
        }
    }

    fn hovered_style_kind(&self, id: &str, domain: &str, element_kind: &str) -> Option<BoardElementStyleKind> {
        if self.is_preselect_active() {
            return None;
        }
        if self.selection.contains(id) {
            return None;
        }
        if self.hovered_id.as_deref() == Some(id) {
            return Some(BoardElementStyleKind::Hovered);
        }
        if self.hovered_id.is_none() {
            if let Some((hover_domain, hover_kind)) = self.hovered_kind.as_ref() {
                if hover_domain == domain && hover_kind == element_kind {
                    return Some(BoardElementStyleKind::Hovered);
                }
            }
        }
        None
    }

    fn resolve_element_kind_hover(&self, id: &str) -> Option<(String, String)> {
        if let Some(node) = self.nodes.get(id) {
            return Some(("node".to_string(), node.node_kind.clone()));
        }
        if let Some(handle) = self.handles.get(id) {
            return Some(("handle".to_string(), handle.handle_kind.clone()));
        }
        if let Some(edge) = self.edges.get(id) {
            return Some(("edge".to_string(), edge.edge_kind.clone()));
        }
        if let Some(wire) = self.wires.get(id) {
            return Some(("wire".to_string(), wire.wire_kind.clone()));
        }
        None
    }

    fn ids_matching_kind_hover(&self) -> Vec<String> {
        let Some((domain, kind_id)) = self.hovered_kind.as_ref() else {
            return Vec::new();
        };
        let mut ids = Vec::new();
        match domain.as_str() {
            "node" => {
                for node in self.nodes.values() {
                    if &node.node_kind == kind_id && !self.selection.contains(&node.id) {
                        ids.push(node.id.clone());
                    }
                }
            }
            "handle" => {
                for handle in self.handles.values() {
                    if &handle.handle_kind == kind_id && !self.selection.contains(&handle.id) {
                        ids.push(handle.id.clone());
                    }
                }
            }
            "edge" => {
                for edge in self.edges.values() {
                    if &edge.edge_kind == kind_id && !self.selection.contains(&edge.id) {
                        ids.push(edge.id.clone());
                    }
                }
            }
            "wire" => {
                for wire in self.wires.values() {
                    if &wire.wire_kind == kind_id && !self.selection.contains(&wire.id) {
                        ids.push(wire.id.clone());
                    }
                }
            }
            _ => {}
        }
        ids
    }

    fn is_preselect_active(&self) -> bool {
        self.is_preselecting() || !self.preselect.is_empty()
    }

    /// @emoji 🎨 During area-select: preselect → Selected; anchor∖preselect → Highlighted; idle selection → Selected.
    fn resolve_interaction_style_kind(&self, id: &str) -> BoardElementStyleKind {
        if self.is_preselect_active() {
            if self.preselect.contains(id) {
                return BoardElementStyleKind::Selected;
            }
            if self.selection.contains(id) {
                return BoardElementStyleKind::Highlighted;
            }
            return BoardElementStyleKind::Neutral;
        }
        if self.selection.contains(id) {
            return BoardElementStyleKind::Selected;
        }
        BoardElementStyleKind::Neutral
    }

    fn locked_style_dim(kind: BoardElementStyleKind, locked: bool) -> BoardElementStyleKind {
        if locked && !matches!(kind, BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Hovered) {
            BoardElementStyleKind::Disabled
        } else {
            kind
        }
    }

    fn resolve_node_style_kind(&self, n: &NodeData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(n.style.as_deref()) {
            return Self::locked_style_dim(kind, n.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => {
                if self.preserve_original_element_style {
                    BoardElementStyleKind::Original
                } else {
                    BoardElementStyleKind::Neutral
                }
            }
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(n.id.as_str(), "node", n.node_kind.as_str()) {
                    return Self::locked_style_dim(kind, n.locked);
                }
                self.resolve_interaction_style_kind(n.id.as_str())
            }
        };
        Self::locked_style_dim(kind, n.locked)
    }

    fn resolve_handle_style_kind(&self, h: &HandleData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(h.style.as_deref()) {
            return Self::locked_style_dim(kind, h.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => {
                if self.preserve_original_element_style {
                    BoardElementStyleKind::Original
                } else {
                    BoardElementStyleKind::Neutral
                }
            }
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(h.id.as_str(), "handle", h.handle_kind.as_str()) {
                    return Self::locked_style_dim(kind, h.locked);
                }
                self.resolve_interaction_style_kind(h.id.as_str())
            }
        };
        Self::locked_style_dim(kind, h.locked)
    }

    fn resolve_edge_style_kind(&self, e: &EdgeData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(e.style.as_deref()) {
            return Self::locked_style_dim(kind, e.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(e.id.as_str(), "edge", e.edge_kind.as_str()) {
                    return Self::locked_style_dim(kind, e.locked);
                }
                self.resolve_interaction_style_kind(e.id.as_str())
            }
        };
        Self::locked_style_dim(kind, e.locked)
    }

    fn resolve_wire_style_kind(&self, w: &WireData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(w.style.as_deref()) {
            return Self::locked_style_dim(kind, w.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(w.id.as_str(), "wire", w.wire_kind.as_str()) {
                    return Self::locked_style_dim(kind, w.locked);
                }
                self.resolve_interaction_style_kind(w.id.as_str())
            }
        };
        Self::locked_style_dim(kind, w.locked)
    }

    /// @emoji 💠 Entity ids whose selection/preselect/hover chrome tints fills and strokes without rebuilding {@link BoardHost.world_content_cache}.
    fn interaction_overlay_entity_ids(&self) -> BTreeSet<String> {
        let mut ids = BTreeSet::new();
        if self.is_preselect_active() {
            ids.extend(self.preselect.iter().cloned());
            ids.extend(self.selection.iter().cloned());
            ids.extend(self.preselect_removed.iter().cloned());
        } else {
            ids.extend(self.selection.iter().cloned());
            ids.extend(self.selection_exit_highlight.iter().cloned());
        }
        if let Some(ref hover_id) = self.hovered_id {
            if !self.is_preselect_active() && !self.selection.contains(hover_id) {
                ids.insert(hover_id.clone());
            }
        }
        if self.hovered_id.is_none() && !self.is_preselect_active() {
            for id in self.ids_matching_kind_hover() {
                ids.insert(id);
            }
        }
        ids
    }

    fn chrome_pass_for_entity(&self, entity_id: &str, overlay_ids: &BTreeSet<String>) -> StyleChromePass {
        if overlay_ids.contains(entity_id) {
            StyleChromePass::InteractionOverlay
        } else {
            StyleChromePass::CachedBase
        }
    }

    fn node_fill_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.node_fill_hovered,
            BoardElementStyleKind::Selected => theme.node_fill_selected,
            BoardElementStyleKind::Highlighted => theme.node_fill_selection_exit,
            BoardElementStyleKind::Disabled => theme.node_fill_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_fill,
        }
    }

    fn node_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.node_stroke_hovered,
            BoardElementStyleKind::Selected => theme.node_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.node_stroke_selection_exit,
            BoardElementStyleKind::Disabled => theme.node_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_stroke,
        }
    }

    fn resolve_handle_fill_color(&self, h: &HandleData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.handle_fill_hovered,
            BoardElementStyleKind::Selected => theme.handle_fill_selected,
            BoardElementStyleKind::Highlighted => theme.handle_fill_selection_exit,
            BoardElementStyleKind::Disabled => theme.handle_fill_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                if let Some(c) = h.color_fill {
                    return c;
                }
                if let Some(def) = self.handle_kinds.get(&h.handle_kind) {
                    return def.color;
                }
                theme.handle_fill
            }
        }
    }

    fn resolve_handle_stroke_color(&self, _h: &HandleData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.handle_stroke_hovered,
            BoardElementStyleKind::Selected => theme.handle_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.handle_stroke_selection_exit,
            BoardElementStyleKind::Disabled => theme.handle_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.handle_stroke,
        }
    }

    fn edge_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.edge_stroke_hovered,
            BoardElementStyleKind::Selected => theme.edge_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.edge_stroke_selection_exit,
            BoardElementStyleKind::Disabled => theme.edge_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.edge_stroke,
        }
    }

    fn lerp_color(a: Color, b: Color, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        let ac = a.to_rgba8();
        let bc = b.to_rgba8();
        Color::from_rgba8(
            (f64::from(ac.r) * (1.0 - t) + f64::from(bc.r) * t).round() as u8,
            (f64::from(ac.g) * (1.0 - t) + f64::from(bc.g) * t).round() as u8,
            (f64::from(ac.b) * (1.0 - t) + f64::from(bc.b) * t).round() as u8,
            (f64::from(ac.a) * (1.0 - t) + f64::from(bc.a) * t).round() as u8,
        )
    }

    fn resolve_node_fill_color(&self, n: &NodeData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
        let theme_fill = Self::node_fill_for_style(theme, kind);
        match kind {
            BoardElementStyleKind::Hovered | BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Disabled => theme_fill,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                let kind_id = n.node_kind.trim();
                if kind_id.is_empty() {
                    return theme_fill;
                }
                self.node_kinds.get(kind_id).and_then(|def| def.color_fill).unwrap_or(theme_fill)
            }
        }
    }

    fn edge_stroke_for_kind_pattern(pattern: EdgeStrokePattern, width: f64) -> Stroke {
        use infinite_cavas::vello::kurbo::Cap;
        let mut stroke = Stroke::new(width);
        match pattern {
            EdgeStrokePattern::Solid => {}
            EdgeStrokePattern::Dashed => {
                stroke.dash_pattern = vec![width * 3.0, width * 2.0].into();
            }
            EdgeStrokePattern::Dotted => {
                stroke.dash_pattern = vec![width * 0.35, width * 1.65].into();
                stroke.start_cap = Cap::Round;
                stroke.end_cap = Cap::Round;
            }
        }
        stroke
    }

    fn resolve_edge_stroke_paint(&self, e: &EdgeData, chrome_pass: StyleChromePass, lod: BoardDrawLod, lod_scale_width: f64) -> (Color, Stroke, f64) {
        let style_kind = self.resolve_edge_style_kind(e, chrome_pass);
        let chrome = Self::edge_stroke_for_style(&self.vello_theme, style_kind);
        let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
        let base_color = kind_def.and_then(|d| d.color).unwrap_or(self.vello_theme.edge_stroke);
        let stroke_color = match style_kind {
            BoardElementStyleKind::Neutral | BoardElementStyleKind::Original => base_color,
            _ if lod == BoardDrawLod::Minimap => chrome,
            _ => Self::lerp_color(base_color, chrome, 0.55),
        };
        let catalog_w = kind_def.map(|d| d.stroke_width).unwrap_or(2.0);
        let width_mult = match style_kind {
            BoardElementStyleKind::Selected => ui_styling::strokes::EDGE_SELECTED_MULT,
            BoardElementStyleKind::Hovered => ui_styling::strokes::EDGE_HOVERED_MULT,
            _ => 1.0,
        };
        let width = lod_scale_width * (catalog_w / 2.0) * width_mult;
        let pattern = kind_def.map(|d| d.pattern).unwrap_or(EdgeStrokePattern::Solid);
        (stroke_color, Self::edge_stroke_for_kind_pattern(pattern, width), width)
    }

    fn resolve_edge_tips<'a>(&'a self, e: &EdgeData) -> (Option<&'a EdgeTipDef>, Option<&'a EdgeTipDef>) {
        let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
        let source_slot = e.source_tip.as_deref().or_else(|| kind_def.and_then(|d| d.source_tip.as_deref()));
        let target_slot = e.target_tip.as_deref().or_else(|| kind_def.and_then(|d| d.target_tip.as_deref()));
        let mut source = self.resolve_tip_slot(source_slot);
        let mut target = self.resolve_tip_slot(target_slot);
        if target.is_none() && target_slot.is_none() {
            let directed = kind_def.map(|d| d.directed).unwrap_or(true);
            if directed {
                target = self.lookup_edge_tip("arrow");
            }
        }
        (source, target)
    }

    fn append_edge_tip(scene: &mut Scene, tip: Point, dir: Vec2, color: Color, stroke_width: f64, tip_def: &EdgeTipDef) {
        use infinite_cavas::vello::kurbo::BezPath;
        let len = dir.hypot();
        if len < 1e-9 {
            return;
        }
        let d = dir / len;
        let n = Vec2::new(-d.y, d.x);
        let sw = stroke_width.max(1.0) * tip_def.scale.max(0.25);
        match tip_def.geometry {
            EdgeTipGeometry::Arrow => {
                let length = sw * 4.2;
                let half_w = sw * 1.15;
                let base = tip - d * length;
                let mut path = BezPath::new();
                path.move_to(tip);
                path.line_to(base + n * half_w);
                path.line_to(base - n * half_w);
                path.close_path();
                if tip_def.filled {
                    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &path);
                } else {
                    scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
                }
            }
            EdgeTipGeometry::FineArrow => {
                let length = sw * 3.2;
                let half_w = sw * 0.75;
                let base = tip - d * length;
                let mut path = BezPath::new();
                path.move_to(tip);
                path.line_to(base + n * half_w);
                path.move_to(tip);
                path.line_to(base - n * half_w);
                let outline = Stroke::new((sw * 0.9).max(1.0));
                scene.stroke(&outline, Affine::IDENTITY, color, None, &path);
            }
            EdgeTipGeometry::Diamond => {
                let length = sw * 3.6;
                let half_w = sw * 1.05;
                let back = tip - d * length;
                let mid = tip - d * (length * 0.5);
                let mut path = BezPath::new();
                path.move_to(tip);
                path.line_to(mid + n * half_w);
                path.line_to(back);
                path.line_to(mid - n * half_w);
                path.close_path();
                if tip_def.filled {
                    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &path);
                } else {
                    scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
                }
            }
            EdgeTipGeometry::Circle => {
                let r = sw * 1.4;
                let center = tip - d * r;
                let circle = Circle::new(center, r);
                if tip_def.filled {
                    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &circle);
                } else {
                    scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &circle);
                }
            }
            EdgeTipGeometry::Bar => {
                let half = sw * ui_styling::strokes::EDGE_TIP_MIN;
                let center = tip - d * (sw * 0.5);
                let mut path = BezPath::new();
                path.move_to(center + n * half);
                path.line_to(center - n * half);
                scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
            }
        }
    }

    fn append_edge_tips_on_curve(scene: &mut Scene, curve: &CubicBez, color: Color, stroke_w: f64, source: Option<&EdgeTipDef>, target: Option<&EdgeTipDef>) {
        let inset = stroke_w * 0.35;
        if let Some(tip_def) = target {
            let mut tangent = curve.p3 - curve.p2;
            if tangent.hypot() < 1e-9 {
                tangent = curve.p3 - curve.p1;
            }
            if tangent.hypot() >= 1e-9 {
                let dir = tangent / tangent.hypot();
                let tip = curve.p3 - dir * inset;
                Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
            }
        }
        if let Some(tip_def) = source {
            let mut tangent = curve.p0 - curve.p1;
            if tangent.hypot() < 1e-9 {
                tangent = curve.p0 - curve.p2;
            }
            if tangent.hypot() >= 1e-9 {
                let dir = tangent / tangent.hypot();
                let tip = curve.p0 - dir * inset;
                Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
            }
        }
    }

    fn wire_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.wire_stroke_hovered,
            BoardElementStyleKind::Selected => theme.wire_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.wire_stroke_highlighted,
            BoardElementStyleKind::Disabled => theme.wire_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.wire_stroke,
        }
    }

    fn handles_link_compatible_for_drag(&self, source: &HandleData, target: &HandleData) -> bool {
        if !Self::handle_port_shapes_compatible(source.handle_kind.as_str(), target.handle_kind.as_str()) {
            return false;
        }
        if self.link_compat_rules.is_empty() {
            return true;
        }
        let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| self.link_gesture_rule_applies(rule, source, target)).collect();
        if matched.is_empty() {
            return false;
        }
        if matched.iter().any(|r| r.important) {
            matched.retain(|r| r.important);
        } else {
            let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
            matched.retain(|r| (r.specificity as i32) == max_rank);
        }
        !matched.is_empty()
    }

    fn compat_pair_matches(rule: &LinkCompatRule, a: &str, b: &str) -> bool {
        if rule.source == a && rule.target == b {
            return true;
        }
        if rule.bidirectional && rule.source == b && rule.target == a {
            return true;
        }
        false
    }

    fn handle_port_shape(handle_kind: &str) -> Option<&'static str> {
        if handle_kind.contains(" circular ") {
            Some("circular")
        } else if handle_kind.contains(" rectangular ") {
            Some("rectangular")
        } else {
            None
        }
    }

    fn handle_port_shapes_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
        match (Self::handle_port_shape(source_handle_kind), Self::handle_port_shape(target_handle_kind)) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }
    }

    fn single_letter_port_family(handle_kind: &str) -> Option<char> {
        let head = handle_kind.split('-').next()?;
        if head.len() == 1 {
            head.chars().next().filter(|c| c.is_ascii_lowercase())
        } else {
            None
        }
    }

    fn single_letter_port_families_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
        match (Self::single_letter_port_family(source_handle_kind), Self::single_letter_port_family(target_handle_kind)) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }
    }

    fn resolve_default_wire_kind_for_handle(&self, h: &HandleData) -> String {
        self.handle_kinds.get(&h.handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    fn resolve_default_edge_kind_for_wire_kind(&self, wire_kind: &str) -> String {
        self.wire_kinds.get(wire_kind).and_then(|d| d.default_edge_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
    }

    fn link_gesture_rule_applies(&self, rule: &LinkCompatRule, source: &HandleData, target: &HandleData) -> bool {
        let w_src = self.resolve_default_wire_kind_for_handle(source);
        let w_tgt = self.resolve_default_wire_kind_for_handle(target);
        let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
        let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
        let sn = self.nodes.get(&source.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
        let tn = self.nodes.get(&target.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
        let sh = source.handle_kind.as_str();
        let th = target.handle_kind.as_str();
        match rule.specificity {
            CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
            CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src.as_str(), e_tgt.as_str()),
            CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src.as_str(), th),
        }
    }

    fn default_edge_kind_for_created_link(&self, source: &HandleData, _target: &HandleData) -> String {
        let wk = self.resolve_default_wire_kind_for_handle(source);
        self.resolve_default_edge_kind_for_wire_kind(&wk)
    }

    fn resolve_default_wire_kind_for_handle_kind(&self, handle_kind: &str) -> String {
        self.handle_kinds.get(handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    fn link_gesture_rule_applies_kind_strings(&self, rule: &LinkCompatRule, sn: &str, sh: &str, w_src: &str, e_src: &str, tn: &str, th: &str, _w_tgt: &str, e_tgt: &str) -> bool {
        match rule.specificity {
            CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
            CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src, e_tgt),
            CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src, th),
        }
    }

    fn link_kinds_compatible_for_brush(&self, sn: &str, sh: &str, tn: &str, th: &str) -> bool {
        if !Self::handle_port_shapes_compatible(sh, th) {
            return false;
        }
        if !Self::single_letter_port_families_compatible(sh, th) {
            return false;
        }
        if self.link_compat_rules.is_empty() {
            return true;
        }
        let w_src = self.resolve_default_wire_kind_for_handle_kind(sh);
        let w_tgt = self.resolve_default_wire_kind_for_handle_kind(th);
        let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
        let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
        let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| self.link_gesture_rule_applies_kind_strings(rule, sn, sh, &w_src, &e_src, tn, th, &w_tgt, &e_tgt)).collect();
        if matched.is_empty() {
            return false;
        }
        if matched.iter().any(|r| r.important) {
            matched.retain(|r| r.important);
        } else {
            let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
            matched.retain(|r| (r.specificity as i32) == max_rank);
        }
        !matched.is_empty()
    }

    fn brush_slot_hit_radius_world(&self) -> f64 {
        (self.brush_node_size * 0.5).max(1.0)
    }

    /// @emoji 🖌️ Brush slot anchor follows indirect-handle layout at overview/normal LOD so hit targets match painted rings.
    fn brush_handle_anchor_world(&self, h: &HandleData) -> Option<Point> {
        if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            self.indirect_handle_world_pos(h).or_else(|| self.handle_world_pos(h))
        } else {
            self.handle_world_pos(h)
        }
    }

    fn brush_effective_suggestion_offset(&self) -> f64 {
        if self.brush_alt_pressed || self.brush_slot_suggestions_active {
            self.suggestion_offset
        } else {
            0.0
        }
    }

    fn handle_slot_center_world(&self, node_id: &str, hw: Point, offset: f64) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        let nc = Point::new(n.x, n.y);
        let normal = normalize_or_zero(hw - nc);
        Some(hw + normal * offset)
    }

    fn brush_slot_center_world(&self, h: &HandleData) -> Option<Point> {
        let hw = self.brush_handle_anchor_world(h)?;
        self.handle_slot_center_world(h.node_id.as_str(), hw, self.brush_effective_suggestion_offset())
    }

    /// @emoji 🖌️ World distance from pointer to brush slot when the pointer is on the slot, anchor, or sole-free node body.
    fn brush_slot_pointer_hit_distance(&self, world: Point, handle_id: &str, h: &HandleData) -> Option<f64> {
        let slot_center = self.brush_slot_center_world(h)?;
        let zoom = self.camera.zoom.max(1e-9);
        let slot_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.brush_slot_hit_radius_world();
        let d_slot = distance_between(world, slot_center);
        if d_slot <= slot_hit_r {
            return Some(d_slot);
        }
        let anchor = self.brush_handle_anchor_world(h)?;
        let anchor_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h).max(self.effective_handle_radius(h));
        if distance_between(world, anchor) <= anchor_hit_r {
            return Some(d_slot);
        }
        if self.sole_eligible_indirect_handle_on_node(&h.node_id).as_deref() == Some(handle_id) {
            let n = self.nodes.get(&h.node_id)?;
            if self.point_in_node_world(n, world) {
                return Some(d_slot);
            }
        }
        None
    }

    fn brush_nearest_slot_source(&self, world: Point) -> Option<String> {
        let mut best: Option<(f64, String)> = None;
        for (hid, h) in &self.handles {
            if !self.handle_effectively_visible(hid.as_str()) || self.handle_has_incident_edge(hid.as_str()) {
                continue;
            }
            let Some(d) = self.brush_slot_pointer_hit_distance(world, hid.as_str(), h) else {
                continue;
            };
            if best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
                best = Some((d, hid.clone()));
            }
        }
        best.map(|(_, id)| id)
    }

    fn brush_candidate_seed(source_handle_id: &str) -> u64 {
        source_handle_id.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(u64::from(b)))
    }

    fn brush_kind_weight(weights: &HashMap<String, f64>, id: &str, uniform_fallback: f64) -> f64 {
        weights.get(id).copied().filter(|w| w.is_finite() && *w > 0.0).unwrap_or(uniform_fallback)
    }

    fn brush_next_seed(state: u64) -> u64 {
        state.wrapping_mul(6364136223846793005).wrapping_add(1)
    }

    fn brush_weighted_sample_index(weights: &[f64], seed: u64) -> usize {
        let wsum: f64 = weights.iter().sum();
        if wsum <= 0.0 {
            return 0;
        }
        let unit = (seed as f64) / (u64::MAX as f64);
        let mut r = unit * wsum;
        for (i, w) in weights.iter().enumerate() {
            if r <= *w || i + 1 == weights.len() {
                return i;
            }
            r -= w;
        }
        weights.len().saturating_sub(1)
    }

    fn brush_weighted_order_strings(ids: &mut Vec<String>, seed: u64, weight_map: &HashMap<String, f64>) {
        if ids.len() < 2 {
            return;
        }
        let uniform = 1.0 / ids.len() as f64;
        let mut remaining: Vec<String> = std::mem::take(ids);
        let mut state = seed;
        while !remaining.is_empty() {
            let weights: Vec<f64> = remaining.iter().map(|id| Self::brush_kind_weight(weight_map, id.as_str(), uniform)).collect();
            state = Self::brush_next_seed(state);
            let pick = Self::brush_weighted_sample_index(&weights, state);
            ids.push(remaining.remove(pick));
        }
    }

    fn brush_compatible_candidates(&self, source: &HandleData) -> Vec<BrushCandidate> {
        let sn = self.nodes.get(&source.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
        let sh = source.handle_kind.as_str();
        let mut out: Vec<BrushCandidate> = Vec::new();
        for (kind_id, kind) in &self.node_kinds {
            if kind.handles.is_empty() {
                continue;
            }
            let tn = kind_id.as_str();
            for (i, tmpl) in kind.handles.iter().enumerate() {
                if self.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                    out.push(BrushCandidate { node_kind_id: kind_id.clone(), target_handle_index: i });
                }
            }
        }
        out
    }

    fn brush_handle_alignment_delta(source_handle_angle: f64, target_template_angle: f64) -> f64 {
        let desired = source_handle_angle + std::f64::consts::PI;
        let mut d = (target_template_angle - desired).abs();
        if d > std::f64::consts::PI {
            d = std::f64::consts::TAU - d;
        }
        d
    }

    fn brush_sort_candidates_by_handle_proximity(&self, source: &HandleData, candidates: &mut [BrushCandidate]) {
        let source_angle = source.angle;
        candidates.sort_by(|left, right| {
            let angle_left = self.node_kinds.get(left.node_kind_id.as_str()).and_then(|kind| kind.handles.get(left.target_handle_index)).map(|tmpl| tmpl.angle).unwrap_or(0.0);
            let angle_right = self.node_kinds.get(right.node_kind_id.as_str()).and_then(|kind| kind.handles.get(right.target_handle_index)).map(|tmpl| tmpl.angle).unwrap_or(0.0);
            let delta_left = Self::brush_handle_alignment_delta(source_angle, angle_left);
            let delta_right = Self::brush_handle_alignment_delta(source_angle, angle_right);
            delta_left.partial_cmp(&delta_right).unwrap_or(std::cmp::Ordering::Equal).then_with(|| left.node_kind_id.cmp(&right.node_kind_id)).then_with(|| left.target_handle_index.cmp(&right.target_handle_index))
        });
    }

    fn brush_template_world_pos(&self, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, angle: f64) -> Point {
        match shape {
            NodeShape::Circle => handle_position_on_circle(center, radius, angle),
            NodeShape::Rectangle => handle_position_on_rectangle(center, width, height, angle),
        }
    }

    fn brush_build_preview(&self, source_handle_id: &str, candidate: &BrushCandidate) -> Option<BrushPreviewSnapshot> {
        let source = self.handles.get(source_handle_id)?;
        let kind = self.node_kinds.get(candidate.node_kind_id.as_str())?;
        let center = self.brush_slot_center_world(source)?;
        let target_handle_index = candidate.target_handle_index;
        if kind.handles.get(target_handle_index).is_none() {
            return None;
        }
        let node_kind_id = candidate.node_kind_id.as_str();
        let radius = self.brush_node_size * 0.5 * kind.scale;
        let (width, height) = if kind.shape == NodeShape::Rectangle { (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
        Some(BrushPreviewSnapshot {
            source_handle_id: source_handle_id.to_string(),
            node_kind_id: node_kind_id.to_string(),
            x: center.x,
            y: center.y,
            shape: kind.shape,
            radius,
            width,
            height,
            handles: kind.handles.clone(),
            target_handle_index,
            icon_kind: kind.icon.clone(),
        })
    }

    fn brush_preview_json(preview: &BrushPreviewSnapshot) -> serde_json::Value {
        let mut node = json!({
            "nodeKind": preview.node_kind_id,
            "x": preview.x,
            "y": preview.y,
            "shape": if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" },
        });
        if preview.shape == NodeShape::Rectangle {
            node["width"] = json!(preview.width);
            node["height"] = json!(preview.height);
        } else {
            node["radius"] = json!(preview.radius);
        }
        if let Some(ref icon) = preview.icon_kind {
            node["iconKind"] = json!(icon);
        }
        let handles: Vec<_> = preview
            .handles
            .iter()
            .map(|h| {
                let mut row = json!({ "angle": h.angle, "handleKind": h.handle_kind });
                if let Some(r) = h.radius {
                    row["radius"] = json!(r);
                }
                row
            })
            .collect();
        node["handles"] = json!(handles);
        json!({
            "node": node,
            "edge": {
                "sourceHandleId": preview.source_handle_id,
                "targetHandleIndex": preview.target_handle_index,
            }
        })
    }

    fn brush_place_json(preview: &BrushPreviewSnapshot, node_id: &str, edge_id: &str) -> serde_json::Value {
        let mut flat = json!({
            "nodeId": node_id,
            "edgeId": edge_id,
            "nodeKind": preview.node_kind_id,
            "sourceHandleId": preview.source_handle_id,
            "targetHandleIndex": preview.target_handle_index,
            "x": preview.x,
            "y": preview.y,
            "shape": if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" },
        });
        if preview.shape == NodeShape::Rectangle {
            flat["width"] = json!(preview.width);
            flat["height"] = json!(preview.height);
        } else {
            flat["radius"] = json!(preview.radius);
        }
        if let Some(ref icon) = preview.icon_kind {
            flat["iconKind"] = json!(icon);
        }
        let handles: Vec<_> = preview
            .handles
            .iter()
            .map(|h| {
                let mut row = json!({ "angle": h.angle, "handleKind": h.handle_kind });
                if let Some(r) = h.radius {
                    row["radius"] = json!(r);
                }
                row
            })
            .collect();
        flat["handles"] = json!(handles);
        flat
    }

    fn brush_sync_preview_events(&mut self) {
        let key = self.brush_preview.as_ref().map(|p| format!("{}|{}|{}|{}|{}", p.source_handle_id, p.node_kind_id, p.target_handle_index, p.x, p.y)).unwrap_or_default();
        if self.brush_preview_emit_key.as_deref() != Some(key.as_str()) {
            self.brush_preview_emit_key = Some(key.clone());
            if let Some(ref preview) = self.brush_preview {
                self.push_event("brushPreview", Self::brush_preview_json(preview));
            } else {
                self.push_event("brushPreview", json!({ "node": null, "edge": null }));
            }
        }
        let candidates_key =
            format!("{}|{}|{}", self.brush_slot_source_id.as_deref().unwrap_or(""), self.brush_candidates.iter().map(|c| format!("{}#{}", c.node_kind_id, c.target_handle_index)).collect::<Vec<_>>().join(","), self.brush_candidate_index);
        if self.brush_candidates_emit_key.as_deref() != Some(candidates_key.as_str()) {
            self.brush_candidates_emit_key = Some(candidates_key);
            let candidates: Vec<_> = self
                .brush_candidates
                .iter()
                .map(|c| {
                    json!({
                        "nodeKind": c.node_kind_id,
                        "targetHandleIndex": c.target_handle_index,
                    })
                })
                .collect();
            self.push_event(
                "brushCandidates",
                json!({
                    "sourceHandleId": self.brush_slot_source_id.clone().unwrap_or_default(),
                    "candidates": candidates,
                    "index": self.brush_candidate_index,
                    "suggestionsActive": self.brush_slot_suggestions_active,
                }),
            );
        }
    }

    fn brush_clear_slot(&mut self) {
        let had_preview = self.brush_preview.is_some();
        let clear_hover = self.brush_slot_suggestions_active;
        self.brush_slot_suggestions_active = false;
        self.brush_slot_source_id = None;
        self.brush_candidates.clear();
        self.brush_candidate_index = 0;
        self.brush_preview = None;
        if clear_hover {
            self.set_hovered_id(None);
        }
        if had_preview {
            self.bump_content_scene_generation();
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.brush_sync_preview_events();
        }
    }

    fn brush_allocate_placement_ids(&mut self) -> (String, String) {
        self.brush_placement_serial = self.brush_placement_serial.wrapping_add(1);
        let serial = self.brush_placement_serial;
        (format!("puzzle2d.brush.{serial}"), format!("puzzle2d.brush.edge.{serial}"))
    }

    fn brush_commit_preview(&mut self) {
        let Some(preview) = self.brush_preview.take() else {
            return;
        };
        let (node_id, edge_id) = self.brush_allocate_placement_ids();
        self.push_event("brushPlace", Self::brush_place_json(&preview, node_id.as_str(), edge_id.as_str()));
        self.bump_content_scene_generation();
        self.brush_preview_emit_key = None;
    }

    fn brush_finish_slot(&mut self) {
        if self.brush_alt_pressed {
            self.brush_commit_preview();
        }
        self.brush_clear_slot();
    }

    fn brush_update_alt(&mut self, alt: bool) {
        if self.brush_alt_pressed == alt {
            return;
        }
        self.brush_alt_pressed = alt;
        if self.brush_slot_source_id.is_some() {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }

    //#region 🪣Fill
    fn fill_preview_bounds(preview: &BrushPreviewSnapshot) -> WorldBox {
        match preview.shape {
            NodeShape::Rectangle => WorldBox { min_x: preview.x - preview.width / 2.0, min_y: preview.y - preview.height / 2.0, max_x: preview.x + preview.width / 2.0, max_y: preview.y + preview.height / 2.0 },
            NodeShape::Circle => WorldBox { min_x: preview.x - preview.radius, min_y: preview.y - preview.radius, max_x: preview.x + preview.radius, max_y: preview.y + preview.radius },
        }
    }

    fn fill_handle_connected(&self, accum: &FillAccum, handle_id: &str) -> bool {
        accum.connected_handles.contains(handle_id) || self.handle_has_incident_edge(handle_id)
    }

    fn fill_collect_free_handles(&self, accum: &FillAccum) -> Vec<String> {
        let mut out = Vec::new();
        for (id, h) in &self.handles {
            if self.handle_effectively_visible(id.as_str()) && !self.fill_handle_connected(accum, id.as_str()) {
                out.push(id.clone());
            }
            let _ = h;
        }
        for (id, vh) in &accum.virtual_handles {
            if !accum.connected_handles.contains(id) && accum.virtual_nodes.contains_key(&vh.node_id) {
                out.push(id.clone());
            }
        }
        out
    }

    fn fill_source_node_and_handle_kind(&self, accum: &FillAccum, handle_id: &str) -> Option<(String, String)> {
        if let Some(h) = self.handles.get(handle_id) {
            let nk = self.nodes.get(&h.node_id)?.node_kind.clone();
            return Some((nk, h.handle_kind.clone()));
        }
        let vh = accum.virtual_handles.get(handle_id)?;
        let node_kind = accum.virtual_nodes.get(&vh.node_id)?.node_kind.clone();
        Some((node_kind, vh.handle_kind.clone()))
    }

    fn fill_virtual_handle_anchor_world(node: &FillVirtualNode, tmpl: &NodeKindHandleTemplate) -> Point {
        let center = Point::new(node.x, node.y);
        match node.shape {
            NodeShape::Circle => handle_position_on_circle(center, node.radius, tmpl.angle),
            NodeShape::Rectangle => handle_position_on_rectangle(center, node.width, node.height, tmpl.angle),
        }
    }

    fn fill_slot_center_world(&self, accum: &FillAccum, handle_id: &str) -> Option<Point> {
        if let Some(h) = self.handles.get(handle_id) {
            let hw = self.brush_handle_anchor_world(h)?;
            return self.handle_slot_center_world(h.node_id.as_str(), hw, self.suggestion_offset);
        }
        let vh = accum.virtual_handles.get(handle_id)?;
        let node = accum.virtual_nodes.get(&vh.node_id)?;
        let hw = Self::fill_virtual_handle_anchor_world(node, &vh.template);
        let nc = Point::new(node.x, node.y);
        let normal = normalize_or_zero(hw - nc);
        Some(hw + normal * self.suggestion_offset)
    }

    fn fill_weight_for_handle(&self, accum: &FillAccum, handle_id: &str, uniform: f64) -> f64 {
        let hk = if let Some(h) = self.handles.get(handle_id) { h.handle_kind.as_str() } else { accum.virtual_handles.get(handle_id).map(|vh| vh.handle_kind.as_str()).unwrap_or("") };
        Self::brush_kind_weight(&self.brush_handle_kind_weights, hk, uniform)
    }

    fn fill_order_handles(&self, accum: &FillAccum, handles: &mut Vec<String>, seed: u64) {
        if handles.len() < 2 {
            return;
        }
        let uniform = 1.0 / handles.len() as f64;
        let mut remaining = std::mem::take(handles);
        let mut state = seed;
        while !remaining.is_empty() {
            let weights: Vec<f64> = remaining.iter().map(|id| self.fill_weight_for_handle(accum, id.as_str(), uniform)).collect();
            state = Self::brush_next_seed(state);
            let pick = Self::brush_weighted_sample_index(&weights, state);
            handles.push(remaining.remove(pick));
        }
    }

    fn fill_compatible_node_kind_ids(&self, accum: &FillAccum, source_handle_id: &str) -> Vec<String> {
        let Some((sn, sh)) = self.fill_source_node_and_handle_kind(accum, source_handle_id) else {
            return Vec::new();
        };
        let mut out: Vec<String> = Vec::new();
        for (kind_id, kind) in &self.node_kinds {
            if kind.handles.is_empty() {
                continue;
            }
            let tn = kind_id.as_str();
            let compatible = kind.handles.iter().any(|t| self.link_kinds_compatible_for_brush(sn.as_str(), sh.as_str(), tn, t.handle_kind.as_str()));
            if compatible {
                out.push(kind_id.clone());
            }
        }
        out
    }

    fn fill_pick_target_handle_index(&self, sn: &str, sh: &str, node_kind_id: &str, kind: &NodeKindDef, seed: u64) -> Option<usize> {
        let tn = node_kind_id;
        let mut compatible: Vec<(usize, f64)> = Vec::new();
        for (i, tmpl) in kind.handles.iter().enumerate() {
            if !self.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                continue;
            }
            let w = Self::brush_kind_weight(&self.brush_handle_kind_weights, tmpl.handle_kind.as_str(), 1.0);
            compatible.push((i, w));
        }
        if compatible.is_empty() {
            return None;
        }
        let weights: Vec<f64> = compatible.iter().map(|(_, w)| *w).collect();
        let pick = Self::brush_weighted_sample_index(&weights, seed);
        Some(compatible[pick].0)
    }

    fn fill_build_preview(&self, accum: &FillAccum, source_handle_id: &str, node_kind_id: &str, seed: u64) -> Option<BrushPreviewSnapshot> {
        let center = self.fill_slot_center_world(accum, source_handle_id)?;
        let (sn, sh) = self.fill_source_node_and_handle_kind(accum, source_handle_id)?;
        let kind = self.node_kinds.get(node_kind_id)?;
        let target_handle_index = self.fill_pick_target_handle_index(sn.as_str(), sh.as_str(), node_kind_id, kind, seed)?;
        let radius = self.brush_node_size * 0.5 * kind.scale;
        let (width, height) = if kind.shape == NodeShape::Rectangle { (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
        Some(BrushPreviewSnapshot {
            source_handle_id: source_handle_id.to_string(),
            node_kind_id: node_kind_id.to_string(),
            x: center.x,
            y: center.y,
            shape: kind.shape,
            radius,
            width,
            height,
            handles: kind.handles.clone(),
            target_handle_index,
            icon_kind: kind.icon.clone(),
        })
    }

    fn fill_collides(&self, accum: &FillAccum, preview: &BrushPreviewSnapshot) -> bool {
        let bounds = Self::fill_preview_bounds(preview);
        for n in self.nodes.values() {
            if world_boxes_overlap(bounds, self.node_world_bounds(n, 0.0)) {
                return true;
            }
        }
        for vb in &accum.virtual_bounds {
            if world_boxes_overlap(bounds, *vb) {
                return true;
            }
        }
        false
    }

    fn fill_apply_placement(accum: &mut FillAccum, preview: BrushPreviewSnapshot) {
        let serial = accum.next_serial;
        accum.next_serial += 1;
        let node_id = format!("puzzle2d.fill.{serial}");
        let edge_id = format!("puzzle2d.fill.edge.{serial}");
        let target_handle_id = format!("{node_id}:h{}", preview.target_handle_index);
        accum.connected_handles.insert(preview.source_handle_id.clone());
        accum.connected_handles.insert(target_handle_id);
        accum.virtual_bounds.push(Self::fill_preview_bounds(&preview));
        accum.virtual_nodes.insert(node_id.clone(), FillVirtualNode { node_kind: preview.node_kind_id.clone(), x: preview.x, y: preview.y, shape: preview.shape, radius: preview.radius, width: preview.width, height: preview.height });
        for (i, tmpl) in preview.handles.iter().enumerate() {
            let hid = format!("{node_id}:h{i}");
            if accum.connected_handles.contains(&hid) {
                continue;
            }
            accum.virtual_handles.insert(hid, FillVirtualHandle { node_id: node_id.clone(), handle_kind: tmpl.handle_kind.clone(), template: tmpl.clone() });
        }
        accum.placements.push((node_id, edge_id, preview));
    }

    fn brush_fill_try_place_once(&self, accum: &mut FillAccum, state: &mut u64, max: usize) -> bool {
        if accum.placements.len() >= max {
            return false;
        }
        let mut free = self.fill_collect_free_handles(accum);
        if free.is_empty() {
            return false;
        }
        *state = Self::brush_next_seed(*state);
        self.fill_order_handles(accum, &mut free, *state);
        for source_handle_id in &free {
            let mut kinds = self.fill_compatible_node_kind_ids(accum, source_handle_id.as_str());
            if kinds.is_empty() {
                continue;
            }
            *state = Self::brush_next_seed(*state);
            Self::brush_weighted_order_strings(&mut kinds, *state, &self.brush_node_kind_weights);
            for node_kind_id in &kinds {
                *state = Self::brush_next_seed(*state);
                let Some(preview) = self.fill_build_preview(accum, source_handle_id.as_str(), node_kind_id.as_str(), *state) else {
                    continue;
                };
                if self.fill_collides(accum, &preview) {
                    continue;
                }
                Self::fill_apply_placement(accum, preview);
                return true;
            }
        }
        false
    }

    fn brush_fill_placements_json(accum: &FillAccum, from: usize) -> Vec<serde_json::Value> {
        accum
            .placements
            .iter()
            .skip(from)
            .map(|(node_id, edge_id, preview)| Self::brush_place_json(preview, node_id.as_str(), edge_id.as_str()))
            .collect()
    }

    /// @emoji 🪣 Deterministic frontier fill sequence (weighted distribution + AABB collision).
    pub fn brush_fill_json(&self, max_count: u32, seed: u64) -> String {
        let mut accum = FillAccum::default();
        let max = max_count.min(1000) as usize;
        let mut state = seed;
        while accum.placements.len() < max {
            if !self.brush_fill_try_place_once(&mut accum, &mut state, max) {
                break;
            }
        }
        let placements = Self::brush_fill_placements_json(&accum, 0);
        serde_json::json!({ "placements": placements }).to_string()
    }

    /// @emoji 🪣 Starts a resumable fill session for chunked builds.
    pub fn brush_fill_session_begin(&mut self, max_count: u32, seed: u64) {
        self.brush_fill_session = Some(BrushFillSession {
            accum: FillAccum::default(),
            state: seed,
            max_count: max_count.min(1000) as usize,
            stalled: false,
        });
    }

    /// @emoji 🪣 Clears the resumable fill session.
    pub fn brush_fill_session_clear(&mut self) {
        self.brush_fill_session = None;
    }

    /// @emoji 🪣 Places up to `chunk_budget` fill nodes and returns new placements since the last step.
    pub fn brush_fill_session_step(&mut self, chunk_budget: u32) -> String {
        let Some(mut session) = self.brush_fill_session.take() else {
            return serde_json::json!({ "placements": [], "done": true, "count": 0 }).to_string();
        };
        if session.stalled || session.accum.placements.len() >= session.max_count {
            let done = session.stalled || session.accum.placements.len() >= session.max_count;
            let count = session.accum.placements.len();
            self.brush_fill_session = Some(session);
            return serde_json::json!({ "placements": [], "done": done, "count": count }).to_string();
        }
        let before = session.accum.placements.len();
        let budget = chunk_budget.clamp(1, 64) as usize;
        for _ in 0..budget {
            if session.accum.placements.len() >= session.max_count {
                break;
            }
            if !self.brush_fill_try_place_once(&mut session.accum, &mut session.state, session.max_count) {
                session.stalled = true;
                break;
            }
        }
        let placements = Self::brush_fill_placements_json(&session.accum, before);
        let done = session.stalled || session.accum.placements.len() >= session.max_count;
        let count = session.accum.placements.len();
        self.brush_fill_session = Some(session);
        serde_json::json!({ "placements": placements, "done": done, "count": count }).to_string()
    }
    //#endregion 🪣Fill

    fn brush_preview_snapshot_from_session_json(node: &serde_json::Value, edge: &serde_json::Value, source_handle_id: &str) -> Option<BrushPreviewSnapshot> {
        let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
        let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
        let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
        let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
            Some("rectangle") => NodeShape::Rectangle,
            _ => NodeShape::Circle,
        };
        let (radius, width, height) = match shape {
            NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
            NodeShape::Rectangle => {
                let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                (0.0, w, h)
            }
        };
        let target_handle_index = edge.get("targetHandleIndex").and_then(|v| v.as_u64()).map(|v| v as usize)?;
        let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
        if let Some(arr) = node.get("handles").and_then(|x| x.as_array()) {
            for row in arr {
                let ho = row.as_object()?;
                let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
                let angle = ho.get("angle").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                let radius = ho.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
            }
        }
        if handles.is_empty() {
            return None;
        }
        let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        Some(BrushPreviewSnapshot { source_handle_id: source_handle_id.to_string(), node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, handles, target_handle_index, icon_kind })
    }

    /// @emoji 🖌️ Mirrors brush slot + preview from another authoring pane (no pointer input on this host).
    pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), String> {
        if json.trim().is_empty() {
            self.brush_slot_suggestions_active = false;
            self.brush_slot_source_id = None;
            self.brush_candidates.clear();
            self.brush_candidate_index = 0;
            self.brush_preview = None;
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.bump_content_scene_generation();
            return Ok(());
        }
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("setBrushSessionJson: {e}"))?;
        let source = v.get("sourceHandleId").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        self.brush_slot_source_id = source.clone();
        self.brush_candidates = v
            .get("candidates")
            .and_then(|x| x.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| {
                        if let Some(kind_id) = x.as_str().map(str::trim).filter(|s| !s.is_empty()) {
                            return Some(BrushCandidate { node_kind_id: kind_id.to_string(), target_handle_index: 0 });
                        }
                        let node_kind = x.get("nodeKind").or_else(|| x.get("nodeKindId")).and_then(|n| n.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
                        let target_handle_index = x.get("targetHandleIndex").and_then(|i| i.as_u64()).unwrap_or(0) as usize;
                        Some(BrushCandidate { node_kind_id: node_kind.to_string(), target_handle_index })
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.brush_candidate_index = v.get("index").and_then(|x| x.as_u64()).map(|i| i as usize).unwrap_or(0);
        if self.brush_candidates.is_empty() {
            self.brush_candidate_index = 0;
        } else {
            self.brush_candidate_index %= self.brush_candidates.len();
        }
        self.brush_preview = match (source.as_deref(), v.get("preview")) {
            (Some(source_id), Some(preview)) if !preview.is_null() => {
                let node = preview.get("node").filter(|n| !n.is_null());
                let edge = preview.get("edge").filter(|e| !e.is_null());
                match (node, edge) {
                    (Some(node), Some(edge)) => Self::brush_preview_snapshot_from_session_json(node, edge, source_id),
                    _ => None,
                }
            }
            _ => None,
        };
        self.brush_preview_emit_key = None;
        self.brush_candidates_emit_key = None;
        self.brush_slot_suggestions_active = v.get("suggestionsActive").and_then(|x| x.as_bool()).unwrap_or(false);
        if self.brush_preview.is_none() && !self.brush_candidates.is_empty() {
            self.brush_rebuild_preview();
        } else {
            self.brush_sync_preview_events();
        }
        self.bump_content_scene_generation();
        Ok(())
    }

    fn brush_enter_slot(&mut self, source_handle_id: String) {
        if self.brush_slot_source_id.as_deref() == Some(source_handle_id.as_str()) {
            return;
        }
        if self.brush_slot_source_id.is_some() {
            self.brush_finish_slot();
        }
        self.brush_slot_source_id = Some(source_handle_id.clone());
        let Some(source) = self.handles.get(source_handle_id.as_str()).cloned() else {
            self.brush_candidates.clear();
            self.brush_candidate_index = 0;
            self.brush_rebuild_preview();
            return;
        };
        let mut candidates = self.brush_compatible_candidates(&source);
        self.brush_sort_candidates_by_handle_proximity(&source, &mut candidates);
        self.brush_candidates = candidates;
        self.brush_candidate_index = 0;
        self.brush_rebuild_preview();
    }

    fn brush_rebuild_preview(&mut self) {
        let Some(ref source_id) = self.brush_slot_source_id else {
            self.brush_preview = None;
            self.brush_sync_preview_events();
            return;
        };
        let candidate = self.brush_candidates.get(self.brush_candidate_index).cloned();
        self.brush_preview = candidate.as_ref().and_then(|c| self.brush_build_preview(source_id, c));
        if self.brush_preview.is_some() {
            self.bump_content_scene_generation();
        }
        self.brush_preview_emit_key = None;
        self.brush_candidates_emit_key = None;
        self.brush_sync_preview_events();
    }

    fn brush_pointer_move(&mut self, world: Point) {
        if let Some(slot) = self.brush_nearest_slot_source(world) {
            self.brush_enter_slot(slot);
            self.set_hovered_id(self.brush_slot_source_id.clone());
        } else if self.brush_slot_source_id.is_some() {
            self.brush_finish_slot();
            self.set_hovered_id(None);
        }
    }

    pub fn set_active_tool(&mut self, label: &str) {
        let next = if label == "brush" { ActiveTool::Brush } else { ActiveTool::Select };
        if self.active_tool == next {
            return;
        }
        if self.active_tool == ActiveTool::Brush {
            self.brush_finish_slot();
        }
        self.active_tool = next;
        self.interaction = Interaction::None;
        self.bump_content_scene_generation();
    }

    pub fn set_suggestion_offset(&mut self, distance: f64) {
        let d = if distance.is_finite() && distance >= 0.0 { distance } else { DEFAULT_SUGGESTION_OFFSET };
        if (self.suggestion_offset - d).abs() < 1e-9 {
            return;
        }
        self.suggestion_offset = d;
        if self.active_tool == ActiveTool::Brush {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }

    pub fn set_brush_kind_weights(&mut self, json: &str) {
        if json.is_empty() {
            return;
        }
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
            return;
        };
        self.brush_node_kind_weights.clear();
        self.brush_handle_kind_weights.clear();
        if let Some(obj) = v.get("nodeWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush_node_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if let Some(obj) = v.get("handleWeights").and_then(|x| x.as_object()) {
            for (k, val) in obj {
                if let Some(w) = val.as_f64() {
                    if w.is_finite() && w >= 0.0 {
                        self.brush_handle_kind_weights.insert(k.clone(), w);
                    }
                }
            }
        }
        if self.active_tool != ActiveTool::Brush {
            return;
        }
        if let Some(source) = self.brush_slot_source_id.clone() {
            self.brush_slot_source_id = None;
            self.brush_enter_slot(source);
        } else {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }

    pub fn set_brush_node_size(&mut self, size: f64) {
        let s = if size.is_finite() && size > 0.0 { size } else { DEFAULT_BRUSH_NODE_SIZE };
        if (self.brush_node_size - s).abs() < 1e-9 {
            return;
        }
        self.brush_node_size = s;
        if self.active_tool == ActiveTool::Brush {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }

    pub fn brush_cycle_candidate(&mut self, forward: bool) {
        if self.brush_candidates.len() < 2 {
            return;
        }
        let len = self.brush_candidates.len();
        self.brush_candidate_index = if forward { (self.brush_candidate_index + 1) % len } else { (self.brush_candidate_index + len - 1) % len };
        self.brush_rebuild_preview();
    }

    pub fn brush_set_candidate_index(&mut self, index: usize) {
        if self.brush_candidates.is_empty() {
            return;
        }
        self.brush_candidate_index = index % self.brush_candidates.len();
        self.brush_rebuild_preview();
    }

    /// @emoji 🖌️ Opens a brush slot on a free handle (suggestions menu; works outside brush tool).
    pub fn brush_open_slot(&mut self, handle_id: &str) {
        if !self.handles.contains_key(handle_id) {
            return;
        }
        self.brush_enter_slot(handle_id.to_string());
        self.brush_slot_suggestions_active = true;
        self.brush_rebuild_preview();
        self.set_hovered_id(Some(handle_id.to_string()));
    }

    /// @emoji 🖌️ Commits the active brush preview and clears the slot.
    pub fn brush_commit_slot(&mut self) {
        self.brush_commit_preview();
        self.brush_clear_slot();
    }

    /// @emoji 🖌️ Discards the active brush slot without placing.
    pub fn brush_cancel_slot(&mut self) {
        self.brush_clear_slot();
    }

    fn append_brush_node_icon_paint(&self, scene: &mut Scene, lod: BoardDrawLod, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: &str, fill: Color, stroke_c: Color, world_space: bool) {
        if !matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
            return;
        }
        let preserve_original_style = false;
        let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.vello_theme);
        let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(icon_kind, icon_fg, icon_bg, preserve_original_style) else {
            return;
        };
        let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
        let fit_inset = ui_styling::metrics::icon::FIT_INSET;
        let (sx_half, sy_half) = match shape {
            NodeShape::Circle => {
                let s = self.draw_space_len(radius, world_space) * fit_inset;
                (s, s)
            }
            NodeShape::Rectangle => (self.draw_space_len(width, world_space) * fit_inset * 0.5, self.draw_space_len(height, world_space) * fit_inset * 0.5),
        };
        let center_ds = self.draw_space_point(center, world_space);
        let cx = bx + bw * 0.5;
        let cy = by + bh * 0.5;
        let avail_w = 2.0 * sx_half;
        let avail_h = 2.0 * sy_half;
        let scale = (avail_w / bw).min(avail_h / bh);
        let aff = Affine::translate((center_ds.x - scale * cx, center_ds.y - scale * cy)) * Affine::scale(scale);
        match shape {
            NodeShape::Circle => {
                let r_clip = self.draw_space_len(radius, world_space) * clip_inset;
                let disc = Circle::new(center_ds, r_clip);
                scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
                match &body {
                    CachedIconBody::Vector(icon_scene) => {
                        scene.append(icon_scene, Some(aff));
                    }
                    CachedIconBody::Raster(img) => {
                        scene.draw_image(&ImageBrush::new((**img).clone()), aff);
                    }
                }
                scene.pop_layer();
            }
            NodeShape::Rectangle => {
                let hw = self.draw_space_len(width, world_space) * clip_inset * 0.5;
                let hh = self.draw_space_len(height, world_space) * clip_inset * 0.5;
                let clip_r = Rect::from_points(Point::new(center_ds.x - hw, center_ds.y - hh), Point::new(center_ds.x + hw, center_ds.y + hh));
                scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &clip_r);
                match &body {
                    CachedIconBody::Vector(icon_scene) => {
                        scene.append(icon_scene, Some(aff));
                    }
                    CachedIconBody::Raster(img) => {
                        scene.draw_image(&ImageBrush::new((**img).clone()), aff);
                    }
                }
                scene.pop_layer();
            }
        }
    }

    fn paint_highlighted_node_preview(&self, scene: &mut Scene, _lod: BoardDrawLod, x: f64, y: f64, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: Option<&str>, world_space: bool) {
        let center = Point::new(x, y);
        let style = BoardElementStyleKind::Highlighted;
        let fill = Self::node_fill_for_style(&self.vello_theme, style);
        let stroke_c = Self::node_stroke_for_style(&self.vello_theme, style);
        let stroke = Stroke::new(ui_styling::strokes::NODE_BODY);
        match shape {
            NodeShape::Circle => {
                let c = self.draw_space_point(center, world_space);
                let r = self.draw_space_len(radius, world_space);
                let circle = Circle::new(c, r);
                scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
                scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &circle);
            }
            NodeShape::Rectangle => {
                let hw = width * 0.5;
                let hh = height * 0.5;
                let p0 = self.draw_space_point(Point::new(x - hw, y - hh), world_space);
                let p1 = self.draw_space_point(Point::new(x + hw, y + hh), world_space);
                let rect = Rect::from_points(p0, p1);
                scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &rect);
                scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &rect);
            }
        }
        if let Some(icon) = icon_kind.map(str::trim).filter(|s| !s.is_empty()) {
            self.append_brush_node_icon_paint(scene, BoardDrawLod::Detail, center, shape, radius, width, height, icon, fill, stroke_c, world_space);
        }
    }

    fn fixture_drop_preview_effective_dims(&self, preview: &FixtureDropPreviewSnapshot) -> (NodeShape, f64, f64, f64) {
        if let Some(kind) = self.node_kinds.get(preview.node_kind_id.as_str()) {
            let radius = self.brush_node_size * 0.5 * kind.scale;
            let (width, height) = if kind.shape == NodeShape::Rectangle { (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
            return (kind.shape, radius, width, height);
        }
        (preview.shape, preview.radius, preview.width, preview.height)
    }

    fn fixture_drop_preview_from_json(&self, node: &serde_json::Value) -> Option<FixtureDropPreviewSnapshot> {
        let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
        let (x, y) = match (node.get("screenX").and_then(|v| v.as_f64()).filter(|v| v.is_finite()), node.get("screenY").and_then(|v| v.as_f64()).filter(|v| v.is_finite())) {
            (Some(sx), Some(sy)) => {
                let world = self.screen_to_world(Point::new(sx, sy));
                (world.x, world.y)
            }
            _ => {
                let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                (x, y)
            }
        };
        let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
            Some("rectangle") => NodeShape::Rectangle,
            _ => NodeShape::Circle,
        };
        let (radius, width, height) = match shape {
            NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
            NodeShape::Rectangle => {
                let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                (0.0, w, h)
            }
        };
        let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
        Some(FixtureDropPreviewSnapshot { node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, icon_kind })
    }

    /// @emoji 👻 Sets or clears the workbench palette fixture drop ghost node (independent of brush tool).
    pub fn set_fixture_drop_preview_json(&mut self, json: &str) -> Result<(), String> {
        if json.trim().is_empty() {
            self.fixture_drop_preview = None;
            self.bump_content_scene_generation();
            return Ok(());
        }
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("setFixtureDropPreviewJson: {e}"))?;
        self.fixture_drop_preview = self.fixture_drop_preview_from_json(&v);
        if self.fixture_drop_preview.is_none() {
            return Err("setFixtureDropPreviewJson: preview payload missing nodeKind, screen/world point, or size".into());
        }
        self.bump_content_scene_generation();
        Ok(())
    }

    fn append_fixture_drop_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
        let Some(ref preview) = self.fixture_drop_preview else {
            return;
        };
        let (shape, radius, width, height) = self.fixture_drop_preview_effective_dims(preview);
        let icon_kind = preview.icon_kind.as_deref().filter(|s| !s.is_empty()).or_else(|| self.node_kinds.get(preview.node_kind_id.as_str()).and_then(|k| k.icon.as_deref()));
        self.paint_highlighted_node_preview(scene, lod, preview.x, preview.y, shape, radius, width, height, icon_kind, world_space);
    }

    fn append_brush_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
        let Some(ref preview) = self.brush_preview else {
            return;
        };
        let _ = lod;
        self.paint_highlighted_node_preview(scene, lod, preview.x, preview.y, preview.shape, preview.radius, preview.width, preview.height, preview.icon_kind.as_deref(), world_space);
        let center = Point::new(preview.x, preview.y);
        let source = match self.handles.get(preview.source_handle_id.as_str()) {
            Some(h) => h,
            None => return,
        };
        let src_pos = match self.handle_world_pos(source) {
            Some(p) => p,
            None => return,
        };
        let tmpl = match preview.handles.get(preview.target_handle_index) {
            Some(t) => t,
            None => return,
        };
        let tgt_pos = self.brush_template_world_pos(center, preview.shape, preview.radius, preview.width, preview.height, tmpl.angle);
        let Some(src_node) = self.nodes.get(&source.node_id) else {
            return;
        };
        let tgt_center = center;
        let curve = compute_edge_bezier_points(src_pos, tgt_pos, Point::new(src_node.x, src_node.y), tgt_center);
        let p0 = self.draw_space_point(curve.p0, world_space);
        let p1 = self.draw_space_point(curve.p1, world_space);
        let p2 = self.draw_space_point(curve.p2, world_space);
        let p3 = self.draw_space_point(curve.p3, world_space);
        let bez = CubicBez::new(p0, p1, p2, p3);
        scene.stroke(&Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT), Affine::IDENTITY, self.vello_theme.wire_stroke_highlighted, None, &bez);
    }

    /// @emoji 🧩 Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
    pub fn set_world_raster_tiling(&mut self, mode: &str) {
        let next = if mode == "world-clip" { "world-clip".into() } else { "none".into() };
        if self.world_raster_tiling == next {
            return;
        }
        self.world_raster_tiling = next;
    }

    pub fn set_original_element_style(&mut self, enabled: bool) {
        if self.preserve_original_element_style == enabled {
            return;
        }
        self.preserve_original_element_style = enabled;
        self.icon_paint_cache.clear();
    }

    pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
        if points.is_none() {
            self.selection_preview_crossing = false;
        }
        self.selection_screen_preview = points;
    }

    pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        self.vello_theme.merge_from_json(json)?;
        self.icon_paint_cache.clear();
        Ok(())
    }

    fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
        if screen_points.len() < 2 {
            self.selection_screen_preview = None;
            self.selection_preview_crossing = false;
            return;
        }
        let last = *screen_points.last().unwrap_or(&start_screen);
        self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
        self.selection_screen_preview = Some(if self.selection_options.method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] });
    }

    fn push_event(&mut self, name: &str, payload: serde_json::Value) {
        self.events.push(json!({ "name": name, "payload": payload }));
    }

    /// @emoji 🏁 Emits final node coordinates after a drag gesture so hosts can commit declarative fixture state once.
    fn push_node_drag_end_events(&mut self, start_positions: &BTreeMap<String, (f64, f64)>) {
        let mut moves = Vec::with_capacity(start_positions.len());
        for id in start_positions.keys() {
            let Some(node) = self.nodes.get(id) else {
                continue;
            };
            moves.push(json!({ "id": id, "x": node.x, "y": node.y }));
        }
        if moves.is_empty() {
            return;
        }
        self.push_event("nodeDragEnd", json!({ "moves": moves }));
    }

    pub fn drain_events_json(&mut self) -> String {
        let out = serde_json::to_string(&self.events).unwrap_or_else(|_| "[]".into());
        self.events.clear();
        out
    }

    fn is_preselecting(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// @emoji 💠 Live area-select preview ids, or committed selection when not preselecting.
    fn selection_chrome_ids(&self) -> BTreeSet<String> {
        if self.is_preselecting() || !self.preselect.is_empty() {
            self.preselect.clone()
        } else {
            self.selection.clone()
        }
    }

    /// @emoji 🖱️ Empty selection on background click without exit/highlight chrome or preselect.
    fn clear_selection_on_background_click(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        self.last_select_emit_sig = None;
        self.selection_exit_highlight.clear();
        self.selection.clear();
        self.sync_selection_flags_to_objects();
        self.push_event("select", json!({ "ids": [], "exitHighlightIds": [] }));
    }

    fn sync_selection_flags_to_objects(&mut self) {
        let chrome = self.selection_chrome_ids();
        for n in self.nodes.values_mut() {
            n.selected = chrome.contains(&n.id);
        }
        for h in self.handles.values_mut() {
            h.selected = chrome.contains(&h.id);
        }
        for e in self.edges.values_mut() {
            e.selected = chrome.contains(&e.id);
        }
        for w in self.wires.values_mut() {
            w.selected = chrome.contains(&w.id);
        }
    }

    fn push_select_event(&mut self) {
        self.last_select_emit_sig = None;
        let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
        sorted.sort();
        self.push_event("select", json!({ "ids": sorted, "exitHighlightIds": [] }));
    }

    pub fn set_selection_ids(&mut self, ids: &[String]) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        if next == self.selection {
            return;
        }
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        self.selection_exit_highlight.clear();
        self.selection = next;
        self.sync_selection_flags_to_objects();
        self.push_select_event();
    }

    /// @emoji 🔇 Updates committed selection without emitting `select` (controlled React sync).
    pub fn set_selection_ids_silent(&mut self, ids: &[String]) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        if next == self.selection {
            return;
        }
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        self.selection_exit_highlight.clear();
        self.selection = next;
        self.sync_selection_flags_to_objects();
    }

    /// @emoji 🔇 Mirrors area-select preview chrome without emitting `preselect` (shared multi-view sync).
    pub fn set_preselect_state_silent(&mut self, ids: &[String], removed_ids: &[String]) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let removed: BTreeSet<String> = removed_ids.iter().cloned().collect();
        if self.preselect == next && self.preselect_removed == removed {
            return;
        }
        self.preselect = next;
        self.preselect_removed = removed;
        self.sync_selection_flags_to_objects();
    }

    fn set_selection_ids_gestured(&mut self, ids: &[String], gesture: Option<&str>) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let mut sorted: Vec<_> = next.iter().cloned().collect();
        sorted.sort();
        let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
        let sig = (sorted.clone(), gesture_owned.clone());
        if next == self.selection && self.last_select_emit_sig.as_ref() == Some(&sig) {
            return;
        }
        self.last_select_emit_sig = Some(sig);
        self.preselect.clear();
        self.preselect_removed.clear();
        self.last_preselect_emit_sig = None;
        if next != self.selection {
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
        }
        let mut payload = json!({ "ids": sorted, "exitHighlightIds": [] });
        if let Some(ref g) = gesture_owned {
            payload["gestureMergeMode"] = json!(g);
        }
        self.push_event("select", payload);
    }

    /// @emoji 👁️ Rectangle/lasso drag preview: `preselect` + `preselect_removed` (anchor \\ preselect); emits `preselect` only.
    fn apply_area_preselect(&mut self, anchor_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let sorted = Self::sorted_selection_ids(&next);
        let removed = Self::sorted_selection_ids(&anchor_ids.difference(&next).cloned().collect());
        let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
        let sig = (sorted.clone(), removed.clone(), gesture_owned.clone());
        if self.preselect == next && self.last_preselect_emit_sig.as_ref() == Some(&sig) {
            return;
        }
        self.last_preselect_emit_sig = Some(sig);
        self.preselect = next;
        self.preselect_removed = anchor_ids.difference(&self.preselect).cloned().collect();
        self.set_hovered_id_silent(None);
        self.sync_selection_flags_to_objects();
        let mut payload = json!({ "ids": sorted, "removedIds": removed });
        if let Some(ref g) = gesture_owned {
            payload["gestureMergeMode"] = json!(g);
        }
        self.push_event("preselect", payload);
    }

    fn sorted_selection_ids(set: &BTreeSet<String>) -> Vec<String> {
        let mut v: Vec<_> = set.iter().cloned().collect();
        v.sort();
        v
    }

    /// @emoji 🧿 Ends a rectangle/lasso cycle: commits `selection`, clears preselect (highlight only lives in preselect).
    fn commit_area_select_from_initial(&mut self, initial_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
        let next: BTreeSet<String> = ids.iter().cloned().collect();
        let sorted = Self::sorted_selection_ids(&next);
        let anchor = Self::sorted_selection_ids(initial_ids);
        let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
        self.last_select_emit_sig = None;
        self.last_preselect_emit_sig = None;
        self.preselect.clear();
        self.preselect_removed.clear();
        self.selection_exit_highlight.clear();
        self.selection = next;
        self.sync_selection_flags_to_objects();
        let mut payload = json!({ "ids": sorted, "anchorIds": anchor, "exitHighlightIds": [] });
        if let Some(ref g) = gesture_owned {
            payload["gestureMergeMode"] = json!(g);
        }
        self.push_event("select", payload);
    }

    /// @emoji 🧿 True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
    pub fn is_dragging_area_select(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// @emoji 🧿 True during area select, link gestures, node drag, or camera pan so JS can defer full `syncDescriptorJson` round-trips.
    pub fn defers_descriptor_sync_from_js(&self) -> bool {
        matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. } | Interaction::DragNodes { .. } | Interaction::Pan { .. })
    }

    pub fn world_to_screen(&self, p: Point) -> Point {
        infinite_cavas::camera::world_to_screen(&self.camera, &self.viewport(), p)
    }

    pub fn screen_to_world(&self, p: Point) -> Point {
        infinite_cavas::camera::screen_to_world(&self.camera, &self.viewport(), p)
    }

    fn node_kind_scale(&self, node_kind: &str) -> f64 {
        self.node_kinds.get(node_kind).map(|k| k.scale).unwrap_or(1.0)
    }

    fn handle_kind_scale(&self, handle_kind: &str) -> f64 {
        self.handle_kinds.get(handle_kind).map(|k| k.scale).unwrap_or(1.0)
    }

    fn effective_node_scale(&self, n: &NodeData) -> f64 {
        (n.scale * self.node_kind_scale(n.node_kind.as_str())).max(1e-9)
    }

    fn scaled_node_radius(&self, n: &NodeData) -> f64 {
        n.radius * self.effective_node_scale(n)
    }

    fn scaled_node_width(&self, n: &NodeData) -> f64 {
        n.width * self.effective_node_scale(n)
    }

    fn scaled_node_height(&self, n: &NodeData) -> f64 {
        n.height * self.effective_node_scale(n)
    }

    fn effective_handle_scale(&self, h: &HandleData) -> f64 {
        let node_scale = self.nodes.get(h.node_id.as_str()).map(|n| self.effective_node_scale(n)).unwrap_or(1.0);
        (node_scale * h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9)
    }

    pub(crate) fn effective_handle_radius(&self, h: &HandleData) -> f64 {
        h.radius * self.effective_handle_scale(h)
    }

    pub(crate) fn handle_world_pos(&self, h: &HandleData) -> Option<Point> {
        let n = self.nodes.get(&h.node_id)?;
        Some(match n.shape {
            NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n), h.angle),
            NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n), self.scaled_node_height(n), h.angle),
        })
    }

    /// @emoji 📐 Node half-extent for indirect ring layout: circle radius or half the shorter rectangle side.
    fn indirect_node_half_extent(&self, n: &NodeData) -> f64 {
        match n.shape {
            NodeShape::Circle => self.scaled_node_radius(n),
            NodeShape::Rectangle => self.scaled_node_width(n).min(self.scaled_node_height(n)) * 0.5,
        }
    }

    /// @emoji 📐 Radial world offset from node rim to indirect-handle center (`INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× half-extent) so ring–node proportions stay fixed when zooming.
    fn indirect_handle_ring_offset_world(&self, n: &NodeData) -> f64 {
        (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_RING_GAP_NODE_SCALE).max(1e-9)
    }

    /// @emoji 📐 Ghost link handles sit on a rim offset by `INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× node half-extent from the node body so ring spacing scales with the node at every zoom.
    pub fn indirect_handle_world_pos(&self, h: &HandleData) -> Option<Point> {
        let n = self.nodes.get(&h.node_id)?;
        let offset = self.indirect_handle_ring_offset_world(n);
        Some(match n.shape {
            NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n) + offset, h.angle),
            NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n) + 2.0 * offset, self.scaled_node_height(n) + 2.0 * offset, h.angle),
        })
    }

    /// @emoji 📐 Indirect-connect marker radius in world units: `INDIRECT_HANDLE_MARKER_NODE_SCALE`× circle radius or × half the shorter rectangle side.
    pub fn indirect_handle_marker_radius_world(&self, h: &HandleData) -> f64 {
        let Some(n) = self.nodes.get(&h.node_id) else {
            return (self.effective_handle_radius(h) * INDIRECT_HANDLE_MARKER_NODE_SCALE).max(1e-9);
        };
        let handle_local_scale = (h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9);
        (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_MARKER_NODE_SCALE * handle_local_scale).max(1e-9)
    }

    /// @emoji 🧭 Source handle id while a link wire is drawn (`LinkDragSnap` / `LinkTargetNode`).
    fn active_link_source_handle_id(&self) -> Option<&str> {
        match &self.interaction {
            Interaction::LinkDragSnap { source_id, .. } | Interaction::LinkTargetNode { source_id, .. } | Interaction::ExternalLinkPreview { source_id, .. } => Some(source_id.as_str()),
            _ => None,
        }
    }

    /// @emoji 🧭 Visible target node ids that expose at least one free handle compatible with `source_handle_id`.
    fn link_drag_compatible_target_node_ids(&self, source_handle_id: &str) -> Vec<String> {
        let Some(source) = self.handles.get(source_handle_id) else {
            return Vec::new();
        };
        let source_node_id = source.node_id.as_str();
        let mut out = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for (hid, h) in &self.handles {
            if h.node_id == source_node_id || !self.handle_effectively_visible(hid.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(hid.as_str()) {
                continue;
            }
            if !self.handles_link_compatible_for_drag(source, h) {
                continue;
            }
            if !self.nodes.get(&h.node_id).is_some_and(|n| n.visible) {
                continue;
            }
            if seen.insert(h.node_id.clone()) {
                out.push(h.node_id.clone());
            }
        }
        out.sort();
        out
    }

    /// @emoji 🧭 Count of visible free handles on `node_id` compatible with `source_handle_id`.
    fn link_compatible_handle_count_on_node(&self, source_handle_id: &str, node_id: &str) -> usize {
        let Some(source) = self.handles.get(source_handle_id) else {
            return 0;
        };
        if source.node_id == node_id {
            return 0;
        }
        self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) && self.handles_link_compatible_for_drag(source, h)).count()
    }

    /// @emoji 🧭 Free compatible handle ids on `node_id` for an active link from `source_handle_id`.
    fn link_compatible_handle_ids_on_node(&self, source_handle_id: &str, node_id: &str) -> Vec<String> {
        let Some(source) = self.handles.get(source_handle_id) else {
            return Vec::new();
        };
        let mut out: Vec<String> = self
            .handles
            .iter()
            .filter_map(|(id, h)| {
                if h.node_id != node_id {
                    return None;
                }
                if !self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) {
                    return None;
                }
                self.handles_link_compatible_for_drag(source, h).then(|| id.clone())
            })
            .collect();
        out.sort();
        out
    }

    /// @emoji 🧭 Compatible target node under `world` while a link wire is active (node body hit).
    fn link_drag_ring_target_node_id(&self, source_handle_id: &str, world: Point) -> Option<String> {
        let nid = self.resolve_node_hit_world(world)?;
        if self.handles.get(source_handle_id)?.node_id == nid {
            return None;
        }
        self.node_has_any_free_link_compatible_handle(source_handle_id, nid.as_str()).then_some(nid)
    }

    /// @emoji 🧭 Resolves which single node draws the overview/normal indirect handle ring when that node has **more than one** eligible free handles (otherwise the sole handle is implicit).
    fn indirect_ring_node_id(&self, lod: BoardDrawLod) -> Option<String> {
        if !matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            return None;
        }
        if let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction {
            if self.link_compatible_handle_count_on_node(source_id, target_node_id) > 1 {
                return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
            }
            return None;
        }
        if let Interaction::ExternalLinkPreview { ring_node_id: Some(target_node_id), ring_handle_ids, .. } = &self.interaction {
            if ring_handle_ids.len() > 1 {
                return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
            }
            return None;
        }
        if let Interaction::LinkDragSnap { source_id, end_world, .. } = &self.interaction {
            let ring_nid = self.link_drag_ring_target_node_id(source_id, *end_world)?;
            if self.link_compatible_handle_count_on_node(source_id, ring_nid.as_str()) > 1 {
                return Some(ring_nid);
            }
            return None;
        }
        if self.active_link_source_handle_id().is_some() {
            return None;
        }
        let ring_nid = if self.selection.len() == 1 {
            self.selection.iter().next()?.clone()
        } else {
            return None;
        };
        let n = self.nodes.get(&ring_nid).filter(|n| n.visible)?;
        if self.eligible_indirect_handle_count_on_node(n.id.as_str()) > 1 {
            Some(ring_nid)
        } else {
            None
        }
    }

    fn eligible_indirect_handle_count_on_node(&self, node_id: &str) -> usize {
        self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_effectively_visible(id.as_str()) && self.handle_eligible_indirect_connect_ring(id.as_str())).count()
    }

    /// @emoji 🧭 Returns the handle id when `node_id` has exactly one visible free indirect-eligible handle.
    fn sole_eligible_indirect_handle_on_node(&self, node_id: &str) -> Option<String> {
        let mut found: Option<String> = None;
        for (id, h) in &self.handles {
            if h.node_id != node_id || !self.handle_effectively_visible(id.as_str()) || !self.handle_eligible_indirect_connect_ring(id.as_str()) {
                continue;
            }
            if found.is_some() {
                return None;
            }
            found = Some(id.clone());
        }
        found
    }

    /// @emoji 🧭 When the drop target has exactly one free handle compatible with `source_handle_id`, returns that handle id (otherwise `None`).
    fn node_sole_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> Option<String> {
        let source = self.handles.get(source_handle_id)?;
        if source.node_id == target_node_id {
            return None;
        }
        let mut found: Option<String> = None;
        for (id, h) in &self.handles {
            if h.node_id != target_node_id || !self.handle_effectively_visible(id.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(id.as_str()) {
                continue;
            }
            if !self.handles_link_compatible_for_drag(source, h) {
                continue;
            }
            if found.is_some() {
                return None;
            }
            found = Some(id.clone());
        }
        found
    }

    fn point_in_node_world(&self, n: &NodeData, point: Point) -> bool {
        match n.shape {
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
            }
            NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
        }
    }

    fn sole_indirect_handle_hit_link_target(&self, point: Point) -> Option<String> {
        let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction else {
            return None;
        };
        let th = self.node_sole_free_link_compatible_handle(source_id, target_node_id)?;
        let n = self.nodes.get(target_node_id)?;
        if !n.visible {
            return None;
        }
        if !self.point_in_node_world(n, point) {
            return None;
        }
        Some(th)
    }

    fn sole_indirect_handle_hit_idle_selected_node(&self, point: Point) -> Option<String> {
        if !matches!(self.interaction, Interaction::None) {
            return None;
        }
        if self.selection.len() != 1 {
            return None;
        }
        let nid = self.selection.iter().next()?;
        if !self.nodes.contains_key(nid) {
            return None;
        }
        let sole = self.sole_eligible_indirect_handle_on_node(nid)?;
        let n = self.nodes.get(nid)?;
        if !n.visible {
            return None;
        }
        if !self.point_in_node_world(n, point) {
            return None;
        }
        Some(sole)
    }

    /// @emoji 🧭 True when `target_node_id` hosts at least one visible free handle that can pair with `source_handle_id` under link-compat rules.
    fn node_has_any_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> bool {
        let Some(source) = self.handles.get(source_handle_id) else {
            return false;
        };
        if source.node_id == target_node_id {
            return false;
        }
        for (hid, h) in &self.handles {
            if h.node_id != target_node_id || !self.handle_effectively_visible(hid.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(hid.as_str()) {
                continue;
            }
            if self.handles_link_compatible_for_drag(source, h) {
                return true;
            }
        }
        false
    }

    /// @emoji 💫 True when the handle may appear on a link-target ghost ring (`overview`/`normal` LOD).
    fn handle_eligible_link_target_ring(&self, handle_id: &str, source_handle_id: &str) -> bool {
        if !self.handle_effectively_visible(handle_id) || self.handle_has_incident_edge(handle_id) {
            return false;
        }
        let Some(source) = self.handles.get(source_handle_id) else {
            return false;
        };
        let Some(target) = self.handles.get(handle_id) else {
            return false;
        };
        if source.node_id == target.node_id {
            return false;
        }
        self.handles_link_compatible_for_drag(source, target)
    }

    fn indirect_ring_handle_eligible(&self, handle_id: &str, ring_node_id: &str) -> bool {
        if self.handles.get(handle_id).is_none_or(|h| h.node_id != ring_node_id) {
            return false;
        }
        if let Some(source_id) = self.active_link_source_handle_id() {
            self.handle_eligible_link_target_ring(handle_id, source_id)
        } else {
            self.handle_eligible_indirect_connect_ring(handle_id)
        }
    }

    fn link_drag_target_ring_hit(&self, source_id: &str, point: Point) -> Option<String> {
        if !matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            return None;
        }
        if let Interaction::ExternalLinkPreview { source_id: active_source, ring_handle_ids, .. } = &self.interaction {
            if active_source != source_id {
                return None;
            }
            let zoom = self.camera.zoom;
            for hid in ring_handle_ids {
                let Some(h) = self.handles.get(hid) else {
                    continue;
                };
                if !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                    continue;
                }
                let Some(pos) = self.indirect_handle_world_pos(h) else {
                    continue;
                };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
            return None;
        }
        let node_id = self.link_drag_ring_target_node_id(source_id, point)?;
        if self.link_compatible_handle_count_on_node(source_id, node_id.as_str()) <= 1 {
            return None;
        }
        let zoom = self.camera.zoom;
        for h in self.handles.values().rev() {
            if h.node_id != node_id || !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                continue;
            }
            let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
            let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
            if distance_between(point, pos) <= tol {
                return Some(h.id.clone());
            }
        }
        None
    }

    fn link_target_ring_snapshot(&self, source_handle_id: &str) -> (Option<String>, Vec<String>) {
        if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = &self.interaction {
            if source_id == source_handle_id {
                return (ring_node_id.clone(), ring_handle_ids.clone());
            }
        }
        let node_id = match &self.interaction {
            Interaction::LinkTargetNode { target_node_id, .. } => Some(target_node_id.clone()),
            Interaction::LinkDragSnap { end_world, .. } => self.link_drag_ring_target_node_id(source_handle_id, *end_world),
            _ => None,
        };
        let Some(nid) = node_id else {
            return (None, Vec::new());
        };
        if self.link_compatible_handle_count_on_node(source_handle_id, nid.as_str()) <= 1 {
            return (None, Vec::new());
        }
        (Some(nid.clone()), self.link_compatible_handle_ids_on_node(source_handle_id, nid.as_str()))
    }

    fn sync_link_gesture_events(&mut self) {
        if let Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } = self.interaction.clone() {
            let compat_key = format!("{}|{}", source_id, compatible_node_ids.join(","));
            if self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str()) {
                self.link_compat_nodes_emit_key = Some(compat_key);
                self.push_event("linkCompatibleNodes", json!({ "source": source_id, "nodeIds": compatible_node_ids }));
            }
            let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
            if self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str()) {
                self.link_target_ring_emit_key = Some(ring_key);
                self.push_event(
                    "linkTargetRing",
                    json!({
                        "source": source_id,
                        "nodeId": ring_node_id,
                        "handleIds": ring_handle_ids,
                    }),
                );
            }
            return;
        }
        let Some(source) = self.active_link_source_handle_id().map(str::to_string) else {
            self.clear_link_gesture_events();
            return;
        };
        let node_ids = self.link_drag_compatible_target_node_ids(&source);
        let compat_key = format!("{}|{}", source, node_ids.join(","));
        if self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str()) {
            self.link_compat_nodes_emit_key = Some(compat_key);
            self.push_event("linkCompatibleNodes", json!({ "source": source, "nodeIds": node_ids }));
        }
        let (ring_node_id, ring_handle_ids) = self.link_target_ring_snapshot(&source);
        let ring_key = format!("{}|{}|{}", source, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
        if self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str()) {
            self.link_target_ring_emit_key = Some(ring_key);
            self.push_event(
                "linkTargetRing",
                json!({
                    "source": source,
                    "nodeId": ring_node_id,
                    "handleIds": ring_handle_ids,
                }),
            );
        }
    }

    fn clear_link_gesture_events(&mut self) {
        if self.link_compat_nodes_emit_key.take().is_some() {
            self.push_event("linkCompatibleNodes", json!({ "source": "", "nodeIds": [] }));
        }
        if self.link_target_ring_emit_key.take().is_some() {
            self.push_event("linkTargetRing", json!({ "source": "", "nodeId": null, "handleIds": [] }));
        }
    }

    fn node_center_world(&self, node_id: &str) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        Some(Point::new(n.x, n.y))
    }

    fn edge_curve(&self, e: &EdgeData) -> Option<CubicBez> {
        if !self.has_ports() {
            let source_node = self.nodes.get(&e.source)?;
            let target_node = self.nodes.get(&e.target)?;
            let source_center = Point::new(source_node.x, source_node.y);
            let target_center = Point::new(target_node.x, target_node.y);
            let source_pos = self.node_rim_point_toward(source_node, target_center)?;
            let target_pos = self.node_rim_point_toward(target_node, source_center)?;
            return Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center));
        }
        let source_handle = self.handles.get(&e.source)?;
        let target_handle = self.handles.get(&e.target)?;
        let source_node = self.nodes.get(&source_handle.node_id)?;
        let target_node = self.nodes.get(&target_handle.node_id)?;
        let source_pos = self.handle_world_pos(source_handle)?;
        let target_pos = self.handle_world_pos(target_handle)?;
        Some(compute_edge_bezier_points(source_pos, target_pos, Point::new(source_node.x, source_node.y), Point::new(target_node.x, target_node.y)))
    }

    fn link_drag_wire_curve_world(&self, source_id: &str, target_id: Option<&str>, end_world: Point) -> Option<CubicBez> {
        let source_handle = self.handles.get(source_id)?;
        let source_node = self.nodes.get(&source_handle.node_id)?;
        let source_pos = self.handle_world_pos(source_handle)?;
        let source_center = Point::new(source_node.x, source_node.y);
        let (target_pos, target_center) = if let Some(tid) = target_id {
            let th = self.handles.get(tid)?;
            let tn = self.nodes.get(&th.node_id)?;
            (self.handle_world_pos(th)?, Point::new(tn.x, tn.y))
        } else {
            (end_world, end_world)
        };
        Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center))
    }

    fn active_link_wire_curve(&self) -> Option<CubicBez> {
        match &self.interaction {
            Interaction::LinkDragSnap { source_id, target_id, end_world } => self.link_drag_wire_curve_world(source_id.as_str(), target_id.as_deref(), *end_world),
            Interaction::LinkTargetNode { source_id, target_node_id } => self.link_drag_wire_curve_world(source_id.as_str(), None, self.node_center_world(target_node_id)?),
            Interaction::ExternalLinkPreview { source_id, end_world, .. } => self.link_drag_wire_curve_world(source_id.as_str(), None, *end_world),
            Interaction::DragNodes { proximity_pair: Some((src, tgt)), .. } => self.link_drag_wire_curve_world(src.as_str(), Some(tgt.as_str()), Point::ZERO),
            _ => None,
        }
    }

    fn wire_curve(&self, w: &WireData) -> Option<CubicBez> {
        let end_world = match (&w.target, w.end_x, w.end_y) {
            (None, Some(x), Some(y)) if x.is_finite() && y.is_finite() => Point::new(x, y),
            (Some(tid), _, _) => {
                self.handles.get(tid)?;
                return self.edge_curve(&EdgeData {
                    id: w.id.clone(),
                    source: w.source.clone(),
                    target: tid.clone(),
                    selected: w.selected,
                    visible: w.visible,
                    locked: w.locked,
                    style: w.style.clone(),
                    edge_kind: String::new(),
                    source_tip: None,
                    target_tip: None,
                });
            }
            _ => return None,
        };
        self.link_drag_wire_curve_world(w.source.as_str(), None, end_world)
    }

    fn apply_link_drag_snap_hover(&mut self, _source_handle_id: &str, world: Point, target_handle_id: Option<&str>) {
        if let Some(tid) = target_handle_id {
            self.set_hovered_id(Some(tid.to_string()));
        } else {
            self.update_hover_from_world(world);
        }
    }

    /// @emoji 🧭 Minimap/overview LOD: group selection and bounded drag only — no per-node/edge/handle picks.
    fn lod_disables_discrete_pick(&self) -> bool {
        matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
    }

    /// @emoji 🔗 Overview LOD: tight world-radius hit on a free handle so link drag can start without enabling broad `resolve_hit_world` handle picks.
    fn resolve_overview_free_link_handle_pointer_world(&self, point: Point) -> Option<String> {
        if !matches!(self.current_draw_lod(), BoardDrawLod::Overview) {
            return None;
        }
        if !self.selection_options.select_handles {
            return None;
        }
        const MAX_D_WORLD: f64 = 2.25;
        let mut best: Option<(f64, String)> = None;
        for h in self.handles.values() {
            if !self.handle_effectively_visible(h.id.as_str()) || self.handle_has_incident_edge(h.id.as_str()) {
                continue;
            }
            let Some(pos) = self.handle_world_pos(h) else {
                continue;
            };
            let d = distance_between(point, pos);
            if d <= MAX_D_WORLD && best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
                best = Some((d, h.id.clone()));
            }
        }
        best.map(|(_, id)| id)
    }

    /// @emoji 🧭 Minimap/overview LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
    fn lod_uses_bounded_drag(&self) -> bool {
        matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
    }

    fn resolve_hover_world(&self, point: Point) -> Option<String> {
        let lod = self.current_draw_lod();
        let zoom = self.camera.zoom;
        if self.has_ports() && !matches!(lod, BoardDrawLod::Minimap) {
            if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                    return Some(hid);
                }
                if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                    if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                        return Some(hid);
                    }
                }
            }
            if let Some(ring_node_id) = self.indirect_ring_node_id(lod) {
                for h in self.handles.values().rev() {
                    if h.node_id != ring_node_id || !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                for h in self.handles.values().rev() {
                    if !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                    if self.handle_selectable(hid.as_str()) {
                        return Some(hid);
                    }
                }
            }
        }
        for n in self.nodes.values().rev() {
            if !self.node_selectable(n.id.as_str()) {
                continue;
            }
            match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                        return Some(n.id.clone());
                    }
                }
                NodeShape::Circle => {
                    if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                        return Some(n.id.clone());
                    }
                }
            }
        }
        for w in self.wires.values().rev() {
            if !self.wire_selectable(w) {
                continue;
            }
            if let Some(c) = self.wire_curve(w) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    return Some(w.id.clone());
                }
            }
        }
        for e in self.edges.values().rev() {
            if !self.edge_selectable(e) {
                continue;
            }
            if let Some(c) = self.edge_curve(e) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    return Some(e.id.clone());
                }
            }
        }
        None
    }

    fn push_pick_target(out: &mut Vec<BoardPickTargetJson>, domain: &str, id: String, generality: u32, label: Option<String>) {
        if out.iter().any(|row| row.domain == domain && row.id == id) {
            return;
        }
        out.push(BoardPickTargetJson { domain: domain.to_string(), id, generality, label });
    }

    fn resolve_pick_targets_world(&self, point: Point) -> Vec<BoardPickTargetJson> {
        let mut out = Vec::new();
        let lod = self.current_draw_lod();
        let zoom = self.camera.zoom;
        if self.has_ports() && !matches!(lod, BoardDrawLod::Minimap) {
            if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                for h in self.handles.values().rev() {
                    if !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                    if distance_between(point, pos) <= tol {
                        Self::push_pick_target(&mut out, "handle", h.id.clone(), 2, Some(h.id.clone()));
                    }
                }
            }
        }
        for n in self.nodes.values().rev() {
            if !self.node_selectable(n.id.as_str()) {
                continue;
            }
            let hit = match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
                }
                NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
            };
            if hit {
                Self::push_pick_target(&mut out, "node", n.id.clone(), 0, n.text.clone());
            }
        }
        for w in self.wires.values().rev() {
            if !self.wire_selectable(w) {
                continue;
            }
            if let Some(c) = self.wire_curve(w) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    Self::push_pick_target(&mut out, "wire", w.id.clone(), 1, Some(w.id.clone()));
                }
            }
        }
        for e in self.edges.values().rev() {
            if !self.edge_selectable(e) {
                continue;
            }
            if let Some(c) = self.edge_curve(e) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    Self::push_pick_target(&mut out, "edge", e.id.clone(), 1, Some(e.id.clone()));
                }
            }
        }
        out
    }

    /// @emoji 🎯 All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        let world = self.screen_to_world_point(sx, sy);
        serde_json::to_string(&self.resolve_pick_targets_world(world)).unwrap_or_else(|_| "[]".into())
    }

    pub fn resolve_hit_world(&self, point: Point) -> Option<String> {
        if self.lod_disables_discrete_pick() {
            return None;
        }
        let zoom = self.camera.zoom;
        let o = &self.selection_options;
        if self.has_ports() && o.select_handles {
            if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                    return Some(hid);
                }
                if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                    if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                        return Some(hid);
                    }
                }
            }
            if let Some(ring_node_id) = self.indirect_ring_node_id(self.current_draw_lod()) {
                for h in self.handles.values().rev() {
                    if h.node_id != ring_node_id || !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                for h in self.handles.values().rev() {
                    if !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
            }
            if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                    return Some(hid);
                }
            }
        }
        if o.select_nodes {
            for n in self.nodes.values().rev() {
                if !self.node_selectable(n.id.as_str()) {
                    continue;
                }
                match n.shape {
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                            return Some(n.id.clone());
                        }
                    }
                    NodeShape::Circle => {
                        if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                            return Some(n.id.clone());
                        }
                    }
                }
            }
        }
        if o.select_edges {
            for e in self.edges.values().rev() {
                if !self.edge_selectable(e) {
                    continue;
                }
                if let Some(c) = self.edge_curve(e) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        return Some(e.id.clone());
                    }
                }
            }
        }
        None
    }

    fn resolve_node_hit_world(&self, point: Point) -> Option<String> {
        for n in self.nodes.values().rev() {
            if !n.visible {
                continue;
            }
            match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                        return Some(n.id.clone());
                    }
                }
                NodeShape::Circle => {
                    if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                        return Some(n.id.clone());
                    }
                }
            }
        }
        None
    }

    pub fn sync_descriptor(&mut self, desc: &SceneDescriptorJson) -> Result<(), String> {
        if matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
            self.interaction = Interaction::None;
            self.clear_link_gesture_events();
        }
        let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
        let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
        let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
        let want_wires: BTreeSet<_> = desc.wires.iter().map(|w| w.id.clone()).collect();
        self.edges.retain(|id, _| want_edges.contains(id));
        self.wires.retain(|id, _| want_wires.contains(id));
        self.handles.retain(|id, _| want_handles.contains(id));
        self.nodes.retain(|id, _| want_nodes.contains(id));
        for n in &desc.nodes {
            let shape = if n.shape.as_deref() == Some("rectangle") { NodeShape::Rectangle } else { NodeShape::Circle };
            let (radius, width, height) = match shape {
                NodeShape::Circle => (n.radius.unwrap_or(0.0), 0.0, 0.0),
                NodeShape::Rectangle => (0.0, n.width.unwrap_or(0.0), n.height.unwrap_or(0.0)),
            };
            let node_kind = n.node_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
            self.nodes.insert(
                n.id.clone(),
                NodeData {
                    id: n.id.clone(),
                    x: n.x,
                    y: n.y,
                    shape,
                    radius,
                    width,
                    height,
                    scale: n.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                    draggable: n.draggable.unwrap_or(true),
                    selected: n.selected.unwrap_or(false),
                    visible: n.visible.unwrap_or(true),
                    locked: n.locked.unwrap_or(false),
                    root: n.root.unwrap_or(false),
                    style: n.style.clone(),
                    text: n.text.clone(),
                    icon_kind: n.icon_kind.clone(),
                    node_kind,
                },
            );
        }
        for h in &desc.handles {
            let kind = h.handle_kind.as_deref().unwrap_or("").trim().to_string();
            let color_fill = match h.color.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                None => None,
                Some(s) => Some(Self::parse_css_color(s).ok_or_else(|| format!("invalid color on handle {}: {s:?}", h.id))?),
            };
            let icon_kind = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
            self.handles.insert(
                h.id.clone(),
                HandleData {
                    id: h.id.clone(),
                    node_id: h.node_id.clone(),
                    angle: h.angle,
                    radius: h.radius.unwrap_or(ui_styling::radii::HANDLE_DEFAULT),
                    scale: h.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                    selected: h.selected.unwrap_or(false),
                    visible: h.visible.unwrap_or(true),
                    locked: h.locked.unwrap_or(false),
                    style: h.style.clone(),
                    handle_kind: kind,
                    color_fill,
                    icon_kind,
                },
            );
        }
        for e in &desc.edges {
            let existed = self.edges.contains_key(&e.id);
            let edge_kind = e.edge_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
            let source_tip = Self::parse_catalog_tip_slot(e.source_tip.as_deref());
            let target_tip = Self::parse_catalog_tip_slot(e.target_tip.as_deref());
            self.edges.insert(
                e.id.clone(),
                EdgeData {
                    id: e.id.clone(),
                    source: e.source.clone(),
                    target: e.target.clone(),
                    selected: e.selected.unwrap_or(false),
                    visible: e.visible.unwrap_or(true),
                    locked: e.locked.unwrap_or(false),
                    style: e.style.clone(),
                    edge_kind,
                    source_tip,
                    target_tip,
                },
            );
            if !existed {
                self.push_event("edgeCreate", json!({ "id": e.id, "source": e.source, "target": e.target }));
            }
        }
        for w in &desc.wires {
            let target = w.target.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
            let (end_x, end_y) = match &target {
                Some(_) => (None, None),
                None => {
                    let x = match w.end_x {
                        Some(v) if v.is_finite() => Some(v),
                        _ => None,
                    };
                    let y = match w.end_y {
                        Some(v) if v.is_finite() => Some(v),
                        _ => None,
                    };
                    if x.is_none() || y.is_none() {
                        continue;
                    }
                    (x, y)
                }
            };
            let wire_kind = w
                .wire_kind
                .as_ref()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .or_else(|| self.handles.get(w.source.as_str()).map(|h| self.resolve_default_wire_kind_for_handle(h)))
                .unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string());
            self.wires.insert(
                w.id.clone(),
                WireData { id: w.id.clone(), source: w.source.clone(), target, end_x, end_y, selected: w.selected.unwrap_or(false), visible: w.visible.unwrap_or(true), locked: w.locked.unwrap_or(false), style: w.style.clone(), wire_kind },
            );
        }
        if !self.is_preselect_active() {
            let mut new_selection = BTreeSet::new();
            for n in &desc.nodes {
                if n.selected == Some(true) {
                    new_selection.insert(n.id.clone());
                }
            }
            for h in &desc.handles {
                if h.selected == Some(true) {
                    new_selection.insert(h.id.clone());
                }
            }
            for e in &desc.edges {
                if e.selected == Some(true) {
                    new_selection.insert(e.id.clone());
                }
            }
            for w in &desc.wires {
                if w.selected == Some(true) {
                    new_selection.insert(w.id.clone());
                }
            }
            let prev_sel = self.selection.clone();
            if prev_sel != new_selection {
                self.selection_exit_highlight.clear();
            }
            self.selection = new_selection;
            if prev_sel != self.selection {
                self.push_select_event();
            }
        }
        self.sync_selection_flags_to_objects();
        self.bump_content_scene_generation();
        Ok(())
    }

    /// @emoji 📍 Applies peer-pane node drags without a full descriptor re-sync.
    pub fn set_node_positions(&mut self, moves: &[(String, f64, f64)]) {
        let mut geometry_changed = false;
        for (id, x, y) in moves {
            if !x.is_finite() || !y.is_finite() {
                continue;
            }
            if let Some(node) = self.nodes.get_mut(id.as_str()) {
                if (node.x - *x).abs() > 1e-9 || (node.y - *y).abs() > 1e-9 {
                    node.x = *x;
                    node.y = *y;
                    geometry_changed = true;
                }
            }
        }
        if geometry_changed {
            self.bump_content_scene_generation();
        }
    }

    /// @emoji 📍 Parses `[{"id","x","y"},…]` and updates existing host nodes in place.
    pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), String> {
        #[derive(Deserialize)]
        struct NodePositionMoveJson {
            id: String,
            x: f64,
            y: f64,
        }
        let rows: Vec<NodePositionMoveJson> = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let moves: Vec<(String, f64, f64)> = rows.into_iter().map(|row| (row.id, row.x, row.y)).collect();
        self.set_node_positions(&moves);
        Ok(())
    }

    pub fn clear_scene(&mut self) {
        self.edges.clear();
        self.wires.clear();
        self.handles.clear();
        self.nodes.clear();
        self.selection.clear();
        self.preselect.clear();
        self.preselect_removed.clear();
        self.selection_exit_highlight.clear();
    }

    pub fn parse_fixture_v1(&mut self, raw: &serde_json::Value) -> bool {
        let f: FixtureV1Json = match serde_json::from_value(raw.clone()) {
            Ok(v) => v,
            Err(_) => return false,
        };
        self.port_mode = match f.schema.as_str() {
            "reasoning.mindmap.fixture/v1" => GraphPortMode::Normal,
            "puzzle.2d.fixture/v1" => GraphPortMode::Ported,
            _ => return false,
        };
        if !self.has_ports() {
            self.selection_options.select_handles = false;
        }
        self.set_camera(f.camera.x, f.camera.y, f.camera.zoom);
        self.clear_scene();
        let mut desc = SceneDescriptorJson::default();
        for entry in f.nodes {
            let Some(obj) = entry.as_object() else {
                return false;
            };
            let Some(id) = obj.get("id").and_then(|v| v.as_str()) else {
                return false;
            };
            let Some(x) = obj.get("x").and_then(|v| v.as_f64()) else {
                return false;
            };
            let Some(y) = obj.get("y").and_then(|v| v.as_f64()) else {
                return false;
            };
            if !x.is_finite() || !y.is_finite() {
                return false;
            }
            let text = obj.get("text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
            if self.has_ports() {
                let Some(handles_arr) = obj.get("handles").and_then(|v| v.as_array()) else {
                    return false;
                };
                let mut handles: Vec<HandleDescJson> = Vec::new();
                for h in handles_arr {
                    let Some(ho) = h.as_object() else {
                        return false;
                    };
                    let Some(hid) = ho.get("id").and_then(|v| v.as_str()) else {
                        return false;
                    };
                    let Some(angle) = ho.get("angle").and_then(|v| v.as_f64()) else {
                        return false;
                    };
                    if !angle.is_finite() {
                        return false;
                    }
                    let handle_kind = ho.get("handleKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from).unwrap_or_else(|| "port".into());
                    let handle_color = ho.get("color").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
                    let handle_icon_kind = ho.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    let handle_scale = ho.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                    handles.push(HandleDescJson {
                        id: hid.into(),
                        node_id: id.into(),
                        angle,
                        radius: None,
                        scale: handle_scale,
                        selected: None,
                        style: None,
                        handle_kind: Some(handle_kind),
                        color: handle_color,
                        icon_kind: handle_icon_kind,
                        user_data: None,
                        visible: board_json_visible_option(ho),
                        locked: board_json_locked_option(ho),
                    });
                }
                desc.handles.extend(handles);
            } else if obj.get("handles").is_some() {
                return false;
            }
            let shape_str = obj.get("shape").and_then(|v| v.as_str());
            let fixture_node_kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            let fixture_node_scale = obj.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
            if shape_str == Some("rectangle") {
                let Some(width) = obj.get("width").and_then(|v| v.as_f64()) else {
                    return false;
                };
                let Some(height) = obj.get("height").and_then(|v| v.as_f64()) else {
                    return false;
                };
                if width <= 0.0 || height <= 0.0 {
                    return false;
                }
                let root = obj.get("root").and_then(|v| v.as_bool());
                let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                desc.nodes.push(NodeDescJson {
                    id: id.into(),
                    x,
                    y,
                    draggable: None,
                    selected: None,
                    style: None,
                    text,
                    icon_kind,
                    node_kind: fixture_node_kind.clone(),
                    user_data: None,
                    visible: board_json_visible_option(obj),
                    locked: board_json_locked_option(obj),
                    root,
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(width),
                    height: Some(height),
                    scale: fixture_node_scale,
                });
            } else {
                let Some(radius) = obj.get("radius").and_then(|v| v.as_f64()) else {
                    return false;
                };
                if radius <= 0.0 {
                    return false;
                }
                let root = obj.get("root").and_then(|v| v.as_bool());
                let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                desc.nodes.push(NodeDescJson {
                    id: id.into(),
                    x,
                    y,
                    draggable: None,
                    selected: None,
                    style: None,
                    text,
                    icon_kind,
                    node_kind: fixture_node_kind.clone(),
                    user_data: None,
                    visible: board_json_visible_option(obj),
                    locked: board_json_locked_option(obj),
                    root,
                    shape: Some("circle".into()),
                    radius: Some(radius),
                    width: None,
                    height: None,
                    scale: fixture_node_scale,
                });
            }
        }
        for entry in f.edges {
            let Some(e) = entry.as_object() else {
                return false;
            };
            let Some(id) = e.get("id").and_then(|v| v.as_str()) else {
                return false;
            };
            let Some((source, target)) = fixture_edge_handle_ids_from_object(e) else {
                return false;
            };
            if !self.has_ports() {
                let node_ids: BTreeSet<&str> = desc.nodes.iter().map(|n| n.id.as_str()).collect();
                if !node_ids.contains(source) || !node_ids.contains(target) {
                    return false;
                }
            }
            let edge_kind = e.get("edgeKind").or_else(|| e.get("edge_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            let source_tip = e.get("sourceTip").or_else(|| e.get("source_tip")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            let target_tip = e.get("targetTip").or_else(|| e.get("target_tip")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            desc.edges.push(EdgeDescJson {
                id: id.into(),
                source: source.into(),
                target: target.into(),
                edge_kind,
                source_tip,
                target_tip,
                selected: None,
                style: None,
                user_data: None,
                visible: board_json_visible_option(e),
                locked: board_json_locked_option(e),
            });
        }
        if self.sync_descriptor(&desc).is_err() {
            return false;
        }
        true
    }

    fn drawable_cull_pad_world(&self) -> f64 {
        16.0 / self.camera.zoom.max(1e-9)
    }

    fn visible_world_box(&self, pad_world: f64) -> WorldBox {
        let corners = [
            self.screen_to_world(Point::new(0.0, 0.0)),
            self.screen_to_world(Point::new(self.width as f64, 0.0)),
            self.screen_to_world(Point::new(self.width as f64, self.height as f64)),
            self.screen_to_world(Point::new(0.0, self.height as f64)),
        ];
        let base = world_box_from_points(&corners).unwrap_or(WorldBox { min_x: self.camera.x - 1.0, min_y: self.camera.y - 1.0, max_x: self.camera.x + 1.0, max_y: self.camera.y + 1.0 });
        inflate_world_box(base, pad_world)
    }

    fn world_tile_screen_clip_rect(&self, ix: i32, iy: i32, tile: f64) -> Rect {
        let wx0 = ix as f64 * tile;
        let wy0 = iy as f64 * tile;
        let wx1 = wx0 + tile;
        let wy1 = wy0 + tile;
        let ps = [self.world_to_screen(Point::new(wx0, wy0)), self.world_to_screen(Point::new(wx1, wy0)), self.world_to_screen(Point::new(wx1, wy1)), self.world_to_screen(Point::new(wx0, wy1))];
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for p in ps {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        Rect::from_points(Point::new(min_x, min_y), Point::new(max_x, max_y)).inflate(1.0, 1.0)
    }

    fn handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
        let pos = self.handle_world_pos(h)?;
        let pad = self.drawable_cull_pad_world() + self.effective_handle_radius(h).max(1.0);
        Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
    }

    fn indirect_handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
        let pos = self.indirect_handle_world_pos(h)?;
        let pad = self.drawable_cull_pad_world() + self.indirect_handle_marker_radius_world(h).max(1.0);
        Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
    }

    fn edge_world_bounds_for_cull(&self, e: &EdgeData) -> Option<WorldBox> {
        let c = self.edge_curve(e)?;
        let axis = cubic_bezier_axis_bounds(c);
        let half_w_world = self.camera.zoom.max(0.75) / self.camera.zoom.max(1e-9);
        Some(inflate_world_box(axis, half_w_world + self.drawable_cull_pad_world()))
    }

    fn stroke_world_step_grid(&self, scene: &mut Scene, color: Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
        let step = world_step * self.camera.zoom;
        if step < min_step_screen {
            return;
        }
        let stroke = Stroke::new(stroke_px);
        let w = self.width as f64;
        let h = self.height as f64;
        let origin = self.world_to_screen(Point::new(0.0, 0.0));
        let x_off = ((origin.x % step) + step) % step;
        let y_off = ((origin.y % step) + step) % step;
        let mut p = infinite_cavas::vello::kurbo::BezPath::new();
        let mut x = x_off;
        while x <= w {
            p.move_to(Point::new(x, 0.0));
            p.line_to(Point::new(x, h));
            x += step;
        }
        let mut y = y_off;
        while y <= h {
            p.move_to(Point::new(0.0, y));
            p.line_to(Point::new(w, y));
            y += step;
        }
        scene.stroke(&stroke, Affine::IDENTITY, color, None, &p);
    }

    fn draw_space_point(&self, world: Point, world_space: bool) -> Point {
        if world_space {
            world
        } else {
            self.world_to_screen(world)
        }
    }

    fn draw_space_len(&self, len_world: f64, world_space: bool) -> f64 {
        if world_space {
            len_world.max(1e-9)
        } else {
            (len_world * self.camera.zoom).max(1.0)
        }
    }

    fn append_handle_marker(
        &self,
        scene: &mut Scene,
        h: &HandleData,
        center: Point,
        radius_world: f64,
        draw_icon: bool,
        style_kind: BoardElementStyleKind,
        paint_override: Option<(Color, Color, f64)>,
        world_space: bool,
        layer: NodeHandlePaintLayer,
        exterior_cap: bool,
    ) {
        let c = self.draw_space_point(center, world_space);
        let r = self.draw_space_len(radius_world, world_space);
        let (fill, stroke_c, stroke_px) =
            if let Some((f, s, sw)) = paint_override { (f, s, sw) } else { (self.resolve_handle_fill_color(h, &self.vello_theme, style_kind), self.resolve_handle_stroke_color(h, &self.vello_theme, style_kind), 2.0_f64) };
        let paint_fill = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill);
        let paint_stroke = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
        let paint_icons = draw_icon && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
        let outward = if exterior_cap { self.nodes.get(h.node_id.as_str()).and_then(|n| handle_outward_at_node_rim(center, Point::new(n.x, n.y), n.shape, n.radius, n.width, n.height)) } else { None };
        if paint_fill {
            if let Some(out) = outward {
                scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &handle_exterior_cap_fill_path(c, out, r));
            } else {
                scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &Circle::new(c, r));
            }
        }
        if paint_stroke {
            if let Some(out) = outward {
                scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &handle_exterior_cap_stroke_path(c, out, r));
            } else {
                scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &Circle::new(c, r));
            }
        }
        if paint_icons {
            if let Some(k) = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
                let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.vello_theme);
                if let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                    let fit_inset = 0.62;
                    let s = self.draw_space_len(radius_world, world_space) * fit_inset;
                    let cx = bx + bw * 0.5;
                    let cy = by + bh * 0.5;
                    let avail = 2.0 * s;
                    let scale = (avail / bw).min(avail / bh);
                    let aff = Affine::translate((c.x - scale * cx, c.y - scale * cy)) * Affine::scale(scale);
                    let r_clip = self.draw_space_len(radius_world, world_space) * 0.82;
                    let disc = Circle::new(c, r_clip);
                    scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
                    match &body {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(&ImageBrush::new((**img).clone()), aff);
                        }
                    }
                    scene.pop_layer();
                }
            }
        }
    }

    fn append_indirect_handle_ring(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, node_id: &str, chrome_pass: StyleChromePass, world_space: bool) {
        for h in self.handles.values() {
            if h.node_id != node_id || !self.handle_effectively_visible(h.id.as_str()) {
                continue;
            }
            if !self.indirect_ring_handle_eligible(h.id.as_str(), node_id) {
                continue;
            }
            if let Some(tb) = tile_filter {
                let Some(hb) = self.indirect_handle_world_bounds_cull(h) else { continue };
                if !world_boxes_overlap(*tb, hb) {
                    continue;
                }
            }
            let Some(wp) = self.indirect_handle_world_pos(h) else { continue };
            let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
            let stroke_px = 2.0_f64;
            let paint_override = if matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { Some((self.vello_theme.indirect_handle_fill, self.vello_theme.indirect_handle_stroke, stroke_px)) } else { None };
            self.append_handle_marker(scene, h, wp, self.indirect_handle_marker_radius_world(h), false, style_kind, paint_override, world_space, NodeHandlePaintLayer::Full, false);
        }
    }

    /// @emoji 📏 Screen-pixel edge stroke width (world-clip tiles and post-cache overlay).
    fn edge_screen_stroke_width_px(&self, lod: BoardDrawLod) -> f64 {
        match lod {
            BoardDrawLod::Minimap => ui_styling::strokes::EDGE_MINIMAP,
            BoardDrawLod::Overview | BoardDrawLod::Compact => (ui_styling::strokes::EDGE_OVERVIEW).max(ui_styling::strokes::EDGE_BASE * self.camera.zoom),
            _ => 2.0 * self.camera.zoom.max(0.75),
        }
    }

    /// @emoji 📏 Edge stroke in world units so {@link BoardHost.camera_content_affine} yields ~{@link Self::edge_screen_stroke_width_px}.
    fn edge_world_stroke_width(&self, lod: BoardDrawLod) -> f64 {
        let screen_px = self.edge_screen_stroke_width_px(lod);
        let z = self.camera.zoom.max(1e-9);
        (screen_px / z).max(1e-3)
    }

    fn append_nodes_handles_edges(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool) {
        self.append_nodes_and_handles(scene, tile_filter, lod, world_space, None, StyleChromePass::CachedBase, NodeHandlePaintLayer::Full);
        if !world_space {
            self.append_edges_wires_and_link(scene, tile_filter, lod, world_space, None, None);
        }
    }

    fn paint_node_geometry(&self, scene: &mut Scene, n: &NodeData, lod: BoardDrawLod, world_space: bool, layer: NodeHandlePaintLayer, chrome_pass: StyleChromePass, link_compat: bool) {
        let draw_node_icons = matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro);
        let resolved_style_kind = self.resolve_node_style_kind(n, chrome_pass);
        let style_kind = if link_compat && matches!(resolved_style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { BoardElementStyleKind::Highlighted } else { resolved_style_kind };
        let draw_node_stroke = lod != BoardDrawLod::Minimap || !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral);
        let stroke_c = Self::node_stroke_for_style(&self.vello_theme, style_kind);
        let fill = if lod == BoardDrawLod::Minimap { stroke_c } else { self.resolve_node_fill_color(n, &self.vello_theme, style_kind) };
        let sw = 2.0_f64;
        let paint_fill = if lod == BoardDrawLod::Minimap {
            matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill)
        } else {
            matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill) && !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral)
        };
        let paint_stroke = draw_node_stroke && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
        let paint_icons = draw_node_icons && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
        match n.shape {
            NodeShape::Circle => {
                let c = self.draw_space_point(Point::new(n.x, n.y), world_space);
                let r = self.draw_space_len(self.scaled_node_radius(n), world_space);
                let circle = Circle::new(c, r);
                if paint_fill {
                    scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
                }
                if paint_stroke {
                    scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
                }
                if paint_icons {
                    self.paint_node_icon(scene, n, world_space, style_kind, stroke_c, fill, Some(&circle), None);
                }
            }
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                let p0 = self.draw_space_point(Point::new(n.x - hw, n.y - hh), world_space);
                let p1 = self.draw_space_point(Point::new(n.x + hw, n.y + hh), world_space);
                let rect = Rect::from_points(p0, p1);
                if paint_fill {
                    scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &rect);
                }
                if paint_stroke {
                    scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &rect);
                }
                if paint_icons {
                    self.paint_node_icon(scene, n, world_space, style_kind, stroke_c, fill, None, Some(rect));
                }
            }
        }
    }

    fn paint_node_icon(&self, scene: &mut Scene, n: &NodeData, world_space: bool, style_kind: BoardElementStyleKind, stroke_c: Color, fill: Color, circle_clip: Option<&Circle>, rect_clip: Option<Rect>) {
        if let Some(k) = n.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
            let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.vello_theme);
            if let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
                let fit_inset = ui_styling::metrics::icon::FIT_INSET;
                let (sx_half, sy_half) = match n.shape {
                    NodeShape::Circle => {
                        let s = self.draw_space_len(self.scaled_node_radius(n), world_space) * fit_inset;
                        (s, s)
                    }
                    NodeShape::Rectangle => (self.draw_space_len(self.scaled_node_width(n), world_space) * fit_inset * 0.5, self.draw_space_len(self.scaled_node_height(n), world_space) * fit_inset * 0.5),
                };
                let center = self.draw_space_point(Point::new(n.x, n.y), world_space);
                let cx = bx + bw * 0.5;
                let cy = by + bh * 0.5;
                let avail_w = 2.0 * sx_half;
                let avail_h = 2.0 * sy_half;
                let scale = (avail_w / bw).min(avail_h / bh);
                let aff = Affine::translate((center.x - scale * cx, center.y - scale * cy)) * Affine::scale(scale);
                match n.shape {
                    NodeShape::Circle => {
                        let r_clip = self.draw_space_len(self.scaled_node_radius(n), world_space) * clip_inset;
                        let disc = circle_clip.copied().unwrap_or_else(|| Circle::new(center, r_clip));
                        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
                        match &body {
                            CachedIconBody::Vector(icon_scene) => {
                                scene.append(icon_scene, Some(aff));
                            }
                            CachedIconBody::Raster(img) => {
                                scene.draw_image(&ImageBrush::new((**img).clone()), aff);
                            }
                        }
                        scene.pop_layer();
                    }
                    NodeShape::Rectangle => {
                        let hw = self.draw_space_len(self.scaled_node_width(n), world_space) * clip_inset * 0.5;
                        let hh = self.draw_space_len(self.scaled_node_height(n), world_space) * clip_inset * 0.5;
                        let clip_r = rect_clip.unwrap_or_else(|| Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh)));
                        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &clip_r);
                        match &body {
                            CachedIconBody::Vector(icon_scene) => {
                                scene.append(icon_scene, Some(aff));
                            }
                            CachedIconBody::Raster(img) => {
                                scene.draw_image(&ImageBrush::new((**img).clone()), aff);
                            }
                        }
                        scene.pop_layer();
                    }
                }
            }
        }
    }

    fn append_nodes_and_handles_with_overlay_chrome(
        &self,
        scene: &mut Scene,
        tile_filter: Option<&WorldBox>,
        lod: BoardDrawLod,
        world_space: bool,
        only_ids: Option<&BTreeSet<String>>,
        overlay_ids: &BTreeSet<String>,
        layer: NodeHandlePaintLayer,
    ) {
        let pad = self.drawable_cull_pad_world();
        let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
        let draw_handle_icons = lod == BoardDrawLod::Micro;
        let link_source = self.active_link_source_handle_id().map(str::to_string);
        let link_compat_nodes: std::collections::BTreeSet<String> = link_source.as_ref().map(|s| self.link_drag_compatible_target_node_ids(s).into_iter().collect()).unwrap_or_default();
        for h in self.handles.values() {
            if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&h.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
                if !world_boxes_overlap(*tb, hb) {
                    continue;
                }
            }
            let Some(wp) = self.handle_world_pos(h) else { continue };
            let style_kind = self.resolve_handle_style_kind(h, self.chrome_pass_for_entity(&h.id, overlay_ids));
            self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None, world_space, layer, true);
        }
        let paint_node = |scene: &mut Scene, n: &NodeData, chrome_pass: StyleChromePass| {
            self.paint_node_geometry(scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
        };
        for n in self.nodes.values() {
            if !n.visible {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&n.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                    continue;
                }
            }
            if overlay_ids.contains(&n.id) {
                continue;
            }
            paint_node(scene, n, StyleChromePass::CachedBase);
        }
        for n in self.nodes.values() {
            if !n.visible {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&n.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                    continue;
                }
            }
            if !overlay_ids.contains(&n.id) {
                continue;
            }
            paint_node(scene, n, StyleChromePass::InteractionOverlay);
        }
    }

    fn append_nodes_and_handles(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, chrome_pass: StyleChromePass, layer: NodeHandlePaintLayer) {
        let pad = self.drawable_cull_pad_world();
        let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
        let draw_handle_icons = lod == BoardDrawLod::Micro;
        let link_source = self.active_link_source_handle_id().map(str::to_string);
        let link_compat_nodes: std::collections::BTreeSet<String> = link_source.as_ref().map(|s| self.link_drag_compatible_target_node_ids(s).into_iter().collect()).unwrap_or_default();
        for h in self.handles.values() {
            if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&h.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
                if !world_boxes_overlap(*tb, hb) {
                    continue;
                }
            }
            let Some(wp) = self.handle_world_pos(h) else { continue };
            let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
            self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None, world_space, layer, true);
        }
        for n in self.nodes.values() {
            if !n.visible {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&n.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                    continue;
                }
            }
            self.paint_node_geometry(scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
        }
    }

    fn append_edges_wires_and_link(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, overlay_ids: Option<&BTreeSet<String>>) {
        let edge_sw = if world_space { self.edge_world_stroke_width(lod) } else { self.edge_screen_stroke_width_px(lod) };
        for e in self.edges.values() {
            if !self.edge_effectively_visible(e) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&e.id) {
                    continue;
                }
            }
            if let Some(tb) = tile_filter {
                let Some(eb) = self.edge_world_bounds_for_cull(e) else { continue };
                if !world_boxes_overlap(*tb, eb) {
                    continue;
                }
            }
            if let Some(c) = self.edge_curve(e) {
                let p0 = self.draw_space_point(c.p0, world_space);
                let p1 = self.draw_space_point(c.p1, world_space);
                let p2 = self.draw_space_point(c.p2, world_space);
                let p3 = self.draw_space_point(c.p3, world_space);
                let curve = CubicBez::new(p0, p1, p2, p3);
                let chrome_pass = overlay_ids.map(|ids| self.chrome_pass_for_entity(&e.id, ids)).unwrap_or(StyleChromePass::CachedBase);
                let (stroke_color, edge_stroke, stroke_w) = self.resolve_edge_stroke_paint(e, chrome_pass, lod, edge_sw);
                scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
                let (source_tip, target_tip) = self.resolve_edge_tips(e);
                Self::append_edge_tips_on_curve(scene, &curve, stroke_color, stroke_w, source_tip, target_tip);
            }
        }
        let wire_sw = 2.25_f64;
        let wire_stroke = Stroke::new(wire_sw);
        for w in self.wires.values() {
            if !self.wire_effectively_visible(w) {
                continue;
            }
            if let Some(ids) = only_ids {
                if !ids.contains(&w.id) {
                    continue;
                }
            }
            if let Some(c) = self.wire_curve(w) {
                let p0 = self.draw_space_point(c.p0, world_space);
                let p1 = self.draw_space_point(c.p1, world_space);
                let p2 = self.draw_space_point(c.p2, world_space);
                let p3 = self.draw_space_point(c.p3, world_space);
                let curve = CubicBez::new(p0, p1, p2, p3);
                let chrome_pass = overlay_ids.map(|ids| self.chrome_pass_for_entity(&w.id, ids)).unwrap_or(StyleChromePass::CachedBase);
                let wc = Self::wire_stroke_for_style(&self.vello_theme, self.resolve_wire_style_kind(w, chrome_pass));
                scene.stroke(&wire_stroke, Affine::IDENTITY, wc, None, &curve);
            }
        }
        let link_wire_sw = 2.85_f64;
        let link_wire_stroke = Stroke::new(link_wire_sw);
        let link_wire_color = self.vello_theme.node_stroke;
        if let Some(c) = self.active_link_wire_curve() {
            let p0 = self.draw_space_point(c.p0, world_space);
            let p1 = self.draw_space_point(c.p1, world_space);
            let p2 = self.draw_space_point(c.p2, world_space);
            let p3 = self.draw_space_point(c.p3, world_space);
            let curve = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
        }
    }

    fn append_cached_world_content(&self, scene: &mut Scene, lod: BoardDrawLod) {
        let gen = self.content_scene_generation;
        let cam_aff = self.camera_content_affine();
        let overlay_ids = self.interaction_overlay_entity_ids();
        let mut fill_layer = Scene::new();
        self.append_nodes_and_handles_with_overlay_chrome(&mut fill_layer, None, lod, true, None, &overlay_ids, NodeHandlePaintLayer::Fill);
        scene.append(&fill_layer, Some(cam_aff));
        let mut cache = self.world_content_cache.borrow_mut();
        let needs_rebuild = cache.as_ref().map(|c| c.0 != gen || c.1 != lod).unwrap_or(true);
        if needs_rebuild {
            let mut content = Scene::new();
            self.append_nodes_and_handles(&mut content, None, lod, true, None, StyleChromePass::CachedBase, NodeHandlePaintLayer::Icons);
            *cache = Some((gen, lod, content));
        }
        if let Some(cached) = cache.as_ref() {
            scene.append(&cached.2, Some(cam_aff));
        }
        let edges_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if edges_in_world_space {
            let mut edge_layer = Scene::new();
            self.append_edges_wires_and_link(&mut edge_layer, None, lod, true, None, Some(&overlay_ids));
            scene.append(&edge_layer, Some(cam_aff));
        } else {
            self.append_edges_wires_and_link(scene, None, lod, false, None, Some(&overlay_ids));
        }
        let mut stroke_layer = Scene::new();
        self.append_nodes_and_handles_with_overlay_chrome(&mut stroke_layer, None, lod, false, None, &overlay_ids, NodeHandlePaintLayer::Stroke);
        scene.append(&stroke_layer, None);
        if let Some(c) = self.active_link_wire_curve() {
            let link_wire_stroke = Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT);
            let link_wire_color = self.vello_theme.node_stroke;
            let p0 = self.draw_space_point(c.p0, false);
            let p1 = self.draw_space_point(c.p1, false);
            let p2 = self.draw_space_point(c.p2, false);
            let p3 = self.draw_space_point(c.p3, false);
            let curve = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
        }
        if self.has_ports() {
            if let Some(node_id) = self.indirect_ring_node_id(lod) {
                self.append_indirect_handle_ring(scene, None, &node_id, StyleChromePass::CachedBase, false);
            }
        }
        let previews_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
        if previews_in_world_space {
            let mut preview_layer = Scene::new();
            if self.fixture_drop_preview.is_some() {
                self.append_fixture_drop_preview_paint(&mut preview_layer, lod, true);
            }
            if self.active_tool == ActiveTool::Brush || self.brush_preview.is_some() {
                self.append_brush_preview_paint(&mut preview_layer, lod, true);
            }
            scene.append(&preview_layer, Some(cam_aff));
        } else {
            if self.fixture_drop_preview.is_some() {
                self.append_fixture_drop_preview_paint(scene, lod, false);
            }
            if self.active_tool == ActiveTool::Brush || self.brush_preview.is_some() {
                self.append_brush_preview_paint(scene, lod, false);
            }
        }
    }

    pub fn set_wheel_zoom_active(&mut self, active: bool) {
        if active && !self.wheel_zoom_active {
            self.wheel_zoom_render_lod = Some(self.current_draw_lod());
        }
        if !active {
            self.wheel_zoom_render_lod = None;
        }
        self.wheel_zoom_active = active;
    }

    pub fn build_vector_scene(&self) -> Scene {
        let mut inner = Scene::new();
        let lod = self.draw_lod_for_frame();
        if !self.wheel_zoom_active {
            let grid_color = self.vello_theme.grid_minor_stroke;
            if lod != BoardDrawLod::Minimap {
                self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
                match lod {
                    BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro => {
                        self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
                    }
                    BoardDrawLod::Minimap | BoardDrawLod::Overview | BoardDrawLod::Compact => {}
                }
                if matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
                }
                if lod == BoardDrawLod::Micro {
                    self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
                }
            }
        }
        if let Some(ref pts) = self.selection_screen_preview {
            if pts.len() >= 2 {
                let mut path = infinite_cavas::vello::kurbo::BezPath::new();
                path.move_to(pts[0]);
                for p in pts.iter().skip(1) {
                    path.line_to(*p);
                }
                path.close_path();
                inner.fill(Fill::NonZero, Affine::IDENTITY, self.vello_theme.selection_preview_fill, None, &path);
                let mut preview_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
                if self.selection_preview_crossing {
                    preview_stroke.dash_pattern = vec![5.0, 4.0].into();
                }
                inner.stroke(&preview_stroke, Affine::IDENTITY, self.vello_theme.selection_preview_stroke, None, &path);
            }
        }
        self.append_cached_world_content(&mut inner, lod);
        let scale = self.dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            inner
        } else {
            let mut scene = Scene::new();
            scene.append(&inner, Some(Affine::scale(scale)));
            scene
        }
    }

    pub fn encoded_scene_hint(&self) -> usize {
        let s = self.build_vector_scene();
        s.encoding().path_tags.len()
    }

    pub fn update_hover_from_world(&mut self, world: Point) {
        let next = self.resolve_hover_world(world);
        self.set_hovered_id(next);
    }

    pub fn set_hovered_id(&mut self, id: Option<String>) {
        let event_kind = id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
        if self.hovered_id == id && self.hovered_kind.is_none() {
            return;
        }
        self.bump_content_scene_generation();
        self.hovered_id = id.clone();
        self.hovered_kind = None;
        self.push_event(
            "hover",
            json!({
                "id": id,
                "kind": event_kind.as_ref().map(|(domain, kind_id)| json!({ "domain": domain, "kindId": kind_id })),
            }),
        );
    }

    /// @emoji 🖱️ Sets transitive kind hover from a catalog row (clears direct `hovered_id`).
    pub fn set_hovered_kind(&mut self, domain: Option<String>, kind_id: Option<String>) {
        let next_kind = domain.zip(kind_id);
        if self.hovered_id.is_none() && self.hovered_kind == next_kind {
            return;
        }
        self.bump_content_scene_generation();
        self.hovered_id = None;
        self.hovered_kind = next_kind.clone();
        self.push_event(
            "hover",
            json!({
                "id": null,
                "kind": next_kind.as_ref().map(|(domain, kind_id)| json!({ "domain": domain, "kindId": kind_id })),
            }),
        );
    }

    /// @emoji 🔇 Updates hover chrome without emitting `hover` (controlled React sync).
    pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
        if self.hovered_id == id && self.hovered_kind.is_none() {
            return;
        }
        self.bump_content_scene_generation();
        self.hovered_id = id;
        self.hovered_kind = None;
    }

    /// @emoji 🔇 Mirrors controlled kind hover without emitting `hover`.
    pub fn set_hovered_kind_silent(&mut self, domain: Option<String>, kind_id: Option<String>) {
        let next_kind = domain.zip(kind_id);
        if self.hovered_id.is_none() && self.hovered_kind == next_kind {
            return;
        }
        self.hovered_id = None;
        self.hovered_kind = next_kind;
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let viewport = self.viewport();
        infinite_cavas::camera::wheel_screen(&mut self.camera, &viewport, sx, sy, delta_y);
        self.set_camera_silent(self.camera.x, self.camera.y, self.camera.zoom);
    }

    pub fn delete_selection(&mut self) {
        if !self.has_ports() {
            let edge_ids: Vec<_> = self.selection.iter().filter(|id| self.edges.contains_key(*id)).cloned().collect();
            for id in &edge_ids {
                self.edges.remove(id);
                self.push_event("edgeDelete", json!({ "id": id }));
            }
            let node_ids: Vec<_> = self.selection.iter().filter(|id| self.nodes.contains_key(*id)).cloned().collect();
            for nid in &node_ids {
                let eids: Vec<_> = self.edges.iter().filter(|(_, e)| e.source == *nid || e.target == *nid).map(|(k, _)| k.clone()).collect();
                for eid in eids {
                    self.edges.remove(&eid);
                    self.selection.remove(&eid);
                    self.push_event("edgeDelete", json!({ "id": eid }));
                }
                self.nodes.remove(nid);
                self.push_event("nodeDelete", json!({ "id": nid }));
            }
            for id in edge_ids {
                self.selection.remove(&id);
            }
            for id in node_ids {
                self.selection.remove(&id);
            }
            self.selection_exit_highlight.clear();
            self.sync_selection_flags_to_objects();
            self.push_select_event();
            return;
        }
        let edge_ids: Vec<_> = self.selection.iter().filter(|id| self.edges.contains_key(*id)).cloned().collect();
        for id in &edge_ids {
            self.edges.remove(id);
            self.push_event("edgeDelete", json!({ "id": id }));
        }
        let mut node_ids: BTreeSet<String> = self.selection.iter().filter(|id| self.nodes.contains_key(*id)).cloned().collect();
        for id in self.selection.iter() {
            if let Some(handle) = self.handles.get(id) {
                node_ids.insert(handle.node_id.clone());
            }
        }
        let node_ids: Vec<_> = node_ids.into_iter().collect();
        for nid in &node_ids {
            let handle_ids: Vec<_> = self.handles.iter().filter(|(_, h)| &h.node_id == nid).map(|(k, _)| k.clone()).collect();
            for hid in handle_ids {
                let wids: Vec<_> = self.wires.iter().filter(|(_, w)| w.source == *hid || w.target.as_ref() == Some(&hid)).map(|(k, _)| k.clone()).collect();
                for wid in &wids {
                    self.wires.remove(wid);
                    self.selection.remove(wid);
                }
                let eids: Vec<_> = self.edges.iter().filter(|(_, e)| e.source == hid || e.target == hid).map(|(k, _)| k.clone()).collect();
                for eid in eids {
                    self.edges.remove(&eid);
                    self.selection.remove(&eid);
                    self.push_event("edgeDelete", json!({ "id": eid }));
                }
                self.handles.remove(&hid);
                self.selection.remove(&hid);
            }
            self.nodes.remove(nid);
            self.push_event("nodeDelete", json!({ "id": nid }));
        }
        for id in edge_ids {
            self.selection.remove(&id);
        }
        for id in node_ids {
            self.selection.remove(&id);
        }
        self.selection_exit_highlight.clear();
        self.sync_selection_flags_to_objects();
        self.push_select_event();
    }

    fn link_snap_drag_tolerance_screen(&self, h: &HandleData) -> f64 {
        let z = self.camera.zoom.max(1e-9);
        HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX + self.effective_handle_radius(h) * z
    }

    fn link_snap_commit_proximity_ok(&self, target_handle_id: &str, world: Point) -> bool {
        let Some(h) = self.handles.get(target_handle_id) else {
            return false;
        };
        if !self.handle_selectable(target_handle_id) {
            return false;
        }
        let Some(pw) = self.handle_world_pos(h) else {
            return false;
        };
        let z = self.camera.zoom.max(1e-9);
        let d_screen = distance_between(self.world_to_screen(world), self.world_to_screen(pw));
        let tol_commit = HANDLE_HIT_TOLERANCE_PX + LINK_COMMIT_SNAP_TIGHT_PX + self.effective_handle_radius(h) * z;
        d_screen <= tol_commit
    }

    /// @emoji 🔗 True when any edge uses this handle as `source` or `target` (handle already participates in a link).
    fn handle_has_incident_edge(&self, handle_id: &str) -> bool {
        self.edges.values().any(|e| e.source == handle_id || e.target == handle_id)
    }

    fn node_has_any_incident_edge(&self, node_id: &str) -> bool {
        self.handles.values().filter(|h| h.node_id == node_id).any(|h| self.handle_has_incident_edge(h.id.as_str()))
    }

    fn lod_allows_node_proximity_connect(&self) -> bool {
        matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro)
    }

    /// @emoji 🧲 While dragging a node with no incident edges, overlapping bounds pick the nearest compatible free handle pair.
    fn node_drag_proximity_handle_pair(&self, moving_node_id: &str) -> Option<(String, String)> {
        if !self.lod_allows_node_proximity_connect() {
            return None;
        }
        if !self.node_selectable(moving_node_id) {
            return None;
        }
        if self.node_has_any_incident_edge(moving_node_id) {
            return None;
        }
        let moving = self.nodes.get(moving_node_id)?;
        let moving_bounds = self.node_world_bounds(moving, 0.0);
        let mut best: Option<(f64, String, String)> = None;
        for (target_id, target) in &self.nodes {
            if target_id == moving_node_id || !self.node_selectable(target_id.as_str()) {
                continue;
            }
            let target_bounds = self.node_world_bounds(target, 0.0);
            if !world_boxes_overlap(moving_bounds, target_bounds) {
                continue;
            }
            let moving_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == moving_node_id && self.handle_selectable(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
            let target_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == target_id.as_str() && self.handle_selectable(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
            for (src_id, src_h) in &moving_handles {
                let Some(src_pos) = self.handle_world_pos(src_h) else {
                    continue;
                };
                for (tgt_id, tgt_h) in &target_handles {
                    let Some(tgt_pos) = self.handle_world_pos(tgt_h) else {
                        continue;
                    };
                    let d = distance_between(src_pos, tgt_pos);
                    let pair = if self.handles_link_compatible_for_drag(src_h, tgt_h) {
                        Some(((*src_id).clone(), (*tgt_id).clone()))
                    } else if self.handles_link_compatible_for_drag(tgt_h, src_h) {
                        Some(((*tgt_id).clone(), (*src_id).clone()))
                    } else {
                        None
                    };
                    if let Some((s, t)) = pair {
                        if best.as_ref().map(|(bd, _, _)| d < *bd).unwrap_or(true) {
                            best = Some((d, s.to_string(), t.to_string()));
                        }
                    }
                }
            }
        }
        best.map(|(_, s, t)| (s, t))
    }

    fn node_effectively_visible(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible)
    }

    fn node_selectable(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible && !n.locked)
    }

    fn handle_selectable(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && !h.locked && self.node_selectable(h.node_id.as_str()))
    }

    fn edge_selectable(&self, edge: &EdgeData) -> bool {
        if !edge.visible || edge.locked {
            return false;
        }
        if !self.has_ports() {
            return self.node_selectable(edge.source.as_str()) && self.node_selectable(edge.target.as_str());
        }
        self.handle_selectable(edge.source.as_str()) && self.handle_selectable(edge.target.as_str())
    }

    fn wire_selectable(&self, wire: &WireData) -> bool {
        if !wire.visible || wire.locked {
            return false;
        }
        if !self.handle_selectable(wire.source.as_str()) {
            return false;
        }
        wire.target.as_ref().map(|id| self.handle_selectable(id.as_str())).unwrap_or(true)
    }

    fn entity_selectable_by_id(&self, id: &str) -> bool {
        if let Some(n) = self.nodes.get(id) {
            return self.node_selectable(n.id.as_str());
        }
        if let Some(h) = self.handles.get(id) {
            return self.handle_selectable(h.id.as_str());
        }
        if let Some(e) = self.edges.get(id) {
            return self.edge_selectable(e);
        }
        if let Some(w) = self.wires.get(id) {
            return self.wire_selectable(w);
        }
        false
    }

    fn handle_effectively_visible(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && self.node_effectively_visible(h.node_id.as_str()))
    }

    fn edge_effectively_visible(&self, edge: &EdgeData) -> bool {
        if !self.has_ports() {
            return edge.visible && self.node_effectively_visible(edge.source.as_str()) && self.node_effectively_visible(edge.target.as_str());
        }
        edge.visible && self.handle_effectively_visible(edge.source.as_str()) && self.handle_effectively_visible(edge.target.as_str())
    }

    fn wire_effectively_visible(&self, wire: &WireData) -> bool {
        wire.visible && self.handle_effectively_visible(wire.source.as_str()) && wire.target.as_ref().map(|id| self.handle_effectively_visible(id.as_str())).unwrap_or(true)
    }

    /// @emoji 💫 True when the handle may be drawn or hit-tested on the indirect-connect ghost ring (`overview`/`normal` LOD).
    fn handle_eligible_indirect_connect_ring(&self, handle_id: &str) -> bool {
        self.handle_selectable(handle_id) && !self.handle_has_incident_edge(handle_id)
    }

    /// @emoji 📍 Drag-phase link snap tests **screen px** to the handle anchor so detail/micro zoom keeps a stable hit halo; pointer-up re-checks with `link_snap_commit_proximity_ok` before `proximityConnect`.
    fn nearest_link_snap_handle_world(&self, source_handle_id: &str, world: Point) -> Option<String> {
        if matches!(self.current_draw_lod(), BoardDrawLod::Minimap) {
            return None;
        }
        let source_handle = self.handles.get(source_handle_id)?;
        if !self.handle_selectable(source_handle_id) {
            return None;
        }
        let source_node_id = source_handle.node_id.as_str();
        let p_scr = self.world_to_screen(world);
        let mut best: Option<(f64, String)> = None;
        for (id, h) in &self.handles {
            if id == source_handle_id || !self.handle_selectable(id.as_str()) {
                continue;
            }
            if self.handle_has_incident_edge(id.as_str()) {
                continue;
            }
            if h.node_id == source_node_id {
                continue;
            }
            if !self.handles_link_compatible_for_drag(source_handle, h) {
                continue;
            }
            let pw = self.handle_world_pos(h)?;
            let h_scr = self.world_to_screen(pw);
            let d_screen = distance_between(p_scr, h_scr);
            let tol_screen = self.link_snap_drag_tolerance_screen(h);
            if d_screen <= tol_screen && best.as_ref().map(|(bd, _)| d_screen < *bd).unwrap_or(true) {
                best = Some((d_screen, id.clone()));
            }
        }
        best.map(|(_, id)| id)
    }

    fn try_commit_link_edge(&mut self, source_handle_id: &str, target_handle_id: &str, also_emit: Option<&'static str>) -> bool {
        if source_handle_id == target_handle_id {
            return false;
        }
        if !self.handle_selectable(source_handle_id) || !self.handle_selectable(target_handle_id) {
            return false;
        }
        let Some(source_row) = self.handles.get(source_handle_id) else {
            return false;
        };
        let Some(target_row) = self.handles.get(target_handle_id) else {
            return false;
        };
        if source_row.node_id == target_row.node_id {
            return false;
        }
        if !self.handles_link_compatible_for_drag(source_row, target_row) {
            return false;
        }
        if self.handle_has_incident_edge(source_handle_id) || self.handle_has_incident_edge(target_handle_id) {
            return false;
        }
        for e in self.edges.values() {
            if e.source == source_handle_id && e.target == target_handle_id {
                return false;
            }
        }
        let mut n = self.edges.len().saturating_add(1);
        let id = loop {
            let candidate = format!("edge-link-{n}");
            if !self.edges.contains_key(&candidate) {
                break candidate;
            }
            n = n.saturating_add(1);
        };
        let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
        self.edges
            .insert(id.clone(), EdgeData { id: id.clone(), source: source_handle_id.to_string(), target: target_handle_id.to_string(), selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None });
        self.push_event("edgeCreate", json!({ "id": id, "source": source_handle_id, "target": target_handle_id }));
        if let Some(name) = also_emit {
            self.push_event(name, json!({ "id": id, "source": source_handle_id, "target": target_handle_id }));
        }
        true
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
        self.set_selection_screen_preview(None);
        let screen = Point::new(sx, sy);
        let world = self.screen_to_world(screen);
        if self.active_tool == ActiveTool::Brush {
            if button == 1 {
                self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
            }
            return;
        }
        let hit = self.resolve_hit_world(world).or_else(|| self.resolve_overview_free_link_handle_pointer_world(world));
        if let Interaction::LinkTargetNode { source_id, target_node_id } = self.interaction.clone() {
            self.interaction = Interaction::None;
            self.clear_link_gesture_events();
            if button == 0 {
                if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                    if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                        self.try_commit_link_edge(&source_id, &th, Some("indirectConnect"));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| self.handles.get(*id).is_some_and(|h| h.node_id == target_node_id) && self.handle_eligible_link_target_ring(id.as_str(), source_id.as_str())) {
                    self.try_commit_link_edge(&source_id, hid, Some("indirectConnect"));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = self.interaction.clone() {
            if button == 0 {
                if let Some(target_node_id) = ring_node_id {
                    if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                        if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                            self.interaction = Interaction::None;
                            self.clear_link_gesture_events();
                            self.try_commit_link_edge(&source_id, &th, Some("indirectConnect"));
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                }
                if let Some(hid) = hit.as_ref().filter(|id| ring_handle_ids.iter().any(|rh| rh == *id)) {
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                    self.try_commit_link_edge(&source_id, hid, Some("indirectConnect"));
                    self.update_hover_from_world(world);
                    return;
                }
            }
            self.update_hover_from_world(world);
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        let pick_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        if button == 0 && !merge_from_modifiers && self.try_begin_bounded_selection_drag_at(world) {
            return;
        }
        if button == 1 {
            self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
            return;
        }
        if let Some(ref hid) = hit {
            if let Some(node) = self.nodes.get(hid) {
                if node.draggable && !node.locked {
                    let nid = hid.clone();
                    let nx = node.x;
                    let ny = node.y;
                    let members_before: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                    let drag_group_before = members_before.contains(&nid) && members_before.len() > 1;
                    let force_pick_merge = (pick_mode == "replace" && !drag_group_before) || pick_mode == "subtractive" || (pick_mode == "invertive" && merge_from_modifiers);
                    if !drag_group_before || force_pick_merge {
                        let next = merge_pick_into_selection(&self.selection, &nid, pick_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                        self.set_selection_ids_gestured(&ids, gesture);
                    }
                    let members: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                    let drag_group = members.contains(&nid) && members.len() > 1;
                    let mut start_positions = BTreeMap::new();
                    for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&nid) } {
                        if let Some(n) = self.nodes.get(id) {
                            start_positions.insert(id.clone(), (n.x, n.y));
                        }
                    }
                    self.interaction = Interaction::DragNodes { primary_id: nid, offset: world - Point::new(nx, ny), start_positions, proximity_pair: None };
                    self.set_hovered_id(hit);
                    return;
                }
            }
        }
        if let Some(ref hid) = hit {
            if button == 0 && self.handles.contains_key(hid) && !self.handle_has_incident_edge(hid.as_str()) {
                let next = merge_pick_into_selection(&self.selection, hid, pick_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                self.set_selection_ids_gestured(&ids, gesture);
                self.interaction = Interaction::LinkAtSourceHandle { source_id: hid.clone(), start_screen: screen };
                self.set_hovered_id(Some(hid.clone()));
                return;
            }
        }
        if hit.is_none() && button == 0 {
            self.interaction = Interaction::SelectionPending { initial_ids: self.selection.clone(), start: world, start_screen: screen };
            self.set_hovered_id(None);
            return;
        }
        self.interaction = Interaction::None;
        if let Some(id) = hit {
            let next = merge_pick_into_selection(&self.selection, &id, pick_mode.as_str());
            let ids: Vec<_> = next.iter().cloned().collect();
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            self.set_selection_ids_gestured(&ids, gesture);
            self.set_hovered_id(Some(id));
        } else {
            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
            self.set_selection_ids_gestured(&[], gesture);
            self.set_hovered_id(None);
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let screen = Point::new(sx, sy);
        let world = self.screen_to_world(screen);
        if self.active_tool == ActiveTool::Brush {
            self.brush_update_alt(alt);
            match std::mem::replace(&mut self.interaction, Interaction::None) {
                Interaction::Pan { origin, start_screen } => {
                    let delta = screen - start_screen;
                    let nx = origin.x - delta.x / origin.zoom;
                    let ny = origin.y - delta.y / origin.zoom;
                    self.set_camera(nx, ny, origin.zoom);
                    self.interaction = Interaction::Pan { origin, start_screen };
                }
                _ => {
                    self.interaction = Interaction::None;
                    self.brush_pointer_move(world);
                }
            }
            return;
        }
        match std::mem::replace(&mut self.interaction, Interaction::None) {
            Interaction::DragNodes { primary_id, offset, start_positions, .. } => {
                let primary_id = primary_id.clone();
                let offset = offset;
                let start_positions_cloned = start_positions.clone();
                let (px0, py0) = start_positions.get(&primary_id).copied().unwrap_or((0.0, 0.0));
                let nx = world.x - offset.x;
                let ny = world.y - offset.y;
                let mut dx = nx - px0;
                let mut dy = ny - py0;
                if self.grid_snap_enabled {
                    let (snx, sny) = self.snap_world_pair(nx, ny);
                    dx = snx - px0;
                    dy = sny - py0;
                }
                let mut geometry_changed = false;
                for (id, (ox0, oy0)) in &start_positions {
                    if let Some(n) = self.nodes.get_mut(id) {
                        let mx = ox0 + dx;
                        let my = oy0 + dy;
                        if (n.x - mx).abs() > 1e-9 || (n.y - my).abs() > 1e-9 {
                            geometry_changed = true;
                        }
                        n.x = mx;
                        n.y = my;
                        self.push_event("nodeMove", json!({ "id": id, "x": mx, "y": my }));
                    }
                }
                if geometry_changed {
                    self.bump_content_scene_generation();
                }
                let proximity_pair = if start_positions.len() == 1 { self.node_drag_proximity_handle_pair(primary_id.as_str()) } else { None };
                self.interaction = Interaction::DragNodes { primary_id, offset, start_positions: start_positions_cloned, proximity_pair };
            }
            Interaction::Pan { origin, start_screen } => {
                let delta = screen - start_screen;
                let nx = origin.x - delta.x / origin.zoom;
                let ny = origin.y - delta.y / origin.zoom;
                self.set_camera(nx, ny, origin.zoom);
                self.interaction = Interaction::Pan { origin, start_screen };
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX {
                    self.interaction = Interaction::SelectionPending { initial_ids, start, start_screen };
                } else {
                    let points = vec![start, world];
                    let screen_points = vec![start_screen, screen];
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                    self.apply_area_preselect(&initial_ids, &ids, gesture);
                    self.sync_selection_screen_overlay(start_screen, &screen_points);
                    self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                }
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                let add_point = self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                if add_point {
                    points.push(world);
                    screen_points.push(screen);
                } else if !points.is_empty() {
                    let last = points.len() - 1;
                    points[last] = world;
                    let ls = screen_points.len() - 1;
                    screen_points[ls] = screen;
                }
                let initial = initial_ids.clone();
                let pts = points.clone();
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                let next = self.resolve_area_selection_with_initial(&initial, start, &pts, merge_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let merge_from_modifiers = ctrl_or_meta || shift;
                let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                self.apply_area_preselect(&initial, &ids, gesture);
                self.sync_selection_screen_overlay(start_screen, &screen_points);
                self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
            }
            Interaction::LinkAtSourceHandle { source_id, start_screen } => {
                if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
                    let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                    self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                    self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                    self.sync_link_gesture_events();
                } else {
                    self.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
                    self.update_hover_from_world(world);
                }
            }
            Interaction::LinkDragSnap { source_id, .. } => {
                let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                self.sync_link_gesture_events();
            }
            Interaction::LinkTargetNode { source_id, target_node_id } => {
                self.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids } => {
                self.interaction = Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids };
                self.update_hover_from_world(world);
            }
            Interaction::None => {
                self.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let screen = Point::new(sx, sy);
        let world = self.screen_to_world(screen);
        if self.active_tool == ActiveTool::Brush {
            self.brush_update_alt(alt);
            if matches!(self.interaction, Interaction::Pan { .. }) {
                self.interaction = Interaction::None;
            }
            self.brush_finish_slot();
            self.set_hovered_id(None);
            return;
        }
        let grabbed = std::mem::take(&mut self.interaction);
        match grabbed {
            Interaction::LinkDragSnap { source_id, target_id, .. } => {
                if let Some(ref target_handle_id) = target_id {
                    if self.link_snap_commit_proximity_ok(target_handle_id, world) && self.try_commit_link_edge(&source_id, target_handle_id, Some("proximityConnect")) {
                        self.interaction = Interaction::None;
                        self.clear_link_gesture_events();
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                if let Some(target_node_id) = self.resolve_node_hit_world(world) {
                    let source_node_id = self.handles.get(&source_id).map(|h| h.node_id.clone());
                    if source_node_id.as_deref() != Some(target_node_id.as_str()) {
                        if let Some(sole_target) = self.node_sole_free_link_compatible_handle(source_id.as_str(), target_node_id.as_str()) {
                            self.try_commit_link_edge(&source_id, &sole_target, Some("indirectConnect"));
                            self.clear_link_gesture_events();
                        } else {
                            self.interaction = Interaction::LinkTargetNode { source_id, target_node_id: target_node_id.clone() };
                            self.set_hovered_id(Some(target_node_id));
                            self.sync_link_gesture_events();
                        }
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::LinkAtSourceHandle { .. } => {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
                self.update_hover_from_world(world);
            }
            Interaction::DragNodes { start_positions, proximity_pair: Some((src, tgt)), .. } => {
                let _ = self.try_commit_link_edge(&src, &tgt, Some("proximityConnect"));
                self.push_node_drag_end_events(&start_positions);
                self.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
            Interaction::DragNodes { start_positions, .. } => {
                self.push_node_drag_end_events(&start_positions);
                self.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
            Interaction::SelectionPending { initial_ids, start, start_screen } => {
                let _ = (start, start_screen);
                let merge_from_modifiers = ctrl_or_meta || shift;
                if !merge_from_modifiers {
                    self.clear_selection_on_background_click();
                } else {
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let gesture = Some(merge_mode.as_str());
                    let next = self.resolve_area_selection_with_initial(&initial_ids, start, &[start], merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    self.set_selection_ids_gestured(&ids, gesture);
                }
                self.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                points.push(world);
                screen_points.push(screen);
                let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                let merge_from_modifiers = ctrl_or_meta || shift;
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                let gesture = merge_from_modifiers.then(|| merge_mode.as_str());
                if click_only {
                    self.commit_area_select_from_initial(&initial_ids, &[], gesture);
                } else {
                    let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    self.commit_area_select_from_initial(&initial_ids, &ids, gesture);
                }
                self.set_selection_screen_preview(None);
                self.update_hover_from_world(world);
            }
            Interaction::ExternalLinkPreview { .. } => {
                self.interaction = grabbed;
                self.update_hover_from_world(world);
            }
            _ => {
                self.interaction = Interaction::None;
                self.update_hover_from_world(world);
            }
        }
    }

    pub fn pointer_leave_screen(&mut self, alt: bool) {
        if self.active_tool == ActiveTool::Brush {
            self.brush_update_alt(alt);
            self.brush_finish_slot();
            self.set_hovered_id(None);
            return;
        }
        if matches!(self.interaction, Interaction::None) {
            self.set_hovered_id(None);
        }
    }

    /// @emoji ↩️ Aborts an in‑flight rectangle/lasso drag and restores the selection snapshot from when the gesture began.
    pub fn cancel_area_select(&mut self) -> bool {
        let prev = std::mem::replace(&mut self.interaction, Interaction::None);
        match prev {
            Interaction::SelectionPending { .. } => {
                self.set_selection_screen_preview(None);
                true
            }
            Interaction::Selection { initial_ids, .. } => {
                self.set_selection_screen_preview(None);
                self.preselect.clear();
                self.preselect_removed.clear();
                self.last_preselect_emit_sig = None;
                self.selection = initial_ids.clone();
                self.sync_selection_flags_to_objects();
                self.bump_content_scene_generation();
                self.last_select_emit_sig = None;
                let sorted = Self::sorted_selection_ids(&self.selection);
                self.push_event("preselectCancel", json!({ "ids": sorted }));
                true
            }
            other => {
                self.interaction = other;
                false
            }
        }
    }

    fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
        let raw = match n.shape {
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                WorldBox { min_x: n.x - hw, min_y: n.y - hh, max_x: n.x + hw, max_y: n.y + hh }
            }
            NodeShape::Circle => WorldBox { min_x: n.x - self.scaled_node_radius(n), min_y: n.y - self.scaled_node_radius(n), max_x: n.x + self.scaled_node_radius(n), max_y: n.y + self.scaled_node_radius(n) },
        };
        inflate_world_box(raw, pad)
    }

    fn selection_draggable_node_members(&self) -> Vec<String> {
        self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect()
    }

    fn selection_union_bounds_world(&self) -> Option<WorldBox> {
        let mut corners: Vec<Point> = Vec::new();
        for id in &self.selection {
            let Some(n) = self.nodes.get(id) else {
                continue;
            };
            let b = self.node_world_bounds(n, 0.0);
            corners.push(Point::new(b.min_x, b.min_y));
            corners.push(Point::new(b.max_x, b.max_y));
        }
        world_box_from_points(&corners)
    }

    /// @emoji 📦 Starts a group drag when `world` lies inside the padded union bounds of the current selection (minimap/overview LOD).
    fn try_begin_bounded_selection_drag_at(&mut self, world: Point) -> bool {
        if !self.lod_uses_bounded_drag() {
            return false;
        }
        let members = self.selection_draggable_node_members();
        if members.is_empty() {
            return false;
        }
        let Some(bounds) = self.selection_union_bounds_world() else {
            return false;
        };
        let pad = BOUNDED_DRAG_HIT_PAD_PX / self.camera.zoom.max(1e-9);
        if !world_box_contains_point(inflate_world_box(bounds, pad), world) {
            return false;
        }
        let primary_id = members
            .iter()
            .min_by(|a, b| {
                let da = self.nodes.get(*a).map(|n| distance_between(world, Point::new(n.x, n.y))).unwrap_or(f64::INFINITY);
                let db = self.nodes.get(*b).map(|n| distance_between(world, Point::new(n.x, n.y))).unwrap_or(f64::INFINITY);
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
            .unwrap_or_else(|| members[0].clone());
        let (px0, py0) = self.nodes.get(&primary_id).map(|n| (n.x, n.y)).unwrap_or((0.0, 0.0));
        let mut start_positions = BTreeMap::new();
        for id in &members {
            if let Some(n) = self.nodes.get(id) {
                start_positions.insert(id.clone(), (n.x, n.y));
            }
        }
        self.interaction = Interaction::DragNodes { primary_id, offset: world - Point::new(px0, py0), start_positions, proximity_pair: None };
        self.set_hovered_id(None);
        true
    }

    fn selection_drag_shape_world(&self, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
        selection_drag_shape(self.selection_options.method.as_str(), start, points)
    }

    fn selection_contains_node(&self, n: &NodeData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
        let bounds = self.node_world_bounds(n, 0.0);
        let lasso = self.selection_options.method == "lasso";
        if enclosing {
            if lasso {
                polygon_contains_world_box(polygon, bounds)
            } else {
                world_box_contains_box(box_, bounds)
            }
        } else if lasso {
            polygon_intersects_world_box(polygon, bounds)
        } else {
            world_boxes_overlap(box_, bounds)
        }
    }

    fn selection_contains_handle(&self, h: &HandleData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
        let Some(pos) = self.handle_world_pos(h) else {
            return false;
        };
        let pad = self.effective_handle_radius(h).max(1.0);
        let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
        let lasso = self.selection_options.method == "lasso";
        if enclosing {
            if lasso {
                polygon_contains_world_box(polygon, bounds)
            } else {
                world_box_contains_box(box_, bounds)
            }
        } else if lasso {
            polygon_intersects_world_box(polygon, bounds)
        } else {
            world_boxes_overlap(box_, bounds)
        }
    }

    fn selection_contains_edge(&self, c: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
        const STEPS: usize = 24;
        let mut samples = Vec::with_capacity(STEPS + 1);
        for i in 0..=STEPS {
            let t = i as f64 / STEPS as f64;
            samples.push(cubic_bezier_point(c, t));
        }
        let lasso = self.selection_options.method == "lasso";
        if enclosing {
            if lasso {
                samples.iter().all(|&p| point_in_polygon(p, polygon))
            } else {
                samples.iter().all(|&p| world_box_contains_point(box_, p))
            }
        } else if lasso {
            (1..samples.len()).any(|i| segment_intersects_polygon(samples[i - 1], samples[i], polygon))
        } else {
            (1..samples.len()).any(|i| segment_intersects_world_box(samples[i - 1], samples[i], box_))
        }
    }

    fn resolve_area_selection_with_initial(&self, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
        let Some((box_, enclosing, ref polygon)) = self.selection_drag_shape_world(start, points) else {
            return initial.clone();
        };
        let mut hits = BTreeSet::new();
        let o = &self.selection_options;
        if o.select_nodes {
            for n in self.nodes.values() {
                if self.node_selectable(n.id.as_str()) && self.selection_contains_node(n, box_, enclosing, polygon) {
                    hits.insert(n.id.clone());
                }
            }
        }
        if o.select_handles {
            for h in self.handles.values() {
                if self.handle_selectable(h.id.as_str()) && self.selection_contains_handle(h, box_, enclosing, polygon) {
                    hits.insert(h.id.clone());
                }
            }
        }
        if o.select_edges {
            for e in self.edges.values() {
                if !self.edge_selectable(e) {
                    continue;
                }
                if let Some(c) = self.edge_curve(e) {
                    if self.selection_contains_edge(c, box_, enclosing, polygon) {
                        hits.insert(e.id.clone());
                    }
                }
            }
        }
        merge_ids_into_selection(initial, &hits, merge_mode)
    }
}

#[doc(hidden)]
impl BoardHost {
    pub fn test_resolve_node_style_kind(&self, node_id: &str) -> Option<BoardElementStyleKind> {
        self.nodes.get(node_id).map(|n| self.resolve_node_style_kind(n, StyleChromePass::InteractionOverlay))
    }
}

impl infinite_cavas::canvas_content::CanvasContent for BoardHost {
    fn build_scene(&self) -> Scene {
        self.build_vector_scene()
    }

    fn clear_color(&self) -> Color {
        self.vello_theme.raster_clear
    }
}
// #endregion board_host
}

pub use board_host::*;
pub use infinite_cavas as cavas;
pub use mathematical_graph_normal_undirected::{
apply_force_graph_layout_to_fixture_v1_json as apply_undirected_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value as apply_undirected_force_graph_layout_to_fixture_v1_value,
apply_redraw_layout_to_fixture_v1_json as apply_normal_undirected_redraw_layout_to_fixture_v1_json, ForceGraphLayoutOptions as UndirectedForceGraphLayoutOptions,
};
pub use mathematical_graph_port_directed::*;
