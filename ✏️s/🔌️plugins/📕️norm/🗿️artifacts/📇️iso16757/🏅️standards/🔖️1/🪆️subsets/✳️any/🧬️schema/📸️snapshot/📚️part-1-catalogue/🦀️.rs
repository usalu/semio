//! 📚️ Part 1 of the ISO 16757 snapshot: the product catalogue with manufacturers, groups, classes, series, properties, variants, products, relationships, selection and BIM embedding. See [`super`].

use super::catalogue_core::{Cardinality, CatalogueId, CatalogueUnit, CatalogueValue, DictionaryRef, ExtensionBag, Lifecycle, Names};
use std::collections::BTreeMap;

/// 🏭️ Manufacturer metadata.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Manufacturer {
    pub id: String,
    pub names: Names,
}

/// 📦️ Product group declaration.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProductGroup {
    pub id: String,
    pub names: Names,
    pub dictionary_subject_id: Option<String>,
}

/// 🏷️ Product class in a hierarchy.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProductClass {
    pub id: String,
    pub group_id: String,
    pub parent_id: Option<String>,
    pub names: Names,
    pub required_property_ids: Vec<String>,
    pub optional_property_ids: Vec<String>,
}

/// 📚️ Product series sharing geometry and properties.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProductSeries {
    pub id: String,
    pub class_id: String,
    pub names: Names,
    pub shared_property_values: BTreeMap<String, CatalogueValue>,
    pub geometry_id: Option<String>,
}

/// 🔧️ Variant parameter domain.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ParameterDomain {
    pub parameter_id: String,
    pub allowed_values: Vec<CatalogueValue>,
    pub default_value: Option<CatalogueValue>,
}

/// 🧮️ Property definition.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PropertyDefinition {
    pub id: String,
    pub names: Names,
    pub data_type: String,
    pub unit: Option<CatalogueUnit>,
    pub cardinality: Cardinality,
    pub kind: PropertyKind,
    pub dictionary_property_id: Option<String>,
}

/// 📊️ Property kind per Part 1 §5.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyKind {
    Static,
    Dynamic,
    Selection,
    External,
}

/// 📋️ Property value on a product or variant.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct PropertyValue {
    pub definition_id: String,
    pub value: CatalogueValue,
    pub function_id: Option<String>,
}

/// 🧩️ Product variant with parameters.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProductVariant {
    pub id: String,
    pub parameter_values: BTreeMap<String, CatalogueValue>,
    pub property_values: Vec<PropertyValue>,
    pub article_number: Option<String>,
    pub geometry_id: Option<String>,
}

/// 📦️ Catalogue product (generic or resolved).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Product {
    pub id: String,
    pub series_id: String,
    pub names: Names,
    pub parameter_domains: Vec<ParameterDomain>,
    pub variants: Vec<ProductVariant>,
    pub static_properties: Vec<PropertyValue>,
}

/// 🔍️ Product index for selection.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ProductIndex {
    pub id: String,
    pub product_id: String,
    pub variant_id: Option<String>,
    pub search_tags: Vec<String>,
}

/// 🔗️ Accessory relationship.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct AccessoryRelationship {
    pub accessory_product_id: String,
    pub required: bool,
    pub quantity: Cardinality,
    pub compatibility_condition: Option<String>,
}

/// 🧱️ Composition relationship (`hasPart`).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CompositionRelationship {
    pub component_product_id: String,
    pub quantity: u32,
}

/// 🖼️ Geometry reference.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GeometryReference {
    pub geometry_id: String,
    pub lod: Option<String>,
}

/// 📄️ Descriptive media object.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DescriptiveObject {
    pub id: String,
    pub media_type: String,
    pub uri: String,
    pub language: Option<String>,
    pub checksum: Option<String>,
}

/// 📚️ Full catalogue document.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Catalogue {
    pub id: CatalogueId,
    pub metadata: CatalogueMetadata,
    pub manufacturer: Manufacturer,
    pub dictionary: DictionaryRef,
    #[dsl(table)]
    pub product_groups: Vec<ProductGroup>,
    #[dsl(table)]
    pub product_classes: Vec<ProductClass>,
    #[dsl(table)]
    pub product_series: Vec<ProductSeries>,
    #[dsl(table)]
    pub products: Vec<Product>,
    #[dsl(table)]
    pub product_indexes: Vec<ProductIndex>,
    #[dsl(table)]
    pub property_definitions: Vec<PropertyDefinition>,
    pub accessories: BTreeMap<String, Vec<AccessoryRelationship>>,
    pub compositions: BTreeMap<String, Vec<CompositionRelationship>>,
    #[dsl(table)]
    pub descriptive_objects: Vec<DescriptiveObject>,
    pub extensions: ExtensionBag,
}

/// 📋️ Catalogue metadata.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CatalogueMetadata {
    pub names: Names,
    pub lifecycle: Lifecycle,
    pub edition_profile: EditionProfile,
}

/// 📑️ Supported ISO 16757 edition profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum EditionProfile {
    #[dsl(key = "part1_2015")]
    Part1_2015,
    #[dsl(key = "part2_2016")]
    Part2_2016,
    #[dsl(key = "part4_2025")]
    Part4_2025,
    #[dsl(key = "part5_2025")]
    Part5_2025,
    #[dsl(key = "fullPublished")]
    FullPublished,
}

/// 🎯️ Selection constraint on a property.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SelectionConstraint {
    #[cfg_attr(test, serde(default))]
    pub id: String,
    pub property_id: String,
    pub operator: ConstraintOperator,
    pub value: CatalogueValue,
}

/// ⚖️ Constraint operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintOperator {
    Equal,
    #[dsl(key = "notEqual")]
    NotEqual,
    #[dsl(key = "lessThan")]
    LessThan,
    #[dsl(key = "greaterThan")]
    GreaterThan,
    #[dsl(key = "inRange")]
    InRange,
}

/// 🔎️ Selection request.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SelectionRequest {
    pub class_id: String,
    pub constraints: Vec<SelectionConstraint>,
    pub series_id: Option<String>,
}

/// ✅️ Selection outcome.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct SelectionResult {
    pub matches: Vec<ProductIndex>,
    pub ambiguity: bool,
    pub explanations: Vec<String>,
}

/// 🏗️ BIM embedding workflow state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BimEmbedding {
    pub selected_index_id: String,
    pub frozen_parameters: std::collections::HashMap<String, CatalogueValue>,
    pub resolved_properties: Vec<PropertyValue>,
    pub resolved_article_number: Option<String>,
    pub resolved_geometry_id: Option<String>,
    pub catalogue_provenance: CatalogueId,
    pub dictionary_provenance: DictionaryRef,
}
