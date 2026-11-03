//! 🏹️ Glyphs: load arrows, moment arcs, line, area and pressure patterns, inertial and impact markers, explosion shells and support symbols; nominal metre size, the host rescales to screen space.

use super::extrude::{axis_points, tube};
use super::fasteners::{cylinder, frustum};
use super::shells::{revolve, surface_mesh};
use super::solids::cuboid;
use super::vector::*;
use super::{Lod, MeshBuilder, MeshData, Tolerance};
use crate::{LoadGlyph, LoadProfile, Polygon3, PressureWave, Quantity, SupportGeometry, SupportKind, SurfaceGeometry, SurfaceShape};

const TAU: f64 = std::f64::consts::TAU;

/// 📏️ Shaft length of an arrow for a magnitude: logarithmic in the SI value so decades read as steps.
pub fn shaft_length(magnitude: &Quantity) -> f64 {
    (0.6 + 0.4 * (1.0 + magnitude.value.abs() / 1e3).log10()).clamp(0.6, 2.5)
}

/// ➡️ Arrow with its tip at the local origin pointing along +Z; the shaft extends to `-length`.
pub fn arrow(length: f64, weight: f64, tol: Tolerance, lod: Lod) -> MeshData {
    let (s, h) = (0.03 * weight, 0.09 * weight);
    let hl = (0.25f64).min(0.5 * length);
    let mut b = MeshBuilder::new();
    revolve(&mut b, &[[0.0, -length], [s, -length], [s, -hl], [h, -hl], [0.0, 0.0]], [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], TAU, tol, lod);
    b.build(35.0)
}

fn place(m: &MeshData, tip: P, dir: P) -> MeshData {
    let (t, u, w) = basis(dir);
    m.placed(tip, u, w, t)
}

/// 🔄️ Moment glyph: three-quarter arc of radius 0.5 around local Z with an arrowhead.
pub fn moment_arc(tol: Tolerance, lod: Lod) -> MeshData {
    let sweep = 1.5 * std::f64::consts::PI;
    let n = super::segments(0.5, sweep, tol, lod).max(3);
    let pts: Vec<P> = (0..=n).map(|i| {
        let a = sweep * i as f64 / n as f64;
        [0.5 * a.cos(), 0.5 * a.sin(), 0.0]
    }).collect();
    let mut b = MeshBuilder::new();
    tube(&mut b, &pts, 0.03, false, tol, lod);
    let arc = b.build(35.0);
    let end = pts[n];
    let tangent = [-sweep.sin(), sweep.cos(), 0.0];
    MeshData::merged([arc, arrow(0.2, 1.0, tol, lod).placed(end, basis(tangent).1, basis(tangent).2, tangent)])
}

fn sphere(radius: f64, tol: Tolerance, lod: Lod) -> MeshData {
    let n = super::segments(radius, std::f64::consts::PI, tol, lod).max(4);
    let ring: Vec<Q> = (0..=n).map(|i| {
        let a = std::f64::consts::PI * i as f64 / n as f64;
        [radius * a.sin(), -radius * a.cos()]
    }).collect();
    let mut b = MeshBuilder::new();
    revolve(&mut b, &ring, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], TAU, tol, lod);
    b.build(40.0)
}

fn profile_value(profile: &LoadProfile, s: f64) -> Quantity {
    match profile {
        LoadProfile::Uniform { value } => value.clone(),
        LoadProfile::Linear { start, end } => Quantity::new(start.kind, start.value + (end.value - start.value) * s),
        LoadProfile::Stepped { steps } => steps.iter().rev().find(|x| x.at <= s).unwrap_or(&steps[0]).value.clone(),
        LoadProfile::Sampled { samples } => {
            let k = samples.iter().position(|x| x.at >= s).unwrap_or(samples.len() - 1).max(1);
            let (a, c) = (&samples[k - 1], &samples[k]);
            let t = ((s - a.at) / (c.at - a.at)).clamp(0.0, 1.0);
            Quantity::new(a.value.kind, a.value.value + (c.value.value - a.value.value) * t)
        }
    }
}

