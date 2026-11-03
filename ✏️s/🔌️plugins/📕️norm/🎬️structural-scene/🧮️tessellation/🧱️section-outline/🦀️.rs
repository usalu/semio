//! 🧱️ Section outlines: canonical planar regions at a tolerance and level (chord-tolerant circles, root and corner fillets), section properties and builders for welded, composite and reinforced sections.

use super::triangulate::{area2, oriented};
use super::vector::*;
use super::{segments, Lod, Tolerance};
use crate::{CompositePart, MaterialRole, Polygon2, ProfileKind, RebarLayout, Ring2, SectionParam, SectionShape, Validate, Vec2, Violation};

/// 📐️ Area, centroid and second moments of a region set; `ixx = ∫ v² dA` and `iyy = ∫ u² dA` about the centroid (section `y` is `u`, section `z` is `v`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionProperties {
    pub area: f64,
    pub cu: f64,
    pub cv: f64,
    pub ixx: f64,
    pub iyy: f64,
    pub ixy: f64,
}

pub fn circle(radius: f64, tol: Tolerance, lod: Lod) -> Vec<Q> {
    let n = segments(radius, std::f64::consts::TAU, tol, lod);
    (0..n).map(|i| {
        let a = std::f64::consts::TAU * i as f64 / n as f64;
        [radius * a.cos(), radius * a.sin()]
    }).collect()
}

/// 🔘️ Replaces vertex `i` by a tangent arc of radius `radii[i]` (zero keeps the corner); radii are clamped to half the shorter adjacent edge.
pub fn round_corners(points: &[Q], radii: &[f64], tol: Tolerance, lod: Lod) -> Vec<Q> {
    let n = points.len();
    let mut out = Vec::with_capacity(n * 2);
    for i in 0..n {
        let (a, c, b) = (points[(i + n - 1) % n], points[i], points[(i + 1) % n]);
        let r = radii.get(i).copied().unwrap_or(0.0);
        let (d1, d2) = ([a[0] - c[0], a[1] - c[1]], [b[0] - c[0], b[1] - c[1]]);
        let (l1, l2) = (d1[0].hypot(d1[1]), d2[0].hypot(d2[1]));
        if r <= 0.0 || l1 == 0.0 || l2 == 0.0 {
            out.push(c);
            continue;
        }
        let (v1, v2) = ([d1[0] / l1, d1[1] / l1], [d2[0] / l2, d2[1] / l2]);
        let theta = (v1[0] * v2[0] + v1[1] * v2[1]).clamp(-1.0, 1.0).acos();
        if theta < 1e-9 || (std::f64::consts::PI - theta) < 1e-9 {
            out.push(c);
            continue;
        }
        let r = r.min(0.5 * l1.min(l2) * (theta / 2.0).tan());
        let d = r / (theta / 2.0).tan();
        let (t1, t2) = ([c[0] + v1[0] * d, c[1] + v1[1] * d], [c[0] + v2[0] * d, c[1] + v2[1] * d]);
        let bis = [v1[0] + v2[0], v1[1] + v2[1]];
        let bl = bis[0].hypot(bis[1]);
        let k = r / (theta / 2.0).sin() / bl;
        let m = [c[0] + bis[0] * k, c[1] + bis[1] * k];
        let (a1, a2) = ((t1[1] - m[1]).atan2(t1[0] - m[0]), (t2[1] - m[1]).atan2(t2[0] - m[0]));
        let mut delta = a2 - a1;
        while delta > std::f64::consts::PI {
            delta -= std::f64::consts::TAU;
        }
        while delta < -std::f64::consts::PI {
            delta += std::f64::consts::TAU;
        }
        let steps = segments(r, delta, tol, lod);
        out.push(t1);
        out.extend((1..steps).map(|s| {
            let ang = a1 + delta * s as f64 / steps as f64;
            [m[0] + r * ang.cos(), m[1] + r * ang.sin()]
        }));
        out.push(t2);
    }
    out
}

fn ring(points: Vec<Q>) -> Ring2 {
    Ring2 { points: points.into_iter().map(v2).collect() }
}

fn points(r: &Ring2) -> Vec<Q> {
    r.points.iter().map(q).collect()
}

fn shifted(mut region: Polygon2, by: &Vec2) -> Polygon2 {
    let shift = |r: &mut Ring2| r.points.iter_mut().for_each(|p| *p = Vec2::new(p.x + by.x, p.y + by.y));
    shift(&mut region.outer);
    region.holes.iter_mut().for_each(shift);
    region
}

