//! 🔄️ Part 5 of the ISO 16757 snapshot: the exchange process, part number rule, external media, IFC catalogue and the script limits, result and error. See [`super`].

use std::collections::BTreeMap;

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
