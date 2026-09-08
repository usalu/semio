
use super::*;
use protocol::{Mutation, MutationDiff};

#[test]
fn gis3d_config_serde_is_strict_and_requires_the_camera_field() {
    assert!(serde_json::from_str::<Gis3dConfig>(r#"{}"#).is_err());
    assert!(serde_json::from_str::<Gis3dConfig>(r#"{"locale":"en-US"}"#).is_err());
    assert!(serde_json::from_str::<Gis3dConfig>(r#"{"cameraJson":null}"#).is_err());
    assert!(serde_json::from_str::<Gis3dConfig>(r#"{"cameraJson":"{}","extra":true}"#).is_err());
    assert!(serde_json::from_str::<Gis3dConfig>(r#"{"cameraJson":"{}"}"#).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_default_matches_the_pre_migration_view_defaults() {
    let config = Gis3dConfig::default();
    assert!(config.camera_json.contains("800"));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_dsl_round_trips_default_and_populated() {
    store::os_store::test_support::assert_dsl_round_trip(&Gis3dConfig::default());
    let populated = Gis3dConfig { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() };
    store::os_store::test_support::assert_dsl_round_trip(&populated);
    store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = Gis3dConfig::default();
    let operation = Gis3dConfigMutation::SetCamera(SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
    let next = operation.diff(&base).diff().apply(&base).expect("apply");
    assert_eq!(next.camera_json, r#"{"position":[1.0,2.0,3.0]}"#);
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![Gis3dConfigMutation::SetCamera(SetCamera { camera_json: base.camera_json.clone() })]);
    assert_eq!(backwards[0].diff(&next).diff().apply(&next).expect("restore"), base);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_config_operation_lines_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&Gis3dConfigMutation::SetCamera(SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() }));
}
