//! 🕳️ Openings near a column (EN 1992-1-1 6.4.2(3), Figure 6.14): the part of a control perimeter between the two tangents drawn from the centre of the loaded area to the outline of an opening within `6 d` of the column periphery is ineffective. Lengths in one unit (metres), column frame with the origin at the column centre, `x` along `c1`, `y` along `c2`; the free slab edges of an edge or corner column lie flush with the `+x` (and `+y`) column face. See [`super`].

use super::{Position, PI};

const CLEAR_FACTOR: f64 = 6.0;
const EPS: f64 = 1e-12;

/// 🕳️ Rectangular opening in the column frame: centre `(x, y)` and size `width` (along x) × `depth` (along y).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpeningRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub depth: f64,
}

/// 🚫 Why an opening cannot be cut out of a control perimeter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpeningRefusal {
    Encloses,
    NoEffectivePerimeter,
}

impl OpeningRefusal {
    /// 🏷️ EN and DE reason shown as the not-applicable explanation.
    pub fn label(self) -> (&'static str, &'static str) {
        match self {
            OpeningRefusal::Encloses => ("An opening contains the column centre; no tangents can be drawn (6.4.2(3))", "Eine Öffnung enthält den Stützenmittelpunkt; es lassen sich keine Tangenten ziehen (6.4.2(3))"),
            OpeningRefusal::NoEffectivePerimeter => ("The openings leave no effective control perimeter (6.4.2(3))", "Die Öffnungen lassen keinen wirksamen Rundschnitt übrig (6.4.2(3))"),
        }
    }
}

/// 🔭 One ray of the tangent wedge: unit direction from the column centre.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wedge {
    pub lo: (f64, f64),
    pub hi: (f64, f64),
}

impl Wedge {
    /// 🔭 Whether the point lies between the two tangents.
    pub fn contains(&self, p: (f64, f64)) -> bool {
        cross(self.lo, p) >= -EPS && cross(p, self.hi) >= -EPS && dot(self.lo, p) + dot(self.hi, p) > 0.0
    }
}

fn cross(a: (f64, f64), b: (f64, f64)) -> f64 {
    a.0 * b.1 - a.1 * b.0
}

fn dot(a: (f64, f64), b: (f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1
}

fn unit(a: (f64, f64)) -> (f64, f64) {
    let n = dot(a, a).sqrt().max(EPS);
    (a.0 / n, a.1 / n)
}

/// 📏 Clear distance between the column periphery (rectangle `c1 × c2`) and the opening; zero when they touch or overlap.
pub fn clear_distance(c1: f64, c2: f64, o: &OpeningRect) -> f64 {
    let gap = |centre: f64, half_column: f64, half_opening: f64| (centre.abs() - half_column - half_opening).max(0.0);
    gap(o.x, c1 / 2.0, o.width / 2.0).hypot(gap(o.y, c2 / 2.0, o.depth / 2.0))
}

/// 🔭 The tangents from the column centre to the outline of the opening: counter-clockwise lower and upper ray.
pub fn tangent_wedge(o: &OpeningRect) -> Result<Wedge, OpeningRefusal> {
    let (hx, hy) = (o.width / 2.0, o.depth / 2.0);
    if o.x.abs() < hx + EPS && o.y.abs() < hy + EPS {
        return Err(OpeningRefusal::Encloses);
    }
    let axis = unit((o.x, o.y));
    let corners = [(o.x - hx, o.y - hy), (o.x + hx, o.y - hy), (o.x + hx, o.y + hy), (o.x - hx, o.y + hy)];
    let angle = |p: (f64, f64)| cross(axis, p).atan2(dot(axis, p));
    let (lo, hi) = corners.iter().fold((f64::MAX, f64::MIN), |acc, &p| (acc.0.min(angle(p)), acc.1.max(angle(p))));
    let ray = |rel: f64| (axis.0 * rel.cos() - axis.1 * rel.sin(), axis.0 * rel.sin() + axis.1 * rel.cos());
    Ok(Wedge { lo: ray(lo), hi: ray(hi) })
}

/// 🔭 Wedges of the openings that lie within `6 d` of the column periphery.
pub fn effective_wedges(c1: f64, c2: f64, d: f64, openings: &[OpeningRect]) -> Result<Vec<Wedge>, OpeningRefusal> {
    openings.iter().filter(|o| clear_distance(c1, c2, o) <= CLEAR_FACTOR * d).map(tangent_wedge).collect()
}

#[derive(Clone, Copy)]
enum Piece {
    Segment((f64, f64), (f64, f64)),
    Arc((f64, f64), f64, f64, f64),
}

fn pieces(position: Position, c1: f64, c2: f64, r: f64) -> Vec<Piece> {
    let (hx, hy) = (c1 / 2.0, c2 / 2.0);
    let top = Piece::Segment((-hx, hy + r), (hx, hy + r));
    let bottom = Piece::Segment((-hx, -hy - r), (hx, -hy - r));
    let left = Piece::Segment((-hx - r, -hy), (-hx - r, hy));
    let right = Piece::Segment((hx + r, -hy), (hx + r, hy));
    let arc = |cx: f64, cy: f64, quarter: f64| Piece::Arc((cx, cy), r, quarter * PI / 2.0, (quarter + 1.0) * PI / 2.0);
    match position {
        Position::Interior => vec![top, bottom, left, right, arc(hx, hy, 0.0), arc(-hx, hy, 1.0), arc(-hx, -hy, 2.0), arc(hx, -hy, 3.0)],
        Position::Edge | Position::WallEnd => vec![top, bottom, left, arc(-hx, hy, 1.0), arc(-hx, -hy, 2.0)],
        Position::Corner => vec![bottom, left, arc(-hx, -hy, 2.0)],
    }
}

fn piece_length(piece: &Piece) -> f64 {
    match *piece {
        Piece::Segment(p, q) => (q.0 - p.0).hypot(q.1 - p.1),
        Piece::Arc(_, r, a0, a1) => r * (a1 - a0),
    }
}

fn at(piece: &Piece, t: f64) -> (f64, f64) {
    match *piece {
        Piece::Segment(p, q) => (p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)),
        Piece::Arc(c, r, a0, a1) => {
            let a = a0 + t * (a1 - a0);
            (c.0 + r * a.cos(), c.1 + r * a.sin())
        }
    }
}

