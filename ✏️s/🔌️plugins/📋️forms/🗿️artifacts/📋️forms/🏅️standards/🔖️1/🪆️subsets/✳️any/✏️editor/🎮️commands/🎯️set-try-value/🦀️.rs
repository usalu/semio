//! 🎯️ Bounded Try-value work addressed to one exact Forms Try window lease.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::{try_value_content_id, FormsTryValues, FormsTryWindowLease, FormsTryWindowTransient, MAX_TRY_VALUE_ENTRIES};
use crate::{op::FormMutation, FormsSnapshot};
use dsl::os_pack::json::{Object, Value};
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, RequestId};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::de::Deserializer;
#[cfg(test)]
use serde::ser::Serializer;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

pub const SET_TRY_VALUE_STEP_ACTION_ID: &str = "setTryValueStep";
pub(crate) const MAX_TRY_VALUE_BYTES_PER_STEP: usize = 4_096;
const MAX_COMMAND_INPUT_BYTES: usize = 16_384;
const MAX_COMMAND_INPUT_CHUNKS: u64 = 16_384;
const MAX_LIVE_TRY_VALUE_SESSIONS: usize = 64;
static NEXT_TRY_VALUE_REQUEST: AtomicU64 = AtomicU64::new(20_000);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChunkAddressableJson(Arc<str>);

impl ChunkAddressableJson {
    pub(crate) fn owner(&self) -> Arc<str> { self.0.clone() }
}

impl std::ops::Deref for ChunkAddressableJson {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<String> for ChunkAddressableJson {
    fn from(value: String) -> Self { Self(Arc::from(value)) }
}

impl From<&str> for ChunkAddressableJson {
    fn from(value: &str) -> Self { Self(Arc::from(value)) }
}

#[cfg(test)]
impl Serialize for ChunkAddressableJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_str(self) }
}

#[cfg(test)]
impl<'de> Deserialize<'de> for ChunkAddressableJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChunkAddressableJson;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a JSON chunk of at most 4,096 UTF-8 bytes") }
            fn visit_borrowed_str<E: serde::de::Error>(self, value: &'de str) -> Result<Self::Value, E> { self.visit_str(value) }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                if value.len() > MAX_TRY_VALUE_BYTES_PER_STEP { return Err(E::custom("Forms command JSON chunks are limited to 4,096 UTF-8 bytes")); }
                Ok(value.into())
            }
            fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
                if value.len() > MAX_TRY_VALUE_BYTES_PER_STEP { return Err(E::custom("Forms command JSON chunks are limited to 4,096 UTF-8 bytes")); }
                Ok(value.into())
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl dsl::ToValue for ChunkAddressableJson {
    fn to_value(&self) -> dsl::DslValue { dsl::DslValue::String(self.to_string()) }
}

impl dsl::FromValue for ChunkAddressableJson {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let dsl::DslValue::String(value) = value else { return Err(dsl::ValueError::new("expected a JSON chunk string")) };
        if value.len() > MAX_TRY_VALUE_BYTES_PER_STEP { return Err(dsl::ValueError::new("Forms command JSON chunks are limited to 4,096 UTF-8 bytes")); }
        Ok(value.into())
    }
}

impl dsl::DslField for ChunkAddressableJson {
    fn shape() -> dsl::Shape { dsl::Shape::Text }
    fn to_value(&self) -> dsl::FieldValue { dsl::FieldValue::Text(self.to_string()) }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Text(value) = value else { return Err(format!("expected Text, found {value:?}")) };
        if value.len() > MAX_TRY_VALUE_BYTES_PER_STEP { return Err("Forms command JSON chunks are limited to 4,096 UTF-8 bytes".into()); }
        Ok(value.as_str().into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "try-value")]
pub struct SetTryValue {
    pub key: String,
    pub value_json: Option<ChunkAddressableJson>,
    pub input_id: Option<String>,
    pub input_index: Option<u64>,
    pub input_count: Option<u64>,
    pub option_value: Option<String>,
    pub vector_index: Option<u64>,
    pub param_key: Option<String>,
    pub window_id: String,
    pub window_kind_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "try-value-step")]
