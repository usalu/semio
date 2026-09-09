use super::*;

#[semio_framework_async_macros::async_test]
async fn process3d_config_dsl_and_pack_round_trip() {
    use store::ArtifactPack;
    let config = Process3dConfig { sun_enabled: true, ..Process3dConfig::default() };
    store::os_store::test_support::assert_dsl_round_trip(&config);
    let bytes = config.encode_pack();
    assert_eq!(Process3dConfig::decode_pack(&bytes).expect("decode"), config);
}

#[semio_framework_async_macros::async_test]
async fn process3d_config_operation_backwards_restores_the_same_field_from_base() {
    let base = Process3dConfig::default();
    let operation = Process3dConfigMutation::SetEngagementInput { value: "cut".into() };
    let inverse = operation.inverse(&base);
    assert_eq!(inverse, vec![Process3dConfigMutation::SetEngagementInput { value: base.engagement_input }]);
}

#[semio_framework_async_macros::async_test]
async fn process3d_config_operation_diff_applies_expected_fields() {
    let base = Process3dConfig::default();
    let next = Process3dConfigMutation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 }.diff(&base).into_parts().0;
    assert_eq!(next.camera_position, [1.0, 2.0, 3.0]);
    assert_eq!(next.camera_target, [0.1, 0.2, 0.3]);
    assert_eq!(next.camera_fov, 60.0);

    let next = Process3dConfigMutation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() }.diff(&base).into_parts().0;
    assert!(next.sun_enabled);
    assert_eq!(next.sun_azimuth, 10.0);
    assert_eq!(next.sun_elevation, 20.0);
    assert_eq!(next.sun_intensity, 0.5);
    assert_eq!(next.sun_color, "#123456");
}

#[semio_framework_async_macros::async_test]
async fn process3d_config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetEngagementInput { value: "cut".into() });
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 });
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() });
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetContributions { json: "[]".into() });
}

#[semio_framework_async_macros::async_test]
async fn process3d_config_default_matches_the_existing_runtime_defaults() {
    let config = Process3dConfig::default();
    assert_eq!(config.camera_position, [3.0, -3.0, 2.0]);
    assert_eq!(config.camera_target, [0.0, 0.0, 0.0]);
    assert_eq!(config.camera_fov, 45.0);
    assert!(!config.sun_enabled);
}
