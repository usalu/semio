//! 🧪️ Arcs as cubics: the curves stay on the circle of the bulge to a few parts in ten thousand, chain from the start to the end of the segment and bulge to the side the sign says.

use super::*;

fn at(start: Point, cubic: &Cubic, t: f64) -> Point {
    let (a, b, c) = (cubic.0, cubic.1, cubic.2);
    let u = 1.0 - t;
    let blend = |s: f64, p: f64, q: f64, r: f64| u * u * u * s + 3.0 * u * u * t * p + 3.0 * u * t * t * q + t * t * t * r;
    (blend(start.0, a.0, b.0, c.0), blend(start.1, a.1, b.1, c.1))
}

fn samples(from: Point, cubics: &[Cubic]) -> Vec<Point> {
    let mut start = from;
    let mut out = Vec::new();
    for cubic in cubics {
        out.extend((0..=16).map(|step| at(start, cubic, f64::from(step) / 16.0)));
        start = cubic.2;
    }
    out
}

fn circle(from: Point, to: Point, bulge: f64) -> (Point, f64) {
    let sweep = 4.0 * bulge.atan();
    let chord = (to.0 - from.0).hypot(to.1 - from.1);
    let radius = chord / (2.0 * (sweep / 2.0).sin().abs());
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let offset = chord / 2.0 / (sweep / 2.0).tan();
    (((from.0 + to.0) / 2.0 - dy / chord * offset, (from.1 + to.1) / 2.0 + dx / chord * offset), radius)
}

#[test]
fn a_zero_bulge_or_a_chord_without_length_is_one_straight_segment() {
    let straight = arc_cubics((0.0, 0.0), (3.0, 0.0), 0.0);
    assert_eq!(straight, vec![((1.0, 0.0), (2.0, 0.0), (3.0, 0.0))]);
    assert_eq!(arc_cubics((1.0, 1.0), (1.0, 1.0), 0.5).len(), 1);
}

#[test]
fn every_point_of_every_arc_lies_on_its_circle() {
    for bulge in [0.05, 0.2, 0.4142135623730951, 1.0, 2.0, 3.0, -0.3, -1.0, -2.5] {
        let (from, to) = ((1.0, 2.0), (4.0, 6.0));
        let (centre, radius) = circle(from, to, bulge);
        for (x, y) in samples(from, &arc_cubics(from, to, bulge)) {
            let distance = (x - centre.0).hypot(y - centre.1);
            assert!((distance - radius).abs() <= 3e-4 * radius, "bulge {bulge}: {distance} against {radius}");
        }
    }
}

#[test]
fn the_cubics_end_on_the_end_point_and_use_at_most_quarter_circles() {
    for (bulge, parts) in [(0.1, 1), (0.4142135623730951, 1), (0.5, 2), (1.0, 2), (2.0, 3), (3.0, 4), (-1.0, 2)] {
        let cubics = arc_cubics((0.0, 0.0), (2.0, 0.0), bulge);
        assert_eq!(cubics.len(), parts, "bulge {bulge}");
        assert_eq!(cubics.last().unwrap().2, (2.0, 0.0));
    }
}

#[test]
fn a_positive_bulge_swings_to_the_right_of_the_chord_in_a_y_up_plan_and_a_negative_one_to_the_left() {
    let positive = samples((0.0, 0.0), &arc_cubics((0.0, 0.0), (2.0, 0.0), 0.5));
    let negative = samples((0.0, 0.0), &arc_cubics((0.0, 0.0), (2.0, 0.0), -0.5));
    assert!(positive.iter().all(|point| point.1 <= 1e-9) && positive.iter().any(|point| point.1 < -0.1), "{positive:?}");
    assert!(negative.iter().all(|point| point.1 >= -1e-9) && negative.iter().any(|point| point.1 > 0.1), "{negative:?}");
}

#[test]
fn a_semicircle_reaches_its_radius_at_the_middle() {
    let cubics = arc_cubics((0.0, 0.0), (2.0, 0.0), 1.0);
    let lowest = samples((0.0, 0.0), &cubics).into_iter().map(|point| point.1).fold(f64::INFINITY, f64::min);
    assert!((lowest + 1.0).abs() < 3e-4, "{lowest}");
}
