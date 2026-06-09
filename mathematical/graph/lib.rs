//! 🕸️ Generic property graph engine on infinite canvas; specialize via quadrant crates.

pub mod geometry {
// #region geometry
//! 📐 Graph geometry: handle positions, edge beziers, hit-test distances.

use crate::cavas::vello::kurbo::{Affine, Arc, BezPath, Circle, CubicBez, ParamCurve, Point, Rect, Shape, Stroke, Vec2};
use crate::NodeShape;
use crate::cavas::vello::peniko::Color;
use crate::cavas::vello::Scene;

#[inline]
pub fn clamp_f64(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

#[inline]
pub fn distance_between(left: Point, right: Point) -> f64 {
    (right - left).hypot()
}

#[inline]
pub fn normalize_or_zero(vector: Vec2) -> Vec2 {
    let len = vector.hypot();
    if len <= f64::EPSILON {
        return Vec2::new(0.0, 0.0);
    }
    vector / len
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn ray_from_origin_to_axis_aligned_rectangle_edge(hw: f64, hh: f64, ux: f64, uy: f64) -> Point {
    let mut t_best = f64::INFINITY;
    if ux.abs() > 1e-12 {
        let tx = ux.signum() * hw / ux;
        let y_at = uy * tx;
        if tx > 0.0 && y_at.abs() <= hh + 1e-9 {
            t_best = t_best.min(tx);
        }
    }
    if uy.abs() > 1e-12 {
        let ty = uy.signum() * hh / uy;
        let x_at = ux * ty;
        if ty > 0.0 && x_at.abs() <= hw + 1e-9 {
            t_best = t_best.min(ty);
        }
    }
    if !t_best.is_finite() || t_best <= 0.0 || t_best == f64::INFINITY {
        return Point::new(hw, 0.0);
    }
    Point::new(ux * t_best, uy * t_best)
}

/// 🕳️ Even-odd clip path: local outer bounds minus the parent node body (keeps handle paint outside transparent nodes).
pub fn handle_outside_node_clip_path(
    handle_center: Point,
    handle_radius: f64,
    node_center: Point,
    node_shape: NodeShape,
    node_radius: f64,
    node_width: f64,
    node_height: f64,
) -> BezPath {
    let margin = (handle_radius * 2.5).max(4.0);
    let outer = Rect::new(
        handle_center.x - margin,
        handle_center.y - margin,
        handle_center.x + margin,
        handle_center.y + margin,
    );
    let mut path = BezPath::new();
    append_shape_elements(&mut path, &outer);
    match node_shape {
        NodeShape::Circle => {
            append_shape_elements(&mut path, &Circle::new(node_center, node_radius.max(1e-9)));
        }
        NodeShape::Rectangle => {
            let hw = node_width.max(1e-9) * 0.5;
            let hh = node_height.max(1e-9) * 0.5;
            append_shape_elements(
                &mut path,
                &Rect::new(
                    node_center.x - hw,
                    node_center.y - hh,
                    node_center.x + hw,
                    node_center.y + hh,
                ),
            );
        }
    }
    path
}

fn append_shape_elements(path: &mut BezPath, shape: &impl Shape) {
    for element in shape.path_elements(0.1) {
        path.push(element);
    }
}

/// 🧭 Outward normal for a handle on a node rim: edge-normal on rectangles, radial on circles.
pub fn handle_outward_at_node_rim(
    handle: Point,
    node_center: Point,
    node_shape: NodeShape,
    node_radius: f64,
    node_width: f64,
    node_height: f64,
) -> Option<Vec2> {
    match node_shape {
        NodeShape::Circle => {
            let outward = normalize_or_zero(handle - node_center);
            if outward.hypot() < 1e-9 {
                None
            } else {
                Some(outward)
            }
        }
        NodeShape::Rectangle => {
            let hw = node_width * 0.5;
            let hh = node_height * 0.5;
            if hw < 1e-9 || hh < 1e-9 {
                return None;
            }
            let dx = handle.x - node_center.x;
            let dy = handle.y - node_center.y;
            if dx.abs() / hw >= dy.abs() / hh {
                Some(Vec2::new(if dx < 0.0 { -1.0 } else { 1.0 }, 0.0))
            } else {
                Some(Vec2::new(0.0, if dy < 0.0 { -1.0 } else { 1.0 }))
            }
        }
    }
}

fn handle_exterior_cap_arc(center: Point, outward: Vec2, radius: f64) -> Option<Arc> {
    let out = normalize_or_zero(outward);
    let r = radius.max(1e-9);
    if out.hypot() < 1e-9 {
        return None;
    }
    let perp = Vec2::new(-out.y, out.x);
    let start = center + perp * r;
    let peak = center + out * r;
    let start_angle = (start.y - center.y).atan2(start.x - center.x);
    let arc_pos = Arc::new(center, (r, r), start_angle, std::f64::consts::PI, 0.0);
    let arc_neg = Arc::new(center, (r, r), start_angle, -std::f64::consts::PI, 0.0);
    if distance_between(arc_pos.eval(0.5), peak) <= distance_between(arc_neg.eval(0.5), peak) {
        Some(arc_pos)
    } else {
        Some(arc_neg)
    }
}

/// 🌗 Closed fill path for the handle cap outside a node body (semicircle on the `outward` side).
pub fn handle_exterior_cap_fill_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
    let r = radius.max(1e-9);
    let mut path = BezPath::new();
    if let Some(arc) = handle_exterior_cap_arc(center, outward, r) {
        append_shape_elements(&mut path, &arc);
        path.close_path();
        return path;
    }
    append_shape_elements(&mut path, &Circle::new(center, r));
    path
}

/// 🌗 Open arc path for stroking only the exterior handle cap (flat rim edge stays behind the node).
pub fn handle_exterior_cap_stroke_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
    let r = radius.max(1e-9);
    let mut path = BezPath::new();
    if let Some(arc) = handle_exterior_cap_arc(center, outward, r) {
        append_shape_elements(&mut path, &arc);
        return path;
    }
    append_shape_elements(&mut path, &Circle::new(center, r));
    path
}

pub fn handle_position_on_circle(center: Point, radius: f64, angle: f64) -> Point {
    let ux = angle.cos();
    let uy = angle.sin();
    center + Vec2::new(ux * radius, uy * radius)
}

/// 🧭 Rectangle handle `angle` is **0 at top edge center (north)**, increasing **counter‑clockwise** in board space (`y` down): `π/4` NW corner, `π/2` west midpoint, `π` south, `3π/2` east; circles keep **east‑zero** `atan2(dy,dx)` convention.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn handle_position_on_rectangle(center: Point, width: f64, height: f64, angle: f64) -> Point {
    let hw = width / 2.0;
    let hh = height / 2.0;
    let ux = -angle.sin();
    let uy = -angle.cos();
    let local = ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
    center + Vec2::new(local.x, local.y)
}

/// 🧭 East-zero polar angle for a circle handle that meets the ray from `center` toward `toward` on the rim.
pub fn circle_handle_angle_toward(center: Point, toward: Point) -> f64 {
    let d = toward - center;
    f64::atan2(d.y, d.x)
}

/// 🧭 North-zero rectangle handle angle so the rim point lies on the ray from `center` toward `toward`.
pub fn rectangle_handle_angle_toward(center: Point, _width: f64, _height: f64, toward: Point) -> f64 {
    let u = normalize_or_zero(toward - center);
    f64::atan2(-u.x, -u.y)
}

pub fn compute_edge_bezier_outward(source_point: Point, target_point: Point, source_outward: Vec2, target_outward: Vec2) -> CubicBez {
    let chord = normalize_or_zero(target_point - source_point);
    let mut source_radial = normalize_or_zero(source_outward);
    if source_radial == Vec2::new(0.0, 0.0) {
        source_radial = chord;
    }
    let mut target_radial = normalize_or_zero(target_outward);
    if target_radial == Vec2::new(0.0, 0.0) {
        target_radial = -chord;
    }
    let handle_distance = distance_between(source_point, target_point);
    let control_length = clamp_f64(handle_distance * 0.35, 24.0, 240.0);
    let p1 = source_point + source_radial * control_length;
    let p2 = target_point + target_radial * control_length;
    CubicBez::new(source_point, p1, p2, target_point)
}

pub fn compute_edge_bezier_points(source_point: Point, target_point: Point, source_center: Point, target_center: Point) -> CubicBez {
    compute_edge_bezier_outward(
        source_point,
        target_point,
        source_point - source_center,
        target_point - target_center,
    )
}

pub fn distance_point_to_cubic_bezier(point: Point, curve: CubicBez, segments: usize) -> f64 {
    let mut smallest = f64::INFINITY;
    let mut previous = curve.eval(0.0);
    let n = segments.max(1);
    for index in 1..=n {
        let t = index as f64 / n as f64;
        let next = curve.eval(t);
        smallest = smallest.min(distance_to_segment(point, previous, next));
        previous = next;
    }
    smallest
}

