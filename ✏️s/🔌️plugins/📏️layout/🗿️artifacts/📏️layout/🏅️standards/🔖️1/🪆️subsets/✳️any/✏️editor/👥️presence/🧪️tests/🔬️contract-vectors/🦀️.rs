use super::*;
use dsl::os_pack as pack;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn layout_presence_contract_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
    let base: LayoutPresence = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
    assert_eq!(<LayoutPresenceMutation as Mutation<LayoutPresence>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
    for vector in vectors["cases"].as_array().expect("cases") {
        let mutation: LayoutPresenceMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
        assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
        assert_eq!(LayoutPresenceMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
        assert_eq!(LayoutPresenceMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
        let outcome = mutation.diff(&base);
        assert!(outcome.messages().is_empty());
        let next = outcome.diff().apply(&base).expect("apply diff");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
        let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
        assert_eq!(restored, base);
    }
}
