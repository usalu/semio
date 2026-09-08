
use super::*;

#[semio_framework_async_macros::async_test]
async fn forms_config_default_matches_the_existing_runtime_defaults() {
    let config = FormsConfig::default();
    assert_eq!(config.current_step_index, 0);
    assert!(config.try_values.iter_json().is_empty());
    assert_eq!(config.contributions_json, "[]");
}

#[semio_framework_async_macros::async_test]
async fn forms_config_dsl_and_pack_round_trip() {
    let chunks = split_try_value_chunks(r#""Ada""#, 4_096);
    let content_id = try_value_content_id(&chunks);
    let config = FormsConfig { current_step_index: 2, try_values: FormsTryValues::default().with_chunks("name", content_id, chunks.into()), contributions_json: "[]".into() };
    store::os_store::test_support::assert_dsl_round_trip(&config);
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

fn config_round_trip(base: &FormsConfig, operation: &FormsConfigMutation) -> FormsConfig {
    let forward = operation.diff(base).diff().clone();
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().clone();
    }
    assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_apply_and_restore_every_field() {
    let base = FormsConfig::default();
    assert_eq!(config_round_trip(&base, &FormsConfigMutation::SetStepIndex(SetStepIndex { index: 2 })).current_step_index, 2);
    let one_chunks = split_try_value_chunks("1", MAX_STAGED_TRY_VALUE_CHUNK_BYTES);
    let one_id = try_value_content_id(&one_chunks);
    let staged = FormsConfigMutation::StageTryValueChunk(StageTryValueChunk { staging_id: "one-stage".into(), index: 0, chunk: "1".into() }).diff(&base).diff().clone();
    assert_eq!(staged, base);
    assert_eq!(config_round_trip(&staged, &FormsConfigMutation::CommitTryValue(CommitTryValue { key: "a".into(), staging_id: "one-stage".into(), content_id: one_id.clone(), chunk_count: 1 })).try_values.get_json("a"), Some(one_id.as_str()));
    assert_eq!(config_round_trip(&base, &FormsConfigMutation::SetContributions(SetContributions { json: "[]".into() })).contributions_json, "[]");
}

#[semio_framework_async_macros::async_test]
async fn config_snapshot_op_text_round_trips() {
    let chunks = split_try_value_chunks(r#""Ada""#, 4_096);
    let content_id = try_value_content_id(&chunks);
    let config = FormsConfig { current_step_index: 1, try_values: FormsTryValues::default().with_chunks("name", content_id, chunks.into()), contributions_json: "[]".into() };
    store::os_store::test_support::assert_op_line_round_trip(&FormsConfigMutation::ReplaceConfig(ReplaceConfig { config }));
    store::os_store::test_support::assert_op_line_round_trip(&FormsConfigMutation::SetStepIndex(SetStepIndex { index: 3 }));
}

#[test]
fn malformed_order_and_identity_conflicts_are_typed() {
    discard_staged_try_value("typed-stage");
    assert_eq!(stage_try_value_chunk("typed-stage", 1, "late"), Err(FormsStageError::Order));
    assert_eq!(stage_try_value_chunk("typed-stage", 0, "value"), Ok(()));
    assert_eq!(commit_staged_try_value("typed-stage", "wrong-content-id", 1), Err(FormsStageError::Conflict));
    assert_eq!(stage_try_value_chunk(&"x".repeat(MAX_TRY_VALUE_OPERATION_ID_BYTES + 1), 0, "value"), Err(FormsStageError::Invalid));
}

#[test]
fn committed_content_survives_registry_clear_and_serialized_reopen() {
    let chunks = split_try_value_chunks(r#"{"answer":"Ada"}"#, 5);
    let content_id = try_value_content_id(&chunks);
    let config = FormsConfig { try_values: FormsTryValues::default().with_chunks("name", content_id.clone(), chunks.clone().into()), ..FormsConfig::default() };
    let serialized = dsl::os_pack::json::to_json_string(&config).into_bytes();
    clear_try_value_staging_for_replay();
    let reopened: FormsConfig = dsl::os_pack::json::from_json_str(std::str::from_utf8(&serialized).expect("config UTF-8")).expect("reopen committed Forms config");
    assert_eq!(reopened.try_values.get_json("name"), Some(content_id.as_str()));
    assert_eq!(reopened.try_values.content_chunks("name"), Some(chunks.as_slice()));
}

#[semio_framework_async_macros::async_test]
async fn bounded_stage_and_compact_commit_encode_decode_and_apply_under_eight_ms() {
    let raw = "x".repeat(4_096);
    let chunks = split_try_value_chunks(&raw, 4_096);
    let content_id = try_value_content_id(&chunks);
    let stage = FormsConfigMutation::StageTryValueChunk(StageTryValueChunk { staging_id: "timed-stage".into(), index: 0, chunk: raw });
    let started = std::time::Instant::now();
    let bytes = <FormsConfigMutation as protocol::OpBinary>::encode_op(&stage).expect("stage encode");
    let decoded = <FormsConfigMutation as protocol::OpBinary>::decode_op(&bytes).expect("stage decode");
    let staged = decoded.diff(&FormsConfig::default()).diff().clone();
    assert!(started.elapsed() < std::time::Duration::from_millis(8));

    let commit = FormsConfigMutation::CommitTryValue(CommitTryValue { key: "large".into(), staging_id: "timed-stage".into(), content_id: content_id.clone(), chunk_count: 1 });
    let started = std::time::Instant::now();
    let bytes = <FormsConfigMutation as protocol::OpBinary>::encode_op(&commit).expect("commit encode");
    let decoded = <FormsConfigMutation as protocol::OpBinary>::decode_op(&bytes).expect("commit decode");
    let committed = decoded.diff(&staged).diff().clone();
    assert_eq!(committed.try_values.get_json("large"), Some(content_id.as_str()));
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
}
