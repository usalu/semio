//! 🧬️ En1994 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

use crate::En1994Snapshot;

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `actions` row at base position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994BeamsActionsPatch {
    pub index: usize,
    pub q_area_pa: Option<f64>,
}

impl En1994BeamsActionsPatch {
    fn apply_to_row(&self, row: &mut crate::CharacteristicAction) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.q_area_pa {
            row.q_area_pa = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.q_area_pa.is_some() {
            self.q_area_pa = other.q_area_pa;
        }
    }

    fn inverse_from_row(&self, row: &crate::CharacteristicAction, index: usize) -> Self {
        Self {
            index,
            q_area_pa: self.q_area_pa.as_ref().map(|_| row.q_area_pa.clone()),
        }
    }
}

/// 🔺️ Positional diff of `actions` rows: removed base positions, inserted rows at final positions and modified base-position patches, each strictly ascending.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994BeamsActionsRows {
    pub modified: Vec<En1994BeamsActionsPatch>,
}

impl En1994BeamsActionsRows {
    pub fn is_empty(&self) -> bool {
        self.modified.is_empty()
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

    fn apply_rows(&self, base: &[crate::CharacteristicAction]) -> Result<Vec<crate::CharacteristicAction>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::CharacteristicAction]) -> Self {
        Self {
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, patch.index))).collect(),
        }
    }

    fn absorb_rows(&mut self, other: Self) {
        for patch in other.modified {
            match self.modified.iter_mut().find(|existing| existing.index == patch.index) {
                Some(existing) => existing.merge(patch),
                None => self.modified.push(patch),
            }
        }
        self.modified.sort_by_key(|patch| patch.index);
    }

}

/// 🩹️ Sparse patch of the `beams` row at base position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994BeamsPatch {
    pub index: usize,
    pub span_m: Option<f64>,
    pub slab_thickness_m: Option<f64>,
    pub construction: Option<String>,
    pub transverse_as_m2_per_m: Option<f64>,
    pub studs_diameter_m: Option<f64>,
    pub studs_f_u_pa: Option<f64>,
    pub studs_spacing_m: Option<f64>,
    pub studs_total_count: Option<u32>,
    pub actions: Option<En1994BeamsActionsRows>,
}

