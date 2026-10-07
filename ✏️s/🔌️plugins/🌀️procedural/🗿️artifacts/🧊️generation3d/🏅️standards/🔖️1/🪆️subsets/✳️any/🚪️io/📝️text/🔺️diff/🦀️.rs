//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Generation3dDiffText = String;

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
use semio_framework_value::{DslValue, FromValue, ToValue, ValueRefusalKind};

impl Generation3dDiff {
    pub(crate) fn __dsl_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "artifact", Shape::Value).optional(), FieldSpec::new(1, "hostSnapshot", Shape::Value).optional(), FieldSpec::new(2, "generation", Shape::Value).optional()])
    }

    pub(crate) fn __dsl_to_record(&self) -> RecordValue {
        let mut record = RecordValue::default();
        if let Some(value) = &self.artifact { record.fields.insert(0, FieldValue::Value(value.to_value())); }
        if let Some(value) = &self.host_snapshot { record.fields.insert(1, FieldValue::Value(value.to_value())); }
        if let Some(value) = &self.generation { record.fields.insert(2, FieldValue::Value(value.to_value())); }
        record
    }

    pub(crate) fn __dsl_from_record(record: &RecordValue) -> Result<Self, semio_framework_diagnostic::TextError> {
        let mut fields = Vec::with_capacity(3);
        for (id, key) in [(0, "artifact"), (1, "hostSnapshot"), (2, "generation")] {
            let value = match record.get(id) {
                None | Some(FieldValue::Absent) => DslValue::Null,
                Some(FieldValue::Value(value)) => value.clone(),
                _ => return Err(semio_framework_diagnostic::TextError::new(ValueRefusalKind::InvalidValue, "expected original generation3d diff field value", semio_framework_diagnostic::TextSpan::at(1, 1))),
            };
            fields.push((key.into(), value));
        }
        Self::from_value(DslValue::Object(fields)).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, error.message, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Generation3dDiff);

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
