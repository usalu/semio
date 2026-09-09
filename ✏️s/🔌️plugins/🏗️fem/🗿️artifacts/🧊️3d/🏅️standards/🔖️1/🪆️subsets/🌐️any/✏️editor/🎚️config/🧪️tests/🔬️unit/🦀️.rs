use super::*;

#[test]
fn fem3d_config_default_is_static_display_with_default_camera() {
    let config = Fem3dConfig::default();
    assert_eq!(config.result_mode, "static");
    assert!(config.result_source_id.is_none());
    assert_eq!(config.result_mode_index, 0);
    assert_eq!(config.camera, FemCamera::default());
}

/// 🧮️ `Fem3dConfig`'s `MutationDiff` is a whole-record replace, mirroring `Fem2dConfig`'s identical
/// B1 pilot pattern: `apply` ignores `base` entirely.
#[test]
fn fem3d_config_operation_diff_is_a_whole_record_replace() {
    let base = Fem3dConfig::default();
    let replacement = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "modal".into(), result_mode_index: 2, camera: FemCamera { json: "{\"x\":1}".into() } };
    let applied = protocol::MutationDiff::apply(&replacement, &base).expect("valid config mutation diff");
    assert_eq!(applied, replacement);
    let mut absorbed = base.clone();
    protocol::MutationDiff::absorb(&mut absorbed, replacement.clone());
    assert_eq!(absorbed, replacement);
}

#[test]
fn config_operation_backwards_always_restores_the_pre_operation_snapshot() {
    let base = Fem3dConfig::default();
    let camera = FemCamera { json: "{\"x\":1}".into() };
    let op = Fem3dConfigMutation::SetCamera { camera: camera.clone() };
    let next = op.diff(&base).diff().clone();
    assert_eq!(next.camera, camera);
    let backwards = op.inverse(&base);
    assert_eq!(backwards, vec![Fem3dConfigMutation::Snapshot { config: base.clone() }]);
    assert_eq!(backwards[0].diff(&next).diff(), &base);
}

#[test]
fn set_result_display_config_operation_round_trips() {
    let base = Fem3dConfig::default();
    let op = Fem3dConfigMutation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 };
    let next = op.diff(&base).diff().clone();
    assert_eq!(next.result_source_id.as_deref(), Some("dead"));
    assert_eq!(next.result_mode, "modal");
    assert_eq!(next.result_mode_index, 2);
}

#[test]
fn fem3d_config_operation_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dConfigMutation::Snapshot { config: Fem3dConfig::default() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dConfigMutation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dConfigMutation::SetCamera { camera: FemCamera { json: "{\"x\":1}".into() } });
}
