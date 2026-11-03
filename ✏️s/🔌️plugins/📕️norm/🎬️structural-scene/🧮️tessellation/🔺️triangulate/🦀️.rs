//! 🔺️ Polygon-with-holes triangulation: hole bridging (Eberly) followed by ear clipping; deterministic and allocation-light.

use super::vector::Q;

pub fn area2(ring: &[Q]) -> f64 {
    (0..ring.len()).map(|i| {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        a[0] * b[1] - b[0] * a[1]
    }).sum()
}

pub fn oriented(ring: &[Q], ccw: bool) -> Vec<Q> {
    let mut out = ring.to_vec();
    if (area2(ring) > 0.0) != ccw {
        out.reverse();
    }
    out
}

fn cross(o: Q, a: Q, b: Q) -> f64 {
    (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
}

fn side(a: Q, b: Q, p: Q) -> f64 {
    cross(a, b, p) / (b[0] - a[0]).hypot(b[1] - a[1]).max(1e-300)
}

fn inside(a: Q, b: Q, c: Q, p: Q) -> bool {
    side(a, b, p) >= -1e-10 && side(b, c, p) >= -1e-10 && side(c, a, p) >= -1e-10
}

fn turn(u: Q, v: Q) -> f64 {
    u[0] * v[1] - u[1] * v[0]
}

fn locally_inside(points: &[Q], poly: &[usize], k: usize, m: Q) -> bool {
    let n = poly.len();
    let (v, a, b) = (points[poly[k]], points[poly[(k + n - 1) % n]], points[poly[(k + 1) % n]]);
    let (out, back, d) = ([b[0] - v[0], b[1] - v[1]], [a[0] - v[0], a[1] - v[1]], [m[0] - v[0], m[1] - v[1]]);
    if cross(a, v, b) > 0.0 {
        turn(out, d) >= 0.0 && turn(d, back) >= 0.0
    } else {
        !(turn(back, d) > 0.0 && turn(d, out) > 0.0)
    }
}

fn refine(points: &[Q], poly: &[usize], k: usize, m: Q) -> usize {
    let at = points[poly[k]];
    (0..poly.len()).find(|&j| points[poly[j]] == at && locally_inside(points, poly, j, m)).unwrap_or(k)
}

fn bridge(points: &[Q], poly: &[usize], hole: &[usize]) -> (usize, usize) {
    let m = *hole.iter().enumerate().max_by(|a, b| points[*a.1][0].partial_cmp(&points[*b.1][0]).unwrap().then(b.0.cmp(&a.0))).unwrap().1;
    let mp = points[m];
    let mut best: Option<(f64, usize)> = None;
    for k in 0..poly.len() {
        let (a, b) = (points[poly[k]], points[poly[(k + 1) % poly.len()]]);
        if (a[1] - mp[1]) * (b[1] - mp[1]) > 0.0 || a[1] == b[1] {
            continue;
        }
        let x = a[0] + (mp[1] - a[1]) / (b[1] - a[1]) * (b[0] - a[0]);
        if x >= mp[0] && best.is_none_or(|(bx, _)| x < bx) {
            let pick = if a[0] > b[0] { k } else { (k + 1) % poly.len() };
            best = Some((x, pick));
        }
    }
    let (ix, pk) = best.unwrap_or((mp[0], 0));
    let mut chosen = pk;
    let p = points[poly[pk]];
    let hit = [ix, mp[1]];
    let tri = if cross(mp, hit, p) >= 0.0 { [mp, hit, p] } else { [mp, p, hit] };
    let degenerate = (p[0] - ix).abs() < 1e-12 && (p[1] - mp[1]).abs() < 1e-12;
    let mut best_angle = f64::MAX;
    for (k, &idx) in poly.iter().enumerate() {
        let c = points[idx];
        if k == pk || c == p || degenerate || !inside(tri[0], tri[1], tri[2], c) {
            continue;
        }
        let angle = ((c[1] - mp[1]).abs() / (c[0] - mp[0]).max(1e-300)).abs();
        if angle < best_angle {
            best_angle = angle;
            chosen = k;
        }
    }
    (refine(points, poly, chosen, mp), hole.iter().position(|&h| h == m).unwrap())
}

/// 🔺️ Triangulates `outer` minus `holes` (any winding); returns the combined point list and index triples wound counter-clockwise.
pub fn triangulate(outer: &[Q], holes: &[Vec<Q>]) -> (Vec<Q>, Vec<[usize; 3]>) {
    let mut points = oriented(outer, true);
    let n = points.len();
    let mut poly: Vec<usize> = (0..n).collect();
    let mut hole_rings: Vec<Vec<usize>> = Vec::new();
    for h in holes {
        let ring = oriented(h, false);
        let start = points.len();
        points.extend(ring);
        hole_rings.push((start..points.len()).collect());
    }
    hole_rings.sort_by(|a, b| {
        let mx = |r: &Vec<usize>| r.iter().map(|&i| points[i][0]).fold(f64::MIN, f64::max);
        mx(b).partial_cmp(&mx(a)).unwrap()
    });
    for hole in &hole_rings {
        let (pk, hk) = bridge(&points, &poly, hole);
        let mut merged = poly[..=pk].to_vec();
        merged.extend((0..=hole.len()).map(|i| hole[(hk + i) % hole.len()]));
        merged.push(poly[pk]);
        merged.extend_from_slice(&poly[pk + 1..]);
        poly = merged;
    }
    let mut tris = Vec::with_capacity(poly.len());
    let mut guard = poly.len() * poly.len() + 8;
    while poly.len() > 3 && guard > 0 {
        guard -= 1;
        let m = poly.len();
        let ear = (0..m).find(|&i| {
            let (a, b, c) = (points[poly[(i + m - 1) % m]], points[poly[i]], points[poly[(i + 1) % m]]);
            cross(a, b, c) > 0.0 && poly.iter().all(|&j| {
                let d = points[j];
                d == a || d == b || d == c || !inside(a, b, c, d)
            })
        });
        let i = ear.or_else(|| (0..m).find(|&i| cross(points[poly[(i + m - 1) % m]], points[poly[i]], points[poly[(i + 1) % m]]) <= 1e-18)).unwrap_or(0);
        let (a, b, c) = (poly[(i + m - 1) % m], poly[i], poly[(i + 1) % m]);
        if cross(points[a], points[b], points[c]) > 1e-18 {
            tris.push([a, b, c]);
        }
        poly.remove(i);
    }
    if poly.len() == 3 && cross(points[poly[0]], points[poly[1]], points[poly[2]]) > 1e-18 {
        tris.push([poly[0], poly[1], poly[2]]);
    }
    (points, tris)
}
