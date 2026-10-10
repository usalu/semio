use super::*;

#[test]
fn a_line_axis_is_as_long_as_its_chord() {
    let axis = Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 3.0, y: 4.0 } };
    assert!((axis_length(&axis) - 5.0).abs() < 1e-12);
}

#[test]
fn a_bulged_axis_is_an_arc_of_the_quarter_tangent_sweep() {
    let axis = Axis::Arc { start: Point2 { x: -1.0, y: 0.0 }, end: Point2 { x: 1.0, y: 0.0 }, bulge: 1.0 };
    assert!((axis_length(&axis) - std::f64::consts::PI).abs() < 1e-12);
    assert_eq!(mark(point(&Point2 { x: 1.5, y: -2.0 })), Point2 { x: 1.5, y: -2.0 });
}
