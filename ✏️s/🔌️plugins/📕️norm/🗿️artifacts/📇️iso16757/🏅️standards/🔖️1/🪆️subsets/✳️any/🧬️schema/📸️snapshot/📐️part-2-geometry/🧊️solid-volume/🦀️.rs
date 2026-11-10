//! 🧊️ ISO 16757-2 7.1 solid volume and bounds of a CSG geometry tree: exact primitive volumes, rigid transforms (`rotationDeg` is intrinsic about the local origin, `R = Rz * Ry * Rx`, applied before `translation`), exact world bounds and boolean volumes. Disjoint operands are summed exactly; overlapping operands are integrated by adaptive Simpson quadrature over exact per-line interval sets (every primitive is convex, so a line meets it in one interval); the reported error is the quadrature estimate, bounded by `SIMPSON_RELATIVE_TOLERANCE` times the bounding volume. See [ISO 16757-2](https://www.iso.org/standard/70011.html).

use super::{BooleanOperator, BoundingBox, GeometryCatalogue, GeometryNode};
use std::cell::Cell;

/// 🎯️ Relative quadrature tolerance (of the bounding volume) for overlapping booleans.
pub const SIMPSON_RELATIVE_TOLERANCE: f64 = 1e-8;
const MAX_DEPTH: u32 = 26;
const SPLITS: usize = 8;
const MAX_REFERENCE_DEPTH: usize = 32;

/// 🚫️ Reasons a geometry tree has no solid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolidError {
    Missing(&'static str),
    UnknownKind,
    EmptyBoolean,
    Unresolved,
    Cycle,
}

/// 🧮️ Solid volume with the quadrature error estimate (`exact` when closed forms sufficed).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolidVolume {
    pub volume_m3: f64,
    pub error_m3: f64,
    pub exact: bool,
}

/// 📐️ A point or extent in metres.
pub type V3 = [f64; 3];
/// 🔄️ A row-major rotation matrix.
pub type M3 = [[f64; 3]; 3];
type Spans = Vec<(f64, f64)>;

const IDENTITY: M3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// 🧱️ A schema primitive with its dimensions: box (width x, height y, depth z, spanning `[0, size]`), cylinder (axis y, radius about `(r, ., r)`) and sphere (centre `(r, r, r)`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Box(V3),
    Cylinder(f64, f64),
    Sphere(f64),
}

/// 📍️ A primitive in its world frame: `world = rot * local + origin`; the single definition of the axis convention shared by checks and scene.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Primitive {
    pub shape: Shape,
    pub rot: M3,
    pub origin: V3,
}

enum Solid {
    Leaf(Primitive),
    Combine(BooleanOperator, Vec<Solid>),
}

fn mul(a: &M3, b: &M3) -> M3 {
    let mut out = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    out
}

