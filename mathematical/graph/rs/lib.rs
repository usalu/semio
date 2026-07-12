//! 🕸️ Generic property graph engine on infinite canvas; specialize via quadrant crates.

pub mod geometry {
    // #region geometry
    //! 📐 Graph geometry: handle positions, edge beziers, hit-test distances.

    use crate::cavas::{append_shape_to_path, Affine, Arc, BezPath, Circle, CubicBez, Point, Rect, Stroke, Vec2};
    use crate::cavas::Color;
    use crate::cavas::Scene;
    use crate::NodeShape;
    pub use mathematical_geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, distance_point_to_polyline, normalize_or_zero, ray_from_origin_to_axis_aligned_rectangle_edge};

    // 🚧 Duplicated in `mathematical_graph_drawing::routing` (the canonical home); deleted here once `GraphEngine` moves to `infinite_board` in a later phase and no longer calls these internally.

    /// 🕳️ Even-odd clip path: local outer bounds minus the parent node body (keeps handle paint outside transparent nodes).
    pub fn handle_outside_node_clip_path(handle_center: Point, handle_radius: f64, node_center: Point, node_shape: NodeShape, node_radius: f64, node_width: f64, node_height: f64) -> BezPath {
        let margin = (handle_radius * 2.5).max(4.0);
        let outer = Rect::new(handle_center.x - margin, handle_center.y - margin, handle_center.x + margin, handle_center.y + margin);
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &outer, 0.1);
        match node_shape {
            NodeShape::Circle => {
                append_shape_to_path(&mut path, &Circle::new(node_center, node_radius.max(1e-9)), 0.1);
            }
            NodeShape::Rectangle => {
                let hw = node_width.max(1e-9) * 0.5;
                let hh = node_height.max(1e-9) * 0.5;
                append_shape_to_path(&mut path, &Rect::new(node_center.x - hw, node_center.y - hh, node_center.x + hw, node_center.y + hh), 0.1);
            }
        }
        path
    }

    /// 🧭 Outward normal for a handle on a node rim: edge-normal on rectangles, radial on circles.
    pub fn handle_outward_at_node_rim(handle: Point, node_center: Point, node_shape: NodeShape, node_radius: f64, node_width: f64, node_height: f64) -> Option<Vec2> {
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
            append_shape_to_path(&mut path, &arc, 0.1);
            path.close_path();
            return path;
        }
        append_shape_to_path(&mut path, &Circle::new(center, r), 0.1);
        path
    }

    /// 🌗 Open arc path for stroking only the exterior handle cap (flat rim edge stays behind the node).
    pub fn handle_exterior_cap_stroke_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let r = radius.max(1e-9);
        let mut path = BezPath::new();
        if let Some(arc) = handle_exterior_cap_arc(center, outward, r) {
            append_shape_to_path(&mut path, &arc, 0.1);
            return path;
        }
        append_shape_to_path(&mut path, &Circle::new(center, r), 0.1);
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

    /// 🎯 World point at the outer peak of a port handle cap (rim + outward × radius).
    pub fn handle_exterior_cap_peak(center: Point, outward: Vec2, radius: f64) -> Point {
        let out = normalize_or_zero(outward);
        let r = radius.max(0.0);
        if out.hypot() < 1e-9 || r <= 0.0 {
            return center;
        }
        center + out * r
    }

    /// 🔺 Closed fill path for a triangle handle cap pointing in the `outward` direction.
    pub fn handle_exterior_cap_triangle_fill_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let out = normalize_or_zero(outward);
        let r = radius.max(1e-9);
        if out.hypot() < 1e-9 {
            return handle_exterior_cap_fill_path(center, outward, r);
        }
        let perp = Vec2::new(-out.y, out.x);
        let peak = center + out * r;
        let base_half = r * 0.65;
        let base_left = center + perp * base_half;
        let base_right = center - perp * base_half;
        let mut path = BezPath::new();
        path.move_to(base_left);
        path.line_to(peak);
        path.line_to(base_right);
        path.close_path();
        path
    }

    /// 🔺 Open stroke path for a triangle handle cap.
    pub fn handle_exterior_cap_triangle_stroke_path(center: Point, outward: Vec2, radius: f64) -> BezPath {
        let out = normalize_or_zero(outward);
        let r = radius.max(1e-9);
        if out.hypot() < 1e-9 {
            return handle_exterior_cap_stroke_path(center, outward, r);
        }
        let perp = Vec2::new(-out.y, out.x);
        let peak = center + out * r;
        let base_half = r * 0.65;
        let base_left = center + perp * base_half;
        let base_right = center - perp * base_half;
        let mut path = BezPath::new();
        path.move_to(base_left);
        path.line_to(peak);
        path.line_to(base_right);
        path
    }

    /// 🔺 Wire attachment peak for a triangle handle cap.
    pub fn handle_exterior_cap_triangle_peak(center: Point, outward: Vec2, radius: f64) -> Point {
        handle_exterior_cap_peak(center, outward, radius)
    }

    /// 📐 Orthogonal S/Z polyline between two port cap peaks.
    pub fn compute_edge_sharp_sz_path(source_point: Point, target_point: Point, source_outward: Vec2, target_outward: Vec2) -> BezPath {
        let out_s = normalize_or_zero(source_outward);
        let out_t = normalize_or_zero(target_outward);
        let stub = 20.0;
        let p1 = source_point + out_s * stub;
        let p4 = target_point + out_t * stub;
        let mut path = BezPath::new();
        path.move_to(source_point);
        path.line_to(p1);
        if (p1.x - p4.x).abs() >= (p1.y - p4.y).abs() {
            let mid_x = (p1.x + p4.x) * 0.5;
            path.line_to(Point::new(mid_x, p1.y));
            path.line_to(Point::new(mid_x, p4.y));
        } else {
            let mid_y = (p1.y + p4.y) * 0.5;
            path.line_to(Point::new(p1.x, mid_y));
            path.line_to(Point::new(p4.x, mid_y));
        }
        path.line_to(p4);
        path.line_to(target_point);
        path
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
        let control_length = clamp_f64(handle_distance * 0.12, 8.0, 72.0);
        let p1 = source_point + source_radial * control_length;
        let p2 = target_point + target_radial * control_length;
        CubicBez::new(source_point, p1, p2, target_point)
    }

    pub fn compute_edge_bezier_points(source_point: Point, target_point: Point, source_center: Point, target_center: Point) -> CubicBez {
        compute_edge_bezier_outward(source_point, target_point, source_point - source_center, target_point - target_center)
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
                    assert!((bb.x1() - peak.x).abs() < 0.25, "east cap must peak at +x");
                    assert!(bb.x0() > trough.x + 0.25, "east cap must not peak inward");
                } else {
                    assert!((bb.x0() - peak.x).abs() < 0.25, "west cap must peak at -x");
                    assert!(bb.x1() < trough.x - 0.25, "west cap must not peak inward");
                }
            } else if out.y > 0.0 {
                assert!((bb.y1() - peak.y).abs() < 0.25, "south cap must peak at +y");
                assert!(bb.y0() > trough.y + 0.25, "south cap must not peak inward");
            } else {
                assert!((bb.y0() - peak.y).abs() < 0.25, "north cap must peak at -y");
                assert!(bb.y1() < trough.y + 0.25, "north cap must not peak inward");
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
        fn edge_bezier_starts_outside_handle_cap_peak() {
            let node_center = Point::new(100.0, 50.0);
            let width = 160.0;
            let height = 72.0;
            let rim = Point::new(node_center.x + width * 0.5, node_center.y);
            let outward = handle_outward_at_node_rim(rim, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
            let radius = 5.0;
            let peak = handle_exterior_cap_peak(rim, outward, radius);
            let target = Point::new(300.0, 50.0);
            let curve = compute_edge_bezier_outward(peak, target, outward, -normalize_or_zero(target - peak));
            let start = curve.eval(0.0);
            assert!((start.x - peak.x).abs() < 1e-9 && (start.y - peak.y).abs() < 1e-9);
            assert!(start.x > rim.x + 0.5, "edge must begin outside the port rim under the cap");
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
            assert!(!stroke.elements().iter().any(|el| matches!(el, crate::cavas::PathEl::ClosePath)));
        }

        #[test]
        fn triangle_cap_peak_matches_outward_direction() {
            let center = Point::new(40.0, 0.0);
            let outward = Vec2::new(1.0, 0.0);
            let radius = 5.0;
            let peak = handle_exterior_cap_triangle_peak(center, outward, radius);
            assert!((peak.x - (center.x + radius)).abs() < 1e-9);
            let fill = handle_exterior_cap_triangle_fill_path(center, outward, radius);
            assert!(fill.bounding_box().x1() > center.x);
        }

        #[test]
        fn sharp_sz_path_is_orthogonal_between_peaks() {
            let source = Point::new(0.0, 0.0);
            let target = Point::new(120.0, 40.0);
            let path = compute_edge_sharp_sz_path(source, target, Vec2::new(1.0, 0.0), Vec2::new(-1.0, 0.0));
            let mut line_count = 0;
            for el in path.elements() {
                if matches!(el, crate::cavas::PathEl::LineTo(_)) {
                    line_count += 1;
                }
            }
            assert!(line_count >= 3, "sharp S/Z path should contain multiple straight segments");
        }
    }

    pub fn encode_board_stroke_scene(curves: &[CubicBez], stroke_width: f64) -> Scene {
        let mut scene = Scene::new();
        let stroke = Stroke::new(stroke_width);
        for curve in curves {
            scene.stroke(&stroke, Affine::IDENTITY, Color::new(ui_styling::CANVAS_LIGHT.icon_bg), None, curve);
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
        pub locked: Option<bool>,
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

    /// 🔒 Resolves fixture element locked flag from JSON (`locked` only).
    pub fn board_json_locked_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
        obj.get("locked").and_then(|v| v.as_bool())
    }
    // #endregion scene_json
}

pub use geometry::{
    circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, encode_board_stroke_scene, handle_exterior_cap_fill_path, handle_exterior_cap_peak,
    handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle,
    rectangle_handle_angle_toward,
};
pub use mathematical_geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
pub use scene_json::{board_json_locked_option, board_json_visible_option, board_json_visible_or_true, CameraJson, NodeDescJson};

pub use infinite_cavas as cavas;
pub use mathematical_graph_manifest::{PropertyBag, PropertyValue};

// #region 🔖Ids
/// 🧩 Stable node identifier.
pub type NodeId = u64;
/// 🪝 Stable handle identifier.
pub type HandleId = u64;
/// 🪢 Stable edge identifier.
pub type EdgeId = u64;
// #endregion 🔖Ids

// #region 🔖Edge
/// 🪢 Edge with typed endpoints (node id or handle id).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoreEdge<E> {
    pub id: EdgeId,
    pub source: E,
    pub target: E,
}

