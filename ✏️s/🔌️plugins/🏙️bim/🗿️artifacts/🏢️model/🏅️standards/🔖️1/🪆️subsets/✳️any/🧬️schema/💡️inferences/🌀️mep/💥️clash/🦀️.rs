//! 💥️ Clashes between MEP elements of different systems on one storey. Two elements clash when the shortest distance between their centre lines is smaller than the sum of their reaches (half of the larger section dimension), so
//! touching elements and elements of the same system never clash. A sweep over the x extents of the (conservative) bounding boxes prunes the pairs, the boxes must overlap on y and z, and only then are the segments of the two
//! centre lines compared. Each pair is reported once, ordered by id. The scan is the one expensive step of the MEP diagnostics, which is why it is a node of its own per storey in the model graph: an edit that moves no MEP
//! element never repeats it.

use super::MepValue;
use semio_framework_geometry::vector::{add3, dot3, scale3, sub3, Xyz};

/// 📏️ A pair closer than the sum of the reaches by less than this (metres) only touches.
pub const TOUCH: f64 = 1e-9;

/// 💥️ One clashing pair: the ids (`a` before `b`), the shortest distance between their centre lines and the sum of their reaches.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct MepClash {
    pub a: String,
    pub b: String,
    pub distance: f64,
    pub reach: f64,
}

/// 💥️ The clashing pairs of one storey in the order of their ids.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct MepClashes {
    pub pairs: Vec<MepClash>,
}

/// 📏️ The shortest distance between the segments `p1 p2` and `q1 q2` (closest points of two clamped lines).
pub fn segment_distance(p1: Xyz, p2: Xyz, q1: Xyz, q2: Xyz) -> f64 {
    let (d1, d2, r) = (sub3(p2, p1), sub3(q2, q1), sub3(p1, q1));
    let (a, e, f) = (dot3(d1, d1), dot3(d2, d2), dot3(d2, r));
    let eps = 1e-18;
    let (s, t) = if a <= eps && e <= eps {
        (0.0, 0.0)
    } else if a <= eps {
        (0.0, (f / e).clamp(0.0, 1.0))
    } else {
        let c = dot3(d1, r);
        if e <= eps {
            ((-c / a).clamp(0.0, 1.0), 0.0)
        } else {
            let b = dot3(d1, d2);
            let denominator = a * e - b * b;
            let mut s = if denominator > eps { ((b * f - c * e) / denominator).clamp(0.0, 1.0) } else { 0.0 };
            let mut t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0);
            }
            (s, t)
        }
    };
    let (on_p, on_q) = (add3(p1, scale3(d1, s)), add3(q1, scale3(d2, t)));
    let gap = sub3(on_p, on_q);
    dot3(gap, gap).sqrt()
}

fn at(point: &crate::Point3) -> Xyz {
    [point.x, point.y, point.z]
}

/// 📏️ The shortest distance between the centre lines of two elements.
pub fn centre_distance(a: &MepValue, b: &MepValue) -> f64 {
    a.segments.iter().flat_map(|first| b.segments.iter().map(move |second| segment_distance(at(&first.from), at(&first.to), at(&second.from), at(&second.to)))).fold(f64::INFINITY, f64::min)
}

fn overlap(a: &MepValue, b: &MepValue) -> bool {
    a.bounds.min.y <= b.bounds.max.y && b.bounds.min.y <= a.bounds.max.y && a.bounds.min.z <= b.bounds.max.z && b.bounds.min.z <= a.bounds.max.z
}

/// 💥️ The clashes among the elements (id and value) of one storey.
pub fn clashes_of<'a>(elements: impl IntoIterator<Item = (&'a str, &'a MepValue)>) -> MepClashes {
    let mut rows: Vec<(&str, &MepValue)> = elements.into_iter().filter(|(_, value)| value.buildable()).collect();
    rows.sort_by(|a, b| a.1.bounds.min.x.total_cmp(&b.1.bounds.min.x).then_with(|| a.0.cmp(b.0)));
    let mut pairs = Vec::new();
    for (rank, (first_id, first)) in rows.iter().enumerate() {
        for (second_id, second) in &rows[rank + 1..] {
            if second.bounds.min.x > first.bounds.max.x {
                break;
            }
            if first.system == second.system || !overlap(first, second) {
                continue;
            }
            let reach = first.section.reach() + second.section.reach();
            let distance = centre_distance(first, second);
            if distance < reach - TOUCH {
                let (a, b) = if first_id <= second_id { (first_id, second_id) } else { (second_id, first_id) };
                pairs.push(MepClash { a: (*a).to_string(), b: (*b).to_string(), distance, reach });
            }
        }
    }
    pairs.sort_by(|x, y| x.a.cmp(&y.a).then_with(|| x.b.cmp(&y.b)));
    MepClashes { pairs }
}
