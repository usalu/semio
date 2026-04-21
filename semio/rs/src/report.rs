use serde::{Deserialize, Serialize};

/// A single note attached to the outcome of an operation.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct OperationNote {
    #[serde(default)]
    pub severity: NoteSeverity,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum NoteSeverity {
    #[default]
    Info,
    Warning,
    Error,
}

/// Outcome of an operation, always carrying the completeness flag and the
/// collected notes. For structural operations the payload lives in `value`.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SemioReport<T> {
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub infos: Vec<OperationNote>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<OperationNote>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<OperationNote>,
}

impl<T> SemioReport<T> {
    pub fn ok(value: T) -> Self {
        Self { ok: true, value: Some(value), infos: Vec::new(), warnings: Vec::new(), errors: Vec::new() }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            value: None,
            infos: Vec::new(),
            warnings: Vec::new(),
            errors: vec![OperationNote { severity: NoteSeverity::Error, message: message.into(), pointer: None }],
        }
    }

    pub fn with_infos(mut self, infos: Vec<OperationNote>) -> Self {
        self.infos = infos;
        self
    }

    pub fn with_warnings(mut self, warnings: Vec<OperationNote>) -> Self {
        self.warnings = warnings;
        self
    }
}

/// Outcome of a validation pass.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self { is_valid: true, errors: Vec::new(), warnings: Vec::new() }
    }

    pub fn with_error(msg: impl Into<String>) -> Self {
        Self { is_valid: false, errors: vec![msg.into()], warnings: Vec::new() }
    }
}
