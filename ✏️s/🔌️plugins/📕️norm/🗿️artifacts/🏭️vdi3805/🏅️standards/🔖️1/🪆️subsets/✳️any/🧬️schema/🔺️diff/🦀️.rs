//! 🧬️ Vdi3805 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff. `DiffAlgebra::between` covers the modelled vocabulary only and exists for sync tooling, never for mutation leaves.

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("diff.target-missing", format!("{what} does not exist"))
}

/// 🩹️ Sparse per-field patch of the `catalog.file` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805ManufacturerFilePatch {
    pub header_version: Option<String>,
    pub manufacturer: Option<String>,
    pub building_system_number: Option<crate::BuildingSystemNumber>,
    pub created: Option<String>,
    pub charset: Option<String>,
    pub record_count: Option<u32>,
    pub extensions: Option<crate::ExtensionBag>,
}

impl Vdi3805ManufacturerFilePatch {
    fn apply_to_row(&self, row: &mut crate::ManufacturerFile) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.header_version {
            row.header_version = value.clone();
        }
        if let Some(value) = &self.manufacturer {
            row.manufacturer = value.clone();
        }
        if let Some(value) = &self.building_system_number {
            row.building_system_number = value.clone();
        }
        if let Some(value) = &self.created {
            row.created = value.clone();
        }
        if let Some(value) = &self.charset {
            row.charset = value.clone();
        }
        if let Some(value) = &self.record_count {
            row.record_count = value.clone();
        }
        if let Some(value) = &self.extensions {
            row.extensions = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.header_version.is_some() {
            self.header_version = other.header_version;
        }
        if other.manufacturer.is_some() {
            self.manufacturer = other.manufacturer;
        }
        if other.building_system_number.is_some() {
            self.building_system_number = other.building_system_number;
        }
        if other.created.is_some() {
            self.created = other.created;
        }
        if other.charset.is_some() {
            self.charset = other.charset;
        }
        if other.record_count.is_some() {
            self.record_count = other.record_count;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
    }

    fn inverse_from_row(&self, row: &crate::ManufacturerFile) -> Self {
        Self {
            header_version: self.header_version.as_ref().map(|_| row.header_version.clone()),
            manufacturer: self.manufacturer.as_ref().map(|_| row.manufacturer.clone()),
            building_system_number: self.building_system_number.as_ref().map(|_| row.building_system_number.clone()),
            created: self.created.as_ref().map(|_| row.created.clone()),
            charset: self.charset.as_ref().map(|_| row.charset.clone()),
            record_count: self.record_count.as_ref().map(|_| row.record_count.clone()),
            extensions: self.extensions.as_ref().map(|_| row.extensions.clone()),
        }
    }
}

/// 🩹️ Sparse patch of the `catalog.products` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805ProductsPatch {
    pub id: String,
    pub title: Option<Vec<crate::document::LocalizedText>>,
    pub configuration: Option<crate::Configuration>,
}

impl Vdi3805ProductsPatch {
    fn apply_to_row(&self, row: &mut crate::CatalogueProduct) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.title {
            row.title = value.clone();
        }
        if let Some(value) = &self.configuration {
            row.configuration = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.title.is_some() {
            self.title = other.title;
        }
        if other.configuration.is_some() {
            self.configuration = other.configuration;
        }
    }

