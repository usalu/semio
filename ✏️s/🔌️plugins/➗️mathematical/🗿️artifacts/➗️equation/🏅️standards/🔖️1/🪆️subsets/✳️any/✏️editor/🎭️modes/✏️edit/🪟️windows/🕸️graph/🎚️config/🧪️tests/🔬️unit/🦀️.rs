use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn math_config_dsl_round_trips() {
    let config = EquationGraphWindowConfig { camera: EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 } };
    store::os_store::test_support::assert_dsl_round_trip(&config);
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn config_operation_set_camera_diff_writes_the_targeted_field() {
    let base = EquationGraphWindowConfig::default();
    let camera = EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 };
    let operation = EquationGraphWindowConfigMutation::SetCamera(SetCamera { camera: camera.clone() });
    assert_eq!(Mutation::diff(&operation, &base).diff().camera, camera);
}

#[semio_framework_async_macros::async_test]
async fn config_operation_set_camera_round_trips() {
    let base = EquationGraphWindowConfig::default();
    let camera = EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 };
    let operation = EquationGraphWindowConfigMutation::SetCamera(SetCamera { camera: camera.clone() });
    let next = Mutation::diff(&operation, &base).diff().clone();
    assert_eq!(next.camera, camera);
    let backwards = Mutation::inverse(&operation, &base);
    assert_eq!(backwards, vec![EquationGraphWindowConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() })]);
    assert_eq!(Mutation::diff(&backwards[0], &next).diff().clone(), base);
    store::os_store::test_support::assert_op_line_round_trip(&operation);
}
