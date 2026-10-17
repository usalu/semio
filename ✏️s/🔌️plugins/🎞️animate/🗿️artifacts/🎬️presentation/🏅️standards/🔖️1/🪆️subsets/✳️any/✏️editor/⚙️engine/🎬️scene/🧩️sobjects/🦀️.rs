//! 🧩️ Sobject trait, VSobject paths, groups, transforms, and layout helpers.

use crate::editor::animate::engine::rate::updater::Updater;
use crate::editor::animate::engine::text::color::Color;
use geometry::{append_shape_to_path, bounding_box, polygon_centroid, Affine, BezPath, PathEl, Point, Vec2};
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};

fn next_id() -> u64 {
    u64::from_str_radix(&framework_hash::hash_bytes(concat!(file!(), line!()).as_bytes())[..8], 16).unwrap_or(1)
}

/// 🎨️ Stroke and fill style for vector Sobjects.
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub fill_opacity: f64,
    pub stroke_opacity: f64,
    pub stroke_width: f64,
}

impl Default for Style {
    fn default() -> Self {
        Self { fill: Some(Color::WHITE), stroke: None, fill_opacity: 1.0, stroke_opacity: 1.0, stroke_width: 4.0 }
    }
}

/// 📐️ Axis-aligned bounds in scene space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn center(self) -> Point {
        Point::new((self.min.x() + self.max.x()) / 2.0, (self.min.y() + self.max.y()) / 2.0)
    }

    pub fn width(self) -> f64 {
        self.max.x() - self.min.x()
    }

    pub fn height(self) -> f64 {
        self.max.y() - self.min.y()
    }

    pub fn empty() -> Self {
        Self { min: Point::ZERO, max: Point::ZERO }
    }
}

/// 🧬️ Base scene-graph object contract.
#[dyn_enum]
pub trait Sobject: Send {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn opacity(&self) -> f64;
    fn set_opacity(&mut self, opacity: f64);
    fn effective_opacity(&self) -> f64;
    fn set_parent_opacity(&mut self, parent: f64);
    fn transform(&self) -> Affine;
    fn transform_mut(&mut self) -> &mut Affine;
    fn bounds(&self) -> Bounds;
    fn center(&self) -> Point {
        self.bounds().center()
    }
    fn shift(&mut self, delta: Vec2) {
        *self.transform_mut() = self.transform() * Affine::IDENTITY.translate(delta);
    }
    fn move_to(&mut self, point: Point) {
        let c = self.center();
        self.shift(point - c);
    }
    fn scale(&mut self, factor: f64) {
        let c = self.center();
        let t = Affine::IDENTITY.translate(c.to_vec2()) * Affine::IDENTITY.scale(factor) * Affine::IDENTITY.translate(-c.to_vec2());
        *self.transform_mut() = self.transform() * t;
    }
    fn rotate(&mut self, angle: f64) {
        let c = self.center();
        let t = Affine::IDENTITY.translate(c.to_vec2()) * Affine::IDENTITY.rotate(angle) * Affine::IDENTITY.translate(-c.to_vec2());
        *self.transform_mut() = self.transform() * t;
    }
    fn set_color(&mut self, color: Color) {
        self.style_mut().fill = Some(color);
        self.style_mut().stroke = Some(color);
    }
    fn set_fill(&mut self, color: Color) {
        self.style_mut().fill = Some(color);
    }
    fn set_stroke(&mut self, color: Color, width: f64) {
        self.style_mut().stroke = Some(color);
        self.style_mut().stroke_width = width;
    }
    fn paths(&self) -> Vec<BezPath>;
    fn children(&self) -> Vec<&Sobjects>;
    fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut Sobjects));
    fn add_child(&mut self, child: Sobjects);
    fn updaters(&self) -> &[Updater];
    fn updaters_mut(&mut self) -> &mut Vec<Updater>;
    fn save_state(&mut self);
    fn restore(&mut self);
    fn generate_target(&mut self);
    fn has_target(&self) -> bool;
    fn apply_target(&mut self);
    fn clone_box(&self) -> Sobjects;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn z_order(&self) -> i64 {
        0
    }
    fn set_z_order(&mut self, _z: i64) {}
    fn point_ratio(&self) -> f64 {
        1.0
    }
}

/// ✏️ Vector Sobject backed by `geometry::BezPath` paths and partial point reveal.
#[derive(Clone)]
pub struct VSobject {
    pub id: u64,
    pub name: String,
    pub paths: Vec<BezPath>,
    pub style: Style,
    pub opacity: f64,
    pub parent_opacity: f64,
    pub transform: Affine,
    pub point_ratio: f64,
    pub z_order: i64,
    pub saved: Option<VSobjectSnapshot>,
    pub target: Option<VSobjectSnapshot>,
    pub updaters: Vec<Updater>,
}

