//! 📏️ Extrusion: section regions swept along straight, tapered, curved and cambered axes with parallel-transported frames and capped ends.

use super::section_outline::{circle, regions};
use super::triangulate::{oriented, triangulate};
use super::vector::*;
use super::{segments, Lod, MeshBuilder, MeshData, Tolerance};
use crate::{MemberGeometry, Path3, Polygon2, SectionShape, Validate, Violation};

/// 🎞️ One section plane along a sweep: origin, in-plane axes, unit tangent and relative arc position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Station {
    pub origin: P,
    pub u: P,
    pub v: P,
    pub t: P,
    pub s: f64,
}

/// 🔷️ Region with a counter-clockwise outer ring and clockwise holes.
#[derive(Clone, Debug, PartialEq)]
pub struct Prepared {
    pub outer: Vec<Q>,
    pub holes: Vec<Vec<Q>>,
}

fn violation(path: &str, rule: &str) -> Vec<Violation> {
    vec![Violation { path: path.to_owned(), rule: rule.to_owned() }]
}

pub fn prepare(regions: &[Polygon2]) -> Vec<Prepared> {
    regions.iter().map(|r| Prepared {
        outer: oriented(&r.outer.points.iter().map(q).collect::<Vec<_>>(), true),
        holes: r.holes.iter().map(|h| oriented(&h.points.iter().map(q).collect::<Vec<_>>(), false)).collect(),
    }).collect()
}

fn blend(a: &[Prepared], b: &[Prepared], t: f64) -> Option<Vec<Prepared>> {
    let same = a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.outer.len() == y.outer.len() && x.holes.len() == y.holes.len() && x.holes.iter().zip(&y.holes).all(|(h, k)| h.len() == k.len()));
    let mix = |x: &[Q], y: &[Q]| x.iter().zip(y).map(|(c, d)| lerp2(*c, *d, t)).collect::<Vec<_>>();
    same.then(|| a.iter().zip(b).map(|(x, y)| Prepared { outer: mix(&x.outer, &y.outer), holes: x.holes.iter().zip(&y.holes).map(|(h, k)| mix(h, k)).collect() }).collect())
}

fn bspline(control: &[P], degree: usize, t: f64) -> P {
    let n = control.len();
    if t >= 1.0 {
        return control[n - 1];
    }
    let knots: Vec<f64> = (0..n + degree + 1).map(|i| if i <= degree { 0.0 } else if i >= n { 1.0 } else { (i - degree) as f64 / (n - degree) as f64 }).collect();
    let k = (degree..n).find(|&k| knots[k] <= t && t < knots[k + 1]).unwrap_or(n - 1);
    let mut d: Vec<P> = control[k - degree..=k].to_vec();
    for r in 1..=degree {
        for j in (r..=degree).rev() {
            let i = j + k - degree;
            let alpha = (t - knots[i]) / (knots[i + degree + 1 - r] - knots[i]);
            d[j] = lerp(d[j - 1], d[j], alpha);
        }
    }
    d[degree]
}

/// 🛤️ Axis samples: polyline vertices, arc and clamped B-spline sampled by the level.
pub fn axis_points(path: &Path3, tol: Tolerance, lod: Lod) -> Vec<P> {
    match path {
        Path3::Polyline(l) => l.points.iter().map(p).collect(),
        Path3::Arc(a) => {
            let (c, n, s) = (p(&a.centre), unit(p(&a.normal)), p(&a.start));
            let radius = len(sub(s, c));
            let count = segments(radius, a.sweep, tol, lod);
            (0..=count).map(|i| add(c, rotate(sub(s, c), n, a.sweep * i as f64 / count as f64))).collect()
        }
        Path3::Spline(sp) => {
            let control: Vec<P> = sp.control.iter().map(p).collect();
            let factor = match lod { Lod::Lod0 | Lod::Lod1 => 2, Lod::Lod2 => 4, Lod::Lod3 => 8 };
            let count = (control.len() * factor).max(4);
            (0..=count).map(|i| bspline(&control, sp.degree as usize, i as f64 / count as f64)).collect()
        }
    }
}

