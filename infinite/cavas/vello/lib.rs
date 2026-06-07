//! 🖼️ Application-neutral tile-based infinite canvas (Vello/WebGPU); extend via `CanvasExtension`.
#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]

pub use vello_svg::usvg;
pub use vello_svg::vello;

// #region 🏷️BoardIconAssets

pub mod board_icon_assets {
    //! @emoji 📎 Static bytes for board icon rendering; `include_bytes!` paths are relative to this `lib.rs` file.

    pub static NOTO_COLOR_EMOJI_SUBSET_TTF: &[u8] = include_bytes!("assets/NotoColorEmoji-subset.ttf");

    pub static MAP_LABEL_SANS_TTF: &[u8] = include_bytes!("assets/MapLabelSans.ttf");
}

// #endregion 🏷️BoardIconAssets

pub mod vcompute {
    use crate::vello::kurbo::{Affine, CubicBez, ParamCurve, Point, Stroke, Vec2};
    use crate::vello::peniko::Color;
    use crate::vello::Scene;

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

    pub fn encode_board_stroke_scene(curves: &[CubicBez], stroke_width: f64) -> Scene {
        let mut scene = Scene::new();
        let stroke = Stroke::new(stroke_width);
        for curve in curves {
            scene.stroke(&stroke, Affine::IDENTITY, Color::WHITE, None, curve);
        }
        scene
    }
}

pub mod geom_sel {
    use crate::vello::kurbo::{CubicBez, ParamCurve, Point};

    #[derive(Clone, Copy, Debug)]
    pub struct WorldBox {
        pub min_x: f64,
        pub min_y: f64,
        pub max_x: f64,
        pub max_y: f64,
    }

    pub fn inflate_world_box(b: WorldBox, pad: f64) -> WorldBox {
        WorldBox { min_x: b.min_x - pad, min_y: b.min_y - pad, max_x: b.max_x + pad, max_y: b.max_y + pad }
    }

    pub fn world_boxes_overlap(a: WorldBox, b: WorldBox) -> bool {
        a.min_x <= b.max_x && a.max_x >= b.min_x && a.min_y <= b.max_y && a.max_y >= b.min_y
    }

    pub fn world_box_contains_point(b: WorldBox, p: Point) -> bool {
        p.x >= b.min_x && p.x <= b.max_x && p.y >= b.min_y && p.y <= b.max_y
    }

    pub fn world_box_contains_box(outer: WorldBox, inner: WorldBox) -> bool {
        inner.min_x >= outer.min_x && inner.max_x <= outer.max_x && inner.min_y >= outer.min_y && inner.max_y <= outer.max_y
    }

    fn world_box_corners(b: WorldBox) -> [Point; 4] {
        [Point::new(b.min_x, b.min_y), Point::new(b.max_x, b.min_y), Point::new(b.max_x, b.max_y), Point::new(b.min_x, b.max_y)]
    }

    pub fn world_box_from_points(points: &[Point]) -> Option<WorldBox> {
        if points.is_empty() {
            return None;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for p in points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        Some(WorldBox { min_x, min_y, max_x, max_y })
    }

    pub fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
        if polygon.len() < 3 {
            return false;
        }
        let mut inside = false;
        let mut j = polygon.len() - 1;
        for i in 0..polygon.len() {
            let a = polygon[i];
            let b = polygon[j];
            let crosses = (a.y > point.y) != (b.y > point.y);
            if crosses && point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
        const EPS: f64 = 1e-9;
        point.x >= start.x.min(end.x) - EPS
            && point.x <= start.x.max(end.x) + EPS
            && point.y >= start.y.min(end.y) - EPS
            && point.y <= start.y.max(end.y) + EPS
            && ((end.x - start.x) * (point.y - start.y) - (end.y - start.y) * (point.x - start.x)).abs() <= EPS
    }

    fn orientation(a: Point, b: Point, c: Point) -> i8 {
        let v = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);
        if v > 1e-9 {
            1
        } else if v < -1e-9 {
            -1
        } else {
            0
        }
    }

    fn segments_intersect(a0: Point, a1: Point, b0: Point, b1: Point) -> bool {
        let o1 = orientation(a0, a1, b0);
        let o2 = orientation(a0, a1, b1);
        let o3 = orientation(b0, b1, a0);
        let o4 = orientation(b0, b1, a1);
        if o1 != o2 && o3 != o4 {
            return true;
        }
        point_on_segment(b0, a0, a1) || point_on_segment(b1, a0, a1) || point_on_segment(a0, b0, b1) || point_on_segment(a1, b0, b1)
    }

