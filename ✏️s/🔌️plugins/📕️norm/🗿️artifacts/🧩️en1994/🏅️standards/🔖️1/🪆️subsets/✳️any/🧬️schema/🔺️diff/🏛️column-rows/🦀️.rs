//! 🏛️ Column sections of the EN 1994 diff: the column action rows and the keyed column row patches. See [`super`].

use super::*;

/// 🩹️ Sparse patch of the `actions` row at base position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994ColumnsActionsPatch {
    pub index: usize,
    pub n_k_n: Option<f64>,
}

impl En1994ColumnsActionsPatch {
    pub(super) fn apply_to_row(&self, row: &mut semio_s_artifact_norm_en1994_model::ColumnAction) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.n_k_n {
            row.n_k_n = value.clone();
        }
        Ok(())
    }

    pub(super) fn merge(&mut self, other: Self) {
        if other.n_k_n.is_some() {
            self.n_k_n = other.n_k_n;
        }
    }

    pub(super) fn inverse_from_row(&self, row: &semio_s_artifact_norm_en1994_model::ColumnAction, index: usize) -> Self {
        Self {
            index,
            n_k_n: self.n_k_n.as_ref().map(|_| row.n_k_n.clone()),
        }
    }
}

/// 🔺️ Positional diff of `actions` rows: removed base positions, inserted rows at final positions and modified base-position patches, each strictly ascending.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994ColumnsActionsRows {
    pub modified: Vec<En1994ColumnsActionsPatch>,
}

impl En1994ColumnsActionsRows {
    pub fn is_empty(&self) -> bool {
        self.modified.is_empty()
    }

    pub(super) fn strictly_ascending(indices: impl Iterator<Item = usize>) -> bool {
        let mut previous: Option<usize> = None;
        for index in indices {
            if previous.is_some_and(|before| before >= index) {
                return false;
            }
            previous = Some(index);
        }
        true
    }

    pub(super) fn apply_rows(&self, base: &[semio_s_artifact_norm_en1994_model::ColumnAction]) -> Result<Vec<semio_s_artifact_norm_en1994_model::ColumnAction>, protocol::MutationApplyError> {
        let out_of_range = |index: usize| protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("row position {index} is out of range")).at([index.to_string()]);
        if !(Self::strictly_ascending(self.modified.iter().map(|patch| patch.index))) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index-order", "row positions must be strictly ascending"));
        }
        if let Some(patch) = self.modified.iter().find(|patch| patch.index >= base.len()) {
            return Err(out_of_range(patch.index));
        }
        let mut rows = Vec::with_capacity(base.len());
        for (index, row) in base.iter().enumerate() {
            let mut row = row.clone();
            if let Ok(at) = self.modified.binary_search_by_key(&index, |patch| patch.index) {
                self.modified[at].apply_to_row(&mut row)?;
            }
            rows.push(row);
        }
        Ok(rows)
    }

    pub(super) fn inverse_rows(&self, base: &[semio_s_artifact_norm_en1994_model::ColumnAction]) -> Self {
        Self {
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, patch.index))).collect(),
        }
    }

    pub(super) fn absorb_rows(&mut self, other: Self) {
        for patch in other.modified {
            match self.modified.iter_mut().find(|existing| existing.index == patch.index) {
                Some(existing) => existing.merge(patch),
                None => self.modified.push(patch),
            }
        }
        self.modified.sort_by_key(|patch| patch.index);
    }

}

/// 🩹️ Sparse patch of the `columns` row at base position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994ColumnsPatch {
    pub index: usize,
    pub kind: Option<String>,
    pub actions: Option<En1994ColumnsActionsRows>,
}