/// 🎞️ Stations along `points` with minimal-rotation transport of the initial in-plane axes; `closed` wraps tangents and repeats the first point.
pub fn stations(points: &[P], u0: P, v0: P, t0: P, closed: bool) -> Vec<Station> {
    let n = points.len();
    let segs: Vec<P> = (0..if closed { n } else { n - 1 }).map(|i| unit(sub(points[(i + 1) % n], points[i]))).collect();
    let tangent = |i: usize| -> P {
        let prev = if i > 0 { Some(segs[i - 1]) } else if closed { Some(segs[segs.len() - 1]) } else { None };
        let next = segs.get(i).copied().filter(|_| i < segs.len());
        match (prev, next) {
            (Some(a), Some(b)) if len(add(a, b)) > 1e-9 => unit(add(a, b)),
            (Some(a), _) => a,
            (None, Some(b)) => b,
            _ => t0,
        }
    };
    let total: f64 = segs.iter().enumerate().map(|(i, _)| len(sub(points[(i + 1) % n], points[i]))).sum();
    let first = tangent(0);
    let (mut u, mut v) = (rotate_between(u0, t0, first), rotate_between(v0, t0, first));
    let mut out = Vec::with_capacity(n + 1);
    let mut run = 0.0;
    let mut prev_t = first;
    let count = if closed { n + 1 } else { n };
    for i in 0..count {
        let t = if i == n { first } else { tangent(i) };
        if i > 0 {
            u = rotate_between(u, prev_t, t);
            v = rotate_between(v, prev_t, t);
            run += len(sub(points[i % n], points[i - 1]));
        }
        out.push(Station { origin: points[i % n], u, v, t, s: if total > 0.0 { run / total } else { 0.0 } });
        prev_t = t;
    }
    out
}

fn world(st: &Station, c: Q) -> P {
    add(st.origin, add(mul(st.u, c[0]), mul(st.v, c[1])))
}

/// 🧵️ Sweeps `sections[k]` (identical topology) through `st[k]`: smooth-shaded side walls, optional flat caps.
pub fn sweep(b: &mut MeshBuilder, sections: &[Vec<Prepared>], st: &[Station], caps: bool) {
    for k in 0..st.len().saturating_sub(1) {
        for (j, region) in sections[k].iter().enumerate() {
            let rings = std::iter::once((&region.outer, &sections[k + 1][j].outer)).chain(region.holes.iter().zip(&sections[k + 1][j].holes));
            for (ra, rb) in rings {
                let m = ra.len();
                for i in 0..m {
                    let (a0, a1, b0, b1) = (ra[i], ra[(i + 1) % m], rb[i], rb[(i + 1) % m]);
                    let d = [a1[0] - a0[0], a1[1] - a0[1]];
                    let hint = add(mul(st[k].u, d[1]), mul(st[k].v, -d[0]));
                    b.quad_toward_s([world(&st[k], a0), world(&st[k], a1), world(&st[k + 1], b1), world(&st[k + 1], b0)], hint, [st[k].s as f32, st[k].s as f32, st[k + 1].s as f32, st[k + 1].s as f32]);
                }
            }
        }
    }
    if caps && st.len() >= 2 {
        for (idx, sign) in [(0, -1.0), (st.len() - 1, 1.0)] {
            for region in &sections[idx] {
                let (pts, tris) = triangulate(&region.outer, &region.holes);
                let s = st[idx].s as f32;
                for t in tris {
                    b.tri_toward_s(world(&st[idx], pts[t[0]]), world(&st[idx], pts[t[1]]), world(&st[idx], pts[t[2]]), mul(st[idx].t, sign), [s; 3]);
                }
            }
        }
    }
}