    fn inverse_from_row(&self, row: &crate::CatalogueProduct) -> Self {
        Self {
            id: self.id.clone(),
            title: self.title.as_ref().map(|_| row.title.clone()),
            configuration: self.configuration.as_ref().map(|_| row.configuration.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `catalog.products` rows (by `identity.article_number`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805ProductsRows {
    pub added: Vec<crate::CatalogueProduct>,
    pub removed: Vec<String>,
    pub modified: Vec<Vdi3805ProductsPatch>,
    pub order: Option<Vec<String>>,
}

impl Vdi3805ProductsRows {
    fn key(row: &crate::CatalogueProduct) -> &str {
        row.identity.article_number.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::CatalogueProduct]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::CatalogueProduct]) -> Result<Vec<crate::CatalogueProduct>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::CatalogueProduct]) -> Self {
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

    fn between_rows(base: &[crate::CatalogueProduct], other: &[crate::CatalogueProduct]) -> Self {
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

/// 🔑️ A `edition_profile` entry: its map key and value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805EditionProfileEntry {
    pub key: String,
    pub value: crate::EditionProfileChoice,
}

/// 🔺️ Keyed diff of the `edition_profile` map: added entries, removed keys and modified values.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805EditionProfileRows {
    pub added: Vec<Vdi3805EditionProfileEntry>,
    pub removed: Vec<String>,
    pub modified: Vec<Vdi3805EditionProfileEntry>,
}

impl Vdi3805EditionProfileRows {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }

    fn apply_rows(&self, base: &std::collections::BTreeMap<String, crate::EditionProfileChoice>) -> Result<std::collections::BTreeMap<String, crate::EditionProfileChoice>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &std::collections::BTreeMap<String, crate::EditionProfileChoice>) -> Self {
        Self {
            removed: self.added.iter().map(|entry| entry.key.clone()).collect(),
            added: self.removed.iter().filter_map(|key| base.get(key).map(|value| Vdi3805EditionProfileEntry { key: key.clone(), value: value.clone() })).collect(),
            modified: self.modified.iter().filter_map(|entry| base.get(&entry.key).map(|value| Vdi3805EditionProfileEntry { key: entry.key.clone(), value: value.clone() })).collect(),
        }
    }

    fn between_rows(base: &std::collections::BTreeMap<String, crate::EditionProfileChoice>, other: &std::collections::BTreeMap<String, crate::EditionProfileChoice>) -> Self {
        Self {
            removed: base.iter().filter(|(key, value)| other.get(*key) != Some(*value)).map(|(key, _)| key.clone()).collect(),
            added: other.iter().filter(|(key, value)| base.get(*key) != Some(*value)).map(|(key, value)| Vdi3805EditionProfileEntry { key: key.clone(), value: value.clone() }).collect(),
            modified: Vec::new(),
        }
    }
}

/// 🩹️ Sparse patch of the `index.entries` row addressed by `id`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805IndexEntriesPatch {
    pub id: String,
    pub dn: Option<Vdi3805IndexEntriesPatchDnValue>,
    pub tags: Option<Vec<String>>,
}

impl Vdi3805IndexEntriesPatch {
    fn apply_to_row(&self, row: &mut crate::CatalogIndexEntry) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.dn {
            row.dn = value.value.clone();
        }
        if let Some(value) = &self.tags {
            row.tags = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.dn.is_some() {
            self.dn = other.dn;
        }
        if other.tags.is_some() {
            self.tags = other.tags;
        }
    }

    fn inverse_from_row(&self, row: &crate::CatalogIndexEntry) -> Self {
        Self {
            id: self.id.clone(),
            dn: self.dn.as_ref().map(|_| Vdi3805IndexEntriesPatchDnValue { value: row.dn.clone() }),
            tags: self.tags.as_ref().map(|_| row.tags.clone()),
        }
    }
}

/// 🔺️ Keyed diff of `index.entries` rows (by `product_id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805IndexEntriesRows {
    pub added: Vec<crate::CatalogIndexEntry>,
    pub removed: Vec<String>,
    pub modified: Vec<Vdi3805IndexEntriesPatch>,
    pub order: Option<Vec<String>>,
}

impl Vdi3805IndexEntriesRows {
    fn key(row: &crate::CatalogIndexEntry) -> &str {
        row.product_id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none() && self.modified.is_empty()
    }

