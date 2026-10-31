use semio_framework_schema_validator::OwnedJsonSchemaValidator;

#[test]
fn canonical_create_object_schema_preserves_complete_managed_mesh_ownership() {
    let schema = include_str!("../🧬️schema/🔣️.json");
    let documents = [
        include_str!("../../../🔣️.json"),
        include_str!("../../../🕸️mesh/🔣️.json"),
        include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔣️.json"),
        include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json"),
        include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🔣️.json"),
        include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json"),
    ];
    let validator = OwnedJsonSchemaValidator::compile_with_documents(schema, &documents).unwrap();
    let laws: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let committed: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-object/⛵️inserts/🦠️mutation/🔣️.json")).unwrap();
    let payload = committed["CreateObject"].clone();
    assert!(validator.is_valid_json(&payload.to_string()));
    for row in laws["meshStates"].as_array().unwrap() {
        let mut value = payload.clone();
        value["object"]["meshState"] = row["value"].clone();
        assert_eq!(validator.is_valid_json(&value.to_string()), row["valid"].as_bool().unwrap(), "{}", row["name"]);
    }
    for field in laws["requiredObjectFields"].as_array().unwrap() {
        let mut value = payload.clone();
        value["object"].as_object_mut().unwrap().remove(field.as_str().unwrap());
        assert!(!validator.is_valid_json(&value.to_string()), "{}", field);
    }
}
