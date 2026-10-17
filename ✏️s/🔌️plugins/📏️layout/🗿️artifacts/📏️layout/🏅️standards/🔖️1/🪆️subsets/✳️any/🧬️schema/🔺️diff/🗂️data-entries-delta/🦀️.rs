//! 🗂️ Data entries delta of the layout diff: the keyed data entry rows and their removal, insertion and relocation records. See [`super`].

use super::*;

//#region 🔖️DeltaHelpers
/// 🧾️ One dictionary row keyed by its question; the forms crate owns the entry type, so the delta keys it through this wrapper.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct LayoutDataEntryRow(pub(super) semio_framework_value_dictionary::QuestionDictionaryEntry);

impl protocol::list_delta::Keyed for LayoutDataEntryRow {
    type Key = String;
    fn key(&self) -> String {
        self.0.question_id.clone()
    }
}

/// ➖️ One `entries` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LayoutDataEntryRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `entries` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LayoutDataEntryInsertion {
    pub index: usize,
    pub row: semio_framework_value_dictionary::QuestionDictionaryEntry,
}

/// ↕️ One `entries` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LayoutDataEntryRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🧩 Positional keyed rows of the `entries` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct LayoutDataEntriesDelta {
    pub removed: Vec<LayoutDataEntryRemoval>,
    pub inserted: Vec<LayoutDataEntryInsertion>,
    pub moved: Vec<LayoutDataEntryRelocation>,
}

type LayoutDataEntriesDeltaParts = protocol::list_delta::Parts<LayoutDataEntryRow, protocol::list_delta::NoPatch>;

impl LayoutDataEntriesDelta {
    pub(super) fn into_parts(self) -> LayoutDataEntriesDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, LayoutDataEntryRow(entry.row))).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: Vec::new(),
        }
    }

    pub(super) fn from_parts(parts: LayoutDataEntriesDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| LayoutDataEntryRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| LayoutDataEntryInsertion { index, row: row.0 }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| LayoutDataEntryRelocation { id, from, to }).collect(),
        }
    }

    /// ➕️ The delta that inserts `row` at `index` of the after list.
    pub fn insertion(index: usize, row: semio_framework_value_dictionary::QuestionDictionaryEntry) -> Self {
        Self::from_parts(protocol::list_delta::Parts::insertion(index, LayoutDataEntryRow(row)))
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
    pub fn relocation_by_id(id: impl Into<String>, from: usize, to: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::relocation_by_id(id.into(), from, to))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &[semio_framework_value_dictionary::QuestionDictionaryEntry], capability: ApplyCapability) -> Result<Vec<semio_framework_value_dictionary::QuestionDictionaryEntry>, protocol::list_delta::ApplyError> {
        let rows = base.iter().cloned().map(LayoutDataEntryRow).collect::<Vec<LayoutDataEntryRow>>();
        Ok(self.clone().into_parts().commit_onto(&rows, capability)?.into_iter().map(|row| row.0).collect())
    }

    /// 🔁️ The negative delta over `base`, read row by row.
    pub fn inverse(&self, base: &[semio_framework_value_dictionary::QuestionDictionaryEntry]) -> Self {
        let rows = base.iter().cloned().map(LayoutDataEntryRow).collect::<Vec<LayoutDataEntryRow>>();
        Self::from_parts(self.clone().into_parts().inverse(&rows))
    }

    /// ➕️ Composes `self` with the delta `later` applied after it.
    pub fn absorb(&mut self, later: Self) {
        let mut parts = std::mem::take(self).into_parts();
        parts.absorb(later.into_parts());
        *self = Self::from_parts(parts);
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty()
    }
}
//#endregion 🔖️DeltaHelpers