impl<E: Copy + Ord> CoreEdge<E> {
    /// 📐 Normalize endpoints for undirected storage.
    pub fn normalize_undirected(source: E, target: E) -> (E, E) {
        if source <= target {
            (source, target)
        } else {
            (target, source)
        }
    }
}
// #endregion 🔖Edge

// #region 🔖Directedness
/// ↔️ Compile-time directed vs undirected graph axis.
pub trait Directedness {
    const DIRECTED: bool;
}

/// ➡️ Directed edges keep source→target order.
#[derive(Clone, Copy, Debug, Default)]
pub struct Directed;

impl Directedness for Directed {
    const DIRECTED: bool = true;
}

/// ↔️ Undirected edges store ordered endpoint pair.
#[derive(Clone, Copy, Debug, Default)]
pub struct Undirected;

impl Directedness for Undirected {
    const DIRECTED: bool = false;
}

/// 📐 Apply directedness when storing edge endpoints.
#[inline]
pub fn orient_endpoints<E: Copy + Ord, D: Directedness>(source: E, target: E) -> (E, E) {
    if D::DIRECTED {
        (source, target)
    } else {
        CoreEdge::<E>::normalize_undirected(source, target)
    }
}
// #endregion 🔖Directedness

// #region 🔖PortModel
/// 🔌 Compile-time normal (node) vs ported (handle) graph axis.
pub trait PortModel {
    type Endpoint: Copy + Ord + std::fmt::Debug;
    const HAS_PORTS: bool;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64;
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint>;
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId>;
}

/// 🟠 Node-to-node edges without handles.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl PortModel for Normal {
    type Endpoint = NodeId;
    const HAS_PORTS: bool = false;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(_: HandleId) -> Option<Self::Endpoint> {
        None
    }
    fn endpoint_as_handle(_: Self::Endpoint) -> Option<HandleId> {
        None
    }
}

/// 🪝 Handle-to-handle edges on nodes.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ported;

impl PortModel for Ported {
    type Endpoint = HandleId;
    const HAS_PORTS: bool = true;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint> {
        Some(handle_id)
    }
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId> {
        Some(endpoint)
    }
}
// #endregion 🔖PortModel

// #region 🔖Algorithms
pub mod algorithms {
    //! 🧮 Index-based graph algorithms: traversal, ordering, cycles, components, shortest paths.

    use std::collections::HashMap;

    // #region 🔖Adjacency
    /// 🧮 Compact adjacency built once per query batch.
    #[derive(Clone, Debug)]
    pub struct Adjacency {
        n: usize,
        out: Vec<Vec<usize>>,
        inc: Vec<Vec<usize>>,
    }

    impl Adjacency {
        pub fn node_count(&self) -> usize {
            self.n
        }
        pub fn out_neighbors(&self, i: usize) -> &[usize] {
            &self.out[i]
        }
        pub fn in_neighbors(&self, i: usize) -> &[usize] {
            &self.inc[i]
        }
    }

    /// 🧮 Builds adjacency lists from index edges; `directed` controls whether reverse edges are also recorded as out-edges.
    pub fn adjacency(node_count: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
        let mut out = vec![Vec::new(); node_count];
        let mut inc = vec![Vec::new(); node_count];
        for &(a, b) in edges {
            if a >= node_count || b >= node_count {
                continue;
            }
            out[a].push(b);
            inc[b].push(a);
            if !directed {
                out[b].push(a);
                inc[a].push(b);
            }
        }
        Adjacency { n: node_count, out, inc }
    }
    // #endregion 🔖Adjacency

    // #region 🔖IdIndex
    /// 🔤 Deterministic string-id <-> index bridge (ids sorted for reproducible ordering).
    #[derive(Clone, Debug, Default)]
    pub struct IdIndex {
        ids: Vec<String>,
        index: HashMap<String, usize>,
    }

