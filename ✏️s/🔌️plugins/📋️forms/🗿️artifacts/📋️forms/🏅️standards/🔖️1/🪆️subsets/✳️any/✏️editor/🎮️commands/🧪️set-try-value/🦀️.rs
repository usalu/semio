//! 🧪️ Forms try-value updates with bounded vector expansion.

use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{discard_staged_try_value, FormsConfig, FormsConfigMutation};
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, RequestId};
use serde::de::Deserializer;
use serde::ser::Serializer;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
pub const SET_TRY_VALUE_STEP_ACTION_ID: &str = "setTryValueStep";
pub(crate) const MAX_VECTOR_COMPONENTS_PER_STEP: usize = 64;
pub(crate) const MAX_TRY_VALUE_BYTES_PER_STEP: usize = 4_096;
const MAX_LIVE_TRY_VALUE_SESSIONS: usize = 64;
const MAX_COMMAND_INPUT_CHUNKS: u64 = 16_384;
const MAX_COMMAND_INPUT_IDLE_ACTIONS: u64 = 32_768;
//#endregion 🔖️Constants

//#region 🔖️ChunkAddressableInput
/// 🪢 Action-decoded immutable JSON storage. The action decoder pays the single wire allocation;
/// worker handlers retain this storage with an `Arc` clone and every continuation addresses <=4 KiB.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChunkAddressableJson(std::sync::Arc<str>);

impl ChunkAddressableJson {
    pub(crate) fn owner(&self) -> std::sync::Arc<str> {
        self.0.clone()
    }
}

impl std::ops::Deref for ChunkAddressableJson {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for ChunkAddressableJson {
    fn from(value: String) -> Self {
        Self(std::sync::Arc::from(value))
    }
}

impl From<&str> for ChunkAddressableJson {
    fn from(value: &str) -> Self {
        Self(std::sync::Arc::from(value))
    }
}

impl Serialize for ChunkAddressableJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self)
    }
}

impl<'de> Deserialize<'de> for ChunkAddressableJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct BoundedChunkVisitor;
        impl<'de> serde::de::Visitor<'de> for BoundedChunkVisitor {
            type Value = ChunkAddressableJson;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a JSON chunk of at most 4,096 UTF-8 bytes")
            }

            fn visit_borrowed_str<E: serde::de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
                self.visit_str(value)
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                if value.len() > MAX_TRY_VALUE_BYTES_PER_STEP {
                    return Err(E::custom("Forms command JSON chunks are limited to 4,096 UTF-8 bytes"));
                }
                Ok(value.into())
            }

            fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
                if value.len() > MAX_TRY_VALUE_BYTES_PER_STEP {
                    return Err(E::custom("Forms command JSON chunks are limited to 4,096 UTF-8 bytes"));
                }
                Ok(value.into())
            }
        }
        deserializer.deserialize_str(BoundedChunkVisitor)
    }
}

impl dsl::DslField for ChunkAddressableJson {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }

    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.to_string())
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Text(value) = value else { return Err(format!("expected Text, found {value:?}")) };
        if value.len() > MAX_TRY_VALUE_BYTES_PER_STEP {
            return Err("Forms command JSON chunks are limited to 4,096 UTF-8 bytes".into());
        }
        Ok(value.as_str().into())
    }
}
//#endregion 🔖️ChunkAddressableInput

