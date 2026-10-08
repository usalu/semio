use super::*;

fn close(a: [f64; 3], b: [f64; 3]) -> bool {
    (0..3).all(|index| (a[index] - b[index]).abs() < 1e-9)
}

#[test]
fn axes_follow_the_ifc_defaults_and_stay_right_handed() {
    let plain = Rigid::from_axes([1.0, 2.0, 3.0], None, None);
    assert_eq!((plain.x, plain.y, plain.z), ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]));
    let facing = Rigid::from_axes([0.0; 3], Some([0.0, -1.0, 0.0]), Some([1.0, 0.0, 0.0]));
    assert!(close(facing.y, [0.0, 0.0, 1.0]), "a filling turned to face the wall keeps +z up as its y axis");
    let tilted = Rigid::from_axes([0.0; 3], Some([0.0, 0.0, 1.0]), Some([1.0, 0.0, 5.0]));
    assert!(close(tilted.x, [1.0, 0.0, 0.0]), "the reference direction is made orthogonal to the axis");
}

#[test]
fn a_placement_composed_with_its_inverse_is_the_identity() {
    let placement = Rigid::from_axes([4.0, -2.0, 1.5], Some([0.0, 0.6, 0.8]), Some([1.0, 0.0, 0.0]));
    let round = placement.transformed_by(&placement.inverse());
    assert!(close(round.origin, [0.0; 3]) && close(round.x, [1.0, 0.0, 0.0]) && close(round.y, [0.0, 1.0, 0.0]) && close(round.z, [0.0, 0.0, 1.0]));
    let point = [0.3, 0.4, 0.5];
    assert!(close(placement.inverse().point(placement.point(point)), point));
}

#[test]
fn a_child_placement_is_found_again_relative_to_its_parent() {
    let building = Rigid::from_axes([2.0, 3.0, 0.5], None, Some([0.1f64.cos(), 0.1f64.sin(), 0.0]));
    let wall = Rigid::from_axes([8.0, 0.0, 0.2], None, Some([0.0, 1.0, 0.0]));
    let in_world = wall.transformed_by(&building);
    let back = in_world.relative_to(&building);
    assert!(close(back.origin, wall.origin) && close(back.x, wall.x));
    assert!((in_world.heading() - (0.1 + std::f64::consts::FRAC_PI_2)).abs() < 1e-6, "the heading is snapped to a micro-radian");
}

#[test]
fn a_bulge_comes_back_from_the_circle_centre_and_the_sense() {
    let bulge = 0.5;
    let sweep = 4.0 * f64::atan(bulge);
    let radius = 4.0 / (2.0 * (sweep / 2.0).sin());
    let centre = [2.0, radius * (sweep / 2.0).cos()];
    assert!((bulge_of(centre, [0.0, 0.0], [4.0, 0.0], true) - bulge).abs() < 1e-9);
    let clockwise = [2.0, -radius * (sweep / 2.0).cos()];
    assert!((bulge_of(clockwise, [0.0, 0.0], [4.0, 0.0], false) + bulge).abs() < 1e-9);
    assert!(bulge_of([2.0, 0.0], [0.0, 0.0], [4.0, 0.0], true) - 1.0 < 1e-9, "a half circle has bulge 1");
}
