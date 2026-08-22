//! 🧪️ 🧪️ Forms play app commands command — `set-try-values`.

use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::commands::set_try_value::{cancel_pending_generations, stage_command_input, ChunkAddressableJson, ChunkedSource, SetTryValueStep, MAX_TRY_VALUE_BYTES_PER_STEP, SET_TRY_VALUE_STEP_ACTION_ID};
use crate::editor::forms::config::{discard_staged_try_value, discard_staged_try_values_batch, FormsConfig, FormsConfigMutation};
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, RequestId};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

//#region 🔖️BulkSession
const MAX_BULK_KEY_BYTES: usize = 4_096;
const MAX_LIVE_BULK_SESSIONS: usize = 64;
static ACTIVE_BULK_GENERATIONS: OnceLock<Mutex<BTreeMap<(String, String, String), u64>>> = OnceLock::new();
static NEXT_BULK_REQUEST: AtomicU64 = AtomicU64::new(40_000);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BulkJobKey {
    app_id: String,
    document_id: String,
    operation_id: String,
    base_revision: String,
    generation: u64,
}

static BULK_SESSIONS: OnceLock<Mutex<BTreeMap<BulkJobKey, BulkSession>>> = OnceLock::new();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BulkPhase {
    Object,
    Key,
    Colon,
    Value,
    Stage,
    Entry,
    Commit,
}

#[derive(Debug)]
struct BulkSession {
    app_id: String,
    document_id: String,
    operation_id: String,
    baseline_root_token: usize,
    baseline_revision: u64,
    source: Arc<ChunkedSource>,
    cursor: usize,
    phase: BulkPhase,
    in_string: bool,
    escaped: bool,
    depth: usize,
    key_start: usize,
    key_end: usize,
    value_start: usize,
    value_end: usize,
    key: String,
    batch_id: String,
    value_staging_id: String,
    staged_cursor: usize,
    staged_chunks: u64,
    digest: [u64; 4],
    digest_len: u64,
    content_id: String,
    entry_count: u64,
    malformed: bool,
}

fn bulk_sessions() -> &'static Mutex<BTreeMap<BulkJobKey, BulkSession>> {
    BULK_SESSIONS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn active_bulk_generations() -> &'static Mutex<BTreeMap<(String, String, String), u64>> {
    ACTIVE_BULK_GENERATIONS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn clear_active_bulk(session: &BulkSession, generation: u64) {
    let scope = (session.app_id.clone(), session.document_id.clone(), session.operation_id.clone());
    let mut active = active_bulk_generations().lock().expect("forms bulk active lock");
    if active.get(&scope) == Some(&generation) {
        active.remove(&scope);
    }
}

pub(crate) fn cancel_pending_bulk(app_instance_id: u32, document_id: &str) -> Vec<FormsConfigMutation> {
    let scopes: Vec<_> = active_bulk_generations().lock().expect("forms bulk active lock").keys().filter(|(app, document, _)| app == &app_instance_id.to_string() && document == document_id).cloned().collect();
    let mut mutations = Vec::new();
    for scope in scopes {
        let generation = active_bulk_generations().lock().expect("forms bulk active lock").remove(&scope);
        let Some(generation) = generation else { continue };
        let key = BulkJobKey { app_id: scope.0, document_id: scope.1, operation_id: scope.2, generation };
        let Some(session) = bulk_sessions().lock().expect("forms bulk sessions lock").remove(&key) else { continue };
        discard_staged_try_value(&session.value_staging_id);
        discard_staged_try_values_batch(&session.batch_id);
        mutations.extend([FormsConfigMutation::DiscardTryValueStaging { staging_id: session.value_staging_id }, FormsConfigMutation::DiscardTryValuesBatch { staging_id: session.batch_id }]);
    }
    mutations
}

