//! 🔩️ Fasteners: bolts with nuts and washers, studs, screws, nails, dowels, anchors, welds, rebar with bends, stirrup cages and tendons; patterns instance one unit mesh.

use super::extrude::{axis_points, prism, prepare, tube};
use super::section_outline::round_corners;
use super::shells::revolve;
use super::vector::*;
use super::{Lod, MeshBuilder, MeshData, Tolerance};
use crate::{FastenerGeometry, FastenerKind, HeadShape, Pattern, Polygon2, Validate, Violation};

const Z: P = [0.0, 0.0, 1.0];
const X: P = [1.0, 0.0, 0.0];
const Y: P = [0.0, 1.0, 0.0];

/// 🛢️ Cylinder of `radius` along local Z between `z0` and `z1`.
pub fn cylinder(b: &mut MeshBuilder, radius: f64, z0: f64, z1: f64, tol: Tolerance, lod: Lod) {
    revolve(b, &[[0.0, z0], [radius, z0], [radius, z1], [0.0, z1]], [0.0; 3], X, Y, Z, std::f64::consts::TAU, tol, lod);
}

/// 🔻️ Truncated cone from radius `r0` at `z0` to `r1` at `z1`.
pub fn frustum(b: &mut MeshBuilder, r0: f64, r1: f64, z0: f64, z1: f64, tol: Tolerance, lod: Lod) {
    revolve(b, &[[0.0, z0], [r0, z0], [r1, z1], [0.0, z1]], [0.0; 3], X, Y, Z, std::f64::consts::TAU, tol, lod);
}

/// ⬡ Hexagonal prism of the given width across flats along local Z.
pub fn hex_prism(b: &mut MeshBuilder, across_flats: f64, z0: f64, z1: f64) {
    let r = across_flats / 3f64.sqrt();
    let ring: Vec<_> = (0..6).map(|i| {
        let a = std::f64::consts::FRAC_PI_3 * i as f64;
        crate::Vec2::new(r * a.cos(), r * a.sin())
    }).collect();
    prism(b, &prepare(&[Polygon2::solid(ring)]), [0.0, 0.0, z0], X, Y, Z, z1 - z0);
}

fn dome(b: &mut MeshBuilder, radius: f64, height: f64, z0: f64, tol: Tolerance, lod: Lod) {
    let n = super::segments(radius, std::f64::consts::FRAC_PI_2, tol, lod).max(2);
    let mut ring = vec![[0.0, z0]];
    ring.extend((0..=n).map(|i| {
        let a = std::f64::consts::FRAC_PI_2 * i as f64 / n as f64;
        [radius * a.cos(), z0 + height * a.sin()]
    }));
    revolve(b, &ring, [0.0; 3], X, Y, Z, std::f64::consts::TAU, tol, lod);
}

fn head(b: &mut MeshBuilder, shape: HeadShape, d: f64, tol: Tolerance, lod: Lod) {
    match shape {
        HeadShape::Hex => hex_prism(b, 1.5 * d, -0.65 * d, 0.0),
        HeadShape::Countersunk => frustum(b, d, d / 2.0, -0.5 * d, 0.0, tol, lod),
        HeadShape::Round => dome(b, 1.0 * d, 0.5 * d, -0.5 * d, tol, lod),
        HeadShape::Headed => cylinder(b, 0.9 * d, -0.4 * d, 0.0, tol, lod),
        HeadShape::None => {}
    }
}

fn bolt(b: &mut MeshBuilder, d: f64, length: f64, tol: Tolerance, lod: Lod) {
    let (k, m, tw) = (0.65 * d, 0.8 * d, 0.2 * d);
    hex_prism(b, 1.5 * d, -tw - k, -tw);
    cylinder(b, 1.1 * d, -tw, 0.0, tol, lod);
    cylinder(b, d / 2.0, -tw, length, tol, lod);
    if length > m + tw + 0.3 * d {
        hex_prism(b, 1.5 * d, length - 0.3 * d - m, length - 0.3 * d);
        cylinder(b, 1.1 * d, length - 0.3 * d - m - tw, length - 0.3 * d - m, tol, lod);
    }
}

fn weld_seam(b: &mut MeshBuilder, throat: f64, weld: crate::WeldKind, d: f64, length: f64) {
    let a = throat * 2f64.sqrt();
    let tri = match weld {
        crate::WeldKind::Fillet => vec![crate::Vec2::new(0.0, 0.0), crate::Vec2::new(a, 0.0), crate::Vec2::new(0.0, a)],
        crate::WeldKind::Butt => vec![crate::Vec2::new(-d / 2.0, 0.0), crate::Vec2::new(d / 2.0, 0.0), crate::Vec2::new(d / 2.0, throat), crate::Vec2::new(-d / 2.0, throat)],
    };
    prism(b, &prepare(&[Polygon2::solid(tri)]), [0.0; 3], X, Y, Z, length);
}

