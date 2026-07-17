//! 🧩 Sobject trait, VSobject paths, groups, transforms, and layout helpers.

use crate::color::Color;
use crate::updater::Updater;
use mathematical_geometry::{append_shape_to_path, bounding_box, polygon_centroid, Affine, BezPath, Point, Rect, Vec2};
use std::sync::atomic::{AtomicU64, Ordering};

static SOBJECT_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    SOBJECT_ID.fetch_add(1, Ordering::Relaxed)
}

/// 🎨 Stroke and fill style for vector Sobjects.
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
        Self {
            fill: Some(Color::WHITE),
            stroke: None,
            fill_opacity: 1.0,
            stroke_opacity: 1.0,
            stroke_width: 4.0,
        }
    }
}

/// 📐 Axis-aligned bounds in scene space.
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
        Self {
            min: Point::ZERO,
            max: Point::ZERO,
        }
    }
}

/// 🧬 Base scene-graph object contract.
pub trait Sobject: Send {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
    fn set_name(&mut self, name: impl Into<String>);
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
    fn children(&self) -> Vec<&dyn Sobject>;
    fn children_mut(&mut self) -> Vec<&mut dyn Sobject>;
    fn add_child(&mut self, child: Box<dyn Sobject>);
    fn updaters(&self) -> &[Updater];
    fn updaters_mut(&mut self) -> &mut Vec<Updater>;
    fn save_state(&mut self);
    fn restore(&mut self);
    fn generate_target(&mut self);
    fn has_target(&self) -> bool;
    fn apply_target(&mut self);
    fn clone_box(&self) -> Box<dyn Sobject>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// ✏️ Vector Sobject backed by kurbo paths and partial point reveal.
#[derive(Clone, Debug)]
pub struct VSobject {
    pub id: u64,
    pub name: String,
    pub paths: Vec<BezPath>,
    pub style: Style,
    pub opacity: f64,
    pub parent_opacity: f64,
    pub transform: Affine,
    pub point_ratio: f64,
    pub saved: Option<VSobjectSnapshot>,
    pub target: Option<VSobjectSnapshot>,
    pub updaters: Vec<Updater>,
}

#[derive(Clone, Debug)]
struct VSobjectSnapshot {
    paths: Vec<BezPath>,
    style: Style,
    opacity: f64,
    transform: Affine,
    point_ratio: f64,
}

impl VSobject {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            name: String::new(),
            paths: Vec::new(),
            style: Style::default(),
            opacity: 1.0,
            parent_opacity: 1.0,
            transform: Affine::IDENTITY,
            point_ratio: 1.0,
            saved: None,
            target: None,
            updaters: Vec::new(),
        }
    }

    pub fn from_path(path: BezPath) -> Self {
        let mut s = Self::new();
        s.paths.push(path);
        s
    }

    pub fn from_shape(shape: impl Into<mathematical_geometry::ShapeRef<'_>>) -> Self {
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
        VSobjectSnapshot {
            paths: self.paths.clone(),
            style: self.style.clone(),
            opacity: self.opacity,
            transform: self.transform,
            point_ratio: self.point_ratio,
        }
    }

    fn restore_snapshot(&mut self, snap: VSobjectSnapshot) {
        self.paths = snap.paths;
        self.style = snap.style;
        self.opacity = snap.opacity;
        self.transform = snap.transform;
        self.point_ratio = snap.point_ratio;
    }
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
    fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
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
            Bounds {
                min: Point::new(bb.min_x, bb.min_y),
                max: Point::new(bb.max_x, bb.max_y),
            }
        } else {
            Bounds::empty()
        }
    }
    fn paths(&self) -> Vec<BezPath> {
        let t = self.transform.to_kurbo();
        self.paths
            .iter()
            .map(|p| {
                let mut k = p.to_kurbo();
                k.apply_affine(t);
                BezPath(k)
            })
            .collect()
    }
    fn children(&self) -> Vec<&dyn Sobject> {
        Vec::new()
    }
    fn children_mut(&mut self) -> Vec<&mut dyn Sobject> {
        Vec::new()
    }
    fn add_child(&mut self, _child: Box<dyn Sobject>) {}
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
    fn clone_box(&self) -> Box<dyn Sobject> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// 📦 Group of heterogeneous Sobjects.
#[derive(Debug)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub children: Vec<Box<dyn Sobject>>,
    pub style: Style,
    pub opacity: f64,
    pub parent_opacity: f64,
    pub transform: Affine,
    pub saved: Option<GroupSnapshot>,
    pub target: Option<GroupSnapshot>,
    pub updaters: Vec<Updater>,
}

#[derive(Clone, Debug)]
struct GroupSnapshot {
    opacity: f64,
    transform: Affine,
}

