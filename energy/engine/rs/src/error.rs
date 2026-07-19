//! ⚠️ Simulation and model error taxonomy.

use std::fmt;

// #region 🔖Severity
/// 🚨 Diagnostic severity aligned with BEM engine conventions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Fatal,
    Severe,
    Warning,
    RecurringWarning,
}
// #endregion 🔖Severity

// #region 🔖Error
/// ❌ Recoverable or fatal engine error with optional location context.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Error {
    pub severity: Severity,
    pub message: String,
    pub context: Option<String>,
}

impl Error {
    pub fn fatal(message: impl Into<String>) -> Self {
        Self { severity: Severity::Fatal, message: message.into(), context: None }
    }

    pub fn severe(message: impl Into<String>) -> Self {
        Self { severity: Severity::Severe, message: message.into(), context: None }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self { severity: Severity::Warning, message: message.into(), context: None }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ctx) = &self.context {
            write!(f, "[{:?}] {} ({})", self.severity, self.message, ctx)
        } else {
            write!(f, "[{:?}] {}", self.severity, self.message)
        }
    }
}

impl std::error::Error for Error {}
// #endregion 🔖Error

// #region 🔖Diagnostics
/// 📋 Collected diagnostics from validation or simulation.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Diagnostics {
    pub messages: Vec<Error>,
}

impl Diagnostics {
    pub fn push(&mut self, err: Error) {
        self.messages.push(err);
    }

    pub fn has_fatal(&self) -> bool {
        self.messages.iter().any(|e| e.severity == Severity::Fatal)
    }

    pub fn merge(&mut self, other: Diagnostics) {
        self.messages.extend(other.messages);
    }
}
// #endregion 🔖Diagnostics

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fatal_has_correct_severity() {
        let e = Error::fatal("bad model");
        assert_eq!(e.severity, Severity::Fatal);
    }
}
