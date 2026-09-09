use super::*;

#[semio_framework_async_macros::async_test]
async fn shooting_config_default_matches_the_existing_action_arg_sticky_defaults() {
    let config = ShootingConfig::default();
    assert_eq!(config.default_shot_format, "png");
    assert_eq!(config.default_shot_shape, "rectangle");
    assert_eq!(config.default_asset_format, "glb");
}

/// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `ShootingConfig`.
#[semio_framework_async_macros::async_test]
async fn shooting_config_dsl_pack_round_trip() {
    let config =
        ShootingConfig { selected_shot_ids: vec!["s1".into()], center_model: false, fit_revision: 3, camera_draft_label: "Hero".into(), camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() }, ..ShootingConfig::default() };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn shooting_config_operation_text_binary_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: ShootingConfig { selected_shot_ids: vec!["s1".into()], ..ShootingConfig::default() } }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s1".into(), "s2".into()] }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCenterModel(SetCenterModel { value: true }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetFitRevision(SetFitRevision { value: 4 }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCameraDraftLabel(SetCameraDraftLabel { value: "Hero".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCamera(SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetDefaults(SetDefaults { shot_format: "svg".into(), shot_shape: "ellipse".into(), asset_format: "glb".into() }));
}

#[semio_framework_async_macros::async_test]
async fn shooting_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = ShootingConfig { selected_shot_ids: vec!["s1".into()], ..ShootingConfig::default() };
    let operation = ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s2".into()] });
    let forward = operation.diff(&base).into_parts().0;
    assert_eq!(forward.selected_shot_ids, vec!["s2".to_string()]);
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]);
    let restored = backwards[0].diff(&forward).into_parts().0;
    assert_eq!(restored, base);
}
