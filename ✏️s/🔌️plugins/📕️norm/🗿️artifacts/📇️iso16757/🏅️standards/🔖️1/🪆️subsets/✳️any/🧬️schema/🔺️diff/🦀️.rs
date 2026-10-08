//! 🧬️ Iso16757 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff. `DiffAlgebra::between` covers the modelled vocabulary only and exists for sync tooling, never for mutation leaves.

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("diff.target-missing", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `catalogue.product_groups` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductGroupsPatch {
    pub id: String,
    pub name: Option<String>,
}

impl Iso16757ProductGroupsPatch {
    fn apply_to_row(&self, row: &mut crate::part_1::ProductGroup) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.name {
            row.names.preferred.text = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
    }

    fn inverse_from_row(&self, row: &crate::part_1::ProductGroup) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.as_ref().map(|_| row.names.preferred.text.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `catalogue.product_groups` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductGroupsRows {
    pub added: Vec<crate::part_1::ProductGroup>,
    pub removed: Vec<String>,
    pub modified: Vec<Iso16757ProductGroupsPatch>,
    pub order: Option<Vec<String>>,
}

impl Iso16757ProductGroupsRows {
    fn key(row: &crate::part_1::ProductGroup) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::part_1::ProductGroup]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::part_1::ProductGroup]) -> Result<Vec<crate::part_1::ProductGroup>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for patch in &self.modified {
            let row = rows.iter_mut().find(|row| Self::key(row) == patch.id).ok_or_else(|| missing_target(format!("modified row \"{}\"", patch.id)).at([patch.id.clone()]))?;
            patch.apply_to_row(row)?;
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                self.modified.retain(|patch| patch.id != id);
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        for patch in other.modified {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == patch.id) {
                if patch.apply_to_row(&mut self.added[at]).is_err() {
                    self.modified.push(patch);
                }
            } else if let Some(existing) = self.modified.iter_mut().find(|existing| existing.id == patch.id) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::part_1::ProductGroup]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        inverse.modified = self.modified.iter().filter_map(|patch| base.iter().find(|row| Self::key(row) == patch.id).map(|row| patch.inverse_from_row(row))).collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::part_1::ProductGroup], other: &[crate::part_1::ProductGroup]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🔺️ Keyed diff of `catalogue.product_classes` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductClassesRows {
    pub added: Vec<crate::part_1::ProductClass>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl Iso16757ProductClassesRows {
    fn key(row: &crate::part_1::ProductClass) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::part_1::ProductClass]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::part_1::ProductClass]) -> Result<Vec<crate::part_1::ProductClass>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::part_1::ProductClass]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::part_1::ProductClass], other: &[crate::part_1::ProductClass]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🔺️ Keyed diff of `catalogue.product_series` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductSeriesRows {
    pub added: Vec<crate::part_1::ProductSeries>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl Iso16757ProductSeriesRows {
    fn key(row: &crate::part_1::ProductSeries) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::part_1::ProductSeries]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::part_1::ProductSeries]) -> Result<Vec<crate::part_1::ProductSeries>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::part_1::ProductSeries]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::part_1::ProductSeries], other: &[crate::part_1::ProductSeries]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🩹️ Sparse patch of the `catalogue.products` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductsPatch {
    pub id: String,
    pub name: Option<String>,
}

impl Iso16757ProductsPatch {
    fn apply_to_row(&self, row: &mut crate::part_1::Product) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.name {
            row.names.preferred.text = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
    }

