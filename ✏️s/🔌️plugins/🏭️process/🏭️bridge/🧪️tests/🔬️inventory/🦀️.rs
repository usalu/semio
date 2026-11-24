//! 🧪️ Mutation bridge laws: the inventory answers the production dispatch descriptors, checked against an independent `serde_json` oracle and the language-agnostic fixture.
//!
//! @see 🧪️tests/🧫️fixtures/inventory.json — the shared fixture; 🧪️tests/🧫️fixtures/inventory.schema.json — its schema.

use super::{answer, protocol::Mutation};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/inventory.json")).unwrap()
}

fn request(coordinate: &Value) -> Vec<String> {
    coordinate["request"].as_array().unwrap().iter().map(|part| part.as_str().unwrap().to_string()).collect()
}

fn rows(inventory: &Value) -> Vec<(String, String, Vec<String>)> {
    inventory["mutations"].as_array().unwrap().iter().map(|row| {
        (
            row["id"].as_str().unwrap().to_string(),
            row["variant"].as_str().unwrap().to_string(),
            row["outcomes"].as_array().unwrap().iter().map(|o| o.as_str().unwrap().to_string()).collect(),
        )
    }).collect()
}

#[test]
fn the_document_inventory_matches_the_shared_fixture() {
    let fixture = fixture();
    let document = &fixture["document"];
    let inventory: Value = serde_json::from_str(&answer(&request(document)).unwrap()).unwrap();
    assert_eq!(inventory["schema"], fixture["schema"]);
    assert_eq!(inventory["bridgeVersion"], fixture["bridgeVersion"]);
    assert_eq!(inventory["producedBy"], fixture["producedBy"]);
    assert_eq!(inventory["artifact"], document["request"][1]);
    assert_eq!(inventory["standard"], document["request"][2]);
    assert_eq!(inventory["subset"], document["request"][3]);
    assert!(inventory.get("surface").is_none());
    let expected: Vec<&str> = document["ids"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
    let actual = rows(&inventory);
    assert_eq!(actual.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(), expected);
}

#[test]
fn every_row_is_a_production_descriptor_and_ids_are_unique() {
    let fixture = fixture();
    let inventory: Value = serde_json::from_str(&answer(&request(&fixture["document"])).unwrap()).unwrap();
    let production = <semio_s_artifact_process_process3d_mutations::Process3dMutation as Mutation<semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot>>::DESCRIPTORS;
    let mut seen = std::collections::BTreeSet::new();
    for (id, variant, outcomes) in rows(&inventory) {
        assert!(seen.insert(id.clone()), "duplicate id {id}");
        let descriptor = production.iter().find(|descriptor| descriptor.semantic_kind == id).unwrap_or_else(|| panic!("{id} is not dispatched in production"));
        assert_eq!(descriptor.aggregate_variant, variant);
        assert_eq!(descriptor.outcome_classes.iter().map(|class| class.as_str().to_string()).collect::<Vec<_>>(), outcomes);
        assert!(!outcomes.is_empty());
    }
    assert!(!seen.is_empty());
}

#[test]
fn unanswered_coordinates_are_refused_with_exit_code_two() {
    let fixture = fixture();
    for refusal in fixture["refusals"].as_array().unwrap() {
        let args: Vec<String> = refusal.as_array().unwrap().iter().map(|part| part.as_str().unwrap().to_string()).collect();
        assert_eq!(answer(&args).unwrap_err().0, 2, "{args:?}");
    }
}