    fn world_box_edges(box_: WorldBox) -> [(Point, Point); 4] {
        let [a, b, c, d] = world_box_corners(box_);
        [(a, b), (b, c), (c, d), (d, a)]
    }

    pub fn segment_intersects_world_box(start: Point, end: Point, box_: WorldBox) -> bool {
        if world_box_contains_point(box_, start) || world_box_contains_point(box_, end) {
            return true;
        }
        world_box_edges(box_).iter().any(|&(a, b)| segments_intersect(start, end, a, b))
    }

    fn polygon_segments(polygon: &[Point]) -> Vec<(Point, Point)> {
        if polygon.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(polygon.len());
        for i in 0..polygon.len() {
            out.push((polygon[i], polygon[(i + 1) % polygon.len()]));
        }
        out
    }

    pub fn polygon_contains_world_box(polygon: &[Point], box_: WorldBox) -> bool {
        world_box_corners(box_).iter().all(|&p| point_in_polygon(p, polygon))
    }

    pub fn polygon_intersects_world_box(polygon: &[Point], box_: WorldBox) -> bool {
        if world_box_corners(box_).iter().any(|&p| point_in_polygon(p, polygon)) {
            return true;
        }
        if polygon.iter().any(|&p| world_box_contains_point(box_, p)) {
            return true;
        }
        polygon_segments(polygon).iter().any(|&(s, e)| segment_intersects_world_box(s, e, box_))
    }

    pub fn segment_intersects_polygon(start: Point, end: Point, polygon: &[Point]) -> bool {
        if point_in_polygon(start, polygon) || point_in_polygon(end, polygon) {
            return true;
        }
        polygon_segments(polygon).iter().any(|&(a, b)| segments_intersect(start, end, a, b))
    }

    pub fn cubic_bezier_axis_bounds(c: CubicBez) -> WorldBox {
        let xs = [c.p0.x, c.p1.x, c.p2.x, c.p3.x];
        let ys = [c.p0.y, c.p1.y, c.p2.y, c.p3.y];
        WorldBox {
            min_x: xs.iter().copied().fold(f64::INFINITY, f64::min),
            max_x: xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            min_y: ys.iter().copied().fold(f64::INFINITY, f64::min),
            max_y: ys.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        }
    }

    pub fn cubic_bezier_point(c: CubicBez, t: f64) -> Point {
        c.eval(t.clamp(0.0, 1.0))
    }
}

