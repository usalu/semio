//! 🫧️ Unfinished answers and continuation authority for one exact Forms Try window.

use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use semio_framework_value::retirement::{deferred, deferred_birth_bytes, deferred_birth_bytes_for, sequence, sequence_birth_bytes, arc_custody::SharedControlledRetirement, RetireOwned, RetirementCursor};
use semio_framework_value::SharedUtf8;

pub const MAX_TRY_VALUE_ENTRIES: usize = 64;

pub fn try_value_content_id(chunks: &[SharedUtf8]) -> String {
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
pub fn split_try_value_chunks(raw: &str, max_bytes: usize) -> Vec<SharedUtf8> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < raw.len() {
        let mut end = start.saturating_add(max_bytes.max(1)).min(raw.len());
        while end > start && !raw.is_char_boundary(end) { end -= 1; }
        if end == start { end += raw[start..].chars().next().map(char::len_utf8).unwrap_or(1); }
        chunks.push(SharedUtf8::from(&raw[start..end]));
        start = end;
    }
    if chunks.is_empty() { chunks.push(SharedUtf8::from("")); }
    chunks
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TryValueContent {
    id: SharedUtf8,
    chunks: Arc<Vec<SharedUtf8>>,
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
    pub fn get_json(&self, key: &str) -> Option<&str> { self.root.get(key).map(|value| value.id.as_str()) }
    pub fn with_chunks(&self, key: &str, content_id: String, chunks: Arc<Vec<SharedUtf8>>) -> Self {
        let mut root = self.root.as_ref().clone();
        root.insert(key.into(), TryValueContent { id: SharedUtf8::from(content_id), chunks });
        Self { root: Arc::new(root), revision: self.revision.wrapping_add(1) }
    }
    #[cfg(test)]
    pub fn content_chunks(&self, key: &str) -> Option<&[SharedUtf8]> { self.root.get(key).map(|value| value.chunks.as_slice()) }
    pub fn content_chunk_by_id(&self, content_id: &str, index: u64) -> Option<SharedUtf8> {
        self.root.values().find(|value| value.id.as_str() == content_id).and_then(|value| value.chunks.get(index as usize).cloned())
    }
    pub fn content_chunk_count_by_id(&self, content_id: &str) -> u64 {
        self.root.values().find(|value| value.id.as_str() == content_id).map_or(0, |value| value.chunks.len() as u64)
    }
    pub fn contains_content_id(&self, content_id: &str) -> bool { self.root.values().any(|value| value.id.as_str() == content_id) }
    pub fn without(&self, key: &str) -> Self {
        if !self.root.contains_key(key) { return self.clone(); }
        let mut root = self.root.as_ref().clone();
        root.remove(key);
        Self { root: Arc::new(root), revision: self.revision.wrapping_add(1) }
    }
    pub fn iter_json(&self) -> Vec<(String, SharedUtf8)> { self.root.iter().map(|(key, value)| (key.clone(), value.id.clone())).collect() }
    pub fn iter_chunks(&self) -> Vec<(String, Arc<Vec<SharedUtf8>>)> { self.root.iter().map(|(key, value)| (key.clone(), value.chunks.clone())).collect() }
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
                chunks.push(SharedUtf8::from(chunk));
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
                chunks.push(SharedUtf8::from(chunk.clone()));
            }
            let content_id = try_value_content_id(&chunks);
            values = values.with_chunks(key, content_id, Arc::new(chunks));
        }
        Ok(values)
    }
}

/// 🧾️ One answer retires its content id and leases its shared chunk list, so aliases of an unchanged answer never wait on each other.
impl RetireOwned for TryValueContent {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![deferred(self.id), deferred(SharedControlledRetirement::lease(self.chunks))])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        sequence_birth_bytes(&[deferred_birth_bytes_for(&self.id), deferred_birth_bytes::<SharedControlledRetirement<Vec<SharedUtf8>>>()])
    }
    fn controlled_retirement_supported() -> bool { SharedUtf8::controlled_retirement_supported() && <SharedControlledRetirement<Vec<SharedUtf8>> as RetireOwned>::controlled_retirement_supported() }
}

/// 🌳️ The answer tree leases its shared root: only the last alias retires the entries, and every earlier alias releases its count.
impl RetireOwned for FormsTryValues {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![deferred(SharedControlledRetirement::lease(self.root)), deferred(self.revision)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        sequence_birth_bytes(&[deferred_birth_bytes::<SharedControlledRetirement<BTreeMap<String, TryValueContent>>>(), deferred_birth_bytes_for(&self.revision)])
    }
    fn controlled_retirement_supported() -> bool { <SharedControlledRetirement<BTreeMap<String, TryValueContent>> as RetireOwned>::controlled_retirement_supported() }
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

//#endregion 🪢️TaxonomyMounts

//#region 🔖️Owner
semio_framework_plugin::transient_root! {
    state: FormsTryWindowTransient,
    mutation: FormsTryWindowTransientMutation,
    diff: FormsTryWindowTransientDiff,
    owner: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Forms Try Window Transient",
    payload_schema: "forms.try-window-transient",
    envelope: "s.forms.forms.try-window-transient",
    extension: "formstrywindowtransient",
    fields: { try_values: FormsTryValues },
}

semio_framework_plugin::window_transient_owners! {
    state: FormsTryWindowTransient,
    mutation: FormsTryWindowTransientMutation,
    windows: {
        FormsTryWindowTransientOwner => super::FORMS_PLAY_WINDOW_TRY,
    },
}
//#endregion 🔖️Owner
