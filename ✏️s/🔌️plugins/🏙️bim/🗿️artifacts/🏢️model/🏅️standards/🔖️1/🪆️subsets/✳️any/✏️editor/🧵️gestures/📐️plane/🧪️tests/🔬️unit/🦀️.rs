use super::*;

#[semio_framework_async_macros::async_test]
async fn the_bulge_through_three_points_is_signed_by_the_turn_and_collinear_points_have_none() {
    let ccw = bulge_through([0.0, 0.0], [2.0, -1.0], [4.0, 0.0]).expect("an arc");
    let cw = bulge_through([0.0, 0.0], [2.0, 1.0], [4.0, 0.0]).expect("an arc");
    assert!((ccw - 0.5).abs() < 1e-9 && (cw + 0.5).abs() < 1e-9);
    assert_eq!(bulge_through([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]), None);
}

#[semio_framework_async_macros::async_test]
async fn a_negligible_bulge_is_a_line_and_the_ends_read_back() {
    let line = axis_of([0.0, 0.0], [3.0, 4.0], 1e-12);
    assert!(matches!(line, Axis::Line { .. }));
    assert_eq!(axis_ends(&line), ([0.0, 0.0], [3.0, 4.0]));
    assert_eq!(axis_bulge(&axis_of([0.0, 0.0], [3.0, 4.0], 0.25)), 0.25);
    assert!((axis_length(&line) - 5.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn projecting_onto_an_axis_gives_the_arc_length_the_foot_and_the_side() {
    let axis = axis_of([0.0, 0.0], [8.0, 0.0], 0.0);
    let left = project(&axis, [3.0, 0.5]);
    assert_eq!((left.offset, left.point, left.distance), (3.0, [3.0, 0.0], 0.5));
    assert!(left.side > 0.0 && project(&axis, [3.0, -0.5]).side < 0.0);
    assert_eq!(project(&axis, [20.0, 0.0]).offset, 8.0, "beyond the end the foot is the end");
}

#[semio_framework_async_macros::async_test]
async fn rings_are_made_counter_clockwise_and_rectangles_span_their_corners() {
    let clockwise = vec![[0.0, 0.0], [0.0, 4.0], [5.0, 4.0], [5.0, 0.0]];
    assert!(signed_area(&clockwise) < 0.0);
    let ccw = counter_clockwise(clockwise);
    assert_eq!(signed_area(&ccw), 20.0);
    assert_eq!(rectangle([5.0, 4.0], [0.0, 0.0]), vec![[0.0, 0.0], [5.0, 0.0], [5.0, 4.0], [0.0, 4.0]]);
}

#[semio_framework_async_macros::async_test]
async fn rotation_translation_and_quantising_behave() {
    let turned = rotate_about([1.0, 0.0], [0.0, 0.0], std::f64::consts::FRAC_PI_2);
    assert!(turned[0].abs() < 1e-12 && (turned[1] - 1.0).abs() < 1e-12);
    assert_eq!(translate([1.0, 2.0], [3.0, -2.0]), [4.0, 0.0]);
    assert!((quantised(0.3, std::f64::consts::PI / 12.0) - std::f64::consts::PI / 12.0).abs() < 1e-12);
    assert_eq!(bounds([[1.0, 5.0], [-2.0, 3.0]]), Some([-2.0, 3.0, 1.0, 5.0]));
    assert_eq!(bounds(std::iter::empty()), None);
}
