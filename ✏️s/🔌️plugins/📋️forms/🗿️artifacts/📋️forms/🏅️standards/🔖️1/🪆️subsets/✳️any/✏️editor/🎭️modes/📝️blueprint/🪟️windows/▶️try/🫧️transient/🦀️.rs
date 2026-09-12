//! 🫧️ Unfinished answers and continuation authority for one exact Forms Try window.

use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;
use std::fmt;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use store::retirement::{RetireOwned, RetirementCursor};

pub(crate) const MAX_TRY_VALUE_ENTRIES: usize = 64;

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

#[cfg(test)]
pub(crate) fn split_try_value_chunks(raw: &str, max_bytes: usize) -> Vec<Arc<str>> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < raw.len() {
        let mut end = start.saturating_add(max_bytes.max(1)).min(raw.len());
        while end > start && !raw.is_char_boundary(end) { end -= 1; }
        if end == start { end += raw[start..].chars().next().map(char::len_utf8).unwrap_or(1); }
        chunks.push(Arc::from(&raw[start..end]));
        start = end;
    }
    if chunks.is_empty() { chunks.push(Arc::from("")); }
    chunks
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TryValueContent {
    id: Arc<str>,
    chunks: Arc<Vec<Arc<str>>>,
}

#[derive(Clone, Default)]
pub struct FormsTryValues {
    root: Arc<BTreeMap<String, TryValueContent>>,
    revision: u64,
}

impl FormsTryValues {
    pub fn len(&self) -> usize { self.root.len() }
    pub fn is_empty(&self) -> bool { self.root.is_empty() }
    pub fn root_token(&self) -> usize { Arc::as_ptr(&self.root) as usize }
    pub fn revision(&self) -> u64 { self.revision }
    pub fn get_json(&self, key: &str) -> Option<&str> { self.root.get(key).map(|value| value.id.as_ref()) }
    pub(crate) fn get_owned_json(&self, key: &str) -> Option<Arc<str>> { self.root.get(key).map(|value| value.id.clone()) }
    pub(crate) fn with_chunks(&self, key: &str, content_id: String, chunks: Arc<Vec<Arc<str>>>) -> Self {
        let mut root = self.root.as_ref().clone();
        root.insert(key.into(), TryValueContent { id: Arc::from(content_id), chunks });
        Self { root: Arc::new(root), revision: self.revision.wrapping_add(1) }
    }
    #[cfg(test)]
    pub(crate) fn content_chunks(&self, key: &str) -> Option<&[Arc<str>]> { self.root.get(key).map(|value| value.chunks.as_slice()) }
    pub(crate) fn content_chunk_by_id(&self, content_id: &str, index: u64) -> Option<Arc<str>> {
        self.root.values().find(|value| value.id.as_ref() == content_id).and_then(|value| value.chunks.get(index as usize).cloned())
    }
    pub fn content_chunk_count_by_id(&self, content_id: &str) -> u64 {
        self.root.values().find(|value| value.id.as_ref() == content_id).map_or(0, |value| value.chunks.len() as u64)
    }
    pub fn contains_content_id(&self, content_id: &str) -> bool { self.root.values().any(|value| value.id.as_ref() == content_id) }
    pub fn without(&self, key: &str) -> Self {
        if !self.root.contains_key(key) { return self.clone(); }
        let mut root = self.root.as_ref().clone();
        root.remove(key);
        Self { root: Arc::new(root), revision: self.revision.wrapping_add(1) }
    }
    pub(crate) fn iter_json(&self) -> Vec<(String, Arc<str>)> { self.root.iter().map(|(key, value)| (key.clone(), value.id.clone())).collect() }
    pub(crate) fn iter_chunks(&self) -> Vec<(String, Arc<Vec<Arc<str>>>)> { self.root.iter().map(|(key, value)| (key.clone(), value.chunks.clone())).collect() }
    fn retained_bytes(&self) -> usize {
        self.root.iter().fold(0usize, |total, (key, value)| total.saturating_add(key.len()).saturating_add(value.id.len()).saturating_add(value.chunks.iter().map(|chunk| chunk.len()).sum::<usize>()))
    }
}

impl PartialEq for FormsTryValues {
    fn eq(&self, other: &Self) -> bool { self.root == other.root }
}

impl fmt::Debug for FormsTryValues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { formatter.debug_map().entries(self.iter_json()).finish() }
}

impl dsl::ToValue for FormsTryValues {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::Object(self.root.iter().map(|(key, content)| (key.clone(), dsl::DslValue::Array(content.chunks.iter().map(|chunk| dsl::DslValue::String(chunk.to_string())).collect()))).collect())
    }
}