fn param(params: &[SectionParam], name: &str) -> Option<f64> {
    params.iter().find(|p| p.name == name).map(|p| p.value)
}

fn radii_at(len: usize, at: &[usize], r: f64) -> Vec<f64> {
    (0..len).map(|i| if at.contains(&i) { r } else { 0.0 }).collect()
}

/// 🧱️ Canonical regions of `section`: circles follow the chord tolerance, and parametric profiles carrying an optional radius `r` get root fillets (hollow sections an outer corner radius `r` and inner `r - t`) at Lod2 and above.
pub fn regions(section: &SectionShape, tol: Tolerance, lod: Lod) -> Result<Vec<Polygon2>, Vec<Violation>> {
    section.validate()?;
    match section {
        SectionShape::Parametric { profile, params } => {
            let base = section.outline()?;
            let (d, t, r) = (param(params, "d").unwrap_or(0.0), param(params, "t").unwrap_or(0.0), param(params, "r").unwrap_or(0.0));
            let fillet = lod >= Lod::Lod2 && r > 0.0;
            Ok(match profile {
                ProfileKind::Circle => vec![Polygon2::solid(circle(d / 2.0, tol, lod).into_iter().map(v2).collect())],
                ProfileKind::Chs => vec![Polygon2 { outer: ring(circle(d / 2.0, tol, lod)), holes: vec![ring(circle(d / 2.0 - t, tol, lod))] }],
                ProfileKind::Rhs if fillet => {
                    let (outer, inner) = (points(&base[0].outer), points(&base[0].holes[0]));
                    let hole_radius = (r - t).max(0.0);
                    vec![Polygon2 { outer: ring(round_corners(&outer, &[r; 4], tol, lod)), holes: vec![ring(round_corners(&inner, &[hole_radius; 4], tol, lod))] }]
                }
                ProfileKind::IShape if fillet => vec![round_polygon(&base[0], &radii_at(12, &[3, 4, 9, 10], r), tol, lod)],
                ProfileKind::Channel if fillet => vec![round_polygon(&base[0], &radii_at(8, &[3, 4], r), tol, lod)],
                ProfileKind::Tee if fillet => vec![round_polygon(&base[0], &radii_at(8, &[2, 5], r), tol, lod)],
                ProfileKind::Angle if fillet => vec![round_polygon(&base[0], &radii_at(6, &[3], r), tol, lod)],
                _ => base,
            })
        }
        SectionShape::Composite { parts, .. } => {
            let mut out = Vec::new();
            for part in parts {
                out.extend(regions(&part.shape, tol, lod)?.into_iter().map(|r| shifted(r, &part.offset)));
            }
            Ok(out)
        }
        _ => section.outline(),
    }
}

fn round_polygon(region: &Polygon2, radii: &[f64], tol: Tolerance, lod: Lod) -> Polygon2 {
    Polygon2 { outer: ring(round_corners(&points(&region.outer), radii, tol, lod)), holes: region.holes.clone() }
}

/// 📊️ Properties of disjoint regions by Green's theorem over counter-clockwise outers and clockwise holes.
pub fn properties(regions: &[Polygon2]) -> SectionProperties {
    let (mut a, mut sx, mut sy, mut xx, mut yy, mut xy) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for region in regions {
        let rings = std::iter::once(oriented(&points(&region.outer), true)).chain(region.holes.iter().map(|h| oriented(&points(h), false)));
        for r in rings {
            for i in 0..r.len() {
                let (p0, p1) = (r[i], r[(i + 1) % r.len()]);
                let c = p0[0] * p1[1] - p1[0] * p0[1];
                a += c / 2.0;
                sx += (p0[0] + p1[0]) * c / 6.0;
                sy += (p0[1] + p1[1]) * c / 6.0;
                xx += (p0[0] * p0[0] + p0[0] * p1[0] + p1[0] * p1[0]) * c / 12.0;
                yy += (p0[1] * p0[1] + p0[1] * p1[1] + p1[1] * p1[1]) * c / 12.0;
                xy += (p0[0] * p1[1] + 2.0 * p0[0] * p0[1] + 2.0 * p1[0] * p1[1] + p1[0] * p0[1]) * c / 24.0;
            }
        }
    }
    let (cu, cv) = (sx / a, sy / a);
    SectionProperties { area: a, cu, cv, ixx: yy - a * cv * cv, iyy: xx - a * cu * cu, ixy: xy - a * cu * cv }
}