    fn ids(rows: &[crate::CatalogIndexEntry]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::CatalogIndexEntry]) -> Result<Vec<crate::CatalogIndexEntry>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::CatalogIndexEntry]) -> Self {
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

    fn between_rows(base: &[crate::CatalogIndexEntry], other: &[crate::CatalogIndexEntry]) -> Self {
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

/// 🔺️ Keyed diff of `connections` rows (by `id`): added rows, removed ids, modified row patches and the resulting id order when it deviates from base order minus removed plus added.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805GeometryConnectionsRows {
    pub added: Vec<crate::ConnectionPoint>,
    pub removed: Vec<String>,
    pub order: Option<Vec<String>>,
}

impl Vdi3805GeometryConnectionsRows {
    fn key(row: &crate::ConnectionPoint) -> &str {
        row.id.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.order.is_none()
    }

    fn ids(rows: &[crate::ConnectionPoint]) -> Vec<String> {
        rows.iter().map(|row| Self::key(row).to_string()).collect()
    }

    fn apply_rows(&self, base: &[crate::ConnectionPoint]) -> Result<Vec<crate::ConnectionPoint>, protocol::MutationApplyError> {
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

    fn inverse_rows(&self, base: &[crate::ConnectionPoint]) -> Self {
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

    fn between_rows(base: &[crate::ConnectionPoint], other: &[crate::ConnectionPoint]) -> Self {
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

/// 🔑️ A `geometry` entry: its map key and value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805GeometryEntry {
    pub key: String,
    pub value: crate::ParametricGeometry,
}

/// 🩹️ Sparse patch of the `geometry` entry addressed by `key`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805GeometryPatch {
    pub key: String,
    pub bbox: Option<crate::BoundingBox>,
    pub parameters: Option<std::collections::BTreeMap<String, f64>>,
    pub connections: Option<Vdi3805GeometryConnectionsRows>,
}

impl Vdi3805GeometryPatch {
    fn apply_to_row(&self, row: &mut crate::ParametricGeometry) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.bbox {
            row.bbox = value.clone();
        }
        if let Some(value) = &self.parameters {
            row.parameters = value.clone();
        }
        if let Some(rows) = &self.connections {
            row.connections = rows.apply_rows(&row.connections).map_err(|error| error.under(["connections"]))?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.bbox.is_some() {
            self.bbox = other.bbox;
        }
        if other.parameters.is_some() {
            self.parameters = other.parameters;
        }
        if let Some(theirs) = other.connections {
            match self.connections.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.connections = Some(theirs),
            }
        }
        self.connections = self.connections.take().filter(|rows| !rows.is_empty());
    }

    fn inverse_from_row(&self, row: &crate::ParametricGeometry) -> Self {
        Self {
            key: self.key.clone(),
            bbox: self.bbox.as_ref().map(|_| row.bbox.clone()),
            parameters: self.parameters.as_ref().map(|_| row.parameters.clone()),
            connections: self.connections.as_ref().map(|rows| rows.inverse_rows(&row.connections)).filter(|rows| !rows.is_empty()),
        }
    }
}

/// 🔺️ Keyed diff of the `geometry` map: added entries, removed keys and modified entry patches.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805GeometryRows {
    pub added: Vec<Vdi3805GeometryEntry>,
    pub removed: Vec<String>,
    pub modified: Vec<Vdi3805GeometryPatch>,
}

impl Vdi3805GeometryRows {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }

    fn apply_rows(&self, base: &std::collections::BTreeMap<String, crate::ParametricGeometry>) -> Result<std::collections::BTreeMap<String, crate::ParametricGeometry>, protocol::MutationApplyError> {
        let mut map = base.clone();
        for key in &self.removed {
            map.remove(key).ok_or_else(|| missing_target(format!("removed entry \"{key}\"")).at([key.clone()]))?;
        }
        for patch in &self.modified {
            let value = map.get_mut(&patch.key).ok_or_else(|| missing_target(format!("modified entry \"{}\"", patch.key)).at([patch.key.clone()]))?;
            patch.apply_to_row(value)?;
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
        for patch in other.modified {
            if let Some(at) = self.added.iter().position(|added| added.key == patch.key) {
                if patch.apply_to_row(&mut self.added[at].value).is_err() {
                    self.modified.push(patch);
                }
            } else if let Some(existing) = self.modified.iter_mut().find(|existing| existing.key == patch.key) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
    }

    fn inverse_rows(&self, base: &std::collections::BTreeMap<String, crate::ParametricGeometry>) -> Self {
        Self {
            removed: self.added.iter().map(|entry| entry.key.clone()).collect(),
            added: self.removed.iter().filter_map(|key| base.get(key).map(|value| Vdi3805GeometryEntry { key: key.clone(), value: value.clone() })).collect(),
            modified: self.modified.iter().filter_map(|patch| base.get(&patch.key).map(|value| patch.inverse_from_row(value))).collect(),
        }
    }

    fn between_rows(base: &std::collections::BTreeMap<String, crate::ParametricGeometry>, other: &std::collections::BTreeMap<String, crate::ParametricGeometry>) -> Self {
        Self {
            removed: base.iter().filter(|(key, value)| other.get(*key) != Some(*value)).map(|(key, _)| key.clone()).collect(),
            added: other.iter().filter(|(key, value)| base.get(*key) != Some(*value)).map(|(key, value)| Vdi3805GeometryEntry { key: key.clone(), value: value.clone() }).collect(),
            modified: Vec::new(),
        }
    }
}

/// 🔑️ A `curves` entry: its map key and value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805CurvesEntry {
    pub key: String,
    pub value: crate::CharacteristicCurve,
}

/// 🩹️ Sparse patch of the `curves` entry addressed by `key`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805CurvesPatch {
    pub key: String,
    pub points: Option<Vec<crate::CurvePoint>>,
}