impl dsl::FromValue for FormsTryValues {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let dsl::DslValue::Object(entries) = value else { return Err(dsl::ValueError::new("expected an object for Forms try values")) };
        if entries.len() > MAX_TRY_VALUE_ENTRIES { return Err(dsl::ValueError::new("Forms try values exceed 64 entries")); }
        let mut values = Self::default();
        for (key, entry) in entries {
            if key.len() > 512 { return Err(dsl::ValueError::new("a Forms try-value key exceeds 512 UTF-8 bytes")); }
            let dsl::DslValue::Array(items) = entry else { return Err(dsl::ValueError::new("expected an array for a Forms try value entry")) };
            let mut chunks = Vec::with_capacity(items.len());
            for item in items {
                let dsl::DslValue::String(chunk) = item else { return Err(dsl::ValueError::new("expected a string chunk")) };
                if chunk.len() > 4_096 { return Err(dsl::ValueError::new("a Forms try-value chunk exceeds 4,096 UTF-8 bytes")); }
                chunks.push(Arc::<str>::from(chunk));
            }
            let content_id = try_value_content_id(&chunks);
            values = values.with_chunks(&key, content_id, Arc::new(chunks));
        }
        Ok(values)
    }
}

impl dsl::DslField for FormsTryValues {
    fn shape() -> dsl::Shape { dsl::Shape::Map(Box::new(dsl::Shape::List(Box::new(dsl::Shape::Text)))) }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Map(self.root.iter().map(|(key, content)| (key.clone(), dsl::FieldValue::List(content.chunks.iter().map(|chunk| dsl::FieldValue::Text(chunk.to_string())).collect()))).collect())
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
            values = values.with_chunks(key, content_id, Arc::new(chunks));
        }
        Ok(values)
    }
}

struct SharedTextRetirement {
    value: ManuallyDrop<Option<Arc<str>>>,
    remaining: usize,
}

impl store::retirement::RetirementCursor for SharedTextRetirement {
    fn close_step(&mut self, maximum_bytes: usize) -> store::retirement::RetirementStep {
        let Some(value) = self.value.as_ref() else { return store::retirement::RetirementStep::Complete };
        if Arc::strong_count(value) > 1 {
            self.value.take();
            self.remaining = 0;
            return store::retirement::RetirementStep::Complete;
        }
        if self.remaining > 0 {
            if maximum_bytes == 0 { return store::retirement::RetirementStep::BudgetExhausted; }
            let bytes = maximum_bytes.min(self.remaining);
            self.remaining -= bytes;
            return store::retirement::RetirementStep::Bytes(bytes);
        }
        self.value.take();
        store::retirement::RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.value.is_none() && self.remaining == 0 }
}

impl Drop for SharedTextRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Forms shared text retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.value) };
    }
}

struct SharedText(Arc<str>);

impl store::retirement::RetireOwned for SharedText {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        let remaining = self.0.len();
        Box::new(SharedTextRetirement { value: ManuallyDrop::new(Some(self.0)), remaining })
    }
}

struct SharedChunksRetirement {
    owner: ManuallyDrop<Option<Arc<Vec<Arc<str>>>>>,
    chunks: ManuallyDrop<Option<Vec<Arc<str>>>>,
}

impl store::retirement::RetirementCursor for SharedChunksRetirement {
    fn close_step(&mut self, _maximum_bytes: usize) -> store::retirement::RetirementStep {
        if let Some(owner) = self.owner.take() {
            match Arc::try_unwrap(owner) {
                Ok(chunks) => *self.chunks = Some(chunks),
                Err(shared) => {
                    drop(shared);
                    return store::retirement::RetirementStep::Complete;
                }
            }
        }
        let Some(chunks) = self.chunks.as_mut() else { return store::retirement::RetirementStep::Complete };
        match chunks.pop() {
            Some(chunk) => store::retirement::RetirementStep::Child(SharedText(chunk).retirement()),
            None => {
                self.chunks.take();
                store::retirement::RetirementStep::Complete
            }
        }
    }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.chunks.is_none() }
}

impl Drop for SharedChunksRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Forms shared chunks retired before terminal-empty");
        unsafe {
            ManuallyDrop::drop(&mut self.owner);
            ManuallyDrop::drop(&mut self.chunks);
        }
    }
}

struct SharedChunks(Arc<Vec<Arc<str>>>);

impl store::retirement::RetireOwned for SharedChunks {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        Box::new(SharedChunksRetirement { owner: ManuallyDrop::new(Some(self.0)), chunks: ManuallyDrop::new(None) })
    }
}

impl store::retirement::RetireOwned for TryValueContent {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::sequence(vec![SharedText(self.id).retirement(), SharedChunks(self.chunks).retirement()])
    }
}

struct SharedTryValuesRetirement {
    root: ManuallyDrop<Option<Arc<BTreeMap<String, TryValueContent>>>>,
}

