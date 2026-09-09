use super::*;
use crate::editor::forms::testkit::forms_app_with_registry;
use crate::editor::forms::FormsCommand;
use semio_framework_plugin::testkit::meta;
use semio_framework_plugin::PluginApp;

fn rope(parts: &[&str]) -> ChunkedSource {
    let mut source = ChunkedSource::default();
    for part in parts {
        source.push(Arc::from(*part));
    }
    source
}

fn continuation(result: semio_framework_plugin::InvocationResult) -> Option<serde_json::Value> {
    result.requested_effects.into_iter().find_map(|effect| match effect {
        Effect::DispatchAction { action, args, .. } if action == SET_TRY_VALUE_STEP_ACTION_ID => args.map(store::pack_rt::dsl_value_to_json),
        _ => None,
    })
}

fn materialize_owned_try_value(values: &crate::editor::forms::config::FormsTryValues, key: &str) -> Option<String> {
    values.content_chunks(key).map(|chunks| {
        chunks.iter().fold(String::new(), |mut raw, chunk| {
            raw.push_str(chunk);
            raw
        })
    })
}

#[test]
fn bulk_scanner_crosses_chunk_boundaries_with_a_bounded_key_copy() {
    let operation = semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "doc-a".into(), operation_id: 1, generation: 1, canonical_base_revision: [0; 32] };
    let mut session = new_bulk_session(rope(&[r#"{"na"#, r#"me":{"nested":[1,2,3]}}"#]), &operation, 1, 0);
    while matches!(session.phase, BulkPhase::Object | BulkPhase::Key | BulkPhase::Colon | BulkPhase::Value) {
        let started = std::time::Instant::now();
        scan_bulk(&mut session).expect("bulk scan");
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }
    assert_eq!(session.key, "name");
    assert!(session.value_end > session.value_start);
}

#[test]
fn bulk_sessions_are_document_and_operation_scoped() {
    let a = BulkJobKey { app_id: "1".into(), document_id: "a".into(), operation_id: "11".into(), base_revision: "0".repeat(64), generation: 7 };
    let b = BulkJobKey { app_id: "1".into(), document_id: "b".into(), operation_id: "12".into(), base_revision: "0".repeat(64), generation: 7 };
    assert_ne!(a, b);
}

#[test]
fn bulk_continuation_identity_matches_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔑️continuations.json")).unwrap();
    let key = |value: &serde_json::Value| BulkJobKey {
        app_id: value["appId"].as_str().unwrap().into(),
        document_id: value["documentId"].as_str().unwrap().into(),
        operation_id: value["operationId"].as_str().unwrap().into(),
        base_revision: value["baseRevision"].as_str().unwrap().into(),
        generation: value["generation"].as_u64().unwrap(),
    };
    for vector in vectors.as_array().unwrap() {
        let expected = vector["same"].as_bool().unwrap();
        assert_eq!(vector["left"] == vector["right"], expected);
        let sessions = BTreeMap::from([(key(&vector["left"]), ())]);
        assert_eq!(sessions.contains_key(&key(&vector["right"])), expected);
    }
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_replays_large_bulk_input_and_commits_atomically_under_eight_ms() {
    bulk_sessions().lock().expect("forms bulk sessions lock").clear();
    active_bulk_generations().lock().expect("forms bulk active lock").clear();
    *crate::editor::forms::commands::set_try_value::input_registry().lock().expect("forms input registry lock") = crate::editor::forms::commands::set_try_value::FormsInputRegistry::default();
    let chunks = [format!("{{\"a\":\"{}", "a".repeat(MAX_TRY_VALUE_BYTES_PER_STEP - 7)), format!("{}\",\"b\":1}}", "b".repeat(MAX_TRY_VALUE_BYTES_PER_STEP - 8))];
    assert!(chunks.iter().all(|chunk| chunk.len() <= MAX_TRY_VALUE_BYTES_PER_STEP));
    let mut app = forms_app_with_registry().await;
    let args = |input_id: &str, index: usize| {
        serde_json::json!({
            "valuesJson": chunks[index],
            "inputId": input_id,
            "inputIndex": index as u64,
            "inputCount": chunks.len() as u64
        })
    };
    let before_restart = args("bulk-before-restart", 0);
    app.handle_action("setTryValues", Some(&crate::editor::forms::testkit::action_args(&before_restart)), &meta("bulk")).await.expect("initial bulk action log entry");

    bulk_sessions().lock().expect("forms bulk sessions lock").clear();
    active_bulk_generations().lock().expect("forms bulk active lock").clear();
    *crate::editor::forms::commands::set_try_value::input_registry().lock().expect("forms input registry lock") = crate::editor::forms::commands::set_try_value::FormsInputRegistry::default();

    let mut result = None;
    for index in 0..chunks.len() {
        let command = FormsCommand::SetTryValues(SetTryValues { values_json: chunks[index].clone().into(), input_id: Some("bulk-after-restart".into()), input_index: Some(index as u64), input_count: Some(chunks.len() as u64) });
        let started = std::time::Instant::now();
        let wire = <FormsCommand as protocol::OpBinary>::encode_op(&command).expect("bulk Forms command encode");
        assert_eq!(<FormsCommand as protocol::OpBinary>::decode_op(&wire).expect("bulk Forms command decode"), command);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "bulk Forms public command codec exceeded 8 ms");
        let started = std::time::Instant::now();
        result = Some(app.handle_action("setTryValues", Some(&crate::editor::forms::testkit::action_args(&args("bulk-after-restart", index))), &meta("bulk")).await.expect("replayed bulk Forms action"));
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "bulk Forms public action envelope exceeded 8 ms");
    }
    let mut next = result.and_then(continuation);
    for _ in 0..128 {
        let Some(checkpoint) = next.take() else { break };
        let started = std::time::Instant::now();
        let result = app.handle_action(SET_TRY_VALUE_STEP_ACTION_ID, Some(&crate::editor::forms::testkit::action_args(&checkpoint)), &meta("bulk")).await.expect("bulk Forms continuation");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "bulk Forms handler/op-codec/diff/apply envelope exceeded 8 ms");
        next = continuation(result);
    }
    assert!(next.is_none(), "bulk Forms job must complete inside the bounded continuation budget");
    let config = crate::editor::forms::testkit::config(&app).await;
    let a_content_id = config.try_values.get_json("a").expect("committed bulk a content id").to_string();
    let b_content_id = config.try_values.get_json("b").expect("committed bulk b content id").to_string();
    assert_eq!(a_content_id.len(), 85);
    assert_eq!(b_content_id.len(), 85);
    let expected: serde_json::Value = serde_json::from_str(&chunks.concat()).expect("bulk input JSON");
    assert_eq!(materialize_owned_try_value(&config.try_values, "a").and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok()), Some(expected["a"].clone()));
    assert_eq!(materialize_owned_try_value(&config.try_values, "b"), Some("1".into()));
    assert!(bulk_sessions().lock().expect("forms bulk sessions lock").is_empty());
    assert!(active_bulk_generations().lock().expect("forms bulk active lock").is_empty());
    let serialized = dsl::os_pack::json::to_json_string(&config).into_bytes();
    crate::editor::forms::config::clear_try_value_staging_for_replay();
    bulk_sessions().lock().expect("forms bulk sessions lock").clear();
    active_bulk_generations().lock().expect("forms bulk active lock").clear();
    *crate::editor::forms::commands::set_try_value::input_registry().lock().expect("forms input registry lock") = crate::editor::forms::commands::set_try_value::FormsInputRegistry::default();
    let reopened: FormsConfig = dsl::os_pack::json::from_json_str(std::str::from_utf8(&serialized).expect("config UTF-8")).expect("cold reopen completed public bulk config");
    assert_eq!(reopened.try_values.get_json("a"), Some(a_content_id.as_str()));
    assert_eq!(reopened.try_values.get_json("b"), Some(b_content_id.as_str()));
    assert_eq!(materialize_owned_try_value(&reopened.try_values, "a").and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok()), Some(expected["a"].clone()));
    assert_eq!(materialize_owned_try_value(&reopened.try_values, "b"), Some("1".into()));
    assert!(bulk_sessions().lock().expect("forms bulk sessions lock").is_empty());
    assert!(active_bulk_generations().lock().expect("forms bulk active lock").is_empty());
}
