
use super::*;
#[test]
fn storey_payload_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🏢️storeys.json")).unwrap();
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
