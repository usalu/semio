use super::*;

#[semio_framework_async_macros::async_test]
async fn sequence_config_default_matches_the_existing_runtime_defaults() {
    let config = SequenceConfig::default();
    assert!(config.last_run_json.is_empty());
    assert_eq!(config.orientation, "leftRight");
}

#[semio_framework_async_macros::async_test]
async fn sequence_config_dsl_round_trips() {
    let config = SequenceConfig { last_run_json: "{}".into(), orientation: "topBottom".into(), camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } };
    let text = store::ArtifactDsl::print_dsl(&config);
    let parsed = <SequenceConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config dsl round trip");
    assert_eq!(parsed, config);
}

#[semio_framework_async_macros::async_test]
async fn sequence_config_pack_round_trips() {
    let config = SequenceConfig { last_run_json: "{\"ok\":true}".into(), orientation: "leftRight".into(), camera: SequenceCamera::default() };
    let bytes = store::ArtifactPack::encode_pack(&config);
    let decoded = <SequenceConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack round trip");
    assert_eq!(decoded, config);
}

//#region 🔖️ConfigMutationTests
fn round_trip_config(config: &SequenceConfig, operation: &SequenceConfigMutation) -> SequenceConfig {
    let forward = operation.diff(config).diff().clone();
    let backwards = operation.inverse(config);
    assert_eq!(backwards.len(), 1);
    let restored = backwards[0].diff(&forward).diff().clone();
    assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn config_set_last_run_round_trips() {
    let config = SequenceConfig::default();
    let next = round_trip_config(&config, &SequenceConfigMutation::SetLastRun(SetLastRun { json: "{\"ok\":true}".into() }));
    assert_eq!(next.last_run_json, "{\"ok\":true}");
}

#[semio_framework_async_macros::async_test]
async fn config_set_orientation_round_trips() {
    let config = SequenceConfig::default();
    let next = round_trip_config(&config, &SequenceConfigMutation::SetOrientation(SetOrientation { value: "topBottom".into() }));
    assert_eq!(next.orientation, "topBottom");
}

#[semio_framework_async_macros::async_test]
async fn config_set_camera_round_trips() {
    let config = SequenceConfig::default();
    let camera = SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 };
    let next = round_trip_config(&config, &SequenceConfigMutation::SetCamera(SetCamera { camera: camera.clone() }));
    assert_eq!(next.camera, camera);
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetLastRun(SetLastRun { json: "{}".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetOrientation(SetOrientation { value: "leftRight".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetCamera(SetCamera { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } }));
}
//#endregion 🔖️ConfigMutationTests
