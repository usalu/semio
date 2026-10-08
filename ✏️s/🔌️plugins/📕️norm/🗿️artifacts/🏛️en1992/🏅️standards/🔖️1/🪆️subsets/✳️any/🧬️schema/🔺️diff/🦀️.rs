//! 🧬️ En1992 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff. `DiffAlgebra::between` covers the modelled vocabulary only and exists for sync tooling, never for mutation leaves.

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("diff.target-missing", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `concrete_grades` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ConcreteGradesPatch {
    pub id: String,
    pub f_ck: Option<f64>,
}

impl En1992ConcreteGradesPatch {
    fn apply_to_row(&self, row: &mut crate::ConcreteGrade) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.f_ck {
            row.f_ck = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.f_ck.is_some() {
            self.f_ck = other.f_ck;
        }
    }

    fn inverse_from_row(&self, row: &crate::ConcreteGrade) -> Self {
        Self {
            id: self.id.clone(),
            f_ck: self.f_ck.as_ref().map(|_| row.f_ck.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `concrete_grades` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ConcreteGradesRows {
    pub added: Vec<crate::ConcreteGrade>,
    pub removed: Vec<String>,
    pub modified: Vec<En1992ConcreteGradesPatch>,
    pub order: Option<Vec<String>>,
}

impl En1992ConcreteGradesRows {
    fn key(row: &crate::ConcreteGrade) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::ConcreteGrade]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::ConcreteGrade]) -> Result<Vec<crate::ConcreteGrade>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::ConcreteGrade]) -> Self {
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

    fn between_rows(base: &[crate::ConcreteGrade], other: &[crate::ConcreteGrade]) -> Self {
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

/// 🩹️ Sparse patch of the `reinforcement_grades` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ReinforcementGradesPatch {
    pub id: String,
    pub f_yk: Option<f64>,
}

impl En1992ReinforcementGradesPatch {
    fn apply_to_row(&self, row: &mut crate::ReinforcementGrade) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.f_yk {
            row.f_yk = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.f_yk.is_some() {
            self.f_yk = other.f_yk;
        }
    }

    fn inverse_from_row(&self, row: &crate::ReinforcementGrade) -> Self {
        Self {
            id: self.id.clone(),
            f_yk: self.f_yk.as_ref().map(|_| row.f_yk.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `reinforcement_grades` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ReinforcementGradesRows {
    pub added: Vec<crate::ReinforcementGrade>,
    pub removed: Vec<String>,
    pub modified: Vec<En1992ReinforcementGradesPatch>,
    pub order: Option<Vec<String>>,
}

impl En1992ReinforcementGradesRows {
    fn key(row: &crate::ReinforcementGrade) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::ReinforcementGrade]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::ReinforcementGrade]) -> Result<Vec<crate::ReinforcementGrade>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::ReinforcementGrade]) -> Self {
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

    fn between_rows(base: &[crate::ReinforcementGrade], other: &[crate::ReinforcementGrade]) -> Self {
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

/// 🔺️ Keyed diff of `prestress_steels` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992PrestressSteelsRows {
    pub added: Vec<crate::PrestressSteel>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl En1992PrestressSteelsRows {
    fn key(row: &crate::PrestressSteel) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::PrestressSteel]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::PrestressSteel]) -> Result<Vec<crate::PrestressSteel>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::PrestressSteel]) -> Self {
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

    fn between_rows(base: &[crate::PrestressSteel], other: &[crate::PrestressSteel]) -> Self {
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

/// 🩹️ Sparse patch of the `longitudinal` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersLongitudinalPatch {
    pub id: String,
    pub diameter: Option<f64>,
    pub count: Option<u32>,
}

impl En1992MembersLongitudinalPatch {
    fn apply_to_row(&self, row: &mut crate::BarLayer) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.diameter {
            row.diameter = value.clone();
        }
        if let Some(value) = &self.count {
            row.count = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.diameter.is_some() {
            self.diameter = other.diameter;
        }
        if other.count.is_some() {
            self.count = other.count;
        }
    }

    fn inverse_from_row(&self, row: &crate::BarLayer) -> Self {
        Self {
            id: self.id.clone(),
            diameter: self.diameter.as_ref().map(|_| row.diameter.clone()),
            count: self.count.as_ref().map(|_| row.count.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `longitudinal` rows (by `id`): modified row patches.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersLongitudinalRows {
    pub modified: Vec<En1992MembersLongitudinalPatch>,
}

impl En1992MembersLongitudinalRows {
    fn key(row: &crate::BarLayer) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.modified.is_empty()
    }

    fn ids(rows: &[crate::BarLayer]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::BarLayer]) -> Result<Vec<crate::BarLayer>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::BarLayer]) -> Self {
        Self {
            modified: self.modified.iter().filter_map(|patch| base.iter().find(|row| Self::key(row) == patch.id).map(|row| patch.inverse_from_row(row))).collect(),
        }
    }

}

/// 🩹️ Sparse patch of the `actions` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersActionsPatch {
    pub id: String,
    pub m_k: Option<f64>,
    pub n_k: Option<f64>,
    pub v_k: Option<f64>,
}