fn distance_to_segment(point: Point, start: Point, end: Point) -> f64 {
    let segment = end - start;
    let segment_len_squared = segment.dot(segment);
    if segment_len_squared <= f64::EPSILON {
        return distance_between(point, start);
    }
    let projection = clamp_f64((point - start).dot(segment) / segment_len_squared, 0.0, 1.0);
    let closest = start + segment * projection;
    distance_between(point, closest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_node_clip_path_excludes_node_interior() {
        let node_center = Point::new(0.0, 0.0);
        let handle_center = Point::new(40.0, 0.0);
        let clip = handle_outside_node_clip_path(handle_center, 5.0, node_center, NodeShape::Circle, 40.0, 80.0, 80.0);
        assert!(clip.elements().len() > 4);
        assert!(node_center.distance(handle_center) > 39.0);
    }

    fn assert_cap_bulges_outward(center: Point, outward: Vec2, radius: f64) {
        let out = normalize_or_zero(outward);
        let peak = center + out * radius;
        let arc = handle_exterior_cap_arc(center, outward, radius).expect("exterior arc");
        assert!(distance_between(arc.eval(0.5), peak) < 0.35, "arc midpoint must sit on outward peak");
        let fill = handle_exterior_cap_fill_path(center, outward, radius);
        let bb = fill.bounding_box();
        let trough = center - out * radius;
        if out.x.abs() >= out.y.abs() {
            if out.x > 0.0 {
                assert!((bb.x1 - peak.x).abs() < 0.25, "east cap must peak at +x");
                assert!(bb.x0 > trough.x + 0.25, "east cap must not peak inward");
            } else {
                assert!((bb.x0 - peak.x).abs() < 0.25, "west cap must peak at -x");
                assert!(bb.x1 < trough.x - 0.25, "west cap must not peak inward");
            }
        } else if out.y > 0.0 {
            assert!((bb.y1 - peak.y).abs() < 0.25, "south cap must peak at +y");
            assert!(bb.y0 > trough.y + 0.25, "south cap must not peak inward");
        } else {
            assert!((bb.y0 - peak.y).abs() < 0.25, "north cap must peak at -y");
            assert!(bb.y1 < trough.y + 0.25, "north cap must not peak inward");
        }
    }

    #[test]
    fn edge_bezier_free_target_end_tangent_matches_incoming_chord() {
        let source = Point::new(0.0, 0.0);
        let target = Point::new(200.0, 40.0);
        let curve = compute_edge_bezier_points(source, target, Point::new(-50.0, 0.0), target);
        let approach = normalize_or_zero(target - source);
        let tangent = curve.eval(1.0) - curve.eval(0.995);
        let tangent_dir = normalize_or_zero(Vec2::new(tangent.x, tangent.y));
        assert!(tangent_dir.dot(approach) > 0.99, "free target tangent should match incoming chord");
    }

    #[test]
    fn edge_bezier_rectangle_port_uses_outward_normal() {
        let node_center = Point::new(100.0, 50.0);
        let width = 120.0;
        let height = 80.0;
        let source = Point::new(node_center.x - width * 0.5, node_center.y - 20.0);
        let target = Point::new(280.0, 50.0);
        let outward = handle_outward_at_node_rim(source, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
        let curve = compute_edge_bezier_outward(source, target, outward, -normalize_or_zero(target - source));
        let leave = curve.eval(0.005) - curve.eval(0.0);
        let leave_dir = normalize_or_zero(Vec2::new(leave.x, leave.y));
        assert!(leave_dir.dot(outward) > 0.99, "anchored port should leave along rim outward");
    }

    #[test]
    fn rectangle_rim_outward_uses_edge_normal_not_radial() {
        let node_center = Point::new(100.0, 50.0);
        let width = 120.0;
        let height = 80.0;
        let handle = Point::new(node_center.x - width * 0.5, node_center.y - 20.0);
        let radial = normalize_or_zero(handle - node_center);
        let outward = handle_outward_at_node_rim(handle, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
        assert!((outward.x + 1.0).abs() < 1e-9 && outward.y.abs() < 1e-9);
        assert!(radial.y.abs() > 0.1, "radial must tilt for off-center left ports");
    }

    #[test]
    fn exterior_cap_paths_bulge_outward_on_all_cardinals() {
        let radius = 5.0;
        assert_cap_bulges_outward(Point::new(40.0, 0.0), Vec2::new(1.0, 0.0), radius);
        assert_cap_bulges_outward(Point::new(-40.0, 0.0), Vec2::new(-1.0, 0.0), radius);
        assert_cap_bulges_outward(Point::new(0.0, 30.0), Vec2::new(0.0, 1.0), radius);
        assert_cap_bulges_outward(Point::new(0.0, -30.0), Vec2::new(0.0, -1.0), radius);
        let stroke = handle_exterior_cap_stroke_path(Point::new(40.0, 0.0), Vec2::new(1.0, 0.0), radius);
        assert!(!stroke.elements().iter().any(|el| matches!(el, crate::cavas::vello::kurbo::PathEl::ClosePath)));
    }
}

pub fn encode_board_stroke_scene(curves: &[CubicBez], stroke_width: f64) -> Scene {
    let mut scene = Scene::new();
    let stroke = Stroke::new(stroke_width);
    for curve in curves {
        scene.stroke(
            &stroke,
            Affine::IDENTITY,
            Color::new(ui_styling::CANVAS_LIGHT.icon_bg),
            None,
            curve,
        );
    }
    scene
}
// #endregion geometry
}

pub mod scene_json {
// #region scene_json
//! 🧾 Generic scene descriptor JSON (port/edge-agnostic node base).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraJson {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDescJson {
    pub id: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub draggable: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    /// @emoji 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
    #[serde(default)]
    pub icon_kind: Option<String>,
    /// @emoji 🧩 Semantic node-kind id for compatibility rows at `node` specificity.
    #[serde(default)]
    pub node_kind: Option<String>,
    #[serde(default)]
    pub user_data: Option<serde_json::Value>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub root: Option<bool>,
    pub shape: Option<String>,
    #[serde(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
    #[serde(default)]
    pub scale: Option<f64>,
}

fn board_json_hidden_flag(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    obj.get("hidden").and_then(|v| v.as_bool())
}

/// 🙈 Resolves fixture element visibility from `hidden` or `visible` JSON fields.
pub fn board_json_visible_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    match board_json_hidden_flag(obj) {
        Some(hidden) => Some(!hidden),
        None => obj.get("visible").and_then(|v| v.as_bool()),
    }
}

/// 🙈 Returns true when a fixture element is visible (default true when unset).
pub fn board_json_visible_or_true(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
    board_json_visible_option(obj).unwrap_or(true)
}
// #endregion scene_json
}


pub use geometry::{
    circle_handle_angle_toward, clamp_f64, compute_edge_bezier_outward, compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier,
    encode_board_stroke_scene,
    handle_exterior_cap_fill_path, handle_exterior_cap_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim,
    handle_position_on_circle, handle_position_on_rectangle, normalize_or_zero,
    ray_from_origin_to_axis_aligned_rectangle_edge, rectangle_handle_angle_toward,
};
pub use scene_json::{board_json_visible_option, board_json_visible_or_true, CameraJson, NodeDescJson};

pub use infinite_cavas as cavas;
pub use mathematical_core::{self as core, Directed, Directedness, Edge as CoreEdge, EdgeId, HandleId, NodeId, Normal, PortModel, Ported, Undirected};

pub use core::orient_endpoints;

// #region 🔖GraphExtension
/// 🧩 Extension hook for domain-specific graph behavior.
pub trait GraphExtension: cavas::CanvasExtension {}
// #endregion 🔖GraphExtension

// #region 🔖Kinds
use std::collections::{BTreeMap, BTreeSet};

use cavas::vello::kurbo::{CubicBez, ParamCurve, Point, Vec2};

/// 🧭 Camera state in world units with a zoom scalar suitable for a WASM host bridge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🔵 Circle or axis-aligned rectangle node body.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NodeShape {
    #[default]
    Circle,
    Rectangle,
}

/// 🪝 Port direction for directed edge wiring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandleRole {
  Source,
  Target,
  #[default]
  Any,
}

/// 🟠 Retained node state with world-space center and shape extents.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub center: Point,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub shape: NodeShape,
    pub draggable: bool,
}

/// 🟣 Tangent handle anchored to a node at a polar angle.
#[derive(Clone, Debug, PartialEq)]
pub struct Handle {
    pub angle: f64,
    pub id: HandleId,
    pub node_id: NodeId,
    pub radius: f64,
    pub role: HandleRole,
}

/// 🪢 Retained edge with typed endpoints.
pub type GraphEdge<E> = CoreEdge<E>;

/// 🎯 Semantic board event emitted after interaction or selection changes.
#[derive(Clone, Debug, PartialEq)]
pub enum BoardEvent {
    HoverChanged { id: Option<u64> },
    NodeMoved { id: NodeId, x: f64, y: f64 },
    EdgeConnected { id: EdgeId, source: HandleId, target: HandleId },
    EdgeRemoved { id: EdgeId },
    SelectionChanged { edge_ids: Vec<EdgeId>, handle_ids: Vec<HandleId>, node_ids: Vec<NodeId> },
    PreselectChanged {
        edge_ids: Vec<EdgeId>,
        handle_ids: Vec<HandleId>,
        node_ids: Vec<NodeId>,
        removed_edge_ids: Vec<EdgeId>,
        removed_handle_ids: Vec<HandleId>,
        removed_node_ids: Vec<NodeId>,
    },
}

/// ✅ Selection snapshot maintained by the engine hot path.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Selection {
    pub edge_ids: BTreeSet<EdgeId>,
    pub handle_ids: BTreeSet<HandleId>,
    pub node_ids: BTreeSet<NodeId>,
}

/// 🖼️ Minimal render snapshot suitable for a host-side drawing layer or tests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderSnapshot {
    pub edges: Vec<CubicBez>,
    pub handles: Vec<(HandleId, Point, f64)>,
    pub nodes: Vec<(NodeId, Point, f64)>,
    pub pending_edge: Option<CubicBez>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HitObject<E> {
    Edge(EdgeId),
    Endpoint(E),
    Node(NodeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionMode {
    DragNode { node_id: NodeId, offset: Vec2 },
    DragNodes {
        primary_id: NodeId,
        offset: Vec2,
    },
    DrawEdge {
        anchor_handle: HandleId,
        anchor_is_source: bool,
        fixed_target: Option<HandleId>,
        cursor: Point,
        reconnecting: Option<EdgeId>,
        snap_target: Option<HandleId>,
    },
    SelectionPending {
        start: Point,
        start_screen: Point,
    },
    AreaSelect {
        start: Point,
        start_screen: Point,
    },
    Pan {
        start_screen: Point,
        cam_x: f64,
        cam_y: f64,
        zoom: f64,
    },
    Idle,
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ProximityConnection {
    source: HandleId,
    target: HandleId,
}

pub fn handle_position(node: &Node, handle: &Handle) -> Point {
    match node.shape {
        NodeShape::Circle => geometry::handle_position_on_circle(node.center, node.radius, handle.angle),
        NodeShape::Rectangle => geometry::handle_position_on_rectangle(node.center, node.width, node.height, handle.angle),
    }
}

fn distance(left: Point, right: Point) -> f64 {
    geometry::distance_between(left, right)
}

pub const DEFAULT_PROXIMITY_DISTANCE_WORLD: f64 = ui_styling::metrics::board::PROXIMITY_DISTANCE_WORLD;

fn node_contains_point(node: &Node, point: Point) -> bool {
    match node.shape {
        NodeShape::Circle => distance(point, node.center) <= node.radius,
        NodeShape::Rectangle => {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            point.x >= node.center.x - hw
                && point.x <= node.center.x + hw
                && point.y >= node.center.y - hh
                && point.y <= node.center.y + hh
        }
    }
}

// #region 🔖GraphPortModel
/// 🔌 Port model with graph selection semantics.
pub trait GraphPortModel: PortModel {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint);
}

impl GraphPortModel for Normal {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint) {
        selection.node_ids.insert(endpoint);
    }
}

impl GraphPortModel for Ported {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint) {
        selection.handle_ids.insert(endpoint);
    }
}
// #endregion 🔖GraphPortModel
// #endregion 🔖Kinds

// #region 🔖SelectionMarquee
pub use cavas::geom_sel::{
    inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box,
    world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox,
};

pub const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_CLICK_MAX_DISTANCE_PX;
pub const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
pub const SELECTION_MARQUEE_DRAG_THRESHOLD_PX: f64 = ui_styling::metrics::board::SELECTION_MARQUEE_MAX_DISTANCE_PX;

/// 🎯 Normalizes `default` to `replace` for merge-mode strings.
pub fn normalize_selection_mode(mode: &str) -> String {
    if mode == "default" { "replace".into() } else { mode.to_string() }
}

