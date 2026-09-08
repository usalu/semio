//! 🎨️ Architect play app — the presentation factories every window and panel builds its `UiNode`
//! tree from: tree sections/items, inspector fields, and the empty component-scene shell.
//!
//! These live at app level (not in the artifact engine) because they produce framework UI types and
//! app-addressed `ActionDescriptor`s — an artifact must never depend on an app.

use crate::artifacts::program::registers::AdjacencyKind;
use crate::artifacts::program::{EntityId, ProgramSnapshot};
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
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[semio_framework_async_macros::async_test]
    async fn element_label_falls_back_to_the_raw_id() {
        let program = sample_plugin();
        assert_eq!(element_label(&program, &EntityId("nope".into())), "nope");
        assert_eq!(element_label(&program, &program.elements[0].header.id), program.elements[0].header.name);
    }

    #[semio_framework_async_macros::async_test]
    async fn entity_json_readers_accept_both_flat_and_header_shapes() {
        assert_eq!(entity_id_from_json(&Value::object([("id".to_string(), Value::String("a".to_string()))])).as_deref(), Some("a"));
        assert_eq!(entity_id_from_json(&Value::object([("header".to_string(), Value::object([("id".to_string(), Value::String("b".to_string()))]))])).as_deref(), Some("b"));
        assert_eq!(entity_name_from_json(&Value::object([("header".to_string(), Value::object([("name".to_string(), Value::String("N".to_string()))]))])), "N");
        assert_eq!(entity_name_from_json(&Value::object(Vec::<(String, Value)>::new())), "Untitled");
    }

    #[semio_framework_async_macros::async_test]
    async fn every_adjacency_kind_has_a_label() {
        for kind in [AdjacencyKind::Required, AdjacencyKind::Preferred, AdjacencyKind::Optional, AdjacencyKind::Prohibited] {
            assert!(!adjacency_kind_label(&kind).is_empty());
        }
    }
}
//#endregion 🧪️Tests
