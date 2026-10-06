//! 🚪️ Artifact representation module ownership.

#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;

#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;

#[path = "🔺️diff/🦀️.rs"]
pub mod diff;

#[path="💡️inferences/🦀️.rs"]
pub mod inferences;

/// 🔤️ Encodes the artifact's intrinsic byte values as compact physical base64 fields.
pub(crate) fn bitmap_json_value<T: semio_framework_value::ToValue>(value: &T) -> semio_framework_value::DslValue {
    fn encode(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
        use semio_framework_value::DslValue;
        match value {
            DslValue::Bytes(bytes) => DslValue::String(snapshot::encode_base64(&bytes)),
            DslValue::Array(values) => DslValue::Array(values.into_iter().map(encode).collect()),
            DslValue::Object(fields) => DslValue::Object(fields.into_iter().map(|(name, value)| (name, encode(value))).collect()),
            value => value,
        }
    }
    encode(semio_framework_value::ToValue::to_value(value))
}

/// 📤️ Renders decoded artifact values using the declared physical JSON byte representation.
pub(crate) fn bitmap_json_encode<T: semio_framework_value::ToValue>(value: &T) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&bitmap_json_value(value)))
}

/// 📥️ Binds declared base64 byte fields before semantic type validation.
pub(crate) fn bitmap_json_bind<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<T, semio_framework_value::ValueError> {
    fn decode(value: semio_framework_value::DslValue) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> {
        use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};
        match value {
            DslValue::Array(values) => Ok(DslValue::Array(values.into_iter().enumerate().map(|(index, value)| decode(value).map_err(|error| error.under(index.to_string()))).collect::<Result<_, _>>()?)),
            DslValue::Object(fields) => Ok(DslValue::Object(fields.into_iter().map(|(name, value)| {
                let result = if matches!(name.as_str(), "pixels" | "inputPixels") {
                    match value {
                        DslValue::Null if name == "inputPixels" => Ok(DslValue::Null),
                        DslValue::String(encoded) => snapshot::decode_pixel_text(&encoded).map(DslValue::Bytes),
                        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "physical byte fields require base64 strings")),
                    }
                } else { decode(value) };
                result.map(|value| (name.clone(), value)).map_err(|error| error.under(name))
            }).collect::<Result<_, _>>()?)),
            value => Ok(value),
        }
    }
    semio_framework_value::FromValue::from_value(decode(value)?)
}

/// 📖️ Parses physical JSON independently from the semantic intrinsic byte binder.
pub(crate) fn bitmap_json_decode<T: semio_framework_value::FromValue>(text: &str) -> Result<T, semio_framework_value::ValueError> {
    let value = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
    bitmap_json_bind(semio_framework_pack_json::to_dsl_value(&value))
}