/// 🎯 Maps shift/ctrl modifiers to marquee selection mode (ctrl+shift → invertive).
pub fn pick_merge_mode_for_modifiers(ctrl_or_meta: bool, shift: bool, option_mode: &str) -> String {
    if ctrl_or_meta && shift {
        return "invertive".into();
    }
    if ctrl_or_meta {
        return "subtractive".into();
    }
    if shift {
        return "additive".into();
    }
    normalize_selection_mode(option_mode)
}

/// 🎯 Applies pick merge mode for a single id.
pub fn merge_pick_into_selection(initial: &BTreeSet<String>, hit_id: &str, mode: &str) -> BTreeSet<String> {
    let mut next = initial.clone();
    match mode {
        "additive" => {
            next.insert(hit_id.to_string());
        }
        "subtractive" => {
            next.remove(hit_id);
        }
        "replace" => {
            next.clear();
            next.insert(hit_id.to_string());
        }
        _ => {
            if next.contains(hit_id) {
                next.remove(hit_id);
            } else {
                next.insert(hit_id.to_string());
            }
        }
    }
    next
}

/// 🎯 Applies pick merge mode for a marquee hit set.
pub fn merge_ids_into_selection(initial: &BTreeSet<String>, hits: &BTreeSet<String>, mode: &str) -> BTreeSet<String> {
    if mode == "replace" {
        return hits.clone();
    }
    let mut next = initial.clone();
    for id in hits {
        match mode {
            "additive" => {
                next.insert(id.clone());
            }
            "subtractive" => {
                next.remove(id);
            }
            _ => {
                if next.contains(id) {
                    next.remove(id);
                } else {
                    next.insert(id.clone());
                }
            }
        }
    }
    next
}

pub const SELECTION_DRAG_DIRECTION_THRESHOLD_PX: f64 = ui_styling::metrics::board::SELECTION_DRAG_DIRECTION_PX;

/// 🎯 Drag left→right = enclosing/full; right→left = crossing/partial (rectangle endpoints).
pub fn selection_drag_enclosing_rectangle(start: Point, end: Point) -> bool {
    end.x >= start.x
}

/// 🎯 Lasso uses the first horizontal step; rectangle compares start vs end.
pub fn selection_drag_enclosing(method: &str, start: Point, points: &[Point]) -> bool {
    if method == "lasso" {
        for point in points.iter().skip(1) {
            let dx = point.x - start.x;
            if dx.abs() < SELECTION_DRAG_DIRECTION_THRESHOLD_PX {
                continue;
            }
            return dx > 0.0;
        }
    }
    let end = points.last().copied().unwrap_or(start);
    selection_drag_enclosing_rectangle(start, end)
}

/// 🧿 Builds the world-space marquee shape for rectangle or lasso drags.
pub fn selection_drag_shape(method: &str, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
    let last = points.last().copied().unwrap_or(start);
    let enclosing = selection_drag_enclosing(method, start, points);
    if method == "lasso" && points.len() >= 3 {
        let poly = points.to_vec();
        let b = world_box_from_points(&poly)?;
        return Some((b, enclosing, poly));
    }
    let b = world_box_from_points(&[start, last])?;
    let poly = vec![
        Point::new(b.min_x, b.min_y),
        Point::new(b.max_x, b.min_y),
        Point::new(b.max_x, b.max_y),
        Point::new(b.min_x, b.max_y),
    ];
    Some((b, enclosing, poly))
}

/// 🧿 Screen-space overlay points for the shared `SelectionMarquee` overlay.
pub fn selection_screen_overlay_points(method: &str, start_screen: Point, screen_points: &[Point]) -> Option<Vec<Point>> {
    if screen_points.len() < 2 {
        return None;
    }
    let last = *screen_points.last().unwrap_or(&start_screen);
    Some(if method == "lasso" {
        screen_points.to_vec()
    } else {
        vec![
            start_screen,
            Point::new(last.x, start_screen.y),
            last,
            Point::new(start_screen.x, last.y),
        ]
    })
}

/// 🧿 Returns sorted ids for the next preselect set and removed anchor ids.
pub fn area_preselect_ids(anchor: &BTreeSet<String>, ids: &[String]) -> (Vec<String>, Vec<String>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let mut sorted: Vec<_> = next.iter().cloned().collect();
    sorted.sort();
    let mut removed: Vec<_> = anchor.difference(&next).cloned().collect();
    removed.sort();
    (sorted, removed)
}

fn node_rect_bounds(center: Point, width: f64, height: f64) -> WorldBox {
    let hw = width * 0.5;
    let hh = height * 0.5;
    WorldBox {
        min_x: center.x - hw,
        min_y: center.y - hh,
        max_x: center.x + hw,
        max_y: center.y + hh,
    }
}

fn node_circle_bounds(center: Point, radius: f64) -> WorldBox {
    WorldBox {
        min_x: center.x - radius,
        min_y: center.y - radius,
        max_x: center.x + radius,
        max_y: center.y + radius,
    }
}

