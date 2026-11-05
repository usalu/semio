//! 🌲️ Document entity rows and their canonical interaction addresses.

use semio_framework_value::DslValue;
use semio_s_plugin_norm_app_surface_foundation::list_element_path;

/// 🌲️ One entity of a collection.
#[derive(Clone, Debug)]
pub struct EntityRow<'a> {
    pub path: String,
    pub id: String,
    pub title: String,
    pub index: usize,
    pub value: &'a DslValue,
}
fn string_field<'a>(value: &'a DslValue, key: &str) -> Option<&'a str> {
    match value.get(key) {
        Some(DslValue::String(raw)) if !raw.is_empty() => Some(raw.as_str()),
        _ => None,
    }
}

/// 🆔️ The entity's `id`, empty when it has none.
pub fn entity_id(value: &DslValue) -> String {
    string_field(value, "id").unwrap_or_default().to_string()
}

/// 🏷️ `id — name` when both exist, else whichever exists, else the position.
pub fn entity_title(value: &DslValue, index: usize) -> String {
    let id = string_field(value, "id");
    let name = string_field(value, "name").or_else(|| string_field(value, "label")).or_else(|| string_field(value, "title"));
    match (id, name) {
        (Some(id), Some(name)) if id != name => format!("{id} — {name}"),
        (Some(id), _) => id.to_string(),
        (None, Some(name)) => name.to_string(),
        (None, None) => format!("#{}", index + 1),
    }
}

/// 🔎️ Case-insensitive match of every blank-separated filter word against the haystack; an empty filter admits all.
pub fn filter_matches(filter: &str, haystack: &str) -> bool {
    let haystack = haystack.to_lowercase();
    filter.split_whitespace().all(|word| haystack.contains(&word.to_lowercase()))
}

/// 🌲️ The rows of a collection that pass the filter, in document order.
pub fn collection_rows<'a>(path: &str, items: &'a [DslValue], filter: &str) -> Vec<EntityRow<'a>> {
    items
        .iter()
        .enumerate()
        .map(|(index, value)| EntityRow { path: list_element_path(path, value, index), id: entity_id(value), title: entity_title(value, index), index, value })
        .filter(|row| filter_matches(filter, &format!("{} {}", row.title, row.path)))
        .collect()
}

/// 🌲️ Whether a collection is entity-shaped: empty, or only objects.
pub fn is_entity_collection(items: &[DslValue]) -> bool {
    items.iter().all(|item| matches!(item, DslValue::Object(_)))
}


/// 🎯️ The interaction address of an entity row (`collection/id`, unique per collection); the selection mirror resolves it to the entity id, the viewport highlights the same entity.
pub fn row_address(row: &EntityRow<'_>) -> Option<String> {
    let collection = row.path.split('[').next().unwrap_or_default();
    (!row.id.is_empty() && !collection.is_empty()).then(|| crate::protocol::wire::encode_scoped_identity(&[collection, row.id.as_str()]).ok()).flatten()
}

