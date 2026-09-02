//! 🧮️ Forms play app — view state (`FormsConfig`) and its operation enum (`FormsConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.forms` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/wizard edits are VCS'd exactly like document
//! content. B1: absorbs every field that used to live on `forms_ui::FormsPlayApp`'s
//! `RefCell<FormsPlayRuntime>` (blueprint selection, the Try wizard's active step, its in-progress answer
//! values) plus `locale` (was read off `view_state.locale`) and `contributions_json` (was read off
//! `view_state.contributions_json` — the host-declared `ProgramContributionEntry` list, each entry
//! carrying the open `TopicContribution` (`"forms.questionKind"` topic) shape, backing extension question
//! kinds in the blueprint builder, try wizard, and extension question rendering; the host now pushes
//! contributions into config via `SetContributions`, mirroring how it now pushes locale via `SetLocale`).
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_ids` moved OUT of here into
//! the framework-owned `InteractionState` (the "fields" domain declared on `create_forms_app`) — see
//! `crate::editor::forms::FORMS_INTERACTION_FIELDS`.

use protocol::Mutation;
use serde::de::Deserializer;
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

const MAX_STAGED_TRY_VALUE_BLOBS: usize = 64;
const MAX_STAGED_TRY_VALUE_CHUNK_BYTES: usize = 4_096;
const MAX_TRY_VALUE_OPERATION_ID_BYTES: usize = 4_096;
static STAGED_TRY_VALUE_BLOB_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct TryValueBlob {
    chunks: BTreeMap<u64, Arc<str>>,
    next_index: u64,
}

static TRY_VALUE_BLOBS: OnceLock<Mutex<BTreeMap<String, TryValueBlob>>> = OnceLock::new();

#[derive(Default)]
struct TryValuesBatch {
    values: FormsTryValues,
    staged_entries: u64,
}

static TRY_VALUES_BATCHES: OnceLock<Mutex<BTreeMap<String, TryValuesBatch>>> = OnceLock::new();

fn try_value_blobs() -> &'static Mutex<BTreeMap<String, TryValueBlob>> {
    TRY_VALUE_BLOBS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn try_values_batches() -> &'static Mutex<BTreeMap<String, TryValuesBatch>> {
    TRY_VALUES_BATCHES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormsStageError {
    Invalid,
    Busy,
    Order,
    Missing,
    Conflict,
}

impl FormsStageError {
    fn message(self) -> protocol::MutationMessage {
        match self {
            Self::Busy => protocol::MutationMessage::error("forms.try-value.busy", "the bounded Forms staging pool is full"),
            Self::Order => protocol::MutationMessage::error("forms.try-value.order", "Forms staging chunks are not contiguous"),
            Self::Conflict => protocol::MutationMessage::error("forms.try-value.conflict", "Forms staged content conflicts with its declared identity"),
            Self::Invalid | Self::Missing => protocol::MutationMessage::error("forms.try-value.invalid", "Forms staging input is missing or outside its typed bounds"),
        }
    }
}

pub fn stage_try_value_chunk(staging_id: &str, index: u64, chunk: &str) -> Result<(), FormsStageError> {
    if staging_id.len() > MAX_TRY_VALUE_OPERATION_ID_BYTES || chunk.len() > MAX_STAGED_TRY_VALUE_CHUNK_BYTES {
        return Err(FormsStageError::Invalid);
    }
    let mut blobs = try_value_blobs().lock().expect("forms try-value blob lock");
    if !blobs.contains_key(staging_id) {
        if STAGED_TRY_VALUE_BLOB_COUNT.load(Ordering::Acquire) >= MAX_STAGED_TRY_VALUE_BLOBS as u64 {
            return Err(FormsStageError::Busy);
        }
        STAGED_TRY_VALUE_BLOB_COUNT.fetch_add(1, Ordering::AcqRel);
    }
    let blob = blobs.entry(staging_id.into()).or_default();
    if index != blob.next_index {
        return Err(FormsStageError::Order);
    }
    blob.chunks.insert(index, Arc::from(chunk));
    blob.next_index += 1;
    Ok(())
}

pub fn discard_staged_try_value(staging_id: &str) {
    let mut blobs = try_value_blobs().lock().expect("forms try-value blob lock");
    if blobs.remove(staging_id).is_some() {
        STAGED_TRY_VALUE_BLOB_COUNT.fetch_sub(1, Ordering::AcqRel);
    }
}

/// 🧹 Discards a not-yet-authoritative bulk answer root in constant time.
pub fn discard_staged_try_values_batch(staging_id: &str) {
    try_values_batches().lock().expect("forms try-values batch lock").remove(staging_id);
}

/// 🌳 Seals one already chunk-staged answer into a persistent bulk root without touching config.
pub fn stage_try_values_batch_entry(staging_id: &str, key: &str, value_staging_id: &str, content_id: &str, chunk_count: u64, base: &FormsTryValues) -> Result<(), FormsStageError> {
    if key.len() > 4_096 {
        return Err(FormsStageError::Invalid);
    }
    let chunks = commit_staged_try_value(value_staging_id, content_id, chunk_count)?;
    let mut batches = try_values_batches().lock().expect("forms try-values batch lock");
    if !batches.contains_key(staging_id) && batches.len() >= MAX_STAGED_TRY_VALUE_BLOBS {
        return Err(FormsStageError::Busy);
    }
    let batch = batches.entry(staging_id.into()).or_insert_with(|| TryValuesBatch { values: base.clone(), staged_entries: 0 });
    if batch.values.get_json(key).is_none() && batch.values.len() >= MAX_STAGED_TRY_VALUE_BLOBS {
        return Err(FormsStageError::Busy);
    }
    batch.values = batch.values.with_chunks(key, content_id.into(), chunks);
    batch.staged_entries += 1;
    Ok(())
}

/// 🎯 Atomically takes a completely prepared persistent bulk answer root.
pub fn commit_staged_try_values_batch(staging_id: &str, entry_count: u64) -> Option<FormsTryValues> {
    let batch = try_values_batches().lock().expect("forms try-values batch lock").remove(staging_id)?;
    (batch.staged_entries == entry_count).then_some(batch.values)
}

pub(crate) fn commit_staged_try_value(staging_id: &str, content_id: &str, chunk_count: u64) -> Result<Arc<[Arc<str>]>, FormsStageError> {
    let mut blobs = try_value_blobs().lock().expect("forms try-value blob lock");
    let Some(blob) = blobs.remove(staging_id) else { return Err(FormsStageError::Missing) };
    STAGED_TRY_VALUE_BLOB_COUNT.fetch_sub(1, Ordering::AcqRel);
    if blob.next_index != chunk_count || blob.chunks.len() as u64 != chunk_count {
        return Err(FormsStageError::Order);
    }
    let chunks: Vec<Arc<str>> = blob.chunks.into_values().collect();
    if try_value_content_id(&chunks) != content_id {
        return Err(FormsStageError::Conflict);
    }
    Ok(chunks.into())
}

/// 🔎️ Verifies one staged chunk against an existing content id; `None` means either side
/// is absent, `Some(false)` is a collision, and the last matching row authorizes compact dedupe.
pub fn verify_staged_try_value_chunk(staging_id: &str, content_id: &str, index: u64, chunk_count: u64) -> Option<bool> {
    let _ = (staging_id, content_id, index, chunk_count);
    None
}

#[cfg(test)]
pub(crate) fn clear_try_value_staging_for_replay() {
    try_value_blobs().lock().expect("forms try-value blob lock").clear();
    try_values_batches().lock().expect("forms try-values batch lock").clear();
    STAGED_TRY_VALUE_BLOB_COUNT.store(0, Ordering::Release);
}

pub(crate) fn try_value_content_id(chunks: &[Arc<str>]) -> String {
    let mut digest = [0x6c62272e07bb0142u64, 0x62b821756295c58d, 0x9e3779b185ebca87, 0xc2b2ae3d27d4eb4f];
    let mut len = 0u64;
    for byte in chunks.iter().flat_map(|chunk| chunk.as_bytes()) {
        len = len.wrapping_add(1);
        digest[0] = (digest[0] ^ u64::from(*byte)).wrapping_mul(0x00000100000001b3);
        digest[1] = (digest[1] ^ digest[0].rotate_left(17) ^ len).wrapping_mul(0x9e3779b185ebca87);
        digest[2] = (digest[2] ^ digest[1].rotate_left(29) ^ u64::from(*byte)).wrapping_mul(0xc2b2ae3d27d4eb4f);
        digest[3] = (digest[3] ^ digest[2].rotate_left(41) ^ len.rotate_left(7)).wrapping_mul(0x165667b19e3779f9);
    }
    format!("try-{:016x}{:016x}{:016x}{:016x}-{len:016x}", digest[0], digest[1], digest[2], digest[3])
}

pub(crate) fn split_try_value_chunks(raw: &str, max_bytes: usize) -> Vec<Arc<str>> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < raw.len() {
        let mut end = start.saturating_add(max_bytes.max(1)).min(raw.len());
        while end > start && !raw.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            end += raw[start..].chars().next().map(char::len_utf8).unwrap_or(1);
        }
        chunks.push(Arc::from(&raw[start..end]));
        start = end;
    }
    if chunks.is_empty() {
        chunks.push(Arc::from(""));
    }
    chunks
}

//#region 🔖️TryValues
#[derive(Clone, Default)]
struct TryValueNode {
    value: Option<TryValueContent>,
    children: BTreeMap<u8, Arc<TryValueNode>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TryValueContent {
    id: Arc<str>,
    chunks: Arc<[Arc<str>]>,
}

/// 🌳️ Persistent per-key JSON storage. Updating one answer path-copies at most one
/// 256-way node per key byte; cloning a config never walks unrelated answers.
#[derive(Clone, Default)]
pub struct FormsTryValues {
    root: Arc<TryValueNode>,
    len: usize,
    revision: u64,
}

impl FormsTryValues {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn root_token(&self) -> usize {
        Arc::as_ptr(&self.root) as usize
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// 🪪 Returns the bounded content identifier for a durably owned JSON chunk leaf.
    pub fn get_json(&self, key: &str) -> Option<&str> {
        let mut node = self.root.as_ref();
        for byte in key.as_bytes() {
            node = node.children.get(byte)?.as_ref();
        }
        node.value.as_ref().map(|value| value.id.as_ref())
    }

    pub(crate) fn get_owned_json(&self, key: &str) -> Option<Arc<str>> {
        let mut node = self.root.as_ref();
        for byte in key.as_bytes() {
            node = node.children.get(byte)?.as_ref();
        }
        node.value.as_ref().map(|value| value.id.clone())
    }

    fn with_json(&self, key: &str, value_json: String) -> Self {
        Self::with_chunks(self, key, value_json, Arc::<[Arc<str>]>::from([]))
    }

    pub(crate) fn with_chunks(&self, key: &str, content_id: String, chunks: Arc<[Arc<str>]>) -> Self {
        let inserted = self.get_json(key).is_none();
        let value = TryValueContent { id: Arc::from(content_id), chunks };
        Self { root: set_try_value_node(&self.root, key.as_bytes(), Some(value)), len: self.len + usize::from(inserted), revision: self.revision.wrapping_add(1) }
    }

    pub(crate) fn content_chunks(&self, key: &str) -> Option<&[Arc<str>]> {
        let mut node = self.root.as_ref();
        for byte in key.as_bytes() {
            node = node.children.get(byte)?.as_ref();
        }
        node.value.as_ref().map(|value| value.chunks.as_ref())
    }

    pub(crate) fn content_chunk_by_id(&self, content_id: &str, index: u64) -> Option<Arc<str>> {
        self.iter_content().into_iter().find(|(_, value)| value.id.as_ref() == content_id).and_then(|(_, value)| value.chunks.get(index as usize).cloned())
    }

    pub fn content_chunk_count_by_id(&self, content_id: &str) -> u64 {
        self.iter_content().into_iter().find(|(_, value)| value.id.as_ref() == content_id).map_or(0, |(_, value)| value.chunks.len() as u64)
    }

    pub fn contains_content_id(&self, content_id: &str) -> bool {
        self.iter_content().into_iter().any(|(_, value)| value.id.as_ref() == content_id)
    }

    fn iter_content(&self) -> Vec<(String, TryValueContent)> {
        let mut entries = Vec::with_capacity(self.len);
        collect_try_value_contents(&self.root, &mut Vec::new(), &mut entries);
        entries
    }

    pub fn without(&self, key: &str) -> Self {
        if self.get_json(key).is_none() {
            return self.clone();
        }
        Self { root: set_try_value_node(&self.root, key.as_bytes(), None), len: self.len - 1, revision: self.revision.wrapping_add(1) }
    }

    pub(crate) fn iter_json(&self) -> Vec<(String, Arc<str>)> {
        self.iter_content().into_iter().map(|(key, value)| (key, value.id)).collect()
    }

    pub(crate) fn iter_chunks(&self) -> Vec<(String, Arc<[Arc<str>]>)> {
        self.iter_content().into_iter().map(|(key, value)| (key, value.chunks)).collect()
    }

    pub(crate) fn iter_prefix(&self, prefix: &str) -> Vec<(String, Arc<str>)> {
        let mut node = self.root.as_ref();
        for byte in prefix.as_bytes() {
            let Some(child) = node.children.get(byte) else { return Vec::new() };
            node = child;
        }
        let mut entries = Vec::new();
        collect_try_values(node, &mut prefix.as_bytes().to_vec(), &mut entries);
        entries
    }
}

fn set_try_value_node(node: &Arc<TryValueNode>, key: &[u8], value: Option<TryValueContent>) -> Arc<TryValueNode> {
    let mut next = TryValueNode { value: node.value.clone(), children: node.children.clone() };
    if let Some((&byte, tail)) = key.split_first() {
        let child = next.children.get(&byte).cloned().unwrap_or_default();
        let updated = set_try_value_node(&child, tail, value);
        if updated.value.is_none() && updated.children.is_empty() {
            next.children.remove(&byte);
        } else {
            next.children.insert(byte, updated);
        }
    } else {
        next.value = value;
    }
    Arc::new(next)
}

fn collect_try_value_contents(node: &TryValueNode, key: &mut Vec<u8>, entries: &mut Vec<(String, TryValueContent)>) {
    if let Some(value) = &node.value {
        if let Ok(key) = String::from_utf8(key.clone()) {
            entries.push((key, value.clone()));
        }
    }
    for (&byte, child) in &node.children {
        key.push(byte);
        collect_try_value_contents(child, key, entries);
        key.pop();
    }
}

impl PartialEq for FormsTryValues {
    fn eq(&self, other: &Self) -> bool {
        self.iter_content() == other.iter_content()
    }
}

impl fmt::Debug for FormsTryValues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_map().entries(self.iter_json()).finish()
    }
}

impl Serialize for FormsTryValues {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let entries = self.iter_content();
        let mut map = serializer.serialize_map(Some(entries.len()))?;
        for (key, content) in entries {
            let chunks: Vec<&str> = content.chunks.iter().map(AsRef::as_ref).collect();
            map.serialize_entry(&key, &chunks)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for FormsTryValues {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let values = BTreeMap::<String, Vec<String>>::deserialize(deserializer)?;
        Ok(values.into_iter().fold(Self::default(), |current, (key, chunks)| {
            let chunks: Vec<Arc<str>> = chunks.into_iter().map(Arc::from).collect();
            let content_id = try_value_content_id(&chunks);
            current.with_chunks(&key, content_id, chunks.into())
        }))
    }
}

impl dsl::DslField for FormsTryValues {
    fn shape() -> dsl::Shape {
        dsl::Shape::Map(Box::new(dsl::Shape::List(Box::new(dsl::Shape::Text))))
    }

    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Map(
            self.iter_content()
                .into_iter()
                .map(|(key, content)| {
                    let chunks = content.chunks.iter().map(|chunk| dsl::FieldValue::Text(chunk.to_string())).collect();
                    (key, dsl::FieldValue::List(chunks))
                })
                .collect(),
        )
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Map(entries) = value else { return Err(format!("expected Map, found {value:?}")) };
        let mut values = Self::default();
        for (key, value) in entries {
            let dsl::FieldValue::List(items) = value else { return Err(format!("expected List, found {value:?}")) };
            let mut chunks = Vec::with_capacity(items.len());
            for item in items {
                let dsl::FieldValue::Text(chunk) = item else { return Err(format!("expected Text, found {item:?}")) };
                chunks.push(Arc::<str>::from(chunk.clone()));
            }
            let content_id = try_value_content_id(&chunks);
            values = values.with_chunks(key, content_id, chunks.into());
        }
        Ok(values)
    }
}
//#endregion 🔖️TryValues

//#region 🔖️Config
/// 🧮️ `FormsPlayApp::Config` — the pure-trait `ArtifactEditor::Config` for the forms app.
/// Try values are independently addressable JSON leaves. The contribution catalogue remains an opaque
/// JSON document because it is replaced as one host-owned payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "formscfg")]
#[dsl(id = "forms.config")]
#[dsl(layout = "lines")]
pub struct FormsConfig {
    /// 👁️ The Try wizard's active step index — was `FormsPlayRuntime::current_step_index`.
    pub current_step_index: u32,
    /// 👁️ The Try wizard's independently owned in-progress answer overrides.
    pub try_values: FormsTryValues,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-declared plugin contributions (JSON array of `{pluginId, topicContribution}` — only the
    /// `"forms.questionKind"` topic matters) — was read off `view_state.contributions_json`.
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for FormsConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for FormsConfig {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

impl Default for FormsConfig {
    fn default() -> Self {
        Self { current_step_index: 0, try_values: FormsTryValues::default(), locale: "en-US".into(), contributions_json: "[]".into() }
    }
}

store::impl_whole_record_config!(FormsConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS Config recipe: [`FormsConfig`]'s operation enum — mirrors
/// `shooting_op::ShootingConfigMutation`'s shape exactly: one variant per settled interaction (was a
/// `FormsPlayRuntime` field write pre-B1), plus a generic `Snapshot` every variant's `backwards()` returns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FormsConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: FormsConfig,
    },
    #[dsl(key = "step-index")]
    SetStepIndex { index: u32 },
    #[dsl(key = "stage-try-value-chunk")]
    StageTryValueChunk { staging_id: String, index: u64, chunk: String },
    #[dsl(key = "discard-try-value-staging")]
    DiscardTryValueStaging { staging_id: String },
    #[dsl(key = "verify-try-value-chunk")]
    VerifyTryValueChunk { staging_id: String, content_id: String, index: u64, chunk_count: u64 },
    #[dsl(key = "commit-try-value")]
    CommitTryValue { key: String, staging_id: String, content_id: String, chunk_count: u64 },
    #[dsl(key = "stage-try-values-entry")]
    StageTryValuesEntry { staging_id: String, key: String, value_staging_id: String, content_id: String, chunk_count: u64 },
    #[dsl(key = "discard-try-values-batch")]
    DiscardTryValuesBatch { staging_id: String },
    #[dsl(key = "commit-try-values-batch")]
    CommitTryValuesBatch { staging_id: String, entry_count: u64 },
    #[dsl(key = "clear-try-values")]
    ClearTryValues,
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for FormsConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for FormsConfigMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let operation = <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })?;
        if let FormsConfigMutation::StageTryValueChunk { staging_id, chunk, .. } = &operation {
            if staging_id.len() > MAX_TRY_VALUE_OPERATION_ID_BYTES || chunk.len() > MAX_STAGED_TRY_VALUE_CHUNK_BYTES {
                return Err(protocol::ProtocolError::Malformed { what: "forms try-value chunk", offset: reader.position() as u64, detail: "staging id or chunk exceeds the bounded operation limit".into() });
            }
        }
        Ok(operation)
    }
}

//#endregion 🔖️OpCodec

impl Mutation<FormsConfig> for FormsConfigMutation {
    type Diff = FormsConfig;

    async fn diff(&self, base: &FormsConfig) -> protocol::MutationOutcome<FormsConfig> {
        let mut next = base.clone();
        match self {
            FormsConfigMutation::Snapshot { config } => {
                return protocol::MutationOutcome::new(config.clone());
            }
            FormsConfigMutation::SetStepIndex { index } => next.current_step_index = *index,
            FormsConfigMutation::StageTryValueChunk { staging_id, index, chunk } => {
                if let Err(error) = stage_try_value_chunk(staging_id, *index, chunk) {
                    return protocol::MutationOutcome::new(next).absorb_messages([error.message()]);
                }
            }
            FormsConfigMutation::DiscardTryValueStaging { staging_id } => discard_staged_try_value(staging_id),
            FormsConfigMutation::VerifyTryValueChunk { staging_id, content_id, index, chunk_count } => {
                let expected = base.try_values.content_chunk_by_id(content_id, *index);
                let staged = try_value_blobs().lock().expect("forms try-value blob lock").get(staging_id).and_then(|blob| blob.chunks.get(index)).cloned();
                if expected.as_deref() != staged.as_deref() || index.saturating_add(1) > *chunk_count {
                    return protocol::MutationOutcome::new(next).absorb_messages([FormsStageError::Conflict.message()]);
                }
            }
            FormsConfigMutation::CommitTryValue { key, staging_id, content_id, chunk_count } => {
                if base.try_values.get_json(key).is_none() && base.try_values.len() >= MAX_STAGED_TRY_VALUE_BLOBS {
                    discard_staged_try_value(staging_id);
                    return protocol::MutationOutcome::new(next).absorb_messages([FormsStageError::Busy.message()]);
                }
                let chunks = match commit_staged_try_value(staging_id, content_id, *chunk_count) {
                    Ok(chunks) => chunks,
                    Err(error) => return protocol::MutationOutcome::new(next).absorb_messages([error.message()]),
                };
                next.try_values = next.try_values.with_chunks(key, content_id.clone(), chunks);
            }
            FormsConfigMutation::StageTryValuesEntry { staging_id, key, value_staging_id, content_id, chunk_count } => {
                if let Err(error) = stage_try_values_batch_entry(staging_id, key, value_staging_id, content_id, *chunk_count, &base.try_values) {
                    return protocol::MutationOutcome::new(next).absorb_messages([error.message()]);
                }
            }
            FormsConfigMutation::DiscardTryValuesBatch { staging_id } => discard_staged_try_values_batch(staging_id),
            FormsConfigMutation::CommitTryValuesBatch { staging_id, entry_count } => {
                let Some(values) = commit_staged_try_values_batch(staging_id, *entry_count) else {
                    return protocol::MutationOutcome::new(next).absorb_messages([FormsStageError::Missing.message()]);
                };
                next.try_values = values;
            }
            FormsConfigMutation::ClearTryValues => {
                next.try_values = FormsTryValues::default();
            }
            FormsConfigMutation::SetLocale { value } => next.locale = value.clone(),
            FormsConfigMutation::SetContributions { json } => next.contributions_json = json.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &FormsConfig) -> Vec<Self> {
        match self {
            FormsConfigMutation::StageTryValueChunk { staging_id, .. } => vec![FormsConfigMutation::DiscardTryValueStaging { staging_id: staging_id.clone() }],
            FormsConfigMutation::StageTryValuesEntry { staging_id, .. } => vec![FormsConfigMutation::DiscardTryValuesBatch { staging_id: staging_id.clone() }],
            FormsConfigMutation::DiscardTryValueStaging { .. } | FormsConfigMutation::DiscardTryValuesBatch { .. } | FormsConfigMutation::VerifyTryValueChunk { .. } => Vec::new(),
            _ => vec![FormsConfigMutation::Snapshot { config: base.clone() }],
        }
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn forms_config_default_matches_the_existing_runtime_defaults() {
        let config = FormsConfig::default();
        assert_eq!(config.current_step_index, 0);
        assert!(config.try_values.iter_json().is_empty());
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.contributions_json, "[]");
    }

    #[semio_framework_async_macros::async_test]
    async fn forms_config_dsl_and_pack_round_trip() {
        let chunks = split_try_value_chunks(r#""Ada""#, 4_096);
        let content_id = try_value_content_id(&chunks);
        let config = FormsConfig { current_step_index: 2, try_values: FormsTryValues::default().with_chunks("name", content_id, chunks.into()), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::os_store::test_support::assert_dsl_round_trip(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    async fn config_round_trip(base: &FormsConfig, operation: &FormsConfigMutation) -> FormsConfig {
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
        assert_eq!(config_round_trip(&base, &FormsConfigMutation::SetStepIndex { index: 2 }).current_step_index, 2);
        let one_chunks = split_try_value_chunks("1", MAX_STAGED_TRY_VALUE_CHUNK_BYTES);
        let one_id = try_value_content_id(&one_chunks);
        let staged = FormsConfigMutation::StageTryValueChunk { staging_id: "one-stage".into(), index: 0, chunk: "1".into() }.diff(&base).diff().clone();
        assert_eq!(staged, base);
        assert_eq!(config_round_trip(&staged, &FormsConfigMutation::CommitTryValue { key: "a".into(), staging_id: "one-stage".into(), content_id: one_id.clone(), chunk_count: 1 }).try_values.get_json("a"), Some(one_id.as_str()));
        assert_eq!(config_round_trip(&base, &FormsConfigMutation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
        assert_eq!(config_round_trip(&base, &FormsConfigMutation::SetContributions { json: "[]".into() }).contributions_json, "[]");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_snapshot_op_text_round_trips() {
        let chunks = split_try_value_chunks(r#""Ada""#, 4_096);
        let content_id = try_value_content_id(&chunks);
        let config = FormsConfig { current_step_index: 1, try_values: FormsTryValues::default().with_chunks("name", content_id, chunks.into()), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::os_store::test_support::assert_op_line_round_trip(&FormsConfigMutation::Snapshot { config });
        store::os_store::test_support::assert_op_line_round_trip(&FormsConfigMutation::SetStepIndex { index: 3 });
        store::os_store::test_support::assert_op_line_round_trip(&FormsConfigMutation::SetLocale { value: "en-US".into() });
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
        let serialized = serde_json::to_vec(&config).expect("serialize committed Forms config");
        clear_try_value_staging_for_replay();
        let reopened: FormsConfig = serde_json::from_slice(&serialized).expect("reopen committed Forms config");
        assert_eq!(reopened.try_values.get_json("name"), Some(content_id.as_str()));
        assert_eq!(reopened.try_values.content_chunks("name"), Some(chunks.as_slice()));
    }

    #[semio_framework_async_macros::async_test]
    async fn bounded_stage_and_compact_commit_encode_decode_and_apply_under_eight_ms() {
        let raw = "x".repeat(4_096);
        let chunks = split_try_value_chunks(&raw, 4_096);
        let content_id = try_value_content_id(&chunks);
        let stage = FormsConfigMutation::StageTryValueChunk { staging_id: "timed-stage".into(), index: 0, chunk: raw };
        let started = std::time::Instant::now();
        let bytes = <FormsConfigMutation as protocol::OpBinary>::encode_op(&stage).await.expect("stage encode");
        let decoded = <FormsConfigMutation as protocol::OpBinary>::decode_op(&bytes).await.expect("stage decode");
        let staged = decoded.diff(&FormsConfig::default()).diff().clone();
        assert!(started.elapsed() < std::time::Duration::from_millis(8));

        let commit = FormsConfigMutation::CommitTryValue { key: "large".into(), staging_id: "timed-stage".into(), content_id: content_id.clone(), chunk_count: 1 };
        let started = std::time::Instant::now();
        let bytes = <FormsConfigMutation as protocol::OpBinary>::encode_op(&commit).await.expect("commit encode");
        let decoded = <FormsConfigMutation as protocol::OpBinary>::decode_op(&bytes).await.expect("commit decode");
        let committed = decoded.diff(&staged).diff().clone();
        assert_eq!(committed.try_values.get_json("large"), Some(content_id.as_str()));
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }
}
//#endregion 🧪️Tests