fn centroid(poly: &Polygon3) -> P {
    mul(poly.outer.points.iter().fold([0.0; 3], |a, c| add(a, p(c))), 1.0 / poly.outer.points.len() as f64)
}

fn normal(poly: &Polygon3) -> P {
    let pts: Vec<P> = poly.outer.points.iter().map(p).collect();
    let c = centroid(poly);
    unit((0..pts.len()).fold([0.0; 3], |a, i| add(a, cross(sub(pts[i], c), sub(pts[(i + 1) % pts.len()], c)))))
}

fn inside(poly: &[Q], c: Q) -> bool {
    let mut odd = false;
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        if (a[1] > c[1]) != (b[1] > c[1]) && c[0] < (b[0] - a[0]) * (c[1] - a[1]) / (b[1] - a[1]) + a[0] {
            odd = !odd;
        }
    }
    odd
}

/// ⬇️ Load glyph mesh in world coordinates; thermal glyphs have no own position and yield an empty mesh.
pub fn load_mesh(g: &LoadGlyph, tol: Tolerance, lod: Lod) -> MeshData {
    let cells = match lod { Lod::Lod0 | Lod::Lod1 => 2, Lod::Lod2 => 4, Lod::Lod3 => 6 };
    let along = match lod { Lod::Lod0 | Lod::Lod1 => 3, Lod::Lod2 => 7, Lod::Lod3 => 15 };
    match g {
        LoadGlyph::Point { at, direction, magnitude, .. } => place(&arrow(shaft_length(magnitude), 1.0, tol, lod), p(at), p(direction)),
        LoadGlyph::Impact { at, direction, energy_or_force, .. } => place(&arrow(shaft_length(energy_or_force), 2.0, tol, lod), p(at), p(direction)),
        LoadGlyph::Moment { at, axis, .. } => place(&moment_arc(tol, lod), p(at), p(axis)),
        LoadGlyph::Line { path, direction, profile, .. } => {
            let pts = axis_points(path, tol, lod);
            let lens: Vec<f64> = pts.windows(2).map(|w| len(sub(w[1], w[0]))).collect();
            let total: f64 = lens.iter().sum();
            let at_s = |s: f64| {
                let (mut run, target) = (0.0, s * total);
                for (i, l) in lens.iter().enumerate() {
                    if run + l >= target || i == lens.len() - 1 {
                        return lerp(pts[i], pts[i + 1], if *l > 0.0 { ((target - run) / l).clamp(0.0, 1.0) } else { 0.0 });
                    }
                    run += l;
                }
                pts[0]
            };
            MeshData::merged((0..along).map(|i| {
                let s = i as f64 / (along - 1) as f64;
                place(&arrow(shaft_length(&profile_value(profile, s)), 1.0, tol, lod), at_s(s), p(direction))
            }))
        }
        LoadGlyph::Area { polygon, direction, intensity, .. } => {
            let n = normal(polygon);
            let (_, u, v) = basis(n);
            let c0 = centroid(polygon);
            let flat: Vec<Q> = polygon.outer.points.iter().map(|c| [dot(sub(p(c), c0), u), dot(sub(p(c), c0), v)]).collect();
            let (lo, hi) = flat.iter().fold(([f64::MAX; 2], [f64::MIN; 2]), |a, c| ([a.0[0].min(c[0]), a.0[1].min(c[1])], [a.1[0].max(c[0]), a.1[1].max(c[1])]));
            let mut cells_hit: Vec<P> = (0..cells).flat_map(|i| (0..cells).map(move |j| (i, j))).filter_map(|(i, j)| {
                let c = [lo[0] + (hi[0] - lo[0]) * (i as f64 + 0.5) / cells as f64, lo[1] + (hi[1] - lo[1]) * (j as f64 + 0.5) / cells as f64];
                inside(&flat, c).then(|| add(c0, add(mul(u, c[0]), mul(v, c[1]))))
            }).collect();
            if cells_hit.is_empty() {
                cells_hit.push(c0);
            }
            let m = arrow(shaft_length(intensity), 1.0, tol, lod);
            MeshData::merged(cells_hit.into_iter().map(|c| place(&m, c, p(direction))))
        }
        LoadGlyph::Pressure { zones, .. } => MeshData::merged(zones.iter().flat_map(|z| {
            let toward = mul(normal(&z.polygon), -z.value.value.signum());
            let mut tips = vec![centroid(&z.polygon)];
            if lod >= Lod::Lod2 {
                tips.extend(z.polygon.outer.points.iter().take(8).map(|c| lerp(p(c), centroid(&z.polygon), 0.2)));
            }
            let m = arrow(shaft_length(&z.value), 1.0, tol, lod);
            tips.into_iter().map(move |t| place(&m, t, toward))
        })),
        LoadGlyph::Inertial { direction, masses, .. } => MeshData::merged(masses.iter().flat_map(|m| {
            let tip = p(&m.at);
            [place(&arrow(shaft_length(&m.mass), 1.0, tol, lod), tip, p(direction)), sphere(0.06, tol, lod).translated(tip)]
        })),
        LoadGlyph::Explosion { centre, shape, .. } => match shape {
            PressureWave::Spherical => sphere(0.5, tol, lod).translated(p(centre)),
            PressureWave::Planar => {
                let mut b = MeshBuilder::new();
                cuboid(&mut b, [-0.5, -0.5, -0.01], [0.5, 0.5, 0.01]);
                b.build(35.0).translated(p(centre))
            }
        },
        LoadGlyph::Thermal { .. } => MeshData::default(),
    }
}