//#region 🔖️Session
#[derive(Debug)]
struct TryValueSession {
    app_id: String,
    document_id: String,
    operation_id: String,
    base_revision: String,
    baseline_content_id: Option<std::sync::Arc<str>>,
    progress_base: u64,
    key: String,
    staging_id: String,
    staged_cursor: usize,
    staged_chunks: u64,
    digest_high: u64,
    digest_low: u64,
    digest_third: u64,
    digest_fourth: u64,
    digest_len: u64,
    source: ChunkedSource,
    source_content_id: Option<std::sync::Arc<str>>,
    source_chunk_cursor: u64,
    rewrite: Option<TryValueRewrite>,
    prepared_value: Option<std::sync::Arc<ChunkedSource>>,
    prepared_cursor: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FormsJobKey {
    app_id: String,
    document_id: String,
    operation_id: String,
    generation: u64,
}

static TRY_VALUE_SESSIONS: OnceLock<Mutex<BTreeMap<FormsJobKey, TryValueSession>>> = OnceLock::new();
static ACTIVE_TRY_VALUE_GENERATIONS: OnceLock<Mutex<BTreeMap<(String, String, String), u64>>> = OnceLock::new();
static NEXT_TRY_VALUE_REQUEST: AtomicU64 = AtomicU64::new(20_000);

fn sessions() -> &'static Mutex<BTreeMap<FormsJobKey, TryValueSession>> {
    TRY_VALUE_SESSIONS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn active_generations() -> &'static Mutex<BTreeMap<(String, String, String), u64>> {
    ACTIVE_TRY_VALUE_GENERATIONS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub(crate) fn cancel_pending_generations(operation: &semio_framework_plugin::AppOperationContext) -> Vec<FormsConfigMutation> {
    cancel_command_inputs(operation.app_instance_id, &operation.parent_document_id);
    let mut mutations = Vec::with_capacity(2);
    let scopes: Vec<_> = active_generations().lock().expect("forms active generations lock").keys().filter(|(app, document, _)| app == &operation.app_instance_id.to_string() && document == &operation.parent_document_id).cloned().collect();
    for scope in scopes {
        let generation = active_generations().lock().expect("forms active generations lock").remove(&scope);
        let Some(generation) = generation else { continue };
        let key = FormsJobKey { app_id: scope.0, document_id: scope.1, operation_id: scope.2, generation };
        let Some(session) = sessions().lock().expect("forms try-value sessions lock").remove(&key) else { continue };
        let staging_id = session.staging_id;
        discard_staged_try_value(&staging_id);
        mutations.push(FormsConfigMutation::DiscardTryValueStaging { staging_id });
    }
    mutations.extend(crate::editor::forms::commands::set_try_values::cancel_pending_bulk(operation.app_instance_id, &operation.parent_document_id));
    mutations
}

fn job_key(payload: &SetTryValueStep) -> FormsJobKey {
    FormsJobKey { app_id: payload.app_id.clone(), document_id: payload.document_id.clone(), operation_id: payload.operation_id.clone(), generation: payload.generation }
}

fn take_session(payload: &SetTryValueStep) -> Option<TryValueSession> {
    sessions().lock().expect("forms try-value sessions lock").remove(&job_key(payload))
}

fn put_session(key: FormsJobKey, session: TryValueSession) -> Result<(), TryValueSession> {
    let mut live = sessions().lock().expect("forms try-value sessions lock");
    if !live.contains_key(&key) && live.len() >= MAX_LIVE_TRY_VALUE_SESSIONS {
        return Err(session);
    }
    live.insert(key, session);
    Ok(())
}

fn session_job_key(session: &TryValueSession, generation: u64) -> FormsJobKey {
    FormsJobKey { app_id: session.app_id.clone(), document_id: session.document_id.clone(), operation_id: session.operation_id.clone(), generation }
}

fn update_digest(session: &mut TryValueSession, bytes: &[u8]) {
    for byte in bytes {
        session.digest_len = session.digest_len.wrapping_add(1);
        session.digest_high ^= u64::from(*byte);
        session.digest_high = session.digest_high.wrapping_mul(0x00000100000001b3);
        session.digest_low ^= session.digest_high.rotate_left(17) ^ session.digest_len;
        session.digest_low = session.digest_low.wrapping_mul(0x9e3779b185ebca87);
        session.digest_third ^= session.digest_low.rotate_left(29) ^ u64::from(*byte);
        session.digest_third = session.digest_third.wrapping_mul(0xc2b2ae3d27d4eb4f);
        session.digest_fourth ^= session.digest_third.rotate_left(41) ^ session.digest_len.rotate_left(7);
        session.digest_fourth = session.digest_fourth.wrapping_mul(0x165667b19e3779f9);
    }
}

fn content_id(session: &TryValueSession) -> String {
    format!("try-{:016x}{:016x}{:016x}{:016x}-{:016x}", session.digest_high, session.digest_low, session.digest_third, session.digest_fourth, session.digest_len)
}
//#endregion 🔖️Session

//#region 🔖️Values
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RewritePhase {
    Locate,
    Array,
    Value,
    Prefix,
    Replacement,
    Suffix,
    Done,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReplacementMode {
    ExistingElement,
    ExistingArrayInsertion,
    FullArray,
    MissingField,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RewriteStep {
    complete: bool,
    bytes: usize,
    components: usize,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ChunkedSource {
    chunks: BTreeMap<usize, std::sync::Arc<str>>,
    len: usize,
}

impl ChunkedSource {
    pub(crate) fn push(&mut self, chunk: std::sync::Arc<str>) {
        if chunk.is_empty() {
            return;
        }
        let chunk_len = chunk.len();
        self.chunks.insert(self.len, chunk);
        self.len = self.len.saturating_add(chunk_len);
    }

    fn from_text(text: String) -> Self {
        let mut source = Self::default();
        source.push(std::sync::Arc::from(text));
        source
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn byte(&self, index: usize) -> Option<u8> {
        let (start, chunk) = self.chunks.range(..=index).next_back()?;
        chunk.as_bytes().get(index - start).copied()
    }

    pub(crate) fn append_range(&self, cursor: &mut usize, end: usize, output: &mut Vec<std::sync::Arc<str>>) -> usize {
        let start = *cursor;
        let limit = start.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(end);
        while *cursor < limit {
            let (chunk_start, chunk) = self.chunks.range(..=*cursor).next_back().expect("bounded source chunk");
            let local_start = *cursor - *chunk_start;
            let mut local_end = (limit - *chunk_start).min(chunk.len());
            while local_end > local_start && !chunk.is_char_boundary(local_end) {
                local_end -= 1;
            }
            if local_end == local_start {
                local_end += chunk[local_start..].chars().next().map(char::len_utf8).unwrap_or(1);
            }
            output.push(std::sync::Arc::from(&chunk[local_start..local_end]));
            *cursor = *chunk_start + local_end;
        }
        cursor.saturating_sub(start)
    }

    pub(crate) fn bounded_range(&self, start: usize, end: usize) -> Option<String> {
        if end < start || end.saturating_sub(start) > MAX_TRY_VALUE_BYTES_PER_STEP {
            return None;
        }
        let mut cursor = start;
        let mut parts = Vec::new();
        self.append_range(&mut cursor, end, &mut parts);
        (cursor == end).then(|| {
            parts.iter().fold(String::new(), |mut text, part| {
                text.push_str(part);
                text
            })
        })
    }

    #[cfg(test)]
    fn materialize(&self) -> String {
        self.chunks.values().fold(String::new(), |mut output, chunk| {
            output.push_str(chunk);
            output
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FormsInputKey {
    app_instance_id: u32,
    document_id: String,
    operation_id: u64,
    generation: u64,
    base_revision: [u8; 32],
    semantic_id: String,
    input_id: String,
}

pub(crate) struct StagedCommandInput {
    pub(crate) source: ChunkedSource,
    pub(crate) operation: semio_framework_plugin::AppOperationContext,
}

#[derive(Default)]
struct FormsInputBlob {
    source: ChunkedSource,
    next_index: u64,
    chunk_count: u64,
    last_touch: u64,
}

#[derive(Default)]
pub(crate) struct FormsInputRegistry {
    blobs: BTreeMap<FormsInputKey, FormsInputBlob>,
    expiry: BTreeMap<u64, FormsInputKey>,
    active: BTreeMap<(u32, String, String), FormsInputKey>,
    tick: u64,
}

static FORMS_INPUT_REGISTRY: OnceLock<Mutex<FormsInputRegistry>> = OnceLock::new();

pub(crate) fn input_registry() -> &'static Mutex<FormsInputRegistry> {
    FORMS_INPUT_REGISTRY.get_or_init(|| Mutex::new(FormsInputRegistry::default()))
}

fn cancel_command_inputs(app_instance_id: u32, document_id: &str) {
    let mut registry = input_registry().lock().expect("forms input registry lock");
    for operation_id in ["setTryValue", "setTryValues"] {
        let scope = (app_instance_id, document_id.to_string(), operation_id.to_string());
        let Some(key) = registry.active.remove(&scope) else { continue };
        if let Some(blob) = registry.blobs.remove(&key) {
            registry.expiry.remove(&blob.last_touch);
        }
    }
}

fn expire_one_command_input(registry: &mut FormsInputRegistry) {
    let Some((&touch, key)) = registry.expiry.first_key_value() else { return };
    if registry.tick.saturating_sub(touch) <= MAX_COMMAND_INPUT_IDLE_ACTIONS {
        return;
    }
    let key = key.clone();
    registry.expiry.remove(&touch);
    if registry.blobs.get(&key).is_some_and(|blob| blob.last_touch == touch) {
        registry.blobs.remove(&key);
        registry.active.remove(&(key.app_instance_id, key.document_id.clone(), key.semantic_id.clone()));
    }
}

pub(crate) fn stage_command_input(operation: &semio_framework_plugin::AppOperationContext, semantic_id: &str, input_id: &str, index: u64, chunk_count: u64, chunk: std::sync::Arc<str>) -> Result<Option<StagedCommandInput>, Fault> {
    if input_id.len() > 256 || chunk.len() > MAX_TRY_VALUE_BYTES_PER_STEP || chunk_count == 0 || chunk_count > MAX_COMMAND_INPUT_CHUNKS || index >= chunk_count {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.input-invalid"), "the Forms input chunk address or size is invalid"));
    }
    let incoming_key = FormsInputKey {
        app_instance_id: operation.app_instance_id,
        document_id: operation.parent_document_id.clone(),
        operation_id: operation.operation_id,
        generation: operation.generation,
        base_revision: operation.canonical_base_revision,
        semantic_id: semantic_id.into(),
        input_id: input_id.into(),
    };
    let scope = (incoming_key.app_instance_id, incoming_key.document_id.clone(), incoming_key.semantic_id.clone());
    let mut registry = input_registry().lock().expect("forms input registry lock");
    registry.tick = registry.tick.wrapping_add(1);
    expire_one_command_input(&mut registry);
    let key = match registry.active.get(&scope).cloned() {
        Some(previous) if previous.input_id == incoming_key.input_id && previous.generation == incoming_key.generation && previous.base_revision == incoming_key.base_revision => previous,
        Some(previous) => {
            if let Some(blob) = registry.blobs.remove(&previous) {
                registry.expiry.remove(&blob.last_touch);
            }
            registry.active.remove(&scope);
            incoming_key
        }
        None => incoming_key,
    };
    if !registry.blobs.contains_key(&key) && registry.blobs.len() >= MAX_LIVE_TRY_VALUE_SESSIONS {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.busy"), "the bounded Forms input staging pool is full"));
    }
    if let Some(previous_touch) = registry.blobs.get(&key).map(|blob| blob.last_touch) {
        registry.expiry.remove(&previous_touch);
    }
    let touch = registry.tick;
    let blob = registry.blobs.entry(key.clone()).or_insert_with(|| FormsInputBlob { chunk_count, ..Default::default() });
    if blob.chunk_count != chunk_count || blob.next_index != index {
        registry.blobs.remove(&key);
        registry.active.remove(&scope);
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.input-order"), "Forms input chunks must be contiguous and generation-stable"));
    }
    blob.source.push(chunk);
    blob.next_index += 1;
    blob.last_touch = touch;
    if blob.next_index != blob.chunk_count {
        registry.expiry.insert(touch, key.clone());
        registry.active.insert(scope, key);
        return Ok(None);
    }
    registry.active.remove(&scope);
    Ok(registry.blobs.remove(&key).map(|blob| StagedCommandInput {
        source: blob.source,
        operation: semio_framework_plugin::AppOperationContext { app_instance_id: key.app_instance_id, parent_document_id: key.document_id, operation_id: key.operation_id, generation: key.generation, canonical_base_revision: key.base_revision },
    }))
}

#[derive(Debug)]
struct VectorRewrite {
    key_token: String,
    raw: std::sync::Arc<ChunkedSource>,
    final_raw: std::sync::Arc<ChunkedSource>,
    requested_target_index: u64,
    target_index: u64,
    phase: RewritePhase,
    cursor: usize,
    progress: u64,
    root_started: bool,
    depth: usize,
    in_string: bool,
    escaped: bool,
    string_start: usize,
    expect_key: bool,
    key_matches: bool,
    await_value: bool,
    object_has_members: bool,
    element_start: Option<usize>,
    element_last_non_whitespace: usize,
    element_depth: usize,
    element_index: u64,
    value_start: usize,
    value_depth: usize,
    value_container: bool,
    value_root_string: bool,
    replace_start: usize,
    replace_end: usize,
    mode: ReplacementMode,
    output: Vec<std::sync::Arc<str>>,
    output_len: usize,
    copy_cursor: usize,
    header: String,
    header_cursor: usize,
    write_index: u64,
    raw_cursor: usize,
    footer: &'static str,
}

impl VectorRewrite {
    fn new(raw: std::sync::Arc<ChunkedSource>, target_index: u64) -> Self {
        Self::from_serialized(String::new(), raw, target_index)
    }

    fn from_serialized(key_token: String, raw: std::sync::Arc<ChunkedSource>, target_index: u64) -> Self {
        Self {
            key_token,
            raw: raw.clone(),
            final_raw: raw,
            requested_target_index: target_index,
            target_index,
            phase: RewritePhase::Locate,
            cursor: 0,
            progress: 0,
            root_started: false,
            depth: 0,
            in_string: false,
            escaped: false,
            string_start: 0,
            expect_key: true,
            key_matches: false,
            await_value: false,
            object_has_members: false,
            element_start: None,
            element_last_non_whitespace: 0,
            element_depth: 0,
            element_index: 0,
            value_start: 0,
            value_depth: 0,
            value_container: false,
            value_root_string: false,
            replace_start: 0,
            replace_end: 0,
            mode: ReplacementMode::FullArray,
            output: Vec::new(),
            output_len: 0,
            copy_cursor: 0,
            header: String::new(),
            header_cursor: 0,
            write_index: 0,
            raw_cursor: 0,
            footer: "",
        }
    }

    fn checkpoint(&self) -> u64 {
        self.progress
    }

    fn is_final_preview(&self) -> bool {
        self.target_index == self.requested_target_index
    }

    fn restart(&self) -> Self {
        Self::from_serialized(self.key_token.clone(), self.final_raw.clone(), self.requested_target_index)
    }

    fn advance(&mut self, source: &ChunkedSource) -> RewriteStep {
        let mut step = match self.phase {
            RewritePhase::Locate => self.scan_locate(source),
            RewritePhase::Array => self.scan_array(source),
            RewritePhase::Value => self.scan_value(source),
            RewritePhase::Prefix => self.copy_prefix(source),
            RewritePhase::Replacement => self.write_replacement(),
            RewritePhase::Suffix => self.copy_suffix(source),
            RewritePhase::Done | RewritePhase::Failed => RewriteStep { complete: true, ..RewriteStep::default() },
        };
        self.progress = self.progress.saturating_add((step.bytes + step.components).max(1) as u64);
        step.complete = matches!(self.phase, RewritePhase::Done | RewritePhase::Failed);
        step
    }

    fn scan_locate(&mut self, source: &ChunkedSource) -> RewriteStep {
        let limit = self.cursor.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(source.len);
        let start = self.cursor;
        while self.cursor < limit {
            let byte = source.byte(self.cursor).expect("bounded source byte");
            if byte.is_ascii_whitespace() {
                self.cursor += 1;
                continue;
            }
            if byte == b'[' {
                self.phase = RewritePhase::Array;
                self.cursor += 1;
            } else {
                self.begin_build(source.len, 0, source.len, 0, ReplacementMode::FullArray, false);
            }
            break;
        }
        if self.cursor >= source.len && matches!(self.phase, RewritePhase::Locate) {
            self.begin_build(source.len, 0, source.len, 0, ReplacementMode::FullArray, false);
        }
        RewriteStep { bytes: self.cursor.saturating_sub(start), ..RewriteStep::default() }
    }

    fn scan_array(&mut self, source: &ChunkedSource) -> RewriteStep {
        let limit = self.cursor.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(source.len);
        let start = self.cursor;
        while self.cursor < limit {
            let byte = source.byte(self.cursor).expect("bounded source byte");
            if self.in_string {
                if self.escaped {
                    self.escaped = false;
                } else if byte == b'\\' {
                    self.escaped = true;
                } else if byte == b'"' {
                    self.in_string = false;
                }
                self.element_last_non_whitespace = self.cursor;
                self.cursor += 1;
                continue;
            }
            if self.element_start.is_none() {
                if byte.is_ascii_whitespace() {
                    self.cursor += 1;
                    continue;
                }
                if byte == b']' {
                    self.begin_build(source.len, self.cursor, self.cursor, self.element_index, ReplacementMode::ExistingArrayInsertion, false);
                    break;
                }
                self.element_start = Some(self.cursor);
                self.element_last_non_whitespace = self.cursor;
                self.element_depth = 0;
            }
            if !byte.is_ascii_whitespace() {
                self.element_last_non_whitespace = self.cursor;
            }
            match byte {
                b'"' => self.in_string = true,
                b'{' | b'[' => self.element_depth += 1,
                b'}' | b']' if self.element_depth > 0 => self.element_depth -= 1,
                b',' if self.element_depth == 0 => {
                    let element_start = self.element_start.take().expect("array element start");
                    if self.element_index == self.target_index {
                        self.begin_build(source.len, element_start, self.element_last_non_whitespace + 1, self.element_index + 1, ReplacementMode::ExistingElement, false);
                        break;
                    }
                    self.element_index = self.element_index.saturating_add(1);
                }
                b']' if self.element_depth == 0 => {
                    let element_start = self.element_start.take().expect("array element start");
                    let vector_len = self.element_index.saturating_add(1);
                    if self.element_index == self.target_index {
                        self.begin_build(source.len, element_start, self.element_last_non_whitespace + 1, vector_len, ReplacementMode::ExistingElement, false);
                    } else {
                        self.begin_build(source.len, self.cursor, self.cursor, vector_len, ReplacementMode::ExistingArrayInsertion, false);
                    }
                    break;
                }
                _ => {}
            }
            self.cursor += 1;
        }
        if self.cursor >= source.len && matches!(self.phase, RewritePhase::Array) {
            self.begin_build(source.len, 0, source.len, 0, ReplacementMode::MissingField, false);
        }
        RewriteStep { bytes: self.cursor.saturating_sub(start), ..RewriteStep::default() }
    }

    fn scan_value(&mut self, source: &ChunkedSource) -> RewriteStep {
        let limit = self.cursor.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(source.len);
        let start = self.cursor;
        while self.cursor < limit {
            let byte = source.byte(self.cursor).expect("bounded source byte");
            if self.in_string {
                if self.escaped {
                    self.escaped = false;
                } else if byte == b'\\' {
                    self.escaped = true;
                } else if byte == b'"' {
                    self.in_string = false;
                    if self.value_root_string {
                        self.cursor += 1;
                        self.begin_build(source.len, self.value_start, self.cursor, 0, ReplacementMode::FullArray, false);
                        break;
                    }
                }
                self.cursor += 1;
                continue;
            }
            match byte {
                b'"' if self.cursor == self.value_start => {
                    self.in_string = true;
                    self.value_root_string = true;
                }
                b'{' | b'[' => {
                    self.value_container = true;
                    self.value_depth += 1;
                }
                b'}' | b']' if self.value_depth > 0 => {
                    self.value_depth -= 1;
                    if self.value_depth == 0 && self.value_container {
                        self.cursor += 1;
                        self.begin_build(source.len, self.value_start, self.cursor, 0, ReplacementMode::FullArray, false);
                        break;
                    }
                }
                b',' | b'}' if self.value_depth == 0 => {
                    self.begin_build(source.len, self.value_start, self.cursor, 0, ReplacementMode::FullArray, false);
                    break;
                }
                byte if byte.is_ascii_whitespace() && self.value_depth == 0 => {
                    self.begin_build(source.len, self.value_start, self.cursor, 0, ReplacementMode::FullArray, false);
                    break;
                }
                _ => {}
            }
            self.cursor += 1;
        }
        if self.cursor >= source.len && matches!(self.phase, RewritePhase::Value) {
            self.begin_build(source.len, self.value_start, source.len, 0, ReplacementMode::FullArray, false);
        }
        RewriteStep { bytes: self.cursor.saturating_sub(start), ..RewriteStep::default() }
    }

    fn begin_build(&mut self, source_len: usize, replace_start: usize, replace_end: usize, vector_len: u64, mode: ReplacementMode, object_has_members: bool) {
        if !matches!(mode, ReplacementMode::ExistingElement) {
            self.target_index = self.requested_target_index.min(vector_len.saturating_add(MAX_VECTOR_COMPONENTS_PER_STEP as u64 - 1));
            self.raw = if self.target_index == self.requested_target_index { self.final_raw.clone() } else { std::sync::Arc::new(ChunkedSource::from_text("0".into())) };
        }
        self.replace_start = replace_start;
        self.replace_end = replace_end;
        self.mode = mode;
        self.header = match mode {
            ReplacementMode::ExistingElement => String::new(),
            ReplacementMode::ExistingArrayInsertion => {
                if vector_len == 0 {
                    String::new()
                } else {
                    ",".into()
                }
            }
            ReplacementMode::FullArray => "[".into(),
            ReplacementMode::MissingField => format!("{}{}:[", if object_has_members { "," } else { "" }, self.key_token),
        };
        self.footer = if matches!(mode, ReplacementMode::FullArray | ReplacementMode::MissingField) { "]" } else { "" };
        self.write_index = if matches!(mode, ReplacementMode::ExistingArrayInsertion) { vector_len } else { self.target_index };
        if !matches!(mode, ReplacementMode::ExistingElement) {
            self.write_index = if matches!(mode, ReplacementMode::ExistingArrayInsertion) { vector_len } else { 0 };
        }
        let gap = self.target_index.saturating_sub(self.write_index);
        let Some(gap) = usize::try_from(gap).ok() else {
            self.phase = RewritePhase::Failed;
            return;
        };
        let replacement_len = self.header.len().checked_add(gap.saturating_mul(2)).and_then(|len| len.checked_add(self.raw.len)).and_then(|len| len.checked_add(self.footer.len()));
        let output_len = source_len.checked_sub(replace_end.saturating_sub(replace_start)).and_then(|len| replacement_len.and_then(|replacement| len.checked_add(replacement)));
        let Some(_output_len) = output_len else {
            self.phase = RewritePhase::Failed;
            return;
        };
        self.output.clear();
        self.output_len = 0;
        self.copy_cursor = 0;
        self.header_cursor = 0;
        self.raw_cursor = 0;
        self.phase = RewritePhase::Prefix;
    }

    fn copy_prefix(&mut self, source: &ChunkedSource) -> RewriteStep {
        let copied = source.append_range(&mut self.copy_cursor, self.replace_start, &mut self.output);
        self.output_len = self.output_len.saturating_add(copied);
        if self.copy_cursor == self.replace_start {
            self.phase = RewritePhase::Replacement;
        }
        RewriteStep { bytes: copied, ..RewriteStep::default() }
    }

    fn write_replacement(&mut self) -> RewriteStep {
        if self.header_cursor < self.header.len() {
            let copied = copy_text_chunk(&self.header, &mut self.header_cursor, self.header.len(), &mut self.output);
            self.output_len = self.output_len.saturating_add(copied);
            return RewriteStep { bytes: copied, ..RewriteStep::default() };
        }
        if !matches!(self.mode, ReplacementMode::ExistingElement) && self.write_index < self.target_index {
            let count = usize::try_from(self.target_index - self.write_index).unwrap_or(MAX_VECTOR_COMPONENTS_PER_STEP).min(MAX_VECTOR_COMPONENTS_PER_STEP);
            for _ in 0..count {
                self.output.push(std::sync::Arc::from("0,"));
            }
            self.output_len = self.output_len.saturating_add(count * 2);
            self.write_index += count as u64;
            return RewriteStep { bytes: count * 2, components: count, complete: false };
        }
        if self.raw_cursor < self.raw.len {
            let copied = self.raw.append_range(&mut self.raw_cursor, self.raw.len, &mut self.output);
            self.output_len = self.output_len.saturating_add(copied);
            return RewriteStep { bytes: copied, ..RewriteStep::default() };
        }
        if !self.footer.is_empty() {
            self.output.push(std::sync::Arc::from(self.footer));
            self.output_len = self.output_len.saturating_add(self.footer.len());
        }
        self.copy_cursor = self.replace_end;
        self.phase = RewritePhase::Suffix;
        RewriteStep { bytes: self.footer.len(), ..RewriteStep::default() }
    }

    fn copy_suffix(&mut self, source: &ChunkedSource) -> RewriteStep {
        let copied = source.append_range(&mut self.copy_cursor, source.len, &mut self.output);
        self.output_len = self.output_len.saturating_add(copied);
        if self.copy_cursor == source.len {
            self.phase = RewritePhase::Done;
        }
        RewriteStep { bytes: copied, complete: self.phase == RewritePhase::Done, ..RewriteStep::default() }
    }

    fn take_output(&mut self) -> Option<ChunkedSource> {
        (self.phase == RewritePhase::Done).then(|| {
            let mut source = ChunkedSource::default();
            for chunk in std::mem::take(&mut self.output) {
                source.push(chunk);
            }
            self.output_len = 0;
            source
        })
    }
}

#[derive(Clone, Debug)]
enum ContainerEdit {
    Option { token: String, value: String, pressed: bool },
    Object { key_token: String, key: String, raw: std::sync::Arc<ChunkedSource> },
}

#[derive(Clone, Debug)]
struct ContainerRewrite {
    edit: ContainerEdit,
    cursor: usize,
    started: bool,
    in_string: bool,
    escaped: bool,
    depth: usize,
    member_start: Option<usize>,
    member_key_end: Option<usize>,
    member_value_start: Option<usize>,
    previous_separator: Option<usize>,
    found_start: Option<usize>,
    found_end: Option<usize>,
    close: Option<usize>,
    phase: RewritePhase,
    output: Vec<std::sync::Arc<str>>,
    copy_cursor: usize,
    replacement: String,
    replacement_cursor: usize,
    replacement_source: Option<std::sync::Arc<ChunkedSource>>,
    replacement_source_cursor: usize,
    replacement_suffix: String,
    replacement_suffix_cursor: usize,
    replace_start: usize,
    replace_end: usize,
    progress: u64,
    pressed_source: Option<std::sync::Arc<ChunkedSource>>,
    pressed_cursor: usize,
    pressed_token: String,
    pressed_possible: bool,
    changed: bool,
}

impl ContainerRewrite {
    fn option(option_value: &str, pressed_json: std::sync::Arc<ChunkedSource>) -> Option<Self> {
        if option_value.len() > 512 {
            return None;
        }
        let token = serde_json::to_string(option_value).ok()?;
        (token.len() <= MAX_TRY_VALUE_BYTES_PER_STEP).then(|| {
            let mut rewrite = Self::new(ContainerEdit::Option { token, value: option_value.into(), pressed: false });
            rewrite.pressed_source = Some(pressed_json);
            rewrite
        })
    }

    fn object(param_key: &str, raw: std::sync::Arc<ChunkedSource>) -> Option<Self> {
        if param_key.len() > 512 {
            return None;
        }
        let key_token = serde_json::to_string(param_key).ok()?;
        (key_token.len() <= MAX_TRY_VALUE_BYTES_PER_STEP).then(|| Self::new(ContainerEdit::Object { key_token, key: param_key.into(), raw }))
    }

    fn new(edit: ContainerEdit) -> Self {
        Self {
            edit,
            cursor: 0,
            started: false,
            in_string: false,
            escaped: false,
            depth: 0,
            member_start: None,
            member_key_end: None,
            member_value_start: None,
            previous_separator: None,
            found_start: None,
            found_end: None,
            close: None,
            phase: RewritePhase::Locate,
            output: Vec::new(),
            copy_cursor: 0,
            replacement: String::new(),
            replacement_cursor: 0,
            replacement_source: None,
            replacement_source_cursor: 0,
            replacement_suffix: String::new(),
            replacement_suffix_cursor: 0,
            replace_start: 0,
            replace_end: 0,
            progress: 0,
            pressed_source: None,
            pressed_cursor: 0,
            pressed_token: String::new(),
            pressed_possible: true,
            changed: false,
        }
    }

    fn checkpoint(&self) -> u64 {
        self.progress
    }

    fn advance(&mut self, source: &ChunkedSource) -> RewriteStep {
        if self.pressed_source.is_some() {
            let step = self.scan_pressed();
            self.progress = self.progress.saturating_add(step.bytes.max(1) as u64);
            return step;
        }
        let mut step = match self.phase {
            RewritePhase::Locate | RewritePhase::Array | RewritePhase::Value => self.scan(source),
            RewritePhase::Prefix => self.copy_prefix(source),
            RewritePhase::Replacement => self.write_replacement(),
            RewritePhase::Suffix => self.copy_suffix(source),
            RewritePhase::Done | RewritePhase::Failed => RewriteStep { complete: true, ..Default::default() },
        };
        self.progress = self.progress.saturating_add(step.bytes.max(1) as u64);
        step.complete = matches!(self.phase, RewritePhase::Done | RewritePhase::Failed);
        step
    }

    fn scan_pressed(&mut self) -> RewriteStep {
        let raw = self.pressed_source.as_ref().expect("pressed source");
        let start = self.pressed_cursor;
        let limit = start.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(raw.len);
        for cursor in start..limit {
            let byte = raw.byte(cursor).expect("pressed source byte");
            if byte.is_ascii_whitespace() {
                continue;
            }
            if self.pressed_token.len() < 5 {
                self.pressed_token.push(char::from(byte));
            } else {
                self.pressed_possible = false;
            }
        }
        self.pressed_cursor = limit;
        if limit == raw.len {
            if let ContainerEdit::Option { pressed, .. } = &mut self.edit {
                *pressed = self.pressed_possible && self.pressed_token == "true";
            }
            self.pressed_source = None;
        }
        RewriteStep { bytes: limit - start, ..Default::default() }
    }

    fn scan(&mut self, source: &ChunkedSource) -> RewriteStep {
        let start = self.cursor;
        let limit = start.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(source.len);
        let expected_open = if matches!(self.edit, ContainerEdit::Option { .. }) { b'[' } else { b'{' };
        let expected_close = if expected_open == b'[' { b']' } else { b'}' };
        while self.cursor < limit {
            let byte = source.byte(self.cursor).expect("bounded container byte");
            if !self.started {
                if byte.is_ascii_whitespace() {
                    self.cursor += 1;
                    continue;
                }
                if byte != expected_open {
                    self.prepare_fallback(source.len);
                    break;
                }
                self.started = true;
                self.cursor += 1;
                continue;
            }
            if self.in_string {
                if self.escaped {
                    self.escaped = false;
                } else if byte == b'\\' {
                    self.escaped = true;
                } else if byte == b'"' {
                    self.in_string = false;
                    if matches!(self.edit, ContainerEdit::Object { .. }) && self.depth == 0 && self.member_key_end.is_none() {
                        self.member_key_end = Some(self.cursor + 1);
                    }
                }
                self.cursor += 1;
                continue;
            }
            if self.member_start.is_none() && !byte.is_ascii_whitespace() && byte != expected_close {
                self.member_start = Some(self.cursor);
            }
            match byte {
                b'"' => self.in_string = true,
                b'{' | b'[' => self.depth += 1,
                b'}' | b']' if self.depth > 0 => self.depth -= 1,
                b':' if self.depth == 0 && matches!(self.edit, ContainerEdit::Object { .. }) && self.member_value_start.is_none() => {
                    self.member_value_start = Some(self.cursor + 1);
                }
                b',' if self.depth == 0 => {
                    self.finish_member(source, self.cursor, false);
                    self.previous_separator = Some(self.cursor);
                    self.member_start = None;
                    self.member_key_end = None;
                    self.member_value_start = None;
                }
                close if close == expected_close && self.depth == 0 => {
                    self.finish_member(source, self.cursor, true);
                    self.close = Some(self.cursor);
                    self.prepare_patch(source.len);
                    break;
                }
                _ => {}
            }
            self.cursor += 1;
        }
        if self.cursor >= source.len && !matches!(self.phase, RewritePhase::Prefix | RewritePhase::Done) {
            self.prepare_fallback(source.len);
        }
        RewriteStep { bytes: self.cursor.saturating_sub(start), ..Default::default() }
    }

    fn finish_member(&mut self, source: &ChunkedSource, end: usize, last: bool) {
        let Some(start) = self.member_start else { return };
        match &self.edit {
            ContainerEdit::Option { value, .. } => {
                let candidate = source.bounded_range(start, end).and_then(|raw| serde_json::from_str::<String>(raw.trim()).ok());
                if candidate.as_deref() == Some(value) {
                    self.found_start = Some(if last { self.previous_separator.unwrap_or(start) } else { start });
                    self.found_end = Some(if last { end } else { end + 1 });
                }
            }
            ContainerEdit::Object { key, .. } => {
                let Some(key_end) = self.member_key_end else { return };
                let candidate = source.bounded_range(start, key_end).and_then(|raw| serde_json::from_str::<String>(raw.trim()).ok());
                if candidate.as_deref() == Some(key) {
                    self.found_start = self.member_value_start.map(|cursor| {
                        let mut cursor = cursor;
                        while cursor < end && source.byte(cursor).is_some_and(|byte| byte.is_ascii_whitespace()) {
                            cursor += 1;
                        }
                        cursor
                    });
                    self.found_end = Some(end);
                }
            }
        }
    }

    fn prepare_fallback(&mut self, source_len: usize) {
        self.replace_start = 0;
        self.replace_end = source_len;
        self.replacement = match &self.edit {
            ContainerEdit::Option { token, pressed, .. } => {
                if *pressed {
                    format!("[{token}]")
                } else {
                    "[]".into()
                }
            }
            ContainerEdit::Object { key_token, raw, .. } => {
                self.replacement_source = Some(raw.clone());
                self.replacement_suffix = "}".into();
                format!("{{{key_token}:")
            }
        };
        self.phase = RewritePhase::Prefix;
    }

    fn prepare_patch(&mut self, source_len: usize) {
        let close = self.close.unwrap_or(source_len);
        match &self.edit {
            ContainerEdit::Option { token, pressed, .. } if *pressed && self.found_start.is_none() => {
                self.replace_start = close;
                self.replace_end = close;
                self.replacement = format!("{}{token}", if self.previous_separator.is_some() || self.member_start.is_some() { "," } else { "" });
            }
            ContainerEdit::Option { pressed: false, .. } if self.found_start.is_some() => {
                self.replace_start = self.found_start.unwrap_or(close);
                self.replace_end = self.found_end.unwrap_or(close);
                self.changed = true;
            }
            ContainerEdit::Object { key_token, raw, .. } if self.found_start.is_none() => {
                self.replace_start = close;
                self.replace_end = close;
                self.replacement = format!("{}{key_token}:", if self.previous_separator.is_some() || self.member_start.is_some() { "," } else { "" });
                self.replacement_source = Some(raw.clone());
            }
            ContainerEdit::Object { raw, .. } => {
                self.replace_start = self.found_start.unwrap_or(close);
                self.replace_end = self.found_end.unwrap_or(close);
                self.replacement_source = Some(raw.clone());
            }
            _ => {
                self.replace_start = 0;
                self.replace_end = 0;
            }
        }
        self.phase = RewritePhase::Prefix;
    }

    fn copy_prefix(&mut self, source: &ChunkedSource) -> RewriteStep {
        let copied = source.append_range(&mut self.copy_cursor, self.replace_start, &mut self.output);
        if self.copy_cursor == self.replace_start {
            self.phase = RewritePhase::Replacement;
        }
        RewriteStep { bytes: copied, ..Default::default() }
    }

    fn write_replacement(&mut self) -> RewriteStep {
        let mut copied = copy_text_chunk(&self.replacement, &mut self.replacement_cursor, self.replacement.len(), &mut self.output);
        if self.replacement_cursor == self.replacement.len() && copied == 0 {
            if let Some(source) = &self.replacement_source {
                copied += source.append_range(&mut self.replacement_source_cursor, source.len, &mut self.output);
            }
        }
        if self.replacement_cursor == self.replacement.len() && self.replacement_source.as_ref().is_none_or(|source| self.replacement_source_cursor == source.len) && copied == 0 {
            copied += copy_text_chunk(&self.replacement_suffix, &mut self.replacement_suffix_cursor, self.replacement_suffix.len(), &mut self.output);
        }
        if self.replacement_cursor == self.replacement.len() && self.replacement_source.as_ref().is_none_or(|source| self.replacement_source_cursor == source.len) && self.replacement_suffix_cursor == self.replacement_suffix.len() {
            self.copy_cursor = self.replace_end;
            self.phase = RewritePhase::Suffix;
        }
        RewriteStep { bytes: copied, ..Default::default() }
    }

    fn copy_suffix(&mut self, source: &ChunkedSource) -> RewriteStep {
        let copied = source.append_range(&mut self.copy_cursor, source.len, &mut self.output);
        if self.copy_cursor == source.len {
            self.phase = RewritePhase::Done;
        }
        RewriteStep { bytes: copied, ..Default::default() }
    }

    fn take_output(&mut self) -> Option<ChunkedSource> {
        (self.phase == RewritePhase::Done).then(|| {
            let mut source = ChunkedSource::default();
            for chunk in std::mem::take(&mut self.output) {
                source.push(chunk);
            }
            source
        })
    }

    fn needs_restart(&self) -> bool {
        matches!(self.edit, ContainerEdit::Option { pressed: false, .. }) && self.changed
    }

    fn restart(&self) -> Option<Self> {
        self.needs_restart().then(|| Self::new(self.edit.clone()))
    }
}

#[derive(Debug)]
enum TryValueRewrite {
    Vector(VectorRewrite),
    Container(ContainerRewrite),
}

impl TryValueRewrite {
    fn checkpoint(&self) -> u64 {
        match self {
            Self::Vector(rewrite) => rewrite.checkpoint(),
            Self::Container(rewrite) => rewrite.checkpoint(),
        }
    }

    fn target_index(&self) -> u64 {
        match self {
            Self::Vector(rewrite) => rewrite.requested_target_index,
            Self::Container(_) => 0,
        }
    }

    fn advance(&mut self, source: &ChunkedSource) -> RewriteStep {
        match self {
            Self::Vector(rewrite) => rewrite.advance(source),
            Self::Container(rewrite) => rewrite.advance(source),
        }
    }

    fn output(&self) -> &[std::sync::Arc<str>] {
        match self {
            Self::Vector(rewrite) => &rewrite.output,
            Self::Container(rewrite) => &rewrite.output,
        }
    }

    fn is_final_preview(&self) -> bool {
        match self {
            Self::Vector(rewrite) => rewrite.is_final_preview(),
            Self::Container(rewrite) => !rewrite.needs_restart(),
        }
    }

    fn restart(&self) -> Option<Self> {
        match self {
            Self::Vector(rewrite) if !rewrite.is_final_preview() => Some(Self::Vector(rewrite.restart())),
            Self::Container(rewrite) => rewrite.restart().map(Self::Container),
            _ => None,
        }
    }

    fn take_output(&mut self) -> Option<ChunkedSource> {
        match self {
            Self::Vector(rewrite) => rewrite.take_output(),
            Self::Container(rewrite) => rewrite.take_output(),
        }
    }
}

fn copy_text_chunk(source: &str, cursor: &mut usize, end: usize, output: &mut Vec<std::sync::Arc<str>>) -> usize {
    if *cursor >= end {
        return 0;
    }
    let start = *cursor;
    let mut next = start.saturating_add(MAX_TRY_VALUE_BYTES_PER_STEP).min(end);
    while next > start && !source.is_char_boundary(next) {
        next -= 1;
    }
    if next == start {
        next = end.min(start + source[start..].chars().next().map(char::len_utf8).unwrap_or(1));
    }
    output.push(std::sync::Arc::from(&source[start..next]));
    *cursor = next;
    next - start
}
//#endregion 🔖️Values

//#region 🔖️Continuation
fn queue(payload: &SetTryValueStep) -> Effect {
    Effect::DispatchAction {
        req: RequestId(NEXT_TRY_VALUE_REQUEST.fetch_add(1, Ordering::Relaxed)),
        action: SET_TRY_VALUE_STEP_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(json!({
            "appId": payload.app_id,
            "documentId": payload.document_id,
            "operationId": payload.operation_id,
            "generation": payload.generation,
            "cursor": payload.cursor,
            "targetIndex": payload.target_index,
            "baseRevision": payload.base_revision,
        }))),
        delay_ms: 0,
    }
}

fn continuation_emit(generation: u64, next: SetTryValueStep) -> Emit<FormMutation, FormsConfigMutation> {
    Emit { coalesce_key: Some(format!("setTryValue:{generation}")), effects: vec![queue(&next)], ui_scope: UiDirtyScope::None, ..Default::default() }
}

fn stage_chunk(session: &mut TryValueSession, chunk: String) -> FormsConfigMutation {
    update_digest(session, chunk.as_bytes());
    let index = session.staged_chunks;
    session.staged_chunks += 1;
    FormsConfigMutation::StageTryValueChunk { staging_id: session.staging_id.clone(), index, chunk }
}

fn commit_mutation(session: &TryValueSession) -> FormsConfigMutation {
    FormsConfigMutation::CommitTryValue { key: session.key.clone(), staging_id: session.staging_id.clone(), content_id: content_id(session), chunk_count: session.staged_chunks }
}

fn finish_try_value(generation: u64, session: TryValueSession, mutations: Vec<FormsConfigMutation>) -> Emit<FormMutation, FormsConfigMutation> {
    let mut mutations = mutations;
    mutations.push(commit_mutation(&session));
    Emit { config_mutations: mutations, coalesce_key: Some(format!("setTryValue:{generation}")), ui_scope: UiDirtyScope::Full, ..Default::default() }
}

fn next_payload(generation: u64, session: &TryValueSession, target_index: u64) -> SetTryValueStep {
    let rewrite_progress = session.rewrite.as_ref().map_or(session.prepared_cursor as u64, TryValueRewrite::checkpoint);
    SetTryValueStep {
        app_id: session.app_id.clone(),
        document_id: session.document_id.clone(),
        operation_id: session.operation_id.clone(),
        generation,
        cursor: session.progress_base.saturating_add(rewrite_progress),
        target_index,
        base_revision: session.base_revision.clone(),
    }
}

fn advance_prepared(generation: u64, mut session: TryValueSession) -> Emit<FormMutation, FormsConfigMutation> {
    let value = session.prepared_value.as_ref().expect("prepared value session");
    let value_len = value.len();
    let mut parts = Vec::new();
    value.append_range(&mut session.prepared_cursor, value_len, &mut parts);
    let chunk = parts.iter().fold(String::new(), |mut output, part| {
        output.push_str(part);
        output
    });
    let end = session.prepared_cursor;
    let staged = stage_chunk(&mut session, chunk);
    if end == value_len {
        return finish_try_value(generation, session, vec![staged]);
    }
    let next = next_payload(generation, &session, 0);
    let key = session_job_key(&session, generation);
    put_session(key, session).expect("taken Forms session always has admission");
    Emit { config_mutations: vec![staged], coalesce_key: Some(format!("setTryValue:{generation}")), effects: vec![queue(&next)], ui_scope: UiDirtyScope::None, ..Default::default() }
}

/// ⏱️ Advances one bounded scan/copy/vector slice and queues the next generation-checked prefix.
pub async fn advance_try_value(payload: &SetTryValueStep, config: &FormsConfig) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let Some(mut session) = take_session(payload) else { return Ok(Emit::default()) };
    let target_index = session.rewrite.as_ref().map_or(0, TryValueRewrite::target_index);
    let checkpoint = session.rewrite.as_ref().map_or(session.prepared_cursor as u64, TryValueRewrite::checkpoint);
    let active = active_generations().lock().expect("forms active generations lock").get(&(payload.app_id.clone(), payload.document_id.clone(), payload.operation_id.clone())).copied();
    if active != Some(payload.generation) || session.baseline_content_id.as_deref() != config.try_values.get_json(&session.key) || target_index != payload.target_index || session.progress_base.saturating_add(checkpoint) != payload.cursor {
        discard_staged_try_value(&session.staging_id);
        return Ok(Emit::default());
    }
    if let Some(source_content_id) = session.source_content_id.clone() {
        let chunk_count = config.try_values.content_chunk_count_by_id(&source_content_id);
        if let Some(chunk) = config.try_values.content_chunk_by_id(&source_content_id, session.source_chunk_cursor) {
            session.progress_base = session.progress_base.saturating_add(chunk.len() as u64);
            session.source.push(chunk);
            session.source_chunk_cursor += 1;
        }
        if session.source_chunk_cursor >= chunk_count {
            session.source_content_id = None;
        }
        let next = next_payload(payload.generation, &session, target_index);
        let key = session_job_key(&session, payload.generation);
        put_session(key, session).expect("taken Forms session always has admission");
        return Ok(continuation_emit(payload.generation, next));
    }
    if session.prepared_value.is_some() {
        return Ok(advance_prepared(payload.generation, session));
    }
    let rewrite = session.rewrite.as_mut().expect("vector rewrite session");
    let step = rewrite.advance(&session.source);
    let mut mutations = Vec::new();
    if rewrite.is_final_preview() && rewrite.output().len() > session.staged_cursor {
        let chunk = rewrite.output()[session.staged_cursor..].iter().fold(String::new(), |mut output, part| {
            output.push_str(part);
            output
        });
        session.staged_cursor = rewrite.output().len();
        mutations.push(stage_chunk(&mut session, chunk));
    }
    if step.complete {
        let is_final = session.rewrite.as_ref().is_some_and(TryValueRewrite::is_final_preview);
        return Ok(match session.rewrite.as_mut().and_then(TryValueRewrite::take_output) {
            Some(_source) if is_final => finish_try_value(payload.generation, session, mutations),
            Some(source) => {
                let rewrite = session.rewrite.as_ref().expect("vector rewrite session");
                let rewrite_checkpoint = rewrite.checkpoint();
                let restarted = rewrite.restart().expect("non-final vector preview restarts");
                session.progress_base = session.progress_base.saturating_add(rewrite_checkpoint);
                session.source = source;
                session.rewrite = Some(restarted);
                session.staged_cursor = 0;
                let next = next_payload(payload.generation, &session, target_index);
                let key = session_job_key(&session, payload.generation);
                put_session(key, session).expect("taken Forms session always has admission");
                continuation_emit(payload.generation, next)
            }
            None => Emit::default(),
        });
    }
    let next = next_payload(payload.generation, &session, target_index);
    let key = session_job_key(&session, payload.generation);
    put_session(key, session).expect("taken Forms session always has admission");
    Ok(Emit { config_mutations: mutations, coalesce_key: Some(format!("setTryValue:{}", payload.generation)), effects: vec![queue(&next)], ui_scope: UiDirtyScope::None, ..Default::default() })
}
//#endregion 🔖️Continuation

//#region 🔖️Payloads
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
}
//#endregion 🔖️Payloads

//#region 🔖️Handlers
async fn start_try_value(payload: &SetTryValue, operation: &semio_framework_plugin::AppOperationContext, config: &FormsConfig, input: ChunkedSource) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    if payload.key.len() > 512 {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.key-too-large"), "the Forms try-value key exceeds 512 UTF-8 bytes"));
    }
    let cleanup = cancel_pending_generations(operation);
    let generation = operation.generation;
    let operation_id = operation.operation_id.to_string();
    let scope = (operation.app_instance_id.to_string(), operation.parent_document_id.clone(), operation_id.clone());
    active_generations().lock().expect("forms active generations lock").insert(scope.clone(), generation);
    let staging_id = format!("try-value-stage-{}-{operation_id}-{generation}", operation.app_instance_id);
    let current_content_id = config.try_values.get_owned_json(&payload.key);
    let input = std::sync::Arc::new(input);
    let rewrite = if let Some(target_index) = payload.vector_index {
        if usize::try_from(target_index).is_err() {
            return Ok(Emit::default());
        }
        let mut rewrite = VectorRewrite::new(input.clone(), target_index);
        rewrite.key_token = serde_json::to_string(&payload.key).map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.key-invalid"), "the Forms try-value key cannot be serialized"))?;
        Some(TryValueRewrite::Vector(rewrite))
    } else if let Some(option_value) = payload.option_value.as_deref() {
        Some(TryValueRewrite::Container(
            ContainerRewrite::option(option_value, input.clone()).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.option-too-large"), "the option identifier exceeds the bounded Forms value limit"))?,
        ))
    } else if let Some(param_key) = payload.param_key.as_deref() {
        Some(TryValueRewrite::Container(
            ContainerRewrite::object(param_key, input.clone()).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.parameter-too-large"), "the parameter identifier exceeds the bounded Forms value limit"))?,
        ))
    } else {
        None
    };
    if let Some(rewrite) = rewrite {
        let target_index = rewrite.target_index();
        let cursor = rewrite.checkpoint();
        let job_key = FormsJobKey { app_id: scope.0.clone(), document_id: scope.1.clone(), operation_id: scope.2.clone(), generation };
        if let Err(rejected) = put_session(
            job_key,
            TryValueSession {
                app_id: scope.0.clone(),
                document_id: scope.1.clone(),
                operation_id: scope.2.clone(),
                base_revision: operation.canonical_base_revision_hex(),
                baseline_content_id: current_content_id.clone(),
                progress_base: 0,
                key: payload.key.clone(),
                staging_id,
                staged_cursor: 0,
                staged_chunks: 0,
                digest_high: 0x6c62272e07bb0142,
                digest_low: 0x62b821756295c58d,
                digest_third: 0x9e3779b185ebca87,
                digest_fourth: 0xc2b2ae3d27d4eb4f,
                digest_len: 0,
                source: current_content_id
                    .is_none()
                    .then(|| {
                        let fallback = match &rewrite {
                            TryValueRewrite::Vector(_) => "null",
                            TryValueRewrite::Container(ContainerRewrite { edit: ContainerEdit::Option { .. }, .. }) => "[]",
                            TryValueRewrite::Container(ContainerRewrite { edit: ContainerEdit::Object { .. }, .. }) => "{}",
                        };
                        ChunkedSource::from_text(fallback.into())
                    })
                    .unwrap_or_default(),
                source_content_id: current_content_id,
                source_chunk_cursor: 0,
                rewrite: Some(rewrite),
                prepared_value: None,
                prepared_cursor: 0,
            },
        ) {
            discard_staged_try_value(&rejected.staging_id);
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.busy"), "the bounded Forms try-value session pool is full"));
        }
        return Ok(Emit {
            config_mutations: cleanup,
            effects: vec![queue(&SetTryValueStep { app_id: scope.0, document_id: scope.1, operation_id: scope.2, generation, cursor, target_index, base_revision: operation.canonical_base_revision_hex() })],
            ui_scope: UiDirtyScope::None,
            ..Default::default()
        });
    }
    let job_key = FormsJobKey { app_id: scope.0.clone(), document_id: scope.1.clone(), operation_id: scope.2.clone(), generation };
    if let Err(rejected) = put_session(
        job_key,
        TryValueSession {
            app_id: scope.0.clone(),
            document_id: scope.1.clone(),
            operation_id: scope.2.clone(),
            base_revision: operation.canonical_base_revision_hex(),
            baseline_content_id: current_content_id,
            progress_base: 0,
            key: payload.key.clone(),
            staging_id,
            staged_cursor: 0,
            staged_chunks: 0,
            digest_high: 0x6c62272e07bb0142,
            digest_low: 0x62b821756295c58d,
            digest_third: 0x9e3779b185ebca87,
            digest_fourth: 0xc2b2ae3d27d4eb4f,
            digest_len: 0,
            source: ChunkedSource::default(),
            source_content_id: None,
            source_chunk_cursor: 0,
            rewrite: None,
            prepared_value: Some(input),
            prepared_cursor: 0,
        },
    ) {
        discard_staged_try_value(&rejected.staging_id);
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.busy"), "the bounded Forms try-value session pool is full"));
    }
    Ok(Emit {
        config_mutations: cleanup,
        effects: vec![queue(&SetTryValueStep { app_id: scope.0, document_id: scope.1, operation_id: scope.2, generation, cursor: 0, target_index: 0, base_revision: operation.canonical_base_revision_hex() })],
        ui_scope: UiDirtyScope::None,
        ..Default::default()
    })
}

pub async fn handle(payload: &SetTryValue, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    if payload.value_json.is_none() && payload.option_value.is_none() {
        return Ok(Emit::default());
    }
    let operation = doc.operation()?;
    let chunk = payload.value_json.as_ref().map(ChunkAddressableJson::owner).unwrap_or_else(|| std::sync::Arc::from("false"));
    let input_count = payload.input_count.unwrap_or(1);
    if input_count > 1 && payload.input_id.is_none() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.input-id-required"), "multi-chunk Forms input requires an explicit input id"));
    }
    let input_id = payload.input_id.as_deref().unwrap_or(&payload.key);
    let Some(input) = stage_command_input(operation, "setTryValue", input_id, payload.input_index.unwrap_or(0), input_count, chunk)? else {
        return Ok(Emit::default());
    };
    start_try_value(payload, &input.operation, cfg.snapshot, input.source).await
}

pub async fn handle_step(payload: &SetTryValueStep, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let operation = doc.operation()?;
    if payload.app_id.len() > 10
        || payload.document_id.len() > 256
        || payload.operation_id.len() > 20
        || payload.base_revision.len() != 64
        || payload.app_id != operation.app_instance_id.to_string()
        || payload.document_id != operation.parent_document_id
        || payload.operation_id.parse::<u64>().is_err()
        || payload.base_revision != operation.canonical_base_revision_hex()
    {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-value.checkpoint-invalid"), "the Forms continuation identity is invalid or stale"));
    }
    if let Some(result) = crate::editor::forms::commands::set_try_values::advance_if_bulk(payload, cfg.snapshot).await {
        return result;
    }
    advance_try_value(payload, cfg.snapshot).await
}
//#endregion 🔖️Handlers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{forms_app_with_registry, FormsApp};
    use crate::editor::forms::FormsCommand;
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::PluginApp;

    fn rope(raw: &str) -> std::sync::Arc<ChunkedSource> {
        std::sync::Arc::new(ChunkedSource::from_text(raw.into()))
    }

    fn test_operation(document_id: &str, operation_id: u64) -> semio_framework_plugin::AppOperationContext {
        semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: document_id.into(), operation_id, generation: 1, canonical_base_revision: [0; 32] }
    }

    async fn app_with_document_id(id: &str) -> FormsApp {
        let mut app = forms_app_with_registry().await;
        let mut snapshot = app.snapshot().await.expect("Forms snapshot");
        snapshot.id = id.into();
        let envelope = store::create_document_envelope::<_, FormMutation>(crate::artifacts::forms::FORMS_DOCUMENT_SCHEMA, id, snapshot, None);
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
            app.handle_action("setTryValue", Some(args), &meta("local")).await.expect("public action dispatch");
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Forms public action envelope exceeded 8 ms");
        }

        sessions().lock().expect("forms sessions lock").clear();
        active_generations().lock().expect("forms active generations lock").clear();
        *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();

        let mut result = None;
        for args in dispatch_chunks("after-restart") {
            let started = std::time::Instant::now();
            result = Some(app.handle_action("setTryValue", Some(&args), &meta("local")).await.expect("replayed public action dispatch"));
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
            result = Some(app.handle_action(SET_TRY_VALUE_STEP_ACTION_ID, Some(&args), &meta("local")).await.expect("Forms continuation action dispatch"));
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "Forms handler/job/op-codec/diff/apply envelope exceeded 8 ms");
        }
        let config = app.test_config().await;
        let content_id = config.try_values.get_json("public-scalar").expect("committed scalar content id").to_string();
        assert_eq!(content_id.len(), 85);
        assert_eq!(materialize_owned_try_value(&config.try_values, "public-scalar"), Some(chunks.concat()));
        assert!(input_registry().lock().expect("forms input registry lock").blobs.is_empty());
        assert!(sessions().lock().expect("forms sessions lock").is_empty());
        assert!(active_generations().lock().expect("forms active generations lock").is_empty());
        let serialized = serde_json::to_vec(&config).expect("serialize completed public scalar config");
        crate::editor::forms::config::clear_try_value_staging_for_replay();
        *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
        sessions().lock().expect("forms sessions lock").clear();
        active_generations().lock().expect("forms active generations lock").clear();
        let reopened: FormsConfig = serde_json::from_slice(&serialized).expect("cold reopen completed public scalar config");
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
        first.handle_action("setTryValue", Some(&chunk("a", 0)), &meta("document-a")).await.expect("first Forms chunk A");
        second.handle_action("setTryValue", Some(&chunk("b", 0)), &meta("document-b")).await.expect("first Forms chunk B");
        assert_eq!(input_registry().lock().expect("forms input registry lock").blobs.len(), 2);
        let first_step = continuation(first.handle_action("setTryValue", Some(&chunk("a", 1)), &meta("document-a")).await.expect("second Forms chunk A")).expect("Forms continuation A");
        let second_step = continuation(second.handle_action("setTryValue", Some(&chunk("b", 1)), &meta("document-b")).await.expect("second Forms chunk B")).expect("Forms continuation B");
        let first_payload: SetTryValueStep = serde_json::from_value(first_step).expect("Forms checkpoint A");
        let second_payload: SetTryValueStep = serde_json::from_value(second_step.clone()).expect("Forms checkpoint B");
        assert_eq!(first_payload.document_id, "forms-document-a");
        assert_eq!(second_payload.document_id, "forms-document-b");