pub mod scene_json {
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

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HandleDescJson {
        pub id: String,
        pub node_id: String,
        pub angle: f64,
        #[serde(default)]
        pub radius: Option<f64>,
        #[serde(default)]
        pub selected: Option<bool>,
        #[serde(default)]
        pub style: Option<String>,
        #[serde(default)]
        pub handle_kind: Option<String>,
        /// CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding catalog color for this handle.
        #[serde(default)]
        pub color: Option<String>,
        /// @emoji 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
        #[serde(default)]
        pub icon_kind: Option<String>,
        #[serde(default)]
        pub user_data: Option<serde_json::Value>,
        #[serde(default)]
        pub visible: Option<bool>,
        #[serde(default)]
        pub scale: Option<f64>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EdgeDescJson {
        pub id: String,
        pub source: String,
        pub target: String,
        /// @emoji 🧩 Semantic edge-kind id for compatibility at `edge` specificity.
        #[serde(default)]
        pub edge_kind: Option<String>,
        /// @emoji 🔺 Per-instance source tip id from the edge tip registry (`none` disables).
        #[serde(default)]
        pub source_tip: Option<String>,
        /// @emoji 🔺 Per-instance target tip id from the edge tip registry (`none` disables).
        #[serde(default)]
        pub target_tip: Option<String>,
        #[serde(default)]
        pub selected: Option<bool>,
        #[serde(default)]
        pub style: Option<String>,
        #[serde(default)]
        pub user_data: Option<serde_json::Value>,
        #[serde(default)]
        pub visible: Option<bool>,
    }

    /// @emoji 🧵 Transient cubic link from a handle to another handle or a free world point (descriptor + link gesture).
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WireDescJson {
        pub id: String,
        pub source: String,
        /// @emoji 🧩 Semantic wire-kind id (defaults from catalog when omitted in fixtures).
        #[serde(default)]
        pub wire_kind: Option<String>,
        #[serde(default)]
        pub target: Option<String>,
        #[serde(default)]
        pub end_x: Option<f64>,
        #[serde(default)]
        pub end_y: Option<f64>,
        #[serde(default)]
        pub selected: Option<bool>,
        #[serde(default)]
        pub style: Option<String>,
        #[serde(default)]
        pub user_data: Option<serde_json::Value>,
        #[serde(default)]
        pub visible: Option<bool>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SceneDescriptorJson {
        pub nodes: Vec<NodeDescJson>,
        pub handles: Vec<HandleDescJson>,
        pub edges: Vec<EdgeDescJson>,
        #[serde(default)]
        pub wires: Vec<WireDescJson>,
        /// @emoji 💠 JS‑authored ids to paint with secondary “left selection” chrome (not in current `selected` flags).
        #[serde(default)]
        pub selection_exit_highlight_ids: Vec<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FixtureV1Json {
        pub schema: String,
        pub camera: CameraJson,
        pub nodes: Vec<serde_json::Value>,
        pub edges: Vec<serde_json::Value>,
        #[serde(default)]
        pub meta: Option<serde_json::Value>,
    }

    /// 🧾 Reads fixture edge endpoint handle ids from `source` and `target` string fields only.
    pub fn fixture_edge_handle_ids_from_object(eo: &serde_json::Map<String, serde_json::Value>) -> Option<(&str, &str)> {
        let source = eo.get("source").and_then(|v| v.as_str())?;
        let target = eo.get("target").and_then(|v| v.as_str())?;
        Some((source, target))
    }
}

pub use scene_json::{fixture_edge_handle_ids_from_object, CameraJson, EdgeDescJson, FixtureV1Json, HandleDescJson, NodeDescJson, SceneDescriptorJson, WireDescJson};

fn board_json_hidden_flag(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    obj.get("hidden").and_then(|v| v.as_bool())
}

pub fn board_json_visible_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    match board_json_hidden_flag(obj) {
        Some(hidden) => Some(!hidden),
        None => obj.get("visible").and_then(|v| v.as_bool()),
    }
}

pub fn board_json_visible_or_true(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
    board_json_visible_option(obj).unwrap_or(true)
}

pub use vcompute::{
    compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, encode_board_stroke_scene, handle_position_on_circle,
    handle_position_on_rectangle, normalize_or_zero, ray_from_origin_to_axis_aligned_rectangle_edge,
};

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn normalize_board_descriptor_hidden_to_visible(value: &mut serde_json::Value) {
    let Some(root) = value.as_object_mut() else {
        return;
    };
    for key in ["nodes", "handles", "edges", "wires"] {
        let Some(rows) = root.get_mut(key).and_then(|v| v.as_array_mut()) else {
            continue;
        };
        for row in rows {
            let Some(obj) = row.as_object_mut() else {
                continue;
            };
            if let Some(visible) = board_json_visible_option(obj) {
                obj.insert("visible".into(), serde_json::json!(visible));
            }
        }
    }
}

pub mod svg_icon_vello09 {
    use std::sync::{Arc, OnceLock};

    use crate::usvg;
    use crate::vello::kurbo::{Affine, BezPath, Point, Stroke};
    use crate::vello::peniko::{Color, Fill};
    use crate::vello::Scene;

    // #region 🔖BoardIconUsvgParseOptions

    static BOARD_ICON_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

    /// @emoji 🔤 Shared `usvg` parse options with bundled Noto Color Emoji so `<text>` in Typst `emoji:` SVG matches the Typst font book; avoids system fallback glyphs.
    pub fn usvg_options_board_icons() -> &'static usvg::Options<'static> {
        BOARD_ICON_USVG_OPTIONS.get_or_init(|| {
            let mut db = fontdb::Database::new();
            db.load_font_data(super::board_icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF.to_vec());
            let mut o = usvg::Options::default();
            o.fontdb = Arc::new(db);
            o.font_family = "Noto Color Emoji".into();
            o
        })
    }

    // #endregion 🔖BoardIconUsvgParseOptions

    fn to_affine(ts: &usvg::Transform) -> Affine {
        let usvg::Transform { sx, kx, ky, sy, tx, ty } = *ts;
        Affine::new([sx, ky, kx, sy, tx, ty].map(f64::from))
    }

    fn to_bez_path(path: &usvg::Path) -> BezPath {
        let mut local_path = BezPath::new();
        let mut just_closed = false;
        let mut most_recent_initial = (0_f64, 0_f64);
        for elt in path.data().segments() {
            match elt {
                usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    most_recent_initial = (p.x.into(), p.y.into());
                    local_path.move_to(most_recent_initial);
                }
                usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.line_to(Point::new(p.x as f64, p.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::QuadTo(p1, p2) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.quad_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p3) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.curve_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64), Point::new(p3.x as f64, p3.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::Close => {
                    just_closed = true;
                    local_path.close_path();
                }
            }
        }
        local_path
    }

    fn map_solid_icon_paint(paint: &usvg::Paint, opacity: usvg::Opacity, fg: Color, bg: Color) -> Option<Color> {
        let usvg::Paint::Color(c) = paint else {
            return None;
        };
        let a = opacity.get();
        if c.red < 22 && c.green < 22 && c.blue < 22 {
            return Some(fg.multiply_alpha(a));
        }
        if c.red > 233 && c.green > 233 && c.blue > 233 {
            return Some(bg.multiply_alpha(a));
        }
        Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
    }

    fn render_group(scene: &mut Scene, group: &usvg::Group, fg: Color, bg: Color) {
        for node in group.children() {
            match node {
                usvg::Node::Group(g) => render_group(scene, g, fg, bg),
                usvg::Node::Path(path) => {
                    if !path.is_visible() {
                        continue;
                    }
                    let transform = to_affine(&path.abs_transform());
                    let local_path = to_bez_path(path);
                    if let Some(fill) = path.fill() {
                        if let Some(color) = map_solid_icon_paint(fill.paint(), fill.opacity(), fg, bg) {
                            scene.fill(
                                match fill.rule() {
                                    usvg::FillRule::NonZero => Fill::NonZero,
                                    usvg::FillRule::EvenOdd => Fill::EvenOdd,
                                },
                                transform,
                                color,
                                None,
                                &local_path,
                            );
                        }
                    }
                    if let Some(stroke) = path.stroke() {
                        if let Some(color) = map_solid_icon_paint(stroke.paint(), stroke.opacity(), fg, bg) {
                            let conv = Stroke::new(f64::from(stroke.width().get()));
                            scene.stroke(&conv, transform, color, None, &local_path);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn icon_rect_xywh(r: usvg::Rect) -> Option<(f64, f64, f64, f64)> {
        let w = f64::from(r.width());
        let h = f64::from(r.height());
        if !(w > 1e-6 && h > 1e-6 && w.is_finite() && h.is_finite()) {
            return None;
        }
        Some((f64::from(r.x()), f64::from(r.y()), w, h))
    }

    fn icon_rect_nonzero(r: usvg::tiny_skia_path::NonZeroRect) -> (f64, f64, f64, f64) {
        (f64::from(r.x()), f64::from(r.y()), f64::from(r.width()), f64::from(r.height()))
    }

    fn icon_union_xywh(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
        let ax1 = a.0 + a.2;
        let ay1 = a.1 + a.3;
        let bx1 = b.0 + b.2;
        let by1 = b.1 + b.3;
        let x0 = a.0.min(b.0);
        let y0 = a.1.min(b.1);
        let x1 = ax1.max(bx1);
        let y1 = ay1.max(by1);
        (x0, y0, x1 - x0, y1 - y0)
    }

    fn icon_union_rects_into(acc: &mut Option<(f64, f64, f64, f64)>, r: usvg::Rect) {
        if let Some(xy) = icon_rect_xywh(r) {
            *acc = Some(match acc.take() {
                None => xy,
                Some(a) => icon_union_xywh(a, xy),
            });
        }
    }

    fn icon_visit_node_bounds(node: &usvg::Node, acc: &mut Option<(f64, f64, f64, f64)>) {
        match node {
            usvg::Node::Group(g) => {
                for c in g.children() {
                    icon_visit_node_bounds(c, acc);
                }
            }
            usvg::Node::Path(p) => {
                if !p.is_visible() {
                    return;
                }
                icon_union_rects_into(acc, p.abs_bounding_box());
                icon_union_rects_into(acc, p.abs_stroke_bounding_box());
            }
            usvg::Node::Image(img) => {
                if !img.is_visible() {
                    return;
                }
                icon_union_rects_into(acc, img.abs_bounding_box());
            }
            usvg::Node::Text(t) => {
                icon_union_rects_into(acc, t.abs_bounding_box());
                icon_union_rects_into(acc, t.abs_stroke_bounding_box());
            }
        }
    }

    /// @emoji 📐 Union of visible paint bounds (paths, raster images, text) in absolute SVG space for uniform scale-and-center fits.
    pub fn svg_icon_content_bounds(tree: &usvg::Tree) -> (f64, f64, f64, f64) {
        let mut acc = None::<(f64, f64, f64, f64)>;
        for c in tree.root().children() {
            icon_visit_node_bounds(c, &mut acc);
        }
        if let Some(u) = acc {
            let (_, _, bw, bh) = u;
            if bw > 1e-6 && bh > 1e-6 {
                return u;
            }
        }
        let root = tree.root();
        let mut u = icon_rect_nonzero(root.abs_layer_bounding_box());
        if let Some(r) = icon_rect_xywh(root.abs_stroke_bounding_box()) {
            u = icon_union_xywh(u, r);
        }
        if let Some(r) = icon_rect_xywh(root.abs_bounding_box()) {
            u = icon_union_xywh(u, r);
        }
        let (_, _, bw, bh) = u;
        if bw > 1e-6 && bh > 1e-6 {
            return u;
        }
        let w = f64::from(tree.size().width());
        let h = f64::from(tree.size().height());
        (0.0, 0.0, w.max(1.0), h.max(1.0))
    }

    pub fn render_svg_tree_themed(scene: &mut Scene, tree: &usvg::Tree, fg: Color, bg: Color) {
        render_group(scene, tree.root(), fg, bg);
    }

    #[allow(dead_code)]
    pub fn append_svg_str_themed(scene: &mut Scene, svg: &str, fg: Color, bg: Color) -> Result<(), String> {
        let tree = usvg::Tree::from_str(svg, usvg_options_board_icons()).map_err(|e| e.to_string())?;
        render_svg_tree_themed(scene, &tree, fg, bg);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn append_svg_str(scene: &mut Scene, svg: &str) -> Result<(), String> {
        append_svg_str_themed(scene, svg, Color::BLACK, Color::WHITE)
    }
}

// #region 🔖Text
pub mod text {
    use std::sync::{Arc, OnceLock};

    use crate::svg_icon_vello09::render_svg_tree_themed;
    use crate::usvg;
    use crate::vello::kurbo::Point;
    use crate::vello::peniko::Color;
    use crate::vello::Scene;

    static MAP_LABEL_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

    /// @emoji 🔤 `usvg` options with bundled MapLabelSans for map place-name labels.
    pub fn usvg_options_map_labels() -> &'static usvg::Options<'static> {
        MAP_LABEL_USVG_OPTIONS.get_or_init(|| {
            let mut db = fontdb::Database::new();
            db.load_font_data(super::board_icon_assets::MAP_LABEL_SANS_TTF.to_vec());
            let mut o = usvg::Options::default();
            o.fontdb = Arc::new(db);
            o.font_family = "MapLabelSans".into();
            o
        })
    }

    fn escape_xml_attr(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn color_to_svg(c: Color) -> String {
        let rgba = c.to_rgba8();
        if rgba.a == 255 {
            format!("#{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b)
        } else {
            let a = f64::from(rgba.a) / 255.0;
            format!("rgba({},{},{},{a})", rgba.r, rgba.g, rgba.b)
        }
    }

    /// @emoji 🏷️ Renders a single map label via SVG text at `origin` (screen px, baseline).
    pub fn append_label(scene: &mut Scene, label: &str, origin: Point, px: f64, fill: Color, halo: Color) {
        let trimmed = label.trim();
        if trimmed.is_empty() || px < 4.0 {
            return;
        }
        let pad = px * 0.35;
        let w = (trimmed.len() as f64 * px * 0.62 + pad * 2.0).clamp(32.0, 2048.0);
        let h = (px * 1.6 + pad * 2.0).clamp(16.0, 256.0);
        let text_y = pad + px;
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="MapLabelSans" fill="{fill}" stroke="{halo}" stroke-width="{stroke}" paint-order="stroke">{text}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            fill = color_to_svg(fill),
            halo = color_to_svg(halo),
            stroke = (px * 0.12).max(1.0),
            text = escape_xml_attr(trimmed),
        );
        let Ok(tree) = usvg::Tree::from_str(&svg, usvg_options_map_labels()) else {
            return;
        };
        let (bx, by, bw, bh) = crate::svg_icon_vello09::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return;
        }
        let scale = (px * 0.9 / bh).min(2.5);
        let mut label_scene = Scene::new();
        render_svg_tree_themed(&mut label_scene, &tree, fill, halo);
        let aff = vello::kurbo::Affine::translate((origin.x - bx * scale, origin.y - by * scale - px * 0.85))
            * vello::kurbo::Affine::scale(scale);
        scene.append(&label_scene, Some(aff));
    }
}
// #endregion 🔖Text

// #region 🔖Camera
pub mod camera {
    use crate::vello::kurbo::{Affine, Point};

    pub const CANVAS_CAMERA_ZOOM_MIN: f64 = 0.05;
    pub const CANVAS_CAMERA_ZOOM_MAX: f64 = 32.0;

    #[derive(Clone, Debug)]
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

    #[derive(Clone, Copy, Debug)]
    pub struct Viewport {
        pub width: u32,
        pub height: u32,
        pub dpr: f64,
    }

    impl Default for Viewport {
        fn default() -> Self {
            Self { width: 1, height: 1, dpr: 1.0 }
        }
    }

    impl Viewport {
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            self.width = width.max(1);
            self.height = height.max(1);
            self.dpr = dpr.max(1.0);
        }

        pub fn physical_size(&self) -> (u32, u32) {
            let pw = ((self.width as f64 * self.dpr).round() as u32).max(1);
            let ph = ((self.height as f64 * self.dpr).round() as u32).max(1);
            (pw, ph)
        }
    }

    pub fn clamp_zoom(zoom: f64) -> f64 {
        zoom.clamp(CANVAS_CAMERA_ZOOM_MIN, CANVAS_CAMERA_ZOOM_MAX)
    }

    pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new(
            (p.x - camera.x) * camera.zoom + viewport.width as f64 / 2.0,
            (p.y - camera.y) * camera.zoom + viewport.height as f64 / 2.0,
        )
    }

    pub fn screen_to_world(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new(
            (p.x - viewport.width as f64 / 2.0) / camera.zoom + camera.x,
            (p.y - viewport.height as f64 / 2.0) / camera.zoom + camera.y,
        )
    }

    pub fn camera_content_affine(camera: &Camera, viewport: &Viewport) -> Affine {
        let z = camera.zoom;
        Affine::new([
            z,
            0.0,
            0.0,
            z,
            viewport.width as f64 * 0.5 - camera.x * z,
            viewport.height as f64 * 0.5 - camera.y * z,
        ])
    }

    pub fn wheel_screen(camera: &mut Camera, viewport: &Viewport, sx: f64, sy: f64, delta_y: f64) {
        let zoom_factor = if delta_y < 0.0 { 1.1 } else { 0.9 };
        let next_zoom = clamp_zoom(camera.zoom * zoom_factor);
        let screen = Point::new(sx, sy);
        let world_before = screen_to_world(camera, viewport, screen);
        camera.x = world_before.x - (sx - viewport.width as f64 / 2.0) / next_zoom;
        camera.y = world_before.y - (sy - viewport.height as f64 / 2.0) / next_zoom;
        camera.zoom = next_zoom;
    }
}
// #endregion 🔖Camera

// #region 🔖Lod
pub mod lod {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Lod {
        pub id: &'static str,
        pub name: &'static str,
        pub description: &'static str,
        pub max_zoom: f64,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct LodScale {
        pub lods: &'static [Lod],
    }

    impl LodScale {
        pub fn resolve_index(self, zoom: f64) -> usize {
            for (i, lod) in self.lods.iter().enumerate() {
                if zoom < lod.max_zoom {
                    return i;
                }
            }
            self.lods.len().saturating_sub(1)
        }

        pub fn resolve(self, zoom: f64) -> &'static Lod {
            &self.lods[self.resolve_index(zoom)]
        }

        pub fn index_of(self, id: &str) -> Option<usize> {
            self.lods.iter().position(|lod| lod.id == id)
        }
    }
}
// #endregion 🔖Lod

// #region 🔖Raster
pub mod raster {
    use crate::vello::kurbo::Affine;
    use crate::vello::peniko::{ImageBrush, ImageData};
    use crate::vello::Scene;
    use std::collections::HashMap;
    use std::sync::Arc;

    pub fn draw_image(scene: &mut Scene, image: &ImageData, transform: Affine) {
        scene.draw_image(&ImageBrush::new(image.clone()), transform);
    }

    pub fn draw_image_arc(scene: &mut Scene, image: &Arc<ImageData>, transform: Affine) {
        scene.draw_image(&ImageBrush::new((**image).clone()), transform);
    }

    #[derive(Clone, Default)]
    pub struct RasterImageCache {
        entries: HashMap<String, Arc<ImageData>>,
    }

    impl RasterImageCache {
        pub fn get(&self, key: &str) -> Option<Arc<ImageData>> {
            self.entries.get(key).cloned()
        }

        pub fn insert(&mut self, key: String, image: ImageData) -> Arc<ImageData> {
            let arc = Arc::new(image);
            self.entries.insert(key, arc.clone());
            arc
        }
    }
}
// #endregion 🔖Raster

// #region 🔖CanvasContent
pub mod canvas_content {
    use crate::vello::peniko::Color;
    use crate::vello::Scene;

    pub trait CanvasContent {
        fn build_scene(&self) -> Scene;
        fn clear_color(&self) -> Color;
    }
}
// #endregion 🔖CanvasContent

// #region 🔖GpuSession
#[cfg(target_arch = "wasm32")]
pub mod gpu_session {
    use crate::vello::peniko::Color;
    use crate::vello::Scene;
    use crate::vello::util::{RenderContext, RenderSurface};
    use wasm_bindgen::prelude::JsValue;
    use web_sys::HtmlCanvasElement;

    pub struct CanvasGpuSession {
        #[allow(dead_code, reason = "Retains canvas for the WebGPU surface lifetime.")]
        canvas: Option<HtmlCanvasElement>,
        render_ctx: Option<RenderContext>,
        renderer: Option<crate::vello::Renderer>,
        surface: Option<RenderSurface<'static>>,
    }

    impl Default for CanvasGpuSession {
        fn default() -> Self {
            Self { canvas: None, render_ctx: None, renderer: None, surface: None }
        }
    }

    impl CanvasGpuSession {
        pub fn gpu_ready(&self) -> bool {
            self.surface.is_some()
        }

        pub async fn create_canvas_surface(canvas: HtmlCanvasElement, pw: u32, ph: u32) -> Result<(RenderContext, crate::vello::Renderer, RenderSurface<'static>), String> {
            let mut render_ctx = RenderContext::new();
            let surface = render_ctx
                .create_surface(crate::vello::wgpu::SurfaceTarget::Canvas(canvas), pw, ph, crate::vello::wgpu::PresentMode::AutoVsync)
                .await
                .map_err(|err| format!("{err:?}"))?;
            let dev = &render_ctx.devices[surface.dev_id].device;
            let renderer = crate::vello::Renderer::new(
                dev,
                crate::vello::RendererOptions {
                    use_cpu: false,
                    antialiasing_support: crate::vello::AaSupport::area_only(),
                    num_init_threads: std::num::NonZeroUsize::new(1),
                    pipeline_cache: None,
                },
            )
            .map_err(|err| format!("{err:?}"))?;
            Ok((render_ctx, renderer, surface))
        }

        pub fn finish_attach(&mut self, canvas: HtmlCanvasElement, render_ctx: RenderContext, renderer: crate::vello::Renderer, surface: RenderSurface<'static>) {
            self.canvas = Some(canvas);
            self.render_ctx = Some(render_ctx);
            self.renderer = Some(renderer);
            self.surface = Some(surface);
        }

        pub fn resize_surface(&mut self, pw: u32, ph: u32) {
            if let (Some(surface), Some(render_ctx)) = (self.surface.as_mut(), self.render_ctx.as_mut()) {
                let cur_w = surface.config.width;
                let cur_h = surface.config.height;
                if cur_w != pw || cur_h != ph {
                    render_ctx.resize_surface(surface, pw, ph);
                }
            }
        }

        pub fn render_frame(&mut self, scene: &Scene, clear_color: Color) -> Result<(), JsValue> {
            for _attempt in 0..3u8 {
                let (surface, renderer, render_ctx) = match (self.surface.as_mut(), self.renderer.as_mut(), self.render_ctx.as_mut()) {
                    (Some(s), Some(r), Some(rc)) => (s, r, rc),
                    _ => return Ok(()),
                };
                let dh = &render_ctx.devices[surface.dev_id];
                let pw = surface.config.width.max(1);
                let ph = surface.config.height.max(1);
                let params = crate::vello::RenderParams {
                    base_color: clear_color,
                    width: pw,
                    height: ph,
                    antialiasing_method: crate::vello::AaConfig::Area,
                };
                renderer
                    .render_to_texture(&dh.device, &dh.queue, scene, &surface.target_view, &params)
                    .map_err(|err| JsValue::from_str(&format!("{err:?}")))?;

                let surface_tex = match surface.surface.get_current_texture() {
                    Ok(t) => t,
                    Err(crate::vello::wgpu::SurfaceError::Outdated) => {
                        surface.surface.configure(&dh.device, &surface.config);
                        continue;
                    }
                    Err(crate::vello::wgpu::SurfaceError::Timeout) | Err(crate::vello::wgpu::SurfaceError::Other) => return Ok(()),
                    Err(crate::vello::wgpu::SurfaceError::Lost) | Err(crate::vello::wgpu::SurfaceError::OutOfMemory) => {
                        return Err(JsValue::from_str("surface lost or validation error"));
                    }
                };
                let view = surface_tex.texture.create_view(&crate::vello::wgpu::TextureViewDescriptor::default());
                let mut encoder = dh.device.create_command_encoder(&crate::vello::wgpu::CommandEncoderDescriptor { label: Some("infinite_cavas_surface_blit") });
                surface.blitter.copy(&dh.device, &mut encoder, &surface.target_view, &view);
                dh.queue.submit(std::iter::once(encoder.finish()));
                surface_tex.present();
                let _ = dh.device.poll(crate::vello::wgpu::PollType::Poll).ok();
                return Ok(());
            }
            Ok(())
        }
    }
}
// #endregion 🔖GpuSession

// #region 🔖CanvasExtension
/// 🧩 Extension hook for domain-specific canvas behavior (hit-test, paint, kinds).
pub trait CanvasExtension: Send + Sync {
    fn extension_id(&self) -> &str;
}

/// ⚙️ Generic infinite-canvas engine shell; domain logic lives in `E`.
pub struct CanvasEngine<E: CanvasExtension> {
    pub extension: E,
}

impl<E: CanvasExtension> CanvasEngine<E> {
    pub fn new(extension: E) -> Self {
        Self { extension }
    }
}
// #endregion 🔖CanvasExtension

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::camera::{screen_to_world, world_to_screen, Camera, Viewport};
    use super::lod::{Lod, LodScale};
    use crate::vello::kurbo::Point;

    #[test]
    fn camera_round_trip() {
        let camera = Camera { x: 10.0, y: -5.0, zoom: 2.0 };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let world = Point::new(12.0, 3.0);
        let screen = world_to_screen(&camera, &viewport, world);
        let back = screen_to_world(&camera, &viewport, screen);
        assert!((back.x - world.x).abs() < 1e-9);
        assert!((back.y - world.y).abs() < 1e-9);
    }

    #[test]
    fn lod_scale_resolve() {
        const LODS: &[Lod] = &[
            Lod { id: "minimap", name: "Minimap", description: "min", max_zoom: 0.15 },
            Lod { id: "overview", name: "Overview", description: "ov", max_zoom: 0.35 },
            Lod { id: "micro", name: "Micro", description: "mi", max_zoom: f64::INFINITY },
        ];
        let scale = LodScale { lods: LODS };
        assert_eq!(scale.resolve(0.1).id, "minimap");
        assert_eq!(scale.resolve(0.2).id, "overview");
        assert_eq!(scale.resolve(3.0).id, "micro");
        assert_eq!(scale.index_of("overview"), Some(1));
    }
}
// #endregion 🔖Tests
