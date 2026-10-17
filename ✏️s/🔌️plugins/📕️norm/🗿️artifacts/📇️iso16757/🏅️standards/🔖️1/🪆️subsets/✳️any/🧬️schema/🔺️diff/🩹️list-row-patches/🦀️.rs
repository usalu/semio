//! 🩹️ Positional list deltas of the ISO 16757 diff: the keyed sparse patches and row deltas of the catalogue lists (product groups, classes, series, products, indexes, property definitions, subjects). See [`super`].

use super::*;

/// 🩹️ Sparse patch of the `catalogue.product_groups` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductGroupsPatch {
    pub name: Option<String>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_iso16757_model::part_1::ProductGroup> for Iso16757ProductGroupsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_1::ProductGroup, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
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

    fn inverse(&self, row: &semio_s_artifact_norm_iso16757_model::part_1::ProductGroup) -> Self {
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
    pub Iso16757ProductGroupsRows { removal: Iso16757ProductGroupsRemoved, insertion: Iso16757ProductGroupsInserted, relocation: Iso16757ProductGroupsMoved, modification: Iso16757ProductGroupsModified, row: semio_s_artifact_norm_iso16757_model::part_1::ProductGroup, patch: Iso16757ProductGroupsPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.product_classes` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductClassesPatch {
    pub group_id: Option<String>,
    pub parent_id: Option<Iso16757ProductClassesPatchParentIdValue>,
    pub names: Option<semio_s_artifact_norm_iso16757_model::Names>,
    pub required_property_ids: Option<Vec<String>>,
    pub optional_property_ids: Option<Vec<String>>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_iso16757_model::part_1::ProductClass> for Iso16757ProductClassesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_1::ProductClass, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
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

    fn inverse(&self, row: &semio_s_artifact_norm_iso16757_model::part_1::ProductClass) -> Self {
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
    pub Iso16757ProductClassesRows { removal: Iso16757ProductClassesRemoved, insertion: Iso16757ProductClassesInserted, relocation: Iso16757ProductClassesMoved, modification: Iso16757ProductClassesModified, row: semio_s_artifact_norm_iso16757_model::part_1::ProductClass, patch: Iso16757ProductClassesPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.product_series` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductSeriesPatch {
    pub class_id: Option<String>,
    pub names: Option<semio_s_artifact_norm_iso16757_model::Names>,
    pub shared_property_values: Option<std::collections::BTreeMap<String, semio_s_artifact_norm_iso16757_model::CatalogueValue>>,
    pub geometry_id: Option<Iso16757ProductSeriesPatchGeometryIdValue>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_iso16757_model::part_1::ProductSeries> for Iso16757ProductSeriesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_1::ProductSeries, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
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

    fn inverse(&self, row: &semio_s_artifact_norm_iso16757_model::part_1::ProductSeries) -> Self {
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
    pub Iso16757ProductSeriesRows { removal: Iso16757ProductSeriesRemoved, insertion: Iso16757ProductSeriesInserted, relocation: Iso16757ProductSeriesMoved, modification: Iso16757ProductSeriesModified, row: semio_s_artifact_norm_iso16757_model::part_1::ProductSeries, patch: Iso16757ProductSeriesPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.products` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757ProductsPatch {
    pub name: Option<String>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_iso16757_model::part_1::Product> for Iso16757ProductsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_1::Product, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
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

    fn inverse(&self, row: &semio_s_artifact_norm_iso16757_model::part_1::Product) -> Self {
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
    pub Iso16757ProductsRows { removal: Iso16757ProductsRemoved, insertion: Iso16757ProductsInserted, relocation: Iso16757ProductsMoved, modification: Iso16757ProductsModified, row: semio_s_artifact_norm_iso16757_model::part_1::Product, patch: Iso16757ProductsPatch, key: id }
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

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_iso16757_model::part_1::ProductIndex> for Iso16757ProductIndexesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_1::ProductIndex, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
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

    fn inverse(&self, row: &semio_s_artifact_norm_iso16757_model::part_1::ProductIndex) -> Self {
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
    pub Iso16757ProductIndexesRows { removal: Iso16757ProductIndexesRemoved, insertion: Iso16757ProductIndexesInserted, relocation: Iso16757ProductIndexesMoved, modification: Iso16757ProductIndexesModified, row: semio_s_artifact_norm_iso16757_model::part_1::ProductIndex, patch: Iso16757ProductIndexesPatch, key: id }
}

/// 🩹️ Sparse patch of the `catalogue.property_definitions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757PropertyDefinitionsPatch {
    pub names: Option<semio_s_artifact_norm_iso16757_model::Names>,
    pub data_type: Option<String>,
    pub unit: Option<Iso16757PropertyDefinitionsPatchUnitValue>,
    pub cardinality: Option<semio_s_artifact_norm_iso16757_model::Cardinality>,
    pub kind: Option<semio_s_artifact_norm_iso16757_model::part_1::PropertyKind>,
    pub dictionary_property_id: Option<Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_iso16757_model::part_1::PropertyDefinition> for Iso16757PropertyDefinitionsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_1::PropertyDefinition, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
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

    fn inverse(&self, row: &semio_s_artifact_norm_iso16757_model::part_1::PropertyDefinition) -> Self {
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
    pub Iso16757PropertyDefinitionsRows { removal: Iso16757PropertyDefinitionsRemoved, insertion: Iso16757PropertyDefinitionsInserted, relocation: Iso16757PropertyDefinitionsMoved, modification: Iso16757PropertyDefinitionsModified, row: semio_s_artifact_norm_iso16757_model::part_1::PropertyDefinition, patch: Iso16757PropertyDefinitionsPatch, key: id }
}

/// 🩹️ Sparse patch of the `dictionary.subjects` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757SubjectsPatch {
    pub kind: Option<semio_s_artifact_norm_iso16757_model::part_4::SubjectKind>,
    pub names: Option<semio_s_artifact_norm_iso16757_model::Names>,
    pub definition: Option<semio_s_plugin_norm_compliance::LocalizedText>,
    pub parent_id: Option<Iso16757SubjectsPatchParentIdValue>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_iso16757_model::part_4::Subject> for Iso16757SubjectsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_iso16757_model::part_4::Subject, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
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

    fn inverse(&self, row: &semio_s_artifact_norm_iso16757_model::part_4::Subject) -> Self {
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
    pub Iso16757SubjectsRows { removal: Iso16757SubjectsRemoved, insertion: Iso16757SubjectsInserted, relocation: Iso16757SubjectsMoved, modification: Iso16757SubjectsModified, row: semio_s_artifact_norm_iso16757_model::part_4::Subject, patch: Iso16757SubjectsPatch, key: id }
}
