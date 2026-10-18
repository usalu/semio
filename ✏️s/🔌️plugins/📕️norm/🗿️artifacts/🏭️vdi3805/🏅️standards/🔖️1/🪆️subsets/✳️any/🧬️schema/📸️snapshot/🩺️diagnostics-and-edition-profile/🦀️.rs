//! 🩺️ Diagnostics and edition profile of the VDI 3805 snapshot: the diagnostic record with its severity and the edition profile choice. See [`super`].

/// 🩺️ Validation diagnostic with severity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Diagnostic {
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

/// ⚠️ Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Diagnostic {
    pub fn info(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into(), severity: Severity::Info }
    }

    pub fn warning(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into(), severity: Severity::Warning }
    }

    pub fn error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into(), severity: Severity::Error }
    }
}

/// 📅️ Edition profile selection for multi-profile sheets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum EditionProfileChoice {
    Legacy,
    Current,
}
