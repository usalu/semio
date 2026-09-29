//! 📤️ `s.stdio.semio/v1/drawing` → `png` (1.2) — this repository's own anti-aliased vector
//! rasterizer, so every domain artifact that projects into a drawing gets a real raster export.
//!
//! Painting model (the same one the sibling svg leaf writes, so an svg renderer is a usable oracle):
//! visible layers in order, each node painted source-over into a premultiplied canvas; a `Path`
//! fills with the non-zero rule (every subpath implicitly closed) and then strokes (butt caps,
//! miter joins, SVG's default miter limit 4); a node without a style fills black and does not
//! stroke, exactly SVG's defaults; `opacity` multiplies both paints. Coverage is exact signed-area
//! accumulation per pixel, clamped to one, so edges are anti-aliased without supersampling.
//!
//! 🔖 Documented lossiness (`IoFidelity::Lossy`): `Text` nodes are not painted (no font program is
//! carried by the drawing); `Image` nodes are painted only for `image/png` payloads, nearest
//! sampled; the raster is `canvas` units at one pixel each, uniformly shrunk so neither edge
//! exceeds [`DRAWING_RASTER_MAX_EDGE`].
//!
//! @see https://www.w3.org/TR/SVG11/painting.html
//! @see https://www.w3.org/TR/SVG11/implnote.html#ArcImplementationNotes

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_png::schema::snapshot::{PngChunkMarker, PngColorType};
use semio_s_artifact_stdio_png::PngSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 📏️ Largest raster edge in pixels; larger canvases are scaled down uniformly.
pub const DRAWING_RASTER_MAX_EDGE: f64 = 4096.0;
const SVG_DEFAULT_MITER_LIMIT: f64 = 4.0;

//#region 🔖️Raster
/// 🖼️ One straight (non-premultiplied) RGBA8 raster, row-major, top row first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemioRaster {
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
}

type Affine = [f64; 6];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn compose(outer: &Affine, inner: &Affine) -> Affine {
    let [a, b, c, d, e, f] = *outer;
    let [a2, b2, c2, d2, e2, f2] = *inner;
    [a * a2 + c * b2, b * a2 + d * b2, a * c2 + c * d2, b * c2 + d * d2, a * e2 + c * f2 + e, b * e2 + d * f2 + f]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply(m: &Affine, p: [f64; 2]) -> [f64; 2] {
    [m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5]]
}

/// 🧭️ The 2D affine matrix a `Group`'s transform denotes — identical to the svg leaf's `matrix(...)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_transform_affine(t: &SemioTransform) -> [f64; 6] {
    let theta = 2.0 * t.rotation.z.atan2(t.rotation.w);
    let (sin, cos) = theta.sin_cos();
    [cos * t.scale.x, sin * t.scale.x, -sin * t.scale.y, cos * t.scale.y, t.translation.x, t.translation.y]
}

struct Canvas {
    width: usize,
    height: usize,
    premultiplied: Vec<[f32; 4]>,
    accumulation: Vec<f32>,
    dirty: Option<[usize; 4]>,
}

