use super::*;
#[test]
fn storey_payload_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏢️storeys.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let oracle: CreateBuildingStorey = serde_json::from_value(vector["payload"].clone()).unwrap();
        let actual: CreateBuildingStorey = protocol::os_pack::json::from_json_str(&vector["payload"].to_string()).unwrap();
        assert_eq!(actual, oracle);
        assert_eq!(actual.storey_label(), vector["label"].as_str().unwrap());
    }
    let descriptor = <CreateBuildingStorey as protocol::MutationLeaf>::DESCRIPTOR;
    assert_eq!(descriptor.semantic_kind, "create-building-storey");
    assert_eq!(descriptor.composition, protocol::MutationComposition::Composite);
}

/// 🧾️ The committed wire witness is the canonical Rust wire of the ground-storey vector: a contributed composite is
/// dispatched with its own `ToValue` payload, and the independent `serde_json` oracle decodes the same value.
#[test]
fn committed_wire_witness_is_the_canonical_rust_wire() {
    let witness = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢️create-building-storey/🧾️wire-witness/🦠️mutation/🔣️.json");
    let witnessed: CreateBuildingStorey = protocol::os_store::test_support::assert_wire_witness(witness);
    assert_eq!(witnessed, serde_json::from_str::<CreateBuildingStorey>(witness).unwrap());
    assert_eq!(witnessed.storey_label(), "Level 0: Ground");
}
