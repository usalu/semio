//! 📍️ Identified GIS features and their domain payload patch.

use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use protocol::{Identified, Patchable};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Types
/// 🗺️ One id-keyed spatial feature carried as its full opaque descriptor payload.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MapFeature {
    #[dsl(positional)]
    pub id: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch.
    pub data: semio_framework_value::DslValue,
}

impl Identified<String> for MapFeature {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Payload patch: an optional whole-payload replacement, then ordered single-property edits. Inverts to the prior
/// payload (replacement) or to the reversed per-property undo (edits).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MapFeaturePatch {
    pub data: Option<semio_framework_value::DslValue>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub properties: Vec<MapFeaturePropertyEdit>,
}

/// ✏️ One edit of a single payload property: sets `key` (in place when present, otherwise inserted before the `before`
/// key, appended without one) or, when `set` is absent, removes it.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MapFeaturePropertyEdit {
    pub key: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub before: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub set: Option<MapFeaturePropertyValue>,
}

/// 🧱️ Explicit wrapper so a property set to `null` survives the wire (a bare nested option collapses).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MapFeaturePropertyValue {
    pub value: semio_framework_value::DslValue,
}

fn apply_property_edit(entries: &mut Vec<(String, semio_framework_value::DslValue)>, edit: &MapFeaturePropertyEdit) {
    let position = entries.iter().position(|(key, _)| *key == edit.key);
    match (&edit.set, position) {
        (Some(set), Some(at)) => entries[at].1 = set.value.clone(),
        (Some(set), None) => {
            let at = edit.before.as_ref().and_then(|before| entries.iter().position(|(key, _)| key == before)).unwrap_or(entries.len());
            entries.insert(at, (edit.key.clone(), set.value.clone()));
        }
        (None, Some(at)) => {
            entries.remove(at);
        }
        (None, None) => {}
    }
}

/// ✏️ Applies ordered property edits to an object payload; a payload that is not an object is left untouched.
pub fn apply_property_edits(data: &mut semio_framework_value::DslValue, edits: &[MapFeaturePropertyEdit]) {
    if edits.is_empty() {
        return;
    }
    if let semio_framework_value::DslValue::Object(entries) = data {
        for edit in edits {
            apply_property_edit(entries, edit);
        }
    }
}

/// ↩️ The edits that undo `edits` applied to `data`, in application order (each edit restores the value or position its
/// own application displaced).
pub fn invert_property_edits(data: &semio_framework_value::DslValue, edits: &[MapFeaturePropertyEdit]) -> Vec<MapFeaturePropertyEdit> {
    let mut state: Vec<(String, semio_framework_value::DslValue)> = data.as_object().map(<[(String, semio_framework_value::DslValue)]>::to_vec).unwrap_or_default();
    let mut inverse = Vec::new();
    for edit in edits {
        let position = state.iter().position(|(key, _)| *key == edit.key);
        match (&edit.set, position) {
            (Some(_), Some(at)) => inverse.push(MapFeaturePropertyEdit { key: edit.key.clone(), before: None, set: Some(MapFeaturePropertyValue { value: state[at].1.clone() }) }),
            (Some(_), None) => inverse.push(MapFeaturePropertyEdit { key: edit.key.clone(), before: None, set: None }),
            (None, Some(at)) => inverse.push(MapFeaturePropertyEdit { key: edit.key.clone(), before: state.get(at + 1).map(|(key, _)| key.clone()), set: Some(MapFeaturePropertyValue { value: state[at].1.clone() }) }),
            (None, None) => {}
        }
        apply_property_edit(&mut state, edit);
    }
    inverse.reverse();
    inverse
}

impl Patchable<MapFeaturePatch> for MapFeature {
    fn apply_patch(&mut self, patch: &MapFeaturePatch) {
        if let Some(data) = &patch.data {
            self.data = data.clone();
        }
        apply_property_edits(&mut self.data, &patch.properties);
    }
}

//#endregion 🔹Types
