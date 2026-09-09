use super::*;

#[semio_framework_async_macros::async_test]
async fn face_drag_negative_distance_yields_cut() {
    let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
    assert!(matches!(step.measure, ProcessMeasure::Cut { .. }));
    assert_eq!(step.label, "Push Cut");
}

#[semio_framework_async_macros::async_test]
async fn face_drag_positive_distance_yields_attach() {
    let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
    assert!(matches!(step.measure, ProcessMeasure::Attach { .. }));
    assert_eq!(step.label, "Pull Attach");
}

#[semio_framework_async_macros::async_test]
async fn face_drag_zero_distance_is_noop() {
    assert!(process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0, None, &Process3dLabels::NATIVE_EN).is_none());
}
