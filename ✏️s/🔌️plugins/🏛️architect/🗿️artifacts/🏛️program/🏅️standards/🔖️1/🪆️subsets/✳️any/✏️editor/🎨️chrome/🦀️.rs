//! 🎨️ Architect play app — the presentation factories every window and panel builds its `UiNode`
//! tree from: tree sections/items, inspector fields, and the empty component-scene shell.
//!
//! These live at app level (not in the artifact engine) because they produce framework UI types and
//! app-addressed `ActionDescriptor`s — an artifact must never depend on an app.

use crate::registers::AdjacencyKind;
use crate::{EntityId, ProgramSnapshot};
use dsl::DslValue as Value;

//#region 🔖️Labels
pub fn element_label(program: &ProgramSnapshot, id: &EntityId) -> String {
    program.elements.iter().find(|element| &element.header.id == id).map_or_else(|| id.to_string(), |element| element.header.name.clone())
}

pub fn adjacency_kind_label(kind: &AdjacencyKind) -> &'static str {
    match kind {
        AdjacencyKind::Required => "Required",
        AdjacencyKind::Preferred => "Preferred",
        AdjacencyKind::Optional => "Optional",
        AdjacencyKind::Prohibited => "Prohibited",
    }
}

pub fn entity_to_json<T: dsl::ToValue>(entity: &T) -> Value {
    dsl::ToValue::to_value(entity)
}

pub fn entity_id_from_json(value: &Value) -> Option<String> {
    value.get("id").and_then(|id| id.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("id")).and_then(|id| id.as_str()).map(str::to_string))
}

pub fn entity_name_from_json(value: &Value) -> String {
    value.get("name").and_then(|name| name.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("name")).and_then(|name| name.as_str()).map(str::to_string)).unwrap_or_else(|| "Untitled".into())
}
//#endregion 🔖️Labels


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