impl En1992MembersActionsPatch {
    fn apply_to_row(&self, row: &mut crate::LoadCaseActions) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.m_k {
            row.m_k = value.clone();
        }
        if let Some(value) = &self.n_k {
            row.n_k = value.clone();
        }
        if let Some(value) = &self.v_k {
            row.v_k = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.m_k.is_some() {
            self.m_k = other.m_k;
        }
        if other.n_k.is_some() {
            self.n_k = other.n_k;
        }
        if other.v_k.is_some() {
            self.v_k = other.v_k;
        }
    }

    fn inverse_from_row(&self, row: &crate::LoadCaseActions) -> Self {
        Self {
            id: self.id.clone(),
            m_k: self.m_k.as_ref().map(|_| row.m_k.clone()),
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            v_k: self.v_k.as_ref().map(|_| row.v_k.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `actions` rows (by `id`): modified row patches.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersActionsRows {
    pub modified: Vec<En1992MembersActionsPatch>,
}

impl En1992MembersActionsRows {
    fn key(row: &crate::LoadCaseActions) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.modified.is_empty()
    }

    fn ids(rows: &[crate::LoadCaseActions]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::LoadCaseActions]) -> Result<Vec<crate::LoadCaseActions>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::LoadCaseActions]) -> Self {
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
pub struct En1992MembersPatch {
    pub id: String,
    pub exposure: Option<crate::ExposureClass>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub effective_depth: Option<f64>,
    pub cover: Option<f64>,
    pub span: Option<f64>,
    pub fire_rating: Option<crate::FireRating>,
    pub fire_axis_distance: Option<f64>,
    pub stirrups_spacing: Option<f64>,
    pub longitudinal: Option<En1992MembersLongitudinalRows>,
    pub actions: Option<En1992MembersActionsRows>,
}

impl En1992MembersPatch {
    fn apply_to_row(&self, row: &mut crate::RcMember) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.exposure {
            row.exposure = value.clone();
        }
        if let Some(value) = &self.width {
            row.width = value.clone();
        }
        if let Some(value) = &self.height {
            row.height = value.clone();
        }
        if let Some(value) = &self.effective_depth {
            row.effective_depth = value.clone();
        }
        if let Some(value) = &self.cover {
            row.cover = value.clone();
        }
        if let Some(value) = &self.span {
            row.span = value.clone();
        }
        if let Some(value) = &self.fire_rating {
            row.fire.as_mut().ok_or_else(|| missing_target("fire"))?.rating = value.clone();
        }
        if let Some(value) = &self.fire_axis_distance {
            row.fire.as_mut().ok_or_else(|| missing_target("fire"))?.axis_distance = value.clone();
        }
        if let Some(value) = &self.stirrups_spacing {
            row.stirrups.as_mut().ok_or_else(|| missing_target("stirrups"))?.spacing = value.clone();
        }
        if let Some(rows) = &self.longitudinal {
            row.longitudinal = rows.apply_rows(&row.longitudinal).map_err(|error| error.under(["longitudinal"]))?;
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.apply_rows(&row.actions).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.exposure.is_some() {
            self.exposure = other.exposure;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.effective_depth.is_some() {
            self.effective_depth = other.effective_depth;
        }
        if other.cover.is_some() {
            self.cover = other.cover;
        }
        if other.span.is_some() {
            self.span = other.span;
        }
        if other.fire_rating.is_some() {
            self.fire_rating = other.fire_rating;
        }
        if other.fire_axis_distance.is_some() {
            self.fire_axis_distance = other.fire_axis_distance;
        }
        if other.stirrups_spacing.is_some() {
            self.stirrups_spacing = other.stirrups_spacing;
        }
        if let Some(theirs) = other.longitudinal {
            match self.longitudinal.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.longitudinal = Some(theirs),
            }
        }
        if let Some(theirs) = other.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.longitudinal = self.longitudinal.take().filter(|rows| !rows.is_empty());
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse_from_row(&self, row: &crate::RcMember) -> Self {
        Self {
            id: self.id.clone(),
            exposure: self.exposure.as_ref().map(|_| row.exposure.clone()),
            width: self.width.as_ref().map(|_| row.width.clone()),
            height: self.height.as_ref().map(|_| row.height.clone()),
            effective_depth: self.effective_depth.as_ref().map(|_| row.effective_depth.clone()),
            cover: self.cover.as_ref().map(|_| row.cover.clone()),
            span: self.span.as_ref().map(|_| row.span.clone()),
            fire_rating: self.fire_rating.as_ref().and_then(|_| (|| Some(row.fire.as_ref()?.rating.clone()))()),
            fire_axis_distance: self.fire_axis_distance.as_ref().and_then(|_| (|| Some(row.fire.as_ref()?.axis_distance.clone()))()),
            stirrups_spacing: self.stirrups_spacing.as_ref().and_then(|_| (|| Some(row.stirrups.as_ref()?.spacing.clone()))()),
            longitudinal: self.longitudinal.as_ref().map(|rows| rows.inverse_rows(&row.longitudinal)).filter(|rows| !rows.is_empty()),
            actions: self.actions.as_ref().map(|rows| rows.inverse_rows(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }
}

/// 🔺️ Keyed diff of `members` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersRows {
    pub added: Vec<crate::RcMember>,
    pub removed: Vec<String>,
    pub modified: Vec<En1992MembersPatch>,
    pub order: Option<Vec<String>>,
}

impl En1992MembersRows {
    fn key(row: &crate::RcMember) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::RcMember]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::RcMember]) -> Result<Vec<crate::RcMember>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::RcMember]) -> Self {
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

    fn between_rows(base: &[crate::RcMember], other: &[crate::RcMember]) -> Self {
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

/// 🩹️ Sparse patch of the `anchors` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992AnchorsPatch {
    pub id: String,
    pub h_ef: Option<f64>,
    pub a_s: Option<f64>,
}

impl En1992AnchorsPatch {
    fn apply_to_row(&self, row: &mut crate::Anchor) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.h_ef {
            row.h_ef = value.clone();
        }
        if let Some(value) = &self.a_s {
            row.a_s = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.h_ef.is_some() {
            self.h_ef = other.h_ef;
        }
        if other.a_s.is_some() {
            self.a_s = other.a_s;
        }
    }

    fn inverse_from_row(&self, row: &crate::Anchor) -> Self {
        Self {
            id: self.id.clone(),
            h_ef: self.h_ef.as_ref().map(|_| row.h_ef.clone()),
            a_s: self.a_s.as_ref().map(|_| row.a_s.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `anchors` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992AnchorsRows {
    pub added: Vec<crate::Anchor>,
    pub removed: Vec<String>,
    pub modified: Vec<En1992AnchorsPatch>,
    pub order: Option<Vec<String>>,
}

impl En1992AnchorsRows {
    fn key(row: &crate::Anchor) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::Anchor]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::Anchor]) -> Result<Vec<crate::Anchor>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::Anchor]) -> Self {
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

    fn between_rows(base: &[crate::Anchor], other: &[crate::Anchor]) -> Self {
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

/// 🔺️ Keyed sparse diff of the En1992 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub design_working_life_years: Option<f64>,
    #[state(artifact)]
    pub delta_c_dev: Option<f64>,
    #[state(artifact)]
    pub cement_type: Option<String>,
    #[state(artifact)]
    pub concrete_grades: Option<En1992ConcreteGradesRows>,
    #[state(artifact)]
    pub reinforcement_grades: Option<En1992ReinforcementGradesRows>,
    #[state(artifact)]
    pub prestress_steels: Option<En1992PrestressSteelsRows>,
    #[state(artifact)]
    pub members: Option<En1992MembersRows>,
    #[state(artifact)]
    pub anchors: Option<En1992AnchorsRows>,
}

impl protocol::MutationDiff<En1992Snapshot> for En1992Diff {
    fn apply(&self, base: &En1992Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1992Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.title {
            next.title = value.clone();
        }
        if let Some(value) = &self.design_working_life_years {
            next.design_working_life_years = value.clone();
        }
        if let Some(value) = &self.delta_c_dev {
            next.delta_c_dev = value.clone();
        }
        if let Some(value) = &self.cement_type {
            next.cement_type = value.clone();
        }
        if let Some(rows) = &self.concrete_grades {
            next.concrete_grades = rows.apply_rows(&base.concrete_grades).map_err(|error| error.under(["concrete_grades"]))?;
        }
        if let Some(rows) = &self.reinforcement_grades {
            next.reinforcement_grades = rows.apply_rows(&base.reinforcement_grades).map_err(|error| error.under(["reinforcement_grades"]))?;
        }
        if let Some(rows) = &self.prestress_steels {
            next.prestress_steels = rows.apply_rows(&base.prestress_steels).map_err(|error| error.under(["prestress_steels"]))?;
        }
        if let Some(rows) = &self.members {
            next.members = rows.apply_rows(&base.members).map_err(|error| error.under(["members"]))?;
        }
        if let Some(rows) = &self.anchors {
            next.anchors = rows.apply_rows(&base.anchors).map_err(|error| error.under(["anchors"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.title.is_some() {
            self.title = other.title;
        }
        if other.design_working_life_years.is_some() {
            self.design_working_life_years = other.design_working_life_years;
        }
        if other.delta_c_dev.is_some() {
            self.delta_c_dev = other.delta_c_dev;
        }
        if other.cement_type.is_some() {
            self.cement_type = other.cement_type;
        }
        if let Some(theirs) = other.concrete_grades {
            match self.concrete_grades.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.concrete_grades = Some(theirs),
            }
            self.concrete_grades = self.concrete_grades.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.reinforcement_grades {
            match self.reinforcement_grades.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.reinforcement_grades = Some(theirs),
            }
            self.reinforcement_grades = self.reinforcement_grades.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.prestress_steels {
            match self.prestress_steels.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.prestress_steels = Some(theirs),
            }
            self.prestress_steels = self.prestress_steels.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.members {
            match self.members.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.members = Some(theirs),
            }
            self.members = self.members.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.anchors {
            match self.anchors.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.anchors = Some(theirs),
            }
            self.anchors = self.anchors.take().filter(|rows| !rows.is_empty());
        }
    }
}

impl protocol::DiffAlgebra<En1992Snapshot> for En1992Diff {
    fn inverse(&self, base: &En1992Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            title: self.title.as_ref().map(|_| base.title.clone()),
            design_working_life_years: self.design_working_life_years.as_ref().map(|_| base.design_working_life_years.clone()),
            delta_c_dev: self.delta_c_dev.as_ref().map(|_| base.delta_c_dev.clone()),
            cement_type: self.cement_type.as_ref().map(|_| base.cement_type.clone()),
            concrete_grades: self.concrete_grades.as_ref().map(|rows| rows.inverse_rows(&base.concrete_grades)).filter(|rows| !rows.is_empty()),
            reinforcement_grades: self.reinforcement_grades.as_ref().map(|rows| rows.inverse_rows(&base.reinforcement_grades)).filter(|rows| !rows.is_empty()),
            prestress_steels: self.prestress_steels.as_ref().map(|rows| rows.inverse_rows(&base.prestress_steels)).filter(|rows| !rows.is_empty()),
            members: self.members.as_ref().map(|rows| rows.inverse_rows(&base.members)).filter(|rows| !rows.is_empty()),
            anchors: self.anchors.as_ref().map(|rows| rows.inverse_rows(&base.anchors)).filter(|rows| !rows.is_empty()),
        }
    }

    fn between(base: &En1992Snapshot, other: &En1992Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            title: (base.title != other.title).then(|| other.title.clone()),
            design_working_life_years: (base.design_working_life_years != other.design_working_life_years).then(|| other.design_working_life_years.clone()),
            delta_c_dev: (base.delta_c_dev != other.delta_c_dev).then(|| other.delta_c_dev.clone()),
            cement_type: (base.cement_type != other.cement_type).then(|| other.cement_type.clone()),
            concrete_grades: Some(En1992ConcreteGradesRows::between_rows(&base.concrete_grades, &other.concrete_grades)).filter(|rows| !rows.is_empty()),
            reinforcement_grades: Some(En1992ReinforcementGradesRows::between_rows(&base.reinforcement_grades, &other.reinforcement_grades)).filter(|rows| !rows.is_empty()),
            prestress_steels: Some(En1992PrestressSteelsRows::between_rows(&base.prestress_steels, &other.prestress_steels)).filter(|rows| !rows.is_empty()),
            members: Some(En1992MembersRows::between_rows(&base.members, &other.members)).filter(|rows| !rows.is_empty()),
            anchors: Some(En1992AnchorsRows::between_rows(&base.anchors, &other.anchors)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.title.is_none() && self.design_working_life_years.is_none() && self.delta_c_dev.is_none() && self.cement_type.is_none() && self.concrete_grades.as_ref().map_or(true, |rows| rows.is_empty()) && self.reinforcement_grades.as_ref().map_or(true, |rows| rows.is_empty()) && self.prestress_steels.as_ref().map_or(true, |rows| rows.is_empty()) && self.members.as_ref().map_or(true, |rows| rows.is_empty()) && self.anchors.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
