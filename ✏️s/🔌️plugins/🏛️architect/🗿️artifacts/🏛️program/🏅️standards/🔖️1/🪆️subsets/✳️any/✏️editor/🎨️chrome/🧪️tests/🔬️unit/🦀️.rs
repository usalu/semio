
use super::*;
use crate::sample_plugin;

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