fn bulk_queue(generation: u64, cursor: usize, session: &BulkSession) -> Effect {
    Effect::DispatchAction {
        req: RequestId(NEXT_BULK_REQUEST.fetch_add(1, Ordering::Relaxed)),
        action: SET_TRY_VALUE_STEP_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(json!({
            "appId": session.app_id,
            "documentId": session.document_id,
            "operationId": session.operation_id,
            "generation": generation,
            "cursor": cursor as u64,
            "targetIndex": u64::MAX
            ,"baseRevision": session.base_revision
        }))),
        delay_ms: 0,
    }
}

fn bulk_emit(generation: u64, session: BulkSession, mutations: Vec<FormsConfigMutation>) -> Emit<FormMutation, FormsConfigMutation> {
    let cursor = session.cursor;
    let key = BulkJobKey { app_id: session.app_id.clone(), document_id: session.document_id.clone(), operation_id: session.operation_id.clone(), generation };
    let effect = bulk_queue(generation, cursor, &session);
    bulk_sessions().lock().expect("forms bulk sessions lock").insert(key, session);
    Emit { config_mutations: mutations, effects: vec![effect], coalesce_key: Some(format!("setTryValues:{generation}")), ui_scope: UiDirtyScope::None, ..Default::default() }
}

fn update_digest(session: &mut BulkSession, bytes: &[u8]) {
    for byte in bytes {
        session.digest_len = session.digest_len.wrapping_add(1);
        session.digest[0] = (session.digest[0] ^ u64::from(*byte)).wrapping_mul(0x00000100000001b3);
        session.digest[1] = (session.digest[1] ^ session.digest[0].rotate_left(17) ^ session.digest_len).wrapping_mul(0x9e3779b185ebca87);
        session.digest[2] = (session.digest[2] ^ session.digest[1].rotate_left(29) ^ u64::from(*byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
        session.digest[3] = (session.digest[3] ^ session.digest[2].rotate_left(41) ^ session.digest_len.rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
    }
}

fn content_id(session: &BulkSession) -> String {
    format!("try-{:016x}{:016x}{:016x}{:016x}-{:016x}", session.digest[0], session.digest[1], session.digest[2], session.digest[3], session.digest_len)
}

fn reset_entry(session: &mut BulkSession) {
    session.phase = BulkPhase::Key;
    session.in_string = false;
    session.escaped = false;
    session.depth = 0;
    session.key_start = 0;
    session.key_end = 0;
    session.value_start = 0;
    session.value_end = 0;
    session.key.clear();
    session.value_staging_id.clear();
    session.staged_cursor = 0;
    session.staged_chunks = 0;
    session.digest = [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f];
    session.digest_len = 0;
    session.content_id.clear();
    session.verification_cursor = 0;
}

fn scan_bulk(session: &mut BulkSession) -> Result<(), Fault> {
    let limit = session.cursor.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(session.source.len());
    while session.cursor < limit {
        let byte = session.source.byte(session.cursor).expect("bounded bulk input byte");
        match session.phase {
            BulkPhase::Object => {
                if byte.is_ascii_whitespace() {
                    session.cursor += 1;
                    continue;
                }
                if byte != b'{' {
                    session.malformed = true;
                    session.phase = BulkPhase::Commit;
                    return Ok(());
                }
                session.phase = BulkPhase::Key;
            }
            BulkPhase::Key => {
                if byte.is_ascii_whitespace() || byte == b',' {
                    session.cursor += 1;
                    continue;
                }
                if byte == b'}' {
                    session.phase = BulkPhase::Commit;
                    return Ok(());
                }
                if byte != b'"' {
                    session.malformed = true;
                    session.phase = BulkPhase::Commit;
                    return Ok(());
                }
                session.key_start = session.cursor;
                session.in_string = true;
                session.phase = BulkPhase::Colon;
            }
            BulkPhase::Colon => {
                if session.in_string {
                    if session.escaped {
                        session.escaped = false;
                    } else if byte == b'\\' {
                        session.escaped = true;
                    } else if byte == b'"' && session.cursor > session.key_start {
                        session.in_string = false;
                        session.key_end = session.cursor + 1;
                    }
                } else if byte == b':' {
                    let key_token_len = session.key_end.saturating_sub(session.key_start);
                    if key_token_len > MAX_BULK_KEY_BYTES {
                        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-values.key-too-large"), "a bulk Forms key exceeds the bounded key limit"));
                    }
                    session.key = session.source.bounded_range(session.key_start, session.key_end).and_then(|token| serde_json::from_str(&token).ok()).unwrap_or_default();
                    session.phase = BulkPhase::Value;
                }
            }
            BulkPhase::Value => {
                if session.value_start == 0 {
                    if byte.is_ascii_whitespace() {
                        session.cursor += 1;
                        continue;
                    }
                    session.value_start = session.cursor;
                }
                if session.in_string {
                    if session.escaped {
                        session.escaped = false;
                    } else if byte == b'\\' {
                        session.escaped = true;
                    } else if byte == b'"' {
                        session.in_string = false;
                    }
                } else {
                    match byte {
                        b'"' => session.in_string = true,
                        b'{' | b'[' => session.depth += 1,
                        b'}' | b']' if session.depth > 0 => session.depth -= 1,
                        b',' | b'}' if session.depth == 0 => {
                            session.value_end = session.cursor;
                            while session.value_end > session.value_start && session.source.byte(session.value_end - 1).is_some_and(|byte| byte.is_ascii_whitespace()) {
                                session.value_end -= 1;
                            }
                            session.value_staging_id = format!("{}-{}", session.batch_id, session.entry_count);
                            session.phase = BulkPhase::Stage;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
            _ => return Ok(()),
        }
        session.cursor += 1;
    }
    if session.cursor == session.source.len() && session.phase == BulkPhase::Value {
        session.value_end = session.cursor;
        session.value_staging_id = format!("{}-{}", session.batch_id, session.entry_count);
        session.phase = BulkPhase::Stage;
    }
    Ok(())
}
//#endregion 🔖️BulkSession

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "try-values")]
pub struct SetTryValues {
    pub values_json: ChunkAddressableJson,
    pub input_id: Option<String>,
    pub input_index: Option<u64>,
    pub input_count: Option<u64>,
}

fn new_bulk_session(values_json: ChunkedSource, operation: &semio_framework_plugin::AppOperationContext, baseline_root_token: usize, baseline_revision: u64) -> BulkSession {
    let generation = operation.generation;
    BulkSession {
        app_id: operation.app_instance_id.to_string(),
        document_id: operation.parent_document_id.clone(),
        operation_id: operation.operation_id.to_string(),
        base_revision: operation.canonical_base_revision_hex(),
        baseline_root_token,
        baseline_revision,
        source: Arc::new(values_json),
        cursor: 0,
        phase: BulkPhase::Object,
        in_string: false,
        escaped: false,
        depth: 0,
        key_start: 0,
        key_end: 0,
        value_start: 0,
        value_end: 0,
        key: String::new(),
        batch_id: format!("try-values-batch-{generation}"),
        value_staging_id: String::new(),
        staged_cursor: 0,
        staged_chunks: 0,
        digest: [0x6c62272e07bb0142, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f],
        digest_len: 0,
        content_id: String::new(),
        entry_count: 0,
        malformed: false,
    }
}

pub async fn handle(payload: &SetTryValues, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let operation = doc.operation()?;
    let input_count = payload.input_count.unwrap_or(1);
    if input_count > 1 && payload.input_id.is_none() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-values.input-id-required"), "multi-chunk Forms bulk input requires an explicit input id"));
    }
    let input_id = payload.input_id.as_deref().unwrap_or("setTryValues-single");
    let Some(input) = stage_command_input(operation, "setTryValues", input_id, payload.input_index.unwrap_or(0), input_count, payload.values_json.owner())? else {
        return Ok(Emit::default());
    };
    if bulk_sessions().lock().expect("forms bulk sessions lock").len() >= MAX_LIVE_BULK_SESSIONS {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-values.busy"), "the bounded Forms bulk session pool is full"));
    }
    let cleanup = cancel_pending_generations(&input.operation);
    let generation = input.operation.generation;
    let scope = (input.operation.app_instance_id.to_string(), input.operation.parent_document_id.clone(), input.operation.operation_id.to_string());
    active_bulk_generations().lock().expect("forms bulk active lock").insert(scope, generation);
    let session = new_bulk_session(input.source, &input.operation, cfg.snapshot.try_values.root_token(), cfg.snapshot.try_values.revision());
    Ok(bulk_emit(generation, session, cleanup))
}

/// ⏱️ Advances one bulk JSON scan, one 4 KiB value stage, one collision check, or the atomic root swap.
pub(crate) async fn advance_if_bulk(payload: &SetTryValueStep, config: &FormsConfig) -> Option<Result<Emit<FormMutation, FormsConfigMutation>, Fault>> {
    if payload.target_index != u64::MAX {
        return None;
    }
    let key = BulkJobKey { app_id: payload.app_id.clone(), document_id: payload.document_id.clone(), operation_id: payload.operation_id.clone(), generation: payload.generation };
    let Some(mut session) = bulk_sessions().lock().expect("forms bulk sessions lock").remove(&key) else { return Some(Ok(Emit::default())) };
    let active = active_bulk_generations().lock().expect("forms bulk active lock").get(&(payload.app_id.clone(), payload.document_id.clone(), payload.operation_id.clone())).copied();
    if active != Some(payload.generation) || session.baseline_root_token != config.try_values.root_token() || session.baseline_revision != config.try_values.revision() || payload.cursor != session.cursor as u64 {
        discard_staged_try_value(&session.value_staging_id);
        discard_staged_try_values_batch(&session.batch_id);
        clear_active_bulk(&session, payload.generation);
        return Some(Ok(Emit::default()));
    }
    let outcome = match session.phase {
        BulkPhase::Object | BulkPhase::Key | BulkPhase::Colon | BulkPhase::Value => match scan_bulk(&mut session) {
            Ok(()) => Ok(bulk_emit(payload.generation, session, Vec::new())),
            Err(fault) => {
                discard_staged_try_value(&session.value_staging_id);
                discard_staged_try_values_batch(&session.batch_id);
                clear_active_bulk(&session, payload.generation);
                Err(fault)
            }
        },
        BulkPhase::Stage => {
            let start = session.staged_cursor;
            let mut absolute_cursor = session.value_start + start;
            let mut parts = Vec::new();
            session.source.append_range(&mut absolute_cursor, session.value_end, &mut parts);
            let end = absolute_cursor - session.value_start;
            let chunk = parts.iter().fold(String::new(), |mut output, part| {
                output.push_str(part);
                output
            });
            update_digest(&mut session, chunk.as_bytes());
            let mutation = FormsConfigMutation::StageTryValueChunk { staging_id: session.value_staging_id.clone(), index: session.staged_chunks, chunk };
            session.staged_chunks += 1;
            session.staged_cursor = end;
            if end == session.value_end - session.value_start {
                session.content_id = content_id(&session);
                session.phase = BulkPhase::Entry;
            }
            Ok(bulk_emit(payload.generation, session, vec![mutation]))
        }
        BulkPhase::Entry => {
            let mutation = FormsConfigMutation::StageTryValuesEntry {
                staging_id: session.batch_id.clone(),
                key: session.key.clone(),
                value_staging_id: session.value_staging_id.clone(),
                content_id: session.content_id.clone(),
                chunk_count: session.staged_chunks,
            };
            session.entry_count += 1;
            reset_entry(&mut session);
            Ok(bulk_emit(payload.generation, session, vec![mutation]))
        }
        BulkPhase::Commit if session.malformed => {
            discard_staged_try_values_batch(&session.batch_id);
            clear_active_bulk(&session, payload.generation);
            Ok(Emit::default())
        }
        BulkPhase::Commit if session.entry_count == 0 => {
            clear_active_bulk(&session, payload.generation);
            Ok(Emit::default())
        }
        BulkPhase::Commit => {
            clear_active_bulk(&session, payload.generation);
            Ok(Emit {
                config_mutations: vec![FormsConfigMutation::CommitTryValuesBatch { staging_id: session.batch_id, entry_count: session.entry_count }],
                coalesce_key: Some(format!("setTryValues:{}", payload.generation)),
                ui_scope: UiDirtyScope::Full,
                ..Default::default()
            })
        }
    };
    Some(outcome)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
        let mut session = new_bulk_session(rope(&[r#"{"na"#, r#"me":{"nested":[1,2,3]}}"#]), 1, "doc-a", 1, 0);
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
        let a = BulkJobKey { app_id: "1".into(), document_id: "a".into(), operation_id: "11".into(), generation: 7 };
        let b = BulkJobKey { app_id: "1".into(), document_id: "b".into(), operation_id: "12".into(), generation: 7 };
        assert_ne!(a, b);
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
        app.handle_action("setTryValues", Some(&before_restart), &meta("bulk")).await.expect("initial bulk action log entry");

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
            result = Some(app.handle_action("setTryValues", Some(&args("bulk-after-restart", index)), &meta("bulk")).await.expect("replayed bulk Forms action"));
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "bulk Forms public action envelope exceeded 8 ms");
        }
        let mut next = result.and_then(continuation);
        for _ in 0..128 {
            let Some(checkpoint) = next.take() else { break };
            let started = std::time::Instant::now();
            let result = app.handle_action(SET_TRY_VALUE_STEP_ACTION_ID, Some(&checkpoint), &meta("bulk")).await.expect("bulk Forms continuation");
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "bulk Forms handler/op-codec/diff/apply envelope exceeded 8 ms");
            next = continuation(result);
        }
        assert!(next.is_none(), "bulk Forms job must complete inside the bounded continuation budget");
        let config = app.test_config().await;
        let a_content_id = config.try_values.get_json("a").expect("committed bulk a content id").to_string();
        let b_content_id = config.try_values.get_json("b").expect("committed bulk b content id").to_string();
        assert_eq!(a_content_id.len(), 85);
        assert_eq!(b_content_id.len(), 85);
        let expected: serde_json::Value = serde_json::from_str(&chunks.concat()).expect("bulk input JSON");
        assert_eq!(materialize_owned_try_value(&config.try_values, "a").and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok()), Some(expected["a"].clone()));
        assert_eq!(materialize_owned_try_value(&config.try_values, "b"), Some("1".into()));
        assert!(bulk_sessions().lock().expect("forms bulk sessions lock").is_empty());
        assert!(active_bulk_generations().lock().expect("forms bulk active lock").is_empty());
        let serialized = serde_json::to_vec(&config).expect("serialize completed public bulk config");
        crate::editor::forms::config::clear_try_value_staging_for_replay();
        bulk_sessions().lock().expect("forms bulk sessions lock").clear();
        active_bulk_generations().lock().expect("forms bulk active lock").clear();
        *crate::editor::forms::commands::set_try_value::input_registry().lock().expect("forms input registry lock") = crate::editor::forms::commands::set_try_value::FormsInputRegistry::default();
        let reopened: FormsConfig = serde_json::from_slice(&serialized).expect("cold reopen completed public bulk config");
        assert_eq!(reopened.try_values.get_json("a"), Some(a_content_id.as_str()));
        assert_eq!(reopened.try_values.get_json("b"), Some(b_content_id.as_str()));
        assert_eq!(materialize_owned_try_value(&reopened.try_values, "a").and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok()), Some(expected["a"].clone()));
        assert_eq!(materialize_owned_try_value(&reopened.try_values, "b"), Some("1".into()));
        assert!(bulk_sessions().lock().expect("forms bulk sessions lock").is_empty());
        assert!(active_bulk_generations().lock().expect("forms bulk active lock").is_empty());
    }
}
//#endregion 🧪️Tests