/// 🧱️ Straight capped prism of `regions` in the plane spanned by `u`, `v` at `origin`, extruded `height` along `w`.
pub fn prism(b: &mut MeshBuilder, regions: &[Prepared], origin: P, u: P, v: P, w: P, height: f64) {
    let t = unit(w);
    let st = [Station { origin, u, v, t, s: 0.0 }, Station { origin: add(origin, mul(t, height)), u, v, t, s: 1.0 }];
    sweep(b, &[regions.to_vec(), regions.to_vec()], &st, true);
}

/// ⭕️ Round tube of `radius` along `points`; closed loops share their seam and carry no caps.
pub fn tube(b: &mut MeshBuilder, points: &[P], radius: f64, closed: bool, tol: Tolerance, lod: Lod) {
    if points.len() < 2 || radius <= 0.0 {
        return;
    }
    let (t0, u0, v0) = basis(sub(points[1], points[0]));
    let st = stations(points, u0, v0, t0, closed);
    let ring = prepare(&[Polygon2::solid(circle(radius, tol, lod).into_iter().map(v2).collect())]);
    sweep(b, &vec![ring; st.len()], &st, !closed);
}

/// 📏️ Member mesh: section (or tapered pair) swept along the axis in the member frame with end offsets; composite rebar as bars at `Lod2` and above; scalar slot is the relative arc position.
pub fn member_mesh(g: &MemberGeometry, tol: Tolerance, lod: Lod) -> Result<MeshData, Vec<Violation>> {
    g.validate()?;
    let pts = axis_points(&g.axis, tol, lod);
    if pts.len() < 2 || pts.windows(2).any(|w| len(sub(w[1], w[0])) <= 0.0) {
        return Err(violation("$.axis", "non-degenerate"));
    }
    let (fx, fy, fz) = (p(&g.frame.x), p(&g.frame.y), p(&g.frame.z));
    let mut st = stations(&pts, fy, fz, fx, false);
    for s in &mut st {
        s.origin = add(s.origin, add(mul(s.u, g.start_offset.x + (g.end_offset.x - g.start_offset.x) * s.s), mul(s.v, g.start_offset.y + (g.end_offset.y - g.start_offset.y) * s.s)));
    }
    let a = prepare(&regions(&g.section, tol, lod)?);
    let sections: Vec<Vec<Prepared>> = match &g.section_taper {
        None => vec![a; st.len()],
        Some(taper) => {
            let b = prepare(&regions(taper, tol, lod)?);
            st.iter().map(|s| blend(&a, &b, s.s)).collect::<Option<_>>().ok_or_else(|| violation("$.sectionTaper", "same-outline-topology"))?
        }
    };
    let mut builder = MeshBuilder::with_scalar();
    sweep(&mut builder, &sections, &st, true);
    if let (SectionShape::Composite { rebar, .. }, true) = (&g.section, lod >= Lod::Lod2) {
        for layout in rebar {
            let ring = prepare(&[Polygon2::solid(circle(layout.bar_diameter / 2.0, tol, lod).into_iter().map(v2).collect())]);
            for pos in &layout.positions {
                let shifted: Vec<Station> = st.iter().map(|s| Station { origin: add(s.origin, add(mul(s.u, pos.x), mul(s.v, pos.y))), ..*s }).collect();
                sweep(&mut builder, &vec![ring.clone(); st.len()], &shifted, true);
            }
        }
    }
    Ok(builder.build(40.0))
}

/// 📐️ Straight member of `length` along +X in the world frame, the shape the geometry oracle checks against `area * length`.
pub fn straight_mesh(section: &SectionShape, length: f64, tol: Tolerance, lod: Lod) -> Result<MeshData, Vec<Violation>> {
    use crate::{EndRelease, Frame, Polyline3, Vec2, Vec3};
    member_mesh(&MemberGeometry {
        axis: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(length, 0.0, 0.0)] }),
        frame: Frame::WORLD,
        section: section.clone(),
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: Vec::new(),
    }, tol, lod)
}