impl En1994BeamsPatch {
    fn apply_to_row(&self, row: &mut crate::CompositeBeam) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.span_m {
            row.span_m = value.clone();
        }
        if let Some(value) = &self.slab_thickness_m {
            row.slab_thickness_m = value.clone();
        }
        if let Some(value) = &self.construction {
            row.construction = value.clone();
        }
        if let Some(value) = &self.transverse_as_m2_per_m {
            row.transverse_as_m2_per_m = value.clone();
        }
        if let Some(value) = &self.studs_diameter_m {
            row.studs.diameter_m = value.clone();
        }
        if let Some(value) = &self.studs_f_u_pa {
            row.studs.f_u_pa = value.clone();
        }
        if let Some(value) = &self.studs_spacing_m {
            row.studs.spacing_m = value.clone();
        }
        if let Some(value) = &self.studs_total_count {
            row.studs.total_count = value.clone();
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.apply_rows(&row.actions).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.span_m.is_some() {
            self.span_m = other.span_m;
        }
        if other.slab_thickness_m.is_some() {
            self.slab_thickness_m = other.slab_thickness_m;
        }
        if other.construction.is_some() {
            self.construction = other.construction;
        }
        if other.transverse_as_m2_per_m.is_some() {
            self.transverse_as_m2_per_m = other.transverse_as_m2_per_m;
        }
        if other.studs_diameter_m.is_some() {
            self.studs_diameter_m = other.studs_diameter_m;
        }
        if other.studs_f_u_pa.is_some() {
            self.studs_f_u_pa = other.studs_f_u_pa;
        }
        if other.studs_spacing_m.is_some() {
            self.studs_spacing_m = other.studs_spacing_m;
        }
        if other.studs_total_count.is_some() {
            self.studs_total_count = other.studs_total_count;
        }
        if let Some(theirs) = other.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse_from_row(&self, row: &crate::CompositeBeam, index: usize) -> Self {
        Self {
            index,
            span_m: self.span_m.as_ref().map(|_| row.span_m.clone()),
            slab_thickness_m: self.slab_thickness_m.as_ref().map(|_| row.slab_thickness_m.clone()),
            construction: self.construction.as_ref().map(|_| row.construction.clone()),
            transverse_as_m2_per_m: self.transverse_as_m2_per_m.as_ref().map(|_| row.transverse_as_m2_per_m.clone()),
            studs_diameter_m: self.studs_diameter_m.as_ref().map(|_| row.studs.diameter_m.clone()),
            studs_f_u_pa: self.studs_f_u_pa.as_ref().map(|_| row.studs.f_u_pa.clone()),
            studs_spacing_m: self.studs_spacing_m.as_ref().map(|_| row.studs.spacing_m.clone()),
            studs_total_count: self.studs_total_count.as_ref().map(|_| row.studs.total_count.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse_rows(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }
}

/// 📌️ A `beams` row inserted at final position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994BeamsInserted {
    pub index: usize,
    pub row: crate::CompositeBeam,
}

/// 🔺️ Positional diff of `beams` rows: removed base positions, inserted rows at final positions and modified base-position patches, each strictly ascending.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994BeamsRows {
    pub removed: Vec<usize>,
    pub inserted: Vec<En1994BeamsInserted>,
    pub modified: Vec<En1994BeamsPatch>,
}

impl En1994BeamsRows {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.modified.is_empty()
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

    fn apply_rows(&self, base: &[crate::CompositeBeam]) -> Result<Vec<crate::CompositeBeam>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::CompositeBeam]) -> Self {
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
            inserted: self.removed.iter().filter_map(|index| base.get(*index).map(|row| En1994BeamsInserted { index: *index, row: row.clone() })).collect(),
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, final_index(patch.index)))).collect(),
        }
    }

    fn absorb_rows(&mut self, other: Self) {
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
        let mut survivors: Vec<En1994BeamsInserted> = Vec::new();
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
            survivors.push(En1994BeamsInserted { index: position, row: inserted.row });
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
    fn apply_to_row(&self, row: &mut crate::ColumnAction) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.n_k_n {
            row.n_k_n = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.n_k_n.is_some() {
            self.n_k_n = other.n_k_n;
        }
    }

    fn inverse_from_row(&self, row: &crate::ColumnAction, index: usize) -> Self {
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

    fn apply_rows(&self, base: &[crate::ColumnAction]) -> Result<Vec<crate::ColumnAction>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::ColumnAction]) -> Self {
        Self {
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, patch.index))).collect(),
        }
    }

    fn absorb_rows(&mut self, other: Self) {
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
    fn apply_to_row(&self, row: &mut crate::CompositeColumn) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind {
            row.kind = value.clone();
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.apply_rows(&row.actions).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
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

    fn inverse_from_row(&self, row: &crate::CompositeColumn, index: usize) -> Self {
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
    pub row: crate::CompositeColumn,
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

    fn apply_rows(&self, base: &[crate::CompositeColumn]) -> Result<Vec<crate::CompositeColumn>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::CompositeColumn]) -> Self {
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

    fn absorb_rows(&mut self, other: Self) {
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

/// 🩹️ Sparse patch of the `actions` row at base position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994SlabsActionsPatch {
    pub index: usize,
    pub q_area_pa: Option<f64>,
}

impl En1994SlabsActionsPatch {
    fn apply_to_row(&self, row: &mut crate::CharacteristicAction) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.q_area_pa {
            row.q_area_pa = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.q_area_pa.is_some() {
            self.q_area_pa = other.q_area_pa;
        }
    }

    fn inverse_from_row(&self, row: &crate::CharacteristicAction, index: usize) -> Self {
        Self {
            index,
            q_area_pa: self.q_area_pa.as_ref().map(|_| row.q_area_pa.clone()),
        }
    }
}

/// 🔺️ Positional diff of `actions` rows: removed base positions, inserted rows at final positions and modified base-position patches, each strictly ascending.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994SlabsActionsRows {
    pub modified: Vec<En1994SlabsActionsPatch>,
}

impl En1994SlabsActionsRows {
    pub fn is_empty(&self) -> bool {
        self.modified.is_empty()
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

    fn apply_rows(&self, base: &[crate::CharacteristicAction]) -> Result<Vec<crate::CharacteristicAction>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::CharacteristicAction]) -> Self {
        Self {
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, patch.index))).collect(),
        }
    }

    fn absorb_rows(&mut self, other: Self) {
        for patch in other.modified {
            match self.modified.iter_mut().find(|existing| existing.index == patch.index) {
                Some(existing) => existing.merge(patch),
                None => self.modified.push(patch),
            }
        }
        self.modified.sort_by_key(|patch| patch.index);
    }

}

/// 🩹️ Sparse patch of the `slabs` row at base position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994SlabsPatch {
    pub index: usize,
    pub concrete_thickness_m: Option<f64>,
    pub actions: Option<En1994SlabsActionsRows>,
}