    fn inverse_from_row(&self, row: &crate::part_1::Product) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.as_ref().map(|_| row.names.preferred.text.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `catalogue.products` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductsRows {
    pub added: Vec<crate::part_1::Product>,
    pub removed: Vec<String>,
    pub modified: Vec<Iso16757ProductsPatch>,
    pub order: Option<Vec<String>>,
}

impl Iso16757ProductsRows {
    fn key(row: &crate::part_1::Product) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::part_1::Product]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::part_1::Product]) -> Result<Vec<crate::part_1::Product>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for patch in &self.modified {
            let row = rows.iter_mut().find(|row| Self::key(row) == patch.id).ok_or_else(|| missing_target(format!("modified row \"{}\"", patch.id)).at([patch.id.clone()]))?;
            patch.apply_to_row(row)?;
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                self.modified.retain(|patch| patch.id != id);
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        for patch in other.modified {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == patch.id) {
                if patch.apply_to_row(&mut self.added[at]).is_err() {
                    self.modified.push(patch);
                }
            } else if let Some(existing) = self.modified.iter_mut().find(|existing| existing.id == patch.id) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::part_1::Product]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        inverse.modified = self.modified.iter().filter_map(|patch| base.iter().find(|row| Self::key(row) == patch.id).map(|row| patch.inverse_from_row(row))).collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::part_1::Product], other: &[crate::part_1::Product]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🔺️ Keyed diff of `catalogue.product_indexes` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductIndexesRows {
    pub added: Vec<crate::part_1::ProductIndex>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl Iso16757ProductIndexesRows {
    fn key(row: &crate::part_1::ProductIndex) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::part_1::ProductIndex]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::part_1::ProductIndex]) -> Result<Vec<crate::part_1::ProductIndex>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::part_1::ProductIndex]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::part_1::ProductIndex], other: &[crate::part_1::ProductIndex]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🔺️ Keyed diff of `catalogue.property_definitions` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757PropertyDefinitionsRows {
    pub added: Vec<crate::part_1::PropertyDefinition>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl Iso16757PropertyDefinitionsRows {
    fn key(row: &crate::part_1::PropertyDefinition) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::part_1::PropertyDefinition]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::part_1::PropertyDefinition]) -> Result<Vec<crate::part_1::PropertyDefinition>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::part_1::PropertyDefinition]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::part_1::PropertyDefinition], other: &[crate::part_1::PropertyDefinition]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🔺️ Keyed diff of `dictionary.subjects` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757SubjectsRows {
    pub added: Vec<crate::part_4::Subject>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl Iso16757SubjectsRows {
    fn key(row: &crate::part_4::Subject) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::part_4::Subject]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::part_4::Subject]) -> Result<Vec<crate::part_4::Subject>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for id in &self.removed {
            let at = rows.iter().position(|row| Self::key(row) == id).ok_or_else(|| missing_target(format!("removed row \"{id}\"")).at([id.clone()]))?;
            rows.remove(at);
        }
        for row in &self.added {
            if rows.iter().any(|existing| Self::key(existing) == Self::key(row)) {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("row \"{}\" already exists", Self::key(row))).at([Self::key(row).to_string()]));
            }
            rows.push(row.clone());
        }
        if let Some(order) = &self.order {
            if order.len() != rows.len() {
                return Err(protocol::MutationApplyError::new("diff.order-mismatch", "order must list every row exactly once"));
            }
            let mut pool = rows;
            let mut ordered = Vec::with_capacity(pool.len());
            for id in order {
                let at = pool.iter().position(|row| Self::key(row) == id).ok_or_else(|| protocol::MutationApplyError::new("diff.order-mismatch", format!("order names unknown row \"{id}\"")).at([id.clone()]))?;
                ordered.push(pool.remove(at));
            }
            rows = ordered;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        let other_removed = other.removed.clone();
        let other_added_ids: Vec<String> = other.added.iter().map(|row| Self::key(row).to_string()).collect();
        for id in other.removed {
            if let Some(at) = self.added.iter().position(|row| Self::key(row) == id) {
                self.added.remove(at);
            } else {
                if !self.removed.contains(&id) {
                    self.removed.push(id);
                }
            }
        }
        self.added.extend(other.added);
        self.order = match (other.order, self.order.take()) {
            (Some(order), _) => Some(order),
            (None, Some(mut order)) => {
                order.retain(|id| !other_removed.contains(id));
                order.extend(other_added_ids);
                Some(order)
            }
            (None, None) => None,
        };
    }

    fn inverse_rows(&self, base: &[crate::part_4::Subject]) -> Self {
        let mut inverse = Self::default();
        inverse.removed = self.added.iter().map(|row| Self::key(row).to_string()).collect();
        inverse.added = base.iter().filter(|row| self.removed.iter().any(|id| id == Self::key(row))).cloned().collect();
        let base_ids = Self::ids(base);
        let mut after_ids: Vec<String> = base_ids.iter().filter(|id| !self.removed.contains(id)).cloned().collect();
        after_ids.extend(self.added.iter().map(|row| Self::key(row).to_string()));
        if let Some(order) = &self.order {
            after_ids = order.clone();
        }
        let mut natural: Vec<String> = after_ids.into_iter().filter(|id| !inverse.removed.contains(id)).collect();
        natural.extend(inverse.added.iter().map(|row| Self::key(row).to_string()));
        if natural != base_ids {
            inverse.order = Some(base_ids);
        }
        inverse
    }

    fn between_rows(base: &[crate::part_4::Subject], other: &[crate::part_4::Subject]) -> Self {
        let mut diff = Self::default();
        diff.removed = base.iter().filter(|row| other.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).map(|row| Self::key(row).to_string()).collect();
        diff.added = other.iter().filter(|row| base.iter().find(|candidate| Self::key(candidate) == Self::key(row)) != Some(*row)).cloned().collect();
        let mut natural: Vec<String> = Self::ids(base).into_iter().filter(|id| !diff.removed.contains(id)).collect();
        natural.extend(diff.added.iter().map(|row| Self::key(row).to_string()));
        let wanted = Self::ids(other);
        if natural != wanted {
            diff.order = Some(wanted);
        }
        diff
    }
}

