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
async fn config_op_inverse_restores_exactly_the_fields_it_owns() {
    let base = LowpolyConfig { active_object_id: "obj-1".into(), ..LowpolyConfig::default() };
    let operation = LowpolyConfigMutation::SetActiveObject(SetActiveObjectEdit { object_id: "obj-2".into() });
    let diff = operation.diff(&base).into_parts().0;
    assert_eq!(diff, LowpolyConfigDiff { active_object_id: Some("obj-2".into()), ..Default::default() });
    let after = protocol::apply_diff(&diff, &base).expect("valid config diff");
    assert_eq!(after.active_object_id, "obj-2");
    let backwards = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![LowpolyConfigMutation::SetActiveObject(SetActiveObjectEdit { object_id: "obj-1".into() })]);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn every_config_mutation_satisfies_the_inverse_sum_law() {
    let base = LowpolyConfig::default();
    let mutations = vec![
        LowpolyConfigMutation::SetActiveObject(SetActiveObjectEdit { object_id: "obj-2".into() }),
        LowpolyConfigMutation::SetPaintUtility(SetPaintUtilityEdit { value: "fill".into() }),
        LowpolyConfigMutation::SetActivePaintLayer(SetActivePaintLayerEdit { value: 2 }),
        LowpolyConfigMutation::SetUtilityParams(SetUtilityParamsEdit { json: "{}".into() }),
        LowpolyConfigMutation::SetPaintColor(SetPaintColorEdit { r: 1, g: 2, b: 3, a: 4 }),
        LowpolyConfigMutation::SetWorldCamera(SetWorldCameraEdit { position: [1.0, 2.0, 3.0], target: [0.0, 1.0, 0.0], fov: 60.0 }),
        LowpolyConfigMutation::SetEngagementInput(SetEngagementInputEdit { value: "extrude".into() }),
        LowpolyConfigMutation::SetShowEdges(SetShowEdgesEdit { value: false }),
        LowpolyConfigMutation::SetSun(SetSunEdit { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#000000".into() }),
    ];
    for mutation in mutations {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trip_set_active_object() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::SetActiveObject(SetActiveObjectEdit { object_id: "obj-2".into() }));
}

#[semio_framework_async_macros::async_test]
async fn config_op_text_round_trip_world_camera() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::SetWorldCamera(SetWorldCameraEdit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }));
}

#[semio_framework_async_macros::async_test]
async fn config_op_binary_round_trips_and_agrees_with_text() {
    let operation = LowpolyConfigMutation::SetActivePaintLayer(SetActivePaintLayerEdit { value: 3 });
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
}