impl Canvas {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(width: usize, height: usize, background: Option<&SemioRgba>) -> Self {
        let fill = background.map_or([0.0; 4], |c| [c.r * c.a, c.g * c.a, c.b * c.a, c.a]);
        Self { width, height, premultiplied: vec![fill; width * height], accumulation: vec![0.0; (width + 2) * height], dirty: None }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edge(&mut self, from: [f64; 2], to: [f64; 2]) {
        let stride = self.width + 2;
        let right = self.width as f32;
        let (p0, p1) = ([from[0] as f32, from[1] as f32], [to[0] as f32, to[1] as f32]);
        if (p0[1] - p1[1]).abs() <= f32::EPSILON || !(p0[0].is_finite() && p0[1].is_finite() && p1[0].is_finite() && p1[1].is_finite()) {
            return;
        }
        let (direction, top, bottom) = if p0[1] < p1[1] { (1.0f32, p0, p1) } else { (-1.0f32, p1, p0) };
        let dxdy = (bottom[0] - top[0]) / (bottom[1] - top[1]);
        let first_row = top[1].max(0.0).floor() as usize;
        let last_row = (bottom[1].ceil().max(0.0) as usize).min(self.height);
        for row in first_row..last_row {
            let row_top = (row as f32).max(top[1]);
            let row_bottom = ((row + 1) as f32).min(bottom[1]);
            let dy = row_bottom - row_top;
            if dy <= 0.0 {
                continue;
            }
            let x_at_top = (top[0] + (row_top - top[1]) * dxdy).clamp(0.0, right);
            let x_at_bottom = (top[0] + (row_bottom - top[1]) * dxdy).clamp(0.0, right);
            let d = dy * direction;
            let (x0, x1) = if x_at_top < x_at_bottom { (x_at_top, x_at_bottom) } else { (x_at_bottom, x_at_top) };
            let line = row * stride;
            let touched = [row, row, x0.floor() as usize, x1.ceil() as usize + 1];
            self.dirty = Some(self.dirty.map_or(touched, |[r0, r1, c0, c1]| [r0.min(row), r1.max(row), c0.min(touched[2]), c1.max(touched[3])]));
            let x0_floor = x0.floor();
            let x0i = x0_floor as usize;
            let x1_ceil = x1.ceil();
            let x1i = x1_ceil as usize;
            if x1i <= x0i + 1 {
                let middle = 0.5 * (x0 + x1) - x0_floor;
                self.accumulation[line + x0i] += d - d * middle;
                self.accumulation[line + x0i + 1] += d * middle;
            } else {
                let inverse_width = (x1 - x0).recip();
                let x0_fraction = x0 - x0_floor;
                let a0 = 0.5 * inverse_width * (1.0 - x0_fraction) * (1.0 - x0_fraction);
                let x1_fraction = x1 - x1_ceil + 1.0;
                let am = 0.5 * inverse_width * x1_fraction * x1_fraction;
                self.accumulation[line + x0i] += d * a0;
                if x1i == x0i + 2 {
                    self.accumulation[line + x0i + 1] += d * (1.0 - a0 - am);
                } else {
                    let a1 = inverse_width * (1.5 - x0_fraction);
                    self.accumulation[line + x0i + 1] += d * (a1 - a0);
                    for column in x0i + 2..x1i - 1 {
                        self.accumulation[line + column] += d * inverse_width;
                    }
                    let a2 = a1 + (x1i - x0i - 3) as f32 * inverse_width;
                    self.accumulation[line + x1i - 1] += d * (1.0 - a2 - am);
                }
                self.accumulation[line + x1i] += d * am;
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn polygon(&mut self, points: &[[f64; 2]]) {
        for index in 0..points.len() {
            self.edge(points[index], points[(index + 1) % points.len()]);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn composite(&mut self, color: [f32; 4]) {
        let stride = self.width + 2;
        let Some([first_row, last_row, first_column, last_column]) = self.dirty.take() else { return };
        for row in first_row..=last_row {
            let mut winding = 0.0f32;
            for column in first_column..=last_column.min(stride - 1) {
                winding += self.accumulation[row * stride + column];
                self.accumulation[row * stride + column] = 0.0;
                if column >= self.width {
                    continue;
                }
                let alpha = winding.abs().min(1.0) * color[3];
                if alpha <= 0.0 {
                    continue;
                }
                let pixel = &mut self.premultiplied[row * self.width + column];
                for channel in 0..3 {
                    pixel[channel] = color[channel] * alpha + pixel[channel] * (1.0 - alpha);
                }
                pixel[3] = alpha + pixel[3] * (1.0 - alpha);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn blit(&mut self, inverse: &Affine, source: &PngSnapshot, size: [f64; 2], alpha: f32) {
        for row in 0..self.height {
            for column in 0..self.width {
                let local = apply(inverse, [column as f64 + 0.5, row as f64 + 0.5]);
                if local[0] < 0.0 || local[1] < 0.0 || local[0] >= size[0] || local[1] >= size[1] {
                    continue;
                }
                let sx = ((local[0] / size[0]) * source.width as f64) as usize;
                let sy = ((local[1] / size[1]) * source.height as f64) as usize;
                let at = (sy.min(source.height as usize - 1) * source.width as usize + sx.min(source.width as usize - 1)) * 4;
                let Some(texel) = source.pixels.get(at..at + 4) else { continue };
                let a = texel[3] as f32 / 255.0 * alpha;
                let pixel = &mut self.premultiplied[row * self.width + column];
                for channel in 0..3 {
                    pixel[channel] = texel[channel] as f32 / 255.0 * a + pixel[channel] * (1.0 - a);
                }
                pixel[3] = a + pixel[3] * (1.0 - a);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn into_raster(self) -> SemioRaster {
        let mut rgba8 = Vec::with_capacity(self.width * self.height * 4);
        for pixel in &self.premultiplied {
            let alpha = pixel[3].clamp(0.0, 1.0);
            let unpremultiply = |value: f32| if alpha > 0.0 { ((value / alpha).clamp(0.0, 1.0) * 255.0).round() as u8 } else { 0 };
            rgba8.extend_from_slice(&[unpremultiply(pixel[0]), unpremultiply(pixel[1]), unpremultiply(pixel[2]), (alpha * 255.0).round() as u8]);
        }
        SemioRaster { width: self.width as u32, height: self.height as u32, rgba8 }
    }
}
//#endregion 🔖️Raster

//#region 🔖️Flatten
/// ✏️ One flattened subpath in local coordinates.
struct Subpath {
    points: Vec<[f64; 2]>,
    closed: bool,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn steps_for(length_in_pixels: f64) -> usize {
    (length_in_pixels / 2.0).ceil().clamp(1.0, 512.0) as usize
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn distance(a: [f64; 2], b: [f64; 2]) -> f64 {
    ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt()
}

/// 🌙️ An SVG endpoint-parameterized elliptical arc as cubic Béziers `[c1, c2, to]`, one per quarter
/// turn at most (radial error below 0.03 % of the radius), radii scaled up when too small to reach.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn arc_to_cubics(from: [f64; 2], rx: f64, ry: f64, x_rotation_degrees: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Vec<[[f64; 2]; 3]> {
    let (mut rx, mut ry) = (rx.abs(), ry.abs());
    if distance(from, to) == 0.0 {
        return Vec::new();
    }
    if rx == 0.0 || ry == 0.0 {
        return vec![[from, to, to]];
    }
    let phi = x_rotation_degrees.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let dx = (from[0] - to[0]) / 2.0;
    let dy = (from[1] - to[1]) / 2.0;
    let x1 = cos_phi * dx + sin_phi * dy;
    let y1 = -sin_phi * dx + cos_phi * dy;
    let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let numerator = (rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1).max(0.0);
    let denominator = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let root = if denominator > 0.0 { (numerator / denominator).sqrt() } else { 0.0 };
    let sign = if large_arc == sweep { -1.0 } else { 1.0 };
    let cx1 = sign * root * rx * y1 / ry;
    let cy1 = -sign * root * ry * x1 / rx;
    let cx = cos_phi * cx1 - sin_phi * cy1 + (from[0] + to[0]) / 2.0;
    let cy = sin_phi * cx1 + cos_phi * cy1 + (from[1] + to[1]) / 2.0;
    let angle = |ux: f64, uy: f64, vx: f64, vy: f64| {
        let turn = (ux * vy - uy * vx).signum();
        let cosine = ((ux * vx + uy * vy) / ((ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt())).clamp(-1.0, 1.0);
        if turn < 0.0 { -cosine.acos() } else { cosine.acos() }
    };
    let theta1 = angle(1.0, 0.0, (x1 - cx1) / rx, (y1 - cy1) / ry);
    let mut delta = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry);
    if !sweep && delta > 0.0 {
        delta -= std::f64::consts::TAU;
    } else if sweep && delta < 0.0 {
        delta += std::f64::consts::TAU;
    }
    let pieces = (delta.abs() / std::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
    let step = delta / pieces as f64;
    let handle = 4.0 / 3.0 * (step / 4.0).tan();
    let on_ellipse = |theta: f64| {
        let (sin, cos) = theta.sin_cos();
        ([cos_phi * rx * cos - sin_phi * ry * sin + cx, sin_phi * rx * cos + cos_phi * ry * sin + cy], [-cos_phi * rx * sin - sin_phi * ry * cos, -sin_phi * rx * sin + cos_phi * ry * cos])
    };
    (0..pieces)
        .map(|piece| {
            let (a, b) = (theta1 + step * piece as f64, theta1 + step * (piece + 1) as f64);
            let ((p0, d0), (p1, d1)) = (on_ellipse(a), on_ellipse(b));
            let end = if piece + 1 == pieces { to } else { p1 };
            [[p0[0] + handle * d0[0], p0[1] + handle * d0[1]], [p1[0] - handle * d1[0], p1[1] - handle * d1[1]], end]
        })
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_cubic(current: &mut Vec<[f64; 2]>, p0: [f64; 2], p1: [f64; 2], p2: [f64; 2], p3: [f64; 2], pixel_scale: f64) {
    let steps = steps_for((distance(p0, p1) + distance(p1, p2) + distance(p2, p3)) * pixel_scale);
    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let u = 1.0 - t;
        let (w0, w1, w2, w3) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
        current.push([w0 * p0[0] + w1 * p1[0] + w2 * p2[0] + w3 * p3[0], w0 * p0[1] + w1 * p1[1] + w2 * p2[1] + w3 * p3[1]]);
    }
}

/// 🧭️ Composes an outer and an inner affine matrix (`outer · inner`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compose_affine(outer: &[f64; 6], inner: &[f64; 6]) -> [f64; 6] {
    compose(outer, inner)
}

/// 📐️ `segments` mapped through `matrix`: points move, arcs become cubics (an affine image of an
/// ellipse arc is not an SVG arc in general); the identity leaves the path untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn transformed_segments(segments: &[PathSegment], matrix: &[f64; 6]) -> Vec<PathSegment> {
    if *matrix == [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] {
        return segments.to_vec();
    }
    let map = |p: [f64; 2]| {
        let [x, y] = apply(matrix, p);
        SemioPoint2 { x, y }
    };
    let mut pen = [0.0, 0.0];
    let mut start = [0.0, 0.0];
    let mut out = Vec::with_capacity(segments.len());
    for segment in segments {
        match segment {
            PathSegment::MoveTo { to } => {
                pen = [to.x, to.y];
                start = pen;
                out.push(PathSegment::MoveTo { to: map(pen) });
            }
            PathSegment::LineTo { to } => {
                pen = [to.x, to.y];
                out.push(PathSegment::LineTo { to: map(pen) });
            }
            PathSegment::QuadTo { c, to } => {
                pen = [to.x, to.y];
                out.push(PathSegment::QuadTo { c: map([c.x, c.y]), to: map(pen) });
            }
            PathSegment::CubicTo { c1, c2, to } => {
                pen = [to.x, to.y];
                out.push(PathSegment::CubicTo { c1: map([c1.x, c1.y]), c2: map([c2.x, c2.y]), to: map(pen) });
            }
            PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
                out.extend(arc_to_cubics(pen, *rx, *ry, *x_rotation, *large_arc, *sweep, [to.x, to.y]).into_iter().map(|[c1, c2, end]| PathSegment::CubicTo { c1: map(c1), c2: map(c2), to: map(end) }));
                pen = [to.x, to.y];
            }
            PathSegment::Close => {
                pen = start;
                out.push(PathSegment::Close);
            }
        }
    }
    out
}

/// ⭕️ `(centre, radius)` of a path in the two-arc circle normal form
/// `[MoveTo(p), ArcTo(r, r → q), ArcTo(r, r → p), Close]`, the shape every circle-bearing bridge
/// (svg, dxf `CIRCLE`, dwg `Circle`) reads and writes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn circle_normal_form(segments: &[PathSegment]) -> Option<([f64; 2], f64)> {
    match segments {
        [PathSegment::MoveTo { to: p }, PathSegment::ArcTo { rx: r1, ry: q1, to: q, .. }, PathSegment::ArcTo { rx: r2, ry: q2, to: back, .. }, PathSegment::Close]
            if (r1 - q1).abs() < 1e-9 && (r2 - q2).abs() < 1e-9 && (r1 - r2).abs() < 1e-9 && (p.x - back.x).abs() < 1e-9 && (p.y - back.y).abs() < 1e-9 && (distance([p.x, p.y], [q.x, q.y]) - 2.0 * r1).abs() < 1e-9 * r1.max(1.0) =>
        {
            Some(([(p.x + q.x) / 2.0, (p.y + q.y) / 2.0], *r1))
        }
        _ => None,
    }
}

/// 🧭️ `Some(scale)` when `matrix` is a similarity (rotation, uniform scale, translation, no
/// mirror), under which a circle stays a circle of radius `scale · r`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn similarity_scale(matrix: &[f64; 6]) -> Option<f64> {
    let [a, b, c, d, _, _] = *matrix;
    ((a - d).abs() < 1e-12 && (b + c).abs() < 1e-12 && a * d - b * c > 0.0).then(|| (a * d - b * c).sqrt())
}

/// 〰️ A path's subpaths as polylines `(points, closed)` — curves and arcs sampled at about two
/// units of `pixel_scale`-scaled length per point — for consumers that need vertices, not paint.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn flatten_segments(segments: &[PathSegment], pixel_scale: f64) -> Vec<(Vec<[f64; 2]>, bool)> {
    flatten(segments, pixel_scale).into_iter().map(|subpath| (subpath.points, subpath.closed)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn flatten(segments: &[PathSegment], pixel_scale: f64) -> Vec<Subpath> {
    let mut subpaths: Vec<Subpath> = Vec::new();
    let mut current: Vec<[f64; 2]> = Vec::new();
    let mut start = [0.0, 0.0];
    let mut pen = [0.0, 0.0];
    let point = |p: &SemioPoint2| [p.x, p.y];
    let flush = |current: &mut Vec<[f64; 2]>, subpaths: &mut Vec<Subpath>, closed: bool| {
        if current.len() > 1 {
            subpaths.push(Subpath { points: std::mem::take(current), closed });
        } else {
            current.clear();
        }
    };
    for segment in segments {
        match segment {
            PathSegment::MoveTo { to } => {
                flush(&mut current, &mut subpaths, false);
                start = point(to);
                pen = start;
                current.push(pen);
            }
            PathSegment::LineTo { to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                pen = point(to);
                current.push(pen);
            }
            PathSegment::QuadTo { c, to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                let (p0, p1, p2) = (pen, point(c), point(to));
                let steps = steps_for((distance(p0, p1) + distance(p1, p2)) * pixel_scale);
                for step in 1..=steps {
                    let t = step as f64 / steps as f64;
                    let u = 1.0 - t;
                    current.push([u * u * p0[0] + 2.0 * u * t * p1[0] + t * t * p2[0], u * u * p0[1] + 2.0 * u * t * p1[1] + t * t * p2[1]]);
                }
                pen = p2;
            }
            PathSegment::CubicTo { c1, c2, to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                push_cubic(&mut current, pen, point(c1), point(c2), point(to), pixel_scale);
                pen = point(to);
            }
            PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
                if current.is_empty() {
                    current.push(pen);
                }
                let mut from = pen;
                for [c1, c2, end] in arc_to_cubics(pen, *rx, *ry, *x_rotation, *large_arc, *sweep, point(to)) {
                    push_cubic(&mut current, from, c1, c2, end, pixel_scale);
                    from = end;
                }
                pen = point(to);
            }
            PathSegment::Close => {
                flush(&mut current, &mut subpaths, true);
                pen = start;
            }
        }
    }
    flush(&mut current, &mut subpaths, false);
    subpaths
}
//#endregion 🔖️Flatten

//#region 🔖️Stroke
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn positively_wound(mut polygon: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    let area: f64 = (0..polygon.len()).map(|i| {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        a[0] * b[1] - b[0] * a[1]
    }).sum();
    if area < 0.0 {
        polygon.reverse();
    }
    polygon
}

/// 🖊️ The stroke outline of one subpath as positively wound polygons (segment quads plus miter or
/// bevel joins), in the subpath's own coordinates.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stroke_polygons(subpath: &Subpath, half_width: f64) -> Vec<Vec<[f64; 2]>> {
    let mut points: Vec<[f64; 2]> = Vec::with_capacity(subpath.points.len());
    for p in &subpath.points {
        if points.last().is_none_or(|last| distance(*last, *p) > 1e-9) {
            points.push(*p);
        }
    }
    if subpath.closed && points.len() > 2 && distance(points[0], *points.last().expect("non-empty")) <= 1e-9 {
        points.pop();
    }
    if points.len() < 2 {
        return Vec::new();
    }
    let segment_count = if subpath.closed { points.len() } else { points.len() - 1 };
    let direction = |i: usize| {
        let (a, b) = (points[i % points.len()], points[(i + 1) % points.len()]);
        let length = distance(a, b);
        [(b[0] - a[0]) / length, (b[1] - a[1]) / length]
    };
    let mut polygons = Vec::new();
    for i in 0..segment_count {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        let d = direction(i);
        let n = [-d[1] * half_width, d[0] * half_width];
        polygons.push(positively_wound(vec![[a[0] + n[0], a[1] + n[1]], [b[0] + n[0], b[1] + n[1]], [b[0] - n[0], b[1] - n[1]], [a[0] - n[0], a[1] - n[1]]]));
    }
    let joints: Vec<usize> = if subpath.closed { (0..points.len()).collect() } else { (1..points.len() - 1).collect() };
    for joint in joints {
        let incoming = direction((joint + points.len() - 1) % points.len());
        let outgoing = direction(joint);
        let turn = incoming[0] * outgoing[1] - incoming[1] * outgoing[0];
        if turn.abs() < 1e-12 {
            continue;
        }
        let side = if turn > 0.0 { -1.0 } else { 1.0 };
        let v = points[joint];
        let n_in = [-incoming[1] * half_width * side, incoming[0] * half_width * side];
        let n_out = [-outgoing[1] * half_width * side, outgoing[0] * half_width * side];
        let a = [v[0] + n_in[0], v[1] + n_in[1]];
        let b = [v[0] + n_out[0], v[1] + n_out[1]];
        let cos_phi = ((n_in[0] * n_out[0] + n_in[1] * n_out[1]) / (half_width * half_width)).clamp(-1.0, 1.0);
        let cos_half = ((1.0 + cos_phi) / 2.0).sqrt();
        let polygon = if cos_half > 1.0 / SVG_DEFAULT_MITER_LIMIT {
            let bisector = [n_in[0] + n_out[0], n_in[1] + n_out[1]];
            let length = (bisector[0].powi(2) + bisector[1].powi(2)).sqrt();
            let reach = half_width / cos_half;
            vec![v, a, [v[0] + bisector[0] / length * reach, v[1] + bisector[1] / length * reach], b]
        } else {
            vec![v, a, b]
        };
        polygons.push(positively_wound(polygon));
    }
    polygons
}
//#endregion 🔖️Stroke

//#region 🔖️Paint
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn premultiplied_paint(color: &SemioRgba, opacity: f32) -> [f32; 4] {
    [color.r.clamp(0.0, 1.0), color.g.clamp(0.0, 1.0), color.b.clamp(0.0, 1.0), (color.a * opacity).clamp(0.0, 1.0)]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paint_node(canvas: &mut Canvas, node: &DrawNode, styles: &[DrawStyle], matrix: &Affine) {
    match node {
        DrawNode::Group { transform, children } => {
            let inner = compose(matrix, &semio_transform_affine(transform));
            for child in children {
                paint_node(canvas, child, styles, &inner);
            }
        }
        DrawNode::Path { segments, style } => {
            let style = style.as_deref().and_then(|name| styles.iter().find(|s| s.name == name));
            let black = SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
            let (fill, stroke, width, opacity) = match style {
                Some(s) => (s.fill, s.stroke, s.stroke_width.unwrap_or(1.0), s.opacity.unwrap_or(1.0)),
                None => (Some(black), None, 1.0, 1.0),
            };
            let pixel_scale = (matrix[0] * matrix[3] - matrix[1] * matrix[2]).abs().sqrt().max(1e-9);
            let subpaths = flatten(segments, pixel_scale);
            if let Some(fill) = fill {
                for subpath in &subpaths {
                    let transformed: Vec<[f64; 2]> = subpath.points.iter().map(|p| apply(matrix, *p)).collect();
                    canvas.polygon(&transformed);
                }
                canvas.composite(premultiplied_paint(&fill, opacity));
            }
            if let Some(stroke) = stroke.filter(|_| width > 0.0) {
                for subpath in &subpaths {
                    for polygon in stroke_polygons(subpath, width / 2.0) {
                        let transformed: Vec<[f64; 2]> = polygon.iter().map(|p| apply(matrix, *p)).collect();
                        canvas.polygon(&transformed);
                    }
                }
                canvas.composite(premultiplied_paint(&stroke, opacity));
            }
        }
        DrawNode::Image { at, width, height, mime, bytes } => {
            if mime != "image/png" || *width <= 0.0 || *height <= 0.0 {
                return;
            }
            let Ok(source) = semio_s_artifact_stdio_png::io::decode_png(bytes) else { return };
            if source.width == 0 || source.height == 0 {
                return;
            }
            let placed = compose(matrix, &[1.0, 0.0, 0.0, 1.0, at.x, at.y]);
            let determinant = placed[0] * placed[3] - placed[1] * placed[2];
            if determinant.abs() < 1e-12 {
                return;
            }
            let inverse = [placed[3] / determinant, -placed[1] / determinant, -placed[2] / determinant, placed[0] / determinant, (placed[2] * placed[5] - placed[3] * placed[4]) / determinant, (placed[1] * placed[4] - placed[0] * placed[5]) / determinant];
            canvas.blit(&inverse, &source, [*width, *height], 1.0);
        }
        DrawNode::Text { .. } => {}
    }
}

/// 🖼️ Paints every visible layer of `drawing` into an RGBA8 raster (see the module doc for the model).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rasterize_drawing(drawing: &SemioDrawingSnapshot) -> Result<SemioRaster, String> {
    let (width, height) = (drawing.canvas.width, drawing.canvas.height);
    if !(width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0) {
        return Err("semio/drawing→png: the canvas has no positive finite size".into());
    }
    let scale = (DRAWING_RASTER_MAX_EDGE / width.max(height)).min(1.0);
    let (columns, rows) = ((width * scale).ceil().max(1.0) as usize, (height * scale).ceil().max(1.0) as usize);
    let mut canvas = Canvas::new(columns, rows, drawing.canvas.background.as_ref());
    let base = [scale, 0.0, 0.0, scale, 0.0, 0.0];
    for layer in drawing.layers.iter().filter(|layer| layer.visible) {
        paint_node(&mut canvas, &layer.root, &drawing.styles, &base);
    }
    Ok(canvas.into_raster())
}
//#endregion 🔖️Paint

//#region 🔖️Serializer
pub struct SemioDrawingToPng;

impl ArtifactSerializer for SemioDrawingToPng {
    type From = SemioDrawingSnapshot;
    type Into = PngSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let raster = rasterize_drawing(from).map_err(store::PackError::Schema)?;
        Ok(PngSnapshot {
            schema: semio_s_artifact_stdio_png::STDIO_PNG_DOCUMENT_SCHEMA.into(),
            width: raster.width,
            height: raster.height,
            bit_depth: 8,
            color_type: PngColorType::Rgba,
            interlace: false,
            pixels: raster.rgba8,
            chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Idat, PngChunkMarker::Iend],
            ..PngSnapshot::default()
        })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
