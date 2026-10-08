//! 🧬️ En1999 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff. `DiffAlgebra::between` covers the modelled vocabulary only and exists for sync tooling, never for mutation leaves.

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("diff.target-missing", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `materials` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MaterialsPatch {
    pub id: String,
    pub designation: Option<String>,
}

impl En1999MaterialsPatch {
    fn apply_to_row(&self, row: &mut crate::snapshot::AluminiumMaterial) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.designation {
            row.designation = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.designation.is_some() {
            self.designation = other.designation;
        }
    }

    fn inverse_from_row(&self, row: &crate::snapshot::AluminiumMaterial) -> Self {
        Self {
            id: self.id.clone(),
            designation: self.designation.as_ref().map(|_| row.designation.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `materials` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MaterialsRows {
    pub added: Vec<crate::snapshot::AluminiumMaterial>,
    pub removed: Vec<String>,
    pub modified: Vec<En1999MaterialsPatch>,
    pub order: Option<Vec<String>>,
}

impl En1999MaterialsRows {
    fn key(row: &crate::snapshot::AluminiumMaterial) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::snapshot::AluminiumMaterial]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::AluminiumMaterial]) -> Result<Vec<crate::snapshot::AluminiumMaterial>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::AluminiumMaterial]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::AluminiumMaterial], other: &[crate::snapshot::AluminiumMaterial]) -> Self {
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

/// 🩹️ Sparse patch of the `elements` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsElementsPatch {
    pub id: String,
    pub thickness: Option<f64>,
}

impl En1999SectionsElementsPatch {
    fn apply_to_row(&self, row: &mut crate::snapshot::PlateElement) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.thickness {
            row.thickness = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.thickness.is_some() {
            self.thickness = other.thickness;
        }
    }

    fn inverse_from_row(&self, row: &crate::snapshot::PlateElement) -> Self {
        Self {
            id: self.id.clone(),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `elements` rows (by `id`): modified row patches.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsElementsRows {
    pub modified: Vec<En1999SectionsElementsPatch>,
}

impl En1999SectionsElementsRows {
    fn key(row: &crate::snapshot::PlateElement) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.modified.is_empty()
    }

    fn ids(rows: &[crate::snapshot::PlateElement]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::PlateElement]) -> Result<Vec<crate::snapshot::PlateElement>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for patch in &self.modified {
            let row = rows.iter_mut().find(|row| Self::key(row) == patch.id).ok_or_else(|| missing_target(format!("modified row \"{}\"", patch.id)).at([patch.id.clone()]))?;
            patch.apply_to_row(row)?;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        for patch in other.modified {
            if let Some(existing) = self.modified.iter_mut().find(|existing| existing.id == patch.id) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
    }

    fn inverse_rows(&self, base: &[crate::snapshot::PlateElement]) -> Self {
        Self {
            modified: self.modified.iter().filter_map(|patch| base.iter().find(|row| Self::key(row) == patch.id).map(|row| patch.inverse_from_row(row))).collect(),
        }
    }

}

/// 🩹️ Sparse patch of the `sections` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsPatch {
    pub id: String,
    pub elements: Option<En1999SectionsElementsRows>,
}

impl En1999SectionsPatch {
    fn apply_to_row(&self, row: &mut crate::snapshot::AluminiumSection) -> Result<(), protocol::MutationApplyError> {
        if let Some(rows) = &self.elements {
            row.elements = rows.apply_rows(&row.elements).map_err(|error| error.under(["elements"]))?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if let Some(theirs) = other.elements {
            match self.elements.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.elements = Some(theirs),
            }
        }
        self.elements = self.elements.take().filter(|rows| !rows.is_empty());
    }

