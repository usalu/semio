//! 🕸️ Conforming triangle mesh of a polygon with holes: ear clipping with hole bridges, Lawson flips and Rivara 4T-LE longest-edge refinement; deterministic, no dependencies. Mirrors the Python prototype `🧰️scripts/mechanics-section/proto.py` of the ticket NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS.

use crate::{Control, MechanicsError, MechanicsResult};
use std::collections::{BTreeMap, BTreeSet};

pub type P2 = [f64; 2];

#[derive(Clone, Debug)]
pub struct Mesh {
    pub points: Vec<P2>,
    pub triangles: Vec<[usize; 3]>,
}

fn invalid<T>(en: &str, de: &str) -> MechanicsResult<T> {
    Err(MechanicsError::invalid(en, de))
}

/// 📐️ Signed area of a ring (positive counter-clockwise).
pub fn signed_area(ring: &[P2]) -> f64 {
    let n = ring.len();
    (0..n)
        .map(|i| {
            let (p, q) = (ring[i], ring[(i + 1) % n]);
            p[0] * q[1] - q[0] * p[1]
        })
        .sum::<f64>()
        / 2.0
}

fn orient(a: P2, b: P2, c: P2) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn dist(a: P2, b: P2) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

fn scale_of(points: &[P2]) -> f64 {
    points.iter().fold(1e-300_f64, |m, p| m.max(p[0].abs()).max(p[1].abs()))
}

/// 🎯️ Even-odd point-in-ring test.
pub fn inside(ring: &[P2], p: P2) -> bool {
    let n = ring.len();
    let mut result = false;
    let mut j = n - 1;
    for i in 0..n {
        let (a, b) = (ring[i], ring[j]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            result = !result;
        }
        j = i;
    }
    result
}

fn clean(ring: &[P2]) -> Vec<P2> {
    let scale = scale_of(ring);
    let mut points = ring.to_vec();
    loop {
        let n = points.len();
        if n <= 3 {
            return points;
        }
        let drop = (0..n).find(|&i| {
            let (a, b, c) = (points[(i + n - 1) % n], points[i], points[(i + 1) % n]);
            dist(a, b) <= 1e-12 * scale || orient(a, b, c).abs() <= 1e-12 * scale * scale
        });
        match drop {
            Some(i) => {
                points.remove(i);
            }
            None => return points,
        }
    }
}

/// 🧹️ Validate and normalise the outline: duplicate and collinear vertices dropped, the outer ring counter-clockwise, holes clockwise and inside the outer ring.
pub fn normalise(outer: &[P2], holes: &[Vec<P2>]) -> MechanicsResult<Vec<Vec<P2>>> {
    let mut rings: Vec<Vec<P2>> = Vec::with_capacity(1 + holes.len());
    for k in 0..=holes.len() {
        let raw: &[P2] = if k == 0 { outer } else { &holes[k - 1] };
        if raw.iter().any(|p| !p[0].is_finite() || !p[1].is_finite()) {
            return invalid("section outline has a non-finite coordinate", "Querschnittsumriss hat eine nicht endliche Koordinate");
        }
        let mut ring = clean(raw);
        if ring.len() < 3 {
            return invalid("section ring needs at least three distinct non-collinear vertices", "Querschnittsring braucht mindestens drei verschiedene nicht kollineare Eckpunkte");
        }
        let area = signed_area(&ring);
        let scale = scale_of(&ring);
        if area.abs() <= 1e-12 * scale * scale {
            return invalid("section ring has no area", "Querschnittsring hat keine Fläche");
        }
        if (area > 0.0) != (k == 0) {
            ring.reverse();
        }
        rings.push(ring);
    }
    for hole in &rings[1..] {
        if !inside(&rings[0], hole[0]) {
            return invalid("a hole lies outside the outer ring", "ein Loch liegt außerhalb des äußeren Rings");
        }
    }
    Ok(rings)
}

