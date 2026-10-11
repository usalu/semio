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
    let operation = Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: "cut".into() });
    let inverse = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: base.engagement_input })]);
}

#[semio_framework_async_macros::async_test]
async fn process3d_config_operation_diff_applies_expected_fields() {
    let base = Process3dConfig::default();
    let next = protocol::apply_diff(&Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 }).diff(&base).into_parts().0, &base).expect("the config diff applies");
    assert_eq!(next.camera_position, [1.0, 2.0, 3.0]);
    assert_eq!(next.camera_target, [0.1, 0.2, 0.3]);
    assert_eq!(next.camera_fov, 60.0);

    let next = protocol::apply_diff(&Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() }).diff(&base).into_parts().0, &base).expect("the config diff applies");
    assert!(next.sun_enabled);
    assert_eq!(next.sun_azimuth, 10.0);
    assert_eq!(next.sun_elevation, 20.0);
    assert_eq!(next.sun_intensity, 0.5);
    assert_eq!(next.sun_color, "#123456");
}

#[semio_framework_async_macros::async_test]
async fn process3d_config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: "cut".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 }));
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: "[]".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: Some(3) }));
    store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: None }));
}

/// ⏱️ The replay cursor is config (view state): it defaults to "show every step", round-trips the config codecs and
/// its setter inverts to the base value — never a document mutation.
#[semio_framework_async_macros::async_test]
async fn the_replay_cursor_is_view_state_in_the_config() {
    use store::ArtifactPack;
    let base = Process3dConfig::default();
    assert_eq!(base.resolved_up_to, None);
    let next = protocol::apply_diff(&Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: Some(2) }).diff(&base).into_parts().0, &base).expect("the config diff applies");
    assert_eq!(next.resolved_up_to, Some(2));
    assert_eq!(Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: Some(2) }).inverse(&base).expect("valid retained mutation inverse fixture"), vec![Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: None })]);
    store::os_store::test_support::assert_dsl_round_trip(&next);
    assert_eq!(Process3dConfig::decode_pack(&next.encode_pack()).expect("decode"), next);
}

#[semio_framework_async_macros::async_test]
async fn process3d_config_default_matches_the_existing_runtime_defaults() {
    let config = Process3dConfig::default();
    assert_eq!(config.camera_position, [3.0, -3.0, 2.0]);
    assert_eq!(config.camera_target, [0.0, 0.0, 0.0]);
    assert_eq!(config.camera_fov, 45.0);
    assert!(!config.sun_enabled);
}

/// 🧬️ The newtype-over-record restructuring keeps the externally tagged named-variant wire byte for byte.
#[test]
fn process3d_config_mutation_wire_is_byte_identical_to_the_named_variant_shape() {
    let cases = [
        (Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput { value: "cut".into() }), r#"{"SetEngagementInput":{"value":"cut"}}"#),
        (Process3dConfigMutation::SetCamera(Process3dConfigSetCamera { position: [1.0, 2.0, 3.0], target: [0.5, 0.25, 0.0], fov: 30.0 }), r#"{"SetCamera":{"position":[1.0,2.0,3.0],"target":[0.5,0.25,0.0],"fov":30.0}}"#),
        (Process3dConfigMutation::SetSun(Process3dConfigSetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#fff".into() }), r##"{"SetSun":{"enabled":true,"azimuth":10.0,"elevation":20.0,"intensity":0.5,"color":"#fff"}}"##),
        (Process3dConfigMutation::SetContributions(Process3dConfigSetContributions { json: "[]".into() }), r#"{"SetContributions":{"json":"[]"}}"#),
        (Process3dConfigMutation::SetCursor(Process3dConfigSetCursor { value: Some(2) }), r#"{"SetCursor":{"value":2}}"#),
    ];
    for (mutation, expected) in cases {
        let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&mutation)).expect("the encoded config mutation is valid json");
        let legacy: serde_json::Value = serde_json::from_str(expected).expect("the legacy wire literal is valid json");
        assert_eq!(encoded, legacy, "{mutation:?}");
        let decoded = <Process3dConfigMutation as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&mutation)).expect("the wire decodes back");
        assert_eq!(decoded, mutation);
    }
}

