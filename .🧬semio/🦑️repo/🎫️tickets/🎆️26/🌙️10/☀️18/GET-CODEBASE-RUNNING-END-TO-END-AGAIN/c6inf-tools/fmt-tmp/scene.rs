//! 🖼️ Normal board scene and style components with the scene painting, style resolution and viewport functions over them.
//!
//! Crate `semio-framework-os-infinite-board-normal-scene`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, BoardPaletteOverlay, builtin_edge_tips, EdgeTipCatalogEntry, EdgeTipDef, EdgeTipGeometry};
use semio_framework_os_infinite_board_engine::{BoardElementStyleKind, CachedIconBody, CachedIconPaintLease, Camera, CanvasPalette, CompatSpecificity, Edge, EdgeData, EdgeKindDef, EdgeStrokePattern, GraphPortMode, GRID_WORLD_LARGE, GRID_WORLD_MEDIUM, GRID_WORLD_MICRO, GRID_WORLD_SMALL, HandleData, HandleKindDef, IconPaintCache, LinkCompatRule, NodeData, NodeKindDef, NodeKindHandleTemplate, RegionData, snap_region_scalar, WireData, WireKindDef};
use semio_framework_os_infinite_board_normal_model::{board_draw_lod_label, BOARD_EVENT_BYTE_CAPACITY, BOARD_EVENT_PAYLOAD_BYTE_CAPACITY, BOARD_HANDLE_VITALS_CAP, BOARD_LOD_SCALE, BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, BoardDrawLod, BoardEventFault, BoardNaming, BoardOwnedEvent, BoardPayloadBuilder, BoardPlacementNaming, BoardPointerSpan, brush_handle_alignment_delta, BrushCandidatePage, compat_pair_matches, DEFAULT_WIRE_KIND_ID, DropPreviewSnapshot, HANDLE_HIT_TOLERANCE_PX, handle_port_shapes_compatible, INDIRECT_HANDLE_MARKER_NODE_SCALE, INDIRECT_HANDLE_RING_GAP_NODE_SCALE, LINK_COMMIT_SNAP_TIGHT_PX, link_gesture_rule_applies_kind_strings, LINK_HANDLE_SNAP_EXTRA_PX, node_fill_for_style, node_stroke_for_style, NodeHandlePaintLayer, NormalPortError, parse_catalog_tip_slot, parse_compat_specificity, parse_css_color, parse_css_hex_color, reject_kind_catalog_row_legacy_label, resolve_handle_stroke_color, single_letter_port_families_compatible, WorldContentCache};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_board_normal_model::board_property_retirement_credits;
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Rect, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet};
use serde::Deserialize;
use semio_framework_graph_manifest::{GraphManifest, PropertyValue};
use graph::{Handle, Node, NodeShape};
use std::cell::RefCell;
use semio_framework_pack_json::Value;
use semio_framework_canvas::geom_sel::{cubic_bezier_axis_bounds, inflate_world_box, world_boxes_overlap, WorldBox};
use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
use geometry::{distance_between, normalize_or_zero};

impl BoardSceneState {
    pub fn drop_preview_from_json(&self, node: &serde_json::Value) -> Option<DropPreviewSnapshot> {
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
        Some(DropPreviewSnapshot { node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, icon_kind })
    }

    pub fn set_pointer_selection_flag(&mut self, id: &str, selected: bool) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.selected = selected;
        }
        if let Some(handle) = self.handles.get_mut(id) {
            handle.selected = selected;
        }
        if let Some(edge) = self.edges.get_mut(id) {
            edge.selected = selected;
        }
        if let Some(wire) = self.wires.get_mut(id) {
            wire.selected = selected;
        }
    }

    pub fn world_to_screen(&self, p: Point) -> Point {
        semio_framework_canvas::camera::world_to_screen(&self.camera, &self.viewport(), p)
    }

    pub fn screen_to_world(&self, p: Point) -> Point {
        semio_framework_canvas::camera::screen_to_world(&self.camera, &self.viewport(), p)
    }

    pub fn eligible_indirect_handle_count_on_node(&self, node_id: &str) -> usize {
        self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_effectively_visible(id.as_str()) && self.handle_eligible_indirect_connect_ring(id.as_str())).count()
    }

    /// 🧭️ Returns the handle id when `node_id` has exactly one visible free indirect-eligible handle.
    pub fn sole_eligible_indirect_handle_on_node(&self, node_id: &str) -> Option<String> {
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

    pub fn node_center_world(&self, node_id: &str) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        Some(Point::new(n.x, n.y))
    }
}

impl BoardStyleState {
    /// 🧩️ Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
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

    pub fn set_canvas_palette(&mut self, overlay: &BoardPaletteOverlay) {
        self.canvas_theme.apply_overlay(overlay);
        self.icon_paint_cache.clear();
    }

    fn node_kind_scale(&self, node_kind: &str) -> f64 {
        self.node_kinds.get(node_kind).map_or(1.0, |k| k.scale)
    }

    fn handle_kind_scale(&self, handle_kind: &str) -> f64 {
        self.handle_kinds.get(handle_kind).map_or(1.0, |k| k.scale)
    }

    fn effective_node_scale(&self, n: &NodeData) -> f64 {
        (n.scale * self.node_kind_scale(n.node_kind.as_str())).max(1e-9)
    }

    pub fn scaled_node_radius(&self, n: &NodeData) -> f64 {
        n.radius * self.effective_node_scale(n)
    }

    pub fn scaled_node_width(&self, n: &NodeData) -> f64 {
        n.width * self.effective_node_scale(n)
    }

    pub fn scaled_node_height(&self, n: &NodeData) -> f64 {
        n.height * self.effective_node_scale(n)
    }

    /// 📐️ Node half-extent for indirect ring layout: circle radius or half the shorter rectangle side.
    fn indirect_node_half_extent(&self, n: &NodeData) -> f64 {
        match n.shape {
            NodeShape::Circle => self.scaled_node_radius(n),
            NodeShape::Rectangle => self.scaled_node_width(n).min(self.scaled_node_height(n)) * 0.5,
        }
    }

    /// 📐️ Radial world offset from node rim to indirect-handle center (`INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× half-extent) so ring–node proportions stay fixed when zooming.
    fn indirect_handle_ring_offset_world(&self, n: &NodeData) -> f64 {
        (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_RING_GAP_NODE_SCALE).max(1e-9)
    }

    pub fn point_in_node_world(&self, n: &NodeData, point: Point) -> bool {
        match n.shape {
            NodeShape::Rectangle => {
                let hw = self.scaled_node_width(n) / 2.0;
                let hh = self.scaled_node_height(n) / 2.0;
                (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
            }
            NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
        }
    }
}

fn append_brush_node_icon_paint(scene_state: &BoardSceneState, style: &BoardStyleState, scene: &mut Scene, lod: BoardDrawLod, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: &str, world_space: bool) {
    if !matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
        return;
    }
    let preserve_original_style = false;
    let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&style.canvas_theme);
    let Some(paint) = style.get_or_build_icon_paint(icon_kind, icon_fg, icon_bg, preserve_original_style) else {
        return;
    };
    let (bx, by, bw, bh) = paint.bounds();
    let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
    let fit_inset = ui_styling::metrics::icon::FIT_INSET;
    let (sx_half, sy_half) = match shape {
        NodeShape::Circle => {
            let s = scene_state.draw_space_len(radius, world_space) * fit_inset;
            (s, s)
        }
        NodeShape::Rectangle => (scene_state.draw_space_len(width, world_space) * fit_inset * 0.5, scene_state.draw_space_len(height, world_space) * fit_inset * 0.5),
    };
    let center_ds = scene_state.draw_space_point(center, world_space);
    let cx = bx + bw * 0.5;
    let cy = by + bh * 0.5;
    let avail_w = 2.0 * sx_half;
    let avail_h = 2.0 * sy_half;
    let scale = (avail_w / bw).min(avail_h / bh);
    let aff = Affine::IDENTITY.translate((center_ds.x - scale * cx, center_ds.y - scale * cy)) * Affine::IDENTITY.scale(scale);
    match shape {
        NodeShape::Circle => {
            let r_clip = scene_state.draw_space_len(radius, world_space) * clip_inset;
            let disc = Circle::new(center_ds, r_clip);
            scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
            match paint.body() {
                CachedIconBody::Vector(icon_scene) => {
                    scene.append(icon_scene, Some(aff));
                }
                CachedIconBody::Raster(img) => {
                    scene.draw_image(img, aff);
                }
            }
            scene.pop_layer();
        }
        NodeShape::Rectangle => {
            let hw = scene_state.draw_space_len(width, world_space) * clip_inset * 0.5;
            let hh = scene_state.draw_space_len(height, world_space) * clip_inset * 0.5;
            let clip_r = Rect::from_points(Point::new(center_ds.x - hw, center_ds.y - hh), Point::new(center_ds.x + hw, center_ds.y + hh));
            scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
            match paint.body() {
                CachedIconBody::Vector(icon_scene) => {
                    scene.append(icon_scene, Some(aff));
                }
                CachedIconBody::Raster(img) => {
                    scene.draw_image(img, aff);
                }
            }
            scene.pop_layer();
        }
    }
}

pub fn paint_highlighted_node_preview(scene_state: &BoardSceneState, style_state: &BoardStyleState, scene: &mut Scene, _lod: BoardDrawLod, x: f64, y: f64, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: Option<&str>, world_space: bool) {
    let center = Point::new(x, y);
    let style = BoardElementStyleKind::Highlighted;
    let fill = node_fill_for_style(&style_state.canvas_theme, style);
    let stroke_c = node_stroke_for_style(&style_state.canvas_theme, style);
    let stroke = Stroke::new(ui_styling::strokes::NODE_BODY);
    match shape {
        NodeShape::Circle => {
            let c = scene_state.draw_space_point(center, world_space);
            let r = scene_state.draw_space_len(radius, world_space);
            let circle = Circle::new(c, r);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
            scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &circle);
        }
        NodeShape::Rectangle => {
            let hw = width * 0.5;
            let hh = height * 0.5;
            let p0 = scene_state.draw_space_point(Point::new(x - hw, y - hh), world_space);
            let p1 = scene_state.draw_space_point(Point::new(x + hw, y + hh), world_space);
            let rect = Rect::from_points(p0, p1);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
            scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &rect);
        }
    }
    if let Some(icon) = icon_kind.map(str::trim).filter(|s| !s.is_empty()) {
        append_brush_node_icon_paint(scene_state, style_state, scene, BoardDrawLod::Detail, center, shape, radius, width, height, icon, world_space);
    }
}

/// 🧲️ The common offset a drag of `start_positions` grabbed at `offset` reaches at `world`, snapped to the grid
/// when snapping is on — the one rule the live preview, the release record and the wgpu plan all share.
pub fn drag_delta(scene: &BoardSceneState, style: &BoardStyleState, world: Point, primary_id: &str, offset: Vec2, start_positions: &BTreeMap<String, (f64, f64)>) -> Option<Vec2> {
    let (px0, py0) = start_positions.get(primary_id).copied()?;
    let (nx, ny) = (world.x - offset.x, world.y - offset.y);
    let (tx, ty) = if style.grid_snap_enabled { board_snap_world_pair(scene, style, nx, ny) } else { (nx, ny) };
    Some(Vec2::new(tx - px0, ty - py0))
}