#[derive(Clone, Debug)]
pub struct VSobjectSnapshot {
    paths: Vec<BezPath>,
    style: Style,
    opacity: f64,
    transform: Affine,
    point_ratio: f64,
}

impl VSobject {
    pub fn new() -> Self {
        Self { id: next_id(), name: String::new(), paths: Vec::new(), style: Style::default(), opacity: 1.0, parent_opacity: 1.0, transform: Affine::IDENTITY, point_ratio: 1.0, z_order: 0, saved: None, target: None, updaters: Vec::new() }
    }

    pub fn from_path(path: BezPath) -> Self {
        let mut s = Self::new();
        s.paths.push(path);
        s
    }

    pub fn from_shape<'a>(shape: impl Into<geometry::ShapeRef<'a>>) -> Self {
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, shape, 0.01);
        Self::from_path(path)
    }

    pub fn set_paths(&mut self, paths: Vec<BezPath>) {
        self.paths = paths;
    }

    pub fn set_point_ratio(&mut self, ratio: f64) {
        self.point_ratio = ratio.clamp(0.0, 1.0);
    }

    fn snapshot(&self) -> VSobjectSnapshot {
        VSobjectSnapshot { paths: self.paths.clone(), style: self.style.clone(), opacity: self.opacity, transform: self.transform, point_ratio: self.point_ratio }
    }

    fn restore_snapshot(&mut self, snap: VSobjectSnapshot) {
        self.paths = snap.paths;
        self.style = snap.style;
        self.opacity = snap.opacity;
        self.transform = snap.transform;
        self.point_ratio = snap.point_ratio;
    }

    /// 🔀️ Linearly blend two snapshots into the live VSobject state.
    pub fn interpolate_snapshots(&mut self, from: &VSobjectSnapshot, to: &VSobjectSnapshot, t: f64) {
        let t = t.clamp(0.0, 1.0);
        self.opacity = lerp_f64(from.opacity, to.opacity, t);
        self.transform = lerp_affine(from.transform, to.transform, t);
        self.point_ratio = lerp_f64(from.point_ratio, to.point_ratio, t);
        self.style = if t < 0.5 { from.style.clone() } else { to.style.clone() };
        self.paths = interpolate_path_sets(&from.paths, &to.paths, t);
    }

    /// 🎯️ Blend from saved state toward the generated target.
    pub fn interpolate_saved_to_target(&mut self, t: f64) {
        if self.saved.is_none() {
            self.save_state();
        }
        if !self.has_target() {
            self.generate_target();
        }
        let from = self.saved.clone();
        let to = self.target.clone();
        if let (Some(from), Some(to)) = (from, to) {
            self.interpolate_snapshots(&from, &to, t);
        }
    }
}