    impl IdIndex {
        pub fn from_ids<'a>(ids: impl Iterator<Item = &'a str>) -> Self {
            let mut sorted: Vec<String> = ids.map(|s| s.to_string()).collect();
            sorted.sort();
            sorted.dedup();
            let index = sorted.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
            Self { ids: sorted, index }
        }

        pub fn from_edges<'a>(edges: impl Iterator<Item = (&'a str, &'a str)>) -> Self {
            let mut all: Vec<String> = Vec::new();
            for (a, b) in edges {
                all.push(a.to_string());
                all.push(b.to_string());
            }
            Self::from_ids(all.iter().map(|s| s.as_str()))
        }

        pub fn index_of(&self, id: &str) -> Option<usize> {
            self.index.get(id).copied()
        }

        pub fn id_of(&self, index: usize) -> Option<&str> {
            self.ids.get(index).map(|s| s.as_str())
        }

        pub fn len(&self) -> usize {
            self.ids.len()
        }

        pub fn is_empty(&self) -> bool {
            self.ids.is_empty()
        }

        pub fn edges_to_indices(&self, edges: &[(String, String)]) -> Vec<(usize, usize)> {
            edges.iter().filter_map(|(a, b)| Some((self.index_of(a)?, self.index_of(b)?))).collect()
        }
    }
    // #endregion 🔖IdIndex

    // #region 🔖Traversal
    /// 🌊 Breadth-first visitation order from the given seeds.
    pub fn bfs_order(adj: &Adjacency, seeds: &[usize]) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        for &s in seeds {
            if s < adj.n && !visited[s] {
                visited[s] = true;
                queue.push_back(s);
            }
        }
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj.out[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        order
    }

    /// 🌊 Breadth-first layers (distance bands) from the given seeds.
    pub fn bfs_layers(adj: &Adjacency, seeds: &[usize]) -> Vec<Vec<usize>> {
        let mut visited = vec![false; adj.n];
        let mut layers = Vec::new();
        let mut frontier: Vec<usize> = seeds.iter().copied().filter(|&s| s < adj.n).collect();
        for &s in &frontier {
            visited[s] = true;
        }
        while !frontier.is_empty() {
            layers.push(frontier.clone());
            let mut next = Vec::new();
            for &u in &frontier {
                for &v in &adj.out[u] {
                    if !visited[v] {
                        visited[v] = true;
                        next.push(v);
                    }
                }
            }
            frontier = next;
        }
        layers
    }

    /// 📏 Unweighted BFS distance from a single seed to every reachable node.
    pub fn bfs_distances(adj: &Adjacency, seed: usize) -> Vec<Option<u32>> {
        let mut dist = vec![None; adj.n];
        if seed >= adj.n {
            return dist;
        }
        dist[seed] = Some(0);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(seed);
        while let Some(u) = queue.pop_front() {
            let du = dist[u].unwrap();
            for &v in &adj.out[u] {
                if dist[v].is_none() {
                    dist[v] = Some(du + 1);
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    /// 🌲 Depth-first preorder from a single seed.
    pub fn dfs_preorder(adj: &Adjacency, seed: usize) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        if seed >= adj.n {
            return order;
        }
        let mut stack = vec![seed];
        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            order.push(u);
            for &v in adj.out[u].iter().rev() {
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
        order
    }

    /// 🌲 Depth-first postorder from a single seed.
    pub fn dfs_postorder(adj: &Adjacency, seed: usize) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        if seed >= adj.n {
            return order;
        }
        fn visit(u: usize, adj: &Adjacency, visited: &mut [bool], order: &mut Vec<usize>) {
            visited[u] = true;
            for &v in &adj.out[u] {
                if !visited[v] {
                    visit(v, adj, visited, order);
                }
            }
            order.push(u);
        }
        visit(seed, adj, &mut visited, &mut order);
        order
    }
    // #endregion 🔖Traversal

    // #region 🔖Ordering
    /// ⚠️ A cycle was found where a DAG was required; `cycle` lists the node indices on the cycle.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CycleError {
        pub cycle: Vec<usize>,
    }

    /// 🔢 Kahn's algorithm topological sort; index-ascending tie-break for determinism.
    pub fn topo_sort(adj: &Adjacency) -> Result<Vec<usize>, CycleError> {
        let mut in_deg = vec![0usize; adj.n];
        for list in &adj.out {
            for &v in list {
                in_deg[v] += 1;
            }
        }
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..adj.n {
            if in_deg[i] == 0 {
                heap.push(std::cmp::Reverse(i));
            }
        }
        let mut order = Vec::with_capacity(adj.n);
        while let Some(std::cmp::Reverse(u)) = heap.pop() {
            order.push(u);
            for &v in &adj.out[u] {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    heap.push(std::cmp::Reverse(v));
                }
            }
        }
        if order.len() == adj.n {
            Ok(order)
        } else {
            let remaining: Vec<usize> = (0..adj.n).filter(|&i| in_deg[i] > 0).collect();
            Err(CycleError { cycle: find_cycle_among(adj, &remaining).unwrap_or(remaining) })
        }
    }

    /// 🪜 Topological levels: each level contains nodes whose dependencies are all in earlier levels.
    pub fn topo_levels(adj: &Adjacency) -> Result<Vec<Vec<usize>>, CycleError> {
        let mut in_deg = vec![0usize; adj.n];
        for list in &adj.out {
            for &v in list {
                in_deg[v] += 1;
            }
        }
        let mut levels = Vec::new();
        let mut remaining = in_deg.clone();
        let mut placed = vec![false; adj.n];
        let mut placed_count = 0;
        loop {
            let mut frontier: Vec<usize> = (0..adj.n).filter(|&i| !placed[i] && remaining[i] == 0).collect();
            if frontier.is_empty() {
                break;
            }
            frontier.sort_unstable();
            for &u in &frontier {
                placed[u] = true;
                placed_count += 1;
            }
            for &u in &frontier {
                for &v in &adj.out[u] {
                    remaining[v] -= 1;
                }
            }
            levels.push(frontier);
        }
        if placed_count == adj.n {
            Ok(levels)
        } else {
            let unplaced: Vec<usize> = (0..adj.n).filter(|&i| !placed[i]).collect();
            Err(CycleError { cycle: find_cycle_among(adj, &unplaced).unwrap_or(unplaced) })
        }
    }

    /// 🪜 Longest-path layer index per node (DAG layering for hierarchical drawing); layer 0 = roots.
    pub fn longest_path_layers(adj: &Adjacency) -> Result<Vec<u32>, CycleError> {
        let levels = topo_levels(adj)?;
        let mut layer = vec![0u32; adj.n];
        for (li, level) in levels.iter().enumerate() {
            for &u in level {
                layer[u] = li as u32;
            }
        }
        Ok(layer)
    }
    // #endregion 🔖Ordering

    // #region 🔖Cycles
    /// 🔎 Whether `to` is reachable from `from` following out-edges.
    pub fn is_reachable(adj: &Adjacency, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }
        bfs_order(adj, &[from]).contains(&to)
    }

    /// ➕ Whether adding an edge `source -> target` would create a cycle (i.e. `target` can already reach `source`).
    pub fn would_create_cycle(adj: &Adjacency, source: usize, target: usize) -> bool {
        source == target || is_reachable(adj, target, source)
    }

    /// ➕ String-id convenience: whether adding `source -> target` to `existing` directed edges would create a cycle.
    pub fn would_create_cycle_ids(existing: &[(String, String)], source: &str, target: &str) -> bool {
        if source == target {
            return true;
        }
        let index = IdIndex::from_edges(existing.iter().map(|(a, b)| (a.as_str(), b.as_str())));
        let (Some(s), Some(t)) = (index.index_of(source), index.index_of(target)) else {
            return false;
        };
        let adj = adjacency(index.len(), &index.edges_to_indices(existing), true);
        would_create_cycle(&adj, s, t)
    }

    /// ➕ Batched acyclic filter: for each `candidates[i]`, whether adding it to `existing` (+ prior accepted candidates) keeps the graph acyclic.
    pub fn acyclic_edge_subset(existing: &[(String, String)], candidates: &[(String, String)]) -> Vec<bool> {
        let all_ids = existing.iter().chain(candidates.iter()).flat_map(|(a, b)| [a.as_str(), b.as_str()]);
        let index = IdIndex::from_ids(all_ids);
        let mut edges = index.edges_to_indices(existing);
        let mut accepted = Vec::with_capacity(candidates.len());
        for (a, b) in candidates {
            let (Some(s), Some(t)) = (index.index_of(a), index.index_of(b)) else {
                accepted.push(false);
                continue;
            };
            let adj = adjacency(index.len(), &edges, true);
            if would_create_cycle(&adj, s, t) {
                accepted.push(false);
            } else {
                edges.push((s, t));
                accepted.push(true);
            }
        }
        accepted
    }

    fn find_cycle_among(adj: &Adjacency, candidates: &[usize]) -> Option<Vec<usize>> {
        let mut color = vec![0u8; adj.n];
        let mut path = Vec::new();
        fn dfs(u: usize, adj: &Adjacency, color: &mut [u8], path: &mut Vec<usize>) -> Option<Vec<usize>> {
            color[u] = 1;
            path.push(u);
            for &v in &adj.out[u] {
                if color[v] == 1 {
                    let start = path.iter().position(|&x| x == v).unwrap();
                    return Some(path[start..].to_vec());
                }
                if color[v] == 0 {
                    if let Some(cycle) = dfs(v, adj, color, path) {
                        return Some(cycle);
                    }
                }
            }
            path.pop();
            color[u] = 2;
            None
        }
        for &start in candidates {
            if color[start] == 0 {
                if let Some(cycle) = dfs(start, adj, &mut color, &mut path) {
                    return Some(cycle);
                }
            }
        }
        None
    }

    /// 🔎 Finds one cycle in the graph, if any exist.
    pub fn find_cycle(adj: &Adjacency) -> Option<Vec<usize>> {
        let all: Vec<usize> = (0..adj.n).collect();
        find_cycle_among(adj, &all)
    }
    // #endregion 🔖Cycles

    // #region 🔖Components
    /// 🧮 Union-find (disjoint-set) with path compression and union-by-rank.
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<u8>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self { parent: (0..n).collect(), rank: vec![0; n] }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]);
            }
            self.parent[x]
        }

        pub fn union(&mut self, a: usize, b: usize) {
            let (ra, rb) = (self.find(a), self.find(b));
            if ra == rb {
                return;
            }
            match self.rank[ra].cmp(&self.rank[rb]) {
                std::cmp::Ordering::Less => self.parent[ra] = rb,
                std::cmp::Ordering::Greater => self.parent[rb] = ra,
                std::cmp::Ordering::Equal => {
                    self.parent[rb] = ra;
                    self.rank[ra] += 1;
                }
            }
        }

        pub fn same_set(&mut self, a: usize, b: usize) -> bool {
            self.find(a) == self.find(b)
        }
    }

    /// 🧩 Weak connected-component id per node (undirected reachability, works for directed adjacency too).
    pub fn connected_components(adj: &Adjacency) -> Vec<usize> {
        let mut uf = UnionFind::new(adj.n);
        for u in 0..adj.n {
            for &v in &adj.out[u] {
                uf.union(u, v);
            }
        }
        let mut root_to_component: HashMap<usize, usize> = HashMap::new();
        let mut labels = vec![0usize; adj.n];
        for u in 0..adj.n {
            let root = uf.find(u);
            let next_id = root_to_component.len();
            let id = *root_to_component.entry(root).or_insert(next_id);
            labels[u] = id;
        }
        labels
    }

    /// 🧩 Tarjan's strongly connected components; returned in reverse-topological order, nodes sorted within each.
    pub fn strongly_connected_components(adj: &Adjacency) -> Vec<Vec<usize>> {
        struct State {
            index: Vec<Option<u32>>,
            lowlink: Vec<u32>,
            on_stack: Vec<bool>,
            stack: Vec<usize>,
            counter: u32,
            out: Vec<Vec<usize>>,
        }
        fn strongconnect(u: usize, adj: &Adjacency, st: &mut State) {
            st.index[u] = Some(st.counter);
            st.lowlink[u] = st.counter;
            st.counter += 1;
            st.stack.push(u);
            st.on_stack[u] = true;
            for &v in &adj.out[u] {
                if st.index[v].is_none() {
                    strongconnect(v, adj, st);
                    st.lowlink[u] = st.lowlink[u].min(st.lowlink[v]);
                } else if st.on_stack[v] {
                    st.lowlink[u] = st.lowlink[u].min(st.index[v].unwrap());
                }
            }
            if st.lowlink[u] == st.index[u].unwrap() {
                let mut component = Vec::new();
                loop {
                    let w = st.stack.pop().unwrap();
                    st.on_stack[w] = false;
                    component.push(w);
                    if w == u {
                        break;
                    }
                }
                component.sort_unstable();
                st.out.push(component);
            }
        }
        let mut st = State { index: vec![None; adj.n], lowlink: vec![0; adj.n], on_stack: vec![false; adj.n], stack: Vec::new(), counter: 0, out: Vec::new() };
        for u in 0..adj.n {
            if st.index[u].is_none() {
                strongconnect(u, adj, &mut st);
            }
        }
        st.out
    }

    /// ⬇️ In-degree per node.
    pub fn in_degrees(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).map(|i| adj.inc[i].len()).collect()
    }

    /// ⬆️ Out-degree per node.
    pub fn out_degrees(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).map(|i| adj.out[i].len()).collect()
    }

    /// 🌱 Indices of nodes with in-degree 0 (DAG roots).
    pub fn root_indices(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).filter(|&i| adj.inc[i].is_empty()).collect()
    }
    // #endregion 🔖Components

    // #region 🔖Paths
    /// 📏 Shortest path (by hop count) between two nodes, if reachable.
    pub fn shortest_path_unweighted(adj: &Adjacency, from: usize, to: usize) -> Option<Vec<usize>> {
        if from >= adj.n || to >= adj.n {
            return None;
        }
        let mut visited = vec![false; adj.n];
        let mut parent = vec![usize::MAX; adj.n];
        visited[from] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(from);
        while let Some(u) = queue.pop_front() {
            if u == to {
                let mut path = vec![to];
                let mut cur = to;
                while cur != from {
                    cur = parent[cur];
                    path.push(cur);
                }
                path.reverse();
                return Some(path);
            }
            for &v in &adj.out[u] {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }
        None
    }

    /// 📏 Dijkstra shortest distances from `from` to every node, given non-negative edge weights parallel to adjacency out-edges.
    pub fn dijkstra(adj: &Adjacency, weights: &HashMap<(usize, usize), f64>, from: usize) -> Vec<Option<f64>> {
        let mut dist = vec![None; adj.n];
        if from >= adj.n {
            return dist;
        }
        dist[from] = Some(0.0);
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(OrderedFloat(0.0, from)));
        while let Some(std::cmp::Reverse(OrderedFloat(d, u))) = heap.pop() {
            if dist[u].map(|cur| d > cur).unwrap_or(true) {
                continue;
            }
            for &v in &adj.out[u] {
                let w = weights.get(&(u, v)).copied().unwrap_or(1.0);
                let nd = d + w;
                if dist[v].map(|cur| nd < cur).unwrap_or(true) {
                    dist[v] = Some(nd);
                    heap.push(std::cmp::Reverse(OrderedFloat(nd, v)));
                }
            }
        }
        dist
    }

    /// 📏 Dijkstra shortest path and distance between two nodes, if reachable.
    pub fn dijkstra_path(adj: &Adjacency, weights: &HashMap<(usize, usize), f64>, from: usize, to: usize) -> Option<(Vec<usize>, f64)> {
        if from >= adj.n || to >= adj.n {
            return None;
        }
        let mut dist = vec![None; adj.n];
        let mut parent = vec![usize::MAX; adj.n];
        dist[from] = Some(0.0);
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(OrderedFloat(0.0, from)));
        while let Some(std::cmp::Reverse(OrderedFloat(d, u))) = heap.pop() {
            if dist[u].map(|cur| d > cur).unwrap_or(true) {
                continue;
            }
            if u == to {
                let mut path = vec![to];
                let mut cur = to;
                while cur != from {
                    cur = parent[cur];
                    path.push(cur);
                }
                path.reverse();
                return Some((path, d));
            }
            for &v in &adj.out[u] {
                let w = weights.get(&(u, v)).copied().unwrap_or(1.0);
                let nd = d + w;
                if dist[v].map(|cur| nd < cur).unwrap_or(true) {
                    dist[v] = Some(nd);
                    parent[v] = u;
                    heap.push(std::cmp::Reverse(OrderedFloat(nd, v)));
                }
            }
        }
        None
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct OrderedFloat(f64, usize);
    impl Eq for OrderedFloat {}
    impl PartialOrd for OrderedFloat {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for OrderedFloat {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal).then(self.1.cmp(&other.1))
        }
    }

    /// 🌲 Kruskal minimum spanning tree; returns the indices (into `edges`) of the selected edges.
    pub fn minimum_spanning_tree(node_count: usize, edges: &[(usize, usize, f64)]) -> Vec<usize> {
        let mut order: Vec<usize> = (0..edges.len()).collect();
        order.sort_by(|&a, &b| edges[a].2.partial_cmp(&edges[b].2).unwrap_or(std::cmp::Ordering::Equal));
        let mut uf = UnionFind::new(node_count);
        let mut selected = Vec::new();
        for i in order {
            let (a, b, _) = edges[i];
            if a >= node_count || b >= node_count {
                continue;
            }
            if !uf.same_set(a, b) {
                uf.union(a, b);
                selected.push(i);
            }
        }
        selected
    }
    // #endregion 🔖Paths

    // #region 🔖Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn adj_from(n: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
            adjacency(n, edges, directed)
        }

        #[test]
        fn bfs_order_visits_reachable_nodes_breadth_first() {
            let adj = adj_from(5, &[(0, 1), (0, 2), (1, 3), (2, 4)], true);
            let order = bfs_order(&adj, &[0]);
            assert_eq!(order, vec![0, 1, 2, 3, 4]);
        }

        #[test]
        fn bfs_layers_group_by_distance() {
            let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3)], true);
            let layers = bfs_layers(&adj, &[0]);
            assert_eq!(layers, vec![vec![0], vec![1, 2], vec![3]]);
        }

        #[test]
        fn bfs_distances_unreachable_is_none() {
            let adj = adj_from(3, &[(0, 1)], true);
            let dist = bfs_distances(&adj, 0);
            assert_eq!(dist, vec![Some(0), Some(1), None]);
        }

        #[test]
        fn dfs_preorder_and_postorder_agree_on_leaf_first_last() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert_eq!(dfs_preorder(&adj, 0), vec![0, 1, 2]);
            assert_eq!(dfs_postorder(&adj, 0), vec![2, 1, 0]);
        }

        #[test]
        fn topo_sort_orders_dependencies_before_dependents() {
            let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3), (2, 3)], true);
            let order = topo_sort(&adj).expect("acyclic");
            let pos = |n: usize| order.iter().position(|&x| x == n).unwrap();
            assert!(pos(0) < pos(1));
            assert!(pos(1) < pos(3));
            assert!(pos(2) < pos(3));
        }

        #[test]
        fn topo_sort_detects_cycle() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            let err = topo_sort(&adj).unwrap_err();
            assert_eq!(err.cycle.len(), 3);
        }

        #[test]
        fn topo_levels_groups_independent_nodes() {
            let adj = adj_from(4, &[(0, 2), (1, 2), (2, 3)], true);
            let levels = topo_levels(&adj).expect("acyclic");
            assert_eq!(levels[0], vec![0, 1]);
            assert_eq!(levels[1], vec![2]);
            assert_eq!(levels[2], vec![3]);
        }

        #[test]
        fn longest_path_layers_assigns_root_layer_zero() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            let layers = longest_path_layers(&adj).expect("acyclic");
            assert_eq!(layers, vec![0, 1, 2]);
        }

        #[test]
        fn would_create_cycle_detects_back_edge() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert!(would_create_cycle(&adj, 2, 0));
            assert!(!would_create_cycle(&adj, 0, 2));
        }

        #[test]
        fn would_create_cycle_ids_matches_index_version() {
            let existing = vec![("a".to_string(), "b".to_string()), ("b".to_string(), "c".to_string())];
            assert!(would_create_cycle_ids(&existing, "c", "a"));
            assert!(!would_create_cycle_ids(&existing, "a", "c"));
        }

        #[test]
        fn acyclic_edge_subset_accumulates_accepted_candidates() {
            let existing = vec![("a".to_string(), "b".to_string())];
            let candidates = vec![("b".to_string(), "c".to_string()), ("c".to_string(), "a".to_string()), ("c".to_string(), "d".to_string())];
            let accepted = acyclic_edge_subset(&existing, &candidates);
            assert_eq!(accepted, vec![true, false, true]);
        }

        #[test]
        fn find_cycle_returns_none_for_dag() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert!(find_cycle(&adj).is_none());
        }

        #[test]
        fn find_cycle_returns_some_for_cyclic_graph() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            assert!(find_cycle(&adj).is_some());
        }

        #[test]
        fn connected_components_groups_weak_components() {
            let adj = adj_from(5, &[(0, 1), (1, 2), (3, 4)], true);
            let labels = connected_components(&adj);
            assert_eq!(labels[0], labels[1]);
            assert_eq!(labels[1], labels[2]);
            assert_eq!(labels[3], labels[4]);
            assert_ne!(labels[0], labels[3]);
        }

        #[test]
        fn strongly_connected_components_finds_cycle_as_one_component() {
            let adj = adj_from(4, &[(0, 1), (1, 2), (2, 0), (2, 3)], true);
            let sccs = strongly_connected_components(&adj);
            let cyclic = sccs.iter().find(|c| c.contains(&0)).unwrap();
            assert_eq!(cyclic, &vec![0, 1, 2]);
            assert!(sccs.iter().any(|c| c == &vec![3]));
        }

        #[test]
        fn degrees_and_roots_match_edge_shape() {
            let adj = adj_from(3, &[(0, 1), (0, 2)], true);
            assert_eq!(out_degrees(&adj), vec![2, 0, 0]);
            assert_eq!(in_degrees(&adj), vec![0, 1, 1]);
            assert_eq!(root_indices(&adj), vec![0]);
        }

        #[test]
        fn union_find_unions_and_queries_sets() {
            let mut uf = UnionFind::new(4);
            uf.union(0, 1);
            uf.union(2, 3);
            assert!(uf.same_set(0, 1));
            assert!(!uf.same_set(0, 2));
        }

        #[test]
        fn shortest_path_unweighted_finds_hop_path() {
            let adj = adj_from(4, &[(0, 1), (1, 3), (0, 2), (2, 3)], true);
            let path = shortest_path_unweighted(&adj, 0, 3).expect("reachable");
            assert_eq!(path.len(), 3);
            assert_eq!(path[0], 0);
            assert_eq!(*path.last().unwrap(), 3);
        }

        #[test]
        fn shortest_path_unweighted_none_when_unreachable() {
            let adj = adj_from(3, &[(0, 1)], true);
            assert!(shortest_path_unweighted(&adj, 0, 2).is_none());
        }

        #[test]
        fn dijkstra_prefers_cheaper_longer_path() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
            let mut weights = HashMap::new();
            weights.insert((0, 1), 1.0);
            weights.insert((1, 2), 1.0);
            weights.insert((0, 2), 5.0);
            let dist = dijkstra(&adj, &weights, 0);
            assert_eq!(dist[2], Some(2.0));
        }

        #[test]
        fn dijkstra_path_reconstructs_cheapest_route() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
            let mut weights = HashMap::new();
            weights.insert((0, 1), 1.0);
            weights.insert((1, 2), 1.0);
            weights.insert((0, 2), 5.0);
            let (path, dist) = dijkstra_path(&adj, &weights, 0, 2).expect("reachable");
            assert_eq!(path, vec![0, 1, 2]);
            assert_eq!(dist, 2.0);
        }

        #[test]
        fn minimum_spanning_tree_selects_cheapest_edges_without_cycles() {
            let edges = vec![(0, 1, 1.0), (1, 2, 2.0), (0, 2, 3.0)];
            let selected = minimum_spanning_tree(3, &edges);
            assert_eq!(selected.len(), 2);
            assert!(selected.contains(&0));
            assert!(selected.contains(&1));
        }

        #[test]
        fn id_index_is_deterministic_and_sorted() {
            let edges = vec![("c".to_string(), "a".to_string()), ("a".to_string(), "b".to_string())];
            let index = IdIndex::from_edges(edges.iter().map(|(a, b)| (a.as_str(), b.as_str())));
            assert_eq!(index.id_of(0), Some("a"));
            assert_eq!(index.id_of(1), Some("b"));
            assert_eq!(index.id_of(2), Some("c"));
        }
    }
    // #endregion 🔖Tests
}
// #endregion 🔖Algorithms

