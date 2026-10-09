//! 🧱️ The one place that names the typed value of the `s.stdio.json` artifact: members and values in, the RFC 8259 document text out (two-space indent, members in the order given), and the text read back into the same values.
//! 📎 https://www.rfc-editor.org/rfc/rfc8259

use semio_s_artifact_stdio_json::schema::snapshot::{JsonMember, JsonValue};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::{parse_json_text, write_json_pretty};

/// 🧾️ A JSON object with the members in the order given.
pub fn object<'a>(members: impl IntoIterator<Item = (&'a str, JsonValue)>) -> JsonValue {
    JsonValue::Object { members: members.into_iter().map(|(key, value)| JsonMember { key: key.to_string(), value }).collect() }
}

/// 🧾️ A JSON array.
pub fn array(items: impl IntoIterator<Item = JsonValue>) -> JsonValue {
    JsonValue::Array { items: items.into_iter().collect() }
}

/// 🔤️ A JSON string.
pub fn string(value: impl Into<String>) -> JsonValue {
    JsonValue::String { value: value.into() }
}

/// 🔢️ A JSON number with the shortest decimal text that reads back as the same `f64`; a number that is not finite is `null`, which JSON has no number for.
pub fn number(value: f64) -> JsonValue {
    if value.is_finite() {
        JsonValue::Number { lexeme: format!("{value}") }
    } else {
        JsonValue::Null
    }
}

/// 🔢️ A JSON integer.
pub fn count(value: u32) -> JsonValue {
    JsonValue::Number { lexeme: value.to_string() }
}

/// 📄️ The RFC 8259 text of a value, with a final newline.
pub fn document_text(value: &JsonValue) -> String {
    let mut text = write_json_pretty(value);
    text.push('\n');
    text
}

/// 📥️ The value of RFC 8259 text: the reading half the tests and the feature adapter use to check the writer against its own decoder.
pub fn read_value(text: &str) -> Result<JsonValue, String> {
    parse_json_text(text).map_err(|error| error.to_string())
}

/// 🔎️ The member `key` of an object value.
pub fn member<'a>(value: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    match value {
        JsonValue::Object { members } => members.iter().find(|member| member.key == key).map(|member| &member.value),
        _ => None,
    }
}

/// 🔤️ The text of a string value.
pub fn text_of(value: &JsonValue) -> Option<&str> {
    match value {
        JsonValue::String { value } => Some(value),
        _ => None,
    }
}

/// 🔢️ The number of a number value.
pub fn number_of(value: &JsonValue) -> Option<f64> {
    match value {
        JsonValue::Number { lexeme } => lexeme.parse().ok(),
        _ => None,
    }
}

/// 🧾️ The items of an array value.
pub fn items_of(value: &JsonValue) -> &[JsonValue] {
    match value {
        JsonValue::Array { items } => items,
        _ => &[],
    }
}

/// 🧾️ The members of an object value.
pub fn members_of(value: &JsonValue) -> &[JsonMember] {
    match value {
        JsonValue::Object { members } => members,
        _ => &[],
    }
}
