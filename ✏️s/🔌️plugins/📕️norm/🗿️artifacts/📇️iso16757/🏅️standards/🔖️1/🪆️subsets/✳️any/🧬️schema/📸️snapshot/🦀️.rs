//! 🧬️ Iso16757 snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use crate::{part_1, part_2, part_4, part_5};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.iso16757", layout = "lines")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Snapshot {
    #[state(artifact)]
    pub catalogue: crate::part_1::Catalogue,
    #[state(artifact)]
    pub dictionary: crate::part_4::Dictionary,
    #[state(artifact)]
    pub geometry: crate::part_2::GeometryCatalogue,
    #[state(artifact)]
    pub selection: crate::part_1::SelectionRequest,
    #[state(artifact)]
    pub part_number_rule: crate::part_5::PartNumberRule,
    #[state(artifact)]
    pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(artifact)]
    pub script_limits: crate::part_5::ScriptLimits,
    #[state(artifact)]
    pub exchange_process: crate::part_5::ExchangeProcess,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).

//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Iso16757Snapshot {
    fn default() -> Self {
        Self::reference_fixture()
    }
}
//#endregion 🔖️Snapshot

/// ♻️ Releases decoded catalogue owners without recursive value destruction.
pub fn retire_decoded_catalogue_value(value:CatalogueValue){let mut pending=vec![value];while let Some(value)=pending.pop(){if let CatalogueValue::List{items}=value{pending.extend(items);}}}

/// ♻️ Releases decoded catalogue owners without recursive value destruction.
pub(crate) fn retire_decoded_catalogue_geometry(node:part_2::GeometryNode){use crate::part_2::GeometryNode as N;let mut pending=vec![node];while let Some(node)=pending.pop(){match node{N::Transform{child,..}=>pending.push(*child),N::Boolean{children,..}=>pending.extend(children),_=>{}}}}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use std::collections::BTreeMap;
use semio_s_plugin_norm_compliance::LocalizedText;
use semio_s_plugin_norm_app_surface;

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../../📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

/// 🆔️ Stable catalogue identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(transparent)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CatalogueId(pub String);

/// 🆔️ Dictionary identifier with version.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DictionaryRef {
    pub id: String,
    pub version: String,
}

/// 📝️ Preferred and alternative names.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Names {
    pub preferred: LocalizedText,
    pub short_name: Option<String>,
    #[dsl(table)]
    pub alternatives: Vec<LocalizedText>,
}

/// 📊️ Physical dimension signature for unit compatibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DimensionSignature {
    pub length: i8,
    pub mass: i8,
    pub time: i8,
    pub temperature: i8,
}

impl DimensionSignature {
    pub const DIMENSIONLESS: Self = Self { length: 0, mass: 0, time: 0, temperature: 0 };
    pub const LENGTH: Self = Self { length: 1, mass: 0, time: 0, temperature: 0 };
    pub const LENGTH_3: Self = Self { length: 3, mass: 0, time: 0, temperature: 0 };

    pub fn compatible(self, other: Self) -> bool {
        self == other
    }
}

/// 📐️ Catalogue unit with canonical SI display.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CatalogueUnit {
    pub symbol: String,
    pub dimension: DimensionSignature,
    pub si_factor: f64,
}

/// 🔢️ Typed catalogue value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CatalogueValue {
    Boolean { value: bool },
    Integer { value: i64 },
    Decimal { value: f64 },
    Text { value: String },
    Identifier { value: String },
    Enumeration { value: String },
    Controlled { value: String, list_id: String },
    Quantity { value: f64, unit: CatalogueUnit },
    Range { min: f64, max: f64, unit: Option<CatalogueUnit> },
    Null { state: NullState },
    Reference { target_id: String },
    List { items: Vec<CatalogueValue> },
}

/// ∅ Value availability states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum NullState {
    Unavailable,
    Unknown,
    /// 🔡️ `NotApplicable` auto-kebabs to `not-applicable`; kept camelCase to match this crate's
    /// ISO-16757-native external property naming convention (see `SubjectKind`/`RelationshipKind`).
    #[dsl(key = "notApplicable")]
    NotApplicable,
}

/// 🔢️ Cardinality constraint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Cardinality {
    pub min: u32,
    pub max: Option<u32>,
}

impl Cardinality {
    pub fn optional() -> Self {
        Self { min: 0, max: Some(1) }
    }

    pub fn required() -> Self {
        Self { min: 1, max: Some(1) }
    }

    pub fn unbounded() -> Self {
        Self { min: 0, max: None }
    }

    pub fn satisfies(&self, count: u32) -> bool {
        count >= self.min && self.max.is_none_or(|max| count <= max)
    }
}

/// 🧩️ Lossless extension bag for unknown fields.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ExtensionBag {
    pub fields: BTreeMap<String, semio_framework_value::DslValue>,
}

/// 📅️ Lifecycle metadata.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Lifecycle {
    pub revision: String,
    pub status: String,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}

pub mod part_1 {
    use crate::*;

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
}

