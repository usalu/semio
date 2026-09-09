use super::*;
use store::ArtifactPack;

#[semio_framework_async_macros::async_test]
async fn lowpoly_config_dsl_round_trips_default() {
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&LowpolyConfig::default());
}

#[semio_framework_async_macros::async_test]
async fn lowpoly_config_dsl_round_trips_non_default() {
    let config = LowpolyConfig { active_object_id: "obj-2".into(), engagement_input: "extrude".into(), ..LowpolyConfig::default() };
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&config);
}

#[semio_framework_async_macros::async_test]
async fn lowpoly_config_pack_round_trips() {
    let config = LowpolyConfig { active_object_id: "obj-9".into(), sun_enabled: true, ..LowpolyConfig::default() };
    let bytes = config.encode_pack();
    let restored = LowpolyConfig::decode_pack(&bytes).expect("decode");
    assert_eq!(restored, config);
}

#[semio_framework_async_macros::async_test]
async fn config_op_backwards_always_snapshots_prior_state() {
    let base = LowpolyConfig { active_object_id: "obj-1".into(), ..LowpolyConfig::default() };
    let operation = LowpolyConfigMutation::SetActiveObject { object_id: "obj-2".into() };
    let after = operation.diff(&base).into_parts().0;
    assert_eq!(after.active_object_id, "obj-2");
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![LowpolyConfigMutation::Snapshot { config: base.clone() }]);
    assert_eq!(backwards[0].diff(&after).into_parts().0, base);
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trip_set_active_object() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::SetActiveObject { object_id: "obj-2".into() });
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trip_world_camera() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::SetWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trip_snapshot() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::Snapshot { config: LowpolyConfig::default() });
}

#[semio_framework_async_macros::async_test]
async fn config_op_binary_round_trips_and_agrees_with_text() {
    let operation = LowpolyConfigMutation::SetActivePaintLayer { value: 3 };
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
}
