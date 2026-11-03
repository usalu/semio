//! 🌐️ Shells and plates: solids of revolution, offset patches, plates with stiffeners, silo and tank generatrices, and double-sided surfaces.

use super::extrude::{prepare, prism, stations, sweep, Prepared};
use super::section_outline::regions;
use super::triangulate::{oriented, triangulate};
use super::vector::*;
use super::{segments, Lod, MeshBuilder, MeshData, Tolerance};
use crate::{PlateGeometry, PointGrid, Polyline2, ShellGeometry, ShellShape, SurfaceGeometry, SurfaceShape, Validate, Vec2, Vec3, Violation};

const TAU: f64 = std::f64::consts::TAU;

/// 🌀️ Revolves the closed `profile` (radius, axial) about the `ez` axis through `origin` by `sweep` radians; partial sweeps get flat end caps.
pub fn revolve(b: &mut MeshBuilder, profile: &[Q], origin: P, ex: P, ey: P, ez: P, sweep: f64, tol: Tolerance, lod: Lod) {
    let ring = oriented(profile, true);
    let rmax = ring.iter().map(|c| c[0].abs()).fold(1e-6, f64::max);
    let n = segments(rmax, sweep, tol, lod);
    let at = |c: Q, phi: f64| add(origin, add(add(mul(ex, c[0] * phi.cos()), mul(ey, c[0] * phi.sin())), mul(ez, c[1])));
    for k in 0..n {
        let (p0, p1) = (sweep * k as f64 / n as f64, sweep * (k + 1) as f64 / n as f64);
        let mid = (p0 + p1) / 2.0;
        let radial = add(mul(ex, mid.cos()), mul(ey, mid.sin()));
        for i in 0..ring.len() {
            let (a, c) = (ring[i], ring[(i + 1) % ring.len()]);
            let hint = add(mul(radial, c[1] - a[1]), mul(ez, -(c[0] - a[0])));
            b.quad_toward([at(a, p0), at(c, p0), at(c, p1), at(a, p1)], hint);
        }
    }
    if sweep.abs() < TAU - 1e-9 {
        let (pts, tris) = triangulate(&ring, &[]);
        let tangent = |phi: f64| mul(add(mul(ex, -phi.sin()), mul(ey, phi.cos())), sweep.signum());
        for (phi, sign) in [(0.0, -1.0), (sweep, 1.0)] {
            for t in &tris {
                b.tri_toward(at(pts[t[0]], phi), at(pts[t[1]], phi), at(pts[t[2]], phi), mul(tangent(phi), sign));
            }
        }
    }
}

fn offset_band(line: &[Q], thickness: f64) -> Vec<Q> {
    let n = line.len();
    let normal = |i: usize| {
        let d = [line[i + 1][0] - line[i][0], line[i + 1][1] - line[i][1]];
        let l = d[0].hypot(d[1]).max(1e-300);
        [-d[1] / l, d[0] / l]
    };
    let shift = |i: usize, sign: f64| {
        let m = match i {
            0 => normal(0),
            i if i == n - 1 => normal(n - 2),
            _ => {
                let (a, b) = (normal(i - 1), normal(i));
                let s = [a[0] + b[0], a[1] + b[1]];
                let l = s[0].hypot(s[1]).max(1e-300);
                let m = [s[0] / l, s[1] / l];
                let k = 1.0 / (m[0] * b[0] + m[1] * b[1]).max(0.2);
                [m[0] * k, m[1] * k]
            }
        };
        [(line[i][0] + sign * m[0] * thickness / 2.0).max(0.0), line[i][1] + sign * m[1] * thickness / 2.0]
    };
    (0..n).map(|i| shift(i, 1.0)).chain((0..n).rev().map(|i| shift(i, -1.0))).collect()
}

