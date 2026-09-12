use super::*;

#[semio_framework_async_macros::async_test]
async fn cad_config_default_matches_the_existing_runtime_defaults() {
    let config = CadConfig::default();
    assert_eq!(config.engagement_step, "Idle");
    assert!(config.selected_node_ids.is_empty());
    assert_eq!(config.engagement_preview_generation, 0);
}

#[semio_framework_async_macros::async_test]
async fn cad_config_dsl_round_trips_a_populated_record() {
    let config = CadConfig {
        selected_node_ids: vec!["node-1".into(), "node-2".into()],
        hovered_reference_id: Some("ref-1".into()),
        engagement_session_json: Some("{\"interactionId\":\"box\"}".into()),
        engagement_input: "select".into(),
        ..CadConfig::default()
    };
    let text = store::ArtifactDsl::print_dsl(&config);
    let parsed = <CadConfig as store::ArtifactDsl>::parse_dsl(&text).expect("cad config dsl parses");
    assert_eq!(parsed, config);
}

#[semio_framework_async_macros::async_test]
async fn cad_config_pack_round_trips() {
    let config = CadConfig { selected_node_ids: vec!["node-1".into()], hovered_reference_id: Some("reference-1".into()), ..CadConfig::default() };
    let bytes = store::ArtifactPack::encode_pack(&config);
    let decoded = <CadConfig as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, config);
}

#[semio_framework_async_macros::async_test]
async fn cad_sun_config_round_trips_through_world_sun_config() {
    let world = semio_framework_plugin::WorldSunConfig { enabled: true, azimuth: 12.0, elevation: 34.0, intensity: 0.5, color: "#112233".into() };
    let cad_sun = cad_sun_config_from_world(&world);
    let back = cad_sun_config_to_world(&cad_sun);
    assert_eq!(back, world);
}

#[semio_framework_async_macros::async_test]
async fn cad_config_operation_snapshot_round_trips_and_restores_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️inline-layout/🔣️.json")).expect("neutral config mutation layout fixture");
    let base = CadConfig::default();
    let next = CadConfig { selected_node_ids: serde_json::from_value(fixture["selectedNodeIds"].clone()).expect("neutral selection"), ..CadConfig::default() };
    let operation = CadConfigMutation::Snapshot { config: Box::new(next.clone()) };
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(forward, next);
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![CadConfigMutation::Snapshot { config: Box::new(base.clone()) }]);
    assert_eq!(serde_json::to_value(&forward.selected_node_ids).expect("selection JSON oracle"), fixture["selectedNodeIds"]);
    let bytes = protocol::OpBinary::encode_op(&operation).expect("config mutation binary");
    assert_eq!(<CadConfigMutation as protocol::OpBinary>::decode_op(&bytes).expect("config mutation binary round trip"), operation);
    let inline_bytes = size_of::<CadConfigMutation>();
    eprintln!("[DEBUG] CAD config mutation inline bytes={inline_bytes}");
    assert!(inline_bytes <= fixture["maximumInlineBytes"].as_u64().unwrap() as usize, "CAD config mutation exceeds its neutral inline budget");
    let restored = backwards[0].diff(&forward).diff().clone();
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&operation);
}

#[semio_framework_async_macros::async_test]
async fn cad_config_set_contributions_round_trips() {
    let base = CadConfig::default();
    let json = r#"[{"pluginId":"cad-extension-spatial-shape","contribution":{"kind":"cadComputer","appId":"cad-play","moduleId":"spatial-shape","label":"Spatial Shape","iconId":"box","computersJson":"{}"}}]"#;
    let operation = CadConfigMutation::SetContributions { json: json.into() };
    let next = operation.diff(&base).diff().clone();
    assert_eq!(next.contributions_json, json);
    store::os_store::test_support::assert_op_line_round_trip(&operation);
}