fn coil(tol: Tolerance, lod: Lod) -> MeshData {
    let (turns, per) = (6usize, 12usize);
    let pts: Vec<P> = (0..=turns * per).map(|i| {
        let a = TAU * i as f64 / per as f64;
        [0.15 * a.cos(), 0.15 * a.sin(), -0.6 * i as f64 / (turns * per) as f64]
    }).collect();
    let mut b = MeshBuilder::new();
    tube(&mut b, &pts, 0.02, false, tol, lod);
    b.build(35.0)
}

/// 🔧️ Support symbol below its point in the support frame (fixed block, pinned cone, roller, coil spring, pile head) or a slab for an elastic bed.
pub fn support_mesh(g: &SupportGeometry, tol: Tolerance, lod: Lod) -> MeshData {
    let mut b = MeshBuilder::new();
    let local = match &g.kind {
        SupportKind::Fixed => {
            cuboid(&mut b, [-0.3, -0.3, -0.1], [0.3, 0.3, 0.0]);
            b.build(35.0)
        }
        SupportKind::Pinned => {
            frustum(&mut b, 0.3, 0.0, -0.5, 0.0, tol, lod);
            cuboid(&mut b, [-0.4, -0.4, -0.58], [0.4, 0.4, -0.5]);
            b.build(35.0)
        }
        SupportKind::Roller { .. } => {
            frustum(&mut b, 0.3, 0.0, -0.4, 0.0, tol, lod);
            cylinder(&mut b, 0.1, -0.6, -0.4, tol, lod);
            cuboid(&mut b, [-0.4, -0.4, -0.68], [0.4, 0.4, -0.6]);
            b.build(35.0)
        }
        SupportKind::Spring { .. } => coil(tol, lod),
        SupportKind::ElasticBed { extent, .. } => {
            return surface_mesh(&SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: extent.clone() } }, tol, lod);
        }
        SupportKind::PileHead => {
            cylinder(&mut b, 0.2, -0.4, 0.0, tol, lod);
            b.build(35.0)
        }
    };
    local.placed(p(&g.at), p(&g.frame.x), p(&g.frame.y), p(&g.frame.z))
}