fn grid_normals(g: &PointGrid) -> Vec<P> {
    let (r, c) = (g.rows as usize, g.columns as usize);
    let pt = |i: usize, j: usize| p(&g.points[i * c + j]);
    (0..r * c).map(|k| {
        let (i, j) = (k / c, k % c);
        let dr = sub(pt((i + 1).min(r - 1), j), pt(i.saturating_sub(1), j));
        let dc = sub(pt(i, (j + 1).min(c - 1)), pt(i, j.saturating_sub(1)));
        unit(cross(dc, dr))
    }).collect()
}

fn patch(b: &mut MeshBuilder, g: &PointGrid, half: f64) {
    let (r, c) = (g.rows as usize, g.columns as usize);
    let normals = grid_normals(g);
    let at = |i: usize, j: usize, side: f64| add(p(&g.points[i * c + j]), mul(normals[i * c + j], side * half));
    let centre = mul((0..r * c).fold([0.0; 3], |a, k| add(a, p(&g.points[k]))), 1.0 / (r * c) as f64);
    for i in 0..r - 1 {
        for j in 0..c - 1 {
            let h = normals[i * c + j];
            b.quad_toward([at(i, j, 1.0), at(i, j + 1, 1.0), at(i + 1, j + 1, 1.0), at(i + 1, j, 1.0)], h);
            b.quad_toward([at(i, j, -1.0), at(i, j + 1, -1.0), at(i + 1, j + 1, -1.0), at(i + 1, j, -1.0)], mul(h, -1.0));
        }
    }
    if half > 0.0 {
        let edge = |a: (usize, usize), z: (usize, usize), b2: &mut MeshBuilder| {
            let mid = lerp(p(&g.points[a.0 * c + a.1]), p(&g.points[z.0 * c + z.1]), 0.5);
            b2.quad_toward([at(a.0, a.1, 1.0), at(z.0, z.1, 1.0), at(z.0, z.1, -1.0), at(a.0, a.1, -1.0)], sub(mid, centre));
        };
        (0..c - 1).for_each(|j| {
            edge((0, j), (0, j + 1), b);
            edge((r - 1, j), (r - 1, j + 1), b);
        });
        (0..r - 1).for_each(|i| {
            edge((i, 0), (i + 1, 0), b);
            edge((i, c - 1), (i + 1, c - 1), b);
        });
    }
}

/// 🥣️ Shell mesh: a generatrix band revolved about its axis, or an offset surface patch; thickness is real.
pub fn shell_mesh(g: &ShellGeometry, tol: Tolerance, lod: Lod) -> Result<MeshData, Vec<Violation>> {
    g.validate()?;
    let mut b = MeshBuilder::new();
    match &g.shape {
        ShellShape::Revolved { origin, axis, generatrix, sweep } => {
            let line: Vec<Q> = generatrix.points.iter().map(q).collect();
            let (ez, ex, ey) = basis(p(axis));
            revolve(&mut b, &offset_band(&line, g.thickness), p(origin), ex, ey, ez, *sweep, tol, lod);
        }
        ShellShape::Patch { grid } => patch(&mut b, grid, g.thickness / 2.0),
    }
    Ok(b.build(40.0))
}

/// 🟫️ Plate mesh: outline minus holes extruded across the thickness about the mid-plane `offset`, stiffeners standing on the top face.
pub fn plate_mesh(g: &PlateGeometry, tol: Tolerance, lod: Lod) -> Result<MeshData, Vec<Violation>> {
    g.validate()?;
    let (o, ex, ey, ez) = (p(&g.frame.origin), p(&g.frame.x), p(&g.frame.y), p(&g.frame.z));
    let mut b = MeshBuilder::new();
    prism(&mut b, &prepare(std::slice::from_ref(&g.outline)), add(o, mul(ez, g.offset - g.thickness / 2.0)), ex, ey, ez, g.thickness);
    let top = g.offset + g.thickness / 2.0;
    for s in &g.stiffeners {
        let pts: Vec<P> = s.path.points.iter().map(|c| add(o, add(add(mul(ex, c.x), mul(ey, c.y)), mul(ez, top)))).collect();
        let t0 = unit(sub(pts[1], pts[0]));
        let st = stations(&pts, cross(ez, t0), ez, t0, false);
        let mut section: Vec<Prepared> = prepare(&regions(&s.section, tol, lod)?);
        let floor = section.iter().flat_map(|r| r.outer.iter()).map(|c| c[1]).fold(f64::MAX, f64::min);
        section.iter_mut().for_each(|r| r.outer.iter_mut().chain(r.holes.iter_mut().flatten()).for_each(|c| c[1] -= floor));
        sweep(&mut b, &vec![section; st.len()], &st, true);
    }
    Ok(b.build(40.0))
}

