//! 🧮️ Neutral triangle meshes of the viewport lowering: unit prisms from section outlines, cones, grids and ear-clipped caps.

use super::pose::*;
use super::scene::{Hatch, Polygon2, Vec2};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshData {
    pub positions: Vec<f64>,
    pub normals: Vec<f64>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn vertex(&mut self, p: V3, n: V3) -> u32 {
        self.positions.extend_from_slice(&p);
        self.normals.extend_from_slice(&n);
        (self.positions.len() / 3 - 1) as u32
    }

    pub fn tri(&mut self, a: u32, b: u32, c: u32) {
        self.indices.extend_from_slice(&[a, b, c]);
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn point(&self, i: usize) -> V3 {
        [self.positions[3 * i], self.positions[3 * i + 1], self.positions[3 * i + 2]]
    }

    pub fn flat_triangle(&mut self, a: V3, b: V3, c: V3) {
        let n = unit(cross(sub(b, a), sub(c, a)));
        let i = self.vertex(a, n);
        self.vertex(b, n);
        self.vertex(c, n);
        self.tri(i, i + 1, i + 2);
    }

    /// 🧊️ Signed enclosed volume by the divergence theorem; positive for outward-wound closed meshes.
    pub fn volume(&self) -> f64 {
        self.indices.chunks(3).map(|t| dot(self.point(t[0] as usize), cross(self.point(t[1] as usize), self.point(t[2] as usize))) / 6.0).sum()
    }

    pub fn surface_area(&self) -> f64 {
        self.indices.chunks(3).map(|t| len(cross(sub(self.point(t[1] as usize), self.point(t[0] as usize)), sub(self.point(t[2] as usize), self.point(t[0] as usize)))) / 2.0).sum()
    }
}

fn cross2(o: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
}

fn ring_points(points: &[Vec2], counter_clockwise: bool) -> Vec<[f64; 2]> {
    let mut ring: Vec<[f64; 2]> = points.iter().map(|p| [p.x, p.y]).collect();
    let area: f64 = (0..ring.len()).map(|i| ring[i][0] * ring[(i + 1) % ring.len()][1] - ring[(i + 1) % ring.len()][0] * ring[i][1]).sum();
    if (area > 0.0) != counter_clockwise { ring.reverse(); }
    ring
}

fn segments_cross(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    let (d1, d2, d3, d4) = (cross2(a, b, c), cross2(a, b, d), cross2(c, d, a), cross2(c, d, b));
    ((d1 > 1e-14 && d2 < -1e-14) || (d1 < -1e-14 && d2 > 1e-14)) && ((d3 > 1e-14 && d4 < -1e-14) || (d3 < -1e-14 && d4 > 1e-14))
}

fn visible(from: [f64; 2], to: [f64; 2], rings: &[&[[f64; 2]]]) -> bool {
    rings.iter().all(|ring| (0..ring.len()).all(|i| {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        a == from || b == from || a == to || b == to || !segments_cross(from, to, a, b)
    }))
}

fn merge_holes(outer: Vec<[f64; 2]>, mut holes: Vec<Vec<[f64; 2]>>) -> Vec<[f64; 2]> {
    holes.sort_by(|a, b| b.iter().map(|p| p[0]).fold(f64::MIN, f64::max).total_cmp(&a.iter().map(|p| p[0]).fold(f64::MIN, f64::max)));
    let mut poly = outer;
    for (n, hole) in holes.iter().enumerate() {
        let h = (0..hole.len()).max_by(|&a, &b| hole[a][0].total_cmp(&hole[b][0])).unwrap_or(0);
        let mut order: Vec<usize> = (0..poly.len()).collect();
        order.sort_by(|&a, &b| ((poly[a][0] - hole[h][0]).powi(2) + (poly[a][1] - hole[h][1]).powi(2)).total_cmp(&((poly[b][0] - hole[h][0]).powi(2) + (poly[b][1] - hole[h][1]).powi(2))));
        let rest: Vec<&[[f64; 2]]> = holes[n + 1..].iter().map(Vec::as_slice).collect();
        let mut rings: Vec<&[[f64; 2]]> = vec![&poly[..], &hole[..]];
        rings.extend(rest);
        let bridge = order.iter().copied().find(|&c| visible(hole[h], poly[c], &rings)).unwrap_or(order[0]);
        let mut merged: Vec<[f64; 2]> = poly[..=bridge].to_vec();
        merged.extend((0..hole.len()).map(|k| hole[(h + k) % hole.len()]));
        merged.push(hole[h]);
        merged.extend_from_slice(&poly[bridge..]);
        poly = merged;
    }
    poly
}

fn inside(p: [f64; 2], a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> bool {
    cross2(a, b, p) > 1e-14 && cross2(b, c, p) > 1e-14 && cross2(c, a, p) > 1e-14
}

/// 🔺️ Ear-clipped triangulation of a polygon with holes (holes bridged into the outer ring); returns the merged vertex list and index triples, all counter-clockwise.
pub fn triangulate(polygon: &Polygon2) -> (Vec<[f64; 2]>, Vec<[usize; 3]>) {
    let outer = ring_points(&polygon.outer.points, true);
    let holes: Vec<Vec<[f64; 2]>> = polygon.holes.iter().map(|h| ring_points(&h.points, false)).collect();
    let points = merge_holes(outer, holes);
    let mut remaining: Vec<usize> = (0..points.len()).collect();
    let mut triangles = Vec::new();
    while remaining.len() > 3 {
        let n = remaining.len();
        let ear = (0..n).find(|&i| {
            let (a, b, c) = (points[remaining[(i + n - 1) % n]], points[remaining[i]], points[remaining[(i + 1) % n]]);
            cross2(a, b, c) > 1e-14 && remaining.iter().all(|&k| { let p = points[k]; p == a || p == b || p == c || !inside(p, a, b, c) })
        }).unwrap_or(0);
        triangles.push([remaining[(ear + n - 1) % n], remaining[ear], remaining[(ear + 1) % n]]);
        remaining.remove(ear);
    }
    if remaining.len() == 3 && cross2(points[remaining[0]], points[remaining[1]], points[remaining[2]]).abs() > 1e-14 {
        triangles.push([remaining[0], remaining[1], remaining[2]]);
    }
    (points, triangles)
}

fn decimated(points: &[Vec2], stride: usize) -> Vec<Vec2> {
    if stride <= 1 || points.len() < 16 { points.to_vec() } else { points.iter().step_by(stride).cloned().collect() }
}

/// 🧊️ Prism of a planar region extruded from z = 0 to z = 1 with flat side normals; `stride` thins rings of 16 or more vertices for coarse levels of detail.
pub fn prism(polygon: &Polygon2, stride: usize) -> MeshData {
    let thinned = Polygon2 { outer: super::scene::Ring2 { points: decimated(&polygon.outer.points, stride) }, holes: polygon.holes.iter().map(|h| super::scene::Ring2 { points: decimated(&h.points, stride) }).collect() };
    let mut mesh = MeshData::default();
    let (points, triangles) = triangulate(&thinned);
    for (z, nz) in [(0.0, -1.0), (1.0, 1.0)] {
        let base = mesh.vertex_count() as u32;
        for p in &points { mesh.vertex([p[0], p[1], z], [0.0, 0.0, nz]); }
        for t in &triangles {
            if nz > 0.0 { mesh.tri(base + t[0] as u32, base + t[1] as u32, base + t[2] as u32) } else { mesh.tri(base + t[0] as u32, base + t[2] as u32, base + t[1] as u32) }
        }
    }
    let outer = ring_points(&thinned.outer.points, true);
    let holes: Vec<Vec<[f64; 2]>> = thinned.holes.iter().map(|h| ring_points(&h.points, false)).collect();
    for ring in std::iter::once(&outer).chain(holes.iter()) {
        for i in 0..ring.len() {
            let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
            let n = unit([b[1] - a[1], a[0] - b[0], 0.0]);
            let base = mesh.vertex(  [a[0], a[1], 0.0], n);
            mesh.vertex([b[0], b[1], 0.0], n);
            mesh.vertex([b[0], b[1], 1.0], n);
            mesh.vertex([a[0], a[1], 1.0], n);
            mesh.tri(base, base + 1, base + 2);
            mesh.tri(base, base + 2, base + 3);
        }
    }
    mesh
}

pub fn unit_box() -> MeshData {
    prism(&Polygon2::solid(vec![Vec2::new(-0.5, -0.5), Vec2::new(0.5, -0.5), Vec2::new(0.5, 0.5), Vec2::new(-0.5, 0.5)]), 1)
}

pub fn unit_cylinder(segments: usize) -> MeshData {
    prism(&Polygon2::solid((0..segments).map(|i| { let a = std::f64::consts::TAU * i as f64 / segments as f64; Vec2::new(0.5 * a.cos(), 0.5 * a.sin()) }).collect()), 1)
}

/// 🔺️ Cone with base radius 0.5 at z = 0 and apex at z = 1.
pub fn unit_cone(segments: usize) -> MeshData {
    let mut mesh = MeshData::default();
    let ring = |i: usize| { let a = std::f64::consts::TAU * i as f64 / segments as f64; [0.5 * a.cos(), 0.5 * a.sin()] };
    for i in 0..segments {
        let (a, b) = (ring(i), ring((i + 1) % segments));
        mesh.flat_triangle([a[0], a[1], 0.0], [b[0], b[1], 0.0], [0.0, 0.0, 1.0]);
        mesh.flat_triangle([0.0, 0.0, 0.0], [b[0], b[1], 0.0], [a[0], a[1], 0.0]);
    }
    mesh
}

/// 🕸️ Row-major point grid as two-sided triangles.
pub fn grid(rows: usize, columns: usize, points: &[V3]) -> MeshData {
    let mut mesh = MeshData::default();
    for r in 0..rows.saturating_sub(1) {
        for c in 0..columns.saturating_sub(1) {
            let (a, b, d, e) = (points[r * columns + c], points[r * columns + c + 1], points[(r + 1) * columns + c], points[(r + 1) * columns + c + 1]);
            mesh.flat_triangle(a, b, e);
            mesh.flat_triangle(a, e, d);
            mesh.flat_triangle(a, e, b);
            mesh.flat_triangle(a, d, e);
        }
    }
    mesh
}

pub fn soup(vertices: &[V3], triangles: &[[u32; 3]]) -> MeshData {
    let mut mesh = MeshData::default();
    for t in triangles { mesh.flat_triangle(vertices[t[0] as usize], vertices[t[1] as usize], vertices[t[2] as usize]); }
    mesh
}

/// 🟫️ Flat two-sided cap of a spatial polygon, triangulated in its own plane.
pub fn flat_polygon(points: &[V3]) -> MeshData {
    let mut mesh = MeshData::default();
    if points.len() < 3 { return mesh; }
    let normal = unit(points.iter().enumerate().fold([0.0; 3], |n, (i, p)| add(n, cross(*p, points[(i + 1) % points.len()]))));
    let u = orthogonal(normal, [1.0, 0.0, 0.0]);
    let v = cross(normal, u);
    let planar = Polygon2::solid(points.iter().map(|p| Vec2::new(dot(*p, u), dot(*p, v))).collect());
    let (flat, triangles) = triangulate(&planar);
    let origin = points[0];
    let lift = |q: [f64; 2]| add(origin, add(mul(u, q[0] - dot(origin, u)), mul(v, q[1] - dot(origin, v))));
    for t in &triangles {
        let (a, b, c) = (lift(flat[t[0]]), lift(flat[t[1]]), lift(flat[t[2]]));
        mesh.flat_triangle(a, b, c);
        mesh.flat_triangle(a, c, b);
    }
    mesh
}

/// 🌐️ UV sphere of radius 0.5 centred on the origin.
pub fn unit_sphere(segments: usize) -> MeshData {
    let (rows, columns) = (segments / 2 + 1, segments + 1);
    let points: Vec<V3> = (0..rows).flat_map(|r| (0..columns).map(move |c| {
        let (phi, theta) = (std::f64::consts::PI * r as f64 / (rows - 1) as f64, std::f64::consts::TAU * c as f64 / segments as f64);
        [0.5 * phi.sin() * theta.cos(), 0.5 * phi.sin() * theta.sin(), 0.5 * phi.cos()]
    })).collect();
    grid(rows, columns, &points)
}

fn scan_segments(rings: &[Vec<[f64; 2]>], theta: f64, spacing: f64) -> Vec<([f64; 2], [f64; 2])> {
    let (c, s) = (theta.cos(), theta.sin());
    let turned: Vec<Vec<[f64; 2]>> = rings.iter().map(|r| r.iter().map(|p| [p[0] * c + p[1] * s, -p[0] * s + p[1] * c]).collect()).collect();
    let (lo, hi) = turned.iter().flatten().fold((f64::MAX, f64::MIN), |(l, h), p| (l.min(p[1]), h.max(p[1])));
    let mut out = Vec::new();
    if !(hi > lo) || spacing <= 0.0 { return out; }
    let first = (lo / spacing).floor() as i64 + 1;
    for k in first..=((hi / spacing).ceil() as i64).min(first + 400) {
        let y = k as f64 * spacing;
        let mut xs: Vec<f64> = turned.iter().flat_map(|r| (0..r.len()).filter_map(move |i| {
            let (a, b) = (r[i], r[(i + 1) % r.len()]);
            ((a[1] <= y) != (b[1] <= y)).then(|| a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
        })).collect();
        xs.sort_by(f64::total_cmp);
        for pair in xs.chunks_exact(2) {
            let back = |x: f64| [x * c - y * s, x * s + y * c];
            out.push((back(pair[0]), back(pair[1])));
        }
    }
    out
}

/// 🏁️ Two-sided ribbon mesh of a hatch pattern clipped to a planar region (z = 0, even-odd over outer ring and holes): `angle` is the direction of `Horizontal` lines from the region's x axis, `Vertical` runs across it, `Diagonal` and `CrossDiagonal` at 45 degrees to it, `Dots` are squares on the `Horizontal` scan lines; lines are `width` wide at `spacing` apart, anchored at the region origin so equal regions hatch equally.
pub fn hatch_ribbons(polygon: &Polygon2, hatch: Hatch, angle: f64, spacing: f64, width: f64) -> MeshData {
    let rings: Vec<Vec<[f64; 2]>> = std::iter::once(&polygon.outer).chain(polygon.holes.iter()).map(|r| r.points.iter().map(|p| [p.x, p.y]).collect()).collect();
    let quarter = std::f64::consts::FRAC_PI_4;
    let (angles, dotted): (Vec<f64>, bool) = match hatch {
        Hatch::Horizontal => (vec![angle], false),
        Hatch::Vertical => (vec![angle + 2.0 * quarter], false),
        Hatch::Diagonal => (vec![angle + quarter], false),
        Hatch::CrossDiagonal => (vec![angle + quarter, angle - quarter], false),
        Hatch::Dots => (vec![angle], true),
    };
    let mut mesh = MeshData::default();
    let quad = |mesh: &mut MeshData, a: [f64; 2], b: [f64; 2]| {
        let d = [b[0] - a[0], b[1] - a[1]];
        let l = (d[0] * d[0] + d[1] * d[1]).sqrt().max(1e-12);
        let n = [-d[1] / l * width / 2.0, d[0] / l * width / 2.0];
        let q = [[a[0] + n[0], a[1] + n[1], 0.0], [b[0] + n[0], b[1] + n[1], 0.0], [b[0] - n[0], b[1] - n[1], 0.0], [a[0] - n[0], a[1] - n[1], 0.0]];
        for (i, j, k) in [(0, 1, 2), (0, 2, 3), (0, 2, 1), (0, 3, 2)] { mesh.flat_triangle(q[i], q[j], q[k]); }
    };
    for theta in angles {
        for (a, b) in scan_segments(&rings, theta, spacing) {
            if !dotted { quad(&mut mesh, a, b); continue; }
            let (d, l) = ([b[0] - a[0], b[1] - a[1]], ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt());
            let dot = width * 1.5;
            let mut at = spacing / 2.0;
            while at < l - dot / 2.0 {
                let (c, u) = ([a[0] + d[0] * at / l, a[1] + d[1] * at / l], [d[0] / l * dot / 2.0, d[1] / l * dot / 2.0]);
                quad(&mut mesh, [c[0] - u[0], c[1] - u[1]], [c[0] + u[0], c[1] + u[1]]);
                at += spacing;
            }
        }
    }
    mesh
}