/// 🩺️ Every handle the pane can actually be POINTED AT, as one bounded probe row:
/// world position, owning node, handle kind, and whether the handle is still free (no incident
/// edge — the precondition `openHandleSuggestions` and `createEdge` both need). `data-board-
/// positions-json` carries NODES only, so nothing in the DOM ever named a handle and no
/// headless caller could aim at one.
///
/// 🧱️ Bounded twice over, because Nakagin carries 358 handles: the row set is the camera's own
/// viewport (a handle off-screen cannot be clicked, so publishing it buys nothing) and then a
/// hard [`BOARD_HANDLE_VITALS_CAP`]. `total`/`onScreen`/`capped` say exactly what was left out.
pub fn handle_positions_json(scene: &BoardSceneState, style: &BoardStyleState) -> String {
    let (w, h) = (f64::from(scene.width), f64::from(scene.height));
    let mut rows: Vec<(&str, Point, &str, &str, bool)> = Vec::new();
    let mut on_screen = 0usize;
    for (id, handle) in &scene.handles {
        let Some(world) = handle_world_pos(scene, style, handle) else { continue };
        let screen = scene.world_to_screen(world);
        if screen.x < 0.0 || screen.y < 0.0 || screen.x > w || screen.y > h || !scene.handle_effectively_visible(id.as_str()) {
            continue;
        }
        on_screen += 1;
        if rows.len() < BOARD_HANDLE_VITALS_CAP {
            rows.push((id.as_str(), world, handle.node_id.as_str(), handle.handle_kind.as_str(), !scene.handle_has_incident_edge(id.as_str())));
        }
    }
    let quote = |value: &str, out: &mut String| {
        out.push('"');
        out.push_str(&value.replace('\\', "\\\\").replace('"', "\\\""));
        out.push('"');
    };
    let mut out = String::from("{\"total\":");
    out.push_str(&scene.handles.len().to_string());
    out.push_str(",\"onScreen\":");
    out.push_str(&on_screen.to_string());
    out.push_str(",\"published\":");
    out.push_str(&rows.len().to_string());
    out.push_str(",\"capped\":");
    out.push_str(if on_screen > rows.len() { "true" } else { "false" });
    out.push_str(",\"rows\":[");
    for (index, (id, world, node_id, handle_kind, open)) in rows.iter().copied().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('[');
        quote(id, &mut out);
        out.push(',');
        out.push_str(&world.x.to_string());
        out.push(',');
        out.push_str(&world.y.to_string());
        out.push(',');
        quote(node_id, &mut out);
        out.push(',');
        quote(handle_kind, &mut out);
        out.push(',');
        out.push_str(if open { "true" } else { "false" });
        out.push(']');
    }
    out.push_str("]}");
    out
}

fn effective_handle_scale(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    let node_scale = scene.nodes.get(h.node_id.as_str()).map_or(1.0, |n| style.effective_node_scale(n));
    (node_scale * h.scale * style.handle_kind_scale(h.handle_kind.as_str())).max(1e-9)
}

pub fn effective_handle_radius(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    h.radius * effective_handle_scale(scene, style, h)
}

pub fn handle_world_pos(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<Point> {
    let n = scene.nodes.get(&h.node_id)?;
    Some(match n.shape {
        NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), style.scaled_node_radius(n), h.angle),
        NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), style.scaled_node_width(n), style.scaled_node_height(n), h.angle),
    })
}

/// 🎯️ Where an edge meets a handle: the cap's outer peak, the outward normal, and the node center.
pub fn handle_cap_attachment(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<(Point, Vec2, Point)> {
    let rim = handle_world_pos(scene, style, h)?;
    let n = scene.nodes.get(&h.node_id)?;
    let node_center = Point::new(n.x, n.y);
    let outward = handle_outward_at_node_rim(rim, node_center, n.shape, style.scaled_node_radius(n), style.scaled_node_width(n), style.scaled_node_height(n))?;
    let peak = handle_exterior_cap_peak(rim, outward, effective_handle_radius(scene, style, h));
    Some((peak, outward, node_center))
}

/// 📐️ Ghost link handles sit on a rim offset by `INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× node half-extent from the node body so ring spacing scales with the node at every zoom.
pub fn indirect_handle_world_pos(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<Point> {
    let n = scene.nodes.get(&h.node_id)?;
    let offset = style.indirect_handle_ring_offset_world(n);
    Some(match n.shape {
        NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), style.scaled_node_radius(n) + offset, h.angle),
        NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), style.scaled_node_width(n) + 2.0 * offset, style.scaled_node_height(n) + 2.0 * offset, h.angle),
    })
}

/// 📐️ Indirect-connect marker radius in world units: `INDIRECT_HANDLE_MARKER_NODE_SCALE`× circle radius or × half the shorter rectangle side.
pub fn indirect_handle_marker_radius_world(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    let Some(n) = scene.nodes.get(&h.node_id) else {
        return (effective_handle_radius(scene, style, h) * INDIRECT_HANDLE_MARKER_NODE_SCALE).max(1e-9);
    };
    let handle_local_scale = (h.scale * style.handle_kind_scale(h.handle_kind.as_str())).max(1e-9);
    (style.indirect_node_half_extent(n) * INDIRECT_HANDLE_MARKER_NODE_SCALE * handle_local_scale).max(1e-9)
}

/// 🧭️ Visible target node ids that expose at least one free handle compatible with `source_handle_id`.
pub fn link_drag_compatible_target_node_ids(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str) -> Vec<String> {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return Vec::new();
    };
    let source_node_id = source.node_id.as_str();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for (hid, h) in &scene.handles {
        if h.node_id == source_node_id || !scene.handle_effectively_visible(hid.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(hid.as_str()) {
            continue;
        }
        if !handles_link_compatible_for_drag(scene, style, source, h) {
            continue;
        }
        if !scene.nodes.get(&h.node_id).is_some_and(|n| n.visible) {
            continue;
        }
        if seen.insert(h.node_id.clone()) {
            out.push(h.node_id.clone());
        }
    }
    out.sort();
    out
}

/// 🧭️ Count of visible free handles on `node_id` compatible with `source_handle_id`.
pub fn link_compatible_handle_count_on_node(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, node_id: &str) -> usize {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return 0;
    };
    if source.node_id == node_id {
        return 0;
    }
    scene.handles.iter().filter(|(id, h)| h.node_id == node_id && handle_eligible_link_target_ring(scene, style, id.as_str(), source_handle_id) && handles_link_compatible_for_drag(scene, style, source, h)).count()
}

/// 🧭️ Free compatible handle ids on `node_id` for an active link from `source_handle_id`.
pub fn link_compatible_handle_ids_on_node(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, node_id: &str) -> Vec<String> {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return Vec::new();
    };
    let mut out: Vec<String> = scene
        .handles
        .iter()
        .filter_map(|(id, h)| {
            if h.node_id != node_id {
                return None;
            }
            if !handle_eligible_link_target_ring(scene, style, id.as_str(), source_handle_id) {
                return None;
            }
            handles_link_compatible_for_drag(scene, style, source, h).then(|| id.clone())
        })
        .collect();
    out.sort();
    out
}

/// 🧭️ Compatible target node under `world` while a link wire is active (node body hit).
pub fn link_drag_ring_target_node_id(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, world: Point) -> Option<String> {
    let nid = resolve_node_hit_world(scene, style, world)?;
    if scene.handles.get(source_handle_id)?.node_id == nid {
        return None;
    }
    node_has_any_free_link_compatible_handle(scene, style, source_handle_id, nid.as_str()).then_some(nid)
}

/// 🧭️ When the drop target has exactly one free handle compatible with `source_handle_id`, returns that handle id (otherwise `None`).
pub fn node_sole_free_link_compatible_handle(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, target_node_id: &str) -> Option<String> {
    let source = scene.handles.get(source_handle_id)?;
    if source.node_id == target_node_id {
        return None;
    }
    let mut found: Option<String> = None;
    for (id, h) in &scene.handles {
        if h.node_id != target_node_id || !scene.handle_effectively_visible(id.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(id.as_str()) {
            continue;
        }
        if !handles_link_compatible_for_drag(scene, style, source, h) {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = Some(id.clone());
    }
    found
}

/// 🧭️ True when `target_node_id` hosts at least one visible free handle that can pair with `source_handle_id` under link-compat rules.
pub fn node_has_any_free_link_compatible_handle(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, target_node_id: &str) -> bool {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return false;
    };
    if source.node_id == target_node_id {
        return false;
    }
    for (hid, h) in &scene.handles {
        if h.node_id != target_node_id || !scene.handle_effectively_visible(hid.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(hid.as_str()) {
            continue;
        }
        if handles_link_compatible_for_drag(scene, style, source, h) {
            return true;
        }
    }
    false
}

/// 💫️ True when the handle may appear on a link-target ghost ring (`overview`/`normal` LOD).
pub fn handle_eligible_link_target_ring(scene: &BoardSceneState, style: &BoardStyleState, handle_id: &str, source_handle_id: &str) -> bool {
    if !scene.handle_effectively_visible(handle_id) || scene.handle_has_incident_edge(handle_id) {
        return false;
    }
    let Some(source) = scene.handles.get(source_handle_id) else {
        return false;
    };
    let Some(target) = scene.handles.get(handle_id) else {
        return false;
    };
    if source.node_id == target.node_id {
        return false;
    }
    handles_link_compatible_for_drag(scene, style, source, target)
}

pub fn board_edge_curve(scene: &BoardSceneState, style: &BoardStyleState, e: &EdgeData) -> Option<CubicBez> {
    if !style.has_ports() {
        let source_node = scene.nodes.get(&e.source)?;
        let target_node = scene.nodes.get(&e.target)?;
        let source_center = Point::new(source_node.x, source_node.y);
        let target_center = Point::new(target_node.x, target_node.y);
        let source_pos = style.node_rim_point_toward(source_node, target_center);
        let target_pos = style.node_rim_point_toward(target_node, source_center);
        return Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center));
    }
    let source_handle = scene.handles.get(&e.source)?;
    let target_handle = scene.handles.get(&e.target)?;
    let (source_pos, source_out, _) = handle_cap_attachment(scene, style, source_handle)?;
    let (target_pos, target_out, _) = handle_cap_attachment(scene, style, target_handle)?;
    Some(compute_edge_bezier_outward(source_pos, target_pos, source_out, target_out))
}

