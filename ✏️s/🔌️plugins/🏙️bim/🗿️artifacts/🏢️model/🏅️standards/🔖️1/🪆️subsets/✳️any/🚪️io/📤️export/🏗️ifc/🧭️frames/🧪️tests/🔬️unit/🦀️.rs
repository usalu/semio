use super::*;
use crate::Vertex;

fn corner(x: f64, y: f64) -> Vertex {
    Vertex { point: Point2 { x, y }, bulge: 0.0 }
}

fn square() -> Vec<Vertex> {
    vec![corner(0.0, 0.0), corner(2.0, 0.0), corner(2.0, 2.0), corner(0.0, 2.0)]
}

#[test]
fn loops_are_normalised_to_the_requested_orientation() {
    let clockwise: Vec<Vertex> = square().into_iter().rev().collect();
    let signed = |pairs: &[([f64; 2], f64)]| pairs.iter().enumerate().map(|(index, (a, _))| a[0] * pairs[(index + 1) % pairs.len()].0[1] - pairs[(index + 1) % pairs.len()].0[0] * a[1]).sum::<f64>() / 2.0;
    assert!(signed(&ccw(&clockwise)) > 0.0);
    assert!(signed(&ccw(&square())) > 0.0);
    assert!(signed(&cw(&square())) < 0.0);
    assert!(signed(&cw(&clockwise)) < 0.0);
}

#[test]
fn area_and_perimeter_are_exact_for_arcs() {
    assert!((area(&square()) - 4.0).abs() < 1e-12);
    assert!((perimeter(&square()) - 8.0).abs() < 1e-12);
    let half_disc = vec![Vertex { point: Point2 { x: -1.0, y: 0.0 }, bulge: 1.0 }, corner(1.0, 0.0)];
    assert!((area(&half_disc) - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    assert!((perimeter(&half_disc) - (std::f64::consts::PI + 2.0)).abs() < 1e-12);
}

#[test]
fn a_frame_on_an_axis_gives_the_point_and_the_unit_tangent() {
    let line = BulgeSeg::line(Point::new(0.0, 0.0), Point::new(4.0, 0.0));
    assert_eq!(frame_at(&line, 1.0), ([1.0, 0.0], [1.0, 0.0]));
    let quarter = BulgeSeg::new(Point::new(1.0, 0.0), Point::new(0.0, 1.0), (std::f64::consts::FRAC_PI_2 / 4.0).tan());
    let (point, tangent) = frame_at(&quarter, quarter.length() / 2.0);
    assert!((point[0].hypot(point[1]) - 1.0).abs() < 1e-9, "the point stays on the unit circle");
    assert!((tangent[0] * point[0] + tangent[1] * point[1]).abs() < 1e-9, "the tangent is perpendicular to the radius");
}

#[test]
fn the_highest_point_of_a_profile_is_half_its_depth_or_its_highest_vertex() {
    let i_shape = Profile::IShape { width: 0.2, depth: 0.4, web: 0.01, flange: 0.02 };
    assert_eq!(profile_top(&i_shape), 0.2);
    assert_eq!(profile_top(&Profile::Custom { outline: square() }), 2.0);
}
