//! 🗃️ Bulk edit of a multi-selection: one field written to every selected row as one `setFields` bundle (one undo step), and the properties view of a field over the selection (common value or mixed). Mounted in the panels crate, below the editor, so the properties window renders it.

use crate::*;
use semio_framework_value::DslValue;

/// 🗃️ What the properties window shows for one field over the selected rows.
#[derive(Clone, Debug, PartialEq)]
pub enum SharedValue {
    Same(DslValue),
    Mixed,
    Absent,
}

/// ✍️ The writes that set `field` of every selected row of `collection` to `value`; ids that do not exist are an error so a stale selection never writes half a bundle.
pub fn bulk_writes(tree: &DslValue, collection: &str, ids: &[String], field: &str, value: &DslValue) -> Result<Vec<FieldWrite>, String> {
    let rows = get_value_at_path(tree, collection)?.as_array().ok_or_else(|| format!("{collection} is not a list"))?;
    ids.iter()
        .map(|id| {
            let (index, row) = rows.iter().enumerate().find(|(_, row)| object_id_field(row) == Some(id.as_str())).ok_or_else(|| format!("unknown id '{id}' in {collection}"))?;
            Ok(FieldWrite { path: format!("{}.{field}", list_element_path(collection, row, index)), value: value.clone() })
        })
        .collect()
}

/// 🔎️ The value of `field` over the selected rows: the common value, mixed when they differ, absent when no selected row has it.
pub fn shared_value(tree: &DslValue, collection: &str, ids: &[String], field: &str) -> SharedValue {
    let Ok(Some(rows)) = get_value_at_path(tree, collection).map(DslValue::as_array) else { return SharedValue::Absent };
    let mut values = rows.iter().filter(|row| object_id_field(row).is_some_and(|id| ids.iter().any(|wanted| wanted == id))).filter_map(|row| row.get(field));
    let Some(first) = values.next() else { return SharedValue::Absent };
    if values.all(|value| value == first) { SharedValue::Same(first.clone()) } else { SharedValue::Mixed }
}

/// 🔢️ A relative bulk edit: every selected row gets its own value plus `delta`, so "move all by 0.5 m" keeps their spacing. Rows without a numeric `field` are skipped.
pub fn bulk_delta_writes(tree: &DslValue, collection: &str, ids: &[String], field: &str, delta: f64) -> Result<Vec<FieldWrite>, String> {
    let rows = get_value_at_path(tree, collection)?.as_array().ok_or_else(|| format!("{collection} is not a list"))?;
    let mut writes = Vec::new();
    for id in ids {
        let (index, row) = rows.iter().enumerate().find(|(_, row)| object_id_field(row) == Some(id.as_str())).ok_or_else(|| format!("unknown id '{id}' in {collection}"))?;
        if let Some(current) = row.get(field).and_then(DslValue::as_f64) {
            writes.push(FieldWrite { path: format!("{}.{field}", list_element_path(collection, row, index)), value: DslValue::float(current + delta) });
        }
    }
    Ok(writes)
}

/// 🔤️ The label of the mixed state, EN and DE.
pub fn mixed_label(german: bool) -> &'static str {
    if german { "Gemischt" } else { "Mixed" }
}

/// 🗃️ One line of the properties view over a multi-selection: the field, its text (the common value or the mixed label) and whether the selected rows disagree.
#[derive(Clone, Debug, PartialEq)]
pub struct BulkRow {
    pub field: String,
    pub text: String,
    pub mixed: bool,
}

fn value_text(value: &DslValue) -> String {
    match value {
        DslValue::String(text) => text.clone(),
        other => semio_framework_pack_json::to_json_string(other),
    }
}

/// 🗃️ The properties lines of the selected rows: every field any selected row has (except `id`) in first-seen order, with the common value or the localized mixed label, so the window shows "Mixed" instead of the value of one arbitrary row.
pub fn bulk_rows(tree: &DslValue, collection: &str, ids: &[String], german: bool) -> Vec<BulkRow> {
    let Ok(Some(rows)) = get_value_at_path(tree, collection).map(DslValue::as_array) else { return Vec::new() };
    let mut fields: Vec<String> = Vec::new();
    for row in rows.iter().filter(|row| object_id_field(row).is_some_and(|id| ids.iter().any(|wanted| wanted == id))) {
        if let DslValue::Object(entries) = row {
            for (key, _) in entries.iter().filter(|(key, _)| key != "id") {
                if !fields.contains(key) {
                    fields.push(key.clone());
                }
            }
        }
    }
    fields
        .into_iter()
        .filter_map(|field| match shared_value(tree, collection, ids, &field) {
            SharedValue::Same(value) => Some(BulkRow { text: value_text(&value), field, mixed: false }),
            SharedValue::Mixed => Some(BulkRow { text: mixed_label(german).to_string(), field, mixed: true }),
            SharedValue::Absent => None,
        })
        .collect()
}