pub fn link_drag_wire_curve_world(scene: &BoardSceneState, style: &BoardStyleState, source_id: &str, target_id: Option<&str>, end_world: Point) -> Option<CubicBez> {
    let source_handle = scene.handles.get(source_id)?;
    let (source_pos, source_out, _) = handle_cap_attachment(scene, style, source_handle)?;
    let (target_pos, target_out) = if let Some(tid) = target_id {
        let (peak, outward, _) = handle_cap_attachment(scene, style, scene.handles.get(tid)?)?;
        (peak, outward)
    } else {
        (end_world, Vec2::ZERO)
    };
    Some(compute_edge_bezier_outward(source_pos, target_pos, source_out, target_out))
}

pub fn wire_curve(scene: &BoardSceneState, style_state: &BoardStyleState, w: &WireData) -> Option<CubicBez> {
    let end_world = match (&w.target, w.end_x, w.end_y) {
        (None, Some(x), Some(y)) if x.is_finite() && y.is_finite() => Point::new(x, y),
        (Some(tid), _, _) => {
            scene.handles.get(tid)?;
            return board_edge_curve(scene, style_state, &EdgeData {
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
                properties: w.properties.clone(),
            });
        }
        _ => return None,
    };
    link_drag_wire_curve_world(scene, style_state, w.source.as_str(), None, end_world)
}

/// 🧭️ Minimap/overview LOD: group selection and bounded drag only — no per-node/edge/handle picks.
pub fn lod_disables_discrete_pick(scene: &BoardSceneState, style: &BoardStyleState) -> bool {
    matches!(current_draw_lod(scene, style), BoardDrawLod::Minimap | BoardDrawLod::Overview)
}

/// 🧭️ Minimap/overview LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
pub fn board_lod_uses_bounded_drag(scene: &BoardSceneState, style: &BoardStyleState) -> bool {
    matches!(current_draw_lod(scene, style), BoardDrawLod::Minimap | BoardDrawLod::Overview)
}

pub fn resolve_node_hit_world(scene: &BoardSceneState, style: &BoardStyleState, point: Point) -> Option<String> {
    for n in scene.nodes.values().rev() {
        if !n.visible {
            continue;
        }
        match n.shape {
            NodeShape::Rectangle => {
                let hw = style.scaled_node_width(n) / 2.0;
                let hh = style.scaled_node_height(n) / 2.0;
                if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                    return Some(n.id.clone());
                }
            }
            NodeShape::Circle => {
                if distance_between(point, Point::new(n.x, n.y)) <= style.scaled_node_radius(n) {
                    return Some(n.id.clone());
                }
            }
        }
    }
    None
}

impl BoardSceneState {
    pub fn bump_content_scene_generation(&mut self) {
        self.content_scene_generation = self.content_scene_generation.wrapping_add(1);
    }

    #[doc(hidden)]
    pub fn test_content_scene_generation(&self) -> u64 {
        self.content_scene_generation
    }

    pub fn viewport(&self) -> semio_framework_canvas::camera::Viewport {
        semio_framework_canvas::camera::Viewport { width: self.width, height: self.height, dpr: self.dpr }
    }

    pub fn camera_content_affine(&self) -> Affine {
        semio_framework_canvas::camera::camera_content_affine(&self.camera, &self.viewport())
    }
}

/// 📶️ Draw LOD used while building the vector scene (pins during wheel zoom).
pub fn board_draw_lod_for_frame(scene: &BoardSceneState, style: &BoardStyleState) -> BoardDrawLod {
    if style.wheel_zoom_active {
        if let Some(pinned) = style.wheel_zoom_render_lod {
            return pinned;
        }
    }
    current_draw_lod(scene, style)
}

/// 🏷️ Camera, draw LOD, and visible node centers from the WASM host for the JS text overlay (must match the last GPU frame).
pub fn overlay_paint_state_json(scene: &BoardSceneState, style: &BoardStyleState) -> String {
    let nodes: Vec<serde_json::Value> = scene.nodes.values().filter(|n| n.visible).map(|n| serde_json::json!({ "id": n.id, "x": n.x, "y": n.y })).collect();
    serde_json::json!({
        "camera": { "x": scene.camera.x, "y": scene.camera.y, "zoom": scene.camera.zoom },
        "lod": board_draw_lod_label(board_draw_lod_for_frame(scene, style)),
        "nodes": nodes,
    })
    .to_string()
}