impl En1994ColumnsPatch {
    pub(super) fn apply_to_row(&self, row: &mut semio_s_artifact_norm_en1994_model::CompositeColumn) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind {
            row.kind = value.clone();
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.apply_rows(&row.actions).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    pub(super) fn merge(&mut self, other: Self) {
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if let Some(theirs) = other.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    pub(super) fn inverse_from_row(&self, row: &semio_s_artifact_norm_en1994_model::CompositeColumn, index: usize) -> Self {
        Self {
            index,
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse_rows(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }
}

/// 📌️ A `columns` row inserted at final position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994ColumnsInserted {
    pub index: usize,
    pub row: semio_s_artifact_norm_en1994_model::CompositeColumn,
}

/// 🔺️ Positional diff of `columns` rows: removed base positions, inserted rows at final positions and modified base-position patches, each strictly ascending.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994ColumnsRows {
    pub removed: Vec<usize>,
    pub inserted: Vec<En1994ColumnsInserted>,
    pub modified: Vec<En1994ColumnsPatch>,
}

impl En1994ColumnsRows {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.modified.is_empty()
    }

    pub(super) fn strictly_ascending(indices: impl Iterator<Item = usize>) -> bool {
        let mut previous: Option<usize> = None;
        for index in indices {
            if previous.is_some_and(|before| before >= index) {
                return false;
            }
            previous = Some(index);
        }
        true
    }

    pub(super) fn apply_rows(&self, base: &[semio_s_artifact_norm_en1994_model::CompositeColumn]) -> Result<Vec<semio_s_artifact_norm_en1994_model::CompositeColumn>, protocol::MutationApplyError> {
        let out_of_range = |index: usize| protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("row position {index} is out of range")).at([index.to_string()]);
        if !(Self::strictly_ascending(self.removed.iter().copied()) && Self::strictly_ascending(self.inserted.iter().map(|inserted| inserted.index)) && Self::strictly_ascending(self.modified.iter().map(|patch| patch.index))) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index-order", "row positions must be strictly ascending"));
        }
        if let Some(index) = self.removed.iter().find(|index| **index >= base.len()) {
            return Err(out_of_range(*index));
        }
        if let Some(patch) = self.modified.iter().find(|patch| patch.index >= base.len()) {
            return Err(out_of_range(patch.index));
        }
        if let Some(patch) = self.modified.iter().find(|patch| self.removed.binary_search(&patch.index).is_ok()) {
            return Err(missing_target(format!("modified row {} is removed by the same diff", patch.index)).at([patch.index.to_string()]));
        }
        let mut rows = Vec::with_capacity(base.len());
        for (index, row) in base.iter().enumerate() {
            if self.removed.binary_search(&index).is_ok() {
                continue;
            }
            let mut row = row.clone();
            if let Ok(at) = self.modified.binary_search_by_key(&index, |patch| patch.index) {
                self.modified[at].apply_to_row(&mut row)?;
            }
            rows.push(row);
        }
        for inserted in &self.inserted {
            if inserted.index > rows.len() {
                return Err(out_of_range(inserted.index));
            }
            rows.insert(inserted.index, inserted.row.clone());
        }
        Ok(rows)
    }

    pub(super) fn inverse_rows(&self, base: &[semio_s_artifact_norm_en1994_model::CompositeColumn]) -> Self {
        let final_index = |index: usize| {
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
            inserted: self.removed.iter().filter_map(|index| base.get(*index).map(|row| En1994ColumnsInserted { index: *index, row: row.clone() })).collect(),
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, final_index(patch.index)))).collect(),
        }
    }

    pub(super) fn absorb_rows(&mut self, other: Self) {
        enum Origin {
            New(usize),
            Base(usize),
        }
        let origin = |position: usize| -> Origin {
            if let Some(at) = self.inserted.iter().position(|inserted| inserted.index == position) {
                return Origin::New(at);
            }
            let mut base_index = position - self.inserted.iter().filter(|inserted| inserted.index < position).count();
            for removed in &self.removed {
                if *removed <= base_index {
                    base_index += 1;
                }
            }
            Origin::Base(base_index)
        };
        let removed_origins: Vec<Origin> = other.removed.iter().map(|position| origin(*position)).collect();
        let patch_origins: Vec<Origin> = other.modified.iter().map(|patch| origin(patch.index)).collect();
        let mut dropped = vec![false; self.inserted.len()];
        let mut removed_base: Vec<usize> = Vec::new();
        for found in removed_origins {
            match found {
                Origin::New(at) => dropped[at] = true,
                Origin::Base(base_index) => removed_base.push(base_index),
            }
        }
        for (found, mut patch) in patch_origins.into_iter().zip(other.modified) {
            match found {
                Origin::New(at) => {
                    let _ = patch.apply_to_row(&mut self.inserted[at].row);
                }
                Origin::Base(base_index) => {
                    patch.index = base_index;
                    match self.modified.iter_mut().find(|existing| existing.index == base_index) {
                        Some(existing) => existing.merge(patch),
                        None => self.modified.push(patch),
                    }
                }
            }
        }
        let mut survivors: Vec<En1994ColumnsInserted> = Vec::new();
        for (at, inserted) in std::mem::take(&mut self.inserted).into_iter().enumerate() {
            if dropped[at] {
                continue;
            }
            let mut position = inserted.index - other.removed.iter().filter(|removed| **removed < inserted.index).count();
            for placed in &other.inserted {
                if placed.index <= position {
                    position += 1;
                }
            }
            survivors.push(En1994ColumnsInserted { index: position, row: inserted.row });
        }
        survivors.extend(other.inserted);
        survivors.sort_by_key(|inserted| inserted.index);
        self.inserted = survivors;
        self.removed.extend(removed_base);
        self.removed.sort_unstable();
        self.removed.dedup();
        let removed = self.removed.clone();
        self.modified.retain(|patch| removed.binary_search(&patch.index).is_err());
        self.modified.sort_by_key(|patch| patch.index);
    }

}
