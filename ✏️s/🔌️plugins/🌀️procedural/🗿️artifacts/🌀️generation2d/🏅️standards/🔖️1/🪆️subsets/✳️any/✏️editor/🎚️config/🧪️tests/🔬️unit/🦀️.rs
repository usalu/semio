use super::*;

#[semio_framework_async_macros::async_test]
async fn config_set_show_mode_round_trips_and_restores() {
    let base = Generation2dConfig::default();
    let mutation = Generation2dConfigMutation::SetShowMode(SetShowModeSetting { value: "wire".into() });
    let forward = mutation.diff(&base).into_parts().0;
    assert_eq!(forward, Generation2dConfigDiff { show_mode: Some("wire".into()), ..Default::default() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn config_selected_generation_set_and_clear_satisfy_the_inverse_sum_law() {
    let selected = Generation2dConfig { selected_generation_id: Some("g1".into()), ..Generation2dConfig::default() };
    for (mutation, base) in [
        (Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id: Some("g2".into()) }), Generation2dConfig::default()),
        (Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id: None }), selected),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

#[test]
fn config_op_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetShowMode(SetShowModeSetting { value: "generate".into() }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id: None }));
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id: Some("g1".into()) }));
}

/// 🧾️ The payload-record variants keep the former named variants' externally tagged wire byte for byte, checked against an
/// independent serde_json reading of the exact historical literals.
#[test]
fn config_mutation_wire_is_identical_to_the_former_named_variants() {
    let fixture = [
        (Generation2dConfigMutation::SetShowMode(SetShowModeSetting { value: "wire".into() }), r#"{"SetShowMode":{"value":"wire"}}"#),
        (Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id: Some("g1".into()) }), r#"{"SetSelectedGeneration":{"selected_generation_id":"g1"}}"#),
        (Generation2dConfigMutation::SetSelectedGeneration(SetSelectedGenerationSetting { selected_generation_id: None }), r#"{"SetSelectedGeneration":{"selected_generation_id":null}}"#),
    ];
    for (mutation, literal) in fixture {
        let json = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&mutation));
        let encoded: serde_json::Value = serde_json::from_str(&json).expect("the encoded wire is valid json");
        let historical: serde_json::Value = serde_json::from_str(literal).expect("the historical literal is valid json");
        assert_eq!(encoded, historical);
        let parsed = semio_framework_pack_json::parse(literal, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the historical literal parses");
        assert_eq!(<Generation2dConfigMutation as semio_framework_value::FromValue>::from_value(parsed).expect("the historical literal decodes"), mutation);
    }
}
