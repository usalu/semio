//! 🗂️ Keyed row sections of the ISO 16757 diff: the geometry objects, selection constraints and part number inputs rows and the script limits patch. See [`super`].

use super::*;

/// 🔑️ A `geometry.objects` entry: its map key and value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Iso16757GeometryObjectsEntry {
    pub key: String,
    pub value: semio_s_artifact_norm_iso16757_model::part_2::GeometryObject,
}

/// 🔺️ Keyed diff of the `geometry.objects` map: added entries, removed keys and modified entry patches.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757GeometryObjectsRows {
    pub added: Vec<Iso16757GeometryObjectsEntry>,
    pub removed: Vec<String>,
}

impl Iso16757GeometryObjectsRows {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }

    pub(super) fn apply_rows(&self, base: &std::collections::BTreeMap<String, semio_s_artifact_norm_iso16757_model::part_2::GeometryObject>) -> Result<std::collections::BTreeMap<String, semio_s_artifact_norm_iso16757_model::part_2::GeometryObject>, protocol::MutationApplyError> {
        let mut map = base.clone();
        for key in &self.removed {
            map.remove(key).ok_or_else(|| missing_target(format!("removed entry \"{key}\"")).at([key.clone()]))?;
        }
        for entry in &self.added {
            if map.insert(entry.key.clone(), entry.value.clone()).is_some() {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-id", format!("entry \"{}\" already exists", entry.key)).at([entry.key.clone()]));
            }
        }
        Ok(map)
    }

    pub(super) fn absorb_rows(&mut self, other: Self) {
        for key in other.removed {
            if let Some(at) = self.added.iter().position(|entry| entry.key == key) {
                self.added.remove(at);
            } else {
                if !self.removed.contains(&key) {
                    self.removed.push(key);
                }
            }
        }
        self.added.extend(other.added);
    }

    pub(super) fn inverse_rows(&self, base: &std::collections::BTreeMap<String, semio_s_artifact_norm_iso16757_model::part_2::GeometryObject>) -> Self {
        Self {
            removed: self.added.iter().map(|entry| entry.key.clone()).collect(),
            added: self.removed.iter().filter_map(|key| base.get(key).map(|value| Iso16757GeometryObjectsEntry { key: key.clone(), value: value.clone() })).collect(),
        }
    }

}

/// 📌️ A `selection.constraints` row inserted at final position `index`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Iso16757SelectionConstraintsInserted {
    pub index: usize,
    pub row: semio_s_artifact_norm_iso16757_model::part_1::SelectionConstraint,
}

/// 🔺️ Positional diff of `selection.constraints` rows: removed base positions, inserted rows at final positions and modified base-position patches, each strictly ascending.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757SelectionConstraintsRows {
    pub removed: Vec<usize>,
    pub inserted: Vec<Iso16757SelectionConstraintsInserted>,
}

impl Iso16757SelectionConstraintsRows {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty()
    }

    fn strictly_ascending(indices: impl Iterator<Item = usize>) -> bool {
        let mut previous: Option<usize> = None;
        for index in indices {
            if previous.is_some_and(|before| before >= index) {
                return false;
            }
            previous = Some(index);
        }
        true
    }

    pub(super) fn apply_rows(&self, base: &[semio_s_artifact_norm_iso16757_model::part_1::SelectionConstraint]) -> Result<Vec<semio_s_artifact_norm_iso16757_model::part_1::SelectionConstraint>, protocol::MutationApplyError> {
        let out_of_range = |index: usize| protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("row position {index} is out of range")).at([index.to_string()]);
        if !(Self::strictly_ascending(self.removed.iter().copied()) && Self::strictly_ascending(self.inserted.iter().map(|inserted| inserted.index))) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index-order", "row positions must be strictly ascending"));
        }
        if let Some(index) = self.removed.iter().find(|index| **index >= base.len()) {
            return Err(out_of_range(*index));
        }
        let mut rows = Vec::with_capacity(base.len());
        for (index, row) in base.iter().enumerate() {
            if self.removed.binary_search(&index).is_ok() {
                continue;
            }
            rows.push(row.clone());
        }
        for inserted in &self.inserted {
            if inserted.index > rows.len() {
                return Err(out_of_range(inserted.index));
            }
            rows.insert(inserted.index, inserted.row.clone());
        }
        Ok(rows)
    }

    pub(super) fn inverse_rows(&self, base: &[semio_s_artifact_norm_iso16757_model::part_1::SelectionConstraint]) -> Self {
        let _final_index = |index: usize| {
            let mut position = index - self.removed.iter().filter(|removed| **removed < index).count();
            for inserted in &self.inserted {
                if inserted.index <= position {
                    position += 1;
                }
            }
            position
        };
        Self {
            removed: self.inserted.iter().map(|inserted| inserted.index).collect(),
            inserted: self.removed.iter().filter_map(|index| base.get(*index).map(|row| Iso16757SelectionConstraintsInserted { index: *index, row: row.clone() })).collect(),
        }
    }

    pub(super) fn absorb_rows(&mut self, other: Self) {
        let origin_base = |position: usize| -> Option<usize> {
            if self.inserted.iter().any(|inserted| inserted.index == position) {
                return None;
            }
            let mut base_index = position - self.inserted.iter().filter(|inserted| inserted.index < position).count();
            for removed in &self.removed {
                if *removed <= base_index {
                    base_index += 1;
                }
            }
            Some(base_index)
        };
        let mut dropped: Vec<usize> = Vec::new();
        let mut removed_base: Vec<usize> = Vec::new();
        for position in &other.removed {
            match origin_base(*position) {
                Some(base_index) => removed_base.push(base_index),
                None => dropped.push(*position),
            }
        }
        let mut survivors: Vec<Iso16757SelectionConstraintsInserted> = Vec::new();
        for inserted in std::mem::take(&mut self.inserted) {
            if dropped.contains(&inserted.index) {
                continue;
            }
            let mut position = inserted.index - other.removed.iter().filter(|removed| **removed < inserted.index).count();
            for placed in &other.inserted {
                if placed.index <= position {
                    position += 1;
                }
            }
            survivors.push(Iso16757SelectionConstraintsInserted { index: position, row: inserted.row });
        }
        survivors.extend(other.inserted);
        survivors.sort_by_key(|inserted| inserted.index);
        self.inserted = survivors;
        self.removed.extend(removed_base);
        self.removed.sort_unstable();
        self.removed.dedup();
    }

}