fn lerp_f64(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// ✂️ Trim a path to a partial reveal ratio in [0,1].
pub fn trim_path_at_ratio(path: &BezPath, ratio: f64) -> BezPath {
    let ratio = ratio.clamp(0.0, 1.0);
    if ratio >= 1.0 {
        return path.clone();
    }
    if ratio <= 0.0 {
        return BezPath::new();
    }
    let segments = path.path_segments();
    let total: f64 = segments.iter().map(|s| s.arclen(0.25)).sum();
    if total <= 1e-12 {
        return path.clone();
    }
    let mut remaining = total * ratio;
    let mut out = BezPath::new();
    for seg in segments {
        let len = seg.arclen(0.25);
        if remaining >= len - 1e-9 {
            if out.is_empty() {
                out.move_to(seg.start());
            }
            out.push(seg.as_path_el());
            remaining -= len;
        } else {
            let frac = (remaining / len).clamp(0.0, 1.0);
            let sub = seg.subsegment(0.0, frac);
            if out.is_empty() {
                out.move_to(sub.start());
            }
            out.push(sub.as_path_el());
            break;
        }
    }
    out
}

fn interpolate_path_sets(from: &[BezPath], to: &[BezPath], t: f64) -> Vec<BezPath> {
    if from.is_empty() {
        return to.to_vec();
    }
    if to.is_empty() {
        return from.to_vec();
    }
    let count = from.len().max(to.len());
    (0..count)
        .map(|i| {
            let a = &from[i.min(from.len() - 1)];
            let b = &to[i.min(to.len() - 1)];
            interpolate_bezpaths(a, b, t)
        })
        .collect()
}

fn interpolate_bezpaths(from: &BezPath, to: &BezPath, t: f64) -> BezPath {
    let fa = resample_points(&sample_path_points(from, 32), 32);
    let tb = resample_points(&sample_path_points(to, 32), 32);
    let mut out = BezPath::new();
    for (i, (a, b)) in fa.iter().zip(tb.iter()).enumerate() {
        let p = Point::new(lerp_f64(a.x(), b.x(), t), lerp_f64(a.y(), b.y(), t));
        if i == 0 {
            out.move_to(p);
        } else {
            out.line_to(p);
        }
    }
    out
}

fn sample_path_points(path: &BezPath, samples: usize) -> Vec<Point> {
    let segments = path.path_segments();
    let total: f64 = segments.iter().map(|s| s.arclen(0.25)).sum();
    if total <= 1e-12 {
        return Vec::new();
    }
    let mut pts = Vec::with_capacity(samples);
    for i in 0..samples {
        let target = total * i as f64 / (samples - 1).max(1) as f64;
        let mut acc = 0.0;
        for (idx, seg) in segments.iter().enumerate() {
            let len = seg.arclen(0.25);
            if acc + len >= target || idx + 1 == segments.len() {
                let local = ((target - acc) / len.max(1e-12)).clamp(0.0, 1.0);
                pts.push(seg.eval(local));
                break;
            }
            acc += len;
        }
    }
    pts
}

fn resample_points(pts: &[Point], count: usize) -> Vec<Point> {
    if pts.is_empty() {
        return vec![Point::ZERO; count];
    }
    if count <= 1 {
        return vec![pts[0]];
    }
    (0..count)
        .map(|i| {
            let idx = i * (pts.len() - 1) / (count - 1);
            pts[idx]
        })
        .collect()
}

fn lerp_affine(a: Affine, b: Affine, t: f64) -> Affine {
    let ta = a.as_coeffs();
    let tb = b.as_coeffs();
    let t = t.clamp(0.0, 1.0);
    Affine::new([lerp_f64(ta[0], tb[0], t), lerp_f64(ta[1], tb[1], t), lerp_f64(ta[2], tb[2], t), lerp_f64(ta[3], tb[3], t), lerp_f64(ta[4], tb[4], t), lerp_f64(ta[5], tb[5], t)])
}

impl Default for VSobject {
    fn default() -> Self {
        Self::new()
    }
}

impl Sobject for VSobject {
    fn id(&self) -> u64 {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn style(&self) -> &Style {
        &self.style
    }
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
    fn opacity(&self) -> f64 {
        self.opacity
    }
    fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
    fn effective_opacity(&self) -> f64 {
        self.opacity * self.parent_opacity
    }
    fn set_parent_opacity(&mut self, parent: f64) {
        self.parent_opacity = parent.clamp(0.0, 1.0);
    }
    fn transform(&self) -> Affine {
        self.transform
    }
    fn transform_mut(&mut self) -> &mut Affine {
        &mut self.transform
    }
    fn bounds(&self) -> Bounds {
        let mut pts = Vec::new();
        for path in &self.paths {
            for el in path.elements() {
                if let Some(p) = el.as_ref_point() {
                    pts.push(self.transform * p);
                }
            }
        }
        if let Some(bb) = bounding_box(&pts) {
            Bounds { min: Point::new(bb.min_x, bb.min_y), max: Point::new(bb.max_x, bb.max_y) }
        } else {
            Bounds::empty()
        }
    }
    fn paths(&self) -> Vec<BezPath> {
        self.paths
            .iter()
            .map(|p| {
                let trimmed = trim_path_at_ratio(p, self.point_ratio);
                trimmed.apply_affine(self.transform)
            })
            .collect()
    }
    fn children(&self) -> Vec<&Sobjects> {
        Vec::new()
    }
    fn visit_children_mut(&mut self, _f: &mut dyn FnMut(&mut Sobjects)) {}
    fn add_child(&mut self, _child: Sobjects) {}
    fn updaters(&self) -> &[Updater] {
        &self.updaters
    }
    fn updaters_mut(&mut self) -> &mut Vec<Updater> {
        &mut self.updaters
    }
    fn save_state(&mut self) {
        self.saved = Some(self.snapshot());
    }
    fn restore(&mut self) {
        if let Some(s) = self.saved.take() {
            self.restore_snapshot(s);
        }
    }
    fn generate_target(&mut self) {
        self.target = Some(self.snapshot());
    }
    fn has_target(&self) -> bool {
        self.target.is_some()
    }
    fn apply_target(&mut self) {
        if let Some(t) = self.target.take() {
            self.restore_snapshot(t);
        }
    }
    fn clone_box(&self) -> Sobjects {
        self.clone().into()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn z_order(&self) -> i64 {
        self.z_order
    }
    fn set_z_order(&mut self, z: i64) {
        self.z_order = z;
    }
    fn point_ratio(&self) -> f64 {
        self.point_ratio
    }
}

/// 📦️ Group of heterogeneous Sobjects.
pub struct Group {
    pub id: u64,
    pub name: String,
    pub children: Vec<Sobjects>,
    pub style: Style,
    pub opacity: f64,
    pub parent_opacity: f64,
    pub transform: Affine,
    pub z_order: i64,
    pub saved: Option<GroupSnapshot>,
    pub target: Option<GroupSnapshot>,
    pub updaters: Vec<Updater>,
}

#[derive(Clone, Debug)]
pub struct GroupSnapshot {
    opacity: f64,
    transform: Affine,
}

impl Group {
    pub fn new(children: Vec<Sobjects>) -> Self {
        Self { id: next_id(), name: String::new(), children, style: Style::default(), opacity: 1.0, parent_opacity: 1.0, transform: Affine::IDENTITY, z_order: 0, saved: None, target: None, updaters: Vec::new() }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    fn propagate_parent_opacity(&mut self) {
        let eff = self.effective_opacity();
        for child in &mut self.children {
            child.set_parent_opacity(eff);
            if let Some(g) = child.as_any_mut().downcast_mut::<Group>() {
                g.propagate_parent_opacity();
            }
        }
    }
}

impl Sobject for Group {
    fn id(&self) -> u64 {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn style(&self) -> &Style {
        &self.style
    }
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
    fn opacity(&self) -> f64 {
        self.opacity
    }
    fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity.clamp(0.0, 1.0);
        self.propagate_parent_opacity();
    }
    fn effective_opacity(&self) -> f64 {
        self.opacity * self.parent_opacity
    }
    fn set_parent_opacity(&mut self, parent: f64) {
        self.parent_opacity = parent.clamp(0.0, 1.0);
        self.propagate_parent_opacity();
    }
    fn transform(&self) -> Affine {
        self.transform
    }
    fn transform_mut(&mut self) -> &mut Affine {
        &mut self.transform
    }
    fn bounds(&self) -> Bounds {
        let mut min = Point::new(f64::INFINITY, f64::INFINITY);
        let mut max = Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
        for child in &self.children {
            let b = child.bounds();
            if b.min.x().is_finite() {
                min = Point::new(min.x().min(b.min.x()), min.y().min(b.min.y()));
                max = Point::new(max.x().max(b.max.x()), max.y().max(b.max.y()));
            }
        }
        if min.x().is_finite() {
            Bounds { min, max }
        } else {
            Bounds::empty()
        }
    }
    fn paths(&self) -> Vec<BezPath> {
        self.children.iter().flat_map(|c| c.paths()).collect()
    }
    fn children(&self) -> Vec<&Sobjects> {
        self.children.iter().collect()
    }
    fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut Sobjects)) {
        for child in &mut self.children {
            f(child);
        }
    }
    fn add_child(&mut self, child: Sobjects) {
        self.children.push(child);
        self.propagate_parent_opacity();
    }
    fn updaters(&self) -> &[Updater] {
        &self.updaters
    }
    fn updaters_mut(&mut self) -> &mut Vec<Updater> {
        &mut self.updaters
    }
    fn save_state(&mut self) {
        for c in &mut self.children {
            c.save_state();
        }
        self.saved = Some(GroupSnapshot { opacity: self.opacity, transform: self.transform });
    }
    fn restore(&mut self) {
        for c in &mut self.children {
            c.restore();
        }
        if let Some(s) = self.saved.take() {
            self.opacity = s.opacity;
            self.transform = s.transform;
        }
    }
    fn generate_target(&mut self) {
        for c in &mut self.children {
            c.generate_target();
        }
        self.target = Some(GroupSnapshot { opacity: self.opacity, transform: self.transform });
    }
    fn has_target(&self) -> bool {
        self.target.is_some() || self.children.iter().any(|c| c.has_target())
    }
    fn apply_target(&mut self) {
        for c in &mut self.children {
            c.apply_target();
        }
        if let Some(t) = self.target.take() {
            self.opacity = t.opacity;
            self.transform = t.transform;
        }
    }
    fn clone_box(&self) -> Sobjects {
        Group {
            id: next_id(),
            name: self.name.clone(),
            children: self.children.iter().map(|c| c.clone_box()).collect(),
            style: self.style.clone(),
            opacity: self.opacity,
            parent_opacity: self.parent_opacity,
            transform: self.transform,
            z_order: self.z_order,
            saved: None,
            target: None,
            updaters: self.updaters.clone(),
        }
        .into()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn z_order(&self) -> i64 {
        self.z_order
    }
    fn set_z_order(&mut self, z: i64) {
        self.z_order = z;
    }
}

// 🔀️ R11 "closed set" case — three impls in this crate (`VSobject`, `Group` above,
// `ThreeDVSobject` in `⚙️engine/📐️geometry`), so `dyn_enum_close!` generates the enum +
// `impl Sobject for Sobjects` match-delegation (O1). Bare invocation is legal because this call
// site is in the SAME module as `#[dyn_enum] pub trait Sobject` above it (dyn-enum-macro finding 1).
dyn_enum_close! {
    pub enum Sobjects: Sobject {
        VSobject(VSobject),
        Group(Group),
        ThreeDVSobject(crate::editor::animate::engine::geometry::three_d::ThreeDVSobject),
    }
}

/// ✏️ Vector-only group convenience wrapper.
pub type VGroup = Group;

pub fn vgroup(children: Vec<Sobjects>) -> VGroup {
    Group::new(children)
}

/// ↔ Place `mover` next to `anchor` along a direction.
pub fn next_to(mover: &mut Sobjects, anchor: &Sobjects, direction: Vec2, buff: f64) {
    let mb = mover.bounds();
    let ab = anchor.bounds();
    let dir = if direction.hypot() < 1e-9 { Vec2::new(1.0, 0.0) } else { direction / direction.hypot() };
    let shift = if dir.x().abs() > dir.y().abs() {
        let edge = if dir.x() > 0.0 { ab.max.x() } else { ab.min.x() };
        let target = if dir.x() > 0.0 { edge + buff + mb.width() / 2.0 } else { edge - buff - mb.width() / 2.0 };
        Vec2::new(target - mb.center().x(), 0.0)
    } else {
        let edge = if dir.y() > 0.0 { ab.max.y() } else { ab.min.y() };
        let target = if dir.y() > 0.0 { edge + buff + mb.height() / 2.0 } else { edge - buff - mb.height() / 2.0 };
        Vec2::new(0.0, target - mb.center().y())
    };
    mover.shift(shift);
}

/// 📏️ Arrange children in a line.
pub fn arrange(group: &mut Group, direction: Vec2, buff: f64) {
    if group.children.is_empty() {
        return;
    }
    let dir = if direction.hypot() < 1e-9 { Vec2::new(1.0, 0.0) } else { direction / direction.hypot() };
    let mut cursor = group.children[0].center();
    for child in group.children.iter_mut().skip(1) {
        let b = child.bounds();
        let step = if dir.x().abs() > dir.y().abs() { b.width() / 2.0 + buff } else { b.height() / 2.0 + buff };
        cursor = Point::new(cursor.x() + dir.x() * step, cursor.y() + dir.y() * step);
        child.move_to(cursor);
    }
}

/// 🎯️ Align `mover` to `anchor` along an edge or center.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignEdge {
    Left,
    Right,
    Up,
    Down,
    Center,
}

pub fn align_to(mover: &mut Sobjects, anchor: &Sobjects, edge: AlignEdge) {
    let mb = mover.bounds();
    let ab = anchor.bounds();
    let shift = match edge {
        AlignEdge::Left => Vec2::new(ab.min.x() - mb.min.x(), 0.0),
        AlignEdge::Right => Vec2::new(ab.max.x() - mb.max.x(), 0.0),
        AlignEdge::Up => Vec2::new(0.0, ab.max.y() - mb.max.y()),
        AlignEdge::Down => Vec2::new(0.0, ab.min.y() - mb.min.y()),
        AlignEdge::Center => anchor.center() - mover.center(),
    };
    mover.shift(shift);
}

pub fn center_of_points(points: &[Point]) -> Point {
    if points.is_empty() {
        Point::ZERO
    } else {
        polygon_centroid(points)
    }
}

trait PathElPoint {
    fn as_ref_point(&self) -> Option<Point>;
}

impl PathElPoint for PathEl {
    fn as_ref_point(&self) -> Option<Point> {
        match self {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => Some(*p),
            PathEl::QuadTo(p, _) | PathEl::CurveTo(p, _, _) => Some(*p),
            PathEl::ClosePath => None,
        }
    }
}

trait PointVec2 {
    fn to_vec2(self) -> Vec2;
}

impl PointVec2 for Point {
    fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x(), self.y())
    }
}

#[cfg(test)]
include!("../🧪️tests/🔬️sobject-unit/🦀️.rs");