        first.handle_action("resetTry", Some(&serde_json::json!({})), &meta("document-a")).await.expect("cancel Forms document A");
        assert!(!sessions().lock().expect("forms sessions lock").keys().any(|key| key.document_id == "forms-document-a"));
        assert!(sessions().lock().expect("forms sessions lock").keys().any(|key| key.document_id == "forms-document-b"));
        let sibling = second.handle_action(SET_TRY_VALUE_STEP_ACTION_ID, Some(&second_step), &meta("document-b")).await.expect("continue Forms document B");
        assert!(continuation(sibling).is_some(), "cancelling one Forms document must not cancel another");
    }

    #[semio_framework_async_macros::async_test]
    async fn public_action_bus_bounds_pathological_abandoned_and_sixty_fifth_inputs() {
        *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
        let mut pathological = app_with_document_id("forms-pathological").await;
        let started = std::time::Instant::now();
        let rejected = pathological.handle_action("setTryValue", Some(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "pathological", "inputIndex": 0, "inputCount": MAX_COMMAND_INPUT_CHUNKS + 1 })), &meta("pathological")).await;
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "pathological Forms count rejection exceeded 8 ms");
        assert_eq!(rejected.expect_err("pathological Forms count must be rejected").code.0, "forms.try-value.input-invalid");

        let mut abandoned = app_with_document_id("forms-abandoned").await;
        abandoned.handle_action("setTryValue", Some(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "abandoned", "inputIndex": 0, "inputCount": 2 })), &meta("abandoned")).await.expect("stage abandoned Forms input");
        input_registry().lock().expect("forms input registry lock").tick = MAX_COMMAND_INPUT_IDLE_ACTIONS + 2;
        let mut expiry_driver = app_with_document_id("forms-expiry-driver").await;
        expiry_driver
            .handle_action("setTryValue", Some(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "fresh", "inputIndex": 0, "inputCount": 2 })), &meta("expiry-driver"))
            .await
            .expect("expire abandoned Forms input through public dispatch");
        assert!(!input_registry().lock().expect("forms input registry lock").blobs.keys().any(|key| key.input_id == "abandoned"));
        expiry_driver.handle_action("resetTry", Some(&serde_json::json!({})), &meta("expiry-driver")).await.expect("cancel incomplete Forms input");
        assert!(!input_registry().lock().expect("forms input registry lock").blobs.keys().any(|key| key.document_id == "forms-expiry-driver"));

        *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
        let mut admitted = Vec::new();
        for index in 0..MAX_LIVE_TRY_VALUE_SESSIONS {
            let mut app = app_with_document_id(&format!("forms-admission-{index}")).await;
            let started = std::time::Instant::now();
            app.handle_action("setTryValue", Some(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "input", "inputIndex": 0, "inputCount": 2 })), &meta("admission")).await.expect("admit bounded Forms input");
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "Forms admitted action envelope exceeded 8 ms");
            admitted.push(app);
        }
        let mut sixty_fifth = app_with_document_id("forms-admission-65").await;
        let started = std::time::Instant::now();
        let busy =
            sixty_fifth.handle_action("setTryValue", Some(&serde_json::json!({ "key": "k", "valueJson": "0", "inputId": "input", "inputIndex": 0, "inputCount": 2 })), &meta("admission")).await.expect_err("the 65th Forms public input must be Busy");
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "Forms 65th Busy envelope exceeded 8 ms");
        assert_eq!(busy.code.0, "forms.try-value.busy");
        assert_eq!(input_registry().lock().expect("forms input registry lock").blobs.len(), MAX_LIVE_TRY_VALUE_SESSIONS);
        drop(admitted);
        *input_registry().lock().expect("forms input registry lock") = FormsInputRegistry::default();
    }
}
//#endregion 🧪️Tests