/// 🌊️ Double-sided surface mesh for polygons, arc ribbons and grids; invalid input yields an empty mesh.
pub fn surface_mesh(g: &SurfaceGeometry, tol: Tolerance, lod: Lod) -> MeshData {
    if g.validate().is_err() {
        return MeshData::default();
    }
    let mut b = MeshBuilder::new();
    match &g.shape {
        SurfaceShape::Polygon { polygon } => {
            let pts: Vec<P> = polygon.outer.points.iter().map(p).collect();
            let n = unit(pts.windows(2).map(|w| cross(w[0], w[1])).fold(cross(pts[pts.len() - 1], pts[0]), add));
            let (_, u, v) = basis(n);
            let flat = |c: P| [dot(c, u), dot(c, v)];
            let holes: Vec<Vec<Q>> = polygon.holes.iter().map(|h| h.points.iter().map(|c| flat(p(c))).collect()).collect();
            let outer: Vec<Q> = pts.iter().map(|c| flat(*c)).collect();
            let (flat_pts, tris) = triangulate(&outer, &holes);
            let lift = |c: Q| add(mul(u, c[0]), add(mul(v, c[1]), mul(n, dot(pts[0], n))));
            for t in tris {
                let (a, c, d) = (lift(flat_pts[t[0]]), lift(flat_pts[t[1]]), lift(flat_pts[t[2]]));
                b.tri(a, c, d);
                b.tri(a, d, c);
            }
        }
        SurfaceShape::Arc { arc, width } => {
            let pts = super::extrude::axis_points(&crate::Path3::Arc(arc.clone()), tol, lod);
            let w = mul(unit(p(&arc.normal)), *width);
            for pair in pts.windows(2) {
                let q4 = [pair[0], pair[1], add(pair[1], w), add(pair[0], w)];
                let n = cross(sub(q4[1], q4[0]), sub(q4[3], q4[0]));
                b.quad_toward(q4, n);
                b.quad_toward(q4, mul(n, -1.0));
            }
        }
        SurfaceShape::Grid { grid } => patch(&mut b, grid, 0.0),
    }
    b.build(60.0)
}

/// 🛢️ Cylindrical shell wall of `radius` and `height` about +Z from `z = 0`.
pub fn cylindrical_shell(radius: f64, height: f64, thickness: f64) -> ShellGeometry {
    ShellGeometry {
        shape: ShellShape::Revolved { origin: Vec3::ZERO, axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(radius, 0.0), Vec2::new(radius, height)] }, sweep: TAU },
        thickness,
    }
}

/// 🌾️ Silo shell: conical hopper down to `outlet_radius`, cylindrical wall of `wall_height`, conical roof rising `roof_rise`, about +Z with the wall base at `z = 0`.
pub fn silo_shell(radius: f64, wall_height: f64, hopper_height: f64, outlet_radius: f64, roof_rise: f64, thickness: f64) -> ShellGeometry {
    ShellGeometry {
        shape: ShellShape::Revolved {
            origin: Vec3::ZERO,
            axis: Vec3::Z,
            generatrix: Polyline2 { points: vec![Vec2::new(outlet_radius, -hopper_height), Vec2::new(radius, 0.0), Vec2::new(radius, wall_height), Vec2::new(0.0, wall_height + roof_rise)] },
            sweep: TAU,
        },
        thickness,
    }
}
