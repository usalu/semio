
use super::*;
use crate::editor::forms::FormsCommand;
use crate::editor::forms::testkit::{FormsApp, forms_app_with_registry};
use semio_framework::kernel::Effect;
use semio_framework_plugin::PluginApp;
use semio_framework_plugin::testkit::meta;

fn rope(raw: &str) -> std::sync::Arc<ChunkedSource> {
    std::sync::Arc::new(ChunkedSource::from_text(raw.into()))
}

fn test_operation(document_id: &str, operation_id: u64) -> semio_framework_plugin::AppOperationContext {
    semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: document_id.into(), operation_id, generation: 1, canonical_base_revision: [0; 32] }
}

async fn app_with_document_id(id: &str) -> FormsApp {
    let mut app = forms_app_with_registry().await;
    let mut snapshot = app.snapshot().expect("Forms snapshot");
    snapshot.id = id.into();
    let envelope = store::create_document_envelope::<_, FormMutation>(crate::FORMS_DOCUMENT_SCHEMA, id, snapshot, None);
    let files = store::print_document_pack(&envelope).await.expect("Forms document pack");
    app.load_document_pack(&files).await.expect("load Forms document identity");
    app
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

fn finish_rewrite(mut rewrite: VectorRewrite, source: &str) -> (String, std::time::Duration) {
    let mut source = ChunkedSource::from_text(source.to_string());
    let mut worst = std::time::Duration::ZERO;
    loop {
        let started = std::time::Instant::now();
        let step = rewrite.advance(&source);
        worst = worst.max(started.elapsed());
        assert!(step.bytes <= MAX_TRY_VALUE_BYTES_PER_STEP.max(MAX_VECTOR_COMPONENTS_PER_STEP * 2));
        assert!(step.components <= MAX_VECTOR_COMPONENTS_PER_STEP);
        if step.complete {
            let output = rewrite.take_output().expect("completed rewrite output");
            if rewrite.is_final_preview() {
                return (output.materialize(), worst);
            }
            rewrite = rewrite.restart();
            source = output;
        }
    }
}

fn finish_container(mut rewrite: ContainerRewrite, source: &str) -> (String, std::time::Duration) {
    let source = ChunkedSource::from_text(source.to_string());
    let mut worst = std::time::Duration::ZERO;
    loop {
        let started = std::time::Instant::now();
        let step = rewrite.advance(&source);
        worst = worst.max(started.elapsed());
        assert!(step.bytes <= MAX_TRY_VALUE_BYTES_PER_STEP);
        if step.complete {
            return (rewrite.take_output().expect("completed container rewrite").materialize(), worst);
        }
    }
}

#[test]
fn vector_replacement_boundaries_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️vectors.json")).expect("vector fixture");
    for vector in vectors.as_array().expect("vector cases") {
        let source = vector["source"].as_str().expect("source JSON");
        let index = vector["index"].as_u64().expect("target index");
        let replacement = vector["value"].to_string();
        let (actual, _) = finish_rewrite(VectorRewrite::new(rope(&replacement), index), source);
        let mut oracle = serde_json::from_str::<serde_json::Value>(source).ok().and_then(|value| value.as_array().cloned()).unwrap_or_default();
        oracle.resize(oracle.len().max(index as usize + 1), json!(0));
        oracle[index as usize] = vector["value"].clone();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&actual).expect("rewritten JSON"), serde_json::Value::Array(oracle), "{source}");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&actual).unwrap(), vector["expected"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn large_unrelated_config_and_existing_vector_stay_under_one_bounded_slice() {
    let vector = (0..40_000).map(|_| "1").collect::<Vec<_>>().join(",");
    let (json, worst) = finish_rewrite(VectorRewrite::new(rope("9"), 39_999), &format!("[{vector}]"));
    let values: serde_json::Value = serde_json::from_str(&json).expect("rewritten JSON");
    assert_eq!(values.as_array().map(Vec::len), Some(40_000));
    assert_eq!(values[39_999], json!(9));
    assert!(worst < std::time::Duration::from_millis(8), "bounded Forms rewrite slice took {worst:?}");
}

#[semio_framework_async_macros::async_test]
async fn vector_growth_writes_at_most_sixty_four_components_per_slice() {
    let source = r#"[1,2]"#;
    let (json, worst) = finish_rewrite(VectorRewrite::new(rope("7"), 10_000), source);
    let values: serde_json::Value = serde_json::from_str(&json).expect("rewritten JSON");

    assert_eq!(values.as_array().map(Vec::len), Some(10_001));
    assert_eq!(values[0], json!(1));
    assert_eq!(values[10_000], json!(7));
    assert!(worst < std::time::Duration::from_millis(8), "bounded Forms growth slice took {worst:?}");
}

#[semio_framework_async_macros::async_test]
async fn missing_non_array_and_malformed_targets_keep_best_effort_semantics() {
    let (missing, _) = finish_rewrite(VectorRewrite::new(rope("5"), 1), "null");
    let (non_array, _) = finish_rewrite(VectorRewrite::new(rope("6"), 1), "false");
    let (malformed, _) = finish_rewrite(VectorRewrite::new(rope("7"), 1), "not-json");

    assert_eq!(serde_json::from_str::<serde_json::Value>(&missing).expect("missing target rewrite"), json!([0, 5]));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&non_array).expect("non-array target rewrite"), json!([0, 6]));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&malformed).expect("malformed source rewrite"), json!([0, 7]));
}

