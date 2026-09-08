
use super::*;

#[test]
fn config_set_camera_round_trips_and_restores() {
    let base = Generation2dConfig::default();
    let camera = CameraJson { x: 9.0, y: -3.0, zoom: 2.5 };
    let forward = Generation2dConfigMutation::SetCamera { camera: camera.clone() }.diff(&base).into_parts().0;
    assert_eq!(forward.camera, camera);
}

#[test]
fn config_set_show_mode_round_trips_and_restores() {
    let base = Generation2dConfig::default();
    let forward = Generation2dConfigMutation::SetShowMode { value: "wire".into() }.diff(&base).into_parts().0;
    assert_eq!(forward.show_mode, "wire");
}

#[test]
fn config_set_locale_round_trips_and_restores() {
    let base = Generation2dConfig::default();
    let forward = Generation2dConfigMutation::SetLocale { value: "de-DE".into() }.diff(&base).into_parts().0;
    assert_eq!(forward.locale, "de-DE");
}

#[test]
fn config_op_text_round_trips_every_variant() {
    let config = Generation2dConfig { locale: "de-DE".into(), ..Generation2dConfig::default() };
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::Snapshot { config });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetShowMode { value: "generate".into() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetGeneration { selected_generation_id: None, generation_preview_text: None });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetGeneration { selected_generation_id: Some("g1".into()), generation_preview_text: None });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetLocale { value: "en-US".into() });
}