fn apply(m: &M3, v: V3) -> V3 {
    [0, 1, 2].map(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}

fn rotation(deg: V3) -> M3 {
    let [a, b, g] = deg.map(f64::to_radians);
    let (sa, ca, sb, cb, sg, cg) = (a.sin(), a.cos(), b.sin(), b.cos(), g.sin(), g.cos());
    let rx = [[1.0, 0.0, 0.0], [0.0, ca, -sa], [0.0, sa, ca]];
    let ry = [[cb, 0.0, sb], [0.0, 1.0, 0.0], [-sb, 0.0, cb]];
    let rz = [[cg, -sg, 0.0], [sg, cg, 0.0], [0.0, 0.0, 1.0]];
    mul(&rz, &mul(&ry, &rx))
}

fn parameter(parameters: &std::collections::BTreeMap<String, f64>, key: &'static str) -> Result<f64, SolidError> {
    parameters.get(key).copied().ok_or(SolidError::Missing(key))
}

fn build(node: &GeometryNode, catalogue: &GeometryCatalogue, rot: M3, origin: V3, depth: usize) -> Result<Solid, SolidError> {
    match node {
        GeometryNode::Primitive { kind, parameters } => {
            let shape = match kind.as_str() {
                "box" => Shape::Box([parameter(parameters, "width")?, parameter(parameters, "height")?, parameter(parameters, "depth")?]),
                "cylinder" => Shape::Cylinder(parameter(parameters, "radius")?, parameter(parameters, "height")?),
                "sphere" => Shape::Sphere(parameter(parameters, "radius")?),
                _ => return Err(SolidError::UnknownKind),
            };
            Ok(Solid::Leaf(Primitive { shape, rot, origin }))
        }
        GeometryNode::Transform { translation, rotation_deg, child } => {
            let moved = apply(&rot, *translation);
            build(child, catalogue, mul(&rot, &rotation(*rotation_deg)), [origin[0] + moved[0], origin[1] + moved[1], origin[2] + moved[2]], depth)
        }
        GeometryNode::Boolean { operator, children } => {
            if children.is_empty() {
                return Err(SolidError::EmptyBoolean);
            }
            children.iter().map(|c| build(c, catalogue, rot, origin, depth)).collect::<Result<Vec<_>, _>>().map(|c| Solid::Combine(*operator, c))
        }
        GeometryNode::Reference { geometry_id } => {
            if depth >= MAX_REFERENCE_DEPTH {
                return Err(SolidError::Cycle);
            }
            let shape = catalogue.objects.get(geometry_id).and_then(|o| o.shape.as_ref()).ok_or(SolidError::Unresolved)?;
            build(shape, catalogue, rot, origin, depth + 1)
        }
    }
}

impl Primitive {
    pub fn volume(&self) -> f64 {
        match self.shape {
            Shape::Box(s) => s[0] * s[1] * s[2],
            Shape::Cylinder(r, h) => std::f64::consts::PI * r * r * h,
            Shape::Sphere(r) => 4.0 / 3.0 * std::f64::consts::PI * r * r * r,
        }
    }

    pub fn local_center(&self) -> V3 {
        match self.shape {
            Shape::Box(s) => [s[0] / 2.0, s[1] / 2.0, s[2] / 2.0],
            Shape::Cylinder(r, h) => [r, h / 2.0, r],
            Shape::Sphere(r) => [r, r, r],
        }
    }

    pub fn bounds(&self) -> BoundingBox {
        let c = apply(&self.rot, self.local_center());
        let mut b = BoundingBox { min: [0.0; 3], max: [0.0; 3] };
        for i in 0..3 {
            let half = match self.shape {
                Shape::Box(s) => (0..3).map(|j| self.rot[i][j].abs() * s[j] / 2.0).sum(),
                Shape::Cylinder(r, h) => {
                    let ax = self.rot[i][1];
                    ax.abs() * h / 2.0 + r * (1.0 - ax * ax).max(0.0).sqrt()
                }
                Shape::Sphere(r) => r,
            };
            b.min[i] = self.origin[i] + c[i] - half;
            b.max[i] = self.origin[i] + c[i] + half;
        }
        b
    }

    fn chord(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let p = [x - self.origin[0], y - self.origin[1], -self.origin[2]];
        let o = [0, 1, 2].map(|j| (0..3).map(|i| self.rot[i][j] * p[i]).sum::<f64>());
        let d = self.rot[2];
        match self.shape {
            Shape::Box(s) => slab(o, d, [0.0; 3], s, (f64::NEG_INFINITY, f64::INFINITY)),
            Shape::Sphere(r) => {
                let oc = [o[0] - r, o[1] - r, o[2] - r];
                let b = oc[0] * d[0] + oc[1] * d[1] + oc[2] * d[2];
                let disc = b * b - (oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - r * r);
                (disc > 0.0).then(|| (-b - disc.sqrt(), -b + disc.sqrt()))
            }
            Shape::Cylinder(r, h) => {
                let oc = [o[0] - r, o[1] - h / 2.0, o[2] - r];
                let a = d[0] * d[0] + d[2] * d[2];
                let b = oc[0] * d[0] + oc[2] * d[2];
                let c = oc[0] * oc[0] + oc[2] * oc[2] - r * r;
                let ring = if a < 1e-24 {
                    (c < 0.0).then_some((f64::NEG_INFINITY, f64::INFINITY))
                } else {
                    let disc = b * b - a * c;
                    (disc > 0.0).then(|| ((-b - disc.sqrt()) / a, (-b + disc.sqrt()) / a))
                }?;
                slab([0.0, oc[1], 0.0], [0.0, d[1], 0.0], [0.0, -h / 2.0, 0.0], [0.0, h, 0.0], ring)
            }
        }
    }
}

fn slab(o: V3, d: V3, lo: V3, size: V3, mut span: (f64, f64)) -> Option<(f64, f64)> {
    for j in 0..3 {
        if d[j].abs() < 1e-14 {
            if o[j] < lo[j] || o[j] > lo[j] + size[j] {
                return None;
            }
            continue;
        }
        let (t1, t2) = ((lo[j] - o[j]) / d[j], (lo[j] + size[j] - o[j]) / d[j]);
        span = (span.0.max(t1.min(t2)), span.1.min(t1.max(t2)));
    }
    (span.0 < span.1).then_some(span)
}

fn hull(a: BoundingBox, b: BoundingBox) -> BoundingBox {
    BoundingBox { min: [0, 1, 2].map(|i| a.min[i].min(b.min[i])), max: [0, 1, 2].map(|i| a.max[i].max(b.max[i])) }
}

fn separated(a: &BoundingBox, b: &BoundingBox) -> bool {
    (0..3).any(|i| a.max[i] <= b.min[i] || b.max[i] <= a.min[i])
}

impl Solid {
    fn bounds(&self) -> BoundingBox {
        match self {
            Solid::Leaf(p) => p.bounds(),
            Solid::Combine(BooleanOperator::Union, c) => c.iter().map(Solid::bounds).reduce(hull).unwrap_or(BoundingBox { min: [0.0; 3], max: [0.0; 3] }),
            Solid::Combine(BooleanOperator::Difference, c) => c[0].bounds(),
            Solid::Combine(BooleanOperator::Intersection, c) => {
                let mut acc = c[0].bounds();
                for b in c[1..].iter().map(Solid::bounds) {
                    for i in 0..3 {
                        acc.min[i] = acc.min[i].max(b.min[i]);
                        acc.max[i] = acc.max[i].min(b.max[i]).max(acc.min[i]);
                    }
                }
                acc
            }
        }
    }

    fn leaves<'a>(&'a self, out: &mut Vec<&'a Primitive>) {
        match self {
            Solid::Leaf(p) => out.push(p),
            Solid::Combine(_, c) => c.iter().for_each(|s| s.leaves(out)),
        }
    }

    fn exact(&self) -> Option<f64> {
        match self {
            Solid::Leaf(p) => Some(p.volume()),
            Solid::Combine(BooleanOperator::Union, c) => {
                let bounds: Vec<_> = c.iter().map(Solid::bounds).collect();
                let apart = (0..bounds.len()).all(|i| (i + 1..bounds.len()).all(|j| separated(&bounds[i], &bounds[j])));
                if !apart {
                    return None;
                }
                c.iter().map(Solid::exact).sum()
            }
            Solid::Combine(BooleanOperator::Intersection, c) => {
                let bounds: Vec<_> = c.iter().map(Solid::bounds).collect();
                let apart = (0..bounds.len()).any(|i| (i + 1..bounds.len()).any(|j| separated(&bounds[i], &bounds[j])));
                apart.then_some(0.0)
            }
            Solid::Combine(BooleanOperator::Difference, c) => {
                let first = c[0].bounds();
                c[1..].iter().all(|s| separated(&first, &s.bounds())).then(|| c[0].exact()).flatten()
            }
        }
    }

    fn spans(&self, x: f64, y: f64) -> Spans {
        match self {
            Solid::Leaf(p) => p.chord(x, y).into_iter().collect(),
            Solid::Combine(op, c) => {
                let mut parts = c.iter().map(|s| s.spans(x, y));
                let first = parts.next().unwrap_or_default();
                match op {
                    BooleanOperator::Union => parts.fold(first, |a, b| union(a, b)),
                    BooleanOperator::Intersection => parts.fold(first, |a, b| intersect(&a, &b)),
                    BooleanOperator::Difference => {
                        let rest = parts.fold(Vec::new(), |a, b| union(a, b));
                        subtract(&first, &rest)
                    }
                }
            }
        }
    }
}