pub struct SetTryValueStep {
    pub app_id: String,
    pub document_id: String,
    pub operation_id: String,
    pub generation: u64,
    pub cursor: u64,
    pub target_index: u64,
    pub base_revision: String,
    pub window_id: String,
    pub window_kind_id: String,
    pub window_generation: u64,
    pub document_generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FormsInputKey {
    app_id: String,
    document_id: String,
    operation_id: u64,
    generation: u64,
    base_revision: [u8; 32],
    semantic_id: String,
    input_id: String,
    window_id: String,
    window_kind_id: String,
    window_generation: u64,
    document_generation: u64,
}

#[derive(Default)]
struct FormsInputBuffer {
    chunks: BTreeMap<u64, Arc<str>>,
    chunk_count: u64,
    retained_bytes: usize,
}

#[derive(Default)]
pub(crate) struct FormsInputRegistry {
    buffers: BTreeMap<FormsInputKey, FormsInputBuffer>,
}

static INPUT_REGISTRY: OnceLock<Mutex<FormsInputRegistry>> = OnceLock::new();

pub(crate) fn input_registry() -> &'static Mutex<FormsInputRegistry> { INPUT_REGISTRY.get_or_init(|| Mutex::new(FormsInputRegistry::default())) }

#[derive(Clone)]
pub(crate) struct StagedCommandInput {
    pub operation: semio_framework_plugin::AppOperationContext,
    pub source: Arc<str>,
}

pub(crate) fn stage_command_input(
    operation: &semio_framework_plugin::AppOperationContext,
    lease: &FormsTryWindowLease,
    semantic_id: &str,
    input_id: &str,
    index: u64,
    chunk_count: u64,
    chunk: Arc<str>,
) -> Result<Option<StagedCommandInput>, Fault> {
    if chunk_count == 0 || chunk_count > MAX_COMMAND_INPUT_CHUNKS || index >= chunk_count || chunk.len() > MAX_TRY_VALUE_BYTES_PER_STEP {
        return Err(fault("forms.try-value.input-invalid", "the bounded Forms input identity or chunk extent is invalid"));
    }
    let key = FormsInputKey {
        app_id: operation.app_instance_id.to_string(),
        document_id: operation.parent_document_id.clone(),
        operation_id: operation.operation_id,
        generation: operation.generation,
        base_revision: operation.canonical_base_revision,
        semantic_id: semantic_id.into(),
        input_id: input_id.into(),
        window_id: lease.window_id.clone(),
        window_kind_id: lease.window_kind_id.clone(),
        window_generation: lease.window_generation,
        document_generation: lease.document_generation,
    };
    let mut registry = input_registry().lock().expect("forms input registry lock");
    if !registry.buffers.contains_key(&key) && registry.buffers.len() >= MAX_LIVE_TRY_VALUE_SESSIONS {
        return Err(fault("forms.try-value.input-busy", "the bounded Forms input pool is full"));
    }
    let buffer = registry.buffers.entry(key.clone()).or_insert_with(|| FormsInputBuffer { chunk_count, ..Default::default() });
    if buffer.chunk_count != chunk_count || buffer.chunks.contains_key(&index) {
        registry.buffers.remove(&key);
        return Err(fault("forms.try-value.input-stale", "the Forms input sequence is stale"));
    }
    buffer.retained_bytes = buffer.retained_bytes.saturating_add(chunk.len());
    if buffer.retained_bytes > MAX_COMMAND_INPUT_BYTES {
        registry.buffers.remove(&key);
        return Err(fault("forms.try-value.input-too-large", "the bounded Forms input exceeds 16,384 UTF-8 bytes"));
    }
    buffer.chunks.insert(index, chunk);
    if buffer.chunks.len() as u64 != chunk_count { return Ok(None); }
    let buffer = registry.buffers.remove(&key).expect("completed Forms input buffer");
    let mut source = String::with_capacity(buffer.retained_bytes);
    for index in 0..chunk_count {
        let Some(chunk) = buffer.chunks.get(&index) else { return Err(fault("forms.try-value.input-order", "the Forms input sequence has a missing chunk")) };
        source.push_str(chunk);
    }
    Ok(Some(StagedCommandInput { operation: operation.clone(), source: Arc::from(source) }))
}

fn cancel_inputs(operation: &semio_framework_plugin::AppOperationContext, lease: &FormsTryWindowLease) {
    input_registry().lock().expect("forms input registry lock").buffers.retain(|key, _| {
        key.app_id != operation.app_instance_id.to_string()
            || key.document_id != operation.parent_document_id
            || key.window_id != lease.window_id
            || key.window_kind_id != lease.window_kind_id
            || key.window_generation != lease.window_generation
            || key.document_generation != lease.document_generation
    });
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FormsJobKey {
    app_id: String,
    document_id: String,
    operation_id: String,
    generation: u64,
    window_id: String,
    window_kind_id: String,
    window_generation: u64,
    document_generation: u64,
}

enum TryValueWork {
    Single { payload: SetTryValue, source: Arc<str> },
    Bulk { source: Arc<str> },
}

struct TryValueSession {
    key: FormsJobKey,
    base_revision: String,
    baseline_root_token: usize,
    baseline_revision: u64,
    work: TryValueWork,
}

static SESSIONS: OnceLock<Mutex<BTreeMap<FormsJobKey, TryValueSession>>> = OnceLock::new();

fn sessions() -> &'static Mutex<BTreeMap<FormsJobKey, TryValueSession>> { SESSIONS.get_or_init(|| Mutex::new(BTreeMap::new())) }

pub(crate) fn cancel_pending_generations(operation: &semio_framework_plugin::AppOperationContext, lease: &FormsTryWindowLease) {
    cancel_inputs(operation, lease);
    sessions().lock().expect("forms Try sessions lock").retain(|key, _| {
        key.app_id != operation.app_instance_id.to_string()
            || key.document_id != operation.parent_document_id
            || key.window_id != lease.window_id
            || key.window_kind_id != lease.window_kind_id
            || key.window_generation != lease.window_generation
            || key.document_generation != lease.document_generation
    });
}

fn job_key(operation: &semio_framework_plugin::AppOperationContext, lease: &FormsTryWindowLease) -> FormsJobKey {
    FormsJobKey {
        app_id: operation.app_instance_id.to_string(),
        document_id: operation.parent_document_id.clone(),
        operation_id: operation.operation_id.to_string(),
        generation: operation.generation,
        window_id: lease.window_id.clone(),
        window_kind_id: lease.window_kind_id.clone(),
        window_generation: lease.window_generation,
        document_generation: lease.document_generation,
    }
}

fn payload_key(payload: &SetTryValueStep) -> FormsJobKey {
    FormsJobKey {
        app_id: payload.app_id.clone(),
        document_id: payload.document_id.clone(),
        operation_id: payload.operation_id.clone(),
        generation: payload.generation,
        window_id: payload.window_id.clone(),
        window_kind_id: payload.window_kind_id.clone(),
        window_generation: payload.window_generation,
        document_generation: payload.document_generation,
    }
}

fn continuation(session: &TryValueSession) -> SetTryValueStep {
    SetTryValueStep {
        app_id: session.key.app_id.clone(),
        document_id: session.key.document_id.clone(),
        operation_id: session.key.operation_id.clone(),
        generation: session.key.generation,
        cursor: 0,
        target_index: matches!(session.work, TryValueWork::Bulk { .. }).then_some(u64::MAX).unwrap_or(0),
        base_revision: session.base_revision.clone(),
        window_id: session.key.window_id.clone(),
        window_kind_id: session.key.window_kind_id.clone(),
        window_generation: session.key.window_generation,
        document_generation: session.key.document_generation,
    }
}

fn queue(payload: &SetTryValueStep) -> Effect {
    Effect::DispatchAction { req: RequestId(NEXT_TRY_VALUE_REQUEST.fetch_add(1, Ordering::Relaxed)), action: SET_TRY_VALUE_STEP_ACTION_ID.into(), args: Some(dsl::ToValue::to_value(payload)), delay_ms: 0 }
}

pub(crate) struct TryWindowCommandOutput {
    pub emit: Emit<FormMutation, FormsConfigMutation>,
    pub transient: Option<FormsTryWindowTransient>,
}

impl TryWindowCommandOutput {
    fn emit(emit: Emit<FormMutation, FormsConfigMutation>) -> Self { Self { emit, transient: None } }
    fn transient(transient: FormsTryWindowTransient, coalesce_key: String) -> Self {
        Self { emit: Emit { coalesce_key: Some(coalesce_key), ui_scope: UiDirtyScope::Full, ..Default::default() }, transient: Some(transient) }
    }
}

fn validate_target(window_id: &str, window_kind_id: &str, lease: &FormsTryWindowLease) -> Result<(), Fault> {
    if !lease.matches(window_id, window_kind_id, lease.window_generation, lease.document_generation) {
        return Err(fault("forms.try-value.window-stale", "the Forms Try command does not address the captured window lease"));
    }
    Ok(())
}

fn start(
    operation: &semio_framework_plugin::AppOperationContext,
    lease: &FormsTryWindowLease,
    transient: &FormsTryWindowTransient,
    work: TryValueWork,
) -> Result<TryWindowCommandOutput, Fault> {
    cancel_pending_generations(operation, lease);
    let key = job_key(operation, lease);
    let session = TryValueSession {
        key: key.clone(),
        base_revision: operation.canonical_base_revision_hex(),
        baseline_root_token: transient.try_values.root_token(),
        baseline_revision: transient.try_values.revision(),
        work,
    };
    if sessions().lock().expect("forms Try sessions lock").len() >= MAX_LIVE_TRY_VALUE_SESSIONS {
        return Err(fault("forms.try-value.busy", "the bounded Forms Try session pool is full"));
    }
    let next = continuation(&session);
    sessions().lock().expect("forms Try sessions lock").insert(key, session);
    Ok(TryWindowCommandOutput::emit(Emit { effects: vec![queue(&next)], ui_scope: UiDirtyScope::None, ..Default::default() }))
}

pub(crate) fn start_window(
    payload: &SetTryValue,
    operation: &semio_framework_plugin::AppOperationContext,
    transient: &FormsTryWindowTransient,
    lease: &FormsTryWindowLease,
) -> Result<TryWindowCommandOutput, Fault> {
    validate_target(&payload.window_id, &payload.window_kind_id, lease)?;
    if payload.key.len() > 512 { return Err(fault("forms.try-value.key-too-large", "the Forms Try-value key exceeds 512 UTF-8 bytes")); }
    if payload.value_json.is_none() && payload.option_value.is_none() { return Ok(TryWindowCommandOutput::emit(Emit::default())); }
    let input_count = payload.input_count.unwrap_or(1);
    if input_count > 1 && payload.input_id.is_none() { return Err(fault("forms.try-value.input-id-required", "multi-chunk Forms input requires an explicit input id")); }
    let chunk = payload.value_json.as_ref().map_or_else(|| Arc::from("false"), ChunkAddressableJson::owner);
    let input_id = payload.input_id.as_deref().unwrap_or(&payload.key);
    let Some(input) = stage_command_input(operation, lease, "setTryValue", input_id, payload.input_index.unwrap_or(0), input_count, chunk)? else {
        return Ok(TryWindowCommandOutput::emit(Emit::default()));
    };
    start(&input.operation, lease, transient, TryValueWork::Single { payload: payload.clone(), source: input.source })
}

pub(crate) fn start_bulk_window(
    source: Arc<str>,
    operation: &semio_framework_plugin::AppOperationContext,
    transient: &FormsTryWindowTransient,
    lease: &FormsTryWindowLease,
) -> Result<TryWindowCommandOutput, Fault> {
    start(operation, lease, transient, TryValueWork::Bulk { source })
}

fn materialize(values: &FormsTryValues, key: &str) -> Option<String> {
    values.iter_chunks().into_iter().find(|(candidate, _)| candidate == key).map(|(_, chunks)| chunks.iter().map(|chunk| chunk.as_ref()).collect())
}

fn apply_single(values: &FormsTryValues, payload: &SetTryValue, source: &str) -> Result<FormsTryValues, Fault> {
    if values.get_json(&payload.key).is_none() && values.len() >= MAX_TRY_VALUE_ENTRIES {
        return Err(fault("forms.try-value.too-many-entries", "Forms Try values are limited to 64 entries"));
    }
    let incoming = dsl::os_pack::json::parse(source).unwrap_or(Value::Null);
    let current = materialize(values, &payload.key).and_then(|raw| dsl::os_pack::json::parse(&raw).ok()).unwrap_or(Value::Null);
    let value = if let Some(index) = payload.vector_index.and_then(|index| usize::try_from(index).ok()) {
        let mut array = match current { Value::Array(array) => array, _ => Vec::new() };
        if index < array.len() { array[index] = incoming; } else if index == array.len() { array.push(incoming); }
        Value::Array(array)
    } else if let Some(option) = &payload.option_value {
        let mut array = match current { Value::Array(array) => array, _ => Vec::new() };
        if let Some(index) = array.iter().position(|value| value.as_str() == Some(option)) { array.remove(index); } else { array.push(Value::from(option.clone())); }
        Value::Array(array)
    } else if let Some(param_key) = &payload.param_key {
        let mut object = match current { Value::Object(object) => object, _ => Object::new() };
        object.insert(param_key.clone(), incoming);
        Value::Object(object)
    } else {
        incoming
    };
    Ok(with_raw(values, &payload.key, &dsl::os_pack::json::to_string(&value)))
}

fn with_raw(values: &FormsTryValues, key: &str, raw: &str) -> FormsTryValues {
    let chunks = split_chunks(raw);
    let content_id = try_value_content_id(&chunks);
    values.with_chunks(key, content_id, Arc::new(chunks))
}

fn split_chunks(raw: &str) -> Vec<Arc<str>> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < raw.len() {
        let mut end = start.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(raw.len());
        while end > start && !raw.is_char_boundary(end) { end -= 1; }
        if end == start { end += raw[start..].chars().next().map(char::len_utf8).unwrap_or(1); }
        chunks.push(Arc::from(&raw[start..end]));
        start = end;
    }
    if chunks.is_empty() { chunks.push(Arc::from("")); }
    chunks
}

fn apply_bulk(values: &FormsTryValues, source: &str) -> Result<FormsTryValues, Fault> {
    let Value::Object(entries) = dsl::os_pack::json::parse(source).map_err(|_| fault("forms.try-values.invalid", "Forms Try values must be a JSON object"))? else {
        return Err(fault("forms.try-values.invalid", "Forms Try values must be a JSON object"));
    };
    let result = entries.into_iter().try_fold(values.clone(), |values, (key, value)| {
        if key.len() > 512 { return Err(fault("forms.try-values.key-too-large", "a Forms Try-values key exceeds 512 UTF-8 bytes")); }
        if values.get_json(&key).is_none() && values.len() >= MAX_TRY_VALUE_ENTRIES { return Err(fault("forms.try-values.too-many-entries", "Forms Try values are limited to 64 entries")); }
        Ok(with_raw(&values, &key, &dsl::os_pack::json::to_string(&value)))
    });
    result
}

fn validate_continuation(payload: &SetTryValueStep, operation: &semio_framework_plugin::AppOperationContext, lease: &FormsTryWindowLease) -> Result<(), Fault> {
    if payload.app_id != operation.app_instance_id.to_string()
        || payload.document_id != operation.parent_document_id
        || payload.base_revision != operation.canonical_base_revision_hex()
        || !lease.matches(&payload.window_id, &payload.window_kind_id, payload.window_generation, payload.document_generation)
    {
        return Err(fault("forms.try-value.checkpoint-invalid", "the Forms continuation identity or window lease is invalid"));
    }
    Ok(())
}

pub(crate) fn advance_window(
    payload: &SetTryValueStep,
    operation: &semio_framework_plugin::AppOperationContext,
    transient: &FormsTryWindowTransient,
    lease: &FormsTryWindowLease,
) -> Result<TryWindowCommandOutput, Fault> {
    validate_continuation(payload, operation, lease)?;
    let key = payload_key(payload);
    let session = sessions().lock().expect("forms Try sessions lock").remove(&key).ok_or_else(|| fault("forms.try-value.checkpoint-stale", "the Forms continuation no longer owns a live Try window session"))?;
    if session.base_revision != payload.base_revision || session.baseline_root_token != transient.try_values.root_token() || session.baseline_revision != transient.try_values.revision() {
        return Err(fault("forms.try-value.checkpoint-stale", "the Forms continuation baseline changed"));
    }
    let try_values = match session.work {
        TryValueWork::Single { payload, source } => apply_single(&transient.try_values, &payload, &source)?,
        TryValueWork::Bulk { source } => apply_bulk(&transient.try_values, &source)?,
    };
    Ok(TryWindowCommandOutput::transient(FormsTryWindowTransient { try_values }, format!("formsTry:{}:{}", lease.window_id, payload.generation)))
}

pub fn handle(_payload: &SetTryValue, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::default())
}

pub fn handle_step(_payload: &SetTryValueStep, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::default())
}

fn fault(code: &'static str, message: &'static str) -> Fault { Fault::new(FaultOrigin::App, FaultCode::new(code), message) }

#[cfg(test)]
#[path = "🧪️tests/🔬️chunk-value-vectors/🦀️.rs"]
mod chunk_value_vectors;