impl Vdi3805CurvesPatch {
    fn apply_to_row(&self, row: &mut crate::CharacteristicCurve) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.points {
            row.points = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.points.is_some() {
            self.points = other.points;
        }
    }

    fn inverse_from_row(&self, row: &crate::CharacteristicCurve) -> Self {
        Self {
            key: self.key.clone(),
            points: self.points.as_ref().map(|_| row.points.clone()),
        }
    }
}

/// 🔺️ Keyed diff of the `curves` map: added entries, removed keys and modified entry patches.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805CurvesRows {
    pub added: Vec<Vdi3805CurvesEntry>,
    pub removed: Vec<String>,
    pub modified: Vec<Vdi3805CurvesPatch>,
}

impl Vdi3805CurvesRows {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }

    fn apply_rows(&self, base: &std::collections::BTreeMap<String, crate::CharacteristicCurve>) -> Result<std::collections::BTreeMap<String, crate::CharacteristicCurve>, protocol::MutationApplyError> {
        let mut map = base.clone();
        for key in &self.removed {
            map.remove(key).ok_or_else(|| missing_target(format!("removed entry \"{key}\"")).at([key.clone()]))?;
        }
        for patch in &self.modified {
            let value = map.get_mut(&patch.key).ok_or_else(|| missing_target(format!("modified entry \"{}\"", patch.key)).at([patch.key.clone()]))?;
            patch.apply_to_row(value)?;
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
        for patch in other.modified {
            if let Some(at) = self.added.iter().position(|added| added.key == patch.key) {
                if patch.apply_to_row(&mut self.added[at].value).is_err() {
                    self.modified.push(patch);
                }
            } else if let Some(existing) = self.modified.iter_mut().find(|existing| existing.key == patch.key) {
                existing.merge(patch);
            } else {
                self.modified.push(patch);
            }
        }
    }

    fn inverse_rows(&self, base: &std::collections::BTreeMap<String, crate::CharacteristicCurve>) -> Self {
        Self {
            removed: self.added.iter().map(|entry| entry.key.clone()).collect(),
            added: self.removed.iter().filter_map(|key| base.get(key).map(|value| Vdi3805CurvesEntry { key: key.clone(), value: value.clone() })).collect(),
            modified: self.modified.iter().filter_map(|patch| base.get(&patch.key).map(|value| patch.inverse_from_row(value))).collect(),
        }
    }

    fn between_rows(base: &std::collections::BTreeMap<String, crate::CharacteristicCurve>, other: &std::collections::BTreeMap<String, crate::CharacteristicCurve>) -> Self {
        Self {
            removed: base.iter().filter(|(key, value)| other.get(*key) != Some(*value)).map(|(key, _)| key.clone()).collect(),
            added: other.iter().filter(|(key, value)| base.get(*key) != Some(*value)).map(|(key, value)| Vdi3805CurvesEntry { key: key.clone(), value: value.clone() }).collect(),
            modified: Vec::new(),
        }
    }
}