#[semio_framework_async_macros::async_test]
async fn scalar_option_and_object_shapes_stay_intact() {
    let (options, option_worst) = finish_container(ContainerRewrite::option("a", rope("true")).expect("option rewrite"), "[]");
    let (object, object_worst) = finish_container(ContainerRewrite::object("height", rope("5")).expect("object rewrite"), "{}");
    assert_eq!(options, r#"["a"]"#);
    assert_eq!(object, r#"{"height":5}"#);
    assert!(option_worst < std::time::Duration::from_millis(8));
    assert!(object_worst < std::time::Duration::from_millis(8));
}

#[test]
fn public_input_handoff_is_chunk_bounded_scoped_and_has_explicit_backpressure() {
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    let chunk = std::sync::Arc::<str>::from("x".repeat(MAX_TRY_VALUE_BYTES_PER_STEP));
    for index in 0..MAX_LIVE_TRY_VALUE_SESSIONS {
        let result = stage_command_input(&test_operation(&format!("doc-{index}"), index as u64 + 1), "setTryValue", "input", 0, 2, chunk.clone()).expect("bounded input admission");
        assert!(result.is_none());
    }
    assert!(stage_command_input(&test_operation("doc-65", 65), "setTryValue", "input", 0, 2, chunk).is_err());
    assert_eq!(input_registry().lock().expect("forms input registry lock").blobs.len(), MAX_LIVE_TRY_VALUE_SESSIONS);
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
}

#[test]
fn public_input_handoff_never_materializes_the_whole_value() {
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    let operation = test_operation("doc-a", 7);
    let first = stage_command_input(&operation, "setTryValue", "generation-a", 0, 2, std::sync::Arc::from("a".repeat(MAX_TRY_VALUE_BYTES_PER_STEP))).expect("first chunk");
    assert!(first.is_none());
    let complete = stage_command_input(&test_operation("doc-a", 8), "setTryValue", "generation-a", 1, 2, std::sync::Arc::from("b".repeat(MAX_TRY_VALUE_BYTES_PER_STEP))).expect("second chunk").expect("completed rope");
    assert_eq!(complete.source.len(), MAX_TRY_VALUE_BYTES_PER_STEP * 2);
    assert_eq!(complete.source.chunks.len(), 2, "completed handoff retains chunk leaves instead of joining them");
    assert_eq!(complete.operation.operation_id, operation.operation_id, "the first admitted operation remains the durable continuation owner");
}

#[test]
fn same_document_two_app_instances_cancel_and_restart_independently() {
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    let first = test_operation("shared-document", 11);
    let mut second = test_operation("shared-document", 12);
    second.app_instance_id = 2;
    assert!(stage_command_input(&first, "setTryValue", "same-input", 0, 2, std::sync::Arc::from("a")).expect("first app stage").is_none());
    assert!(stage_command_input(&second, "setTryValue", "same-input", 0, 2, std::sync::Arc::from("b")).expect("second app stage").is_none());
    assert_eq!(input_registry().lock().expect("forms input registry lock").blobs.len(), 2);
    cancel_command_inputs(first.app_instance_id, &first.parent_document_id);
    let remaining: Vec<_> = input_registry().lock().expect("forms input registry lock").blobs.keys().map(|key| key.app_instance_id).collect();
    assert_eq!(remaining, vec![second.app_instance_id]);
    assert!(stage_command_input(&second, "setTryValue", "same-input", 1, 2, std::sync::Arc::from("c")).expect("second app restart").is_some());
}

#[test]
fn pathological_counts_and_abandoned_inputs_are_bounded() {
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    assert!(stage_command_input(&test_operation("doc", 1), "setTryValue", "pathological", 0, MAX_COMMAND_INPUT_CHUNKS + 1, std::sync::Arc::from("x")).is_err());
    assert!(stage_command_input(&test_operation("doc", 2), "setTryValue", "abandoned", 0, 2, std::sync::Arc::from("x")).expect("initial chunk").is_none());
    input_registry().lock().expect("forms input registry lock").tick = MAX_COMMAND_INPUT_IDLE_ACTIONS + 2;
    assert!(stage_command_input(&test_operation("other", 3), "setTryValue", "fresh", 0, 2, std::sync::Arc::from("y")).expect("expiry-driving chunk").is_none());
    assert!(!input_registry().lock().expect("forms input registry lock").blobs.keys().any(|key| key.input_id == "abandoned"));
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_replays_chunked_input_after_process_registry_loss_under_eight_ms() {
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    sessions().lock().expect("forms sessions lock").clear();
    active_generations().lock().expect("forms active generations lock").clear();
    let chunks = [format!("\"{}", "a".repeat(MAX_TRY_VALUE_BYTES_PER_STEP - 1)), format!("{}\"", "b".repeat(MAX_TRY_VALUE_BYTES_PER_STEP - 1))];
    let mut app = forms_app_with_registry().await;
    let dispatch_chunks = |input_id: &str| {
        chunks
            .iter()
            .enumerate()
            .map(|(index, chunk)| {
                serde_json::json!({
                    "key": "public-scalar",
                    "valueJson": chunk,
                    "inputId": input_id,
                    "inputIndex": index as u64,
                    "inputCount": chunks.len() as u64
                })
            })
            .collect::<Vec<_>>()
    };
    let checkpoint_actions = dispatch_chunks("before-restart");
    for args in &checkpoint_actions {
        let command = FormsCommand::SetTryValue(SetTryValue {
            key: "public-scalar".into(),
            value_json: args.get("valueJson").and_then(serde_json::Value::as_str).map(Into::into),
            input_id: args.get("inputId").and_then(serde_json::Value::as_str).map(str::to_string),
            input_index: args.get("inputIndex").and_then(serde_json::Value::as_u64),
            input_count: args.get("inputCount").and_then(serde_json::Value::as_u64),
            ..Default::default()
        });
        let started = std::time::Instant::now();
        let wire = <FormsCommand as protocol::OpBinary>::encode_op(&command).expect("public command encode");
        let decoded = <FormsCommand as protocol::OpBinary>::decode_op(&wire).expect("public command decode");
        assert_eq!(decoded, command);
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Forms public command codec envelope exceeded 8 ms");
        let started = std::time::Instant::now();
        app.handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&args)), &meta("local")).await.expect("public action dispatch");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Forms public action envelope exceeded 8 ms");
    }

    sessions().lock().expect("forms sessions lock").clear();
    active_generations().lock().expect("forms active generations lock").clear();
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();

    let mut result = None;
    for args in dispatch_chunks("after-restart") {
        let started = std::time::Instant::now();
        result = Some(app.handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&args)), &meta("local")).await.expect("replayed public action dispatch"));
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "replayed Forms public action envelope exceeded 8 ms");
    }
    for _ in 0..128 {
        let current = result.take().expect("Forms continuation result");
        let next = current.requested_effects.into_iter().find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == SET_TRY_VALUE_STEP_ACTION_ID => args.map(store::pack_rt::dsl_value_to_json),
            _ => None,
        });
        let Some(args) = next else { break };
        let started = std::time::Instant::now();
        result = Some(app.handle_action(SET_TRY_VALUE_STEP_ACTION_ID, Some(&crate::editor::forms::testkit::action_args(&args)), &meta("local")).await.expect("Forms continuation action dispatch"));
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Forms handler/job/op-codec/diff/apply envelope exceeded 8 ms");
    }
    let config = crate::editor::forms::testkit::config(&app).await;
    let content_id = config.try_values.get_json("public-scalar").expect("committed scalar content id").to_string();
    assert_eq!(content_id.len(), 85);
    assert_eq!(materialize_owned_try_value(&config.try_values, "public-scalar"), Some(chunks.concat()));
    assert!(input_registry().lock().expect("forms input registry lock").blobs.is_empty());
    assert!(sessions().lock().expect("forms sessions lock").is_empty());
    assert!(active_generations().lock().expect("forms active generations lock").is_empty());
    let serialized = dsl::os_pack::json::to_json_string(&config).into_bytes();
    crate::editor::forms::config::clear_try_value_staging_for_replay();
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    sessions().lock().expect("forms sessions lock").clear();
    active_generations().lock().expect("forms active generations lock").clear();
    let reopened: FormsConfig = dsl::os_pack::json::from_json_str(std::str::from_utf8(&serialized).expect("config UTF-8")).expect("cold reopen completed public scalar config");
    assert_eq!(reopened.try_values.get_json("public-scalar"), Some(content_id.as_str()));
    assert_eq!(materialize_owned_try_value(&reopened.try_values, "public-scalar"), Some(chunks.concat()));
    assert!(sessions().lock().expect("forms sessions lock").is_empty());
    assert!(active_generations().lock().expect("forms active generations lock").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_isolates_two_documents_and_cancellation() {
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    sessions().lock().expect("forms sessions lock").clear();
    active_generations().lock().expect("forms active generations lock").clear();
    let mut first = app_with_document_id("forms-document-a").await;
    let mut second = app_with_document_id("forms-document-b").await;
    let chunk = |document: &str, index: u64| {
        serde_json::json!({
            "key": "shared-key",
            "valueJson": if index == 0 { "\"first" } else { "second\"" },
            "inputId": format!("{document}-input"),
            "inputIndex": index,
            "inputCount": 2
        })
    };
    first.handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&chunk("a", 0))), &meta("document-a")).await.expect("first Forms chunk A");
    second.handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&chunk("b", 0))), &meta("document-b")).await.expect("first Forms chunk B");
    assert_eq!(input_registry().lock().expect("forms input registry lock").blobs.len(), 2);
    let first_step = continuation(first.handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&chunk("a", 1))), &meta("document-a")).await.expect("second Forms chunk A")).expect("Forms continuation A");
    let second_step = continuation(second.handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&chunk("b", 1))), &meta("document-b")).await.expect("second Forms chunk B")).expect("Forms continuation B");
    let first_payload: SetTryValueStep = dsl::os_pack::json::from_json_str(&first_step.to_string()).expect("Forms checkpoint A");
    let second_payload: SetTryValueStep = dsl::os_pack::json::from_json_str(&second_step.to_string()).expect("Forms checkpoint B");
    assert_eq!(first_payload.document_id, "forms-document-a");
    assert_eq!(second_payload.document_id, "forms-document-b");

    first.handle_action("resetTry", Some(&crate::editor::forms::testkit::action_args(&serde_json::json!({}))), &meta("document-a")).await.expect("cancel Forms document A");
    assert!(!sessions().lock().expect("forms sessions lock").keys().any(|key| key.document_id == "forms-document-a"));
    assert!(sessions().lock().expect("forms sessions lock").keys().any(|key| key.document_id == "forms-document-b"));
    let sibling = second.handle_action(SET_TRY_VALUE_STEP_ACTION_ID, Some(&crate::editor::forms::testkit::action_args(&second_step)), &meta("document-b")).await.expect("continue Forms document B");
    assert!(continuation(sibling).is_some(), "cancelling one Forms document must not cancel another");
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_bounds_pathological_abandoned_and_sixty_fifth_inputs() {
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    let mut pathological = app_with_document_id("forms-pathological").await;
    let started = std::time::Instant::now();
    let rejected = pathological
        .handle_action(
            "setTryValue",
            Some(&crate::editor::forms::testkit::action_args(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "pathological", "inputIndex": 0, "inputCount": MAX_COMMAND_INPUT_CHUNKS + 1 }))),
            &meta("pathological"),
        )
        .await;
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "pathological Forms count rejection exceeded 8 ms");
    assert_eq!(rejected.expect_err("pathological Forms count must be rejected").code.0, "forms.try-value.input-invalid");

    let mut abandoned = app_with_document_id("forms-abandoned").await;
    abandoned
        .handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "abandoned", "inputIndex": 0, "inputCount": 2 }))), &meta("abandoned"))
        .await
        .expect("stage abandoned Forms input");
    input_registry().lock().expect("forms input registry lock").tick = MAX_COMMAND_INPUT_IDLE_ACTIONS + 2;
    let mut expiry_driver = app_with_document_id("forms-expiry-driver").await;
    expiry_driver
        .handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "fresh", "inputIndex": 0, "inputCount": 2 }))), &meta("expiry-driver"))
        .await
        .expect("expire abandoned Forms input through public dispatch");
    assert!(!input_registry().lock().expect("forms input registry lock").blobs.keys().any(|key| key.input_id == "abandoned"));
    expiry_driver.handle_action("resetTry", Some(&crate::editor::forms::testkit::action_args(&serde_json::json!({}))), &meta("expiry-driver")).await.expect("cancel incomplete Forms input");
    assert!(!input_registry().lock().expect("forms input registry lock").blobs.keys().any(|key| key.document_id == "forms-expiry-driver"));

    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    let mut admitted = Vec::new();
    for index in 0..MAX_LIVE_TRY_VALUE_SESSIONS {
        let mut app = app_with_document_id(&format!("forms-admission-{index}")).await;
        let started = std::time::Instant::now();
        app.handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "input", "inputIndex": 0, "inputCount": 2 }))), &meta("admission"))
            .await
            .expect("admit bounded Forms input");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Forms admitted action envelope exceeded 8 ms");
        admitted.push(app);
    }
    let mut sixty_fifth = app_with_document_id("forms-admission-65").await;
    let started = std::time::Instant::now();
    let busy = sixty_fifth
        .handle_action("setTryValue", Some(&crate::editor::forms::testkit::action_args(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "input", "inputIndex": 0, "inputCount": 2 }))), &meta("admission"))
        .await
        .expect_err("the 65th Forms public input must be Busy");
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "Forms 65th Busy envelope exceeded 8 ms");
    assert_eq!(busy.code.0, "forms.try-value.busy");
    assert_eq!(input_registry().lock().expect("forms input registry lock").blobs.len(), MAX_LIVE_TRY_VALUE_SESSIONS);
    drop(admitted);
    *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
}