fn crosses(p: P2, q: P2, r: P2, s: P2) -> bool {
    let (o1, o2, o3, o4) = (orient(p, q, r), orient(p, q, s), orient(r, s, p), orient(r, s, q));
    ((o1 > 0.0 && o2 < 0.0) || (o1 < 0.0 && o2 > 0.0)) && ((o3 > 0.0 && o4 < 0.0) || (o3 < 0.0 && o4 > 0.0))
}

fn touches(p: P2, q: P2, c: P2, tol: f64) -> bool {
    if dist(c, p) <= tol || dist(c, q) <= tol {
        return false;
    }
    let d = dist(p, q);
    if d == 0.0 || orient(p, q, c).abs() / d > tol {
        return false;
    }
    let t = ((c[0] - p[0]) * (q[0] - p[0]) + (c[1] - p[1]) * (q[1] - p[1])) / (d * d);
    t > 0.0 && t < 1.0
}

fn in_triangle(a: P2, b: P2, c: P2, p: P2) -> bool {
    orient(a, b, p) >= 0.0 && orient(b, c, p) >= 0.0 && orient(c, a, p) >= 0.0
}

fn is_ear(points: &[P2], ring: &[usize], i: usize, min_sin: f64) -> bool {
    let n = ring.len();
    let (a, b, c) = (points[ring[(i + n - 1) % n]], points[ring[i]], points[ring[(i + 1) % n]]);
    orient(a, b, c) > min_sin * dist(a, b) * dist(b, c) && ring.iter().all(|&k| {
        let p = points[k];
        p == a || p == b || p == c || !in_triangle(a, b, c, p)
    })
}

fn ear_clip(points: &[P2], mut ring: Vec<usize>, control: &Control<'_>) -> MechanicsResult<Vec<[usize; 3]>> {
    let mut triangles = Vec::with_capacity(ring.len());
    while ring.len() > 3 {
        control.check()?;
        let n = ring.len();
        let ear = (0..n).find(|&i| is_ear(points, &ring, i, 1e-9)).or_else(|| (0..n).find(|&i| is_ear(points, &ring, i, 0.0)));
        let Some(i) = ear else {
            return invalid("section outline cannot be triangulated (self-intersecting or touching rings)", "Querschnittsumriss lässt sich nicht triangulieren (sich schneidende oder berührende Ringe)");
        };
        triangles.push([ring[(i + n - 1) % n], ring[i], ring[(i + 1) % n]]);
        ring.remove(i);
    }
    if orient(points[ring[0]], points[ring[1]], points[ring[2]]) > 0.0 {
        triangles.push([ring[0], ring[1], ring[2]]);
    }
    Ok(triangles)
}

fn triangulate(points: &[P2], outer: &[usize], holes: &[Vec<usize>], control: &Control<'_>) -> MechanicsResult<Vec<[usize; 3]>> {
    let tol = 1e-12 * scale_of(points);
    let mut poly: Vec<usize> = outer.to_vec();
    let mut order: Vec<usize> = (0..holes.len()).collect();
    let right = |k: usize| holes[k].iter().fold(f64::MIN, |m, &i| m.max(points[i][0]));
    order.sort_by(|&a, &b| right(b).total_cmp(&right(a)));
    for pos in 0..order.len() {
        control.check()?;
        let hole = &holes[order[pos]];
        let mut edges: Vec<(usize, usize)> = (0..poly.len()).map(|i| (poly[i], poly[(i + 1) % poly.len()])).collect();
        for &other in &order[pos..] {
            let ring = &holes[other];
            edges.extend((0..ring.len()).map(|i| (ring[i], ring[(i + 1) % ring.len()])));
        }
        let mut count = vec![0_usize; points.len()];
        for &x in &poly {
            count[x] += 1;
        }
        let rest: Vec<usize> = order[pos..].iter().flat_map(|&o| holes[o].iter().copied()).collect();
        let mut best: Option<(f64, usize, usize)> = None;
        for (hp, &h) in hole.iter().enumerate() {
            for (vp, &v) in poly.iter().enumerate() {
                if count[v] > 1 {
                    continue;
                }
                let d = dist(points[h], points[v]);
                if d <= tol || best.is_some_and(|b| d >= b.0) {
                    continue;
                }
                let clear = edges.iter().all(|&(a, b)| !crosses(points[h], points[v], points[a], points[b]))
                    && poly.iter().chain(rest.iter()).all(|&c| c == v || c == h || !touches(points[h], points[v], points[c], tol));
                if clear {
                    best = Some((d, hp, vp));
                }
            }
        }
        let Some((_, hp, vp)) = best else {
            return invalid("a hole cannot be connected to the outer ring", "ein Loch lässt sich nicht mit dem äußeren Ring verbinden");
        };
        let mut merged: Vec<usize> = poly[..=vp].to_vec();
        merged.extend((0..hole.len()).map(|i| hole[(hp + i) % hole.len()]));
        merged.push(hole[hp]);
        merged.extend_from_slice(&poly[vp..]);
        poly = merged;
    }
    ear_clip(points, poly, control)
}

