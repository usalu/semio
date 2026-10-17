//! 🔁️ Apply and inverse of the ISO 16757 diff: the single id value patches, the central applier hook and the algebra that returns the negative diff. See [`super`].

use super::*;

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
    pub value: Option<semio_s_artifact_norm_iso16757_model::CatalogueUnit>,
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