// #region 🔖PropertyJson
/// 🧾 Converts JSON fixture `userData` into a typed property bag.
pub fn property_bag_from_json(value: &serde_json::Value) -> PropertyBag {
    serde_json::from_value(value.clone()).unwrap_or_default()
}

/// 🧾 Serializes a property bag back to JSON for fixture export.
pub fn property_bag_to_json(bag: &PropertyBag) -> Option<serde_json::Value> {
    if bag.is_empty() {
        None
    } else {
        serde_json::to_value(bag).ok()
    }
}
// #endregion 🔖PropertyJson

// #region 🔖GraphExtension
/// 🧩 Extension hook for domain-specific graph behavior.
pub trait GraphExtension: cavas::CanvasExtension {}
// #endregion 🔖GraphExtension

// #region 🔖Kinds
use std::collections::{BTreeMap, BTreeSet};

use cavas::{CubicBez, Point, Vec2};

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

/// 🏷️ Semantic kind and property payload shared by graph elements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElementSemantics {
    pub kind: Option<String>,
    pub properties: PropertyBag,
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
    pub kind: Option<String>,
    pub label: Option<String>,
    pub properties: PropertyBag,
}

/// 🟣 Tangent handle anchored to a node at a polar angle.
#[derive(Clone, Debug, PartialEq)]
pub struct Handle {
    pub angle: f64,
    pub id: HandleId,
    pub node_id: NodeId,
    pub radius: f64,
    pub role: HandleRole,
    pub kind: Option<String>,
    pub properties: PropertyBag,
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
    PreselectChanged { edge_ids: Vec<EdgeId>, handle_ids: Vec<HandleId>, node_ids: Vec<NodeId>, removed_edge_ids: Vec<EdgeId>, removed_handle_ids: Vec<HandleId>, removed_node_ids: Vec<NodeId> },
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

/// @emoji 🎯 One graph pick target with generality rank (lower = more general).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct GraphPickTarget {
    pub domain: String,
    pub id: u64,
    pub generality: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionMode {
    DragNode { node_id: NodeId, offset: Vec2 },
    DragNodes { primary_id: NodeId, offset: Vec2 },
    DrawEdge { anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, cursor: Point, reconnecting: Option<EdgeId>, snap_target: Option<HandleId> },
    SelectionPending { start: Point, start_screen: Point },
    AreaSelect { start: Point, start_screen: Point },
    Pan { start_screen: Point, cam_x: f64, cam_y: f64, zoom: f64 },
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
            point.x >= node.center.x - hw && point.x <= node.center.x + hw && point.y >= node.center.y - hh && point.y <= node.center.y + hh
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
    inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points,
    world_boxes_overlap, WorldBox,
};

pub const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_CLICK_MAX_DISTANCE_PX;
pub const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
pub const SELECTION_MARQUEE_DRAG_THRESHOLD_PX: f64 = ui_styling::metrics::board::SELECTION_MARQUEE_MAX_DISTANCE_PX;

/// 🎯 Normalizes `default` to `replace` for merge-mode strings.
pub fn normalize_selection_mode(mode: &str) -> String {
    if mode == "default" {
        "replace".into()
    } else {
        mode.to_string()
    }
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
    let poly = vec![Point::new(b.min_x, b.min_y), Point::new(b.max_x, b.min_y), Point::new(b.max_x, b.max_y), Point::new(b.min_x, b.max_y)];
    Some((b, enclosing, poly))
}

/// 🧿 Screen-space overlay points for the shared `SelectionMarquee` overlay.
pub fn selection_screen_overlay_points(method: &str, start_screen: Point, screen_points: &[Point]) -> Option<Vec<Point>> {
    if screen_points.len() < 2 {
        return None;
    }
    let last = *screen_points.last().unwrap_or(&start_screen);
    Some(if method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] })
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
    WorldBox { min_x: center.x - hw, min_y: center.y - hh, max_x: center.x + hw, max_y: center.y + hh }
}