/// 🩹️ Sparse per-field patch of the `limits` section.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805LimitsPatch {
    pub max_file_bytes: Option<usize>,
    pub max_records: Option<usize>,
    pub max_field_length: Option<usize>,
    pub max_nesting_depth: Option<usize>,
}

impl Vdi3805LimitsPatch {
    fn apply_to_row(&self, row: &mut crate::SecurityLimits) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.max_file_bytes {
            row.max_file_bytes = value.clone();
        }
        if let Some(value) = &self.max_records {
            row.max_records = value.clone();
        }
        if let Some(value) = &self.max_field_length {
            row.max_field_length = value.clone();
        }
        if let Some(value) = &self.max_nesting_depth {
            row.max_nesting_depth = value.clone();
        }
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        if other.max_file_bytes.is_some() {
            self.max_file_bytes = other.max_file_bytes;
        }
        if other.max_records.is_some() {
            self.max_records = other.max_records;
        }
        if other.max_field_length.is_some() {
            self.max_field_length = other.max_field_length;
        }
        if other.max_nesting_depth.is_some() {
            self.max_nesting_depth = other.max_nesting_depth;
        }
    }

    fn inverse_from_row(&self, row: &crate::SecurityLimits) -> Self {
        Self {
            max_file_bytes: self.max_file_bytes.as_ref().map(|_| row.max_file_bytes.clone()),
            max_records: self.max_records.as_ref().map(|_| row.max_records.clone()),
            max_field_length: self.max_field_length.as_ref().map(|_| row.max_field_length.clone()),
            max_nesting_depth: self.max_nesting_depth.as_ref().map(|_| row.max_nesting_depth.clone()),
        }
    }
}

/// 🎁️ Carries the optional `CatalogIndexEntry.dn` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805IndexEntriesPatchDnValue {
    pub value: Option<u16>,
}

/// 🔺️ Keyed sparse diff of the Vdi3805 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Diff {
    #[state(artifact)]
    pub manufacturer_file: Option<Vdi3805ManufacturerFilePatch>,
    #[state(artifact)]
    pub products: Option<Vdi3805ProductsRows>,
    #[state(artifact)]
    pub catalog_extensions: Option<crate::ExtensionBag>,
    #[state(artifact)]
    pub edition_profile: Option<Vdi3805EditionProfileRows>,
    #[state(artifact)]
    pub correction_as_of: Option<crate::EditionId>,
    #[state(artifact)]
    pub strict_mode: Option<bool>,
    #[state(artifact)]
    pub index_entries: Option<Vdi3805IndexEntriesRows>,
    #[state(artifact)]
    pub geometry: Option<Vdi3805GeometryRows>,
    #[state(artifact)]
    pub curves: Option<Vdi3805CurvesRows>,
    #[state(artifact)]
    pub limits: Option<Vdi3805LimitsPatch>,
}

