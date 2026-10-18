//! 🧱️ Catalogue core of the ISO 16757 snapshot: the catalogue identifier, dictionary reference, names, dimension signature, unit, value, null state, cardinality, extension bag and lifecycle. See [`super`].

use semio_s_plugin_norm_compliance::LocalizedText;
use std::collections::BTreeMap;

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
