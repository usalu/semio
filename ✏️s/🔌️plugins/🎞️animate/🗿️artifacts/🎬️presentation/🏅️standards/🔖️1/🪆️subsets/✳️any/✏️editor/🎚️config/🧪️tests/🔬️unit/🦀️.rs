
use super::*;

#[test]
fn presentation_config_default_matches_the_existing_runtime_defaults() {
    let config = PresentationConfig::default();
    assert!(config.engagement_input.is_empty());
}

#[test]
fn presentation_config_dsl_round_trips() {
    let config = PresentationConfig { engagement_input: "2x2".into(), };
    let text = store::ArtifactDsl::print_dsl(&config);
    let parsed = <PresentationConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config dsl round trip");
    assert_eq!(parsed, config);
}

#[test]
fn presentation_config_pack_round_trips() {
    let config = PresentationConfig { engagement_input: "add".into(), };
    let bytes = store::ArtifactPack::encode_pack(&config);
    let decoded = <PresentationConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack round trip");
    assert_eq!(decoded, config);
}

//#region 🔖️ConfigMutationTests
fn round_trip_config(config: &PresentationConfig, operation: &PresentationConfigMutation) -> PresentationConfig {
    let forward = operation.diff(config).diff().clone();
    let backwards = operation.inverse(config);
    assert_eq!(backwards.len(), 1);
    let restored = backwards[0].diff(&forward).diff().clone();
    assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
    forward
}

#[test]
fn config_set_engagement_input_round_trips() {
    let config = PresentationConfig::default();
    let next = round_trip_config(&config, &PresentationConfigMutation::SetEngagementInput(SetEngagementInput { value: "2x2".into() }));
    assert_eq!(next.engagement_input, "2x2");
}


#[test]
fn config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&PresentationConfigMutation::SetEngagementInput(SetEngagementInput { value: "add".into() }));
}
//#endregion 🔖️ConfigMutationTests
