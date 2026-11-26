use std::collections::BTreeSet;
use semio_framework_schema_registry::{FacetLeaves, SchemaFormat, resolved_leaf_body};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::bim_model_artifact_schema_descriptor;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

fn schema(facet: &str, leaves: &FacetLeaves) -> serde_json::Value {
    let document = resolved_leaf_body("s.bim.model", facet, SchemaFormat::JsonSchema, leaves);
    serde_json::from_str(document).unwrap_or_else(|error| panic!("{facet} schema is not JSON: {error}"))
}

#[semio_framework_async_macros::async_test]
async fn every_json_schema_leaf_parses_and_carries_its_identity() {
    let descriptor = bim_model_artifact_schema_descriptor();
    assert_eq!(descriptor.id, "s.bim.model");
    for (facet, leaves) in [("artifact", &descriptor.artifact), ("snapshot", &descriptor.snapshot), ("diff", &descriptor.diff), ("mutations", &descriptor.mutations)] {
        let identity = format!("https://json.schemas.assets.semio-tech.com/s/bim/model/{facet}.json");
        assert_eq!(schema(facet, leaves)["$id"], identity);
        for format in [SchemaFormat::Typescript, SchemaFormat::Graphql] {
            assert!(!resolved_leaf_body(descriptor.id, facet, format, leaves).is_empty(), "{facet} {format:?} schema");
        }
        assert!(resolved_leaf_body(descriptor.id, facet, SchemaFormat::Protobuf, leaves).starts_with("syntax = \"proto3\";"), "{facet} protobuf schema");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_snapshot_schema_declares_every_collection_with_the_artifact_state_class() {
    let descriptor = bim_model_artifact_schema_descriptor();
    let document = schema("snapshot", &descriptor.snapshot);
    let properties = document["properties"].as_object().expect("properties");
    let expected: BTreeSet<String> = ModelSnapshot::__dsl_spec().fields.iter().map(|field| field.key.replace('-', "_")).collect();
    assert_eq!(properties.keys().cloned().collect::<BTreeSet<_>>(), expected);
    assert!(properties.values().all(|property| property["x-semio-state"] == "artifact"));
}

#[semio_framework_async_macros::async_test]
async fn the_mutation_schema_lists_one_payload_per_kind() {
    let descriptor = bim_model_artifact_schema_descriptor();
    let document = schema("mutations", &descriptor.mutations);
    let references = document["oneOf"].as_array().expect("mutation payloads");
    let actual: BTreeSet<_> = references.iter().map(|reference| reference["$ref"].as_str().expect("payload reference")).collect();
    let expected: BTreeSet<_> = crate::standards::v1::subsets::any::schema::mutations::KINDS.iter()
        .map(|kind| format!("https://json.schemas.assets.semio-tech.com/s/bim/model/mutation/{kind}/schema.json")).collect();
    assert_eq!(references.len(), expected.len());
    assert_eq!(actual, expected.iter().map(String::as_str).collect());
}