/// 🧵️ Weld seam swept along a polyline: per straight segment one continuous prism, or runs of `length` separated by `gap` restarting at every segment.
pub fn weld_run(pts: &[P], throat: f64, weld: crate::WeldKind, d: f64, pitch: Option<(f64, f64)>) -> MeshData {
    let pieces = pts.windows(2).flat_map(|w| {
        let (seg, (t, u, v)) = (len(sub(w[1], w[0])), basis(sub(w[1], w[0])));
        let spans: Vec<(f64, f64)> = match pitch {
            None => vec![(0.0, seg)],
            Some((run, gap)) => (0..).map(|i| f64::from(i) * (run + gap)).take_while(|s| *s < seg).map(|s| (s, (s + run).min(seg))).collect(),
        };
        spans.into_iter().map(move |(a, b)| {
            let mut m = MeshBuilder::new();
            weld_seam(&mut m, throat, weld, d, b - a);
            m.build(35.0).placed(add(w[0], mul(t, a)), u, v, t)
        })
    });
    MeshData::merged(pieces)
}

fn unit_mesh(g: &FastenerGeometry, length: f64, tol: Tolerance, lod: Lod) -> MeshData {
    let (d, r) = (g.diameter, g.diameter / 2.0);
    let mut b = MeshBuilder::new();
    match &g.kind {
        FastenerKind::Bolt { .. } => bolt(&mut b, d, length, tol, lod),
        FastenerKind::Stud { head_height, head_diameter } => {
            cylinder(&mut b, r, 0.0, (length - head_height).max(0.0), tol, lod);
            cylinder(&mut b, head_diameter / 2.0, (length - head_height).max(0.0), length, tol, lod);
        }
        FastenerKind::Screw => {
            cylinder(&mut b, r, 0.0, length, tol, lod);
            head(&mut b, g.head, d, tol, lod);
        }
        FastenerKind::Nail => {
            cylinder(&mut b, r, 0.0, length, tol, lod);
            cylinder(&mut b, 1.1 * d, -0.3 * d, 0.0, tol, lod);
        }
        FastenerKind::Dowel => cylinder(&mut b, r, 0.0, length, tol, lod),
        FastenerKind::Anchor => {
            cylinder(&mut b, r, 0.0, length, tol, lod);
            hex_prism(&mut b, 1.5 * d, -0.8 * d, 0.0);
        }
        FastenerKind::Weld { throat, weld } => weld_seam(&mut b, *throat, *weld, d, length),
        FastenerKind::Rebar { .. } | FastenerKind::Tendon { .. } | FastenerKind::WeldRun { .. } => {}
    }
    b.build(35.0)
}

/// 🪝️ Rebar polyline from `from` to `to` with the hook `bend = (angle, radius, leg)` at `to`, and with `both_ends` the same hook mirrored at `from`, turning to the same side of the bar.
pub fn bent_bar(from: P, to: P, bend: Option<(f64, f64, f64)>, both_ends: bool, tol: Tolerance, lod: Lod) -> Vec<P> {
    let Some((angle, radius, leg)) = bend else { return vec![from, to] };
    let (t, perp, _) = basis(sub(to, from));
    let n = super::segments(radius, angle, tol, lod);
    let hook = |end: P, out: P| -> Vec<P> {
        let axis = unit(cross(out, perp));
        let centre = add(end, mul(perp, radius));
        let mut pts: Vec<P> = (0..=n).map(|i| add(centre, rotate(sub(end, centre), axis, angle * i as f64 / n as f64))).collect();
        pts.push(add(pts[pts.len() - 1], mul(rotate(out, axis, angle), leg)));
        pts
    };
    let mut pts = if both_ends {
        let mut start = hook(from, mul(t, -1.0));
        start.reverse();
        start
    } else {
        vec![from]
    };
    pts.extend(hook(to, t));
    pts
}

/// 🔲️ Closed stirrup loop in the plane of `e1`, `e2` through `origin`, rounded at bend radius `2 * diameter`.
pub fn stirrup_loop(origin: P, e1: P, e2: P, width: f64, height: f64, diameter: f64, tol: Tolerance, lod: Lod) -> Vec<P> {
    let corners = [[-width / 2.0, -height / 2.0], [width / 2.0, -height / 2.0], [width / 2.0, height / 2.0], [-width / 2.0, height / 2.0]];
    let rounded = round_corners(&corners, &[2.0 * diameter; 4], tol, lod);
    rounded.iter().map(|c| add(origin, add(mul(e1, c[0]), mul(e2, c[1])))).collect()
}