fn union(mut a: Spans, mut b: Spans) -> Spans {
    a.append(&mut b);
    a.sort_by(|p, q| p.0.total_cmp(&q.0));
    let mut out: Spans = Vec::new();
    for s in a {
        match out.last_mut() {
            Some(last) if s.0 <= last.1 => last.1 = last.1.max(s.1),
            _ => out.push(s),
        }
    }
    out
}

fn intersect(a: &Spans, b: &Spans) -> Spans {
    a.iter().flat_map(|p| b.iter().map(move |q| (p.0.max(q.0), p.1.min(q.1)))).filter(|s| s.0 < s.1).collect()
}

fn subtract(a: &Spans, b: &Spans) -> Spans {
    let mut out = Vec::new();
    for &(lo, hi) in a {
        let mut cursor = lo;
        for &(l, h) in b.iter().filter(|s| s.1 > lo && s.0 < hi) {
            if l > cursor {
                out.push((cursor, l));
            }
            cursor = cursor.max(h);
        }
        if cursor < hi {
            out.push((cursor, hi));
        }
    }
    out
}

fn adaptive(f: &mut dyn FnMut(f64) -> f64, a: f64, b: f64, fa: f64, fm: f64, fb: f64, whole: f64, tol: f64, depth: u32, err: &mut f64) -> f64 {
    let m = (a + b) / 2.0;
    let (flm, frm) = (f((a + m) / 2.0), f((m + b) / 2.0));
    let left = (m - a) / 6.0 * (fa + 4.0 * flm + fm);
    let right = (b - m) / 6.0 * (fm + 4.0 * frm + fb);
    let delta = left + right - whole;
    if depth == 0 || delta.abs() <= 15.0 * tol {
        *err += delta.abs() / 15.0;
        return left + right + delta / 15.0;
    }
    adaptive(f, a, m, fa, flm, fm, left, tol / 2.0, depth - 1, err) + adaptive(f, m, b, fm, frm, fb, right, tol / 2.0, depth - 1, err)
}

