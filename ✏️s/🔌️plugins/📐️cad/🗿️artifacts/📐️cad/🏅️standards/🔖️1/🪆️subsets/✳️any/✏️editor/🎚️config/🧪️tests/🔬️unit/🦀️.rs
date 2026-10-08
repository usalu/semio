use super::*;

#[semio_framework_async_macros::async_test]
async fn cad_config_default_matches_the_existing_runtime_defaults() {
    let config = CadConfig::default();
    assert!(config.selected_node_ids.is_empty());
    assert_eq!(config.contributions_json, "[]");
}

#[semio_framework_async_macros::async_test]
async fn cad_config_dsl_round_trips_a_populated_record() {
    let config = CadConfig {
        selected_node_ids: vec!["node-1".into(), "node-2".into()],
        hovered_reference_id: Some("ref-1".into()),
        active_example_id: Some("demo".into()),
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
    let operation = CadConfigMutation::Set { config: Box::new(next.clone()) };
    let forward = operation.diff(&base).diff().clone();
    assert_eq!(protocol::apply_diff(&forward, &base).unwrap(), next);
    assert_eq!(forward, CadConfigDiff { selected_node_ids: Some(next.selected_node_ids.clone()), ..Default::default() });
    let backwards = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(backwards, vec![CadConfigMutation::Set { config: Box::new(base.clone()) }]);
    assert_eq!(serde_json::to_value(&forward.selected_node_ids).expect("selection JSON oracle"), fixture["selectedNodeIds"]);
    let bytes = protocol::OpBinary::encode_op(&operation).expect("config mutation binary");
    assert_eq!(<CadConfigMutation as protocol::OpBinary>::decode_op(&bytes).expect("config mutation binary round trip"), operation);
    let inline_bytes = size_of::<CadConfigMutation>();
    assert!(inline_bytes <= fixture["maximumInlineBytes"].as_u64().unwrap() as usize, "CAD config mutation exceeds its neutral inline budget");
    let restored = protocol::apply_diff(backwards[0].diff(&next).diff(), &next).unwrap();
    assert_eq!(restored, base);
    store::os_store::test_support::assert_op_line_round_trip(&operation);
}

#[semio_framework_async_macros::async_test]
async fn cad_config_set_contributions_round_trips() {
    let base = CadConfig::default();
    let json = r#"[{"pluginId":"cad-extension-spatial-shape","contribution":{"kind":"cadComputer","appId":"cad-play","moduleId":"spatial-shape","label":"Spatial Shape","iconId":"box","computersJson":"{}"}}]"#;
    let operation = CadConfigMutation::SetContributions { json: json.into() };
    let next = protocol::apply_diff(operation.diff(&base).diff(), &base).unwrap();
    assert_eq!(next.contributions_json, json);
    store::os_store::test_support::assert_op_line_round_trip(&operation);
}

#[semio_framework_async_macros::async_test]
async fn cad_config_inverses_sum_to_the_negative_diff() {
    let base = CadConfig { hovered_reference_id: Some("reference-1".into()), selected_node_ids: vec!["node-0".into()], ..CadConfig::default() };
    let next = CadConfig { selected_node_ids: vec!["node-1".into(), "node-2".into()], hovered_reference_id: None, active_example_id: Some("example".into()), ..base.clone() };
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&CadConfigMutation::Set { config: Box::new(next.clone()) }, &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&CadConfigMutation::SetContributions { json: "[]".into() }, &CadConfig { contributions_json: "[{}]".into(), ..base.clone() }).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<CadConfig, CadConfigDiff>(&base, &next).await;
}