impl store::retirement::RetirementCursor for SharedTryValuesRetirement {
    fn close_step(&mut self, _maximum_bytes: usize) -> store::retirement::RetirementStep {
        let Some(root) = self.root.take() else { return store::retirement::RetirementStep::Complete };
        match Arc::try_unwrap(root) {
            Ok(values) => store::retirement::RetirementStep::Child(values.retirement()),
            Err(shared) => {
                drop(shared);
                store::retirement::RetirementStep::Complete
            }
        }
    }
    fn terminal_is_empty(&self) -> bool { self.root.is_none() }
}

impl Drop for SharedTryValuesRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Forms Try values retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.root) };
    }
}

impl store::retirement::RetireOwned for FormsTryValues {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::sequence(vec![Box::new(SharedTryValuesRetirement { root: ManuallyDrop::new(Some(self.root)) }), self.revision.retirement()])
    }
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FormsTryWindowTransient {
    pub try_values: FormsTryValues,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum FormsTryWindowTransientMutation {
    Snapshot { transient: FormsTryWindowTransient },
}

impl protocol::Mutation<FormsTryWindowTransient> for FormsTryWindowTransientMutation {
    type Diff = FormsTryWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set Forms Try Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "forms.try-window-transient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &FormsTryWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()) }
    }
    fn inverse(&self, base: &FormsTryWindowTransient) -> Vec<Self> { vec![Self::Snapshot { transient: base.clone() }] }
}

impl protocol::MutationDiff<FormsTryWindowTransient> for FormsTryWindowTransient {
    fn apply(&self, _base: &FormsTryWindowTransient) -> protocol::MutationApplyResult<FormsTryWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

impl store::ArtifactDsl for FormsTryWindowTransient {
    const EXTENSION: &'static str = "formstrywindowtransient";
    fn envelope_id() -> &'static str { "s.forms.forms.try-window-transient" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        dsl::json::from_json_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = dsl::json::to_json_string(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Forms Try window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for FormsTryWindowTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = dsl::json::to_json_string(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::Schema("Forms Try window transient pack envelope mismatch".into())); }
        let text = std::str::from_utf8(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::json::from_json_str(text).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

impl protocol::OpText for FormsTryWindowTransientMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for FormsTryWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

store::artifact_retire_struct!(FormsTryWindowTransient { try_values });

impl store::retirement::RetireOwned for FormsTryWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self { Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), transient.retirement()]) }
    }
}

fn preflight(mutation: &FormsTryWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let FormsTryWindowTransientMutation::Snapshot { transient } = mutation;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: transient.try_values.retained_bytes() };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Forms Try window transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: FormsTryWindowTransientMutation) -> FormsTryWindowTransient {
    match mutation { FormsTryWindowTransientMutation::Snapshot { transient } => transient }
}

pub struct FormsTryWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for FormsTryWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::FORMS_PLAY_WINDOW_TRY;
    type State = FormsTryWindowTransient;
    type Mutation = FormsTryWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

pub fn register(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<FormsTryWindowTransientOwner>()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> FormsTryWindowTransient {
    snapshot.and_then(|snapshot| snapshot.get::<FormsTryWindowTransientOwner>()).cloned().unwrap_or_default()
}

pub fn current(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> FormsTryWindowTransient { from_snapshot(view.window) }

pub fn addressed(view: &semio_framework_plugin::ViewModel, transient: FormsTryWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("forms-try-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("forms-try-window-stale"))?;
    if kind != super::FORMS_PLAY_WINDOW_TRY { return Err(semio_framework_plugin::Fault::from("forms-try-window-kind-required")); }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<FormsTryWindowTransientOwner>(id, FormsTryWindowTransientMutation::Snapshot { transient }))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormsTryWindowLease {
    pub window_id: String,
    pub window_kind_id: String,
    pub window_generation: u64,
    pub document_generation: u64,
}

impl FormsTryWindowLease {
    pub fn capture(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> Result<Self, semio_framework_plugin::Fault> {
        let snapshot = snapshot.ok_or_else(|| semio_framework_plugin::Fault::from("forms-try-window-transient-required"))?;
        if snapshot.window_kind_id() != super::FORMS_PLAY_WINDOW_TRY || snapshot.get::<FormsTryWindowTransientOwner>().is_none() { return Err(semio_framework_plugin::Fault::from("forms-try-window-transient-kind-required")); }
        Ok(Self { window_id: snapshot.window_id().into(), window_kind_id: snapshot.window_kind_id().into(), window_generation: snapshot.generation(), document_generation: snapshot.document_generation() })
    }

    pub fn matches(&self, window_id: &str, window_kind_id: &str, window_generation: u64, document_generation: u64) -> bool {
        self.window_id == window_id && self.window_kind_id == window_kind_id && self.window_generation == window_generation && self.document_generation == document_generation
    }
}