fn in_circle(a: P2, b: P2, c: P2, d: P2) -> f64 {
    let (ax, ay, bx, by, cx, cy) = (a[0] - d[0], a[1] - d[1], b[0] - d[0], b[1] - d[1], c[0] - d[0], c[1] - d[1]);
    (ax * ax + ay * ay) * (bx * cy - cx * by) - (bx * bx + by * by) * (ax * cy - cx * ay) + (cx * cx + cy * cy) * (ax * by - bx * ay)
}

fn flip(points: &[P2], triangles: &mut [[usize; 3]], control: &Control<'_>) -> MechanicsResult<()> {
    let eps = 1e-12 * scale_of(points).powi(4);
    for _ in 0..200 {
        control.check()?;
        let mut owners: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();
        for (ti, t) in triangles.iter().enumerate() {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                owners.entry((a.min(b), a.max(b))).or_default().push((ti, k));
            }
        }
        let mut dead = vec![false; triangles.len()];
        let mut changed = false;
        for owner in owners.values() {
            if owner.len() != 2 {
                continue;
            }
            let ((t1, k1), (t2, k2)) = (owner[0], owner[1]);
            if dead[t1] || dead[t2] {
                continue;
            }
            let (first, second) = (triangles[t1], triangles[t2]);
            let (a, b, c) = (first[k1], first[(k1 + 1) % 3], first[(k1 + 2) % 3]);
            let d = second[(k2 + 2) % 3];
            if in_circle(points[a], points[b], points[c], points[d]) > eps && orient(points[c], points[d], points[b]) > 0.0 && orient(points[d], points[c], points[a]) > 0.0 {
                triangles[t1] = [c, a, d];
                triangles[t2] = [d, b, c];
                dead[t1] = true;
                dead[t2] = true;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    Ok(())
}

fn longest(points: &[P2], t: [usize; 3]) -> usize {
    let mut best = 0;
    let mut key = (0.0_f64, 0_usize, 0_usize);
    for k in 0..3 {
        let (a, b) = (t[k], t[(k + 1) % 3]);
        let candidate = (dist(points[a], points[b]), a.min(b), a.max(b));
        if k == 0 || candidate.0 > key.0 || (candidate.0 == key.0 && (candidate.1, candidate.2) > (key.1, key.2)) {
            best = k;
            key = candidate;
        }
    }
    best
}

fn refine(points: &mut Vec<P2>, triangles: &mut Vec<[usize; 3]>, target: f64, cap: usize, control: &Control<'_>) -> MechanicsResult<()> {
    for round in 0..64 {
        control.check()?;
        let mut marked: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut work: Vec<(usize, usize)> = Vec::new();
        for &t in triangles.iter() {
            let k = longest(points, t);
            let (a, b) = (t[k], t[(k + 1) % 3]);
            if dist(points[a], points[b]) > target && marked.insert((a.min(b), a.max(b))) {
                work.push((a.min(b), a.max(b)));
            }
        }
        if marked.is_empty() {
            return Ok(());
        }
        let mut owners: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
        for (ti, t) in triangles.iter().enumerate() {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                owners.entry((a.min(b), a.max(b))).or_default().push(ti);
            }
        }
        while let Some(e) = work.pop() {
            for &ti in &owners[&e] {
                let t = triangles[ti];
                let k = longest(points, t);
                let (a, b) = (t[k], t[(k + 1) % 3]);
                if marked.insert((a.min(b), a.max(b))) {
                    work.push((a.min(b), a.max(b)));
                }
            }
        }
        let mut mid: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for &e in &marked {
            let m = [(points[e.0][0] + points[e.1][0]) / 2.0, (points[e.0][1] + points[e.1][1]) / 2.0];
            mid.insert(e, points.len());
            points.push(m);
        }
        let mut out: Vec<[usize; 3]> = Vec::with_capacity(triangles.len() * 2);
        for &t in triangles.iter() {
            let k = longest(points, t);
            let v = [t[k], t[(k + 1) % 3], t[(k + 2) % 3]];
            let find = |a: usize, b: usize| mid.get(&(a.min(b), a.max(b))).copied();
            let Some(m01) = find(v[0], v[1]) else {
                out.push(t);
                continue;
            };
            match find(v[2], v[0]) {
                Some(m20) => {
                    out.push([v[0], m01, m20]);
                    out.push([m01, v[2], m20]);
                }
                None => out.push([v[0], m01, v[2]]),
            }
            match find(v[1], v[2]) {
                Some(m12) => {
                    out.push([m01, v[1], m12]);
                    out.push([m01, m12, v[2]]);
                }
                None => out.push([m01, v[1], v[2]]),
            }
        }
        *triangles = out;
        control.report("mesh", 0.3 + 0.6 * (1.0 - 1.0 / (1.0 + round as f64)));
        if triangles.len() > 2 * cap {
            return Ok(());
        }
    }
    Err(MechanicsError::NoConvergence)
}

/// 📏️ The mesh edge length for a section: the mean wall thickness 2A/P divided by `cells_across_wall`, never below the length that keeps the triangle count under `max_triangles`, or the explicit `max_edge` when it is positive.
pub fn target_edge(rings: &[Vec<P2>], cells_across_wall: f64, max_edge: f64, max_triangles: usize) -> f64 {
    let area: f64 = rings.iter().map(|r| signed_area(r)).sum();
    let perimeter: f64 = rings.iter().map(|r| (0..r.len()).map(|i| dist(r[i], r[(i + 1) % r.len()])).sum::<f64>()).sum();
    let wanted = if max_edge > 0.0 { max_edge } else { 2.0 * area / perimeter / cells_across_wall.max(1.0) };
    wanted.max((6.0 * area / max_triangles.max(8) as f64).sqrt())
}

/// 🕸️ Mesh the normalised rings (see [`normalise`]) with edges no longer than `target`; the result is conforming and counter-clockwise.
pub fn generate(rings: &[Vec<P2>], target: f64, max_triangles: usize, control: &Control<'_>) -> MechanicsResult<Mesh> {
    let mut points: Vec<P2> = Vec::new();
    let mut indices: Vec<Vec<usize>> = Vec::new();
    for ring in rings {
        indices.push((points.len()..points.len() + ring.len()).collect());
        points.extend_from_slice(ring);
    }
    control.report("mesh", 0.0);
    let mut triangles = triangulate(&points, &indices[0], &indices[1..], control)?;
    flip(&points, &mut triangles, control)?;
    control.report("mesh", 0.3);
    refine(&mut points, &mut triangles, target, max_triangles, control)?;
    control.report("mesh", 0.9);
    flip(&points, &mut triangles, control)?;
    control.report("mesh", 1.0);
    Ok(Mesh { points, triangles })
}
