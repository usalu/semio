
use super::*;
use dsl::os_pack as pack;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
#[test]
fn forms_configuration_contract_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔁️mutation-contracts.json")).unwrap();
    let base: FormsConfig = pack::from_json_str(&vectors["base"].to_string()).unwrap();
    assert_eq!(<FormsConfigMutation as Mutation<FormsConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().unwrap().len());
    for vector in vectors["cases"].as_array().unwrap() {
        let mutation: FormsConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).unwrap(), vector["mutation"]);
        assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
        assert_eq!(FormsConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(FormsConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.messages().len(), vector["errors"].as_u64().unwrap() as usize);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).unwrap(), vector["expected"]);
        let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base);
    }
}