fn node_circle_bounds(center: Point, radius: f64) -> WorldBox {
    WorldBox { min_x: center.x - radius, min_y: center.y - radius, max_x: center.x + radius, max_y: center.y + radius }
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
    let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
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
        Self { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_handles: true, select_edges: true }
    }
}

/// ⚙️ Retained graph engine parameterized by port model and directedness.
#[derive(Clone, Debug)]
pub struct GraphEngine<P: GraphPortModel, D: Directedness> {
    pub camera: Camera,
    pub edges: BTreeMap<EdgeId, GraphEdge<P::Endpoint>>,
    pub edge_semantics: BTreeMap<EdgeId, ElementSemantics>,
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
    pub proximity_distance_world: f64,
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
            edge_semantics: BTreeMap::new(),
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
                kind: None,
                label: None,
                properties: PropertyBag::new(),
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
                kind: None,
                label: None,
                properties: PropertyBag::new(),
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
            let removed_edges: Vec<EdgeId> = self.edges.values().filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id).map(|edge| edge.id).collect();
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
                    kind: None,
                    properties: PropertyBag::new(),
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
            self.edge_semantics.remove(&id);
            self.selection.edge_ids.remove(&id);
            self.events.push(BoardEvent::EdgeRemoved { id });
        }
    }

    pub fn set_edge_semantics(&mut self, id: EdgeId, kind: Option<String>, properties: PropertyBag) {
        self.edge_semantics.insert(id, ElementSemantics { kind, properties });
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

    pub fn selection_preview_method(&self) -> &str {
        self.selection_options.method.as_str()
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
        let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
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
        self.interaction = InteractionMode::DragNodes { primary_id, offset: point - primary.center };
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
        self.hover = None;
        true
    }

    /// @emoji 🫳 Selects a draggable node and starts moving it from `point`.
    pub fn pointer_down_on_draggable_node_at(&mut self, node_id: NodeId, point: Point, shift: bool, ctrl_or_meta: bool) {
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let merge_from_modifiers = ctrl_or_meta || shift;
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
                    self.events.push(BoardEvent::NodeMoved { id: node_id, x: node.center.x(), y: node.center.y() });
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
            InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, reconnecting, .. } => {
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                let snap_target = if alt { None } else { self.nearest_wire_snap_handle(anchor_handle, anchor_is_source, fixed_target, reconnecting, point) };
                if alt {
                    self.proximity_distance_override = None;
                }
                self.interaction = InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor: point, reconnecting, snap_target };
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
            InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, reconnecting, snap_target, .. } => {
                let endpoint = snap_target.filter(|hid| self.wire_snap_still_active(*hid, point)).or_else(|| {
                    self.hit_test(point).and_then(|hit| match hit {
                        HitObject::Endpoint(ep) => Some(P::endpoint_as_u64(ep)),
                        _ => None,
                    })
                });
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
                let next = if click_only { BTreeSet::new() } else { self.resolve_area_hits(&self.area_initial_string_set(), start, &points, merge_mode.as_str()) };
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
        if let InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, snap_target, .. } = self.interaction {
            snapshot.pending_edge = self.draw_edge_preview_curve(anchor_handle, anchor_is_source, fixed_target, cursor, snap_target);
        } else if let Some(conn) = self.proximity_connection {
            snapshot.pending_edge = self.proximity_preview_curve(conn);
        }
        let _stroke_scene = encode_board_stroke_scene(&snapshot.edges, 2.0);
        let _ = _stroke_scene.path_count();
        snapshot
    }

    pub fn drain_events(&mut self) -> Vec<BoardEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn edge_curve(&self, edge_id: EdgeId) -> Option<CubicBez> {
        let edge = self.edges.get(&edge_id)?;
        let (source_position, target_position, source_node, target_node, source_cap_radius, target_cap_radius) = self.endpoint_wire_nodes(edge)?;
        Some(self.wire_bezier_between(source_position, target_position, source_node, target_node, source_cap_radius, target_cap_radius))
    }

    fn wire_bezier_between(&self, source_point: Point, target_point: Point, source_node: Option<&Node>, target_node: Option<&Node>, source_cap_radius: f64, target_cap_radius: f64) -> CubicBez {
        let chord = normalize_or_zero(target_point - source_point);
        let source_out = source_node.and_then(|node| handle_outward_at_node_rim(source_point, node.center, node.shape, node.radius, node.width, node.height)).filter(|outward| outward.hypot() > f64::EPSILON).unwrap_or(chord);
        let target_out = target_node.and_then(|node| handle_outward_at_node_rim(target_point, node.center, node.shape, node.radius, node.width, node.height)).filter(|outward| outward.hypot() > f64::EPSILON).unwrap_or(-chord);
        let source_wire = if source_node.is_some() { handle_exterior_cap_peak(source_point, source_out, source_cap_radius) } else { source_point };
        let target_wire = if target_node.is_some() { handle_exterior_cap_peak(target_point, target_out, target_cap_radius) } else { target_point };
        compute_edge_bezier_outward(source_wire, target_wire, source_out, target_out)
    }

    fn draw_edge_preview_curve(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, cursor: Point, snap_target: Option<HandleId>) -> Option<CubicBez> {
        if let Some(fixed_tgt) = fixed_target {
            let tgt_h = self.handles.get(&fixed_tgt)?;
            let tgt_node = self.nodes.get(&tgt_h.node_id)?;
            let target_point = handle_position(tgt_node, tgt_h);
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let source_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(source_point, target_point, Some(snap_node), Some(tgt_node), snap_h.radius, tgt_h.radius));
            }
            return Some(self.wire_bezier_between(cursor, target_point, None, Some(tgt_node), 0.0, tgt_h.radius));
        }
        let anchor = self.handles.get(&anchor_handle)?;
        let anchor_node = self.nodes.get(&anchor.node_id)?;
        let anchor_point = handle_position(anchor_node, anchor);
        if anchor_is_source {
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let snap_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(anchor_point, snap_point, Some(anchor_node), Some(snap_node), anchor.radius, snap_h.radius));
            }
            Some(self.wire_bezier_between(anchor_point, cursor, Some(anchor_node), None, anchor.radius, 0.0))
        } else if let Some(snap) = snap_target {
            let snap_h = self.handles.get(&snap)?;
            let snap_node = self.nodes.get(&snap_h.node_id)?;
            let snap_point = handle_position(snap_node, snap_h);
            Some(self.wire_bezier_between(snap_point, anchor_point, Some(snap_node), Some(anchor_node), snap_h.radius, anchor.radius))
        } else {
            Some(self.wire_bezier_between(cursor, anchor_point, None, Some(anchor_node), 0.0, anchor.radius))
        }
    }

    fn active_proximity_distance_world(&self) -> f64 {
        self.proximity_distance_override.unwrap_or(self.proximity_distance_world)
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

    fn nearest_wire_snap_handle(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, reconnecting: Option<EdgeId>, cursor: Point) -> Option<HandleId> {
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

    fn endpoint_wire_nodes(&self, edge: &GraphEdge<P::Endpoint>) -> Option<(Point, Point, Option<&Node>, Option<&Node>, f64, f64)> {
        if P::HAS_PORTS {
            let source_handle = self.handles.get(&P::endpoint_as_handle(edge.source)?)?;
            let target_handle = self.handles.get(&P::endpoint_as_handle(edge.target)?)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let target_node = self.nodes.get(&target_handle.node_id)?;
            let source_position = handle_position(source_node, source_handle);
            let target_position = handle_position(target_node, target_handle);
            return Some((source_position, target_position, Some(source_node), Some(target_node), source_handle.radius, target_handle.radius));
        }
        let source_node = self.nodes.get(&P::endpoint_as_u64(edge.source))?;
        let target_node = self.nodes.get(&P::endpoint_as_u64(edge.target))?;
        Some((source_node.center, target_node.center, Some(source_node), Some(target_node), 0.0, 0.0))
    }

    fn remove_handle(&mut self, id: HandleId) {
        self.handles.remove(&id);
        let removed_edges: Vec<EdgeId> = self.edges.values().filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id).map(|edge| edge.id).collect();
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
        self.events.push(BoardEvent::SelectionChanged { edge_ids: self.selection.edge_ids.iter().copied().collect(), handle_ids: self.selection.handle_ids.iter().copied().collect(), node_ids: self.selection.node_ids.iter().copied().collect() });
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
            HandleRole::Target => incoming.and_then(|e| reconnect_from_target(e, handle_id)).unwrap_or((handle_id, false, None, None)),
            HandleRole::Source => (handle_id, true, None, None),
            HandleRole::Any => incoming.and_then(|e| reconnect_from_target(e, handle_id)).unwrap_or((handle_id, true, None, None)),
        };
        self.interaction = InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, reconnecting, snap_target: None };
    }

    fn incoming_edge_for_handle(&self, handle_id: HandleId) -> Option<EdgeId> {
        self.edges.values().find(|edge| P::endpoint_as_u64(edge.target) == handle_id).map(|edge| edge.id)
    }

    fn displaced_incoming_edge(&self, target_hid: HandleId, reconnecting: Option<EdgeId>) -> Option<EdgeId> {
        self.edges.values().find(|edge| Some(edge.id) != reconnecting && P::endpoint_as_u64(edge.target) == target_hid).map(|edge| edge.id)
    }

    fn is_valid_connection(&self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>, allow_target_replace: bool) -> bool {
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
        if self.edges.values().any(|e| Some(e.id) != reconnecting && P::endpoint_as_u64(e.source) == source_hid && P::endpoint_as_u64(e.target) == target_hid) {
            return false;
        }
        if !allow_target_replace && self.edges.values().any(|e| Some(e.id) != reconnecting && P::endpoint_as_u64(e.target) == target_hid) {
            return false;
        }
        if self.enforce_acyclic {
            let src_node = source_handle.node_id;
            let tgt_node = target_handle.node_id;
            let excluding = reconnecting.or_else(|| if allow_target_replace { self.displaced_incoming_edge(target_hid, reconnecting) } else { None });
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
        self.events.push(BoardEvent::EdgeConnected { id: new_id, source: source_hid, target: target_hid });
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

    fn handle_has_incident_edge(&self, handle_id: HandleId) -> bool {
        self.edges.values().any(|edge| P::endpoint_as_u64(edge.source) == handle_id || P::endpoint_as_u64(edge.target) == handle_id)
    }

    fn update_node_drag_proximity(&mut self, dragged_ids: &[NodeId]) {
        self.proximity_connection = None;
        if !self.proximity_enabled() || !P::HAS_PORTS {
            return;
        }
        let dragged: std::collections::BTreeSet<NodeId> = dragged_ids.iter().copied().collect();
        let mut best: Option<(f64, ProximityConnection)> = None;
        for dragged_handle in self.handles.values().filter(|h| dragged.contains(&h.node_id)) {
            if self.handle_has_incident_edge(dragged_handle.id) {
                continue;
            }
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
        }
    }

    fn proximity_preview_curve(&self, conn: ProximityConnection) -> Option<CubicBez> {
        let source_handle = self.handles.get(&conn.source)?;
        let target_handle = self.handles.get(&conn.target)?;
        let source_node = self.nodes.get(&source_handle.node_id)?;
        let target_node = self.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        Some(self.wire_bezier_between(source_position, target_position, Some(source_node), Some(target_node), source_handle.radius, target_handle.radius))
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

    /// @emoji 🎯 Returns every graph entity under a world point for pick disambiguation menus.
    pub fn hit_test_pick_targets(&self, point: Point) -> Vec<GraphPickTarget> {
        let mut out = Vec::new();
        for node in self.nodes.values().rev() {
            if node_contains_point(node, point) {
                out.push(GraphPickTarget { domain: "node".into(), id: node.id, generality: 0 });
            }
        }
        for edge in self.edges.values().rev() {
            if let Some(curve) = self.edge_curve(edge.id) {
                if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
                    out.push(GraphPickTarget { domain: "edge".into(), id: edge.id, generality: 1 });
                }
            }
        }
        if P::HAS_PORTS && self.handle_pointer_picking {
            for handle in self.handles.values().rev() {
                let node = match self.nodes.get(&handle.node_id) {
                    Some(node) => node,
                    None => continue,
                };
                if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
                    if let Some(ep) = P::try_handle_endpoint(handle.id) {
                        out.push(GraphPickTarget { domain: "handle".into(), id: P::endpoint_as_u64(ep), generality: 2 });
                    }
                }
            }
        }
        out
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
    fn hit_test_pick_targets_collects_node_and_handle() {
        use cavas::Point;

        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.create_rect_node(1, 0.0, 0.0, 80.0, 56.0, true);
        engine.create_handle(10, 1, 0.0);
        let node = engine.nodes.get(&1).unwrap().clone();
        let handle = engine.handles.get(&10).unwrap().clone();
        let point = handle_position(&node, &handle);
        let targets = engine.hit_test_pick_targets(point);
        assert!(targets.iter().any(|row| row.domain == "node"));
        assert!(targets.iter().any(|row| row.domain == "handle"));
    }

    #[test]
    fn selection_union_drag_starts_inside_bounds_without_node_hit() {
        use cavas::Point;

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
        use cavas::Point;

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
        use cavas::Point;
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
        use cavas::Point;

        fn midpoint_bulge(curve: CubicBez) -> f64 {
            let p0 = curve.p0();
            let p3 = curve.p3();
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
        let target_node = engine.nodes.get(&2).expect("target node");
        let target_handle = engine.handles.get(&11).expect("target handle");
        let outward = handle_outward_at_node_rim(inp, target_node.center, target_node.shape, target_node.radius, target_node.width, target_node.height).expect("target outward");
        let peak = handle_exterior_cap_peak(inp, outward, target_handle.radius);
        assert!((preview.p3.x - peak.x).abs() < 0.01);
        assert!((preview.p3.y - peak.y).abs() < 0.01);
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
    fn node_drag_proximity_skips_wired_handles_on_dragged_node() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.enforce_acyclic = true;
        engine.proximity_distance_world = 120.0;
        engine.create_rect_node(1, 0.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(2, 220.0, 0.0, 160.0, 72.0, true);
        engine.create_rect_node(3, 440.0, 0.0, 160.0, 72.0, true);
        engine.create_handle(10, 1, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(11, 2, std::f64::consts::FRAC_PI_2);
        engine.create_handle(12, 2, 3.0 * std::f64::consts::FRAC_PI_2);
        engine.create_handle(13, 3, std::f64::consts::FRAC_PI_2);
        engine.set_handle_role(10, HandleRole::Source);
        engine.set_handle_role(11, HandleRole::Target);
        engine.set_handle_role(12, HandleRole::Source);
        engine.set_handle_role(13, HandleRole::Target);
        engine.create_edge(100, 10, 11);
        engine.create_edge(101, 12, 13);
        engine.pointer_down(220.0, 0.0, false);
        engine.pointer_move(260.0, 0.0);
        assert!(engine.render_snapshot().pending_edge.is_none(), "wired mid-node drag must not flip proximity preview between input and output");
        engine.pointer_up(260.0, 0.0);
        assert_eq!(engine.edges.len(), 2);
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
