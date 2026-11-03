//! 📐️ Geometry primitives: SI metres/radians, right-handed, Z up; plus the shared scalar and text vocabulary.

use super::*;

records! {
    /// 🌐️ Mandatory bilingual copy; scenes carry both languages and never a default one.
    pub struct LocalizedCopy {
        pub en: String,
        pub de: String,
    }

    /// 📖️ Clause coordinate (family, part, section) a scene annotation can point at; opaque to this crate.
    pub struct ClauseId {
        pub family: String,
        pub part: String,
        pub section: String,
    }

    /// 🔢️ A scalar of a physical kind in SI base units.
    pub struct Quantity {
        pub kind: QuantityKind,
        pub value: f64,
    }

    /// ↔️ Planar point or vector in metres.
    pub struct Vec2 {
        pub x: f64,
        pub y: f64,
    }

    /// 🧭️ Spatial point or vector in metres.
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    /// 🔄️ Unit quaternion rotation.
    pub struct Rotation {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }

    /// 🧮️ Scale, then rotation, then translation.
    pub struct Transform {
        pub translation: Vec3,
        pub rotation: Rotation,
        pub scale: Vec3,
    }

    /// 🧷️ Orthonormal right-handed frame: `x` along a member axis, `y` and `z` the section axes.
    pub struct Frame {
        pub origin: Vec3,
        pub x: Vec3,
        pub y: Vec3,
        pub z: Vec3,
    }

    /// 📦️ Axis-aligned bounding box.
    pub struct Aabb {
        pub min: Vec3,
        pub max: Vec3,
    }

    /// 〰️ Open planar chain of at least two points.
    pub struct Polyline2 {
        pub points: Vec<Vec2>,
    }

    /// 📈️ Open spatial chain of at least two points.
    pub struct Polyline3 {
        pub points: Vec<Vec3>,
    }

    /// ⭕️ Closed planar loop of at least three points.
    pub struct Ring2 {
        pub points: Vec<Vec2>,
    }

    /// 🔁️ Closed spatial loop of at least three points.
    pub struct Ring3 {
        pub points: Vec<Vec3>,
    }

    /// 🔷️ Planar region: an outer loop minus holes.
    pub struct Polygon2 {
        pub outer: Ring2,
        pub holes: Vec<Ring2>,
    }

    /// 🔶️ Spatial planar region: an outer loop minus holes.
    pub struct Polygon3 {
        pub outer: Ring3,
        pub holes: Vec<Ring3>,
    }

    /// 🌈️ Circular arc starting at `start`, swept around `normal` through `centre`.
    pub struct Arc {
        pub centre: Vec3,
        pub normal: Vec3,
        pub start: Vec3,
        pub sweep: f64,
    }

    /// 🪢️ Clamped B-spline through control points.
    pub struct Spline3 {
        pub control: Vec<Vec3>,
        pub degree: u32,
    }

    /// 📏️ Straight segment between two points.
    pub struct Segment3 {
        pub from: Vec3,
        pub to: Vec3,
    }
}

variants! {
    /// 🛤️ Member axis path.
    pub enum Path3 {
        Polyline(Polyline3),
        Arc(Arc),
        Spline(Spline3),
    }
}

choices! {
    /// 🧪️ Physical kind of a scalar; each kind has one SI base unit.
    pub enum QuantityKind {
        Dimensionless,
        Length,
        Area,
        Volume,
        Mass,
        Time,
        Temperature,
        Angle,
        Force,
        Moment,
        Pressure,
        Stress,
        Energy,
        Acceleration,
        Velocity,
        Frequency,
        ForcePerLength,
        Stiffness,
    }
}

impl LocalizedCopy {
    pub fn new(en: impl Into<String>, de: impl Into<String>) -> Self {
        Self { en: en.into(), de: de.into() }
    }
}

impl Quantity {
    pub fn new(kind: QuantityKind, value: f64) -> Self {
        Self { kind, value }
    }
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, o: &Self) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o: &Self) -> Self {
        Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn distance(&self, o: &Self) -> f64 {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z).length()
    }
}

impl Rotation {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
}

impl Transform {
    pub const IDENTITY: Self = Self { translation: Vec3::ZERO, rotation: Rotation::IDENTITY, scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 } };
}

impl Frame {
    pub const WORLD: Self = Self { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::Y, z: Vec3::Z };
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn of_points<'a>(points: impl IntoIterator<Item = &'a Vec3>) -> Option<Self> {
        points.into_iter().fold(None, |bounds: Option<Self>, p| {
            Some(match bounds {
                None => Self::new(p.clone(), p.clone()),
                Some(b) => Self::new(
                    Vec3::new(b.min.x.min(p.x), b.min.y.min(p.y), b.min.z.min(p.z)),
                    Vec3::new(b.max.x.max(p.x), b.max.y.max(p.y), b.max.z.max(p.z)),
                ),
            })
        })
    }
}

