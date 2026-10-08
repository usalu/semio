use super::*;
use std::collections::BTreeMap;

#[path = "../../../🧪️tests/🧰️oracle-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    support::assert_category("math.vector", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert!(support::run_fixture(FIXTURE, COMPUTES) >= 38);
}

fn samples() -> Vec<Triple> {
    let mut state = 0x2545_f491_4f6c_dd1du64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0 - 10.0
    };
    (0..200).map(|_| [next(), next(), next()]).collect()
}

#[test]
fn normalized_vectors_have_unit_length_and_cross_products_are_perpendicular() {
    let samples = samples();
    for pair in samples.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let unit_a = unit(a).expect("random vectors are not zero");
        assert!((length(unit_a) - 1.0).abs() < 1e-12);
        let normal = cross(a, b);
        assert!(dot(normal, a).abs() < 1e-9 * length(normal).max(1.0) * length(a));
        assert!(dot(normal, b).abs() < 1e-9 * length(normal).max(1.0) * length(b));
    }
}

#[test]
fn the_angle_is_symmetric_scale_free_and_inside_zero_to_pi() {
    let samples = samples();
    for pair in samples.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let angle = angle_between(a, b).expect("angle");
        assert!((0.0..=std::f64::consts::PI).contains(&angle));
        assert!((angle - angle_between(b, a).expect("angle")).abs() < 1e-12);
        assert!((angle - angle_between(scale(a, 3.5), scale(b, 0.25)).expect("angle")).abs() < 1e-12);
    }
}

#[test]
fn a_plane_through_three_points_contains_them_and_has_a_unit_normal() {
    let kind = support::kind_of("math.planeFromPoints");
    let start = COMPUTES.iter().find(|entry| entry.id == kind.id).expect("registered").start;
    let samples = samples();
    for triple in samples.chunks_exact(3) {
        let values = BTreeMap::from([("a".to_string(), GeometryValue::Point(triple[0])), ("b".to_string(), GeometryValue::Point(triple[1])), ("c".to_string(), GeometryValue::Point(triple[2]))]);
        let evaluation = support::drive(start(kind, WidgetInputs::new("w", kind, values)), 1);
        let Some(GeometryValue::Plane(plane)) = evaluation.outputs.get("plane") else { panic!("a plane: {evaluation:?}") };
        assert!((length(plane.normal) - 1.0).abs() < 1e-12);
        for point in triple {
            assert!(dot(plane.normal, sub(*point, plane.origin)).abs() < 1e-9);
        }
    }
}
