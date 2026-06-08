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

pub fn compute_edge_bezier_points(source_point: Point, target_point: Point, source_center: Point, target_center: Point) -> CubicBez {
    let mut source_radial = normalize_or_zero(source_point - source_center);
    if source_radial == Vec2::new(0.0, 0.0) {
        source_radial = normalize_or_zero(target_point - source_point);
    }
    let mut target_radial = normalize_or_zero(target_point - target_center);
    if target_radial == Vec2::new(0.0, 0.0) {
        target_radial = normalize_or_zero(target_point - source_point);
    }
    let handle_distance = distance_between(source_point, target_point);
    let control_length = clamp_f64(handle_distance * 0.35, 24.0, 240.0);
    let p1 = source_point + source_radial * control_length;
    let p2 = target_point + target_radial * control_length;
    CubicBez::new(source_point, p1, p2, target_point)
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
        scene.stroke(&stroke, Affine::IDENTITY, Color::WHITE, None, curve);
    }
    scene
}