/// 🏗️ Welded mono-symmetric I-girder with unequal flanges, web centred, bottom flange lowest.
pub fn plate_girder(web_height: f64, web_thickness: f64, top_width: f64, top_thickness: f64, bottom_width: f64, bottom_thickness: f64) -> SectionShape {
    let (tw, h) = (web_thickness / 2.0, web_height);
    let pts: Vec<Vec2> = [
        (-bottom_width / 2.0, 0.0), (bottom_width / 2.0, 0.0), (bottom_width / 2.0, bottom_thickness), (tw, bottom_thickness), (tw, bottom_thickness + h),
        (top_width / 2.0, bottom_thickness + h), (top_width / 2.0, bottom_thickness + h + top_thickness), (-top_width / 2.0, bottom_thickness + h + top_thickness),
        (-top_width / 2.0, bottom_thickness + h), (-tw, bottom_thickness + h), (-tw, bottom_thickness), (-bottom_width / 2.0, bottom_thickness),
    ].iter().map(|&(x, y)| Vec2::new(x, y)).collect();
    SectionShape::Polygon { polygon: Polygon2::solid(pts) }
}

/// 🌊️ Concrete of a trapezoidal profiled-sheeting slab: rib voids `rib_bottom` wide at the base and `rib_top` wide at the top, `pitch` apart, a topping above; bottom of ribs at `v = 0`.
pub fn deck_slab(width: f64, topping: f64, rib_height: f64, rib_top: f64, rib_bottom: f64, pitch: f64) -> Polygon2 {
    let count = (width / pitch).floor().max(1.0) as usize;
    let mut pts = vec![Vec2::new(-width / 2.0, rib_height)];
    for k in 0..count {
        let c = -width / 2.0 + (k as f64 + 0.5) * pitch;
        pts.extend([Vec2::new(c - rib_top / 2.0, rib_height), Vec2::new(c - rib_bottom / 2.0, 0.0), Vec2::new(c + rib_bottom / 2.0, 0.0), Vec2::new(c + rib_top / 2.0, rib_height)]);
    }
    pts.extend([Vec2::new(width / 2.0, rib_height), Vec2::new(width / 2.0, rib_height + topping), Vec2::new(-width / 2.0, rib_height + topping)]);
    Polygon2::solid(pts)
}

/// 🧩️ Steel section topped by a profiled-sheeting slab resting on its upper flange (steel bounding box centred at the origin).
pub fn composite_floor(steel: SectionShape, steel_height: f64, slab: Polygon2) -> SectionShape {
    SectionShape::Composite {
        parts: vec![
            CompositePart { shape: Box::new(steel), role: MaterialRole::Steel, offset: Vec2::ZERO },
            CompositePart { shape: Box::new(SectionShape::Polygon { polygon: slab }), role: MaterialRole::Concrete, offset: Vec2::new(0.0, steel_height / 2.0) },
        ],
        rebar: Vec::new(),
    }
}

/// 🪡️ Rectangular reinforced-concrete section with `bottom` and `top` bars of diameter `bar` at `cover` to the bar surface (link diameter `link` included).
pub fn rc_rectangle(width: f64, height: f64, cover: f64, bar: f64, link: f64, bottom: u32, top: u32) -> SectionShape {
    let inset = cover + link + bar / 2.0;
    let row = |n: u32, v: f64| -> Vec<Vec2> {
        (0..n).map(|i| Vec2::new(if n == 1 { 0.0 } else { -width / 2.0 + inset + (width - 2.0 * inset) * f64::from(i) / f64::from(n - 1) }, v)).collect()
    };
    let rows: Vec<RebarLayout> = [(bottom, -height / 2.0 + inset), (top, height / 2.0 - inset)].into_iter().filter(|(n, _)| *n > 0)
        .map(|(n, v)| RebarLayout { bar_diameter: bar, positions: row(n, v), cover }).collect();
    SectionShape::Composite {
        parts: vec![CompositePart { shape: Box::new(SectionShape::parametric(ProfileKind::Rect, &[("h", height), ("b", width)])), role: MaterialRole::Concrete, offset: Vec2::ZERO }],
        rebar: rows,
    }
}

pub fn net_area(region: &Polygon2) -> f64 {
    area2(&points(&region.outer)).abs() / 2.0 - region.holes.iter().map(|h| area2(&points(h)).abs() / 2.0).sum::<f64>()
}
