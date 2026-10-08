use super::*;

fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).unwrap_or_else(|error| panic!("a facet leaf is not JSON: {error}"))
}

#[semio_framework_async_macros::async_test]
async fn every_json_schema_leaf_parses_and_carries_its_identity() {
    let descriptor = bim_model_artifact_schema_descriptor();
    assert_eq!(descriptor.id, "s.bim.model");
    for (family, leaves, id) in [("artifact", &descriptor.artifact, "artifact.json"), ("snapshot", &descriptor.snapshot, "snapshot.json"), ("diff", &descriptor.diff, "diff.json"), ("mutations", &descriptor.mutations, "mutations.json")] {
        let schema = json(leaves.json_schema);
        assert!(schema["$id"].as_str().expect("$id").ends_with(id), "{family} schema identity");
        assert!(!leaves.typescript.is_empty() && !leaves.graphql.is_empty() && leaves.proto.starts_with("syntax = \"proto3\";"), "{family} facets exist");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_snapshot_schema_declares_every_collection_with_the_artifact_state_class() {
    let schema = json(bim_model_artifact_schema_descriptor().snapshot.json_schema);
    let properties = schema["properties"].as_object().expect("properties");
    assert_eq!(properties.len(), 26, "schema, project and 24 collections");
    assert!(properties.values().all(|property| property["x-semio-state"] == "artifact"));
}

#[semio_framework_async_macros::async_test]
async fn the_mutation_schema_lists_one_payload_per_kind() {
    let schema = json(bim_model_artifact_schema_descriptor().mutations.json_schema);
    assert_eq!(schema["oneOf"].as_array().expect("oneOf").len(), crate::standards::v1::subsets::any::schema::mutations::KINDS.len());
}
