
use super::*;

#[semio_framework_async_macros::async_test]
async fn flow_config_default_matches_flow_play_runtime_defaults() {
    let config = FlowConfig::default();
    assert_eq!(config.camera, CameraJson { x: 0.0, y: 0.0, zoom: 1.0 });
    assert_eq!(config.lod_mode, FLOW_LOD_MODE_AUTOMATIC);
    assert_eq!(config.proximity_distance, FLOW_DEFAULT_PROXIMITY_DISTANCE);
    assert!(config.grid_visible);
    assert!(!config.grid_snap_enabled);
    assert_eq!(config.grid_factor, FLOW_DEFAULT_GRID_FACTOR);
    assert_eq!(config.catalogue_sections_json, "[]");
    assert_eq!(config.automation_enabled(), HashMap::new());
    assert_eq!(config.generation(), GenerationPlayState::default());
}

/// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `FlowConfig`.
#[semio_framework_async_macros::async_test]
async fn flow_config_dsl_pack_round_trip() {
    let config = FlowConfig {
        preview_off_node_ids: vec!["n2".into()],
        camera: CameraJson { x: 12.5, y: -3.0, zoom: 2.25 },
        lod_mode: "micro".into(),
        proximity_distance: 96.0,
        grid_visible: false,
        grid_snap_enabled: true,
        grid_factor: 5.0,
        catalogue_sections_json: "[{\"id\":\"custom\"}]".into(),
        automation_enabled_json: "{\"auto-layout\":true}".into(),
        contributions_json: "[]".into(),
        generation_json: "{\"generations\":[]}".into(),
        duplicate_widget_progress_json: String::new(),
        };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn flow_config_operation_text_binary_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::Snapshot { config: FlowConfig { ..FlowConfig::default() } });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetPreviewOff { node_ids: vec!["n1".into()] });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetLodMode { value: "micro".into() });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetProximityDistance { value: 48.0 });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGridVisible { value: true });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGridSnapEnabled { value: false });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGridFactor { value: 10.0 });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetCatalogueSections { sections_json: "[]".into() });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetAutomationEnabled { json: "{\"auto-layout\":true}".into() });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGeneration { json: "{\"generations\":[]}".into() });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetDuplicateWidgetProgress { json: "{\"generation\":7}".into() });
    store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::CancelDuplicateWidget { generation: 7 });
}

#[semio_framework_async_macros::async_test]
async fn flow_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = FlowConfig { ..FlowConfig::default() };
    let operation = FlowConfigMutation::SetPreviewOff { node_ids: vec!["n2".into()] };
    let forward = operation.diff(&base).into_parts().0;
    assert_eq!(forward.preview_off_node_ids, vec!["n2".to_string()]);
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![FlowConfigMutation::Snapshot { config: base.clone() }]);
    let restored = backwards[0].diff(&forward).into_parts().0;
    assert_eq!(restored, base);
}