impl Ring2 {
    pub fn signed_area(&self) -> f64 {
        let n = self.points.len();
        (0..n).map(|i| {
            let (a, b) = (&self.points[i], &self.points[(i + 1) % n]);
            a.x * b.y - b.x * a.y
        }).sum::<f64>() / 2.0
    }
}

impl Polygon2 {
    pub fn solid(points: Vec<Vec2>) -> Self {
        Self { outer: Ring2 { points }, holes: Vec::new() }
    }

    pub fn area(&self) -> f64 {
        self.outer.signed_area().abs() - self.holes.iter().map(|h| h.signed_area().abs()).sum::<f64>()
    }
}

impl Validate for LocalizedCopy {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "en"), &self.en);
        non_empty(out, &at(path, "de"), &self.de);
    }
}

impl Validate for ClauseId {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "family"), &self.family);
        non_empty(out, &at(path, "section"), &self.section);
    }
}

impl Validate for Quantity {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, &at(path, "value"), &[self.value]);
    }
}

impl Validate for Vec2 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, path, &[self.x, self.y]);
    }
}

impl Validate for Vec3 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, path, &[self.x, self.y, self.z]);
    }
}

impl Validate for Rotation {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, path, &[self.x, self.y, self.z, self.w]);
        let norm = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        expect(out, path, (norm - 1.0).abs() < TOLERANCE, "unit-quaternion");
    }
}

impl Validate for Transform {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.translation.check(&at(path, "translation"), out);
        self.rotation.check(&at(path, "rotation"), out);
        self.scale.check(&at(path, "scale"), out);
        expect(out, &at(path, "scale"), self.scale.x > 0.0 && self.scale.y > 0.0 && self.scale.z > 0.0, "positive");
    }
}

impl Validate for Frame {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.origin.check(&at(path, "origin"), out);
        for (name, axis) in [("x", &self.x), ("y", &self.y), ("z", &self.z)] {
            axis.check(&at(path, name), out);
            expect(out, &at(path, name), (axis.length() - 1.0).abs() < TOLERANCE, "unit-axis");
        }
        expect(out, path, self.x.dot(&self.y).abs() < TOLERANCE && self.x.dot(&self.z).abs() < TOLERANCE && self.y.dot(&self.z).abs() < TOLERANCE, "orthogonal");
        expect(out, path, self.x.cross(&self.y).dot(&self.z) > 0.0, "right-handed");
    }
}

impl Validate for Aabb {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.min.check(&at(path, "min"), out);
        self.max.check(&at(path, "max"), out);
        expect(out, path, self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z, "ordered");
    }
}

impl Validate for Polyline2 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, path, self.points.len() >= 2, "at-least-2-points");
        each(&self.points, path, "points", out);
    }
}

impl Validate for Polyline3 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, path, self.points.len() >= 2, "at-least-2-points");
        each(&self.points, path, "points", out);
    }
}

impl Validate for Ring2 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, path, self.points.len() >= 3, "at-least-3-points");
        each(&self.points, path, "points", out);
    }
}

impl Validate for Ring3 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, path, self.points.len() >= 3, "at-least-3-points");
        each(&self.points, path, "points", out);
    }
}

impl Validate for Polygon2 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.outer.check(&at(path, "outer"), out);
        each(&self.holes, path, "holes", out);
        if out.is_empty() {
            expect(out, path, self.area() > 0.0, "positive-area");
        }
    }
}

impl Validate for Polygon3 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.outer.check(&at(path, "outer"), out);
        each(&self.holes, path, "holes", out);
    }
}

impl Validate for Arc {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.centre.check(&at(path, "centre"), out);
        self.normal.check(&at(path, "normal"), out);
        self.start.check(&at(path, "start"), out);
        finite(out, &at(path, "sweep"), &[self.sweep]);
        expect(out, &at(path, "normal"), (self.normal.length() - 1.0).abs() < TOLERANCE, "unit-axis");
        expect(out, &at(path, "start"), self.start.distance(&self.centre) > 0.0, "positive-radius");
        expect(out, &at(path, "sweep"), self.sweep != 0.0 && self.sweep.abs() <= std::f64::consts::TAU + TOLERANCE, "sweep-within-full-turn");
    }
}

impl Validate for Spline3 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        each(&self.control, path, "control", out);
        expect(out, &at(path, "degree"), self.degree >= 1 && self.control.len() > self.degree as usize, "degree-below-control-count");
    }
}

impl Validate for Segment3 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.from.check(&at(path, "from"), out);
        self.to.check(&at(path, "to"), out);
        expect(out, path, self.from.distance(&self.to) > 0.0, "non-degenerate");
    }
}

impl Validate for Path3 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Polyline(v) => v.check(path, out),
            Self::Arc(v) => v.check(path, out),
            Self::Spline(v) => v.check(path, out),
        }
    }
}
