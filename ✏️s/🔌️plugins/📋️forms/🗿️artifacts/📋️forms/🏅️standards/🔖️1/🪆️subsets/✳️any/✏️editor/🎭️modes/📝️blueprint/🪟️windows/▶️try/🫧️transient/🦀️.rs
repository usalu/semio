//! 🫧️ Unfinished answers and continuation authority for one exact Forms Try window.

use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;
use std::fmt;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};

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

impl semio_framework_value::ToValue for FormsTryValues {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Object(self.root.iter().map(|(key, content)| (key.clone(), semio_framework_value::DslValue::Array(content.chunks.iter().map(|chunk| semio_framework_value::DslValue::String(chunk.to_string())).collect()))).collect())
    }
}

impl semio_framework_value::FromValue for FormsTryValues {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Forms try values")) };
        if entries.len() > MAX_TRY_VALUE_ENTRIES { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Forms try values exceed 64 entries")); }
        let mut values = Self::default();
        for (key, entry) in entries {
            if key.len() > 512 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "a Forms try-value key exceeds 512 UTF-8 bytes")); }
            let semio_framework_value::DslValue::Array(items) = entry else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for a Forms try value entry")) };
            let mut chunks = Vec::with_capacity(items.len());
            for item in items {
                let semio_framework_value::DslValue::String(chunk) = item else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string chunk")) };
                if chunk.len() > 4_096 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "a Forms try-value chunk exceeds 4,096 UTF-8 bytes")); }
                chunks.push(Arc::<str>::from(chunk));
            }
            let content_id = try_value_content_id(&chunks);
            values = values.with_chunks(&key, content_id, Arc::new(chunks));
        }
        Ok(values)
    }
}

impl semio_framework_dsl_record::DslField for FormsTryValues {
    fn shape() -> semio_framework_dsl_record::Shape { semio_framework_dsl_record::Shape::Map(Box::new(semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Text)))) }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Map(self.root.iter().map(|(key, content)| (key.clone(), semio_framework_dsl_record::FieldValue::List(content.chunks.iter().map(|chunk| semio_framework_dsl_record::FieldValue::Text(chunk.to_string())).collect()))).collect())
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Map(entries) = value else { return Err(format!("expected Map, found {value:?}")) };
        let mut values = Self::default();
        for (key, value) in entries {
            let semio_framework_dsl_record::FieldValue::List(items) = value else { return Err(format!("expected List, found {value:?}")) };
            let mut chunks = Vec::with_capacity(items.len());
            for item in items {
                let semio_framework_dsl_record::FieldValue::Text(chunk) = item else { return Err(format!("expected Text, found {item:?}")) };
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

impl semio_framework_value::retirement::RetirementCursor for SharedTextRetirement {
    fn close_step(&mut self, maximum_bytes: usize) -> semio_framework_value::retirement::RetirementStep {
        let Some(value) = self.value.as_ref() else { return semio_framework_value::retirement::RetirementStep::Complete };
        if Arc::strong_count(value) > 1 {
            self.value.take();
            self.remaining = 0;
            return semio_framework_value::retirement::RetirementStep::Complete;
        }
        if self.remaining > 0 {
            if maximum_bytes == 0 { return semio_framework_value::retirement::RetirementStep::BudgetExhausted; }
            let bytes = maximum_bytes.min(self.remaining);
            self.remaining -= bytes;
            return semio_framework_value::retirement::RetirementStep::Bytes(bytes);
        }
        self.value.take();
        semio_framework_value::retirement::RetirementStep::Complete
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

impl semio_framework_value::retirement::RetireOwned for SharedText {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let remaining = self.0.len();
        Box::new(SharedTextRetirement { value: ManuallyDrop::new(Some(self.0)), remaining })
    }
}

struct SharedChunksRetirement {
    owner: ManuallyDrop<Option<Arc<Vec<Arc<str>>>>>,
    chunks: ManuallyDrop<Option<Vec<Arc<str>>>>,
}

impl semio_framework_value::retirement::RetirementCursor for SharedChunksRetirement {
    fn close_step(&mut self, _maximum_bytes: usize) -> semio_framework_value::retirement::RetirementStep {
        if let Some(owner) = self.owner.take() {
            match Arc::try_unwrap(owner) {
                Ok(chunks) => *self.chunks = Some(chunks),
                Err(shared) => {
                    drop(shared);
                    return semio_framework_value::retirement::RetirementStep::Complete;
                }
            }
        }
        let Some(chunks) = self.chunks.as_mut() else { return semio_framework_value::retirement::RetirementStep::Complete };
        match chunks.pop() {
            Some(chunk) => semio_framework_value::retirement::RetirementStep::Child(SharedText(chunk).retirement()),
            None => {
                self.chunks.take();
                semio_framework_value::retirement::RetirementStep::Complete
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

impl semio_framework_value::retirement::RetireOwned for SharedChunks {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(SharedChunksRetirement { owner: ManuallyDrop::new(Some(self.0)), chunks: ManuallyDrop::new(None) })
    }
}

impl semio_framework_value::retirement::RetireOwned for TryValueContent {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::sequence(vec![SharedText(self.id).retirement(), SharedChunks(self.chunks).retirement()])
    }
}

struct SharedTryValuesRetirement {
    root: ManuallyDrop<Option<Arc<BTreeMap<String, TryValueContent>>>>,
}

impl semio_framework_value::retirement::RetirementCursor for SharedTryValuesRetirement {
    fn close_step(&mut self, _maximum_bytes: usize) -> semio_framework_value::retirement::RetirementStep {
        let Some(root) = self.root.take() else { return semio_framework_value::retirement::RetirementStep::Complete };
        match Arc::try_unwrap(root) {
            Ok(values) => semio_framework_value::retirement::RetirementStep::Child(values.retirement()),
            Err(shared) => {
                drop(shared);
                semio_framework_value::retirement::RetirementStep::Complete
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

impl semio_framework_value::retirement::RetireOwned for FormsTryValues {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::sequence(vec![Box::new(SharedTryValuesRetirement { root: ManuallyDrop::new(Some(self.root)) }), self.revision.retirement()])
    }
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FormsTryWindowTransient {
    pub try_values: FormsTryValues,
}

semio_framework_value::artifact_retire_struct!(FormsTryWindowTransient { try_values });

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

//#region 🪢️TaxonomyMounts
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
//#endregion 🪢️TaxonomyMounts

//#region 🔖️Owner
semio_framework_plugin::transient_root! {
    state: FormsTryWindowTransient,
    mutation: FormsTryWindowTransientMutation,
    owner: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Forms Try Window Transient",
    payload_schema: "forms.try-window-transient",
    envelope: "s.forms.forms.try-window-transient",
    extension: "formstrywindowtransient",
}

semio_framework_plugin::window_transient_owners! {
    state: FormsTryWindowTransient,
    mutation: FormsTryWindowTransientMutation,
    windows: {
        FormsTryWindowTransientOwner => super::FORMS_PLAY_WINDOW_TRY,
    },
}
//#endregion 🔖️Owner
