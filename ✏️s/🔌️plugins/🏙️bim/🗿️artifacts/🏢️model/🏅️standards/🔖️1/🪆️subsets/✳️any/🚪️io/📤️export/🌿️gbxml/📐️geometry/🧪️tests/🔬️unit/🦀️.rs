use super::*;

fn near(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

fn square() -> Vec<P3> {
    vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 0.0, 3.0], [0.0, 0.0, 3.0]]
}

#[test]
fn numbers_round_to_nine_decimals_and_never_to_negative_zero() {
    assert_eq!(round9(1.0 / 3.0), 0.333_333_333);
    assert!(round9(-1e-12).is_sign_positive());
    assert_eq!(round9(2.5), 2.5);
}

#[test]
fn a_building_turned_by_the_bearing_has_true_north_up() {
    let north = turned(&Vec3 { x: 0.0, y: 1.0, z: 2.0 }, std::f64::consts::FRAC_PI_2);
    assert!(near(north[0], 1.0) && near(north[1], 0.0) && near(north[2], 2.0), "{north:?}");
    let same = turned(&Vec3 { x: 3.0, y: 4.0, z: 0.0 }, 0.0);
    assert_eq!(same, [3.0, 4.0, 0.0]);
}

#[test]
fn the_area_vector_points_to_the_counter_clockwise_side_and_its_length_is_twice_the_area() {
    let floor_up = vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 3.0, 0.0], [0.0, 3.0, 0.0]];
    let vector = area_vector(&floor_up);
    assert!(near(vector[2], 12.0) && near(vector[0], 0.0) && near(vector[1], 0.0), "{vector:?}");
    assert!(near(area(&floor_up), 6.0));
    let wall = area_vector(&square());
    assert!(near(wall[1], -12.0), "a wall counter-clockwise seen from the south faces south: {wall:?}");
    let reversed: Vec<P3> = floor_up.iter().rev().copied().collect();
    assert!(near(area_vector(&reversed)[2], -12.0));
}

#[test]
fn a_south_wall_has_azimuth_180_tilt_90_up_vertical_and_right_east() {
    let frame = Frame::of(&square()).expect("a frame");
    assert!(near(frame.azimuth(), 180.0) && near(frame.tilt(), 90.0));
    assert!(near(frame.up[2], 1.0) && near(frame.right[0], 1.0), "{frame:?}");
}

#[test]
fn a_roof_faces_up_with_tilt_0_azimuth_0_and_north_as_up_and_a_floor_faces_down_with_tilt_180() {
    let roof = Frame::of(&[[0.0, 0.0, 3.0], [2.0, 0.0, 3.0], [2.0, 3.0, 3.0], [0.0, 3.0, 3.0]]).expect("roof");
    assert!(near(roof.tilt(), 0.0) && near(roof.azimuth(), 0.0) && near(roof.up[1], 1.0) && near(roof.right[0], 1.0));
    let floor = Frame::of(&[[0.0, 0.0, 0.0], [0.0, 3.0, 0.0], [2.0, 3.0, 0.0], [2.0, 0.0, 0.0]]).expect("floor");
    assert!(near(floor.tilt(), 180.0) && near(floor.azimuth(), 0.0) && near(floor.right[0], -1.0));
}

#[test]
fn a_degenerate_polygon_has_no_frame() {
    assert_eq!(Frame::of(&[[0.0; 3], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]]), None);
}

#[test]
fn the_rectangle_of_a_wall_is_its_extent_in_the_frame_with_the_corner_at_the_lower_left() {
    let polygon: Vec<P3> = vec![[1.0, 0.0, 0.5], [3.0, 0.0, 0.5], [3.0, 0.0, 3.5], [1.0, 0.0, 3.5]];
    let frame = Frame::of(&polygon).expect("frame");
    let rect = rect_in(&frame, &polygon);
    assert_eq!(rect.corner, [1.0, 0.0, 0.5]);
    assert!(near(rect.width, 2.0) && near(rect.height, 3.0));
}

#[test]
fn an_opening_sits_at_its_offset_from_the_corner_of_the_host() {
    let host: Vec<P3> = vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 0.0, 3.0], [0.0, 0.0, 3.0]];
    let frame = Frame::of(&host).expect("frame");
    let corner = rect_in(&frame, &host).corner;
    let window: Vec<P3> = vec![[1.0, 0.0, 0.9], [2.2, 0.0, 0.9], [2.2, 0.0, 2.1], [1.0, 0.0, 2.1]];
    let (offset, rect) = offset_in(&frame, corner, &window);
    assert!(near(offset[0], 1.0) && near(offset[1], 0.9), "{offset:?}");
    assert!(near(rect.width, 1.2) && near(rect.height, 1.2));
}

#[test]
fn a_triangle_has_a_bounding_rectangle_larger_than_its_area() {
    let triangle: Vec<P3> = vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [0.0, 3.0, 0.0]];
    let frame = Frame::of(&triangle).expect("frame");
    let rect = rect_in(&frame, &triangle);
    assert!(near(rect.width * rect.height, 12.0) && near(area(&triangle), 6.0));
}
