//! 📍 Source spans for DSL diagnostics and tokens.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

use serde::{Deserialize, Serialize};

//#region 🔖️Span
/// 📍️ 1-based line/column position with a length, covering a run of source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSpan {
    pub line: u32,
    pub column: u32,
    pub length: u32,
}

/// 🌉️ Owned Value conversion preserves the span's literal camelCase wire keys.
/// All three scalar field names retain their declared diagnostic identity.
impl semio_framework_value::ToValue for TextSpan {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> { crate::component::controlled::encode_span(self, c) }
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Object(vec![
            ("line".to_string(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(u64::from(self.line)))),
            ("column".to_string(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(u64::from(self.column)))),
            ("length".to_string(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(u64::from(self.length)))),
        ])
    }
}

/// 🌉️ Mirror of the `ToValue` bridge above.
impl semio_framework_value::FromValue for TextSpan {
    fn from_value_controlled(value: &semio_framework_value::DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { crate::component::controlled::decode_span(value, c) }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { c.checkpoint().map_err(semio_framework_value::ValueError::new)?; Ok(Self::default()) }
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = match value {
            semio_framework_value::DslValue::Object(entries) => entries,
            other => return Err(semio_framework_value::ValueError::new(format!("expected an object for TextSpan, found {other:?}"))),
        };
        let field = |key: &str| -> Result<u32, semio_framework_value::ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                Some(semio_framework_value::DslValue::Number(number)) => Ok(number.as_u64().unwrap_or(0) as u32),
                None => Ok(0),
                Some(other) => Err(semio_framework_value::ValueError::new(format!("expected a number for TextSpan.{key}, found {other:?}"))),
            }
        };
        Ok(TextSpan { line: field("line")?, column: field("column")?, length: field("length")? })
    }
}

impl TextSpan {
    pub fn at(line: u32, column: u32) -> Self {
        Self { line, column, length: 0 }
    }

    pub fn with_length(line: u32, column: u32, length: u32) -> Self {
        Self { line, column, length }
    }
}
