//! ➕ Insert tool: a palette of the insertable entity kinds, grid snapping of the cursor and the `insertItem` edit of a placement. The ghost follows the cursor in the transient lane; the click commits one edit. See [`super`].

use crate::{get_value_at_path, set_value_at_path, NormEdit};
use semio_framework_value::DslValue;
use semio_s_plugin_norm_structural_scene::EntityKind;

/// 🧲️ Grid step of a placement in metres; Ctrl frees the cursor.
pub const INSERT_GRID: f64 = 0.05;

/// ➕ One tool of the palette: the document collection a click inserts into.
#[derive(Clone, Debug, PartialEq)]
pub struct InsertSpec {
    pub id: String,
    pub collection: String,
    pub kind: EntityKind,
    pub label_en: String,
    pub label_de: String,
}

/// 🧲️ The cursor snapped to the grid; `free` keeps it exact.
pub fn snap_point(at: [f64; 3], free: bool) -> [f64; 3] {
    if free {
        return at;
    }
    at.map(|c| (c / INSERT_GRID).round() * INSERT_GRID + 0.0)
}

fn number(value: f64) -> DslValue {
    DslValue::float(value)
}

fn known_ids(items: &[DslValue]) -> Vec<String> {
    items.iter().filter_map(|item| item.get("id").and_then(DslValue::as_str)).map(str::to_string).collect()
}

fn fresh_id(spec: &InsertSpec, taken: &[String]) -> String {
    (taken.len() + 1..).map(|n| format!("{}-{n}", spec.id)).find(|id| !taken.contains(id)).unwrap_or_default()
}

/// ➕ The edit that places a new item of `spec` at `at`: a copy of the last item of the collection (a dotted `spec.collection` path such as `geometry.objects`, an array or a map keyed by entity id) (a document-valid starting point) with a fresh id and, where the item has them, the position fields `x`, `y`, `z` set; an empty collection inserts the default item.
pub fn insert_edit(document: &DslValue, spec: &InsertSpec, at: [f64; 3]) -> Result<NormEdit, String> {
    let items: Vec<DslValue> = match get_value_at_path(document, &spec.collection) {
        Ok(DslValue::Array(items)) => items.clone(),
        Ok(DslValue::Object(entries)) => entries.iter().map(|(_, value)| value.clone()).collect(),
        _ => return Err(format!("the document has no collection '{}'", spec.collection)),
    };
    let index = items.len();
    let Some(template) = items.last() else { return Ok(NormEdit::InsertItem { path: spec.collection.clone(), index, value: None }) };
    let mut wrapper = DslValue::object(vec![("item".to_string(), template.clone())]);
    if template.get("id").is_some() {
        set_value_at_path(&mut wrapper, "item.id", DslValue::String(fresh_id(spec, &known_ids(&items))))?;
    }
    for (key, value) in ["x", "y", "z"].into_iter().zip(at) {
        if template.get(key).is_some() {
            set_value_at_path(&mut wrapper, &format!("item.{key}"), number(value))?;
        }
    }
    let value = wrapper.get("item").cloned().ok_or("the placed item vanished")?;
    Ok(NormEdit::InsertItem { path: spec.collection.clone(), index, value: Some(value) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
