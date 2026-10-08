use super::*;
use protocol::{Mutation, OpBinary, OpText};

#[test]
fn language_neutral_mutations_match_json_oracle_and_restore_base() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️mutations.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let base: EquationGraphWindowConfig = semio_framework_pack_json::from_json_str(&vector["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutation: EquationGraphWindowConfigMutation = semio_framework_pack_json::from_json_str(&vector["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let oracle: EquationGraphWindowConfigMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
        assert_eq!(mutation, oracle);
        assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
        let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).unwrap();
        assert_eq!(serde_json::to_value(&next).unwrap(), vector["after"]);
        assert_eq!(EquationGraphWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        assert_eq!(EquationGraphWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        let mut restored = next;
        for inverse in mutation.inverse(&base).expect("valid retained mutation inverse fixture") {
            restored = protocol::apply_diff(inverse.diff(&restored).diff(), &restored).unwrap();
        }
        assert_eq!(restored, base);
    }
}
