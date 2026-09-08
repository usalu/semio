
use super::*;

#[test]
fn generation3d_config_default_matches_the_former_runtime_defaults() {
    let config = Generation3dConfig::default();
    assert_eq!(config.show_mode, "shaded");
    assert_eq!(config.sun(), semio_framework_plugin::WorldSunConfig::default());
    let pack = <Generation3dConfig as store::ArtifactPack>::encode_pack_with(&config, &store::PackEncodeOptions::default()).expect("the app default must be pack-encodable before the registry constructs its store");
    assert_eq!(<Generation3dConfig as store::ArtifactPack>::decode_pack_with(&pack, &store::PackDecodeOptions::default()).expect("the app default pack must decode"), config);
}

fn config_round_trip(base: &Generation3dConfig, operation: &Generation3dConfigMutation) -> Generation3dConfig {
    let forward = operation.diff(base).into_parts().0;
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).into_parts().0;
    }
    assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
    forward
}

#[test]
fn config_set_camera_and_preview_camera_round_trip() {
    let base = Generation3dConfig::default();
    let next = config_round_trip(&base, &Generation3dConfigMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
    assert_eq!(next.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
    let camera = Generation3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
    let next2 = config_round_trip(&next, &Generation3dConfigMutation::SetPreviewCamera { camera: camera.clone() });
    assert_eq!(next2.preview_camera, camera);
}

#[test]
fn config_set_sun_round_trip_as_raw_json() {
    let base = Generation3dConfig::default();
    let next = config_round_trip(&base, &Generation3dConfigMutation::SetSun { json: "{\"enabled\":true}".into() });
    assert_eq!(next.sun_json, "{\"enabled\":true}");
}

#[test]
fn config_set_generation_round_trips() {
    let base = Generation3dConfig::default();
    let next = config_round_trip(&base, &Generation3dConfigMutation::SetGeneration { selected_generation_id: Some("generation-1".into()), generation_preview_text: Some("42".into()) });
    assert_eq!(next.selected_generation_id, Some("generation-1".to_string()));
    assert_eq!(next.generation_preview_text, Some("42".to_string()));
}

#[test]
fn config_set_preview_eval_round_trips() {
    let base = Generation3dConfig::default();
    let next = config_round_trip(&base, &Generation3dConfigMutation::SetPreviewEval { eval_text: Some(r#"{"extrude":{}}"#.into()) });
    assert_eq!(next.preview_eval_text, Some(r#"{"extrude":{}}"#.to_string()));
    let cleared = config_round_trip(&next, &Generation3dConfigMutation::SetPreviewEval { eval_text: None });
    assert_eq!(cleared.preview_eval_text, None);
}

#[test]
fn config_op_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetLodMode { value: "coarse".into() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetShowMode { value: "wireframe".into() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetPreviewCamera { camera: Generation3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], fov: 45.0 } });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetSun { json: "{}".into() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetGeneration { selected_generation_id: Some("g1".into()), generation_preview_text: None });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::SetPreviewEval { eval_text: Some("{}".into()) });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation3dConfigMutation::Snapshot { config: Generation3dConfig::default() });
}