/// 🔑️ A `part_number_inputs` entry: its map key and value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Iso16757PartNumberInputsEntry {
    pub key: String,
    pub value: semio_s_artifact_norm_iso16757_model::CatalogueValue,
}

/// 🔺️ Keyed diff of the `part_number_inputs` map: added entries, removed keys and modified values.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757PartNumberInputsRows {
    pub added: Vec<Iso16757PartNumberInputsEntry>,
    pub removed: Vec<String>,
    pub modified: Vec<Iso16757PartNumberInputsEntry>,
}

impl Iso16757PartNumberInputsRows {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }

    pub(super) fn apply_rows(&self, base: &std::collections::BTreeMap<String, semio_s_artifact_norm_iso16757_model::CatalogueValue>) -> Result<std::collections::BTreeMap<String, semio_s_artifact_norm_iso16757_model::CatalogueValue>, protocol::MutationApplyError> {
        let mut map = base.clone();
        for key in &self.removed {
            map.remove(key).ok_or_else(|| missing_target(format!("removed entry \"{key}\"")).at([key.clone()]))?;
        }
        for entry in &self.modified {
            let value = map.get_mut(&entry.key).ok_or_else(|| missing_target(format!("modified entry \"{}\"", entry.key)).at([entry.key.clone()]))?;
            *value = entry.value.clone();
        }
        for entry in &self.added {
            if map.insert(entry.key.clone(), entry.value.clone()).is_some() {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-id", format!("entry \"{}\" already exists", entry.key)).at([entry.key.clone()]));
            }
        }
        Ok(map)
    }

    pub(super) fn absorb_rows(&mut self, other: Self) {
        for key in other.removed {
            if let Some(at) = self.added.iter().position(|entry| entry.key == key) {
                self.added.remove(at);
            } else {
                self.modified.retain(|entry| entry.key != key);
                if !self.removed.contains(&key) {
                    self.removed.push(key);
                }
            }
        }
        self.added.extend(other.added);
        for entry in other.modified {
            if let Some(at) = self.added.iter().position(|added| added.key == entry.key) {
                self.added[at].value = entry.value;
            } else if let Some(existing) = self.modified.iter_mut().find(|existing| existing.key == entry.key) {
                existing.value = entry.value;
            } else {
                self.modified.push(entry);
            }
        }
    }

    pub(super) fn inverse_rows(&self, base: &std::collections::BTreeMap<String, semio_s_artifact_norm_iso16757_model::CatalogueValue>) -> Self {
        Self {
            removed: self.added.iter().map(|entry| entry.key.clone()).collect(),
            added: self.removed.iter().filter_map(|key| base.get(key).map(|value| Iso16757PartNumberInputsEntry { key: key.clone(), value: value.clone() })).collect(),
            modified: self.modified.iter().filter_map(|entry| base.get(&entry.key).map(|value| Iso16757PartNumberInputsEntry { key: entry.key.clone(), value: value.clone() })).collect(),
        }
    }

}

/// 🩹️ Sparse per-field patch of the `script_limits` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ScriptLimitsPatch {
    pub max_steps: Option<u32>,
    pub max_recursion: Option<u32>,
    pub timeout_ms: Option<u64>,
}

impl Iso16757ScriptLimitsPatch {
    pub(super) fn apply_to_row(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_5::ScriptLimits) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.max_steps {
            row.max_steps = value.clone();
        }
        if let Some(value) = &self.max_recursion {
            row.max_recursion = value.clone();
        }
        if let Some(value) = &self.timeout_ms {
            row.timeout_ms = value.clone();
        }
        Ok(())
    }

    pub(super) fn merge(&mut self, other: Self) {
        if other.max_steps.is_some() {
            self.max_steps = other.max_steps;
        }
        if other.max_recursion.is_some() {
            self.max_recursion = other.max_recursion;
        }
        if other.timeout_ms.is_some() {
            self.timeout_ms = other.timeout_ms;
        }
    }

    pub(super) fn inverse_from_row(&self, row: &semio_s_artifact_norm_iso16757_model::part_5::ScriptLimits) -> Self {
        Self {
            max_steps: self.max_steps.as_ref().map(|_| row.max_steps.clone()),
            max_recursion: self.max_recursion.as_ref().map(|_| row.max_recursion.clone()),
            timeout_ms: self.timeout_ms.as_ref().map(|_| row.timeout_ms.clone()),
        }
    }
}