    fn inverse_from_row(&self, row: &crate::snapshot::AluminiumSection) -> Self {
        Self {
            id: self.id.clone(),
            elements: self.elements.as_ref().map(|rows| rows.inverse_rows(&row.elements)).filter(|rows| !rows.is_empty()),
        }
    }
}

/// 🔺️ Keyed diff of `sections` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsRows {
    pub added: Vec<crate::snapshot::AluminiumSection>,
    pub removed: Vec<String>,
    pub modified: Vec<En1999SectionsPatch>,
    pub order: Option<Vec<String>>,
}

impl En1999SectionsRows {
    fn key(row: &crate::snapshot::AluminiumSection) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::snapshot::AluminiumSection]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::AluminiumSection]) -> Result<Vec<crate::snapshot::AluminiumSection>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::AluminiumSection]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::AluminiumSection], other: &[crate::snapshot::AluminiumSection]) -> Self {
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

/// 🩹️ Sparse patch of the `actions` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersActionsPatch {
    pub id: String,
    pub n_k: Option<f64>,
    pub m_y_k: Option<f64>,
}

impl En1999MembersActionsPatch {
    fn apply_to_row(&self, row: &mut crate::snapshot::MemberAction) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.n_k {
            row.n_k = value.clone();
        }
        if let Some(value) = &self.m_y_k {
            row.m_y_k = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.n_k.is_some() {
            self.n_k = other.n_k;
        }
        if other.m_y_k.is_some() {
            self.m_y_k = other.m_y_k;
        }
    }

    fn inverse_from_row(&self, row: &crate::snapshot::MemberAction) -> Self {
        Self {
            id: self.id.clone(),
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            m_y_k: self.m_y_k.as_ref().map(|_| row.m_y_k.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `actions` rows (by `id`): modified row patches.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersActionsRows {
    pub modified: Vec<En1999MembersActionsPatch>,
}

impl En1999MembersActionsRows {
    fn key(row: &crate::snapshot::MemberAction) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.modified.is_empty()
    }

    fn ids(rows: &[crate::snapshot::MemberAction]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::MemberAction]) -> Result<Vec<crate::snapshot::MemberAction>, protocol::MutationApplyError> {
        let mut rows = base.to_vec();
        for patch in &self.modified {
            let row = rows.iter_mut().find(|row| Self::key(row) == patch.id).ok_or_else(|| missing_target(format!("modified row \"{}\"", patch.id)).at([patch.id.clone()]))?;
            patch.apply_to_row(row)?;
        }
        Ok(rows)
    }

    fn absorb_rows(&mut self, other: Self) {
        for patch in other.modified {
            if let Some(existing) = self.modified.iter_mut().find(|existing| existing.id == patch.id) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
    }

    fn inverse_rows(&self, base: &[crate::snapshot::MemberAction]) -> Self {
        Self {
            modified: self.modified.iter().filter_map(|patch| base.iter().find(|row| Self::key(row) == patch.id).map(|row| patch.inverse_from_row(row))).collect(),
        }
    }

}

/// 🩹️ Sparse patch of the `members` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersPatch {
    pub id: String,
    pub buckling_length_y: Option<f64>,
    pub buckling_length_z: Option<f64>,
    pub buckling_length_t: Option<f64>,
    pub ltb_length: Option<f64>,
    pub actions: Option<En1999MembersActionsRows>,
}

impl En1999MembersPatch {
    fn apply_to_row(&self, row: &mut crate::snapshot::AluminiumMember) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.buckling_length_y {
            row.buckling_length_y = value.clone();
        }
        if let Some(value) = &self.buckling_length_z {
            row.buckling_length_z = value.clone();
        }
        if let Some(value) = &self.buckling_length_t {
            row.buckling_length_t = value.clone();
        }
        if let Some(value) = &self.ltb_length {
            row.ltb_length = value.clone();
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.apply_rows(&row.actions).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.buckling_length_y.is_some() {
            self.buckling_length_y = other.buckling_length_y;
        }
        if other.buckling_length_z.is_some() {
            self.buckling_length_z = other.buckling_length_z;
        }
        if other.buckling_length_t.is_some() {
            self.buckling_length_t = other.buckling_length_t;
        }
        if other.ltb_length.is_some() {
            self.ltb_length = other.ltb_length;
        }
        if let Some(theirs) = other.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse_from_row(&self, row: &crate::snapshot::AluminiumMember) -> Self {
        Self {
            id: self.id.clone(),
            buckling_length_y: self.buckling_length_y.as_ref().map(|_| row.buckling_length_y.clone()),
            buckling_length_z: self.buckling_length_z.as_ref().map(|_| row.buckling_length_z.clone()),
            buckling_length_t: self.buckling_length_t.as_ref().map(|_| row.buckling_length_t.clone()),
            ltb_length: self.ltb_length.as_ref().map(|_| row.ltb_length.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse_rows(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }
}

/// 🔺️ Keyed diff of `members` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersRows {
    pub added: Vec<crate::snapshot::AluminiumMember>,
    pub removed: Vec<String>,
    pub modified: Vec<En1999MembersPatch>,
    pub order: Option<Vec<String>>,
}

impl En1999MembersRows {
    fn key(row: &crate::snapshot::AluminiumMember) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::snapshot::AluminiumMember]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::AluminiumMember]) -> Result<Vec<crate::snapshot::AluminiumMember>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::AluminiumMember]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::AluminiumMember], other: &[crate::snapshot::AluminiumMember]) -> Self {
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

/// 🩹️ Sparse patch of the `connections` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ConnectionsPatch {
    pub id: String,
    pub welds_throat: Option<f64>,
    pub bolts_rows: Option<u32>,
    pub bolts_bolts_per_row: Option<u32>,
}