/// 🔑️ A `geometry.objects` entry: its map key and value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757GeometryObjectsEntry {
    pub key: String,
    pub value: crate::part_2::GeometryObject,
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

    fn apply_rows(&self, base: &std::collections::BTreeMap<String, crate::part_2::GeometryObject>) -> Result<std::collections::BTreeMap<String, crate::part_2::GeometryObject>, protocol::MutationApplyError> {
        let mut map = base.clone();
        for key in &self.removed {
            map.remove(key).ok_or_else(|| missing_target(format!("removed entry \"{key}\"")).at([key.clone()]))?;
        }
        for entry in &self.added {
            if map.insert(entry.key.clone(), entry.value.clone()).is_some() {
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("entry \"{}\" already exists", entry.key)).at([entry.key.clone()]));
            }
        }
        Ok(map)
    }

    fn absorb_rows(&mut self, other: Self) {
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

    fn inverse_rows(&self, base: &std::collections::BTreeMap<String, crate::part_2::GeometryObject>) -> Self {
        Self {
            removed: self.added.iter().map(|entry| entry.key.clone()).collect(),
            added: self.removed.iter().filter_map(|key| base.get(key).map(|value| Iso16757GeometryObjectsEntry { key: key.clone(), value: value.clone() })).collect(),
        }
    }

    fn between_rows(base: &std::collections::BTreeMap<String, crate::part_2::GeometryObject>, other: &std::collections::BTreeMap<String, crate::part_2::GeometryObject>) -> Self {
        Self {
            removed: base.iter().filter(|(key, value)| other.get(*key) != Some(*value)).map(|(key, _)| key.clone()).collect(),
            added: other.iter().filter(|(key, value)| base.get(*key) != Some(*value)).map(|(key, value)| Iso16757GeometryObjectsEntry { key: key.clone(), value: value.clone() }).collect(),

        }
    }
}

