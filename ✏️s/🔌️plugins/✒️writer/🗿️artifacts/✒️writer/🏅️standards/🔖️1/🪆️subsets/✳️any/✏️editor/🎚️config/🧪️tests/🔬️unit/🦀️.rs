
use super::*;

#[semio_framework_async_macros::async_test]
async fn writer_config_dsl_round_trips_default_and_populated() {
    store::os_store::test_support::assert_config_round_trip(&WriterConfig::default());
    let populated = WriterConfig { editor_selection: Some(WriterEditorSelection { start: 3, end: 7 }), format_signal: 2, lint_signal: 1, revision: 9, engagement_input: "format".into(), ..WriterConfig::default() };
    store::os_store::test_support::assert_config_round_trip(&populated);
}

#[semio_framework_async_macros::async_test]
async fn writer_config_operation_backwards_restores_pre_state() {
    let pre = WriterConfig::default();
    store::os_store::test_support::assert_operation_round_trip(&pre, WriterConfigMutation::SetEditorSelection(SetEditorSelection { selection: Some(WriterEditorSelection { start: 1, end: 2 }) })).await;
    store::os_store::test_support::assert_operation_round_trip(&pre, WriterConfigMutation::SetCamera(SetCamera { camera: WriterCamera { x: 5.0, y: -2.0, zoom: 1.5 } })).await;
}

#[semio_framework_async_macros::async_test]
async fn writer_config_operation_binary_matches_text() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&WriterConfigMutation::ReplaceConfig(ReplaceConfig { config: WriterConfig::default() }));
}

#[semio_framework_async_macros::async_test]
async fn writer_config_pack_round_trips() {
    let config = WriterConfig { engagement_input: "format".into(), ..WriterConfig::default() };
    let bytes = store::ArtifactPack::encode_pack(&config);
    let decoded = <WriterConfig as store::ArtifactPack>::decode_pack(&bytes).expect("decode writer config pack");
    assert_eq!(decoded, config);
}