impl En1994SlabsPatch {
    fn apply_to_row(&self, row: &mut crate::CompositeSlab) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.concrete_thickness_m {
            row.concrete_thickness_m = value.clone();
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.apply_rows(&row.actions).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.concrete_thickness_m.is_some() {
            self.concrete_thickness_m = other.concrete_thickness_m;
        }
        if let Some(theirs) = other.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse_from_row(&self, row: &crate::CompositeSlab, index: usize) -> Self {
        Self {
            index,
            concrete_thickness_m: self.concrete_thickness_m.as_ref().map(|_| row.concrete_thickness_m.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse_rows(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }
}

/// 📌️ A `slabs` row inserted at final position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994SlabsInserted {
    pub index: usize,
    pub row: crate::CompositeSlab,
}

/// 🔺️ Positional diff of `slabs` rows: removed base positions, inserted rows at final positions and modified base-position patches, each strictly ascending.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1994SlabsRows {
    pub removed: Vec<usize>,
    pub inserted: Vec<En1994SlabsInserted>,
    pub modified: Vec<En1994SlabsPatch>,
}

impl En1994SlabsRows {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.modified.is_empty()
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

    fn apply_rows(&self, base: &[crate::CompositeSlab]) -> Result<Vec<crate::CompositeSlab>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::CompositeSlab]) -> Self {
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
            inserted: self.removed.iter().filter_map(|index| base.get(*index).map(|row| En1994SlabsInserted { index: *index, row: row.clone() })).collect(),
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, final_index(patch.index)))).collect(),
        }
    }

    fn absorb_rows(&mut self, other: Self) {
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
        let mut survivors: Vec<En1994SlabsInserted> = Vec::new();
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
            survivors.push(En1994SlabsInserted { index: position, row: inserted.row });
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

/// 🔺️ Keyed sparse diff of the En1994 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub structure_kind: Option<String>,
    #[state(artifact)]
    pub steel_f_y_pa: Option<f64>,
    #[state(artifact)]
    pub beams: Option<En1994BeamsRows>,
    #[state(artifact)]
    pub columns: Option<En1994ColumnsRows>,
    #[state(artifact)]
    pub slabs: Option<En1994SlabsRows>,
    #[state(artifact)]
    pub fire_rating: Option<String>,
    #[state(artifact)]
    pub insulation_thickness_m: Option<f64>,
    #[state(artifact)]
    pub fatigue_detail: Option<String>,
}

impl protocol::MutationDiff<En1994Snapshot> for En1994Diff {
    fn apply(&self, base: &En1994Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1994Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.structure_kind {
            next.structure_kind = value.clone();
        }
        if let Some(value) = &self.steel_f_y_pa {
            next.steel_f_y_pa = value.clone();
        }
        if let Some(rows) = &self.beams {
            next.beams = rows.apply_rows(&base.beams).map_err(|error| error.under(["beams"]))?;
        }
        if let Some(rows) = &self.columns {
            next.columns = rows.apply_rows(&base.columns).map_err(|error| error.under(["columns"]))?;
        }
        if let Some(rows) = &self.slabs {
            next.slabs = rows.apply_rows(&base.slabs).map_err(|error| error.under(["slabs"]))?;
        }
        if let Some(value) = &self.fire_rating {
            next.fire_rating = value.clone();
        }
        if let Some(value) = &self.insulation_thickness_m {
            next.insulation_thickness_m = value.clone();
        }
        if let Some(value) = &self.fatigue_detail {
            next.fatigue_detail = value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.structure_kind.is_some() {
            self.structure_kind = other.structure_kind;
        }
        if other.steel_f_y_pa.is_some() {
            self.steel_f_y_pa = other.steel_f_y_pa;
        }
        if let Some(theirs) = other.beams {
            match self.beams.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.beams = Some(theirs),
            }
            self.beams = self.beams.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.columns {
            match self.columns.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.columns = Some(theirs),
            }
            self.columns = self.columns.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.slabs {
            match self.slabs.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.slabs = Some(theirs),
            }
            self.slabs = self.slabs.take().filter(|rows| !rows.is_empty());
        }
        if other.fire_rating.is_some() {
            self.fire_rating = other.fire_rating;
        }
        if other.insulation_thickness_m.is_some() {
            self.insulation_thickness_m = other.insulation_thickness_m;
        }
        if other.fatigue_detail.is_some() {
            self.fatigue_detail = other.fatigue_detail;
        }
    }
}

impl protocol::DiffAlgebra<En1994Snapshot> for En1994Diff {
    fn inverse(&self, base: &En1994Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            structure_kind: self.structure_kind.as_ref().map(|_| base.structure_kind.clone()),
            steel_f_y_pa: self.steel_f_y_pa.as_ref().map(|_| base.steel_f_y_pa.clone()),
            beams: self.beams.as_ref().map(|rows| rows.inverse_rows(&base.beams)).filter(|rows| !rows.is_empty()),
            columns: self.columns.as_ref().map(|rows| rows.inverse_rows(&base.columns)).filter(|rows| !rows.is_empty()),
            slabs: self.slabs.as_ref().map(|rows| rows.inverse_rows(&base.slabs)).filter(|rows| !rows.is_empty()),
            fire_rating: self.fire_rating.as_ref().map(|_| base.fire_rating.clone()),
            insulation_thickness_m: self.insulation_thickness_m.as_ref().map(|_| base.insulation_thickness_m.clone()),
            fatigue_detail: self.fatigue_detail.as_ref().map(|_| base.fatigue_detail.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.structure_kind.is_none() && self.steel_f_y_pa.is_none() && self.beams.as_ref().map_or(true, |rows| rows.is_empty()) && self.columns.as_ref().map_or(true, |rows| rows.is_empty()) && self.slabs.as_ref().map_or(true, |rows| rows.is_empty()) && self.fire_rating.is_none() && self.insulation_thickness_m.is_none() && self.fatigue_detail.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
