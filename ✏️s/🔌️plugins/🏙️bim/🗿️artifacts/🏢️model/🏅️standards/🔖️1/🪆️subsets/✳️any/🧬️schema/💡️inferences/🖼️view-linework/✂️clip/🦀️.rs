//! ✂️ Clipping of the view pipeline: convex polygons against half-spaces in 3D (the slab a view looks into), and rings, segments and points against the rectangle of a crop in 2D.

use super::frame::Frame;
use semio_framework_geometry::vector::{dot3, Xyz};

/// 📐️ The half-space of the points `p` with `dot(p, normal) >= offset`.
#[derive(Clone, Copy, Debug)]
pub struct Half {
    pub normal: Xyz,
    pub offset: f64,
}

/// 🔭️ The four half-spaces of the slab a frame looks into: along the plane within its length, behind it up to `depth`. A depth beyond a million metres is no limit.
pub fn slab(frame: &Frame, depth: f64) -> Vec<Half> {
    let (a, l) = ([frame.along[0], frame.along[1], 0.0], frame.look3());
    let mut halves = vec![Half { normal: a, offset: dot3(a, [frame.origin[0], frame.origin[1], 0.0]) }, Half { normal: [-a[0], -a[1], 0.0], offset: -dot3(a, [frame.origin[0], frame.origin[1], 0.0]) - frame.length }, Half { normal: l, offset: dot3(l, [frame.origin[0], frame.origin[1], 0.0]) }];
    if depth < 1e6 {
        halves.push(Half { normal: [-l[0], -l[1], 0.0], offset: -dot3(l, [frame.origin[0], frame.origin[1], 0.0]) - depth });
    }
    halves
}

fn lerp(a: Xyz, b: Xyz, t: f64) -> Xyz {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// ✂️ The part of a convex polygon inside `half` (Sutherland and Hodgman).
pub fn clip_polygon(polygon: &[Xyz], half: &Half) -> Vec<Xyz> {
    let mut out = Vec::with_capacity(polygon.len() + 1);
    for index in 0..polygon.len() {
        let (a, b) = (polygon[index], polygon[(index + 1) % polygon.len()]);
        let (da, db) = (dot3(a, half.normal) - half.offset, dot3(b, half.normal) - half.offset);
        if da >= 0.0 {
            out.push(a);
        }
        if (da >= 0.0) != (db >= 0.0) {
            out.push(lerp(a, b, da / (da - db)));
        }
    }
    out
}

/// ✂️ The triangles a triangle leaves inside every half-space, fanned from the clipped polygon.
pub fn clip_triangle(triangle: [Xyz; 3], halves: &[Half]) -> Vec<[Xyz; 3]> {
    let mut polygon = triangle.to_vec();
    for half in halves {
        polygon = clip_polygon(&polygon, half);
        if polygon.len() < 3 {
            return Vec::new();
        }
    }
    (1..polygon.len() - 1).map(|index| [polygon[0], polygon[index], polygon[index + 1]]).collect()
}

/// ▭️ An axis-aligned rectangle of the drawing plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl Rect {
    /// ▭️ Whether the point lies inside or on the rectangle.
    pub fn contains(&self, point: [f64; 2]) -> bool {
        point[0] >= self.x0 && point[0] <= self.x1 && point[1] >= self.y0 && point[1] <= self.y1
    }

    /// ▭️ Whether the box `[x0, x1] x [y0, y1]` touches the rectangle.
    pub fn meets(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
        x1 >= self.x0 && x0 <= self.x1 && y1 >= self.y0 && y0 <= self.y1
    }
}

fn clip_ring_side(ring: &[[f64; 2]], keep: impl Fn([f64; 2]) -> f64) -> Vec<[f64; 2]> {
    let mut out = Vec::with_capacity(ring.len() + 1);
    for index in 0..ring.len() {
        let (a, b) = (ring[index], ring[(index + 1) % ring.len()]);
        let (da, db) = (keep(a), keep(b));
        if da >= 0.0 {
            out.push(a);
        }
        if (da >= 0.0) != (db >= 0.0) {
            let t = da / (da - db);
            out.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]);
        }
    }
    out
}

/// ✂️ The part of a closed ring inside the rectangle, empty when none of it is.
pub fn clip_ring(ring: &[[f64; 2]], rect: &Rect) -> Vec<[f64; 2]> {
    let mut out = clip_ring_side(ring, |p| p[0] - rect.x0);
    for keep in [Box::new(|p: [f64; 2]| rect.x1 - p[0]) as Box<dyn Fn([f64; 2]) -> f64>, Box::new(|p: [f64; 2]| p[1] - rect.y0), Box::new(|p: [f64; 2]| rect.y1 - p[1])] {
        if out.len() < 3 {
            return Vec::new();
        }
        out = clip_ring_side(&out, keep);
    }
    if out.len() < 3 {
        Vec::new()
    } else {
        out
    }
}

/// ✂️ The part of a segment inside the rectangle (Liang and Barsky), none when it misses.
pub fn clip_segment(a: [f64; 2], b: [f64; 2], rect: &Rect) -> Option<([f64; 2], [f64; 2])> {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for (p, q) in [(-dx, a[0] - rect.x0), (dx, rect.x1 - a[0]), (-dy, a[1] - rect.y0), (dy, rect.y1 - a[1])] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
        } else {
            let t = q / p;
            if p < 0.0 {
                lo = lo.max(t);
            } else {
                hi = hi.min(t);
            }
        }
    }
    (lo <= hi).then(|| ([a[0] + dx * lo, a[1] + dy * lo], [a[0] + dx * hi, a[1] + dy * hi]))
}