impl BoardStyleState {
    pub fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }
    pub fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }
    pub fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }
    pub fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    /// 🪪️ Installs the owning domain's placement naming: brush placements minted and fill snapshots captured
    /// afterwards carry its id prefixes and stage labels.
    pub fn set_naming(&mut self, naming: &'static BoardNaming) {
        self.naming = naming;
        self.placement_naming = None;
    }

    /// 🆔️ The placement naming this host mints ids with.
    pub fn naming(&self) -> &'static BoardNaming {
        self.naming
    }

    /// 📮️ Installs (`Some`) or clears (`None`) the brush prefixes a scene carries in wire form ([`BoardPlacementNaming`]);
    /// brush placements minted afterwards use them instead of the installed naming's. A refused form changes nothing.
    pub fn set_placement_naming_json(&mut self, json: Option<&str>) -> Result<(), NormalPortError> {
        self.placement_naming = json.map(BoardPlacementNaming::from_json).transpose()?;
        Ok(())
    }

    /// 🔍️ The brush prefixes this host mints with, the wire override first, in wire form.
    pub fn brush_placement_naming(&self) -> BoardPlacementNaming {
        BoardPlacementNaming { brush_node_prefix: self.brush_node_prefix().into(), brush_edge_prefix: self.brush_edge_prefix().into() }
    }

    /// 🔤️ The node id prefix the next brush placement is minted with.
    pub fn brush_node_prefix(&self) -> &str {
        self.placement_naming.as_ref().map_or(self.naming.brush_node_prefix, |naming| naming.brush_node_prefix.as_str())
    }

    /// 🔡️ The edge id prefix the next brush placement is minted with.
    pub fn brush_edge_prefix(&self) -> &str {
        self.placement_naming.as_ref().map_or(self.naming.brush_edge_prefix, |naming| naming.brush_edge_prefix.as_str())
    }

    pub fn has_ports(&self) -> bool {
        self.port_mode.has_ports()
    }

    pub(crate) fn node_rim_point_toward(&self, node: &NodeData, toward: Point) -> Point {
        let center = Point::new(node.x, node.y);
        match node.shape {
            NodeShape::Circle => {
                let radius = self.scaled_node_radius(node);
                let angle = circle_handle_angle_toward(center, toward);
                handle_position_on_circle(center, radius, angle)
            }
            NodeShape::Rectangle => {
                let width = self.scaled_node_width(node);
                let height = self.scaled_node_height(node);
                let angle = rectangle_handle_angle_toward(center, width, height, toward);
                handle_position_on_rectangle(center, width, height, angle)
            }
        }
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

    pub fn set_grid_factor(&mut self, v: f64) -> Result<(), NormalPortError> {
        if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
            return Err(NormalPortError::GridFactorOutOfRange);
        }
        self.grid_factor = v;
        Ok(())
    }

    pub(crate) fn get_or_build_icon_paint(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
        self.icon_paint_cache.get_or_build(encoded, fg, bg, preserve_original_style)
    }

    pub fn clear_icon_vector_cache(&mut self) {
        self.icon_paint_cache.clear();
    }

    /// 🔗️ JSON `[{ "source","target","bidirectional"?,"important"?,"specificity"? },…]` gates link gestures; empty clears restrictions.
    pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        let v: serde_json::Value = serde_json::from_str(json)?;
        let arr = v.as_array().ok_or(NormalPortError::CompatNotArray)?;
        let mut next = Vec::new();
        for row in arr {
            let o = row.as_object().ok_or(NormalPortError::RowNotObject("compat"))?;
            let source = o.get("source").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatSourceMissing)?.trim().to_string();
            let target = o.get("target").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatTargetMissing)?.trim().to_string();
            let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
            let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
            let spec_s = o.get("specificity").and_then(|x| x.as_str()).unwrap_or("handle");
            let specificity = parse_compat_specificity(spec_s)?;
            next.push(LinkCompatRule { source, target, bidirectional, important, specificity });
        }
        self.link_compat_rules = next;
        Ok(())
    }

    /// 🧩️ JSON object `{ handleKinds?, wireKinds?, nodeKinds?, edgeKinds? }` replacing prior catalogs (omit arrays to clear that slice).
    pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        if json.len() > BOARD_EVENT_BYTE_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        let v: serde_json::Value = serde_json::from_str(json)?;
        let o = v.as_object().ok_or(NormalPortError::KindCatalogsRootNotObject)?;
        for key in ["handleKinds", "wireKinds", "nodeKinds", "edgeTips", "edgeKinds"] {
            if o.get(key).and_then(serde_json::Value::as_array).is_some_and(|rows| rows.len() > BOARD_POINTER_ITEM_CAPACITY) {
                return Err(NormalPortError::EventCredits);
            }
        }
        let template_count = o
            .get("nodeKinds")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_object)
            .filter_map(|row| row.get("handles"))
            .filter_map(serde_json::Value::as_array)
            .try_fold(0usize, |count, handles| count.checked_add(handles.len()))
            .ok_or(NormalPortError::EventCredits)?;
        if template_count > BOARD_POINTER_ITEM_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let ho = row.as_object().ok_or(NormalPortError::RowNotObject("handle kind"))?;
                reject_kind_catalog_row_legacy_label(ho, "handle")?;
                let id = ho.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("handle kind"))?;
                let name = ho.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color_s = ho.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::HandleKindColorMissing)?;
                let color = parse_css_color(color_s).ok_or_else(|| NormalPortError::InvalidHandleKindColor(color_s.to_string()))?;
                let default_wire_kind = ho.get("defaultWireKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                next.insert(id.to_string(), HandleKindDef { name, color, default_wire_kind, scale });
            }
            self.handle_kinds = next;
        }
        if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let wo = row.as_object().ok_or(NormalPortError::RowNotObject("wire kind"))?;
                reject_kind_catalog_row_legacy_label(wo, "wire")?;
                let id = wo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("wire kind"))?;
                let name = wo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let default_edge_kind = wo.get("defaultEdgeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
            }
            self.wire_kinds = next;
        }
        if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let no = row.as_object().ok_or(NormalPortError::RowNotObject("node kind"))?;
                reject_kind_catalog_row_legacy_label(no, "node")?;
                let id = no.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("node kind"))?;
                let name = no.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                let shape = match no.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                    Some("rectangle") => NodeShape::Rectangle,
                    _ => NodeShape::Circle,
                };
                let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
                if let Some(arr) = no.get("handles").and_then(|x| x.as_array()) {
                    for row in arr {
                        let ho = row.as_object().ok_or(NormalPortError::RowNotObject("node kind handle"))?;
                        let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::NodeKindHandleKindMissing)?;
                        let angle = ho.get("angle").and_then(|x| x.as_f64()).filter(|x| x.is_finite()).ok_or(NormalPortError::NodeKindHandleAngleMissing)?;
                        let radius = ho.get("radius").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0);
                        handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                    }
                }
                let icon = no.get("icon").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let color_fill = no.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(parse_css_hex_color);
                next.insert(id.to_string(), NodeKindDef { name, scale, shape, handles, icon, color_fill });
            }
            self.node_kinds = next;
        }
        if let Some(arr) = o.get("edgeTips").and_then(|x| x.as_array()) {
            let mut tips = builtin_edge_tips();
            for row in arr {
                let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge tip"))?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge tip"))?;
                let geometry=match eo.get("geometry") {
                    None=>None,
                    Some(value)=>Some(match value.as_str().map(str::trim){Some("arrow")=>EdgeTipGeometry::Arrow,Some("fine-arrow")=>EdgeTipGeometry::FineArrow,Some("diamond")=>EdgeTipGeometry::Diamond,Some("circle")=>EdgeTipGeometry::Circle,Some("bar")=>EdgeTipGeometry::Bar,_=>return Err(NormalPortError::EdgeTipRowInvalid(id.to_string()))})
                };
                let entry=EdgeTipCatalogEntry{id:id.to_string(),geometry,filled:eo.get("filled").and_then(|v|v.as_bool()),scale:eo.get("scale").and_then(|v|v.as_f64())};
                let def = EdgeTipDef::from_catalog_entry(&entry).ok_or_else(|| NormalPortError::EdgeTipRowInvalid(id.to_string()))?;
                tips.insert(id.to_string(), def);
            }
            self.edge_tips = tips;
        }
        if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge kind"))?;
                reject_kind_catalog_row_legacy_label(eo, "edge")?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge kind"))?;
                let name = eo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color = eo.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(parse_css_hex_color);
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
                let source_tip = parse_catalog_tip_slot(eo.get("sourceTip").or_else(|| eo.get("source_tip")).and_then(|x| x.as_str()));
                let target_tip = parse_catalog_tip_slot(eo.get("targetTip").or_else(|| eo.get("target_tip")).and_then(|x| x.as_str()).or_else(|| eo.get("marker").and_then(|x| x.as_str())));
                let directed = eo.get("directed").and_then(|x| x.as_bool()).unwrap_or(true);
                next.insert(id.to_string(), EdgeKindDef { name, color, stroke_width, pattern, source_tip, target_tip, directed });
            }
            self.edge_kinds = next;
        }
        Ok(())
    }

    /// 🛡️ Ensures runtime catalogs declare every kind from a compile-time manifest.
    pub fn validate_against_manifest(&self, gm: &GraphManifest) -> Result<(), NormalPortError> {
        for row in &gm.port_kinds {
            let visual = row.presentation.as_ref().is_some_and(|p| p.get("color").is_some());
            if visual && !self.handle_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("handle", row.id.clone()));
            }
        }
        for row in &gm.wire_kinds {
            if !self.wire_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("wire", row.id.clone()));
            }
        }
        for row in &gm.edge_kinds {
            if row.presentation.is_some() && !self.edge_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("edge", row.id.clone()));
            }
        }
        for row in &gm.node_kinds {
            if row.presentation.is_some() && !self.node_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("node", row.id.clone()));
            }
        }
        Ok(())
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

    pub(crate) fn resolve_handle_fill_color(&self, h: &HandleData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
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

    pub fn resolve_node_fill_color(&self, n: &NodeData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        let theme_fill = node_fill_for_style(theme, kind);
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

    pub fn resolve_edge_tips<'a>(&'a self, e: &EdgeData) -> (Option<&'a EdgeTipDef>, Option<&'a EdgeTipDef>) {
        let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
        let source_slot = e.source_tip.as_deref().or_else(|| kind_def.and_then(|d| d.source_tip.as_deref()));
        let target_slot = e.target_tip.as_deref().or_else(|| kind_def.and_then(|d| d.target_tip.as_deref()));
        let source = self.resolve_tip_slot(source_slot);
        let mut target = self.resolve_tip_slot(target_slot);
        if target.is_none() && target_slot.is_none() {
            let directed = kind_def.is_none_or(|d| d.directed);
            if directed {
                target = self.lookup_edge_tip("arrow");
            }
        }
        (source, target)
    }

    pub fn resolve_default_wire_kind_for_handle(&self, h: &HandleData) -> String {
        self.handle_kinds.get(&h.handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    fn resolve_default_edge_kind_for_wire_kind(&self, wire_kind: &str) -> String {
        self.wire_kinds.get(wire_kind).and_then(|d| d.default_edge_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
    }

    pub fn default_edge_kind_for_created_link(&self, source: &HandleData, _target: &HandleData) -> String {
        let wk = self.resolve_default_wire_kind_for_handle(source);
        self.resolve_default_edge_kind_for_wire_kind(&wk)
    }

    fn resolve_default_wire_kind_for_handle_kind(&self, handle_kind: &str) -> String {
        self.handle_kinds.get(handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    pub fn board_fill_default_wire_kind<'a>(&'a self, handle_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
        let Some(value) = self.handle_kinds.get(handle_kind).and_then(|kind| kind.default_wire_kind.as_deref()) else {
            return Ok(DEFAULT_WIRE_KIND_ID);
        };
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        let value = value.trim();
        Ok(if value.is_empty() { DEFAULT_WIRE_KIND_ID } else { value })
    }

    pub fn board_fill_default_edge_kind<'a>(&'a self, wire_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
        let Some(value) = self.wire_kinds.get(wire_kind).and_then(|kind| kind.default_edge_kind.as_deref()) else {
            return Ok("");
        };
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        Ok(value.trim())
    }

    fn link_kinds_compatible_for_brush(&self, sn: &str, sh: &str, tn: &str, th: &str) -> bool {
        if !handle_port_shapes_compatible(sh, th) {
            return false;
        }
        if !single_letter_port_families_compatible(sh, th) {
            return false;
        }
        if self.link_compat_rules.is_empty() {
            return true;
        }
        let w_src = self.resolve_default_wire_kind_for_handle_kind(sh);
        let w_tgt = self.resolve_default_wire_kind_for_handle_kind(th);
        let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
        let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
        let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| link_gesture_rule_applies_kind_strings(rule, sn, sh, &w_src, &e_src, tn, th, &w_tgt, &e_tgt)).collect();
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
}

impl BoardSceneState {
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    /// 📷️ Reads the accepted viewport pose used by touch and pointer gesture planning.
    pub fn camera(&self) -> [f64; 3] {
        [self.camera.x, self.camera.y, self.camera.zoom]
    }

    pub fn resolve_element_kind_hover(&self, id: &str) -> Option<(String, String)> {
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

    pub fn handle_slot_center_world(&self, node_id: &str, hw: Point, offset: f64) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        let nc = Point::new(n.x, n.y);
        let normal = normalize_or_zero(hw - nc);
        Some(hw + normal * offset)
    }
}

pub fn current_draw_lod(scene: &BoardSceneState, style: &BoardStyleState) -> BoardDrawLod {
    if !style.automatic_lod {
        if let Some(lod) = style.forced_draw_lod {
            return lod;
        }
    }
    BoardDrawLod::from_scale_index(BOARD_LOD_SCALE.resolve_index(scene.camera.zoom))
}

pub(crate) fn board_lod_visible_grid_snap_step_world(scene: &BoardSceneState, style: &BoardStyleState) -> Option<f64> {
    match current_draw_lod(scene, style) {
        BoardDrawLod::Minimap => None,
        BoardDrawLod::Overview | BoardDrawLod::Compact => Some(style.grid_step_large_world()),
        BoardDrawLod::Normal => Some(style.grid_step_medium_world()),
        BoardDrawLod::Detail => Some(style.grid_step_small_world()),
        BoardDrawLod::Micro => Some(style.grid_step_micro_world()),
    }
}

fn board_snap_world_scalar(scene: &BoardSceneState, style: &BoardStyleState, v: f64) -> f64 {
    if !style.grid_snap_enabled {
        return v;
    }
    let Some(step) = board_lod_visible_grid_snap_step_world(scene, style) else {
        return v;
    };
    (v / step).round() * step
}

pub(crate) fn board_snap_world_pair(scene: &BoardSceneState, style: &BoardStyleState, x: f64, y: f64) -> (f64, f64) {
    (board_snap_world_scalar(scene, style, x), board_snap_world_scalar(scene, style, y))
}

/// 👁️ Shows or hides the world grid. Hiding it never changes snapping, which reads `grid_snap_enabled`.
pub fn board_set_grid_visible(scene: &mut BoardSceneState, style: &mut BoardStyleState, visible: bool) {
    if style.grid_visible != visible {
        style.grid_visible = visible;
        scene.content_scene_generation = scene.content_scene_generation.wrapping_add(1);
    }
}

pub fn handles_link_compatible_for_drag(scene: &BoardSceneState, style: &BoardStyleState, source: &HandleData, target: &HandleData) -> bool {
    if !handle_port_shapes_compatible(source.handle_kind.as_str(), target.handle_kind.as_str()) {
        return false;
    }
    if style.link_compat_rules.is_empty() {
        return true;
    }
    let mut matched: Vec<&LinkCompatRule> = style.link_compat_rules.iter().filter(|rule| link_gesture_rule_applies(scene, style, rule, source, target)).collect();
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

fn link_gesture_rule_applies(scene: &BoardSceneState, style: &BoardStyleState, rule: &LinkCompatRule, source: &HandleData, target: &HandleData) -> bool {
    let w_src = style.resolve_default_wire_kind_for_handle(source);
    let w_tgt = style.resolve_default_wire_kind_for_handle(target);
    let e_src = style.resolve_default_edge_kind_for_wire_kind(&w_src);
    let e_tgt = style.resolve_default_edge_kind_for_wire_kind(&w_tgt);
    let sn = scene.nodes.get(&source.node_id).map_or("", |n| n.node_kind.as_str());
    let tn = scene.nodes.get(&target.node_id).map_or("", |n| n.node_kind.as_str());
    let sh = source.handle_kind.as_str();
    let th = target.handle_kind.as_str();
    match rule.specificity {
        CompatSpecificity::General => compat_pair_matches(rule, sh, th),
        CompatSpecificity::Node => compat_pair_matches(rule, sn, tn),
        CompatSpecificity::Edge => compat_pair_matches(rule, e_src.as_str(), e_tgt.as_str()),
        CompatSpecificity::Handle => compat_pair_matches(rule, sh, th),
        CompatSpecificity::Wire => compat_pair_matches(rule, w_src.as_str(), th),
    }
}

/// 🖌️ Brush slot anchor follows indirect-handle layout at overview/normal LOD so hit targets match painted rings.
pub fn brush_handle_anchor_world(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<Point> {
    if matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
        indirect_handle_world_pos(scene, style, h).or_else(|| handle_world_pos(scene, style, h))
    } else {
        handle_world_pos(scene, style, h)
    }
}

pub fn brush_compatible_candidates(scene: &BoardSceneState, style: &BoardStyleState, source: &HandleData) -> Option<BrushCandidatePage> {
    let sn = scene.nodes.get(&source.node_id).map_or("", |n| n.node_kind.as_str());
    let sh = source.handle_kind.as_str();
    let mut out = BrushCandidatePage::default();
    for (kind_id, kind) in &style.node_kinds {
        if kind.handles.is_empty() {
            continue;
        }
        let tn = kind_id.as_str();
        for (i, tmpl) in kind.handles.iter().enumerate() {
            if style.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                let delta = brush_handle_alignment_delta(source.angle, tmpl.angle);
                out.push(kind_id, i, delta).ok()?;
            }
        }
    }
    out.sort();
    Some(out)
}

impl BoardSceneState {
    pub fn commit_pointer(&mut self, plan: &BoardPointerPlan) -> bool {
        if self.interaction_revision != plan.revision {
            return false;
        }
        if !matches!(plan.kind, BoardPointerPlanKind::Idle) {
            return false;
        }
        self.interaction_revision = plan.revision.wrapping_add(1);
        true
    }
}

impl BoardStyleState {
    pub fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
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
}

/// 🧲️ The world step every region gesture quantizes to, or `None` while the grid-snap
/// modifier is off — one definition shared by paint, move and resize.
pub fn region_snap_step(scene: &BoardSceneState, style: &BoardStyleState) -> Option<f64> {
    if !style.grid_snap_enabled {
        return None;
    }
    board_lod_visible_grid_snap_step_world(scene, style)
}

pub fn snap_region_point(scene: &BoardSceneState, style: &BoardStyleState, point: Point) -> Point {
    let step = region_snap_step(scene, style);
    Point::new(snap_region_scalar(point.x, step), snap_region_scalar(point.y, step))
}

pub const BOARD_FILL_TEXT_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCaptureFault {
    TextCapacity,
    NodeCapacity,
    HandleCapacity,
    KindCapacity,
    KindHandleCapacity,
    RuleCapacity,
    StaleNode,
    StaleHandle,
    StaleKind,
    StaleRule,
    GenerationExhausted,
}

/// 🧩️ BoardHost scene component: entities, camera, viewport and the cached world content of one board.
pub struct BoardSceneState {
    pub camera: Camera,
    pub nodes: BTreeMap<String, NodeData>,
    pub handles: BTreeMap<String, HandleData>,
    pub edges: BTreeMap<String, EdgeData>,
    pub wires: BTreeMap<String, WireData>,
    /// 🎯️ Fill-constraining rectangles, keyed by document id. Painted beneath every entity,
    /// hit-tested after all of them, and counted against the descriptor census like any other entity.
    pub regions: BTreeMap<String, RegionData>,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
    /// 🧿️ Bumped when drawable content changes (not camera); keys {@link BoardHost.world_content_cache}.
    pub content_scene_generation: u64,
    /// 🎨️ World-space node icons reused across pan/zoom. Keyed by generation and LOD; a pending
    /// retirement keeps the previous bake and the draw shifts each span onto the live node center.
    pub world_content_cache: RefCell<Option<WorldContentCache>>,
    pub interaction_revision: u64,
}

/// 🧩️ BoardHost style component: style catalogs, themes, LOD and grid policy.
pub struct BoardStyleState {
    /// Catalog keyed by `handle_kind` id (see `set_board_kind_catalogs_from_json`).
    pub handle_kinds: BTreeMap<String, HandleKindDef>,
    pub wire_kinds: BTreeMap<String, WireKindDef>,
    pub node_kinds: BTreeMap<String, NodeKindDef>,
    pub edge_kinds: BTreeMap<String, EdgeKindDef>,
    /// 🔺️ Registry of edge tip shapes keyed by catalog id (built-ins seeded at init).
    pub edge_tips: BTreeMap<String, EdgeTipDef>,
    /// 🔗️ Kind-compatibility rules for link gestures; empty = unrestricted.
    pub link_compat_rules: Vec<LinkCompatRule>,
    pub world_raster_tiling: String,
    pub canvas_theme: CanvasPalette,
    /// 👁️ When false, the LOD world grid is not stroked at all (snapping is unaffected — that is `grid_snap_enabled`).
    pub grid_visible: bool,
    /// 📐️ Positive multiplier for LOD world grid steps (`10` / `5` / `1` base world units per band).
    pub grid_factor: f64,
    /// 🧲️ When true, node drags snap to the finest visible LOD grid (step scales with `grid_factor`).
    pub grid_snap_enabled: bool,
    pub preserve_original_element_style: bool,
    /// 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier when set.
    pub automatic_lod: bool,
    pub forced_draw_lod: Option<BoardDrawLod>,
    pub icon_paint_cache: IconPaintCache,
    /// 🔍️ True while the wheel zoom gesture is active (skip grid + per-tile rebuild hot paths).
    pub wheel_zoom_active: bool,
    /// 📶️ LOD tier pinned for the active wheel gesture so pan/zoom does not rebuild {@link BoardHost.world_content_cache} on every band crossing.
    pub wheel_zoom_render_lod: Option<BoardDrawLod>,
    /// 📛️ Placement naming the owning domain installed ([`BoardHost::set_naming`]); [`NEUTRAL_BOARD_NAMING`] until then.
    pub naming: &'static BoardNaming,
    /// 📨️ Brush prefixes received as scene data ([`BoardHost::set_placement_naming_json`]); they override `naming`'s brush prefixes.
    pub placement_naming: Option<BoardPlacementNaming>,
    pub port_mode: GraphPortMode,
}

#[derive(Clone, Debug)]
pub struct BoardWheelPlan {
    pub revision: u64,
    pub expected: Camera,
    pub next: Camera,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardPointerPlanFault {
    ItemCredits,
    ByteCredits,
    Unsupported,
}

#[derive(Clone, Copy, Debug)]
struct BoardPointerDelta {
    pub id: BoardPointerSpan,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardDeleteKind {
    Edge,
    Node,
    Handle,
    Wire,
}

#[derive(Clone, Copy, Debug)]
struct BoardDeleteEntry {
    pub kind: BoardDeleteKind,
    pub id: BoardPointerSpan,
}

pub struct BoardDeletePlan {
    pub revision: u64,
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub entries: Box<[Option<BoardDeleteEntry>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub len: u16,
    property_nodes: u16,
    property_bytes: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardDeletePlanningPhase {
    SelectedEdges,
    Nodes,
    DiscoverNode,
    Handles,
    Wires,
    Edges,
    Handle,
    Node,
    SelectionEvent,
    Finish,
}

pub struct BoardDeletePlanningOperation {
    pub plan: BoardDeletePlan,
    pub phase: BoardDeletePlanningPhase,
    pub scan_after: Option<String>,
    pub node_after: Option<String>,
    pub handle_after: Option<String>,
    pub relation_after: Option<String>,
    pub node_id: Option<String>,
    pub handle_id: Option<String>,
    pub node_relevant: bool,
    pub property_audit: Option<BoardPropertyAudit>,
    pub select_builder: BoardPayloadBuilder,
    pub select_first: bool,
    pub fault: Option<BoardEventFault>,
    pub cancelling: bool,
}

enum BoardPropertyAuditFrame {
    Array { values: Vec<semio_framework_graph_manifest::PropertyValue>, index: usize, pending: Option<usize> },
    Object { values: semio_framework_graph_manifest::PropertyBag, after: Option<String>, pending: Option<String> },
}

pub(crate) struct BoardPropertyAudit {
    pub kind: BoardDeleteKind,
    pub id: String,
    pub current: Option<semio_framework_graph_manifest::PropertyValue>,
    pub completed: Option<semio_framework_graph_manifest::PropertyValue>,
    pub stack: Box<[Option<BoardPropertyAuditFrame>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub depth: u16,
    pub nodes: usize,
    pub bytes: usize,
    pub fault: Option<BoardEventFault>,
}

impl BoardDeletePlan {
    pub fn new(revision: u64) -> Self {
        Self { revision, bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0, property_nodes: 0, property_bytes: 0 }
    }

    pub fn id(&self, span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("board deletion ids originate from UTF-8 strings")
    }

    pub fn contains(&self, kind: BoardDeleteKind, id: &str) -> bool {
        self.entries[..usize::from(self.len)].iter().flatten().any(|entry| entry.kind == kind && self.id(entry.id) == id)
    }

    pub fn removes_selection_id(&self, id: &str) -> bool {
        self.entries[..usize::from(self.len)].iter().flatten().any(|entry| self.id(entry.id) == id)
    }

    pub fn push(&mut self, kind: BoardDeleteKind, id: &str) -> Result<(), BoardEventFault> {
        if self.contains(kind, id) {
            return Ok(());
        }
        if matches!(kind, BoardDeleteKind::Edge | BoardDeleteKind::Node) && 7usize.checked_add(board_json_string_bytes(id)).is_none_or(|bytes| bytes > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY) {
            return Err(BoardEventFault::ByteCredits);
        }
        let index = usize::from(self.len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        if id.len() > u16::MAX as usize {
            return Err(BoardEventFault::ByteCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(id.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(id.as_bytes());
        self.byte_len = end as u16;
        self.entries[index] = Some(BoardDeleteEntry { kind, id: BoardPointerSpan { start: start as u16, len: id.len() as u16 } });
        self.len += 1;
        Ok(())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn push_entity(&mut self, kind: BoardDeleteKind, id: &str, properties: &semio_framework_graph_manifest::PropertyBag) -> Result<(), BoardEventFault> {
        if self.contains(kind, id) {
            return Ok(());
        }
        let (nodes, bytes) = board_property_retirement_credits(properties)?;
        let next_nodes = usize::from(self.property_nodes).checked_add(nodes).ok_or(BoardEventFault::ItemCredits)?;
        let next_bytes = usize::from(self.property_bytes).checked_add(bytes).ok_or(BoardEventFault::ByteCredits)?;
        if next_nodes > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        if next_bytes > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.push(kind, id)?;
        self.property_nodes = next_nodes as u16;
        self.property_bytes = next_bytes as u16;
        Ok(())
    }

    pub fn push_admitted_entity(&mut self, kind: BoardDeleteKind, id: &str, property_nodes: usize, property_bytes: usize) -> Result<(), BoardEventFault> {
        if self.contains(kind, id) {
            return Ok(());
        }
        let next_nodes = usize::from(self.property_nodes).checked_add(property_nodes).ok_or(BoardEventFault::ItemCredits)?;
        let next_bytes = usize::from(self.property_bytes).checked_add(property_bytes).ok_or(BoardEventFault::ByteCredits)?;
        if next_nodes > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        if next_bytes > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.push(kind, id)?;
        self.property_nodes = next_nodes as u16;
        self.property_bytes = next_bytes as u16;
        Ok(())
    }

    pub fn emitted_event_count(&self) -> usize {
        self.entries[..usize::from(self.len)].iter().flatten().filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node)).count()
    }

    pub fn emitted_event_bytes(&self) -> Option<usize> {
        self.entries[..usize::from(self.len)]
            .iter()
            .flatten()
            .filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node))
            .try_fold(0usize, |bytes, entry| bytes.checked_add(7)?.checked_add(board_json_string_bytes(self.id(entry.id))))
    }
}

impl BoardPropertyAudit {
    pub fn new(kind: BoardDeleteKind, id: String, properties: semio_framework_graph_manifest::PropertyBag) -> Self {
        Self { kind, id, current: Some(semio_framework_graph_manifest::PropertyValue::Object(properties)), completed: None, stack: Box::new(std::array::from_fn(|_| None)), depth: 0, nodes: 0, bytes: 0, fault: None }
    }

    fn add_bytes(&mut self, bytes: usize) {
        self.bytes = self.bytes.saturating_add(bytes);
        if self.bytes > BOARD_POINTER_BYTE_CAPACITY {
            self.fault.get_or_insert(BoardEventFault::ByteCredits);
        }
    }

    fn push_frame(&mut self, frame: BoardPropertyAuditFrame) -> Result<(), BoardPropertyAuditFrame> {
        let index = usize::from(self.depth);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(frame);
        }
        self.stack[index] = Some(frame);
        self.depth += 1;
        Ok(())
    }

    pub fn step(&mut self) -> bool {
        if let Some(value) = self.completed.take() {
            if self.depth == 0 {
                self.completed = Some(value);
                return true;
            }
            let frame = self.stack[usize::from(self.depth - 1)].as_mut().expect("property audit frame");
            match frame {
                BoardPropertyAuditFrame::Array { values, pending, .. } => {
                    let index = pending.take().expect("array audit child index");
                    values[index] = value;
                }
                BoardPropertyAuditFrame::Object { values, pending, .. } => {
                    let key = pending.take().expect("object audit child key");
                    *values.get_mut(&key).expect("object audit key remains reserved") = value;
                }
            }
            return false;
        }
        if let Some(value) = self.current.take() {
            self.nodes = self.nodes.saturating_add(1);
            if self.nodes > BOARD_POINTER_ITEM_CAPACITY {
                self.fault.get_or_insert(BoardEventFault::ItemCredits);
            }
            if self.fault.is_some() {
                self.completed = Some(value);
                return false;
            }
            match value {
                semio_framework_graph_manifest::PropertyValue::Array(values) => {
                    let frame = BoardPropertyAuditFrame::Array { values, index: 0, pending: None };
                    if let Err(frame) = self.push_frame(frame) {
                        self.fault = Some(BoardEventFault::ItemCredits);
                        self.completed = Some(match frame {
                            BoardPropertyAuditFrame::Array { values, .. } => semio_framework_graph_manifest::PropertyValue::Array(values),
                            BoardPropertyAuditFrame::Object { .. } => unreachable!(),
                        });
                    }
                }
                semio_framework_graph_manifest::PropertyValue::Object(values) => {
                    let frame = BoardPropertyAuditFrame::Object { values, after: None, pending: None };
                    if let Err(frame) = self.push_frame(frame) {
                        self.fault = Some(BoardEventFault::ItemCredits);
                        self.completed = Some(match frame {
                            BoardPropertyAuditFrame::Object { values, .. } => semio_framework_graph_manifest::PropertyValue::Object(values),
                            BoardPropertyAuditFrame::Array { .. } => unreachable!(),
                        });
                    }
                }
                semio_framework_graph_manifest::PropertyValue::String(value) => {
                    self.add_bytes(value.len());
                    self.completed = Some(semio_framework_graph_manifest::PropertyValue::String(value));
                }
                scalar => self.completed = Some(scalar),
            }
            return false;
        }
        if self.depth == 0 {
            return false;
        }
        let index = usize::from(self.depth - 1);
        if self.fault.is_some() {
            let frame = self.stack[index].take().expect("faulted property audit frame");
            self.depth -= 1;
            self.completed = Some(match frame {
                BoardPropertyAuditFrame::Array { values, .. } => semio_framework_graph_manifest::PropertyValue::Array(values),
                BoardPropertyAuditFrame::Object { values, .. } => semio_framework_graph_manifest::PropertyValue::Object(values),
            });
            return false;
        }
        let frame = self.stack[index].take().expect("property audit frame");
        match frame {
            BoardPropertyAuditFrame::Array { mut values, mut index, pending: _ } => {
                if index == values.len() {
                    self.depth -= 1;
                    self.completed = Some(semio_framework_graph_manifest::PropertyValue::Array(values));
                } else {
                    let child_index = index;
                    index += 1;
                    let pending = Some(child_index);
                    self.current = Some(std::mem::replace(&mut values[child_index], semio_framework_graph_manifest::PropertyValue::Null));
                    self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Array { values, index, pending });
                }
            }
            BoardPropertyAuditFrame::Object { mut values, mut after, mut pending } => {
                let next = match after.as_ref() {
                    Some(after) => values.successor(after),
                    None => values.first_key_value(),
                }
                .map(|(key, _)| admitted_board_pointer_id(key));
                match next {
                    Some(Ok(key)) => {
                        self.bytes = self.bytes.saturating_add(key.len());
                        if self.bytes > BOARD_POINTER_BYTE_CAPACITY {
                            self.fault.get_or_insert(BoardEventFault::ByteCredits);
                        }
                        after = Some(key.clone());
                        pending = Some(key.clone());
                        self.current = Some(std::mem::replace(values.get_mut(&key).expect("property audit key"), semio_framework_graph_manifest::PropertyValue::Null));
                        self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Object { values, after, pending });
                    }
                    Some(Err(fault)) => {
                        self.fault = Some(fault);
                        self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Object { values, after, pending });
                    }
                    None => {
                        self.depth -= 1;
                        self.completed = Some(semio_framework_graph_manifest::PropertyValue::Object(values));
                    }
                }
            }
        }
        false
    }

    pub fn take_result(&mut self) -> Option<(BoardDeleteKind, String, semio_framework_graph_manifest::PropertyBag, usize, usize, Option<BoardEventFault>)> {
        if self.depth != 0 || self.current.is_some() {
            return None;
        }
        let semio_framework_graph_manifest::PropertyValue::Object(properties) = self.completed.take()? else {
            self.fault = Some(BoardEventFault::Schema);
            return None;
        };
        Some((self.kind, std::mem::take(&mut self.id), properties, self.nodes, self.bytes, self.fault.take()))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BoardPointerPlanKind {
    Idle,
    Pan {
        camera: [f64; 3],
    },
    DragMove,
    FinishPan {
        camera: [f64; 3],
    },
    FinishDrag,
    SelectionPreview {
        start: Point,
        start_screen: Point,
    },
    SelectionCommit,
    LinkMove {
        source: BoardPointerSpan,
        target: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        compat_key: BoardPointerSpan,
        ring_key: BoardPointerSpan,
        end_world: Point,
        activated: bool,
        start_screen: Point,
    },
    LinkFinish {
        source: BoardPointerSpan,
        target: Option<BoardPointerSpan>,
        edge: Option<BoardPointerSpan>,
        target_node: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        compat_key: Option<BoardPointerSpan>,
        ring_key: Option<BoardPointerSpan>,
    },
    LinkRetain {
        hover: Option<BoardPointerSpan>,
    },
    Hover {
        hover: Option<BoardPointerSpan>,
    },
    Brush {
        source: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        alt: bool,
        commit_old: bool,
    },
}

#[derive(Debug)]
pub struct BoardPointerPlan {
    pub revision: u64,
    pub kind: BoardPointerPlanKind,
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub deltas: Box<[Option<BoardPointerDelta>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub delta_len: u16,
    pub points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
    pub screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
    pub point_len: u16,
    pub output: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub output_len: u16,
}

impl BoardPointerPlan {
    pub fn empty(revision: u64, kind: BoardPointerPlanKind) -> Self {
        Self {
            revision,
            kind,
            bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
            byte_len: 0,
            deltas: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]),
            delta_len: 0,
            points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
            screen_points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
            point_len: 0,
            output: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
            output_len: 0,
        }
    }

    pub fn push_id(&mut self, id: &str) -> Result<BoardPointerSpan, BoardPointerPlanFault> {
        if id.len() > u16::MAX as usize {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(id.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(id.as_bytes());
        self.byte_len = end as u16;
        Ok(BoardPointerSpan { start: start as u16, len: id.len() as u16 })
    }

    pub fn push_delta(&mut self, id: &str, x: f64, y: f64) -> Result<(), BoardPointerPlanFault> {
        let index = usize::from(self.delta_len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        let id = self.push_id(id)?;
        self.deltas[index] = Some(BoardPointerDelta { id, x, y });
        self.delta_len += 1;
        Ok(())
    }

    pub fn id(&self, span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("board pointer ids originate from UTF-8 strings")
    }

    pub(crate) fn push_point(&mut self, world: Point, screen: Point) -> Result<(), BoardPointerPlanFault> {
        let index = usize::from(self.point_len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        self.points[index] = world;
        self.screen_points[index] = screen;
        self.point_len += 1;
        Ok(())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn selection_ids(&self) -> impl Iterator<Item = &str> {
        self.deltas[..usize::from(self.delta_len)].iter().flatten().map(|delta| self.id(delta.id))
    }

    /// 🎥️ The camera this plan settles on the view lane: a finished pan's. A camera move is view state, so the host
    /// publishes it as its view verb (`setCamera`) and never as a board event.
    pub fn view_camera(&self) -> Option<[f64; 3]> {
        match self.kind {
            BoardPointerPlanKind::FinishPan { camera } => Some(camera),
            _ => None,
        }
    }

    /// 📮️ Whether committing this plan puts anything on the host's action queue: board events or a view camera.
    pub fn publishes(&self) -> bool {
        self.event_count() > 0 || self.view_camera().is_some()
    }

    pub fn event_count(&self) -> usize {
        match self.kind {
            BoardPointerPlanKind::FinishDrag => usize::from(self.output_len > 2),
            BoardPointerPlanKind::SelectionPreview { .. } | BoardPointerPlanKind::SelectionCommit => 1,
            BoardPointerPlanKind::LinkMove { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::LinkFinish { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::LinkRetain { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::Hover { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::Brush { .. } => usize::from(self.output_len > 2),
            _ => 0,
        }
    }

    pub fn requires_retained_commit(&self) -> bool {
        !matches!(self.kind, BoardPointerPlanKind::Idle)
    }

    /// 🈳️ Seals the empty event page of a hand-built plan that publishes no board event.
    #[cfg(any(test, feature = "testkit"))]
    pub fn seal_events(&mut self) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[]")
    }

    fn output_raw(&mut self, value: &str) -> Result<(), BoardPointerPlanFault> {
        let start = usize::from(self.output_len);
        let end = start.checked_add(value.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        self.output[start..end].copy_from_slice(value.as_bytes());
        self.output_len = end as u16;
        Ok(())
    }

    pub(crate) fn seal_owned_event(&mut self, event: &BoardOwnedEvent) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[{\"name\":\"")?;
        self.output_raw(event.kind().name())?;
        self.output_raw("\",\"payload\":")?;
        self.output_raw(event.payload_json())?;
        self.output_raw("}]")
    }

    pub fn seal_optional_events<const N: usize>(&mut self, events: &[Option<BoardOwnedEvent>; N]) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[")?;
        for (emitted, event) in events.iter().flatten().enumerate() {
            if emitted > 0 {
                self.output_raw(",")?;
            }
            self.output_raw("{\"name\":\"")?;
            self.output_raw(event.kind().name())?;
            self.output_raw("\",\"payload\":")?;
            self.output_raw(event.payload_json())?;
            self.output_raw("}")?;
        }
        self.output_raw("]")
    }

    pub fn events_json(&self) -> &str {
        std::str::from_utf8(&self.output[..usize::from(self.output_len)]).expect("board event page is encoded from UTF-8 schema tokens")
    }

}

pub fn board_json_string_bytes(value: &str) -> usize {
    value
        .chars()
        .map(|character| match character {
            '"' | '\\' | '\n' | '\r' | '\t' => 2,
            character if character <= '\u{1f}' => 6,
            character => character.len_utf8(),
        })
        .try_fold(2usize, usize::checked_add)
        .unwrap_or(usize::MAX)
}

pub fn admitted_board_pointer_id(value: &str) -> Result<String, BoardEventFault> {
    if value.len() > BOARD_POINTER_BYTE_CAPACITY {
        return Err(BoardEventFault::ByteCredits);
    }
    Ok(value.to_owned())
}

impl BoardWheelPlan {
    pub fn camera(&self) -> [f64; 3] {
        [self.next.x, self.next.y, self.next.zoom]
    }
}
// #endregion board_host

impl BoardSceneState {
    /// 📍️ Applies peer-pane node drags without a full descriptor re-sync.
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

    /// 📍️ Parses `[{"id","x","y"},…]` and updates existing host nodes in place.
    pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        #[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
        struct NodePositionMoveJson {
            id: String,
            x: f64,
            y: f64,
        }
        let rows: Vec<NodePositionMoveJson> = serde_json::from_str(json)?;
        let moves: Vec<(String, f64, f64)> = rows.into_iter().map(|row| (row.id, row.x, row.y)).collect();
        self.set_node_positions(&moves);
        Ok(())
    }

    pub fn drawable_cull_pad_world(&self) -> f64 {
        16.0 / self.camera.zoom.max(1e-9)
    }

    pub fn stroke_world_step_grid(&self, scene: &mut Scene, color: Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
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
        let mut p = semio_framework_canvas::BezPath::new();
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

    pub fn draw_space_point(&self, world: Point, world_space: bool) -> Point {
        if world_space {
            world
        } else {
            self.world_to_screen(world)
        }
    }

    pub fn draw_space_len(&self, len_world: f64, world_space: bool) -> f64 {
        if world_space {
            len_world.max(1e-9)
        } else {
            (len_world * self.camera.zoom).max(1.0)
        }
    }

    /// 📏️ Screen-pixel edge stroke width (world-clip tiles and post-cache overlay).
    pub fn edge_screen_stroke_width_px(&self, lod: BoardDrawLod) -> f64 {
        match lod {
            BoardDrawLod::Minimap => ui_styling::strokes::EDGE_MINIMAP,
            BoardDrawLod::Overview | BoardDrawLod::Compact => (ui_styling::strokes::EDGE_OVERVIEW).max(ui_styling::strokes::EDGE_BASE * self.camera.zoom),
            _ => 2.0 * self.camera.zoom.max(0.75),
        }
    }

    /// 📏️ Edge stroke in world units so {@link BoardHost.camera_content_affine} yields ~{@link Self::edge_screen_stroke_width_px}.
    pub fn edge_world_stroke_width(&self, lod: BoardDrawLod) -> f64 {
        let screen_px = self.edge_screen_stroke_width_px(lod);
        let z = self.camera.zoom.max(1e-9);
        (screen_px / z).max(1e-3)
    }

    /// 🗺️ Draws baked icon spans at each node's live center. A pending cache retirement keeps the
    /// previous bake; the translation is the whole correction a drag needs.
    pub fn append_baked_node_icons(&self, scene: &mut Scene, cache: &WorldContentCache, cam_aff: Affine) {
        if cache.spans.is_empty() {
            scene.append(&cache.scene, Some(cam_aff));
            return;
        }
        for span in &cache.spans {
            let Some(node) = self.nodes.get(&span.id) else { continue };
            if !node.visible {
                continue;
            }
            let dx = node.x - span.x;
            let dy = node.y - span.y;
            let placed = if dx.abs() <= 1e-9 && dy.abs() <= 1e-9 { cam_aff } else { cam_aff * Affine::IDENTITY.translate((dx, dy)) };
            scene.append_range(&cache.scene, span.start, span.end, Some(placed));
        }
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_y: f64) -> BoardWheelPlan {
        let mut next = self.camera.clone();
        semio_framework_canvas::camera::wheel_screen(&mut next, &self.viewport(), sx, sy, delta_y);
        BoardWheelPlan { revision: self.interaction_revision, expected: self.camera.clone(), next }
    }

    pub fn begin_delete_property_audit(&mut self, planning: &mut BoardDeletePlanningOperation, kind: BoardDeleteKind, id: &str) {
        if planning.plan.contains(kind, id) {
            return;
        }
        if id.len() > BOARD_POINTER_BYTE_CAPACITY.saturating_sub(usize::from(planning.plan.byte_len)) {
            planning.fault = Some(BoardEventFault::ByteCredits);
            return;
        }
        let properties = match kind {
            BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
        };
        let Some(properties) = properties else {
            planning.fault = Some(BoardEventFault::Schema);
            return;
        };
        planning.property_audit = Some(BoardPropertyAudit::new(kind, id.to_owned(), properties));
    }

    pub fn restore_delete_properties(&mut self, kind: BoardDeleteKind, id: &str, properties: semio_framework_graph_manifest::PropertyBag) -> bool {
        match kind {
            BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| entity.properties = properties),
            BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| entity.properties = properties),
        }
        .is_some()
    }

    /// 🔗️ True when any edge uses this handle as `source` or `target` (handle already participates in a link).
    pub fn handle_has_incident_edge(&self, handle_id: &str) -> bool {
        self.edges.values().any(|e| e.source == handle_id || e.target == handle_id)
    }

    fn node_has_any_incident_edge(&self, node_id: &str) -> bool {
        self.handles.values().filter(|h| h.node_id == node_id).any(|h| self.handle_has_incident_edge(h.id.as_str()))
    }

    fn node_effectively_visible(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible)
    }

    pub fn node_selectable(&self, node_id: &str) -> bool {
        self.nodes.get(node_id).is_some_and(|n| n.visible && !n.locked)
    }

    pub fn handle_selectable(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && !h.locked && self.node_selectable(h.node_id.as_str()))
    }

    pub fn wire_selectable(&self, wire: &WireData) -> bool {
        if !wire.visible || wire.locked {
            return false;
        }
        if !self.handle_selectable(wire.source.as_str()) {
            return false;
        }
        wire.target.as_ref().is_none_or(|id| self.handle_selectable(id.as_str()))
    }

    pub fn handle_effectively_visible(&self, handle_id: &str) -> bool {
        self.handles.get(handle_id).is_some_and(|h| h.visible && self.node_effectively_visible(h.node_id.as_str()))
    }

    pub fn wire_effectively_visible(&self, wire: &WireData) -> bool {
        wire.visible && self.handle_effectively_visible(wire.source.as_str()) && wire.target.as_ref().is_none_or(|id| self.handle_effectively_visible(id.as_str()))
    }

    /// 💫️ True when the handle may be drawn or hit-tested on the indirect-connect ghost ring (`overview`/`normal` LOD).
    pub fn handle_eligible_indirect_connect_ring(&self, handle_id: &str) -> bool {
        self.handle_selectable(handle_id) && !self.handle_has_incident_edge(handle_id)
    }

    pub fn build_selection_pointer_plan(&self, kind: BoardPointerPlanKind, next: &BTreeSet<String>, anchor: &BTreeSet<String>, gesture: Option<&str>, points: &[Point], screen_points: &[Point]) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
        if next.len() > BOARD_POINTER_ITEM_CAPACITY || anchor.len() > BOARD_POINTER_ITEM_CAPACITY || points.len() != screen_points.len() || points.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        if anchor.iter().map(String::len).try_fold(0usize, usize::checked_add).is_none_or(|bytes| bytes > BOARD_POINTER_BYTE_CAPACITY) {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        let mut plan = BoardPointerPlan::empty(self.interaction_revision, kind);
        for id in next {
            plan.push_delta(id, 0.0, 0.0)?;
        }
        for (world, screen) in points.iter().copied().zip(screen_points.iter().copied()) {
            plan.push_point(world, screen)?;
        }
        let event = match kind {
            BoardPointerPlanKind::SelectionPreview { .. } => {
                let removed: BTreeSet<String> = anchor.difference(next).cloned().collect();
                BoardOwnedEvent::preselect_sets(next, &removed, gesture)
            }
            BoardPointerPlanKind::SelectionCommit => BoardOwnedEvent::select_set(next, Some(anchor), gesture),
            _ => return Err(BoardPointerPlanFault::Unsupported),
        }
        .map_err(|fault| match fault {
            BoardEventFault::ItemCredits => BoardPointerPlanFault::ItemCredits,
            BoardEventFault::ByteCredits | BoardEventFault::KeyCredits | BoardEventFault::Schema => BoardPointerPlanFault::ByteCredits,
        })?;
        plan.seal_owned_event(&event)?;
        Ok(plan)
    }
}

pub fn handle_world_bounds_cull(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<WorldBox> {
    let pos = handle_world_pos(scene, style, h)?;
    let pad = scene.drawable_cull_pad_world() + effective_handle_radius(scene, style, h).max(1.0);
    Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
}

pub fn indirect_handle_world_bounds_cull(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<WorldBox> {
    let pos = indirect_handle_world_pos(scene, style, h)?;
    let pad = scene.drawable_cull_pad_world() + indirect_handle_marker_radius_world(scene, style, h).max(1.0);
    Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
}

pub fn edge_world_bounds_for_cull(scene: &BoardSceneState, style: &BoardStyleState, e: &EdgeData) -> Option<WorldBox> {
    let c = board_edge_curve(scene, style, e)?;
    let axis = cubic_bezier_axis_bounds(c);
    let half_w_world = scene.camera.zoom.max(0.75) / scene.camera.zoom.max(1e-9);
    Some(inflate_world_box(axis, half_w_world + scene.drawable_cull_pad_world()))
}

pub fn append_handle_marker(
    scene_state: &BoardSceneState, style: &BoardStyleState,
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
    let c = scene_state.draw_space_point(center, world_space);
    let r = scene_state.draw_space_len(radius_world, world_space);
    let (fill, stroke_c, stroke_px) =
        if let Some((f, s, sw)) = paint_override { (f, s, sw) } else { (style.resolve_handle_fill_color(h, &style.canvas_theme, style_kind), resolve_handle_stroke_color(h, &style.canvas_theme, style_kind), 2.0_f64) };
    let paint_fill = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill);
    let paint_stroke = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
    let paint_icons = draw_icon && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
    let outward = if exterior_cap { scene_state.nodes.get(h.node_id.as_str()).and_then(|n| handle_outward_at_node_rim(center, Point::new(n.x, n.y), n.shape, n.radius, n.width, n.height)) } else { None };
    if paint_fill {
        if let Some(out) = outward {
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &handle_exterior_cap_fill_path(c, out, r));
        } else {
            scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &Circle::new(c, r));
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
            let preserve_original_style = style.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
            let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&style.canvas_theme);
            if let Some(paint) = style.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                let (bx, by, bw, bh) = paint.bounds();
                let fit_inset = 0.62;
                let s = scene_state.draw_space_len(radius_world, world_space) * fit_inset;
                let cx = bx + bw * 0.5;
                let cy = by + bh * 0.5;
                let avail = 2.0 * s;
                let scale = (avail / bw).min(avail / bh);
                let aff = Affine::IDENTITY.translate((c.x - scale * cx, c.y - scale * cy)) * Affine::IDENTITY.scale(scale);
                let r_clip = scene_state.draw_space_len(radius_world, world_space) * 0.82;
                let disc = Circle::new(c, r_clip);
                scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                match paint.body() {
                    CachedIconBody::Vector(icon_scene) => {
                        scene.append(icon_scene, Some(aff));
                    }
                    CachedIconBody::Raster(img) => {
                        scene.draw_image(img, aff);
                    }
                }
                scene.pop_layer();
            }
        }
    }
}

pub fn paint_node_icon(scene_state: &BoardSceneState, style: &BoardStyleState, scene: &mut Scene, n: &NodeData, world_space: bool, style_kind: BoardElementStyleKind, circle_clip: Option<&Circle>, rect_clip: Option<Rect>) {
    if let Some(k) = n.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let preserve_original_style = style.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
        let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&style.canvas_theme);
        if let Some(paint) = style.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
            let (bx, by, bw, bh) = paint.bounds();
            let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
            let fit_inset = ui_styling::metrics::icon::FIT_INSET;
            let (sx_half, sy_half) = match n.shape {
                NodeShape::Circle => {
                    let s = scene_state.draw_space_len(style.scaled_node_radius(n), world_space) * fit_inset;
                    (s, s)
                }
                NodeShape::Rectangle => (scene_state.draw_space_len(style.scaled_node_width(n), world_space) * fit_inset * 0.5, scene_state.draw_space_len(style.scaled_node_height(n), world_space) * fit_inset * 0.5),
            };
            let center = scene_state.draw_space_point(Point::new(n.x, n.y), world_space);
            let cx = bx + bw * 0.5;
            let cy = by + bh * 0.5;
            let avail_w = 2.0 * sx_half;
            let avail_h = 2.0 * sy_half;
            let scale = (avail_w / bw).min(avail_h / bh);
            let aff = Affine::IDENTITY.translate((center.x - scale * cx, center.y - scale * cy)) * Affine::IDENTITY.scale(scale);
            match n.shape {
                NodeShape::Circle => {
                    let r_clip = scene_state.draw_space_len(style.scaled_node_radius(n), world_space) * clip_inset;
                    let disc = circle_clip.copied().unwrap_or_else(|| Circle::new(center, r_clip));
                    scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                    match paint.body() {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(img, aff);
                        }
                    }
                    scene.pop_layer();
                }
                NodeShape::Rectangle => {
                    let hw = scene_state.draw_space_len(style.scaled_node_width(n), world_space) * clip_inset * 0.5;
                    let hh = scene_state.draw_space_len(style.scaled_node_height(n), world_space) * clip_inset * 0.5;
                    let clip_r = rect_clip.unwrap_or_else(|| Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh)));
                    scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
                    match paint.body() {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(img, aff);
                        }
                    }
                    scene.pop_layer();
                }
            }
        }
    }
}

pub fn board_set_wheel_zoom_active(scene: &mut BoardSceneState, style: &mut BoardStyleState, active: bool) {
    if active && !style.wheel_zoom_active {
        style.wheel_zoom_render_lod = Some(current_draw_lod(scene, style));
    }
    if !active {
        style.wheel_zoom_render_lod = None;
    }
    style.wheel_zoom_active = active;
}

fn link_snap_drag_tolerance_screen(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    let z = scene.camera.zoom.max(1e-9);
    HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX + effective_handle_radius(scene, style, h) * z
}

pub fn link_snap_commit_proximity_ok(scene: &BoardSceneState, style: &BoardStyleState, target_handle_id: &str, world: Point) -> bool {
    let Some(h) = scene.handles.get(target_handle_id) else {
        return false;
    };
    if !scene.handle_selectable(target_handle_id) {
        return false;
    }
    let Some(pw) = handle_world_pos(scene, style, h) else {
        return false;
    };
    let z = scene.camera.zoom.max(1e-9);
    let d_screen = distance_between(scene.world_to_screen(world), scene.world_to_screen(pw));
    let tol_commit = HANDLE_HIT_TOLERANCE_PX + LINK_COMMIT_SNAP_TIGHT_PX + effective_handle_radius(scene, style, h) * z;
    d_screen <= tol_commit
}

fn lod_allows_node_proximity_connect(scene: &BoardSceneState, style: &BoardStyleState) -> bool {
    matches!(current_draw_lod(scene, style), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro)
}

/// 🧲️ While dragging a node with no incident edges, overlapping bounds pick the nearest compatible free handle pair.
pub fn node_drag_proximity_handle_pair(scene: &BoardSceneState, style: &BoardStyleState, moving_node_id: &str) -> Option<(String, String)> {
    if !lod_allows_node_proximity_connect(scene, style) {
        return None;
    }
    if !scene.node_selectable(moving_node_id) {
        return None;
    }
    if scene.node_has_any_incident_edge(moving_node_id) {
        return None;
    }
    let moving = scene.nodes.get(moving_node_id)?;
    let moving_bounds = style.node_world_bounds(moving, 0.0);
    let mut best: Option<(f64, String, String)> = None;
    for (target_id, target) in &scene.nodes {
        if target_id == moving_node_id || !scene.node_selectable(target_id.as_str()) {
            continue;
        }
        let target_bounds = style.node_world_bounds(target, 0.0);
        if !world_boxes_overlap(moving_bounds, target_bounds) {
            continue;
        }
        let moving_handles: Vec<_> = scene.handles.iter().filter(|(id, h)| h.node_id == moving_node_id && scene.handle_selectable(id.as_str()) && !scene.handle_has_incident_edge(id.as_str())).collect();
        let target_handles: Vec<_> = scene.handles.iter().filter(|(id, h)| h.node_id == target_id.as_str() && scene.handle_selectable(id.as_str()) && !scene.handle_has_incident_edge(id.as_str())).collect();
        for (src_id, src_h) in &moving_handles {
            let Some(src_pos) = handle_world_pos(scene, style, src_h) else {
                continue;
            };
            for (tgt_id, tgt_h) in &target_handles {
                let Some(tgt_pos) = handle_world_pos(scene, style, tgt_h) else {
                    continue;
                };
                let d = distance_between(src_pos, tgt_pos);
                let pair = if handles_link_compatible_for_drag(scene, style, src_h, tgt_h) {
                    Some(((*src_id).clone(), (*tgt_id).clone()))
                } else if handles_link_compatible_for_drag(scene, style, tgt_h, src_h) {
                    Some(((*tgt_id).clone(), (*src_id).clone()))
                } else {
                    None
                };
                if let Some((s, t)) = pair {
                    if best.as_ref().is_none_or(|(bd, _, _)| d < *bd) {
                        best = Some((d, s.to_string(), t.to_string()));
                    }
                }
            }
        }
    }
    best.map(|(_, s, t)| (s, t))
}

pub fn edge_selectable(scene: &BoardSceneState, style: &BoardStyleState, edge: &EdgeData) -> bool {
    if !edge.visible || edge.locked {
        return false;
    }
    if !style.has_ports() {
        return scene.node_selectable(edge.source.as_str()) && scene.node_selectable(edge.target.as_str());
    }
    scene.handle_selectable(edge.source.as_str()) && scene.handle_selectable(edge.target.as_str())
}

pub fn edge_effectively_visible(scene: &BoardSceneState, style: &BoardStyleState, edge: &EdgeData) -> bool {
    if !style.has_ports() {
        return edge.visible && scene.node_effectively_visible(edge.source.as_str()) && scene.node_effectively_visible(edge.target.as_str());
    }
    edge.visible && scene.handle_effectively_visible(edge.source.as_str()) && scene.handle_effectively_visible(edge.target.as_str())
}

/// 📍️ Drag-phase link snap tests **screen px** to the handle anchor so detail/micro zoom keeps a stable hit halo; pointer-up re-checks with `link_snap_commit_proximity_ok` before `proximityConnect`.
pub fn nearest_link_snap_handle_world(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, world: Point) -> Option<String> {
    if matches!(current_draw_lod(scene, style), BoardDrawLod::Minimap) {
        return None;
    }
    let source_handle = scene.handles.get(source_handle_id)?;
    if !scene.handle_selectable(source_handle_id) {
        return None;
    }
    let source_node_id = source_handle.node_id.as_str();
    let p_scr = scene.world_to_screen(world);
    let mut best: Option<(f64, String)> = None;
    for (id, h) in &scene.handles {
        if id == source_handle_id || !scene.handle_selectable(id.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(id.as_str()) {
            continue;
        }
        if h.node_id == source_node_id {
            continue;
        }
        if !handles_link_compatible_for_drag(scene, style, source_handle, h) {
            continue;
        }
        let pw = handle_world_pos(scene, style, h)?;
        let h_scr = scene.world_to_screen(pw);
        let d_screen = distance_between(p_scr, h_scr);
        let tol_screen = link_snap_drag_tolerance_screen(scene, style, h);
        if d_screen <= tol_screen && best.as_ref().is_none_or(|(bd, _)| d_screen < *bd) {
            best = Some((d_screen, id.clone()));
        }
    }
    best.map(|(_, id)| id)
}

pub fn planned_link_edge_id(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, target_handle_id: &str) -> Option<String> {
    if source_handle_id == target_handle_id || !scene.handle_selectable(source_handle_id) || !scene.handle_selectable(target_handle_id) {
        return None;
    }
    let source = scene.handles.get(source_handle_id)?;
    let target = scene.handles.get(target_handle_id)?;
    if source.node_id == target.node_id || !handles_link_compatible_for_drag(scene, style, source, target) || scene.handle_has_incident_edge(source_handle_id) || scene.handle_has_incident_edge(target_handle_id) {
        return None;
    }
    if scene.edges.values().any(|edge| edge.source == source_handle_id && edge.target == target_handle_id) {
        return None;
    }
    let mut serial = scene.edges.len().saturating_add(1);
    loop {
        let candidate = format!("edge-link-{serial}");
        if !scene.edges.contains_key(&candidate) {
            return Some(candidate);
        }
        serial = serial.saturating_add(1);
    }
}