/// 📌️ A `selection.constraints` row inserted at final position `index`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757SelectionConstraintsInserted {
    pub index: usize,
    pub row: crate::part_1::SelectionConstraint,
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

    fn apply_rows(&self, base: &[crate::part_1::SelectionConstraint]) -> Result<Vec<crate::part_1::SelectionConstraint>, protocol::MutationApplyError> {
        let out_of_range = |index: usize| protocol::MutationApplyError::new("diff.index-out-of-range", format!("row position {index} is out of range")).at([index.to_string()]);
        if !(Self::strictly_ascending(self.removed.iter().copied()) && Self::strictly_ascending(self.inserted.iter().map(|inserted| inserted.index))) {
            return Err(protocol::MutationApplyError::new("diff.index-order", "row positions must be strictly ascending"));
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

    fn inverse_rows(&self, base: &[crate::part_1::SelectionConstraint]) -> Self {
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
            inserted: self.removed.iter().filter_map(|index| base.get(*index).map(|row| Iso16757SelectionConstraintsInserted { index: *index, row: row.clone() })).collect(),
        }
    }

    fn absorb_rows(&mut self, other: Self) {
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

    fn between_rows(base: &[crate::part_1::SelectionConstraint], other: &[crate::part_1::SelectionConstraint]) -> Self {
        let prefix = base.iter().zip(other).take_while(|(left, right)| left == right).count();
        let suffix = base[prefix..].iter().rev().zip(other[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
        Self {
            removed: (prefix..base.len() - suffix).collect(),
            inserted: (prefix..other.len() - suffix).map(|index| Iso16757SelectionConstraintsInserted { index, row: other[index].clone() }).collect(),

        }
    }
}

/// 🔑️ A `part_number_inputs` entry: its map key and value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757PartNumberInputsEntry {
    pub key: String,
    pub value: crate::CatalogueValue,
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

    fn apply_rows(&self, base: &std::collections::BTreeMap<String, crate::CatalogueValue>) -> Result<std::collections::BTreeMap<String, crate::CatalogueValue>, protocol::MutationApplyError> {
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
                return Err(protocol::MutationApplyError::new("diff.duplicate-id", format!("entry \"{}\" already exists", entry.key)).at([entry.key.clone()]));
            }
        }
        Ok(map)
    }

    fn absorb_rows(&mut self, other: Self) {
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

    fn inverse_rows(&self, base: &std::collections::BTreeMap<String, crate::CatalogueValue>) -> Self {
        Self {
            removed: self.added.iter().map(|entry| entry.key.clone()).collect(),
            added: self.removed.iter().filter_map(|key| base.get(key).map(|value| Iso16757PartNumberInputsEntry { key: key.clone(), value: value.clone() })).collect(),
            modified: self.modified.iter().filter_map(|entry| base.get(&entry.key).map(|value| Iso16757PartNumberInputsEntry { key: entry.key.clone(), value: value.clone() })).collect(),
        }
    }

    fn between_rows(base: &std::collections::BTreeMap<String, crate::CatalogueValue>, other: &std::collections::BTreeMap<String, crate::CatalogueValue>) -> Self {
        Self {
            removed: base.iter().filter(|(key, value)| other.get(*key) != Some(*value)).map(|(key, _)| key.clone()).collect(),
            added: other.iter().filter(|(key, value)| base.get(*key) != Some(*value)).map(|(key, value)| Iso16757PartNumberInputsEntry { key: key.clone(), value: value.clone() }).collect(),
            modified: Vec::new(),
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
    fn apply_to_row(&self, row: &mut crate::part_5::ScriptLimits) -> Result<(), protocol::MutationApplyError> {
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

    fn merge(&mut self, other: Self) {
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

    fn inverse_from_row(&self, row: &crate::part_5::ScriptLimits) -> Self {
        Self {
            max_steps: self.max_steps.as_ref().map(|_| row.max_steps.clone()),
            max_recursion: self.max_recursion.as_ref().map(|_| row.max_recursion.clone()),
            timeout_ms: self.timeout_ms.as_ref().map(|_| row.timeout_ms.clone()),
        }
    }
}

/// 🎁️ Carries the optional `SelectionRequest.series_id` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757SelectionSeriesIdValue {
    pub value: Option<String>,
}

/// 🔺️ Keyed sparse diff of the Iso16757 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Diff {
    #[state(artifact)]
    pub catalogue_name: Option<String>,
    #[state(artifact)]
    pub manufacturer_name: Option<String>,
    #[state(artifact)]
    pub product_groups: Option<Iso16757ProductGroupsRows>,
    #[state(artifact)]
    pub product_classes: Option<Iso16757ProductClassesRows>,
    #[state(artifact)]
    pub product_series: Option<Iso16757ProductSeriesRows>,
    #[state(artifact)]
    pub products: Option<Iso16757ProductsRows>,
    #[state(artifact)]
    pub product_indexes: Option<Iso16757ProductIndexesRows>,
    #[state(artifact)]
    pub property_definitions: Option<Iso16757PropertyDefinitionsRows>,
    #[state(artifact)]
    pub subjects: Option<Iso16757SubjectsRows>,
    #[state(artifact)]
    pub geometry_objects: Option<Iso16757GeometryObjectsRows>,
    #[state(artifact)]
    pub selection_class_id: Option<String>,
    #[state(artifact)]
    pub selection_series_id: Option<Iso16757SelectionSeriesIdValue>,
    #[state(artifact)]
    pub selection_constraints: Option<Iso16757SelectionConstraintsRows>,
    #[state(artifact)]
    pub part_number_rule: Option<crate::part_5::PartNumberRule>,
    #[state(artifact)]
    pub part_number_inputs: Option<Iso16757PartNumberInputsRows>,
    #[state(artifact)]
    pub script_limits: Option<Iso16757ScriptLimitsPatch>,
    #[state(artifact)]
    pub exchange_process: Option<crate::part_5::ExchangeProcess>,
}

impl protocol::MutationDiff<Iso16757Snapshot> for Iso16757Diff {
    fn apply(&self, base: &Iso16757Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Iso16757Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.catalogue_name {
            next.catalogue.metadata.names.preferred.text = value.clone();
        }
        if let Some(value) = &self.manufacturer_name {
            next.catalogue.manufacturer.names.preferred.text = value.clone();
        }
        if let Some(rows) = &self.product_groups {
            next.catalogue.product_groups = rows.apply_rows(&base.catalogue.product_groups).map_err(|error| error.under(["product_groups"]))?;
        }
        if let Some(rows) = &self.product_classes {
            next.catalogue.product_classes = rows.apply_rows(&base.catalogue.product_classes).map_err(|error| error.under(["product_classes"]))?;
        }
        if let Some(rows) = &self.product_series {
            next.catalogue.product_series = rows.apply_rows(&base.catalogue.product_series).map_err(|error| error.under(["product_series"]))?;
        }
        if let Some(rows) = &self.products {
            next.catalogue.products = rows.apply_rows(&base.catalogue.products).map_err(|error| error.under(["products"]))?;
        }
        if let Some(rows) = &self.product_indexes {
            next.catalogue.product_indexes = rows.apply_rows(&base.catalogue.product_indexes).map_err(|error| error.under(["product_indexes"]))?;
        }
        if let Some(rows) = &self.property_definitions {
            next.catalogue.property_definitions = rows.apply_rows(&base.catalogue.property_definitions).map_err(|error| error.under(["property_definitions"]))?;
        }
        if let Some(rows) = &self.subjects {
            next.dictionary.subjects = rows.apply_rows(&base.dictionary.subjects).map_err(|error| error.under(["subjects"]))?;
        }
        if let Some(rows) = &self.geometry_objects {
            next.geometry.objects = rows.apply_rows(&base.geometry.objects).map_err(|error| error.under(["geometry_objects"]))?;
        }
        if let Some(value) = &self.selection_class_id {
            next.selection.class_id = value.clone();
        }
        if let Some(value) = &self.selection_series_id {
            next.selection.series_id = value.value.clone();
        }
        if let Some(rows) = &self.selection_constraints {
            next.selection.constraints = rows.apply_rows(&base.selection.constraints).map_err(|error| error.under(["selection_constraints"]))?;
        }
        if let Some(value) = &self.part_number_rule {
            next.part_number_rule = value.clone();
        }
        if let Some(rows) = &self.part_number_inputs {
            next.part_number_inputs = rows.apply_rows(&base.part_number_inputs).map_err(|error| error.under(["part_number_inputs"]))?;
        }
        if let Some(patch) = &self.script_limits {
            patch.apply_to_row(&mut next.script_limits).map_err(|error| error.under(["script_limits"]))?;
        }
        if let Some(value) = &self.exchange_process {
            next.exchange_process = value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.catalogue_name.is_some() {
            self.catalogue_name = other.catalogue_name;
        }
        if other.manufacturer_name.is_some() {
            self.manufacturer_name = other.manufacturer_name;
        }
        if let Some(theirs) = other.product_groups {
            match self.product_groups.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.product_groups = Some(theirs),
            }
            self.product_groups = self.product_groups.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.product_classes {
            match self.product_classes.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.product_classes = Some(theirs),
            }
            self.product_classes = self.product_classes.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.product_series {
            match self.product_series.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.product_series = Some(theirs),
            }
            self.product_series = self.product_series.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.products {
            match self.products.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.products = Some(theirs),
            }
            self.products = self.products.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.product_indexes {
            match self.product_indexes.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.product_indexes = Some(theirs),
            }
            self.product_indexes = self.product_indexes.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.property_definitions {
            match self.property_definitions.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.property_definitions = Some(theirs),
            }
            self.property_definitions = self.property_definitions.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.subjects {
            match self.subjects.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.subjects = Some(theirs),
            }
            self.subjects = self.subjects.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.geometry_objects {
            match self.geometry_objects.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.geometry_objects = Some(theirs),
            }
            self.geometry_objects = self.geometry_objects.take().filter(|rows| !rows.is_empty());
        }
        if other.selection_class_id.is_some() {
            self.selection_class_id = other.selection_class_id;
        }
        if other.selection_series_id.is_some() {
            self.selection_series_id = other.selection_series_id;
        }
        if let Some(theirs) = other.selection_constraints {
            match self.selection_constraints.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.selection_constraints = Some(theirs),
            }
            self.selection_constraints = self.selection_constraints.take().filter(|rows| !rows.is_empty());
        }
        if other.part_number_rule.is_some() {
            self.part_number_rule = other.part_number_rule;
        }
        if let Some(theirs) = other.part_number_inputs {
            match self.part_number_inputs.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.part_number_inputs = Some(theirs),
            }
            self.part_number_inputs = self.part_number_inputs.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.script_limits {
            match self.script_limits.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.script_limits = Some(theirs),
            }
        }
        if other.exchange_process.is_some() {
            self.exchange_process = other.exchange_process;
        }
    }
}

impl protocol::DiffAlgebra<Iso16757Snapshot> for Iso16757Diff {
    fn inverse(&self, base: &Iso16757Snapshot) -> Self {
        Self {
            catalogue_name: self.catalogue_name.as_ref().map(|_| base.catalogue.metadata.names.preferred.text.clone()),
            manufacturer_name: self.manufacturer_name.as_ref().map(|_| base.catalogue.manufacturer.names.preferred.text.clone()),
            product_groups: self.product_groups.as_ref().map(|rows| rows.inverse_rows(&base.catalogue.product_groups)).filter(|rows| !rows.is_empty()),
            product_classes: self.product_classes.as_ref().map(|rows| rows.inverse_rows(&base.catalogue.product_classes)).filter(|rows| !rows.is_empty()),
            product_series: self.product_series.as_ref().map(|rows| rows.inverse_rows(&base.catalogue.product_series)).filter(|rows| !rows.is_empty()),
            products: self.products.as_ref().map(|rows| rows.inverse_rows(&base.catalogue.products)).filter(|rows| !rows.is_empty()),
            product_indexes: self.product_indexes.as_ref().map(|rows| rows.inverse_rows(&base.catalogue.product_indexes)).filter(|rows| !rows.is_empty()),
            property_definitions: self.property_definitions.as_ref().map(|rows| rows.inverse_rows(&base.catalogue.property_definitions)).filter(|rows| !rows.is_empty()),
            subjects: self.subjects.as_ref().map(|rows| rows.inverse_rows(&base.dictionary.subjects)).filter(|rows| !rows.is_empty()),
            geometry_objects: self.geometry_objects.as_ref().map(|rows| rows.inverse_rows(&base.geometry.objects)).filter(|rows| !rows.is_empty()),
            selection_class_id: self.selection_class_id.as_ref().map(|_| base.selection.class_id.clone()),
            selection_series_id: self.selection_series_id.as_ref().map(|_| Iso16757SelectionSeriesIdValue { value: base.selection.series_id.clone() }),
            selection_constraints: self.selection_constraints.as_ref().map(|rows| rows.inverse_rows(&base.selection.constraints)).filter(|rows| !rows.is_empty()),
            part_number_rule: self.part_number_rule.as_ref().map(|_| base.part_number_rule.clone()),
            part_number_inputs: self.part_number_inputs.as_ref().map(|rows| rows.inverse_rows(&base.part_number_inputs)).filter(|rows| !rows.is_empty()),
            script_limits: self.script_limits.as_ref().map(|patch| patch.inverse_from_row(&base.script_limits)),
            exchange_process: self.exchange_process.as_ref().map(|_| base.exchange_process.clone()),
        }
    }

    fn between(base: &Iso16757Snapshot, other: &Iso16757Snapshot) -> Self {
        Self {
            catalogue_name: (base.catalogue.metadata.names.preferred.text != other.catalogue.metadata.names.preferred.text).then(|| other.catalogue.metadata.names.preferred.text.clone()),
            manufacturer_name: (base.catalogue.manufacturer.names.preferred.text != other.catalogue.manufacturer.names.preferred.text).then(|| other.catalogue.manufacturer.names.preferred.text.clone()),
            product_groups: Some(Iso16757ProductGroupsRows::between_rows(&base.catalogue.product_groups, &other.catalogue.product_groups)).filter(|rows| !rows.is_empty()),
            product_classes: Some(Iso16757ProductClassesRows::between_rows(&base.catalogue.product_classes, &other.catalogue.product_classes)).filter(|rows| !rows.is_empty()),
            product_series: Some(Iso16757ProductSeriesRows::between_rows(&base.catalogue.product_series, &other.catalogue.product_series)).filter(|rows| !rows.is_empty()),
            products: Some(Iso16757ProductsRows::between_rows(&base.catalogue.products, &other.catalogue.products)).filter(|rows| !rows.is_empty()),
            product_indexes: Some(Iso16757ProductIndexesRows::between_rows(&base.catalogue.product_indexes, &other.catalogue.product_indexes)).filter(|rows| !rows.is_empty()),
            property_definitions: Some(Iso16757PropertyDefinitionsRows::between_rows(&base.catalogue.property_definitions, &other.catalogue.property_definitions)).filter(|rows| !rows.is_empty()),
            subjects: Some(Iso16757SubjectsRows::between_rows(&base.dictionary.subjects, &other.dictionary.subjects)).filter(|rows| !rows.is_empty()),
            geometry_objects: Some(Iso16757GeometryObjectsRows::between_rows(&base.geometry.objects, &other.geometry.objects)).filter(|rows| !rows.is_empty()),
            selection_class_id: (base.selection.class_id != other.selection.class_id).then(|| other.selection.class_id.clone()),
            selection_series_id: (base.selection.series_id != other.selection.series_id).then(|| Iso16757SelectionSeriesIdValue { value: other.selection.series_id.clone() }),
            selection_constraints: Some(Iso16757SelectionConstraintsRows::between_rows(&base.selection.constraints, &other.selection.constraints)).filter(|rows| !rows.is_empty()),
            part_number_rule: (base.part_number_rule != other.part_number_rule).then(|| other.part_number_rule.clone()),
            part_number_inputs: Some(Iso16757PartNumberInputsRows::between_rows(&base.part_number_inputs, &other.part_number_inputs)).filter(|rows| !rows.is_empty()),
            script_limits: Some(Iso16757ScriptLimitsPatch {
                max_steps: (base.script_limits.max_steps != other.script_limits.max_steps).then(|| other.script_limits.max_steps.clone()),
                max_recursion: (base.script_limits.max_recursion != other.script_limits.max_recursion).then(|| other.script_limits.max_recursion.clone()),
                timeout_ms: (base.script_limits.timeout_ms != other.script_limits.timeout_ms).then(|| other.script_limits.timeout_ms.clone()),
            }).filter(|patch| *patch != Iso16757ScriptLimitsPatch::default()),
            exchange_process: (base.exchange_process != other.exchange_process).then(|| other.exchange_process.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.catalogue_name.is_none() && self.manufacturer_name.is_none() && self.product_groups.as_ref().map_or(true, |rows| rows.is_empty()) && self.product_classes.as_ref().map_or(true, |rows| rows.is_empty()) && self.product_series.as_ref().map_or(true, |rows| rows.is_empty()) && self.products.as_ref().map_or(true, |rows| rows.is_empty()) && self.product_indexes.as_ref().map_or(true, |rows| rows.is_empty()) && self.property_definitions.as_ref().map_or(true, |rows| rows.is_empty()) && self.subjects.as_ref().map_or(true, |rows| rows.is_empty()) && self.geometry_objects.as_ref().map_or(true, |rows| rows.is_empty()) && self.selection_class_id.is_none() && self.selection_series_id.is_none() && self.selection_constraints.as_ref().map_or(true, |rows| rows.is_empty()) && self.part_number_rule.is_none() && self.part_number_inputs.as_ref().map_or(true, |rows| rows.is_empty()) && self.script_limits.is_none() && self.exchange_process.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