fn crossings(piece: &Piece, ray: (f64, f64)) -> Vec<f64> {
    match *piece {
        Piece::Segment(p, q) => {
            let (a, b) = (cross(ray, p), cross(ray, q));
            if (a - b).abs() < EPS {
                return vec![];
            }
            let t = a / (a - b);
            if (0.0..=1.0).contains(&t) && dot(ray, at(piece, t)) > 0.0 { vec![t] } else { vec![] }
        }
        Piece::Arc(c, r, a0, a1) => {
            let b = dot(ray, c);
            let disc = b * b - dot(c, c) + r * r;
            if disc < 0.0 {
                return vec![];
            }
            let root = disc.sqrt();
            [b - root, b + root]
                .into_iter()
                .filter(|s| *s > 0.0)
                .filter_map(|s| {
                    let p = (ray.0 * s - c.0, ray.1 * s - c.1);
                    let mut a = p.1.atan2(p.0);
                    while a < a0 - EPS {
                        a += 2.0 * PI;
                    }
                    while a > a1 + EPS {
                        a -= 2.0 * PI;
                    }
                    ((a0 - EPS)..=(a1 + EPS)).contains(&a).then(|| ((a - a0) / (a1 - a0)).clamp(0.0, 1.0))
                })
                .collect()
        }
    }
}

/// 📏 Total length of the perimeter at the offset `r` from the column face, and the length of it between the tangents of `wedges` (ineffective).
pub fn perimeter_lengths(position: Position, c1: f64, c2: f64, r: f64, wedges: &[Wedge]) -> (f64, f64) {
    let mut total = 0.0;
    let mut lost = 0.0;
    for piece in pieces(position, c1, c2, r) {
        let length = piece_length(&piece);
        total += length;
        let mut cuts: Vec<f64> = vec![0.0, 1.0];
        for w in wedges {
            cuts.extend(crossings(&piece, w.lo));
            cuts.extend(crossings(&piece, w.hi));
        }
        cuts.sort_by(f64::total_cmp);
        lost += cuts.windows(2).filter(|w| w[1] > w[0] && wedges.iter().any(|wedge| wedge.contains(at(&piece, 0.5 * (w[0] + w[1]))))).map(|w| (w[1] - w[0]) * length).sum::<f64>();
    }
    (total, lost)
}

/// 📏 Effective length of the perimeter at the offset `r`: the full length less the part between the tangents; refuses when nothing remains.
pub fn effective_perimeter(position: Position, c1: f64, c2: f64, r: f64, wedges: &[Wedge]) -> Result<f64, OpeningRefusal> {
    let (total, lost) = perimeter_lengths(position, c1, c2, r, wedges);
    if total - lost <= 1e-9 { Err(OpeningRefusal::NoEffectivePerimeter) } else { Ok(total - lost) }
}

/// 📐 Offset from the column face at which the effective perimeter has the length `u` (bisection; the effective length grows with the offset); negative when `u` is shorter than the effective column perimeter.
pub fn offset_of_effective_perimeter(position: Position, c1: f64, c2: f64, u: f64, wedges: &[Wedge]) -> f64 {
    let eff = |a: f64| { let (t, l) = perimeter_lengths(position, c1, c2, a, wedges); t - l };
    if u < eff(0.0) {
        return (u - eff(0.0)) / (eff(1.0) - eff(0.0)).max(1e-9);
    }
    let mut hi = 1.0;
    while eff(hi) < u && hi < 1.0e4 {
        hi *= 2.0;
    }
    let mut lo = 0.0;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if eff(mid) < u { lo = mid } else { hi = mid }
    }
    0.5 * (lo + hi)
}
