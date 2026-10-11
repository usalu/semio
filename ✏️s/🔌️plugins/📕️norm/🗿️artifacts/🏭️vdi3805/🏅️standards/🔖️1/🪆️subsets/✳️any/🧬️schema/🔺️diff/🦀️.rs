//! 🧬️ Vdi3805 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

use crate::Vdi3805Snapshot;

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
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

/// 🩹️ Sparse patch of the `catalog.products` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805ProductsPatch {
    pub title: Option<Vec<crate::document::LocalizedText>>,
    pub configuration: Option<crate::Configuration>,
}

impl protocol::list_delta::RowPatch<crate::CatalogueProduct> for Vdi3805ProductsPatch {
    fn commit_into(&self, row: &mut crate::CatalogueProduct, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.title {
            row.title = value.clone();
        }
        if let Some(value) = &self.configuration {
            row.configuration = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.title.is_some() {
            self.title = later.title;
        }
        if later.configuration.is_some() {
            self.configuration = later.configuration;
        }
    }

    fn inverse(&self, row: &crate::CatalogueProduct) -> Self {
        Self {
            title: self.title.as_ref().map(|_| row.title.clone()),
            configuration: self.configuration.as_ref().map(|_| row.configuration.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.title.is_none() && self.configuration.is_none()
    }
}

protocol::list_delta! {
    /// 🔑️ Positional keyed row delta of `catalog.products`; the key of a `crate::CatalogueProduct` row is `identity.article_number`.
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub Vdi3805ProductsRows { removal: Vdi3805ProductsRemoved, insertion: Vdi3805ProductsInserted, relocation: Vdi3805ProductsMoved, modification: Vdi3805ProductsModified, row: crate::CatalogueProduct, patch: Vdi3805ProductsPatch, list: Vec<crate::CatalogueProduct>, key: String = |row| row.identity.article_number.clone() }
}

/// 🔑️ A `edition_profile` entry: its map key and value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-id", format!("entry \"{}\" already exists", entry.key)).at([entry.key.clone()]));
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

}

/// 🩹️ Sparse patch of the `connections` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805GeometryConnectionsPatch {
    pub medium: Option<String>,
    pub position: Option<[f64; 3]>,
    pub direction: Option<[f64; 3]>,
    pub diameter_mm: Option<Vdi3805GeometryConnectionsPatchDiameterMmValue>,
}

impl protocol::list_delta::RowPatch<crate::ConnectionPoint> for Vdi3805GeometryConnectionsPatch {
    fn commit_into(&self, row: &mut crate::ConnectionPoint, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.medium {
            row.medium = value.clone();
        }
        if let Some(value) = &self.position {
            row.position = value.clone();
        }
        if let Some(value) = &self.direction {
            row.direction = value.clone();
        }
        if let Some(value) = &self.diameter_mm {
            row.diameter_mm = value.value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.medium.is_some() {
            self.medium = later.medium;
        }
        if later.position.is_some() {
            self.position = later.position;
        }
        if later.direction.is_some() {
            self.direction = later.direction;
        }
        if later.diameter_mm.is_some() {
            self.diameter_mm = later.diameter_mm;
        }
    }

    fn inverse(&self, row: &crate::ConnectionPoint) -> Self {
        Self {
            medium: self.medium.as_ref().map(|_| row.medium.clone()),
            position: self.position.as_ref().map(|_| row.position.clone()),
            direction: self.direction.as_ref().map(|_| row.direction.clone()),
            diameter_mm: self.diameter_mm.as_ref().map(|_| Vdi3805GeometryConnectionsPatchDiameterMmValue { value: row.diameter_mm.clone() }),
        }
    }

    fn is_empty(&self) -> bool {
        self.medium.is_none() && self.position.is_none() && self.direction.is_none() && self.diameter_mm.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `connections` list (rows keyed by `id`).
    pub Vdi3805GeometryConnectionsRows { removal: Vdi3805GeometryConnectionsRemoved, insertion: Vdi3805GeometryConnectionsInserted, relocation: Vdi3805GeometryConnectionsMoved, modification: Vdi3805GeometryConnectionsModified, row: crate::ConnectionPoint, patch: Vdi3805GeometryConnectionsPatch, key: id }
}

/// 🔑️ A `geometry` entry: its map key and value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
    fn apply_to_row(&self, row: &mut crate::ParametricGeometry, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.bbox {
            row.bbox = value.clone();
        }
        if let Some(value) = &self.parameters {
            row.parameters = value.clone();
        }
        if let Some(rows) = &self.connections {
            row.connections = rows.commit_onto(&row.connections, capability).map_err(|error| error.under(["connections"]))?;
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
                Some(mine) => mine.absorb(theirs),
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
            connections: self.connections.as_ref().map(|rows| rows.inverse(&row.connections)).filter(|rows| !rows.is_empty()),
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

    fn apply_rows(&self, base: &std::collections::BTreeMap<String, crate::ParametricGeometry>, capability: protocol::ApplyCapability) -> Result<std::collections::BTreeMap<String, crate::ParametricGeometry>, protocol::MutationApplyError> {
        let mut map = base.clone();
        for key in &self.removed {
            map.remove(key).ok_or_else(|| missing_target(format!("removed entry \"{key}\"")).at([key.clone()]))?;
        }
        for entry in &self.added {
            if map.insert(entry.key.clone(), entry.value.clone()).is_some() {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-id", format!("entry \"{}\" already exists", entry.key)).at([entry.key.clone()]));
            }
        }
        for patch in &self.modified {
            let value = map.get_mut(&patch.key).ok_or_else(|| missing_target(format!("modified entry \"{}\"", patch.key)).at([patch.key.clone()]))?;
            patch.apply_to_row(value, capability)?;
        }
        Ok(map)
    }

