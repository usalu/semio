//! 🧬️ Iso16757 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

use crate::Iso16757Snapshot;

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `catalogue.product_groups` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductGroupsPatch {
    pub name: Option<String>,
}

impl protocol::list_delta::RowPatch<crate::part_1::ProductGroup> for Iso16757ProductGroupsPatch {
    fn commit_into(&self, row: &mut crate::part_1::ProductGroup, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.name {
            row.names.preferred.text = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.name.is_some() {
            self.name = later.name;
        }
    }

    fn inverse(&self, row: &crate::part_1::ProductGroup) -> Self {
        Self {
            name: self.name.as_ref().map(|_| row.names.preferred.text.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `catalogue.product_groups` list (rows keyed by `id`).
    pub Iso16757ProductGroupsRows { removal: Iso16757ProductGroupsRemoved, insertion: Iso16757ProductGroupsInserted, relocation: Iso16757ProductGroupsMoved, modification: Iso16757ProductGroupsModified, row: crate::part_1::ProductGroup, patch: Iso16757ProductGroupsPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.product_classes` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductClassesPatch {
    pub group_id: Option<String>,
    pub parent_id: Option<Iso16757ProductClassesPatchParentIdValue>,
    pub names: Option<crate::Names>,
    pub required_property_ids: Option<Vec<String>>,
    pub optional_property_ids: Option<Vec<String>>,
}

impl protocol::list_delta::RowPatch<crate::part_1::ProductClass> for Iso16757ProductClassesPatch {
    fn commit_into(&self, row: &mut crate::part_1::ProductClass, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.group_id {
            row.group_id = value.clone();
        }
        if let Some(value) = &self.parent_id {
            row.parent_id = value.value.clone();
        }
        if let Some(value) = &self.names {
            row.names = value.clone();
        }
        if let Some(value) = &self.required_property_ids {
            row.required_property_ids = value.clone();
        }
        if let Some(value) = &self.optional_property_ids {
            row.optional_property_ids = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.group_id.is_some() {
            self.group_id = later.group_id;
        }
        if later.parent_id.is_some() {
            self.parent_id = later.parent_id;
        }
        if later.names.is_some() {
            self.names = later.names;
        }
        if later.required_property_ids.is_some() {
            self.required_property_ids = later.required_property_ids;
        }
        if later.optional_property_ids.is_some() {
            self.optional_property_ids = later.optional_property_ids;
        }
    }

    fn inverse(&self, row: &crate::part_1::ProductClass) -> Self {
        Self {
            group_id: self.group_id.as_ref().map(|_| row.group_id.clone()),
            parent_id: self.parent_id.as_ref().map(|_| Iso16757ProductClassesPatchParentIdValue { value: row.parent_id.clone() }),
            names: self.names.as_ref().map(|_| row.names.clone()),
            required_property_ids: self.required_property_ids.as_ref().map(|_| row.required_property_ids.clone()),
            optional_property_ids: self.optional_property_ids.as_ref().map(|_| row.optional_property_ids.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.group_id.is_none() && self.parent_id.is_none() && self.names.is_none() && self.required_property_ids.is_none() && self.optional_property_ids.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `catalogue.product_classes` list (rows keyed by `id`).
    pub Iso16757ProductClassesRows { removal: Iso16757ProductClassesRemoved, insertion: Iso16757ProductClassesInserted, relocation: Iso16757ProductClassesMoved, modification: Iso16757ProductClassesModified, row: crate::part_1::ProductClass, patch: Iso16757ProductClassesPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.product_series` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductSeriesPatch {
    pub class_id: Option<String>,
    pub names: Option<crate::Names>,
    pub shared_property_values: Option<std::collections::BTreeMap<String, crate::CatalogueValue>>,
    pub geometry_id: Option<Iso16757ProductSeriesPatchGeometryIdValue>,
}

impl protocol::list_delta::RowPatch<crate::part_1::ProductSeries> for Iso16757ProductSeriesPatch {
    fn commit_into(&self, row: &mut crate::part_1::ProductSeries, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.class_id {
            row.class_id = value.clone();
        }
        if let Some(value) = &self.names {
            row.names = value.clone();
        }
        if let Some(value) = &self.shared_property_values {
            row.shared_property_values = value.clone();
        }
        if let Some(value) = &self.geometry_id {
            row.geometry_id = value.value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.class_id.is_some() {
            self.class_id = later.class_id;
        }
        if later.names.is_some() {
            self.names = later.names;
        }
        if later.shared_property_values.is_some() {
            self.shared_property_values = later.shared_property_values;
        }
        if later.geometry_id.is_some() {
            self.geometry_id = later.geometry_id;
        }
    }

    fn inverse(&self, row: &crate::part_1::ProductSeries) -> Self {
        Self {
            class_id: self.class_id.as_ref().map(|_| row.class_id.clone()),
            names: self.names.as_ref().map(|_| row.names.clone()),
            shared_property_values: self.shared_property_values.as_ref().map(|_| row.shared_property_values.clone()),
            geometry_id: self.geometry_id.as_ref().map(|_| Iso16757ProductSeriesPatchGeometryIdValue { value: row.geometry_id.clone() }),
        }
    }

    fn is_empty(&self) -> bool {
        self.class_id.is_none() && self.names.is_none() && self.shared_property_values.is_none() && self.geometry_id.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `catalogue.product_series` list (rows keyed by `id`).
    pub Iso16757ProductSeriesRows { removal: Iso16757ProductSeriesRemoved, insertion: Iso16757ProductSeriesInserted, relocation: Iso16757ProductSeriesMoved, modification: Iso16757ProductSeriesModified, row: crate::part_1::ProductSeries, patch: Iso16757ProductSeriesPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.products` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductsPatch {
    pub name: Option<String>,
}

impl protocol::list_delta::RowPatch<crate::part_1::Product> for Iso16757ProductsPatch {
    fn commit_into(&self, row: &mut crate::part_1::Product, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.name {
            row.names.preferred.text = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.name.is_some() {
            self.name = later.name;
        }
    }

    fn inverse(&self, row: &crate::part_1::Product) -> Self {
        Self {
            name: self.name.as_ref().map(|_| row.names.preferred.text.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `catalogue.products` list (rows keyed by `id`).
    pub Iso16757ProductsRows { removal: Iso16757ProductsRemoved, insertion: Iso16757ProductsInserted, relocation: Iso16757ProductsMoved, modification: Iso16757ProductsModified, row: crate::part_1::Product, patch: Iso16757ProductsPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.product_indexes` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductIndexesPatch {
    pub product_id: Option<String>,
    pub variant_id: Option<Iso16757ProductIndexesPatchVariantIdValue>,
    pub search_tags: Option<Vec<String>>,
}

impl protocol::list_delta::RowPatch<crate::part_1::ProductIndex> for Iso16757ProductIndexesPatch {
    fn commit_into(&self, row: &mut crate::part_1::ProductIndex, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.product_id {
            row.product_id = value.clone();
        }
        if let Some(value) = &self.variant_id {
            row.variant_id = value.value.clone();
        }
        if let Some(value) = &self.search_tags {
            row.search_tags = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.product_id.is_some() {
            self.product_id = later.product_id;
        }
        if later.variant_id.is_some() {
            self.variant_id = later.variant_id;
        }
        if later.search_tags.is_some() {
            self.search_tags = later.search_tags;
        }
    }

    fn inverse(&self, row: &crate::part_1::ProductIndex) -> Self {
        Self {
            product_id: self.product_id.as_ref().map(|_| row.product_id.clone()),
            variant_id: self.variant_id.as_ref().map(|_| Iso16757ProductIndexesPatchVariantIdValue { value: row.variant_id.clone() }),
            search_tags: self.search_tags.as_ref().map(|_| row.search_tags.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.product_id.is_none() && self.variant_id.is_none() && self.search_tags.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `catalogue.product_indexes` list (rows keyed by `id`).
    pub Iso16757ProductIndexesRows { removal: Iso16757ProductIndexesRemoved, insertion: Iso16757ProductIndexesInserted, relocation: Iso16757ProductIndexesMoved, modification: Iso16757ProductIndexesModified, row: crate::part_1::ProductIndex, patch: Iso16757ProductIndexesPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.property_definitions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757PropertyDefinitionsPatch {
    pub names: Option<crate::Names>,
    pub data_type: Option<String>,
    pub unit: Option<Iso16757PropertyDefinitionsPatchUnitValue>,
    pub cardinality: Option<crate::Cardinality>,
    pub kind: Option<crate::part_1::PropertyKind>,
    pub dictionary_property_id: Option<Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue>,
}

impl protocol::list_delta::RowPatch<crate::part_1::PropertyDefinition> for Iso16757PropertyDefinitionsPatch {
    fn commit_into(&self, row: &mut crate::part_1::PropertyDefinition, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.names {
            row.names = value.clone();
        }
        if let Some(value) = &self.data_type {
            row.data_type = value.clone();
        }
        if let Some(value) = &self.unit {
            row.unit = value.value.clone();
        }
        if let Some(value) = &self.cardinality {
            row.cardinality = value.clone();
        }
        if let Some(value) = &self.kind {
            row.kind = value.clone();
        }
        if let Some(value) = &self.dictionary_property_id {
            row.dictionary_property_id = value.value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.names.is_some() {
            self.names = later.names;
        }
        if later.data_type.is_some() {
            self.data_type = later.data_type;
        }
        if later.unit.is_some() {
            self.unit = later.unit;
        }
        if later.cardinality.is_some() {
            self.cardinality = later.cardinality;
        }
        if later.kind.is_some() {
            self.kind = later.kind;
        }
        if later.dictionary_property_id.is_some() {
            self.dictionary_property_id = later.dictionary_property_id;
        }
    }

    fn inverse(&self, row: &crate::part_1::PropertyDefinition) -> Self {
        Self {
            names: self.names.as_ref().map(|_| row.names.clone()),
            data_type: self.data_type.as_ref().map(|_| row.data_type.clone()),
            unit: self.unit.as_ref().map(|_| Iso16757PropertyDefinitionsPatchUnitValue { value: row.unit.clone() }),
            cardinality: self.cardinality.as_ref().map(|_| row.cardinality.clone()),
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            dictionary_property_id: self.dictionary_property_id.as_ref().map(|_| Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue { value: row.dictionary_property_id.clone() }),
        }
    }

    fn is_empty(&self) -> bool {
        self.names.is_none() && self.data_type.is_none() && self.unit.is_none() && self.cardinality.is_none() && self.kind.is_none() && self.dictionary_property_id.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `catalogue.property_definitions` list (rows keyed by `id`).
    pub Iso16757PropertyDefinitionsRows { removal: Iso16757PropertyDefinitionsRemoved, insertion: Iso16757PropertyDefinitionsInserted, relocation: Iso16757PropertyDefinitionsMoved, modification: Iso16757PropertyDefinitionsModified, row: crate::part_1::PropertyDefinition, patch: Iso16757PropertyDefinitionsPatch, key: id }
}

/// 🩹️ Sparse patch of the `dictionary.subjects` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757SubjectsPatch {
    pub kind: Option<crate::part_4::SubjectKind>,
    pub names: Option<crate::Names>,
    pub definition: Option<crate::document::LocalizedText>,
    pub parent_id: Option<Iso16757SubjectsPatchParentIdValue>,
}

impl protocol::list_delta::RowPatch<crate::part_4::Subject> for Iso16757SubjectsPatch {
    fn commit_into(&self, row: &mut crate::part_4::Subject, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind {
            row.kind = value.clone();
        }
        if let Some(value) = &self.names {
            row.names = value.clone();
        }
        if let Some(value) = &self.definition {
            row.definition = value.clone();
        }
        if let Some(value) = &self.parent_id {
            row.parent_id = value.value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.kind.is_some() {
            self.kind = later.kind;
        }
        if later.names.is_some() {
            self.names = later.names;
        }
        if later.definition.is_some() {
            self.definition = later.definition;
        }
        if later.parent_id.is_some() {
            self.parent_id = later.parent_id;
        }
    }

    fn inverse(&self, row: &crate::part_4::Subject) -> Self {
        Self {
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            names: self.names.as_ref().map(|_| row.names.clone()),
            definition: self.definition.as_ref().map(|_| row.definition.clone()),
            parent_id: self.parent_id.as_ref().map(|_| Iso16757SubjectsPatchParentIdValue { value: row.parent_id.clone() }),
        }
    }

    fn is_empty(&self) -> bool {
        self.kind.is_none() && self.names.is_none() && self.definition.is_none() && self.parent_id.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `dictionary.subjects` list (rows keyed by `id`).
    pub Iso16757SubjectsRows { removal: Iso16757SubjectsRemoved, insertion: Iso16757SubjectsInserted, relocation: Iso16757SubjectsMoved, modification: Iso16757SubjectsModified, row: crate::part_4::Subject, patch: Iso16757SubjectsPatch, key: id }
}

/// 🔑️ A `geometry.objects` entry: its map key and value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

}

/// 📌️ A `selection.constraints` row inserted at final position `index`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

}

/// 🔑️ A `part_number_inputs` entry: its map key and value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

    fn inverse_rows(&self, base: &std::collections::BTreeMap<String, crate::CatalogueValue>) -> Self {
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

/// 🎁️ Carries the optional `ProductClass.parent_id` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductClassesPatchParentIdValue {
    pub value: Option<String>,
}

/// 🎁️ Carries the optional `ProductSeries.geometry_id` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductSeriesPatchGeometryIdValue {
    pub value: Option<String>,
}

/// 🎁️ Carries the optional `ProductIndex.variant_id` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductIndexesPatchVariantIdValue {
    pub value: Option<String>,
}

/// 🎁️ Carries the optional `PropertyDefinition.unit` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757PropertyDefinitionsPatchUnitValue {
    pub value: Option<crate::CatalogueUnit>,
}

/// 🎁️ Carries the optional `PropertyDefinition.dictionary_property_id` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue {
    pub value: Option<String>,
}

/// 🎁️ Carries the optional `Subject.parent_id` value so an explicit `None` stays distinct from an untouched field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757SubjectsPatchParentIdValue {
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
    fn apply(&self, base: &Iso16757Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Iso16757Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.catalogue_name {
            next.catalogue.metadata.names.preferred.text = value.clone();
        }
        if let Some(value) = &self.manufacturer_name {
            next.catalogue.manufacturer.names.preferred.text = value.clone();
        }
        if let Some(rows) = &self.product_groups {
            next.catalogue.product_groups = rows.commit_onto(&base.catalogue.product_groups, capability).map_err(|error| error.under(["product_groups"]))?;
        }
        if let Some(rows) = &self.product_classes {
            next.catalogue.product_classes = rows.commit_onto(&base.catalogue.product_classes, capability).map_err(|error| error.under(["product_classes"]))?;
        }
        if let Some(rows) = &self.product_series {
            next.catalogue.product_series = rows.commit_onto(&base.catalogue.product_series, capability).map_err(|error| error.under(["product_series"]))?;
        }
        if let Some(rows) = &self.products {
            next.catalogue.products = rows.commit_onto(&base.catalogue.products, capability).map_err(|error| error.under(["products"]))?;
        }
        if let Some(rows) = &self.product_indexes {
            next.catalogue.product_indexes = rows.commit_onto(&base.catalogue.product_indexes, capability).map_err(|error| error.under(["product_indexes"]))?;
        }
        if let Some(rows) = &self.property_definitions {
            next.catalogue.property_definitions = rows.commit_onto(&base.catalogue.property_definitions, capability).map_err(|error| error.under(["property_definitions"]))?;
        }
        if let Some(rows) = &self.subjects {
            next.dictionary.subjects = rows.commit_onto(&base.dictionary.subjects, capability).map_err(|error| error.under(["subjects"]))?;
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
                Some(mine) => mine.absorb(theirs),
                None => self.product_groups = Some(theirs),
            }
            self.product_groups = self.product_groups.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.product_classes {
            match self.product_classes.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.product_classes = Some(theirs),
            }
            self.product_classes = self.product_classes.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.product_series {
            match self.product_series.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.product_series = Some(theirs),
            }
            self.product_series = self.product_series.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.products {
            match self.products.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.products = Some(theirs),
            }
            self.products = self.products.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.product_indexes {
            match self.product_indexes.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.product_indexes = Some(theirs),
            }
            self.product_indexes = self.product_indexes.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.property_definitions {
            match self.property_definitions.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.property_definitions = Some(theirs),
            }
            self.property_definitions = self.property_definitions.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.subjects {
            match self.subjects.as_mut() {
                Some(mine) => mine.absorb(theirs),
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
            product_groups: self.product_groups.as_ref().map(|rows| rows.inverse(&base.catalogue.product_groups)).filter(|rows| !rows.is_empty()),
            product_classes: self.product_classes.as_ref().map(|rows| rows.inverse(&base.catalogue.product_classes)).filter(|rows| !rows.is_empty()),
            product_series: self.product_series.as_ref().map(|rows| rows.inverse(&base.catalogue.product_series)).filter(|rows| !rows.is_empty()),
            products: self.products.as_ref().map(|rows| rows.inverse(&base.catalogue.products)).filter(|rows| !rows.is_empty()),
            product_indexes: self.product_indexes.as_ref().map(|rows| rows.inverse(&base.catalogue.product_indexes)).filter(|rows| !rows.is_empty()),
            property_definitions: self.property_definitions.as_ref().map(|rows| rows.inverse(&base.catalogue.property_definitions)).filter(|rows| !rows.is_empty()),
            subjects: self.subjects.as_ref().map(|rows| rows.inverse(&base.dictionary.subjects)).filter(|rows| !rows.is_empty()),
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

    fn is_empty(&self) -> bool {
        self.catalogue_name.is_none() && self.manufacturer_name.is_none() && self.product_groups.as_ref().map_or(true, |rows| rows.is_empty()) && self.product_classes.as_ref().map_or(true, |rows| rows.is_empty()) && self.product_series.as_ref().map_or(true, |rows| rows.is_empty()) && self.products.as_ref().map_or(true, |rows| rows.is_empty()) && self.product_indexes.as_ref().map_or(true, |rows| rows.is_empty()) && self.property_definitions.as_ref().map_or(true, |rows| rows.is_empty()) && self.subjects.as_ref().map_or(true, |rows| rows.is_empty()) && self.geometry_objects.as_ref().map_or(true, |rows| rows.is_empty()) && self.selection_class_id.is_none() && self.selection_series_id.is_none() && self.selection_constraints.as_ref().map_or(true, |rows| rows.is_empty()) && self.part_number_rule.is_none() && self.part_number_inputs.as_ref().map_or(true, |rows| rows.is_empty()) && self.script_limits.is_none() && self.exchange_process.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
