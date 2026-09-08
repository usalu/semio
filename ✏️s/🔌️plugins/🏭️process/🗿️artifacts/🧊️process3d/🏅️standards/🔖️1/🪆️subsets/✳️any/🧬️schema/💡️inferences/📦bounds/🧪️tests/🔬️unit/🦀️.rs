
use super::*;
use crate::{WorkingSolid, brep_snapshot_for_working_solid};

#[semio_framework_async_macros::async_test]
async fn default_box_stock_bounds_are_unit_cube() {
    let solid = brep_snapshot_for_working_solid(&WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 });
    let bounds = brep_bounding_box(&solid, &Pose::default());
    assert_eq!(bounds.min, [0.0, 0.0, 0.0]);
    assert_eq!(bounds.max, [1.0, 1.0, 1.0]);
}

#[semio_framework_async_macros::async_test]
async fn translated_box_shifts_bounds() {
    let solid = brep_snapshot_for_working_solid(&WorkingSolid::Box { width: 2.0, depth: 1.0, height: 1.0 });
    let pose = Pose { position: [5.0, 0.0, 0.0], axis: [0.0, 0.0, 1.0], angle: 0.0 };
    let bounds = brep_bounding_box(&solid, &pose);
    assert_eq!(bounds.min, [5.0, 0.0, 0.0]);
    assert_eq!(bounds.max, [7.0, 1.0, 1.0]);
}

#[semio_framework_async_macros::async_test]
async fn sphere_bounds_degenerate_to_pose_position() {
    let solid = brep_snapshot_for_working_solid(&WorkingSolid::Sphere { radius: 2.0 });
    let pose = Pose { position: [3.0, 4.0, 5.0], axis: [0.0, 0.0, 1.0], angle: 0.0 };
    let bounds = brep_bounding_box(&solid, &pose);
    assert_eq!(bounds.min, [3.0, 4.0, 5.0]);
    assert_eq!(bounds.max, [3.0, 4.0, 5.0]);
}

#[semio_framework_async_macros::async_test]
async fn imported_placeholder_degenerates_to_a_point() {
    let solid = brep_snapshot_for_working_solid(&WorkingSolid::ImportedSolid { solid_handle: "h1".into() });
    let pose = Pose { position: [3.0, 4.0, 5.0], axis: [0.0, 0.0, 1.0], angle: 0.0 };
    let bounds = brep_bounding_box(&solid, &pose);
    assert_eq!(bounds.min, [3.0, 4.0, 5.0]);
    assert_eq!(bounds.max, [3.0, 4.0, 5.0]);
}