    fn absorb_rows(&mut self, other: Self) {
        for key in other.removed {
            if let Some(at) = self.added.iter().position(|entry| entry.key == key) {
                self.added.remove(at);
                self.modified.retain(|entry| entry.key != key);
            } else {
                self.modified.retain(|entry| entry.key != key);
                if !self.removed.contains(&key) {
                    self.removed.push(key);
                }
            }
        }
        self.added.extend(other.added);
        for patch in other.modified {
            if let Some(existing) = self.modified.iter_mut().find(|existing| existing.key == patch.key) {
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

}

/// 🔑️ A `curves` entry: its map key and value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-id", format!("entry \"{}\" already exists", entry.key)).at([entry.key.clone()]));
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

/// 🎁️ Carries the optional `ConnectionPoint.diameter_mm` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805GeometryConnectionsPatchDiameterMmValue {
    pub value: Option<f64>,
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
    pub geometry: Option<Vdi3805GeometryRows>,
    #[state(artifact)]
    pub curves: Option<Vdi3805CurvesRows>,
    #[state(artifact)]
    pub limits: Option<Vdi3805LimitsPatch>,
}

impl protocol::MutationDiff<Vdi3805Snapshot> for Vdi3805Diff {
    fn apply(&self, base: &Vdi3805Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Vdi3805Snapshot> {
        let mut next = base.clone();
        if let Some(patch) = &self.manufacturer_file {
            patch.apply_to_row(&mut next.catalog.file).map_err(|error| error.under(["manufacturer_file"]))?;
        }
        if let Some(rows) = &self.products {
            next.catalog.products = rows.commit_onto(&base.catalog.products, capability).map_err(|error| error.under(["products"]))?;
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
        if let Some(rows) = &self.geometry {
            next.geometry = rows.apply_rows(&base.geometry, capability).map_err(|error| error.under(["geometry"]))?;
        }
        if let Some(rows) = &self.curves {
            next.curves = rows.apply_rows(&base.curves).map_err(|error| error.under(["curves"]))?;
        }
        if let Some(patch) = &self.limits {
            patch.apply_to_row(&mut next.limits).map_err(|error| error.under(["limits"]))?;
        }
        if self.products.is_some() {
            next.index = crate::CatalogIndex::from_catalog(&next.catalog);
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
                Some(mine) => mine.absorb(theirs),
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
            products: self.products.as_ref().map(|rows| rows.inverse(&base.catalog.products)).filter(|rows| !rows.is_empty()),
            catalog_extensions: self.catalog_extensions.as_ref().map(|_| base.catalog.extensions.clone()),
            edition_profile: self.edition_profile.as_ref().map(|rows| rows.inverse_rows(&base.edition_profile)).filter(|rows| !rows.is_empty()),
            correction_as_of: self.correction_as_of.as_ref().map(|_| base.correction_as_of.clone()),
            strict_mode: self.strict_mode.as_ref().map(|_| base.strict_mode.clone()),
            geometry: self.geometry.as_ref().map(|rows| rows.inverse_rows(&base.geometry)).filter(|rows| !rows.is_empty()),
            curves: self.curves.as_ref().map(|rows| rows.inverse_rows(&base.curves)).filter(|rows| !rows.is_empty()),
            limits: self.limits.as_ref().map(|patch| patch.inverse_from_row(&base.limits)),
        }
    }

    fn is_empty(&self) -> bool {
        self.manufacturer_file.is_none() && self.products.as_ref().map_or(true, |rows| rows.is_empty()) && self.catalog_extensions.is_none() && self.edition_profile.as_ref().map_or(true, |rows| rows.is_empty()) && self.correction_as_of.is_none() && self.strict_mode.is_none() && self.geometry.as_ref().map_or(true, |rows| rows.is_empty()) && self.curves.as_ref().map_or(true, |rows| rows.is_empty()) && self.limits.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