impl En1999ConnectionsPatch {
    fn apply_to_row(&self, row: &mut crate::snapshot::AluminiumConnection) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.welds_throat {
            row.welds.throat = value.clone();
        }
        if let Some(value) = &self.bolts_rows {
            row.bolts.rows = value.clone();
        }
        if let Some(value) = &self.bolts_bolts_per_row {
            row.bolts.bolts_per_row = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.welds_throat.is_some() {
            self.welds_throat = other.welds_throat;
        }
        if other.bolts_rows.is_some() {
            self.bolts_rows = other.bolts_rows;
        }
        if other.bolts_bolts_per_row.is_some() {
            self.bolts_bolts_per_row = other.bolts_bolts_per_row;
        }
    }

    fn inverse_from_row(&self, row: &crate::snapshot::AluminiumConnection) -> Self {
        Self {
            id: self.id.clone(),
            welds_throat: self.welds_throat.as_ref().map(|_| row.welds.throat.clone()),
            bolts_rows: self.bolts_rows.as_ref().map(|_| row.bolts.rows.clone()),
            bolts_bolts_per_row: self.bolts_bolts_per_row.as_ref().map(|_| row.bolts.bolts_per_row.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `connections` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ConnectionsRows {
    pub added: Vec<crate::snapshot::AluminiumConnection>,
    pub removed: Vec<String>,
    pub modified: Vec<En1999ConnectionsPatch>,
    pub order: Option<Vec<String>>,
}

impl En1999ConnectionsRows {
    fn key(row: &crate::snapshot::AluminiumConnection) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::snapshot::AluminiumConnection]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::AluminiumConnection]) -> Result<Vec<crate::snapshot::AluminiumConnection>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::AluminiumConnection]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::AluminiumConnection], other: &[crate::snapshot::AluminiumConnection]) -> Self {
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

/// 🔺️ Keyed diff of `fire_scenarios` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FireScenariosRows {
    pub added: Vec<crate::snapshot::FireScenario>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl En1999FireScenariosRows {
    fn key(row: &crate::snapshot::FireScenario) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::snapshot::FireScenario]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::FireScenario]) -> Result<Vec<crate::snapshot::FireScenario>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::FireScenario]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::FireScenario], other: &[crate::snapshot::FireScenario]) -> Self {
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

/// 🔺️ Keyed diff of `fatigue_details` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FatigueDetailsRows {
    pub added: Vec<crate::snapshot::FatigueDetail>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl En1999FatigueDetailsRows {
    fn key(row: &crate::snapshot::FatigueDetail) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::snapshot::FatigueDetail]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::FatigueDetail]) -> Result<Vec<crate::snapshot::FatigueDetail>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::FatigueDetail]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::FatigueDetail], other: &[crate::snapshot::FatigueDetail]) -> Self {
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

/// 🔺️ Keyed diff of `cold_formed` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ColdFormedRows {
    pub added: Vec<crate::snapshot::ColdFormedSheet>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl En1999ColdFormedRows {
    fn key(row: &crate::snapshot::ColdFormedSheet) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::snapshot::ColdFormedSheet]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::ColdFormedSheet]) -> Result<Vec<crate::snapshot::ColdFormedSheet>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::ColdFormedSheet]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::ColdFormedSheet], other: &[crate::snapshot::ColdFormedSheet]) -> Self {
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

/// 🔺️ Keyed diff of `shells` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ShellsRows {
    pub added: Vec<crate::snapshot::AluminiumShell>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl En1999ShellsRows {
    fn key(row: &crate::snapshot::AluminiumShell) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::snapshot::AluminiumShell]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::snapshot::AluminiumShell]) -> Result<Vec<crate::snapshot::AluminiumShell>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::snapshot::AluminiumShell]) -> Self {
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

    fn between_rows(base: &[crate::snapshot::AluminiumShell], other: &[crate::snapshot::AluminiumShell]) -> Self {
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

/// 🔺️ Keyed sparse diff of the En1999 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub materials: Option<En1999MaterialsRows>,
    #[state(artifact)]
    pub sections: Option<En1999SectionsRows>,
    #[state(artifact)]
    pub members: Option<En1999MembersRows>,
    #[state(artifact)]
    pub connections: Option<En1999ConnectionsRows>,
    #[state(artifact)]
    pub fire_scenarios: Option<En1999FireScenariosRows>,
    #[state(artifact)]
    pub fatigue_details: Option<En1999FatigueDetailsRows>,
    #[state(artifact)]
    pub cold_formed: Option<En1999ColdFormedRows>,
    #[state(artifact)]
    pub shells: Option<En1999ShellsRows>,
}

impl protocol::MutationDiff<En1999Snapshot> for En1999Diff {
    fn apply(&self, base: &En1999Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1999Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(rows) = &self.materials {
            next.materials = rows.apply_rows(&base.materials).map_err(|error| error.under(["materials"]))?;
        }
        if let Some(rows) = &self.sections {
            next.sections = rows.apply_rows(&base.sections).map_err(|error| error.under(["sections"]))?;
        }
        if let Some(rows) = &self.members {
            next.members = rows.apply_rows(&base.members).map_err(|error| error.under(["members"]))?;
        }
        if let Some(rows) = &self.connections {
            next.connections = rows.apply_rows(&base.connections).map_err(|error| error.under(["connections"]))?;
        }
        if let Some(rows) = &self.fire_scenarios {
            next.fire_scenarios = rows.apply_rows(&base.fire_scenarios).map_err(|error| error.under(["fire_scenarios"]))?;
        }
        if let Some(rows) = &self.fatigue_details {
            next.fatigue_details = rows.apply_rows(&base.fatigue_details).map_err(|error| error.under(["fatigue_details"]))?;
        }
        if let Some(rows) = &self.cold_formed {
            next.cold_formed = rows.apply_rows(&base.cold_formed).map_err(|error| error.under(["cold_formed"]))?;
        }
        if let Some(rows) = &self.shells {
            next.shells = rows.apply_rows(&base.shells).map_err(|error| error.under(["shells"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if let Some(theirs) = other.materials {
            match self.materials.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.materials = Some(theirs),
            }
            self.materials = self.materials.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.sections {
            match self.sections.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.sections = Some(theirs),
            }
            self.sections = self.sections.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.members {
            match self.members.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.members = Some(theirs),
            }
            self.members = self.members.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.connections {
            match self.connections.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.connections = Some(theirs),
            }
            self.connections = self.connections.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.fire_scenarios {
            match self.fire_scenarios.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.fire_scenarios = Some(theirs),
            }
            self.fire_scenarios = self.fire_scenarios.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.fatigue_details {
            match self.fatigue_details.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.fatigue_details = Some(theirs),
            }
            self.fatigue_details = self.fatigue_details.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.cold_formed {
            match self.cold_formed.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.cold_formed = Some(theirs),
            }
            self.cold_formed = self.cold_formed.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.shells {
            match self.shells.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.shells = Some(theirs),
            }
            self.shells = self.shells.take().filter(|rows| !rows.is_empty());
        }
    }
}

impl protocol::DiffAlgebra<En1999Snapshot> for En1999Diff {
    fn inverse(&self, base: &En1999Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            materials: self.materials.as_ref().map(|rows| rows.inverse_rows(&base.materials)).filter(|rows| !rows.is_empty()),
            sections: self.sections.as_ref().map(|rows| rows.inverse_rows(&base.sections)).filter(|rows| !rows.is_empty()),
            members: self.members.as_ref().map(|rows| rows.inverse_rows(&base.members)).filter(|rows| !rows.is_empty()),
            connections: self.connections.as_ref().map(|rows| rows.inverse_rows(&base.connections)).filter(|rows| !rows.is_empty()),
            fire_scenarios: self.fire_scenarios.as_ref().map(|rows| rows.inverse_rows(&base.fire_scenarios)).filter(|rows| !rows.is_empty()),
            fatigue_details: self.fatigue_details.as_ref().map(|rows| rows.inverse_rows(&base.fatigue_details)).filter(|rows| !rows.is_empty()),
            cold_formed: self.cold_formed.as_ref().map(|rows| rows.inverse_rows(&base.cold_formed)).filter(|rows| !rows.is_empty()),
            shells: self.shells.as_ref().map(|rows| rows.inverse_rows(&base.shells)).filter(|rows| !rows.is_empty()),
        }
    }

    fn between(base: &En1999Snapshot, other: &En1999Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            materials: Some(En1999MaterialsRows::between_rows(&base.materials, &other.materials)).filter(|rows| !rows.is_empty()),
            sections: Some(En1999SectionsRows::between_rows(&base.sections, &other.sections)).filter(|rows| !rows.is_empty()),
            members: Some(En1999MembersRows::between_rows(&base.members, &other.members)).filter(|rows| !rows.is_empty()),
            connections: Some(En1999ConnectionsRows::between_rows(&base.connections, &other.connections)).filter(|rows| !rows.is_empty()),
            fire_scenarios: Some(En1999FireScenariosRows::between_rows(&base.fire_scenarios, &other.fire_scenarios)).filter(|rows| !rows.is_empty()),
            fatigue_details: Some(En1999FatigueDetailsRows::between_rows(&base.fatigue_details, &other.fatigue_details)).filter(|rows| !rows.is_empty()),
            cold_formed: Some(En1999ColdFormedRows::between_rows(&base.cold_formed, &other.cold_formed)).filter(|rows| !rows.is_empty()),
            shells: Some(En1999ShellsRows::between_rows(&base.shells, &other.shells)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.materials.as_ref().map_or(true, |rows| rows.is_empty()) && self.sections.as_ref().map_or(true, |rows| rows.is_empty()) && self.members.as_ref().map_or(true, |rows| rows.is_empty()) && self.connections.as_ref().map_or(true, |rows| rows.is_empty()) && self.fire_scenarios.as_ref().map_or(true, |rows| rows.is_empty()) && self.fatigue_details.as_ref().map_or(true, |rows| rows.is_empty()) && self.cold_formed.as_ref().map_or(true, |rows| rows.is_empty()) && self.shells.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