/// 🎯 Tests whether a graph node body intersects or is contained by the marquee shape.
pub fn selection_contains_node_bounds(node: &Node, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    let bounds = match node.shape {
        NodeShape::Rectangle => node_rect_bounds(node.center, node.width, node.height),
        NodeShape::Circle => node_circle_bounds(node.center, node.radius),
    };
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

/// 🎯 Tests whether a port handle intersects or is contained by the marquee shape.
pub fn selection_contains_handle_point(pos: Point, pad: f64, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    let bounds = WorldBox {
        min_x: pos.x - pad,
        min_y: pos.y - pad,
        max_x: pos.x + pad,
        max_y: pos.y + pad,
    };
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

/// 🎯 Tests whether a cubic edge intersects or is contained by the marquee shape.
pub fn selection_contains_edge_curve(curve: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    const STEPS: usize = 24;
    let mut samples = Vec::with_capacity(STEPS + 1);
    for i in 0..=STEPS {
        let t = i as f64 / STEPS as f64;
        samples.push(curve.eval(t));
    }
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
// #endregion 🔖SelectionMarquee

// #region 🔖Engine

/// 🎯 Engine-local area-select options.
#[derive(Clone, Debug, PartialEq)]
pub struct EngineSelectionOptions {
    pub method: String,
    pub mode: String,
    pub select_nodes: bool,
    pub select_handles: bool,
    pub select_edges: bool,
}

impl Default for EngineSelectionOptions {
    fn default() -> Self {
        Self {
            method: "rectangle".into(),
            mode: "replace".into(),
            select_nodes: true,
            select_handles: true,
            select_edges: true,
        }
    }
}

/// ⚙️ Retained graph engine parameterized by port model and directedness.
#[derive(Clone, Debug)]
pub struct GraphEngine<P: GraphPortModel, D: Directedness> {
    pub camera: Camera,
    pub edges: BTreeMap<EdgeId, GraphEdge<P::Endpoint>>,
    pub enforce_acyclic: bool,
    pub events: Vec<BoardEvent>,
    pub handles: BTreeMap<HandleId, Handle>,
    pub hover: Option<u64>,
    pub interaction: InteractionMode,
    pub nodes: BTreeMap<NodeId, Node>,
    pub selection: Selection,
    pub preselect: Selection,
    pub preselect_removed: Selection,
    pub selection_options: EngineSelectionOptions,
    pub handle_pointer_picking: bool,
    pub     proximity_distance_world: f64,
    proximity_distance_override: Option<f64>,
    pub selection_preview_points: Vec<Point>,
    pub selection_preview_crossing: bool,
    area_initial: Selection,
    area_points: Vec<Point>,
    area_screen_points: Vec<Point>,
    drag_start_positions: BTreeMap<NodeId, Point>,
    proximity_connection: Option<ProximityConnection>,
    next_edge_id: u64,
    _directedness: std::marker::PhantomData<D>,
    _port: std::marker::PhantomData<P>,
}

impl<P: GraphPortModel, D: Directedness> Default for GraphEngine<P, D> {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            edges: BTreeMap::new(),
            enforce_acyclic: false,
            events: Vec::new(),
            handles: BTreeMap::new(),
            hover: None,
            interaction: InteractionMode::default(),
            nodes: BTreeMap::new(),
            selection: Selection::default(),
            preselect: Selection::default(),
            preselect_removed: Selection::default(),
            selection_options: EngineSelectionOptions::default(),
            handle_pointer_picking: true,
            proximity_distance_world: DEFAULT_PROXIMITY_DISTANCE_WORLD,
            proximity_distance_override: None,
            selection_preview_points: Vec::new(),
            selection_preview_crossing: false,
            area_initial: Selection::default(),
            area_points: Vec::new(),
            area_screen_points: Vec::new(),
            drag_start_positions: BTreeMap::new(),
            proximity_connection: None,
            next_edge_id: 1000,
            _directedness: std::marker::PhantomData,
            _port: std::marker::PhantomData,
        }
    }
}

impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera = Camera { x, y, zoom };
    }

    pub fn create_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64, draggable: bool) {
        self.nodes.insert(
            id,
            Node {
                center: Point::new(x, y),
                draggable,
                height: radius * 2.0,
                id,
                radius,
                shape: NodeShape::Circle,
                width: radius * 2.0,
            },
        );
    }

    pub fn create_rect_node(&mut self, id: NodeId, x: f64, y: f64, width: f64, height: f64, draggable: bool) {
        let hw = width * 0.5;
        let hh = height * 0.5;
        self.nodes.insert(
            id,
            Node {
                center: Point::new(x, y),
                draggable,
                height,
                id,
                radius: hw.max(hh).max(ui_styling::radii::NODE_MIN),
                shape: NodeShape::Rectangle,
                width,
            },
        );
    }

    pub fn update_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.center = Point::new(x, y);
            node.radius = radius;
            if node.shape == NodeShape::Circle {
                node.width = radius * 2.0;
                node.height = radius * 2.0;
            }
        }
    }

    pub fn set_next_edge_id(&mut self, id: u64) {
        self.next_edge_id = id;
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        if P::HAS_PORTS {
            let removed_handles: Vec<HandleId> = self.handles.values().filter(|handle| handle.node_id == id).map(|handle| handle.id).collect();
            for handle_id in removed_handles {
                self.remove_handle(handle_id);
            }
        } else {
            let removed_edges: Vec<EdgeId> = self
                .edges
                .values()
                .filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id)
                .map(|edge| edge.id)
                .collect();
            for edge_id in removed_edges {
                self.edges.remove(&edge_id);
                self.selection.edge_ids.remove(&edge_id);
            }
        }
        self.selection.node_ids.remove(&id);
        self.push_selection_event();
    }

    pub fn create_handle(&mut self, id: HandleId, node_id: NodeId, angle: f64) {
        if P::HAS_PORTS {
            self.handles.insert(
                id,
                Handle {
                    angle,
                    id,
                    node_id,
                    radius: ui_styling::radii::HANDLE_DEFAULT,
                    role: HandleRole::Any,
                },
            );
        }
    }

    pub fn set_handle_role(&mut self, id: HandleId, role: HandleRole) {
        if let Some(handle) = self.handles.get_mut(&id) {
            handle.role = role;
        }
    }

    pub fn create_edge(&mut self, id: EdgeId, source: P::Endpoint, target: P::Endpoint) {
        let (source, target) = orient_endpoints::<P::Endpoint, D>(source, target);
        self.edges.insert(id, GraphEdge { id, source, target });
        if id >= self.next_edge_id {
            self.next_edge_id = id + 1;
        }
    }

    pub fn remove_edge(&mut self, id: EdgeId) {
        if self.edges.remove(&id).is_some() {
            self.selection.edge_ids.remove(&id);
            self.events.push(BoardEvent::EdgeRemoved { id });
        }
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_handles: bool, select_edges: bool) {
        self.selection_options.method = method.to_string();
        self.selection_options.mode = normalize_selection_mode(mode);
        self.selection_options.select_nodes = select_nodes;
        self.selection_options.select_handles = select_handles;
        self.selection_options.select_edges = select_edges;
    }

    pub fn selection_preview_points(&self) -> &[Point] {
        &self.selection_preview_points
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.selection_preview_crossing
    }

    pub fn cancel_area_select(&mut self) -> bool {
        let prev = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let cancelled = matches!(prev, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. });
        if cancelled {
            self.selection = self.area_initial.clone();
            self.clear_preselect();
            self.selection_preview_points.clear();
            self.selection_preview_crossing = false;
            self.push_selection_event();
        }
        cancelled
    }

    pub fn select_all(&mut self) {
        self.selection = Selection::default();
        if self.selection_options.select_nodes {
            self.selection.node_ids = self.nodes.keys().copied().collect();
        }
        if self.selection_options.select_handles && P::HAS_PORTS {
            self.selection.handle_ids = self.handles.keys().copied().collect();
        }
        if self.selection_options.select_edges {
            self.selection.edge_ids = self.edges.keys().copied().collect();
        }
        self.clear_preselect();
        self.push_selection_event();
    }

    pub fn delete_selection(&mut self) {
        let node_ids: Vec<_> = self.selection.node_ids.iter().copied().collect();
        for id in node_ids {
            self.remove_node(id);
        }
        let edge_ids: Vec<_> = self.selection.edge_ids.iter().copied().collect();
        for id in edge_ids {
            self.remove_edge(id);
        }
        if P::HAS_PORTS {
            let handle_ids: Vec<_> = self.selection.handle_ids.iter().copied().collect();
            for id in handle_ids {
                self.remove_handle(id);
            }
        }
        self.clear_preselect();
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend_selection: bool) {
        self.pointer_down_screen(x, y, x, y, 0, extend_selection, false, false);
    }

    /// @emoji 📦 Starts a group drag when `point` lies inside the padded union bounds of draggable selected nodes.
    pub fn try_begin_selection_union_drag_at(&mut self, point: Point, pad_world: f64) -> bool {
        let members: Vec<NodeId> = self
            .selection
            .node_ids
            .iter()
            .copied()
            .filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable))
            .collect();
        if members.is_empty() {
            return false;
        }
        let mut corners = Vec::new();
        for id in &self.selection.node_ids {
            let Some(node) = self.nodes.get(id) else {
                continue;
            };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            corners.push(Point::new(node.center.x - hw, node.center.y - hh));
            corners.push(Point::new(node.center.x + hw, node.center.y + hh));
        }
        let Some(bounds) = world_box_from_points(&corners) else {
            return false;
        };
        if !world_box_contains_point(inflate_world_box(bounds, pad_world), point) {
            return false;
        }
        let primary_id = members
            .iter()
            .min_by(|a, b| {
                let da = self.nodes.get(a).map(|n| distance_between(point, n.center)).unwrap_or(f64::INFINITY);
                let db = self.nodes.get(b).map(|n| distance_between(point, n.center)).unwrap_or(f64::INFINITY);
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .unwrap_or(members[0]);
        let Some(primary) = self.nodes.get(&primary_id) else {
            return false;
        };
        self.drag_start_positions.clear();
        for id in &members {
            if let Some(node) = self.nodes.get(id) {
                self.drag_start_positions.insert(*id, node.center);
            }
        }
        self.interaction = InteractionMode::DragNodes {
            primary_id,
            offset: point - primary.center,
        };
        self.hover = None;
        true
    }

    /// @emoji 🫳 Starts dragging a selected draggable node (or its multi-selection group) from `point`.
    pub fn try_begin_selected_node_drag_at(&mut self, node_id: NodeId, point: Point) -> bool {
        if !self.selection.node_ids.contains(&node_id) {
            return false;
        }
        let Some(node) = self.nodes.get(&node_id) else {
            return false;
        };
        if !node.draggable {
            return false;
        }
        let members: Vec<NodeId> = self
            .selection
            .node_ids
            .iter()
            .copied()
            .filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable))
            .collect();
        let drag_group = members.contains(&node_id) && members.len() > 1;
        self.drag_start_positions.clear();
        for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
            if let Some(n) = self.nodes.get(id) {
                self.drag_start_positions.insert(*id, n.center);
            }
        }
        if drag_group {
            self.interaction = InteractionMode::DragNodes {
                primary_id: node_id,
                offset: point - node.center,
            };
        } else {
            self.interaction = InteractionMode::DragNode {
                node_id,
                offset: point - node.center,
            };
        }
        self.hover = None;
        true
    }

    /// @emoji 🫳 Selects a draggable node and starts moving it from `point`.
    pub fn pointer_down_on_draggable_node_at(&mut self, node_id: NodeId, point: Point, shift: bool, ctrl_or_meta: bool) {
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let merge_from_modifiers = ctrl_or_meta || shift;
        let members_before: Vec<NodeId> = self
            .selection
            .node_ids
            .iter()
            .copied()
            .filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable))
            .collect();
        let drag_group_before = members_before.contains(&node_id) && members_before.len() > 1;
        let force_pick_merge = (merge_mode == "replace" && !drag_group_before)
            || merge_mode == "subtractive"
            || (merge_mode == "invertive" && merge_from_modifiers);
        if !drag_group_before || force_pick_merge {
            self.apply_pick_with_mode(HitObject::Node(node_id), merge_mode.as_str());
        }
        if let Some(node) = self.nodes.get(&node_id) {
            if node.draggable {
                let members: Vec<NodeId> = self
                    .selection
                    .node_ids
                    .iter()
                    .copied()
                    .filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable))
                    .collect();
                let drag_group = members.contains(&node_id) && members.len() > 1;
                self.drag_start_positions.clear();
                for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
                    if let Some(n) = self.nodes.get(id) {
                        self.drag_start_positions.insert(*id, n.center);
                    }
                }
                if drag_group {
                    self.interaction = InteractionMode::DragNodes {
                        primary_id: node_id,
                        offset: point - node.center,
                    };
                } else {
                    self.interaction = InteractionMode::DragNode {
                        node_id,
                        offset: point - node.center,
                    };
                }
            }
        }
        self.update_hover(Some(node_id));
    }

    /// @emoji 🪝 Merges a handle into the engine selection without starting edge draw.
    pub fn select_handle_with_mode(&mut self, handle_id: HandleId, mode: &str) {
        if !self.selection_options.select_handles || !P::HAS_PORTS {
            return;
        }
        let current: BTreeSet<String> = self.selection.handle_ids.iter().map(|id| id.to_string()).collect();
        let next = merge_pick_into_selection(&current, &handle_id.to_string(), mode);
        self.selection.handle_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
        self.clear_preselect();
        self.push_selection_event();
    }

    pub fn pointer_down_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, _alt: bool) {
        self.proximity_connection = None;
        self.selection_preview_points.clear();
        self.selection_preview_crossing = false;
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let merge_from_modifiers = ctrl_or_meta || shift;
        match self.hit_test(point) {
            Some(HitObject::Node(node_id)) => {
                let members_before: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                let drag_group_before = members_before.contains(&node_id) && members_before.len() > 1;
                let force_pick_merge = (merge_mode == "replace" && !drag_group_before) || merge_mode == "subtractive" || (merge_mode == "invertive" && merge_from_modifiers);
                if !drag_group_before || force_pick_merge {
                    self.apply_pick_with_mode(HitObject::Node(node_id), merge_mode.as_str());
                }
                if let Some(node) = self.nodes.get(&node_id) {
                    if node.draggable {
                        let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                        let drag_group = members.contains(&node_id) && members.len() > 1;
                        self.drag_start_positions.clear();
                        for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
                            if let Some(n) = self.nodes.get(id) {
                                self.drag_start_positions.insert(*id, n.center);
                            }
                        }
                        if drag_group {
                            self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center };
                        } else {
                            self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
                        }
                    }
                }
                self.update_hover(Some(node_id));
            }
            Some(HitObject::Endpoint(ep)) => {
                self.apply_pick_with_mode(HitObject::Endpoint(ep), merge_mode.as_str());
                let hid = P::endpoint_as_u64(ep);
                self.update_hover(Some(hid));
                if P::HAS_PORTS {
                    self.begin_draw_edge_from_handle(hid, point);
                } else {
                    self.interaction = InteractionMode::Idle;
                }
            }
            Some(HitObject::Edge(edge_id)) => {
                self.apply_pick_with_mode(HitObject::Edge(edge_id), merge_mode.as_str());
                self.update_hover(Some(edge_id));
                self.interaction = InteractionMode::Idle;
            }
            None if button == 0 => {
                self.area_initial = self.selection.clone();
                self.interaction = InteractionMode::SelectionPending { start: point, start_screen: screen };
                self.update_hover(None);
            }
            None => {
                if merge_from_modifiers {
                    self.selection = Selection::default();
                    self.push_selection_event();
                } else {
                    self.selection = Selection::default();
                    self.push_selection_event();
                }
                self.update_hover(None);
                self.interaction = InteractionMode::Idle;
            }
        }
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, x, y, false, false, false);
    }

    pub fn pointer_move_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        match std::mem::replace(&mut self.interaction, InteractionMode::Idle) {
            InteractionMode::Pan { start_screen, cam_x, cam_y, zoom } => {
                let dx = (screen.x - start_screen.x) / zoom;
                let dy = (screen.y - start_screen.y) / zoom;
                self.set_camera(cam_x - dx, cam_y - dy, zoom);
                self.interaction = InteractionMode::Pan { start_screen, cam_x, cam_y, zoom };
            }
            InteractionMode::DragNode { node_id, offset } => {
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.center = point - offset;
                    self.events.push(BoardEvent::NodeMoved { id: node_id, x: node.center.x, y: node.center.y });
                }
                self.update_node_drag_proximity(&[node_id]);
                if alt {
                    self.proximity_distance_override = None;
                    self.proximity_connection = None;
                }
                self.interaction = InteractionMode::DragNode { node_id, offset };
            }
            InteractionMode::DragNodes { primary_id, offset } => {
                let Some((px0, py0)) = self.drag_start_positions.get(&primary_id).map(|p| (p.x, p.y)) else {
                    self.interaction = InteractionMode::Idle;
                    return;
                };
                let nx = point.x - offset.x;
                let ny = point.y - offset.y;
                let dx = nx - px0;
                let dy = ny - py0;
                let dragged_ids: Vec<NodeId> = self.drag_start_positions.keys().copied().collect();
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                for (id, start) in &self.drag_start_positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.center = Point::new(start.x + dx, start.y + dy);
                        self.events.push(BoardEvent::NodeMoved { id: *id, x: node.center.x, y: node.center.y });
                    }
                }
                self.update_node_drag_proximity(&dragged_ids);
                if alt {
                    self.proximity_distance_override = None;
                    self.proximity_connection = None;
                }
                self.interaction = InteractionMode::DragNodes { primary_id, offset };
            }
            InteractionMode::DrawEdge {
                anchor_handle,
                anchor_is_source,
                fixed_target,
                reconnecting,
                ..
            } => {
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                let snap_target = if alt {
                    None
                } else {
                    self.nearest_wire_snap_handle(anchor_handle, anchor_is_source, fixed_target, reconnecting, point)
                };
                if alt {
                    self.proximity_distance_override = None;
                }
                self.interaction = InteractionMode::DrawEdge {
                    anchor_handle,
                    anchor_is_source,
                    fixed_target,
                    cursor: point,
                    reconnecting,
                    snap_target,
                };
                let hover = snap_target.or_else(|| {
                    self.hit_test(point).map(|hit| match hit {
                        HitObject::Edge(id) => id,
                        HitObject::Endpoint(ep) => P::endpoint_as_u64(ep),
                        HitObject::Node(id) => id,
                    })
                });
                self.update_hover(hover);
            }
            InteractionMode::SelectionPending { start, start_screen } => {
                if distance_between(screen, start_screen) < SELECTION_MARQUEE_DRAG_THRESHOLD_PX {
                    self.interaction = InteractionMode::SelectionPending { start, start_screen };
                } else {
                    let area_points = vec![start, point];
                    let area_screen_points = vec![start_screen, screen];
                    self.area_points = area_points.clone();
                    self.area_screen_points = area_screen_points.clone();
                    self.apply_area_preselect(start, &area_points, shift, ctrl_or_meta);
                    self.sync_selection_screen_overlay(start_screen, &area_screen_points);
                    self.interaction = InteractionMode::AreaSelect { start, start_screen };
                }
            }
            InteractionMode::AreaSelect { start, start_screen } => {
                let mut points = self.area_points.clone();
                let mut screen_points = self.area_screen_points.clone();
                let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                let add_point = self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                if add_point {
                    points.push(point);
                    screen_points.push(screen);
                } else if !points.is_empty() {
                    let last = points.len() - 1;
                    points[last] = point;
                    let ls = screen_points.len() - 1;
                    screen_points[ls] = screen;
                }
                let points_for_preselect = points.clone();
                let screen_for_overlay = screen_points.clone();
                self.apply_area_preselect(start, &points_for_preselect, shift, ctrl_or_meta);
                self.sync_selection_screen_overlay(start_screen, &screen_for_overlay);
                self.area_points = points;
                self.area_screen_points = screen_points;
                self.interaction = InteractionMode::AreaSelect { start, start_screen };
            }
            InteractionMode::Idle => {
                self.interaction = InteractionMode::Idle;
                self.update_hover(self.hit_test(point).map(|hit| match hit {
                    HitObject::Edge(id) => id,
                    HitObject::Endpoint(ep) => P::endpoint_as_u64(ep),
                    HitObject::Node(id) => id,
                }));
            }
            other => {
                self.interaction = other;
            }
        }
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, x, y, false, false, false);
    }

    pub fn pointer_up_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        let grabbed = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let node_drag_proximity = self.proximity_connection.take();
        match grabbed {
            InteractionMode::DrawEdge {
                anchor_handle,
                anchor_is_source,
                fixed_target,
                reconnecting,
                snap_target,
                ..
            } => {
                let endpoint = snap_target
                    .filter(|hid| self.wire_snap_still_active(*hid, point))
                    .or_else(|| self.hit_test(point).and_then(|hit| match hit {
                        HitObject::Endpoint(ep) => Some(P::endpoint_as_u64(ep)),
                        _ => None,
                    }));
                if !alt {
                    if let Some(hit_hid) = endpoint {
                        let (source_hid, target_handle) = if let Some(tgt) = fixed_target {
                            (hit_hid, tgt)
                        } else if anchor_is_source {
                            (anchor_handle, hit_hid)
                        } else {
                            (hit_hid, anchor_handle)
                        };
                        self.try_connect_handles(source_hid, target_handle, reconnecting);
                    }
                }
            }
            InteractionMode::DragNodes { .. } | InteractionMode::DragNode { .. } => {
                if !alt {
                    if let Some(conn) = node_drag_proximity {
                        self.try_connect_handles(conn.source, conn.target, None);
                    }
                }
            }
            InteractionMode::SelectionPending { start, start_screen } => {
                let merge_from_modifiers = ctrl_or_meta || shift;
                if !merge_from_modifiers {
                    self.selection = Selection::default();
                    self.push_selection_event();
                } else {
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_hits(&self.area_initial_string_set(), start, &[start], merge_mode.as_str());
                    self.commit_selection_from_hits(&next);
                }
                let _ = (start_screen, start);
                self.clear_preselect();
                self.selection_preview_points.clear();
                self.selection_preview_crossing = false;
            }
            InteractionMode::AreaSelect { start, start_screen } => {
                let mut points = self.area_points.clone();
                let mut screen_points = self.area_screen_points.clone();
                points.push(point);
                screen_points.push(screen);
                let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                let next = if click_only {
                    BTreeSet::new()
                } else {
                    self.resolve_area_hits(&self.area_initial_string_set(), start, &points, merge_mode.as_str())
                };
                self.commit_selection_from_hits(&next);
                self.clear_preselect();
                self.selection_preview_points.clear();
                self.selection_preview_crossing = false;
            }
            InteractionMode::Pan { .. } => {}
            other => {
                self.interaction = other;
            }
        }
    }

    pub fn render_snapshot(&self) -> RenderSnapshot {
        let mut snapshot = RenderSnapshot::default();
        for node in self.nodes.values() {
            snapshot.nodes.push((node.id, node.center, node.radius));
        }
        if P::HAS_PORTS {
            for handle in self.handles.values() {
                if let Some(node) = self.nodes.get(&handle.node_id) {
                    snapshot.handles.push((handle.id, handle_position(node, handle), handle.radius));
                }
            }
        }
        for edge in self.edges.values() {
            if let Some(curve) = self.edge_curve(edge.id) {
                snapshot.edges.push(curve);
            }
        }
        if let InteractionMode::DrawEdge {
            anchor_handle,
            anchor_is_source,
            fixed_target,
            cursor,
            snap_target,
            ..
        } = self.interaction
        {
            snapshot.pending_edge = self.draw_edge_preview_curve(anchor_handle, anchor_is_source, fixed_target, cursor, snap_target);
        } else if let Some(conn) = self.proximity_connection {
            snapshot.pending_edge = self.proximity_preview_curve(conn);
        }
        let _stroke_scene = encode_board_stroke_scene(&snapshot.edges, 2.0);
        let _ = _stroke_scene.encoding().path_tags.len();
        snapshot
    }

    pub fn drain_events(&mut self) -> Vec<BoardEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn edge_curve(&self, edge_id: EdgeId) -> Option<CubicBez> {
        let edge = self.edges.get(&edge_id)?;
        let (source_position, target_position, source_node, target_node) = self.endpoint_wire_nodes(edge)?;
        Some(self.wire_bezier_between(source_position, target_position, source_node, target_node))
    }

    fn wire_bezier_between(
        &self,
        source_point: Point,
        target_point: Point,
        source_node: Option<&Node>,
        target_node: Option<&Node>,
    ) -> CubicBez {
        let chord = normalize_or_zero(target_point - source_point);
        let source_out = source_node
            .and_then(|node| {
                handle_outward_at_node_rim(source_point, node.center, node.shape, node.radius, node.width, node.height)
            })
            .filter(|outward| outward.hypot() > f64::EPSILON)
            .unwrap_or(chord);
        let target_out = target_node
            .and_then(|node| {
                handle_outward_at_node_rim(target_point, node.center, node.shape, node.radius, node.width, node.height)
            })
            .filter(|outward| outward.hypot() > f64::EPSILON)
            .unwrap_or(-chord);
        compute_edge_bezier_outward(source_point, target_point, source_out, target_out)
    }

    fn draw_edge_preview_curve(
        &self,
        anchor_handle: HandleId,
        anchor_is_source: bool,
        fixed_target: Option<HandleId>,
        cursor: Point,
        snap_target: Option<HandleId>,
    ) -> Option<CubicBez> {
        if let Some(fixed_tgt) = fixed_target {
            let tgt_h = self.handles.get(&fixed_tgt)?;
            let tgt_node = self.nodes.get(&tgt_h.node_id)?;
            let target_point = handle_position(tgt_node, tgt_h);
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let source_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(source_point, target_point, Some(snap_node), Some(tgt_node)));
            }
            return Some(self.wire_bezier_between(cursor, target_point, None, Some(tgt_node)));
        }
        let anchor = self.handles.get(&anchor_handle)?;
        let anchor_node = self.nodes.get(&anchor.node_id)?;
        let anchor_point = handle_position(anchor_node, anchor);
        if anchor_is_source {
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let snap_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(anchor_point, snap_point, Some(anchor_node), Some(snap_node)));
            }
            Some(self.wire_bezier_between(anchor_point, cursor, Some(anchor_node), None))
        } else if let Some(snap) = snap_target {
            let snap_h = self.handles.get(&snap)?;
            let snap_node = self.nodes.get(&snap_h.node_id)?;
            let snap_point = handle_position(snap_node, snap_h);
            Some(self.wire_bezier_between(snap_point, anchor_point, Some(snap_node), Some(anchor_node)))
        } else {
            Some(self.wire_bezier_between(cursor, anchor_point, None, Some(anchor_node)))
        }
    }

    fn active_proximity_distance_world(&self) -> f64 {
        self.proximity_distance_override
            .unwrap_or(self.proximity_distance_world)
    }

    fn proximity_enabled(&self) -> bool {
        self.active_proximity_distance_world() > 0.0
    }

    fn wire_snap_drag_tolerance_world(&self, handle_radius: f64) -> f64 {
        if !self.proximity_enabled() {
            return 0.0;
        }
        self.active_proximity_distance_world() + handle_radius
    }

    fn wire_snap_still_active(&self, handle_id: HandleId, cursor: Point) -> bool {
        if !self.proximity_enabled() {
            return false;
        }
        let Some(handle) = self.handles.get(&handle_id) else {
            return false;
        };
        let Some(node) = self.nodes.get(&handle.node_id) else {
            return false;
        };
        let pos = handle_position(node, handle);
        distance(cursor, pos) <= self.wire_snap_drag_tolerance_world(handle.radius)
    }

    fn nearest_wire_snap_handle(
        &self,
        anchor_handle: HandleId,
        anchor_is_source: bool,
        fixed_target: Option<HandleId>,
        reconnecting: Option<EdgeId>,
        cursor: Point,
    ) -> Option<HandleId> {
        if !P::HAS_PORTS || !self.proximity_enabled() {
            return None;
        }
        let mut best: Option<(f64, HandleId)> = None;
        for handle in self.handles.values() {
            let candidate = handle.id;
            if candidate == anchor_handle {
                continue;
            }
            let (source_hid, target_hid) = if let Some(fixed) = fixed_target {
                if anchor_is_source {
                    (candidate, fixed)
                } else {
                    (fixed, candidate)
                }
            } else if anchor_is_source {
                (anchor_handle, candidate)
            } else {
                (candidate, anchor_handle)
            };
            if !self.is_valid_connection(source_hid, target_hid, reconnecting, true) {
                continue;
            }
            let Some(node) = self.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let d = distance(cursor, pos);
            let tol = self.wire_snap_drag_tolerance_world(handle.radius);
            if d <= tol && best.as_ref().map(|(best_d, _)| d < *best_d).unwrap_or(true) {
                best = Some((d, candidate));
            }
        }
        best.map(|(_, id)| id)
    }

    fn endpoint_wire_nodes(&self, edge: &GraphEdge<P::Endpoint>) -> Option<(Point, Point, Option<&Node>, Option<&Node>)> {
        if P::HAS_PORTS {
            let source_handle = self.handles.get(&P::endpoint_as_handle(edge.source)?)?;
            let target_handle = self.handles.get(&P::endpoint_as_handle(edge.target)?)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let target_node = self.nodes.get(&target_handle.node_id)?;
            let source_position = handle_position(source_node, source_handle);
            let target_position = handle_position(target_node, target_handle);
            return Some((source_position, target_position, Some(source_node), Some(target_node)));
        }
        let source_node = self.nodes.get(&P::endpoint_as_u64(edge.source))?;
        let target_node = self.nodes.get(&P::endpoint_as_u64(edge.target))?;
        Some((source_node.center, target_node.center, Some(source_node), Some(target_node)))
    }

    fn remove_handle(&mut self, id: HandleId) {
        self.handles.remove(&id);
        let removed_edges: Vec<EdgeId> = self
            .edges
            .values()
            .filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id)
            .map(|edge| edge.id)
            .collect();
        for edge_id in removed_edges {
            self.edges.remove(&edge_id);
            self.selection.edge_ids.remove(&edge_id);
        }
        self.selection.handle_ids.remove(&id);
    }

    fn apply_pick_selection(&mut self, hit: HitObject<P::Endpoint>, extend_selection: bool) {
        let mode = if extend_selection { "additive" } else { "replace" };
        self.apply_pick_with_mode(hit, mode);
    }

    fn apply_pick_with_mode(&mut self, hit: HitObject<P::Endpoint>, mode: &str) {
        match hit {
            HitObject::Node(id) => {
                let current: BTreeSet<String> = self.selection.node_ids.iter().map(|nid| nid.to_string()).collect();
                let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                self.selection.node_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
            }
            HitObject::Endpoint(ep) => {
                let id = P::endpoint_as_u64(ep);
                if P::HAS_PORTS {
                    let current: BTreeSet<String> = self.selection.handle_ids.iter().map(|hid| hid.to_string()).collect();
                    let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                    self.selection.handle_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
                } else {
                    let current: BTreeSet<String> = self.selection.node_ids.iter().map(|nid| nid.to_string()).collect();
                    let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                    self.selection.node_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
                }
            }
            HitObject::Edge(id) => {
                let current: BTreeSet<String> = self.selection.edge_ids.iter().map(|eid| eid.to_string()).collect();
                let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                self.selection.edge_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
            }
        }
        self.clear_preselect();
        self.push_selection_event();
    }

    fn clear_preselect(&mut self) {
        self.preselect = Selection::default();
        self.preselect_removed = Selection::default();
    }

    fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
        if screen_points.len() < 2 {
            self.selection_preview_points.clear();
            self.selection_preview_crossing = false;
            return;
        }
        self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
        self.selection_preview_points = selection_screen_overlay_points(self.selection_options.method.as_str(), start_screen, screen_points).unwrap_or_default();
    }

    fn area_initial_string_set(&self) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        for id in &self.area_initial.node_ids {
            set.insert(id.to_string());
        }
        for id in &self.area_initial.handle_ids {
            set.insert(id.to_string());
        }
        for id in &self.area_initial.edge_ids {
            set.insert(id.to_string());
        }
        set
    }

    fn resolve_area_hits(&self, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
        let Some((box_, enclosing, ref polygon)) = selection_drag_shape(self.selection_options.method.as_str(), start, points) else {
            return initial.clone();
        };
        let lasso = self.selection_options.method == "lasso";
        let mut hits = BTreeSet::new();
        if self.selection_options.select_nodes {
            for node in self.nodes.values() {
                if selection_contains_node_bounds(node, box_, enclosing, polygon, lasso) {
                    hits.insert(node.id.to_string());
                }
            }
        }
        if self.selection_options.select_handles && P::HAS_PORTS {
            for handle in self.handles.values() {
                if let Some(node) = self.nodes.get(&handle.node_id) {
                    let pos = handle_position(node, handle);
                    if selection_contains_handle_point(pos, handle.radius.max(6.0), box_, enclosing, polygon, lasso) {
                        hits.insert(handle.id.to_string());
                    }
                }
            }
        }
        if self.selection_options.select_edges {
            for edge in self.edges.keys() {
                if let Some(curve) = self.edge_curve(*edge) {
                    if selection_contains_edge_curve(curve, box_, enclosing, polygon, lasso) {
                        hits.insert(edge.to_string());
                    }
                }
            }
        }
        merge_ids_into_selection(initial, &hits, merge_mode)
    }

    fn selection_to_string_set(&self) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        for id in &self.selection.node_ids {
            set.insert(id.to_string());
        }
        for id in &self.selection.handle_ids {
            set.insert(id.to_string());
        }
        for id in &self.selection.edge_ids {
            set.insert(id.to_string());
        }
        set
    }

    fn selection_from_string_set(&self, ids: &BTreeSet<String>) -> Selection {
        let mut selection = Selection::default();
        for id in ids {
            if let Ok(nid) = id.parse::<NodeId>() {
                if self.nodes.contains_key(&nid) {
                    selection.node_ids.insert(nid);
                    continue;
                }
            }
            if let Ok(hid) = id.parse::<HandleId>() {
                if self.handles.contains_key(&hid) {
                    selection.handle_ids.insert(hid);
                    continue;
                }
            }
            if let Ok(eid) = id.parse::<EdgeId>() {
                if self.edges.contains_key(&eid) {
                    selection.edge_ids.insert(eid);
                }
            }
        }
        selection
    }

    fn apply_area_preselect(&mut self, start: Point, points: &[Point], shift: bool, ctrl_or_meta: bool) {
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let anchor = self.area_initial_string_set();
        let next_ids: Vec<String> = self.resolve_area_hits(&anchor, start, points, merge_mode.as_str()).into_iter().collect();
        let (sorted, removed) = area_preselect_ids(&anchor, &next_ids);
        let next = self.selection_from_string_set(&next_ids.iter().cloned().collect());
        let removed_sel = self.selection_from_string_set(&removed.iter().cloned().collect());
        if self.preselect == next && self.preselect_removed == removed_sel {
            return;
        }
        self.preselect = next;
        self.preselect_removed = removed_sel;
        let _ = sorted;
        self.push_preselect_event();
    }

    fn commit_selection_from_hits(&mut self, hits: &BTreeSet<String>) {
        self.selection = self.selection_from_string_set(hits);
        self.clear_preselect();
        self.push_selection_event();
    }

    fn push_preselect_event(&mut self) {
        self.events.push(BoardEvent::PreselectChanged {
            node_ids: self.preselect.node_ids.iter().copied().collect(),
            handle_ids: self.preselect.handle_ids.iter().copied().collect(),
            edge_ids: self.preselect.edge_ids.iter().copied().collect(),
            removed_node_ids: self.preselect_removed.node_ids.iter().copied().collect(),
            removed_handle_ids: self.preselect_removed.handle_ids.iter().copied().collect(),
            removed_edge_ids: self.preselect_removed.edge_ids.iter().copied().collect(),
        });
    }

    fn update_hover(&mut self, hover: Option<u64>) {
        if self.hover == hover {
            return;
        }
        self.hover = hover;
        self.events.push(BoardEvent::HoverChanged { id: hover });
    }

    fn push_selection_event(&mut self) {
        self.events.push(BoardEvent::SelectionChanged {
            edge_ids: self.selection.edge_ids.iter().copied().collect(),
            handle_ids: self.selection.handle_ids.iter().copied().collect(),
            node_ids: self.selection.node_ids.iter().copied().collect(),
        });
    }

    fn begin_draw_edge_from_handle(&mut self, handle_id: HandleId, cursor: Point) {
        let Some(handle) = self.handles.get(&handle_id).cloned() else {
            self.interaction = InteractionMode::Idle;
            return;
        };
        let incoming = self.incoming_edge_for_handle(handle_id);
        let reconnect_from_target = |edge_id: EdgeId, target_hid: HandleId| -> Option<(HandleId, bool, Option<HandleId>, Option<EdgeId>)> {
            let src = P::endpoint_as_u64(self.edges.get(&edge_id)?.source);
            Some((src, true, Some(target_hid), Some(edge_id)))
        };
        let (anchor_handle, anchor_is_source, fixed_target, reconnecting) = match handle.role {
            HandleRole::Target => incoming
                .and_then(|e| reconnect_from_target(e, handle_id))
                .unwrap_or((handle_id, false, None, None)),
            HandleRole::Source => (handle_id, true, None, None),
            HandleRole::Any => incoming
                .and_then(|e| reconnect_from_target(e, handle_id))
                .unwrap_or((handle_id, true, None, None)),
        };
        self.interaction = InteractionMode::DrawEdge {
            anchor_handle,
            anchor_is_source,
            fixed_target,
            cursor,
            reconnecting,
            snap_target: None,
        };
    }

    fn incoming_edge_for_handle(&self, handle_id: HandleId) -> Option<EdgeId> {
        self.edges
            .values()
            .find(|edge| P::endpoint_as_u64(edge.target) == handle_id)
            .map(|edge| edge.id)
    }

    fn displaced_incoming_edge(&self, target_hid: HandleId, reconnecting: Option<EdgeId>) -> Option<EdgeId> {
        self.edges
            .values()
            .find(|edge| Some(edge.id) != reconnecting && P::endpoint_as_u64(edge.target) == target_hid)
            .map(|edge| edge.id)
    }

    fn is_valid_connection(
        &self,
        source_hid: HandleId,
        target_hid: HandleId,
        reconnecting: Option<EdgeId>,
        allow_target_replace: bool,
    ) -> bool {
        if source_hid == target_hid {
            return false;
        }
        let Some(source_handle) = self.handles.get(&source_hid) else {
            return false;
        };
        let Some(target_handle) = self.handles.get(&target_hid) else {
            return false;
        };
        if source_handle.node_id == target_handle.node_id {
            return false;
        }
        if !matches!(source_handle.role, HandleRole::Source | HandleRole::Any) {
            return false;
        }
        if !matches!(target_handle.role, HandleRole::Target | HandleRole::Any) {
            return false;
        }
        if self.edges.values().any(|e| {
            Some(e.id) != reconnecting && P::endpoint_as_u64(e.source) == source_hid && P::endpoint_as_u64(e.target) == target_hid
        }) {
            return false;
        }
        if !allow_target_replace
            && self
                .edges
                .values()
                .any(|e| Some(e.id) != reconnecting && P::endpoint_as_u64(e.target) == target_hid)
        {
            return false;
        }
        if self.enforce_acyclic {
            let src_node = source_handle.node_id;
            let tgt_node = target_handle.node_id;
            let excluding = reconnecting.or_else(|| {
                if allow_target_replace {
                    self.displaced_incoming_edge(target_hid, reconnecting)
                } else {
                    None
                }
            });
            if self.would_create_cycle_between_nodes(src_node, tgt_node, excluding) {
                return false;
            }
        }
        true
    }

    fn try_connect_handles(&mut self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>) -> bool {
        if !self.is_valid_connection(source_hid, target_hid, reconnecting, true) {
            return false;
        }
        let new_id = reconnecting.unwrap_or_else(|| {
            let id = self.next_edge_id;
            self.next_edge_id += 1;
            id
        });
        if let Some(old_id) = reconnecting.or_else(|| self.incoming_edge_for_handle(target_hid)) {
            self.remove_edge(old_id);
        }
        let (Some(src_ep), Some(tgt_ep)) = (P::try_handle_endpoint(source_hid), P::try_handle_endpoint(target_hid)) else {
            return false;
        };
        self.create_edge(new_id, src_ep, tgt_ep);
        self.events.push(BoardEvent::EdgeConnected {
            id: new_id,
            source: source_hid,
            target: target_hid,
        });
        true
    }

    fn connection_pairs_for_handles(&self, a: &Handle, b: &Handle) -> Vec<(HandleId, HandleId)> {
        let mut pairs = Vec::new();
        match (a.role, b.role) {
            (HandleRole::Source, HandleRole::Target) => pairs.push((a.id, b.id)),
            (HandleRole::Target, HandleRole::Source) => pairs.push((b.id, a.id)),
            (HandleRole::Source, HandleRole::Any) => pairs.push((a.id, b.id)),
            (HandleRole::Any, HandleRole::Source) => pairs.push((b.id, a.id)),
            (HandleRole::Target, HandleRole::Any) => pairs.push((b.id, a.id)),
            (HandleRole::Any, HandleRole::Target) => pairs.push((a.id, b.id)),
            (HandleRole::Any, HandleRole::Any) => {
                pairs.push((a.id, b.id));
                pairs.push((b.id, a.id));
            }
            _ => {}
        }
        pairs
    }

    fn update_node_drag_proximity(&mut self, dragged_ids: &[NodeId]) {
        self.proximity_connection = None;
        if !self.proximity_enabled() || !P::HAS_PORTS {
            return;
        }
        let dragged: std::collections::BTreeSet<NodeId> = dragged_ids.iter().copied().collect();
        let mut best: Option<(f64, ProximityConnection)> = None;
        for dragged_handle in self.handles.values().filter(|h| dragged.contains(&h.node_id)) {
            for other_handle in self.handles.values().filter(|h| !dragged.contains(&h.node_id)) {
                for (source_hid, target_hid) in self.connection_pairs_for_handles(dragged_handle, other_handle) {
                    if !self.is_valid_connection(source_hid, target_hid, None, false) {
                        continue;
                    }
                    let Some(source_handle) = self.handles.get(&source_hid) else {
                        continue;
                    };
                    let Some(target_handle) = self.handles.get(&target_hid) else {
                        continue;
                    };
                    let Some(source_node) = self.nodes.get(&source_handle.node_id) else {
                        continue;
                    };
                    let Some(target_node) = self.nodes.get(&target_handle.node_id) else {
                        continue;
                    };
                    let src_pos = handle_position(source_node, source_handle);
                    let tgt_pos = handle_position(target_node, target_handle);
                    let d = distance(src_pos, tgt_pos);
                    let tol = self.active_proximity_distance_world() + source_handle.radius + target_handle.radius;
                    if d <= tol && best.as_ref().map(|(best_d, _)| d < *best_d).unwrap_or(true) {
                        best = Some((d, ProximityConnection { source: source_hid, target: target_hid }));
                    }
                }
            }
        }
        if let Some((_, conn)) = best {
            self.proximity_connection = Some(conn);
            self.update_hover(Some(conn.target));
        }
    }

    fn proximity_preview_curve(&self, conn: ProximityConnection) -> Option<CubicBez> {
        let source_handle = self.handles.get(&conn.source)?;
        let target_handle = self.handles.get(&conn.target)?;
        let source_node = self.nodes.get(&source_handle.node_id)?;
        let target_node = self.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        Some(self.wire_bezier_between(
            source_position,
            target_position,
            Some(source_node),
            Some(target_node),
        ))
    }

    fn would_create_cycle_between_nodes(&self, source: NodeId, target: NodeId, excluding: Option<EdgeId>) -> bool {
        if source == target {
            return true;
        }
        let mut adj: std::collections::HashMap<NodeId, Vec<NodeId>> = std::collections::HashMap::new();
        for edge in self.edges.values() {
            if excluding == Some(edge.id) {
                continue;
            }
            let Some(src_h) = self.handles.get(&P::endpoint_as_u64(edge.source)) else {
                continue;
            };
            let Some(tgt_h) = self.handles.get(&P::endpoint_as_u64(edge.target)) else {
                continue;
            };
            adj.entry(src_h.node_id).or_default().push(tgt_h.node_id);
        }
        adj.entry(source).or_default().push(target);
        Self::has_path_nodes(&adj, target, source)
    }

    fn has_path_nodes(adj: &std::collections::HashMap<NodeId, Vec<NodeId>>, from: NodeId, to: NodeId) -> bool {
        let mut seen = std::collections::HashSet::new();
        let mut stack = vec![from];
        while let Some(n) = stack.pop() {
            if n == to {
                return true;
            }
            if !seen.insert(n) {
                continue;
            }
            if let Some(next) = adj.get(&n) {
                for m in next {
                    stack.push(*m);
                }
            }
        }
        false
    }

    fn hit_test(&self, point: Point) -> Option<HitObject<P::Endpoint>> {
        if P::HAS_PORTS && self.handle_pointer_picking {
            for handle in self.handles.values().rev() {
                let node = self.nodes.get(&handle.node_id)?;
                if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
                    if let Some(ep) = P::try_handle_endpoint(handle.id) {
                        return Some(HitObject::Endpoint(ep));
                    }
                }
            }
        }
        for node in self.nodes.values().rev() {
            if node_contains_point(node, point) {
                return Some(HitObject::Node(node.id));
            }
        }
        for edge in self.edges.values().rev() {
            if let Some(curve) = self.edge_curve(edge.id) {
                if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
                    return Some(HitObject::Edge(edge.id));
                }
            }
        }
        None
    }
}
// #endregion 🔖Engine

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn port_directed_engine_round_trip() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(11, 1, std::f64::consts::PI);
        engine.create_edge(100, 10, 11);
        let snap = engine.render_snapshot();
        assert_eq!(snap.nodes.len(), 1);
        assert_eq!(snap.handles.len(), 2);
        assert_eq!(snap.edges.len(), 1);
    }

    #[test]
    fn normal_directed_node_edges() {
        let mut engine = GraphEngine::<Normal, Directed>::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 120.0, 0.0, 40.0, true);
        engine.create_edge(100, 1, 2);
        let snap = engine.render_snapshot();
        assert_eq!(snap.nodes.len(), 2);
        assert!(snap.handles.is_empty());
        assert_eq!(snap.edges.len(), 1);
    }

    #[test]
    fn undirected_normalizes_endpoints() {
        let mut engine = GraphEngine::<Normal, Undirected>::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 120.0, 0.0, 40.0, true);
        engine.create_edge(100, 2, 1);
        let edge = engine.edges.get(&100).unwrap();
        assert_eq!(edge.source, 1);
        assert_eq!(edge.target, 2);
    }

    #[test]
    fn selection_union_drag_starts_inside_bounds_without_node_hit() {
        use cavas::vello::kurbo::Point;

        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
        engine.create_rect_node(2, 200.0, 0.0, 80.0, 56.0, true);
        engine.selection.node_ids.insert(1);
        engine.selection.node_ids.insert(2);
        let gap = Point::new(100.0, 0.0);
        assert!(engine.try_begin_selection_union_drag_at(gap, 0.0));
        assert!(matches!(engine.interaction, InteractionMode::DragNodes { .. }));
        engine.pointer_move(150.0, 30.0);
        engine.pointer_up(150.0, 30.0);
        let a = engine.nodes.get(&1).unwrap().center;
        let b = engine.nodes.get(&2).unwrap().center;
        assert!((a.x - 50.0).abs() < 1e-3 && (a.y - 30.0).abs() < 1e-3);
        assert!((b.x - 250.0).abs() < 1e-3 && (b.y - 30.0).abs() < 1e-3);
    }

    #[test]
    fn selection_drag_enclosing_lasso_uses_first_horizontal_step() {
        use cavas::vello::kurbo::Point;

        let start = Point::new(100.0, 100.0);
        let left_first = vec![start, Point::new(80.0, 100.0), Point::new(120.0, 100.0)];
        assert!(!selection_drag_enclosing("lasso", start, &left_first));
        let right_first = vec![start, Point::new(120.0, 100.0), Point::new(80.0, 100.0)];
        assert!(selection_drag_enclosing("lasso", start, &right_first));
        let rectangle = vec![start, Point::new(80.0, 100.0)];
        assert!(!selection_drag_enclosing("rectangle", start, &rectangle));
        assert!(!selection_drag_enclosing_rectangle(start, Point::new(80.0, 100.0)));
    }

    #[test]
    fn rect_node_drags_from_center() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.create_rect_node(1, 100.0, 50.0, 160.0, 72.0, true);
        engine.pointer_down(100.0, 50.0, false);
        assert!(matches!(engine.interaction, InteractionMode::DragNode { .. }));
        engine.pointer_move(140.0, 80.0);
        engine.pointer_up(140.0, 80.0);
        let c = engine.nodes.get(&1).unwrap().center;
        assert!((c.x - 140.0).abs() < 0.01);
        assert!((c.y - 80.0).abs() < 0.01);
    }

    #[test]
    fn rect_node_hit_and_wire_connect() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 220.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.pointer_down(80.0, 0.0, false);
        engine.pointer_up(140.0, 0.0);
        assert_eq!(engine.edges.len(), 1);
        assert!(engine.render_snapshot().pending_edge.is_none());
    }

    #[test]
    fn reconnect_replaces_incoming_edge() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
        engine.create_rect_node(2, 160.0, 0.0, 80.0, 56.0, true);
        engine.create_rect_node(3, 320.0, 0.0, 80.0, 56.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(12, 3, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Source);
        engine.set_handle_role(12, HandleRole::Target);
        engine.create_edge(4, 11, 12);
        use cavas::vello::kurbo::Point;
        let tgt = handle_position_on_rectangle(Point::new(320.0, 0.0), 80.0, 56.0, std::f64::consts::FRAC_PI_2);
        let src = handle_position_on_rectangle(Point::new(0.0, 0.0), 80.0, 56.0, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.pointer_down(tgt.x, tgt.y, false);
        engine.pointer_move(src.x, src.y);
        engine.pointer_up(src.x, src.y);
        assert_eq!(engine.edges.len(), 1);
        let edge = engine.edges.values().next().unwrap();
        assert_eq!(edge.source, 10);
        assert_eq!(edge.target, 12);
    }

    #[test]
    fn draw_edge_preview_uses_bezier_from_source_and_target() {
        use cavas::vello::kurbo::{ParamCurve, Point};

        fn midpoint_bulge(curve: CubicBez) -> f64 {
            let p0 = curve.p0;
            let p3 = curve.p3;
            let mid = curve.eval(0.5);
            let chord = p3 - p0;
            let len = chord.hypot();
            if len <= f64::EPSILON {
                return 0.0;
            }
            let t = ((mid - p0).dot(chord)) / (len * len);
            let proj = p0 + chord * t;
            (mid - proj).hypot()
        }

        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.proximity_distance_world = 0.0;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
        let cursor = Point::new(220.0, 40.0);
        engine.pointer_down(out.x, out.y, false);
        engine.pointer_move(cursor.x, cursor.y);
        let from_source = engine.render_snapshot().pending_edge.expect("source drag preview");
        assert!(midpoint_bulge(from_source) > 1.0, "source-anchored preview should bow away from chord");
        engine.pointer_up(cursor.x, cursor.y);

        let inp = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
        engine.pointer_down(inp.x, inp.y, false);
        engine.pointer_move(cursor.x, cursor.y);
        let from_target = engine.render_snapshot().pending_edge.expect("target drag preview");
        assert!(midpoint_bulge(from_target) > 1.0, "target-anchored preview should bow away from chord");
    }

    #[test]
    fn wire_snaps_preview_and_connects_to_compatible_handle() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.set_camera(0.0, 0.0, 1.0);
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
        let inp = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
        engine.pointer_down(out.x, out.y, false);
        let near = Point::new(inp.x + 24.0, inp.y);
        engine.pointer_move(near.x, near.y);
        let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
            panic!("expected draw-edge interaction");
        };
        assert_eq!(snap_target, Some(11));
        let preview = engine.render_snapshot().pending_edge.expect("preview");
        assert!((preview.p3.x - inp.x).abs() < 0.01);
        assert!((preview.p3.y - inp.y).abs() < 0.01);
        engine.pointer_up(near.x, near.y);
        assert_eq!(engine.edges.len(), 1);
        let edge = engine.edges.values().next().expect("edge");
        assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
        assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
    }

    #[test]
    fn wire_snap_replaces_occupied_compatible_target() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.set_camera(0.0, 0.0, 1.0);
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.set_handle_role(12, HandleRole::Source);
        engine.create_edge(100, 12, 11);
        let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
        let occupied = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
        engine.pointer_down(out.x, out.y, false);
        engine.pointer_move(occupied.x + 8.0, occupied.y);
        let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
            panic!("expected draw-edge interaction");
        };
        assert_eq!(snap_target, Some(11));
        engine.pointer_up(occupied.x + 8.0, occupied.y);
        assert_eq!(engine.edges.len(), 1);
        let edge = engine.edges.values().next().expect("edge");
        assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
        assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
    }

    #[test]
    fn wire_snap_ignores_incompatible_handles() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.set_camera(0.0, 0.0, 1.0);
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(12, HandleRole::Source);
        let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
        let other_out = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.pointer_down(out.x, out.y, false);
        engine.pointer_move(other_out.x + 8.0, other_out.y);
        let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
            panic!("expected draw-edge interaction");
        };
        assert!(snap_target.is_none());
    }

    #[test]
    fn proximity_zero_disables_wire_snap() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.proximity_distance_world = 0.0;
        engine.enforce_acyclic = true;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 280.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
        let inp = handle_position_on_rectangle(Point::new(280.0, 0.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
        engine.pointer_down(out.x, out.y, false);
        engine.pointer_move(inp.x + 8.0, inp.y);
        let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
            panic!("expected draw-edge interaction");
        };
        assert!(snap_target.is_none());
    }

    #[test]
    fn node_drag_proximity_connects_compatible_channels() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.proximity_distance_world = 80.0;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.pointer_down(0.0, 0.0, false);
        engine.pointer_move(40.0, 0.0);
        assert!(engine.render_snapshot().pending_edge.is_some());
        engine.pointer_up(40.0, 0.0);
        assert_eq!(engine.edges.len(), 1);
        let edge = engine.edges.values().next().expect("edge");
        assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
        assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
    }

    #[test]
    fn node_drag_proximity_skips_occupied_input() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.proximity_distance_world = 80.0;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(3, 400.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.create_handle(12, 3, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.set_handle_role(12, HandleRole::Source);
        engine.create_edge(100, 10, 11);
        engine.pointer_down(400.0, 0.0, false);
        engine.pointer_move(220.0, 0.0);
        assert!(engine.render_snapshot().pending_edge.is_none());
        engine.pointer_up(220.0, 0.0);
        assert_eq!(engine.edges.len(), 1);
        let edge = engine.edges.values().next().expect("edge");
        assert_eq!(Ported::endpoint_as_u64(edge.source), 10);
        assert_eq!(Ported::endpoint_as_u64(edge.target), 11);
    }

    #[test]
    fn node_drag_alt_suppresses_proximity_preview_and_connect() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.proximity_distance_world = 80.0;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.pointer_down(0.0, 0.0, false);
        engine.pointer_move_screen(40.0, 0.0, 40.0, 0.0, false, false, true);
        assert!(engine.render_snapshot().pending_edge.is_none());
        engine.pointer_up_screen(40.0, 0.0, 40.0, 0.0, false, false, true);
        assert_eq!(engine.edges.len(), 0);
        assert_eq!(engine.proximity_distance_world, 80.0);
    }

    #[test]
    fn node_drag_alt_restores_proximity_when_released() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.proximity_distance_world = 80.0;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 200.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.pointer_down(0.0, 0.0, false);
        engine.pointer_move_screen(40.0, 0.0, 40.0, 0.0, false, false, true);
        assert!(engine.render_snapshot().pending_edge.is_none());
        engine.pointer_move_screen(40.0, 0.0, 40.0, 0.0, false, false, false);
        assert!(engine.render_snapshot().pending_edge.is_some());
        assert_eq!(engine.proximity_distance_world, 80.0);
    }

    #[test]
    fn wire_snap_connects_fan_out_from_same_output() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.set_camera(0.0, 0.0, 1.0);
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 280.0, -60.0, 160.0, 72.0, true);
        engine.create_rect_node(3, 280.0, 60.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.create_handle(12, 3, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.set_handle_role(12, HandleRole::Target);
        engine.create_edge(100, 10, 11);
        let out = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, 3.0 * std::f64::consts::FRAC_PI_2);
        let second = handle_position_on_rectangle(Point::new(280.0, 60.0), 160.0, 72.0, std::f64::consts::FRAC_PI_2);
        engine.pointer_down(out.x, out.y, false);
        engine.pointer_move(second.x + 8.0, second.y);
        let InteractionMode::DrawEdge { snap_target, .. } = engine.interaction else {
            panic!("expected draw-edge interaction");
        };
        assert_eq!(snap_target, Some(12));
        engine.pointer_up(second.x + 8.0, second.y);
        assert_eq!(engine.edges.len(), 2);
        assert!(engine.edges.values().any(|edge| Ported::endpoint_as_u64(edge.source) == 10 && Ported::endpoint_as_u64(edge.target) == 11));
        assert!(engine.edges.values().any(|edge| Ported::endpoint_as_u64(edge.source) == 10 && Ported::endpoint_as_u64(edge.target) == 12));
    }

    #[test]
    fn node_drag_proximity_allows_fan_out_from_occupied_source() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.proximity_distance_world = 120.0;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 220.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.set_handle_role(12, HandleRole::Target);
        engine.create_edge(100, 10, 11);
        engine.pointer_down(220.0, 0.0, false);
        engine.pointer_move(40.0, 0.0);
        assert!(engine.render_snapshot().pending_edge.is_some());
        engine.pointer_up(40.0, 0.0);
        assert_eq!(engine.edges.len(), 2);
        assert!(engine.edges.values().any(|edge| Ported::endpoint_as_u64(edge.source) == 10 && Ported::endpoint_as_u64(edge.target) == 12));
    }

    #[test]
    fn pick_merge_mode_for_modifiers_matches_puzzle() {
        assert_eq!(pick_merge_mode_for_modifiers(false, false, "replace"), "replace");
        assert_eq!(pick_merge_mode_for_modifiers(false, true, "replace"), "additive");
        assert_eq!(pick_merge_mode_for_modifiers(true, false, "replace"), "subtractive");
        assert_eq!(pick_merge_mode_for_modifiers(true, true, "replace"), "invertive");
    }

    #[test]
    fn acyclic_rejects_back_edge() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
        engine.create_rect_node(2, 160.0, 0.0, 80.0, 56.0, true);
        engine.create_handle(10, 1, std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.create_edge(100, 10, 11);
        engine.pointer_down(160.0, 0.0, false);
        engine.pointer_up(0.0, 0.0);
        assert_eq!(engine.edges.len(), 1);
    }
}
// #endregion 🔖Tests