impl protocol::MutationDiff<Vdi3805Snapshot> for Vdi3805Diff {
    fn apply(&self, base: &Vdi3805Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Vdi3805Snapshot> {
        let mut next = base.clone();
        if let Some(patch) = &self.manufacturer_file {
            patch.apply_to_row(&mut next.catalog.file).map_err(|error| error.under(["manufacturer_file"]))?;
        }
        if let Some(rows) = &self.products {
            next.catalog.products = rows.apply_rows(&base.catalog.products).map_err(|error| error.under(["products"]))?;
        }
        if let Some(value) = &self.catalog_extensions {
            next.catalog.extensions = value.clone();
        }
        if let Some(rows) = &self.edition_profile {
            next.edition_profile = rows.apply_rows(&base.edition_profile).map_err(|error| error.under(["edition_profile"]))?;
        }
        if let Some(value) = &self.correction_as_of {
            next.correction_as_of = value.clone();
        }
        if let Some(value) = &self.strict_mode {
            next.strict_mode = value.clone();
        }
        if let Some(rows) = &self.index_entries {
            next.index.entries = rows.apply_rows(&base.index.entries).map_err(|error| error.under(["index_entries"]))?;
        }
        if let Some(rows) = &self.geometry {
            next.geometry = rows.apply_rows(&base.geometry).map_err(|error| error.under(["geometry"]))?;
        }
        if let Some(rows) = &self.curves {
            next.curves = rows.apply_rows(&base.curves).map_err(|error| error.under(["curves"]))?;
        }
        if let Some(patch) = &self.limits {
            patch.apply_to_row(&mut next.limits).map_err(|error| error.under(["limits"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if let Some(theirs) = other.manufacturer_file {
            match self.manufacturer_file.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.manufacturer_file = Some(theirs),
            }
        }
        if let Some(theirs) = other.products {
            match self.products.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.products = Some(theirs),
            }
            self.products = self.products.take().filter(|rows| !rows.is_empty());
        }
        if other.catalog_extensions.is_some() {
            self.catalog_extensions = other.catalog_extensions;
        }
        if let Some(theirs) = other.edition_profile {
            match self.edition_profile.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.edition_profile = Some(theirs),
            }
            self.edition_profile = self.edition_profile.take().filter(|rows| !rows.is_empty());
        }
        if other.correction_as_of.is_some() {
            self.correction_as_of = other.correction_as_of;
        }
        if other.strict_mode.is_some() {
            self.strict_mode = other.strict_mode;
        }
        if let Some(theirs) = other.index_entries {
            match self.index_entries.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.index_entries = Some(theirs),
            }
            self.index_entries = self.index_entries.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.geometry {
            match self.geometry.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.geometry = Some(theirs),
            }
            self.geometry = self.geometry.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.curves {
            match self.curves.as_mut() {
                Some(mine) => mine.absorb_rows(theirs),
                None => self.curves = Some(theirs),
            }
            self.curves = self.curves.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.limits {
            match self.limits.as_mut() {
                Some(mine) => mine.merge(theirs),
                None => self.limits = Some(theirs),
            }
        }
    }
}

impl protocol::DiffAlgebra<Vdi3805Snapshot> for Vdi3805Diff {
    fn inverse(&self, base: &Vdi3805Snapshot) -> Self {
        Self {
            manufacturer_file: self.manufacturer_file.as_ref().map(|patch| patch.inverse_from_row(&base.catalog.file)),
            products: self.products.as_ref().map(|rows| rows.inverse_rows(&base.catalog.products)).filter(|rows| !rows.is_empty()),
            catalog_extensions: self.catalog_extensions.as_ref().map(|_| base.catalog.extensions.clone()),
            edition_profile: self.edition_profile.as_ref().map(|rows| rows.inverse_rows(&base.edition_profile)).filter(|rows| !rows.is_empty()),
            correction_as_of: self.correction_as_of.as_ref().map(|_| base.correction_as_of.clone()),
            strict_mode: self.strict_mode.as_ref().map(|_| base.strict_mode.clone()),
            index_entries: self.index_entries.as_ref().map(|rows| rows.inverse_rows(&base.index.entries)).filter(|rows| !rows.is_empty()),
            geometry: self.geometry.as_ref().map(|rows| rows.inverse_rows(&base.geometry)).filter(|rows| !rows.is_empty()),
            curves: self.curves.as_ref().map(|rows| rows.inverse_rows(&base.curves)).filter(|rows| !rows.is_empty()),
            limits: self.limits.as_ref().map(|patch| patch.inverse_from_row(&base.limits)),
        }
    }

    fn between(base: &Vdi3805Snapshot, other: &Vdi3805Snapshot) -> Self {
        Self {
            manufacturer_file: Some(Vdi3805ManufacturerFilePatch {
                header_version: (base.catalog.file.header_version != other.catalog.file.header_version).then(|| other.catalog.file.header_version.clone()),
                manufacturer: (base.catalog.file.manufacturer != other.catalog.file.manufacturer).then(|| other.catalog.file.manufacturer.clone()),
                building_system_number: (base.catalog.file.building_system_number != other.catalog.file.building_system_number).then(|| other.catalog.file.building_system_number.clone()),
                created: (base.catalog.file.created != other.catalog.file.created).then(|| other.catalog.file.created.clone()),
                charset: (base.catalog.file.charset != other.catalog.file.charset).then(|| other.catalog.file.charset.clone()),
                record_count: (base.catalog.file.record_count != other.catalog.file.record_count).then(|| other.catalog.file.record_count.clone()),
                extensions: (base.catalog.file.extensions != other.catalog.file.extensions).then(|| other.catalog.file.extensions.clone()),
            }).filter(|patch| *patch != Vdi3805ManufacturerFilePatch::default()),
            products: Some(Vdi3805ProductsRows::between_rows(&base.catalog.products, &other.catalog.products)).filter(|rows| !rows.is_empty()),
            catalog_extensions: (base.catalog.extensions != other.catalog.extensions).then(|| other.catalog.extensions.clone()),
            edition_profile: Some(Vdi3805EditionProfileRows::between_rows(&base.edition_profile, &other.edition_profile)).filter(|rows| !rows.is_empty()),
            correction_as_of: (base.correction_as_of != other.correction_as_of).then(|| other.correction_as_of.clone()),
            strict_mode: (base.strict_mode != other.strict_mode).then(|| other.strict_mode.clone()),
            index_entries: Some(Vdi3805IndexEntriesRows::between_rows(&base.index.entries, &other.index.entries)).filter(|rows| !rows.is_empty()),
            geometry: Some(Vdi3805GeometryRows::between_rows(&base.geometry, &other.geometry)).filter(|rows| !rows.is_empty()),
            curves: Some(Vdi3805CurvesRows::between_rows(&base.curves, &other.curves)).filter(|rows| !rows.is_empty()),
            limits: Some(Vdi3805LimitsPatch {
                max_file_bytes: (base.limits.max_file_bytes != other.limits.max_file_bytes).then(|| other.limits.max_file_bytes.clone()),
                max_records: (base.limits.max_records != other.limits.max_records).then(|| other.limits.max_records.clone()),
                max_field_length: (base.limits.max_field_length != other.limits.max_field_length).then(|| other.limits.max_field_length.clone()),
                max_nesting_depth: (base.limits.max_nesting_depth != other.limits.max_nesting_depth).then(|| other.limits.max_nesting_depth.clone()),
            }).filter(|patch| *patch != Vdi3805LimitsPatch::default()),
        }
    }

    fn is_empty(&self) -> bool {
        self.manufacturer_file.is_none() && self.products.as_ref().map_or(true, |rows| rows.is_empty()) && self.catalog_extensions.is_none() && self.edition_profile.as_ref().map_or(true, |rows| rows.is_empty()) && self.correction_as_of.is_none() && self.strict_mode.is_none() && self.index_entries.as_ref().map_or(true, |rows| rows.is_empty()) && self.geometry.as_ref().map_or(true, |rows| rows.is_empty()) && self.curves.as_ref().map_or(true, |rows| rows.is_empty()) && self.limits.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
