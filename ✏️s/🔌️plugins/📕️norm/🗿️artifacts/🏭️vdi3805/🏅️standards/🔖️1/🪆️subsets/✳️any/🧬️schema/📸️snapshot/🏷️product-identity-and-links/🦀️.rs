//! 🏷️ Product identity and links of the VDI 3805 snapshot: the extension bag, the product identity, the manufacturer file and the accessory and composition links. See [`super`].

use super::record_families::BuildingSystemNumber;
use std::collections::BTreeMap;

/// 🧩️ Typed extension bag (`string → string`) matching JSON Schema `ExtensionFields.fields`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ExtensionBag {
    pub fields: BTreeMap<String, String>,
}

/// 🆔️ Product identity within a manufacturer catalogue.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ProductIdentity {
    pub manufacturer_code: String,
    pub product_group: String,
    pub article_number: String,
}

/// 🏭️ Manufacturer file header and payload references.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ManufacturerFile {
    pub header_version: String,
    pub manufacturer: String,
    pub building_system_number: BuildingSystemNumber,
    pub created: String,
    pub charset: String,
    pub record_count: u32,
    pub extensions: ExtensionBag,
}

/// 🔗️ Accessory relationship between products.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AccessoryLink {
    pub accessory_id: String,
    pub required: bool,
    pub quantity: u32,
}

/// 🧱️ Composition relationship (`hasPart`).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositionLink {
    pub component_id: String,
    pub quantity: u32,
}