/// 🔁️ Translations of each instance relative to the first, bounded by the level.
pub fn offsets(pattern: &Option<Pattern>, dir: P, tol: Tolerance, lod: Lod) -> Vec<P> {
    let (_, e1, _) = basis(dir);
    let cap = lod.max_instances();
    let list: Vec<P> = match pattern {
        None | Some(Pattern::Cage { .. }) => vec![[0.0; 3]],
        Some(Pattern::Points { offsets }) => {
            let (t, u, w) = basis(dir);
            offsets.iter().map(|o| add(mul(t, o.x), add(mul(u, o.y), mul(w, o.z)))).collect()
        }
        Some(Pattern::Grid { nx, ny, sx, sy }) => (0..*ny).flat_map(|j| (0..*nx).map(move |i| (i, j))).map(|(i, j)| add(mul(e1, f64::from(i) * sx), mul(cross(dir, e1), f64::from(j) * sy))).collect(),
        Some(Pattern::Row { n, s }) => (0..*n).map(|i| mul(e1, f64::from(i) * s)).collect(),
        Some(Pattern::Along { path, spacing }) => {
            let pts = axis_points(path, tol, lod);
            let mut out = vec![[0.0; 3]];
            let (mut carried, mut next) = (0.0, *spacing);
            for w in pts.windows(2) {
                let seg = len(sub(w[1], w[0]));
                while carried + seg >= next && seg > 0.0 {
                    out.push(sub(lerp(w[0], w[1], (next - carried) / seg), pts[0]));
                    next += spacing;
                }
                carried += seg;
            }
            out
        }
    };
    list.into_iter().take(cap).collect()
}

fn cage_basis(t: P, reference: P, fallback: P) -> (P, P) {
    let candidate = sub(reference, mul(t, dot(reference, t)));
    let u = if len(candidate) > 1e-9 { unit(candidate) } else { fallback };
    (u, cross(t, u))
}

/// 🔩️ Fastener mesh: one unit mesh placed on the axis and instanced over the pattern; stirrup cages are the closed pattern outline repeated per link zone along the axis.
pub fn fastener_mesh(g: &FastenerGeometry, tol: Tolerance, lod: Lod) -> Result<MeshData, Vec<Violation>> {
    g.validate()?;
    let (from, to) = (p(&g.axis.from), p(&g.axis.to));
    let length = len(sub(to, from));
    let (t, u, w) = basis(sub(to, from));
    let offs = offsets(&g.pattern, t, tol, lod);
    let world = |kind: &FastenerKind| -> Option<MeshData> {
        let mut b = MeshBuilder::new();
        match kind {
            FastenerKind::Rebar { bend } => {
                let both_ends = bend.as_ref().is_some_and(|s| s.both_ends);
                let bend = bend.as_ref().map(|s| (s.bend_angle, s.bend_radius, s.leg_length));
                tube(&mut b, &bent_bar(from, to, bend, both_ends, tol, lod), g.diameter / 2.0, false, tol, lod);
            }
            FastenerKind::Tendon { profile, .. } => tube(&mut b, &axis_points(profile, tol, lod), g.diameter / 2.0, false, tol, lod),
            FastenerKind::WeldRun { throat, weld, path, pitch } => return Some(weld_run(&axis_points(path, tol, lod), *throat, *weld, g.diameter, pitch.as_ref().map(|q| (q.length, q.gap)))),
            _ => return None,
        }
        Some(b.build(35.0))
    };
    if let (FastenerKind::Rebar { .. }, Some(Pattern::Cage { outline, zones, reference })) = (&g.kind, &g.pattern) {
        let (u, w) = cage_basis(t, p(reference), u);
        let corners: Vec<[f64; 2]> = outline.points.iter().map(|q| [q.x, q.y]).collect();
        let rounded = round_corners(&corners, &vec![2.0 * g.diameter; corners.len()], tol, lod);
        let parts = zones.iter().flat_map(|z| (0..z.count).map(move |i| z.start + z.spacing * f64::from(i))).take(lod.max_instances()).map(|x| {
            let mut b = MeshBuilder::new();
            let ring: Vec<P> = rounded.iter().map(|c| add(add(from, mul(t, x)), add(mul(u, c[0]), mul(w, c[1])))).collect();
            tube(&mut b, &ring, g.diameter / 2.0, true, tol, lod);
            b.build(35.0)
        });
        return Ok(MeshData::merged(parts));
    }
    let base = world(&g.kind).unwrap_or_else(|| unit_mesh(g, length, tol, lod).placed(from, u, w, t));
    Ok(MeshData::merged(offs.iter().map(|o| base.translated(*o))))
}
