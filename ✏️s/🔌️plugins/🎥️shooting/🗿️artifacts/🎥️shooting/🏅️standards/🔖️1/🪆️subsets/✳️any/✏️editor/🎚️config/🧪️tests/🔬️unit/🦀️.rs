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
        ShootingConfig { selected_shot_ids: vec!["s1".into()], center_model: false, fit_revision: 3, camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() }, ..ShootingConfig::default() };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn shooting_config_operation_text_binary_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: ShootingConfig { selected_shot_ids: vec!["s1".into()], ..ShootingConfig::default() } }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s1".into(), "s2".into()] }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCenterModel(SetCenterModel { value: true }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetFitRevision(SetFitRevision { value: 4 }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCamera(SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } }));
    store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetDefaults(SetDefaults { shot_format: "svg".into(), shot_shape: "ellipse".into(), asset_format: "glb".into() }));
}

#[semio_framework_async_macros::async_test]
async fn shooting_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = ShootingConfig { selected_shot_ids: vec!["s1".into()], ..ShootingConfig::default() };
    let operation = ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s2".into()] });
    let forward = operation.diff(&base).into_parts().0;
    assert_eq!(forward, ShootingConfigDiff { selected_shot_ids: Some(vec!["s2".to_string()]), ..Default::default() });
    let backwards = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s1".into()] })]);
    let advanced = protocol::apply_diff(&forward, &base).expect("forward diff applies");
    let restored = protocol::apply_diff(&backwards[0].diff(&advanced).into_parts().0, &advanced).expect("inverse diff applies");
    assert_eq!(restored, base);
}

/// ⚖️ LAW: every config kind's inverse rows sum to the negative of its sparse diff.
#[semio_framework_async_macros::async_test]
async fn every_config_kind_obeys_the_inverse_sum_law() {
    let base = ShootingConfig { selected_shot_ids: vec!["s1".into()], center_model: false, fit_revision: 2, ..ShootingConfig::default() };
    let moved = ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() };
    for operation in [
        ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s2".into(), "s3".into()] }),
        ShootingConfigMutation::SetCenterModel(SetCenterModel { value: true }),
        ShootingConfigMutation::SetFitRevision(SetFitRevision { value: 7 }),
        ShootingConfigMutation::SetCamera(SetCamera { camera: moved.clone() }),
        ShootingConfigMutation::SetDefaults(SetDefaults { shot_format: "svg".into(), shot_shape: "ellipse".into(), asset_format: "gltf".into() }),
        ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: ShootingConfig { selected_shot_ids: vec![], center_model: true, fit_revision: 0, camera: moved, default_shot_shape: "ellipse".into(), ..base.clone() } }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
    }
}

/// 🕳️ A kind that sets the value the config already holds is a no-op: an empty diff and a warning, never a history row.
#[semio_framework_async_macros::async_test]
async fn setting_the_held_value_is_a_no_op() {
    let base = ShootingConfig::default();
    for operation in [
        ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: base.selected_shot_ids.clone() }),
        ShootingConfigMutation::SetCenterModel(SetCenterModel { value: base.center_model }),
        ShootingConfigMutation::SetFitRevision(SetFitRevision { value: base.fit_revision }),
        ShootingConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() }),
        ShootingConfigMutation::SetDefaults(SetDefaults { shot_format: base.default_shot_format.clone(), shot_shape: base.default_shot_shape.clone(), asset_format: base.default_asset_format.clone() }),
        ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() }),
    ] {
        let outcome = operation.diff(&base);
        assert!(protocol::DiffAlgebra::<ShootingConfig>::is_empty(outcome.diff()), "{operation:?} must diff to nothing");
        assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Warning));
    }
}
