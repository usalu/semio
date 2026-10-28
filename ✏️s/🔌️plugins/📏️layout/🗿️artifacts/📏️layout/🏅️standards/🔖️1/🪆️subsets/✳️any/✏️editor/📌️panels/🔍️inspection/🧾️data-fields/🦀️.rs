//! 🧾️ Typed data fields expose every intrinsic member through named controls.

use super::*;
use semio_framework_value::DslValue;

pub(super) struct DataRow<'a> { path: String, label: String, key: Option<&'a str>, value: &'a DslValue }

pub(super) fn rows(document: &LayoutSnapshot) -> Vec<DataRow<'_>> {
    let mut rows = Vec::new();
    let mut pending = Vec::new();
    if let Some(dictionary) = &document.data_fields {
        for (index, entry) in dictionary.entries.iter().enumerate().rev() { pending.push(DataRow { path: format!("data.{index}"), label: entry.question_id.clone(), key: Some(&entry.question_id), value: &entry.value }); }
    }
    while let Some(row) = pending.pop() {
        match row.value {
            DslValue::Object(values) => for (index, (key, value)) in values.iter().enumerate().rev() { pending.push(DataRow { path: format!("{}.{index}", row.path), label: format!("{} / {key}", row.label), key: Some(key), value }); },
            DslValue::Array(values) => for (index, value) in values.iter().enumerate().rev() { pending.push(DataRow { path: format!("{}.{index}", row.path), label: format!("{} / {}", row.label, index + 1), key: None, value }); },
            _ => {}
        }
        rows.push(row);
    }
    rows
}

pub(super) fn render(windows: &semio_framework_plugin::TreeWindows<'_>, row: &DataRow<'_>, labels: &LayoutLabels, stamp: &str) -> UiAssemblyResult<BuiltNode> {
    let prefix = &row.path;
    let mut children = Vec::new();
    if let Some(key) = row.key { children.push(document_input(&format!("layout-play-inspector.patchDocument.{prefix}.key"), &format!("{prefix}.key.at.{stamp}"), labels.data_key, key.into(), InputKind::Text)?); }
    let kind = match row.value { DslValue::Null => "null", DslValue::Bool(_) => "boolean", DslValue::Number(_) => "number", DslValue::String(_) => "text", DslValue::Bytes(_) => "bytes", DslValue::Array(_) => "list", DslValue::Object(_) => "object" };
    children.push(select_row_at(&format!("layout-play-inspector.patchDocument.{prefix}.kind"), "patchDocument", &format!("{prefix}.kind.at.{stamp}"), labels.kind, kind, [("text", labels.data_text.as_str()), ("number", labels.data_number.as_str()), ("boolean", labels.data_boolean.as_str()), ("null", labels.data_null.as_str()), ("list", labels.data_list.as_str()), ("object", labels.data_object.as_str()), ("bytes", labels.data_bytes.as_str())], None, None)?);
    match row.value {
        DslValue::String(value) => children.push(chunked_text(windows, &format!("layout-play-inspector.dictionary.{prefix}.value"), &format!("{prefix}.value"), value, labels.data_value, stamp)?),
        DslValue::Number(_) => children.push(document_input(&format!("layout-play-inspector.patchDocument.{prefix}.value"), &format!("{prefix}.value.at.{stamp}"), labels.data_value, semio_framework_pack_json::to_json_string(row.value), InputKind::Number)?),
        DslValue::Bool(value) => children.push(document_toggle_at(&format!("layout-play-inspector.patchDocument.{prefix}.value"), &format!("{prefix}.value.at.{stamp}"), *value, labels.data_value)?),
        DslValue::Bytes(bytes) => children.push(chunked_bytes(windows, &format!("layout-play-inspector.dictionary.{prefix}.value.bytes"), &format!("{prefix}.value"), bytes, labels.data_bytes, stamp)?),
        DslValue::Array(_) | DslValue::Object(_) => children.push(document_button_at(&format!("layout-play-inspector.patchDocument.{prefix}.add"), &format!("{prefix}.add.at.{stamp}"), "true", labels.add_data_member)?),
        DslValue::Null => {}
    }
    children.push(document_button_at(&format!("layout-play-inspector.patchDocument.{prefix}.delete"), &format!("{prefix}.delete.at.{stamp}"), "true", labels.delete_data_member)?);
    grouped(&format!("layout-play-inspector.{prefix}"), &row.label, children)
}