impl Group {
    pub fn new(children: Vec<Box<dyn Sobject>>) -> Self {
        Self {
            id: next_id(),
            name: String::new(),
            children,
            style: Style::default(),
            opacity: 1.0,
            parent_opacity: 1.0,
            transform: Affine::IDENTITY,
            saved: None,
            target: None,
            updaters: Vec::new(),
        }
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
    fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
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
    fn children(&self) -> Vec<&dyn Sobject> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }
    fn children_mut(&mut self) -> Vec<&mut dyn Sobject> {
        self.children.iter_mut().map(|c| c.as_mut()).collect()
    }
    fn add_child(&mut self, child: Box<dyn Sobject>) {
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
        self.saved = Some(GroupSnapshot {
            opacity: self.opacity,
            transform: self.transform,
        });
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
        self.target = Some(GroupSnapshot {
            opacity: self.opacity,
            transform: self.transform,
        });
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
    fn clone_box(&self) -> Box<dyn Sobject> {
        Box::new(Group {
            id: next_id(),
            name: self.name.clone(),
            children: self.children.iter().map(|c| c.clone_box()).collect(),
            style: self.style.clone(),
            opacity: self.opacity,
            parent_opacity: self.parent_opacity,
            transform: self.transform,
            saved: None,
            target: None,
            updaters: self.updaters.clone(),
        })
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// ✏️ Vector-only group convenience wrapper.
pub type VGroup = Group;

pub fn vgroup(children: Vec<Box<dyn Sobject>>) -> VGroup {
    Group::new(children)
}

/// ↔️ Place `mover` next to `anchor` along a direction.
pub fn next_to(mover: &mut dyn Sobject, anchor: &dyn Sobject, direction: Vec2, buff: f64) {
    let mb = mover.bounds();
    let ab = anchor.bounds();
    let dir = if direction.hypot() < 1e-9 {
        Vec2::new(1.0, 0.0)
    } else {
        direction / direction.hypot()
    };
    let shift = if dir.x().abs() > dir.y().abs() {
        let edge = if dir.x() > 0.0 { ab.max.x() } else { ab.min.x() };
        let target = if dir.x() > 0.0 {
            edge + buff + mb.width() / 2.0
        } else {
            edge - buff - mb.width() / 2.0
        };
        Vec2::new(target - mb.center().x(), 0.0)
    } else {
        let edge = if dir.y() > 0.0 { ab.max.y() } else { ab.min.y() };
        let target = if dir.y() > 0.0 {
            edge + buff + mb.height() / 2.0
        } else {
            edge - buff - mb.height() / 2.0
        };
        Vec2::new(0.0, target - mb.center().y())
    };
    mover.shift(shift);
}

/// 📏 Arrange children in a line.
pub fn arrange(group: &mut Group, direction: Vec2, buff: f64) {
    if group.children.is_empty() {
        return;
    }
    let dir = if direction.hypot() < 1e-9 {
        Vec2::new(1.0, 0.0)
    } else {
        direction / direction.hypot()
    };
    let mut cursor = group.children[0].center();
    for child in group.children.iter_mut().skip(1) {
        let b = child.bounds();
        let step = if dir.x().abs() > dir.y().abs() {
            b.width() / 2.0 + buff
        } else {
            b.height() / 2.0 + buff
        };
        cursor = Point::new(cursor.x() + dir.x() * step, cursor.y() + dir.y() * step);
        child.move_to(cursor);
    }
}

/// 🎯 Align `mover` to `anchor` along an edge or center.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignEdge {
    Left,
    Right,
    Up,
    Down,
    Center,
}

pub fn align_to(mover: &mut dyn Sobject, anchor: &dyn Sobject, edge: AlignEdge) {
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

impl PathElPoint for mathematical_geometry::PathEl {
    fn as_ref_point(&self) -> Option<Point> {
        match self {
            mathematical_geometry::PathEl::MoveTo(p) | mathematical_geometry::PathEl::LineTo(p) => Some(*p),
            mathematical_geometry::PathEl::QuadTo(p, _) | mathematical_geometry::PathEl::CurveTo(p, _, _) => Some(*p),
            mathematical_geometry::PathEl::ClosePath => None,
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
mod tests {
    use super::*;
    use mathematical_geometry::Circle;

    #[test]
    fn vobject_has_finite_bounds() {
        let dot = VSobject::from_shape(Circle::new(Point::new(0.0, 0.0), 1.0));
        let b = dot.bounds();
        assert!(b.max.x() > b.min.x());
    }

    #[test]
    fn parent_opacity_multiplies() {
        let mut v = VSobject::new();
        v.set_opacity(0.5);
        v.set_parent_opacity(0.5);
        assert!((v.effective_opacity() - 0.25).abs() < 1e-9);
    }

    #[test]
    fn group_propagates_parent_opacity() {
        let mut g = Group::new(vec![Box::new(VSobject::new())]);
        g.set_opacity(0.5);
        assert!((g.children[0].effective_opacity() - 0.5).abs() < 1e-9);
    }
}
