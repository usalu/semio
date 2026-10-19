//! 🧾️ Properties of the BIM entity table: the property types, texts, values and entries. See [`super`].

use crate::editor::bim::entities::*;
use semio_s_artifact_bim_model_authored::editor::bim::entities::field_codecs::number;

//#region 🔖️Properties
/// 🏷️ The measure kinds of a typed property value, in the order the entry grammar names them.
pub const PROPERTY_TYPES: &[&str] = &["text", "real", "integer", "boolean", "length", "area", "volume", "angle"];

/// 🏷️ The kind name of a property value.
pub fn property_type(value: &semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue) -> &'static str {
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue::*;
    match value {
        Text { .. } => "text",
        Real { .. } => "real",
        Integer { .. } => "integer",
        Boolean { .. } => "boolean",
        Length { .. } => "length",
        Area { .. } => "area",
        Volume { .. } => "volume",
        Angle { .. } => "angle",
    }
}

/// 🏷️ The text of a property value, without its kind.
pub fn property_text(value: &semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue) -> String {
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue::*;
    match value {
        Text { value } => value.clone(),
        Real { value } | Length { value } | Area { value } | Volume { value } | Angle { value } => number(*value),
        Integer { value } => value.to_string(),
        Boolean { value } => value.to_string(),
    }
}

/// 🏷️ The property value a text names: of the given kind, else inferred (`true` and `false` are flags, whole numbers are integers, other numbers are reals, anything else is text).
pub fn property_value(text: &str, kind: Option<&str>) -> Option<semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue> {
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::PropertyValue::*;
    let text = text.trim();
    let measure = || text.parse::<f64>().ok().filter(|value| value.is_finite());
    match kind.map(str::to_lowercase).as_deref() {
        Some("text") => Some(Text { value: text.to_string() }),
        Some("real") => measure().map(|value| Real { value }),
        Some("integer") => text.parse().ok().map(|value| Integer { value }),
        Some("boolean") => text.parse().ok().map(|value| Boolean { value }),
        Some("length") => measure().map(|value| Length { value }),
        Some("area") => measure().map(|value| Area { value }),
        Some("volume") => measure().map(|value| Volume { value }),
        Some("angle") => measure().map(|value| Angle { value }),
        Some(_) => None,
        None => Some(match text {
            "true" => Boolean { value: true },
            "false" => Boolean { value: false },
            _ => text.parse().ok().map(|value| Integer { value }).or_else(|| measure().map(|value| Real { value })).unwrap_or_else(|| Text { value: text.to_string() }),
        }),
    }
}

/// 🏷️ One typed entry `Set.Property[:kind] = value`.
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyEntry {
    pub pset: String,
    pub property: String,
    pub kind: Option<String>,
    pub value: String,
}

/// 🏷️ The entry a text names: the property set before the first dot, the property name after it, an optional `:kind`, then `=` and the value.
pub fn property_entry(text: &str) -> Option<PropertyEntry> {
    let (name, value) = text.split_once('=')?;
    let (pset, property) = name.trim().split_once('.')?;
    let (property, kind) = match property.rsplit_once(':') {
        Some((property, kind)) => (property, Some(kind.trim().to_lowercase())),
        None => (property, None),
    };
    let (pset, property) = (pset.trim(), property.trim());
    (!pset.is_empty() && !property.is_empty()).then(|| PropertyEntry { pset: pset.to_string(), property: property.to_string(), kind, value: value.trim().to_string() })
}
//#endregion 🔖️Properties