pub mod part_2 {
    use crate::*;

    /// 📐️ Space classification per Part 2 §5.3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum SpaceKind {
        Overall,
        Operation,
        Access,
        #[dsl(key = "placementTransportation")]
        PlacementTransportation,
        Installation,
    }

    /// 🔌️ Port medium and direction.
    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct PortDefinition {
        pub id: String,
        pub medium: String,
        pub position: [f64; 3],
        pub direction: [f64; 3],
        pub port_type: String,
    }

    /// 📦️ Axis-aligned bounding box.
    #[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct BoundingBox {
        pub min: [f64; 3],
        pub max: [f64; 3],
    }

    impl BoundingBox {
        pub fn from_size(width: f64, height: f64, depth: f64) -> Self {
            Self { min: [0.0, 0.0, 0.0], max: [width, height, depth] }
        }

        pub fn volume_m3(self) -> f64 {
            let dx = self.max[0] - self.min[0];
            let dy = self.max[1] - self.min[1];
            let dz = self.max[2] - self.min[2];
            dx * dy * dz
        }

        pub fn overlaps(self, other: Self, clearance_m: f64) -> bool {
            self.min[0] - clearance_m < other.max[0]
                && self.max[0] + clearance_m > other.min[0]
                && self.min[1] - clearance_m < other.max[1]
                && self.max[1] + clearance_m > other.min[1]
                && self.min[2] - clearance_m < other.max[2]
                && self.max[2] + clearance_m > other.min[2]
        }
    }

    /// 🧱️ CSG primitive kind registry entry.
    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct PrimitiveKind {
        pub id: String,
        pub parameters: Vec<String>,
    }

    /// 🌳️ Geometry node in a CSG tree — recursive `#[derive(dsl::DslEnum)]`: `Transform.child` is
    /// `#[dsl(statements)] Box<GeometryNode>` (exactly one nested tagged value) and
    /// `Boolean.children` is `#[dsl(statements, block)] Vec<GeometryNode>` (a nested tagged
    /// collection), both recursing back into this same enum's own `DslVariants` impl.
    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(tag = "node", rename_all = "camelCase", rename_all_fields = "camelCase"))]
    #[value(tag = "node", rename_all = "camelCase")]
    #[dsl(retire_with="crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_catalogue_geometry")]
    pub enum GeometryNode {
        Primitive {
            kind: String,
            parameters: BTreeMap<String, f64>,
        },
        Transform {
            translation: [f64; 3],
            rotation_deg: [f64; 3],
            #[dsl(statements, block)]
            child: Box<GeometryNode>,
        },
        Boolean {
            operator: BooleanOperator,
            #[dsl(statements, block)]
            children: Vec<GeometryNode>,
        },
        Reference {
            geometry_id: String,
        },
    }

    /// ➕️ Boolean CSG operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum BooleanOperator {
        Union,
        Intersection,
        Difference,
    }

    /// 🏗️ Complete geometry object.
    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct GeometryObject {
        pub id: String,
        #[dsl(statements, block)]
        pub shape: Option<GeometryNode>,
        #[dsl(statements, block)]
        pub symbolic: Option<GeometryNode>,
        pub spaces: Vec<SpaceEnvelope>,
        pub surfaces: Vec<SurfaceDefinition>,
        pub ports: Vec<PortDefinition>,
        pub parameter_bindings: BTreeMap<String, String>,
    }

    /// 📦️ Space envelope with kind.
    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct SpaceEnvelope {
        #[cfg_attr(test, serde(default = "default_installation_space_id"))]
        pub id: String,
        pub kind: SpaceKind,
        pub bounds: BoundingBox,
    }

    fn default_installation_space_id() -> String {
        "installation".into()
    }

    /// 🎨️ Semantic surface.
    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct SurfaceDefinition {
        pub id: String,
        pub purpose: String,
        pub bounds: BoundingBox,
    }

    /// 📚️ Geometry catalogue index.
    #[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct GeometryCatalogue {
        pub objects: BTreeMap<String, GeometryObject>,
        #[dsl(table)]
        pub primitive_registry: Vec<PrimitiveKind>,
    }

    impl GeometryCatalogue {
        pub fn default_primitives() -> Vec<PrimitiveKind> {
            vec![
                PrimitiveKind { id: "box".into(), parameters: vec!["width".into(), "height".into(), "depth".into()] },
                PrimitiveKind { id: "cylinder".into(), parameters: vec!["radius".into(), "height".into()] },
                PrimitiveKind { id: "sphere".into(), parameters: vec!["radius".into()] },
            ]
        }
    }
}

pub mod part_4 {
    use crate::*;

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
}

pub mod part_5 {
    use crate::*;

