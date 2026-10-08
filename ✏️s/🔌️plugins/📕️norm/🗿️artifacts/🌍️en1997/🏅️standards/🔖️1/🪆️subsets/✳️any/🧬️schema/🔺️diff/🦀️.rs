//! 🧬️ En1997 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff. `DiffAlgebra::between` covers the modelled vocabulary only and exists for sync tooling, never for mutation leaves.

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("diff.target-missing", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `layers` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997LayersPatch {
    pub id: String,
    pub oedometric_modulus: Option<f64>,
    pub phi_prime_deg: Option<f64>,
}

impl En1997LayersPatch {
    fn apply_to_row(&self, row: &mut crate::SoilLayer) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.oedometric_modulus {
            row.oedometric_modulus = value.clone();
        }
        if let Some(value) = &self.phi_prime_deg {
            row.phi_prime_deg = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.oedometric_modulus.is_some() {
            self.oedometric_modulus = other.oedometric_modulus;
        }
        if other.phi_prime_deg.is_some() {
            self.phi_prime_deg = other.phi_prime_deg;
        }
    }

    fn inverse_from_row(&self, row: &crate::SoilLayer) -> Self {
        Self {
            id: self.id.clone(),
            oedometric_modulus: self.oedometric_modulus.as_ref().map(|_| row.oedometric_modulus.clone()),
            phi_prime_deg: self.phi_prime_deg.as_ref().map(|_| row.phi_prime_deg.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `layers` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997LayersRows {
    pub added: Vec<crate::SoilLayer>,
    pub removed: Vec<String>,
    pub modified: Vec<En1997LayersPatch>,
    pub order: Option<Vec<String>>,
}

impl En1997LayersRows {
    fn key(row: &crate::SoilLayer) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::SoilLayer]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::SoilLayer]) -> Result<Vec<crate::SoilLayer>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::SoilLayer]) -> Self {
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

    fn between_rows(base: &[crate::SoilLayer], other: &[crate::SoilLayer]) -> Self {
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

/// 🩹️ Sparse patch of the `footings` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997FootingsPatch {
    pub id: String,
    pub width: Option<f64>,
    pub embedment: Option<f64>,
}

impl En1997FootingsPatch {
    fn apply_to_row(&self, row: &mut crate::SpreadFoundation) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.width {
            row.width = value.clone();
        }
        if let Some(value) = &self.embedment {
            row.embedment = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.embedment.is_some() {
            self.embedment = other.embedment;
        }
    }

    fn inverse_from_row(&self, row: &crate::SpreadFoundation) -> Self {
        Self {
            id: self.id.clone(),
            width: self.width.as_ref().map(|_| row.width.clone()),
            embedment: self.embedment.as_ref().map(|_| row.embedment.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `footings` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997FootingsRows {
    pub added: Vec<crate::SpreadFoundation>,
    pub removed: Vec<String>,
    pub modified: Vec<En1997FootingsPatch>,
    pub order: Option<Vec<String>>,
}

impl En1997FootingsRows {
    fn key(row: &crate::SpreadFoundation) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::SpreadFoundation]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::SpreadFoundation]) -> Result<Vec<crate::SpreadFoundation>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::SpreadFoundation]) -> Self {
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

    fn between_rows(base: &[crate::SpreadFoundation], other: &[crate::SpreadFoundation]) -> Self {
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

/// 🩹️ Sparse patch of the `piles` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997PilesPatch {
    pub id: String,
    pub length: Option<f64>,
    pub count: Option<u32>,
}

impl En1997PilesPatch {
    fn apply_to_row(&self, row: &mut crate::Pile) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.length {
            row.length = value.clone();
        }
        if let Some(value) = &self.count {
            row.count = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.length.is_some() {
            self.length = other.length;
        }
        if other.count.is_some() {
            self.count = other.count;
        }
    }

    fn inverse_from_row(&self, row: &crate::Pile) -> Self {
        Self {
            id: self.id.clone(),
            length: self.length.as_ref().map(|_| row.length.clone()),
            count: self.count.as_ref().map(|_| row.count.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `piles` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997PilesRows {
    pub added: Vec<crate::Pile>,
    pub removed: Vec<String>,
    pub modified: Vec<En1997PilesPatch>,
    pub order: Option<Vec<String>>,
}

impl En1997PilesRows {
    fn key(row: &crate::Pile) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::Pile]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::Pile]) -> Result<Vec<crate::Pile>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::Pile]) -> Self {
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

    fn between_rows(base: &[crate::Pile], other: &[crate::Pile]) -> Self {
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

/// 🩹️ Sparse patch of the `retaining_walls` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997RetainingWallsPatch {
    pub id: String,
    pub base_width: Option<f64>,
}

impl En1997RetainingWallsPatch {
    fn apply_to_row(&self, row: &mut crate::RetainingWall) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.base_width {
            row.base_width = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.base_width.is_some() {
            self.base_width = other.base_width;
        }
    }

    fn inverse_from_row(&self, row: &crate::RetainingWall) -> Self {
        Self {
            id: self.id.clone(),
            base_width: self.base_width.as_ref().map(|_| row.base_width.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `retaining_walls` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997RetainingWallsRows {
    pub added: Vec<crate::RetainingWall>,
    pub removed: Vec<String>,
    pub modified: Vec<En1997RetainingWallsPatch>,
    pub order: Option<Vec<String>>,
}

impl En1997RetainingWallsRows {
    fn key(row: &crate::RetainingWall) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::RetainingWall]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::RetainingWall]) -> Result<Vec<crate::RetainingWall>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::RetainingWall]) -> Self {
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

    fn between_rows(base: &[crate::RetainingWall], other: &[crate::RetainingWall]) -> Self {
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

/// 🩹️ Sparse patch of the `slopes` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997SlopesPatch {
    pub id: String,
    pub angle_deg: Option<f64>,
}

impl En1997SlopesPatch {
    fn apply_to_row(&self, row: &mut crate::Slope) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.angle_deg {
            row.angle_deg = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.angle_deg.is_some() {
            self.angle_deg = other.angle_deg;
        }
    }

    fn inverse_from_row(&self, row: &crate::Slope) -> Self {
        Self {
            id: self.id.clone(),
            angle_deg: self.angle_deg.as_ref().map(|_| row.angle_deg.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `slopes` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997SlopesRows {
    pub added: Vec<crate::Slope>,
    pub removed: Vec<String>,
    pub modified: Vec<En1997SlopesPatch>,
    pub order: Option<Vec<String>>,
}

impl En1997SlopesRows {
    fn key(row: &crate::Slope) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::Slope]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::Slope]) -> Result<Vec<crate::Slope>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::Slope]) -> Self {
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

    fn between_rows(base: &[crate::Slope], other: &[crate::Slope]) -> Self {
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

/// 🔺️ Keyed diff of `uplift_cases` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997UpliftCasesRows {
    pub added: Vec<crate::UpliftCase>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl En1997UpliftCasesRows {
    fn key(row: &crate::UpliftCase) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::UpliftCase]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::UpliftCase]) -> Result<Vec<crate::UpliftCase>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::UpliftCase]) -> Self {
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

    fn between_rows(base: &[crate::UpliftCase], other: &[crate::UpliftCase]) -> Self {
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

/// 🔺️ Keyed sparse diff of the En1997 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Diff {
    #[state(artifact)]
    pub structure_id: Option<String>,
    #[state(artifact)]
    pub geotechnical_category: Option<u8>,
    #[state(artifact)]
    pub design_situation: Option<String>,
    #[state(artifact)]
    pub design_approach: Option<String>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub groundwater_level: Option<f64>,
    #[state(artifact)]
    pub investigation_depth: Option<f64>,
    #[state(artifact)]
    pub layers: Option<En1997LayersRows>,
    #[state(artifact)]
    pub footings: Option<En1997FootingsRows>,
    #[state(artifact)]
    pub piles: Option<En1997PilesRows>,
    #[state(artifact)]
    pub retaining_walls: Option<En1997RetainingWallsRows>,
    #[state(artifact)]
    pub slopes: Option<En1997SlopesRows>,
    #[state(artifact)]
    pub uplift_cases: Option<En1997UpliftCasesRows>,
}

impl protocol::MutationDiff<En1997Snapshot> for En1997Diff {
    fn apply(&self, base: &En1997Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1997Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.structure_id {
            next.structure_id = value.clone();
        }
        if let Some(value) = &self.geotechnical_category {
            next.geotechnical_category = value.clone();
        }
        if let Some(value) = &self.design_situation {
            next.design_situation = value.clone();
        }
        if let Some(value) = &self.design_approach {
            next.design_approach = value.clone();
        }
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.groundwater_level {
            next.groundwater_level = value.clone();
        }
        if let Some(value) = &self.investigation_depth {
            next.investigation_depth = value.clone();
        }
        if let Some(rows) = &self.layers {
            next.layers = rows.apply_rows(&base.layers).map_err(|error| error.under(["layers"]))?;
        }
        if let Some(rows) = &self.footings {
            next.footings = rows.apply_rows(&base.footings).map_err(|error| error.under(["footings"]))?;
        }
        if let Some(rows) = &self.piles {
            next.piles = rows.apply_rows(&base.piles).map_err(|error| error.under(["piles"]))?;
        }
        if let Some(rows) = &self.retaining_walls {
            next.retaining_walls = rows.apply_rows(&base.retaining_walls).map_err(|error| error.under(["retaining_walls"]))?;
        }
        if let Some(rows) = &self.slopes {
            next.slopes = rows.apply_rows(&base.slopes).map_err(|error| error.under(["slopes"]))?;
        }
        if let Some(rows) = &self.uplift_cases {
            next.uplift_cases = rows.apply_rows(&base.uplift_cases).map_err(|error| error.under(["uplift_cases"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.structure_id.is_some() {
            self.structure_id = other.structure_id;
        }
        if other.geotechnical_category.is_some() {
            self.geotechnical_category = other.geotechnical_category;
        }
        if other.design_situation.is_some() {
            self.design_situation = other.design_situation;
        }
        if other.design_approach.is_some() {
            self.design_approach = other.design_approach;
        }
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.groundwater_level.is_some() {
            self.groundwater_level = other.groundwater_level;
        }
        if other.investigation_depth.is_some() {
            self.investigation_depth = other.investigation_depth;
        }
        if let Some(theirs) = other.layers {
            match self.layers.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.layers = Some(theirs),
            }
            self.layers = self.layers.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.footings {
            match self.footings.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.footings = Some(theirs),
            }
            self.footings = self.footings.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.piles {
            match self.piles.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.piles = Some(theirs),
            }
            self.piles = self.piles.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.retaining_walls {
            match self.retaining_walls.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.retaining_walls = Some(theirs),
            }
            self.retaining_walls = self.retaining_walls.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.slopes {
            match self.slopes.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.slopes = Some(theirs),
            }
            self.slopes = self.slopes.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.uplift_cases {
            match self.uplift_cases.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.uplift_cases = Some(theirs),
            }
            self.uplift_cases = self.uplift_cases.take().filter(|rows| !rows.is_empty());
        }
    }
}

impl protocol::DiffAlgebra<En1997Snapshot> for En1997Diff {
    fn inverse(&self, base: &En1997Snapshot) -> Self {
        Self {
            structure_id: self.structure_id.as_ref().map(|_| base.structure_id.clone()),
            geotechnical_category: self.geotechnical_category.as_ref().map(|_| base.geotechnical_category.clone()),
            design_situation: self.design_situation.as_ref().map(|_| base.design_situation.clone()),
            design_approach: self.design_approach.as_ref().map(|_| base.design_approach.clone()),
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            groundwater_level: self.groundwater_level.as_ref().map(|_| base.groundwater_level.clone()),
            investigation_depth: self.investigation_depth.as_ref().map(|_| base.investigation_depth.clone()),
            layers: self.layers.as_ref().map(|rows| rows.inverse_rows(&base.layers)).filter(|rows| !rows.is_empty()),
            footings: self.footings.as_ref().map(|rows| rows.inverse_rows(&base.footings)).filter(|rows| !rows.is_empty()),
            piles: self.piles.as_ref().map(|rows| rows.inverse_rows(&base.piles)).filter(|rows| !rows.is_empty()),
            retaining_walls: self.retaining_walls.as_ref().map(|rows| rows.inverse_rows(&base.retaining_walls)).filter(|rows| !rows.is_empty()),
            slopes: self.slopes.as_ref().map(|rows| rows.inverse_rows(&base.slopes)).filter(|rows| !rows.is_empty()),
            uplift_cases: self.uplift_cases.as_ref().map(|rows| rows.inverse_rows(&base.uplift_cases)).filter(|rows| !rows.is_empty()),
        }
    }

    fn between(base: &En1997Snapshot, other: &En1997Snapshot) -> Self {
        Self {
            structure_id: (base.structure_id != other.structure_id).then(|| other.structure_id.clone()),
            geotechnical_category: (base.geotechnical_category != other.geotechnical_category).then(|| other.geotechnical_category.clone()),
            design_situation: (base.design_situation != other.design_situation).then(|| other.design_situation.clone()),
            design_approach: (base.design_approach != other.design_approach).then(|| other.design_approach.clone()),
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            groundwater_level: (base.groundwater_level != other.groundwater_level).then(|| other.groundwater_level.clone()),
            investigation_depth: (base.investigation_depth != other.investigation_depth).then(|| other.investigation_depth.clone()),
            layers: Some(En1997LayersRows::between_rows(&base.layers, &other.layers)).filter(|rows| !rows.is_empty()),
            footings: Some(En1997FootingsRows::between_rows(&base.footings, &other.footings)).filter(|rows| !rows.is_empty()),
            piles: Some(En1997PilesRows::between_rows(&base.piles, &other.piles)).filter(|rows| !rows.is_empty()),
            retaining_walls: Some(En1997RetainingWallsRows::between_rows(&base.retaining_walls, &other.retaining_walls)).filter(|rows| !rows.is_empty()),
            slopes: Some(En1997SlopesRows::between_rows(&base.slopes, &other.slopes)).filter(|rows| !rows.is_empty()),
            uplift_cases: Some(En1997UpliftCasesRows::between_rows(&base.uplift_cases, &other.uplift_cases)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.structure_id.is_none() && self.geotechnical_category.is_none() && self.design_situation.is_none() && self.design_approach.is_none() && self.annex.is_none() && self.groundwater_level.is_none() && self.investigation_depth.is_none() && self.layers.as_ref().map_or(true, |rows| rows.is_empty()) && self.footings.as_ref().map_or(true, |rows| rows.is_empty()) && self.piles.as_ref().map_or(true, |rows| rows.is_empty()) && self.retaining_walls.as_ref().map_or(true, |rows| rows.is_empty()) && self.slopes.as_ref().map_or(true, |rows| rows.is_empty()) && self.uplift_cases.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