fn integrate(f: &mut dyn FnMut(f64) -> f64, knots: &[f64], tol: f64, err: &mut f64) -> f64 {
    let mut total = 0.0;
    let span = knots[knots.len() - 1] - knots[0];
    for w in knots.windows(2) {
        let share = (w[1] - w[0]) / span;
        for k in 0..SPLITS {
            let a = w[0] + (w[1] - w[0]) * k as f64 / SPLITS as f64;
            let b = w[0] + (w[1] - w[0]) * (k + 1) as f64 / SPLITS as f64;
            let (fa, fm, fb) = (f(a), f((a + b) / 2.0), f(b));
            let whole = (b - a) / 6.0 * (fa + 4.0 * fm + fb);
            total += adaptive(f, a, b, fa, fm, fb, whole, tol * share / SPLITS as f64, MAX_DEPTH, err);
        }
    }
    total
}

fn knots(solid: &Solid, axis: usize, lo: f64, hi: f64) -> Vec<f64> {
    let mut leaves = Vec::new();
    solid.leaves(&mut leaves);
    let mut k: Vec<f64> = leaves.iter().flat_map(|p| {
        let b = p.bounds();
        [b.min[axis], b.max[axis]]
    }).filter(|v| *v > lo && *v < hi).chain([lo, hi]).collect();
    k.sort_by(f64::total_cmp);
    k.dedup();
    k
}

fn numeric(solid: &Solid) -> SolidVolume {
    let b = solid.bounds();
    let ext = [0, 1, 2].map(|i| b.max[i] - b.min[i]);
    if ext.iter().any(|e| !(*e > 0.0)) {
        return SolidVolume { volume_m3: 0.0, error_m3: 0.0, exact: true };
    }
    let (kx, ky) = (knots(solid, 0, b.min[0], b.max[0]), knots(solid, 1, b.min[1], b.max[1]));
    let inner_error = Cell::new(0.0f64);
    let tol = SIMPSON_RELATIVE_TOLERANCE * ext[0] * ext[1] * ext[2];
    let mut outer_error = 0.0;
    let mut column = |y: f64| {
        let mut err = 0.0;
        let area = integrate(&mut |x: f64| solid.spans(x, y).iter().map(|s| s.1 - s.0).sum::<f64>(), &kx, tol / ext[1], &mut err);
        inner_error.set(inner_error.get().max(err));
        area
    };
    let volume_m3 = integrate(&mut column, &ky, tol, &mut outer_error);
    SolidVolume { volume_m3, error_m3: outer_error + inner_error.get() * ext[1], exact: false }
}

/// 📦️ Exact world bounds of the tree (rotations applied; union = hull, intersection = overlap, difference = minuend).
pub fn bounding_box(node: &GeometryNode, catalogue: &GeometryCatalogue) -> Result<BoundingBox, SolidError> {
    build(node, catalogue, IDENTITY, [0.0; 3], 0).map(|s| s.bounds())
}

/// 🧊️ Solid volume of the tree in cubic metres.
pub fn solid_volume(node: &GeometryNode, catalogue: &GeometryCatalogue) -> Result<SolidVolume, SolidError> {
    let solid = build(node, catalogue, IDENTITY, [0.0; 3], 0)?;
    Ok(match solid.exact() {
        Some(volume_m3) => SolidVolume { volume_m3, error_m3: 0.0, exact: true },
        None => numeric(&solid),
    })
}

/// 🧱️ Every primitive of the tree in its world frame, rotations and translations applied, references resolved.
pub fn placed_primitives(node: &GeometryNode, catalogue: &GeometryCatalogue) -> Result<Vec<Primitive>, SolidError> {
    let solid = build(node, catalogue, IDENTITY, [0.0; 3], 0)?;
    let mut found = Vec::new();
    solid.leaves(&mut found);
    Ok(found.into_iter().copied().collect())
}