    /// 🔄️ Exchange process stage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum ExchangeProcess {
        #[dsl(key = "createFromDictionary")]
        CreateFromDictionary,
        #[dsl(key = "provideCatalogue")]
        ProvideCatalogue,
        #[dsl(key = "determineProduct")]
        DetermineProduct,
        #[dsl(key = "integrateIntoSystem")]
        IntegrateIntoSystem,
        #[dsl(key = "exchangeSystemModel")]
        ExchangeSystemModel,
    }

    /// 🔢️ Part number rule.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum PartNumberRule {
        Literal { value: String },
        Table { rows: Vec<BTreeMap<String, String>>, output_column: String },
        Script { function_id: String, source: String },
    }

    

    /// 📄️ External media reference.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct ExternalMedia {
        pub id: String,
        pub uri: String,
        pub media_type: String,
        pub checksum: Option<String>,
        pub language: Option<String>,
    }

    /// 🏛️ Minimal IFC catalogue node.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct IfcCatalogueNode {
        pub entity_type: String,
        pub global_id: String,
        pub name: String,
        pub attributes: std::collections::HashMap<String, String>,
        pub children: Vec<IfcCatalogueNode>,
    }

    /// 📦️ IFC catalogue root.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct IfcCatalogue {
        pub schema: String,
        pub metadata: IfcCatalogueNode,
        pub product_classes: Vec<IfcCatalogueNode>,
        pub products: Vec<IfcCatalogueNode>,
        pub unknown_entities: Vec<String>,
    }

    /// 🧮️ Script execution limits.
    #[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct ScriptLimits {
        pub max_steps: u32,
        pub max_recursion: u32,
        pub timeout_ms: u64,
    }

    impl Default for ScriptLimits {
        fn default() -> Self {
            Self { max_steps: 10_000, max_recursion: 64, timeout_ms: 50 }
        }
    }

    /// 📤️ Script execution result.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[value(rename_all = "camelCase")]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub struct ScriptResult {
        pub value: f64,
        pub diagnostics: Vec<String>,
    }

    /// ⚠️ Script execution error.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ScriptError {
        Timeout(u64),
        RecursionLimit(u32),
        StepLimit(u32),
        InvalidExpression(String),
        MissingInput(String),
    }

    impl std::fmt::Display for ScriptError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Timeout(milliseconds) => write!(formatter, "timeout after {milliseconds} ms"),
                Self::RecursionLimit(limit) => write!(formatter, "recursion limit {limit} exceeded"),
                Self::StepLimit(limit) => write!(formatter, "step limit {limit} exceeded"),
                Self::InvalidExpression(expression) => write!(formatter, "invalid expression: {expression}"),
                Self::MissingInput(input) => write!(formatter, "missing input: {input}"),
            }
        }
    }

    impl std::error::Error for ScriptError {}
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("iso16757", "ISO 16757", ISO16757_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const ISO16757_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.iso16757", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const ISO16757_DOCUMENT_SCHEMA: &str = "semio.norm.iso16757/v1";

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.iso16757" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.iso16757.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.norm.iso16757@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.iso16757/v1" }, Q { namespace: "codec-extension", value: "22:semio.norm.iso16757/v1:iso16757" }];
    const EN: &[L] = &[L { locale: "en", text: "ISO 16757 building-services product catalogue exchange" }];
    const DE: &[L] = &[L { locale: "de", text: "ISO 16757 Austausch von Produktdaten der technischen Gebäudeausrüstung" }];
    const ROWS: &[C] = &[
        C { identity: "s.norm.iso16757.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.norm.iso16757.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.norm.iso16757.schema.artifact", kind: "schema", descriptor: "s.norm.iso16757", claims: S, localizations: &[] },
        C { identity: "s.norm.iso16757.inference.outline", kind: "inference", descriptor: "s.norm.iso16757.inference", claims: I, localizations: &[] },
        C { identity: "s.norm.iso16757.composer.any", kind: "composer", descriptor: "s.norm.iso16757@1/*", claims: M, localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.document", kind: "grammar", descriptor: "iso16757.document", claims: &[Q { namespace: "grammar", value: "iso16757.document" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.op", kind: "grammar", descriptor: "iso16757.op", claims: &[Q { namespace: "grammar", value: "iso16757.op" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.diff", kind: "grammar", descriptor: "iso16757.diff", claims: &[Q { namespace: "grammar", value: "iso16757.diff" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.pack", kind: "grammar", descriptor: "iso16757.pack", claims: &[Q { namespace: "grammar", value: "iso16757.pack" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.spr", kind: "grammar", descriptor: "iso16757.spr", claims: &[Q { namespace: "grammar", value: "iso16757.spr" }], localizations: &[] },
        C { identity: "s.norm.iso16757.codec.document.v1", kind: "codec", descriptor: "semio.norm.iso16757/v1:iso16757", claims: K, localizations: &[] },
        C { identity: "s.norm.iso16757.localization.en", kind: "localization", descriptor: "ISO 16757 building-services product catalogue exchange", claims: &[], localizations: EN },
        C { identity: "s.norm.iso16757.localization.de", kind: "localization", descriptor: "ISO 16757 Austausch von Produktdaten der technischen Gebäudeausrüstung", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.iso16757", ROWS)
}
