//! 📖️ Part 4 of the ISO 16757 snapshot: the dictionary subjects, relationships, properties, controlled value lists, constraints and ISO 12006 mapping. See [`super`].

use super::catalogue_core::{Cardinality, CatalogueUnit, DictionaryRef, Names};
use super::part_1;
use semio_s_plugin_norm_compliance::LocalizedText;

/// 🏷️ Dictionary subject kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum SubjectKind {
    #[dsl(key = "productGroup")]
    ProductGroup,
    #[dsl(key = "productClass")]
    ProductClass,
    #[dsl(key = "productSpecialization")]
    ProductSpecialization,
    #[dsl(key = "catalogueMetadata")]
    CatalogueMetadata,
    #[dsl(key = "manufacturerMetadata")]
    ManufacturerMetadata,
    #[dsl(key = "propertyBlock")]
    PropertyBlock,
    Port,
    Inlet,
    Outlet,
    #[dsl(key = "inOutlet")]
    InOutlet,
}

/// 📖️ Dictionary subject.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Subject {
    pub id: String,
    pub kind: SubjectKind,
    pub names: Names,
    pub definition: LocalizedText,
    pub parent_id: Option<String>,
}

/// 🔗️ Relationship kind per Part 4 §4.4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum RelationshipKind {
    #[dsl(key = "isSubtypeOf")]
    IsSubtypeOf,
    #[dsl(key = "hasPart")]
    HasPart,
    #[dsl(key = "hasBlock")]
    HasBlock,
    #[dsl(key = "isDependentOn")]
    IsDependentOn,
    #[dsl(key = "isSubkindOf")]
    IsSubkindOf,
}

/// 🔗️ Typed relationship.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Relationship {
    pub id: String,
    pub kind: RelationshipKind,
    pub source_id: String,
    pub target_id: String,
    pub cardinality: Cardinality,
}

/// 📊️ Dictionary property definition.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DictionaryProperty {
    pub id: String,
    pub names: Names,
    pub kind: part_1::PropertyKind,
    pub data_type: String,
    pub unit: Option<CatalogueUnit>,
    pub applicable_subject_ids: Vec<String>,
    pub value_constraints: Vec<ValueConstraint>,
}

/// ✅️ Controlled value list.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ControlledValueList {
    pub id: String,
    pub values: Vec<String>,
    pub context_subject_ids: Vec<String>,
}

/// 🎯️ Value constraint on a property.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ValueConstraint {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub allowed_values: Vec<String>,
}

/// 📚️ Data dictionary snapshot.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Dictionary {
    pub reference: DictionaryRef,
    #[dsl(table)]
    pub subjects: Vec<Subject>,
    #[dsl(table)]
    pub relationships: Vec<Relationship>,
    #[dsl(table)]
    pub properties: Vec<DictionaryProperty>,
    #[dsl(table)]
    pub controlled_lists: Vec<ControlledValueList>,
    #[dsl(table)]
    pub meta_subjects: Vec<Subject>,
}

/// 🗺️ ISO 12006-3 mapping record.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Iso12006Mapping {
    pub dictionary_object_id: String,
    pub iso12006_uri: String,
    pub object_kind: String,
}
