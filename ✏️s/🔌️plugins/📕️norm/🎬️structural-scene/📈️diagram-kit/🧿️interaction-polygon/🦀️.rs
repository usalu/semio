//! 🧿️ Interaction polygon: capacity points to a convex ring, point containment and the utilisation ray.

use crate::diagram_kit::Xy;

/// 🎯️ Where the ray from the origin through the demand leaves the ring and the resulting utilisation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayHit {
    pub boundary: Xy,
    pub utilisation: f64,
}

fn cross(o: Xy, a: Xy, b: Xy) -> f64 {
    (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
}

/// 🧿️ Counter-clockwise convex hull (monotone chain) of capacity points; collinear points are dropped.
pub fn interaction_polygon(points: &[Xy]) -> Vec<Xy> {
    let mut p: Vec<Xy> = points.to_vec();
    p.sort_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
    p.dedup();
    if p.len() < 3 {
        return p;
    }
    let mut hull: Vec<Xy> = Vec::with_capacity(p.len() * 2);
    for pass in 0..2 {
        let start = hull.len();
        let iter: Box<dyn Iterator<Item = &Xy>> = if pass == 0 { Box::new(p.iter()) } else { Box::new(p.iter().rev()) };
        for &q in iter {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], q) <= 0.0 {
                hull.pop();
            }
            hull.push(q);
        }
        hull.pop();
    }
    hull
}

/// 📐️ Signed shoelace area of a ring, positive when counter-clockwise.
pub fn signed_area(ring: &[Xy]) -> f64 {
    let n = ring.len();
    (0..n).map(|i| ring[i][0] * ring[(i + 1) % n][1] - ring[(i + 1) % n][0] * ring[i][1]).sum::<f64>() / 2.0
}

/// 📍️ Whether `p` lies inside or on a convex counter-clockwise ring.
pub fn contains(ring: &[Xy], p: Xy) -> bool {
    let n = ring.len();
    n >= 3 && (0..n).all(|i| cross(ring[i], ring[(i + 1) % n], p) >= -1e-12)
}

/// 🎯️ Utilisation U = |OA|/|OB| for demand A, origin O and the boundary hit B of the ray O→A; `None` for a zero demand or an origin outside the ring.
pub fn utilisation_ray(ring: &[Xy], origin: Xy, demand: Xy) -> Option<RayHit> {
    let d = [demand[0] - origin[0], demand[1] - origin[1]];
    let len = d[0].hypot(d[1]);
    if len < 1e-300 || !contains(ring, origin) {
        return None;
    }
    let n = ring.len();
    let mut best: Option<(f64, Xy)> = None;
    for i in 0..n {
        let (a, b) = (ring[i], ring[(i + 1) % n]);
        let e = [b[0] - a[0], b[1] - a[1]];
        let det = d[0] * e[1] - d[1] * e[0];
        if det.abs() < 1e-300 {
            continue;
        }
        let w = [a[0] - origin[0], a[1] - origin[1]];
        let t = (w[0] * e[1] - w[1] * e[0]) / det;
        let s = (w[0] * d[1] - w[1] * d[0]) / det;
        if t > 0.0 && (-1e-12..=1.0 + 1e-12).contains(&s) && best.is_none_or(|(bt, _)| t > bt) {
            best = Some((t, [origin[0] + t * d[0], origin[1] + t * d[1]]));
        }
    }
    best.map(|(t, boundary)| RayHit { boundary, utilisation: 1.0 / t })
}
