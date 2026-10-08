use super::*;
use dsl::os_pack as pack;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn shooting_configuration_contract_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
    let base: ShootingConfig = semio_framework_pack_json::from_json_str(&vectors["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned base decoder");
    assert_eq!(<ShootingConfigMutation as Mutation<ShootingConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
    for vector in vectors["cases"].as_array().expect("cases") {
        let mutation: ShootingConfigMutation = semio_framework_pack_json::from_json_str(&vector["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned operation decoder");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
        assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
        assert_eq!(ShootingConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
        assert_eq!(ShootingConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
        let outcome = mutation.diff(&base);
        assert!(outcome.messages().is_empty());
        let next = protocol::apply_diff(outcome.diff(), &base).expect("apply diff");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
        let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().fold(next, |state, inverse| protocol::apply_diff(inverse.diff(&state).diff(), &state).expect("apply inverse"));
        assert_eq!(restored, base);
    }
}
