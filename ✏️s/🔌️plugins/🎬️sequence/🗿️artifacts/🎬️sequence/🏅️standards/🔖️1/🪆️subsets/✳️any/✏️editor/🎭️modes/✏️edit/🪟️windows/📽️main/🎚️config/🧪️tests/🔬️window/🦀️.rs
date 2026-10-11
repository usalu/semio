use super::*;
use crate::editor::sequence::modes::edit::windows::script::transient::{SequenceScriptWindowTransient, SequenceScriptWindowTransientMutation, SequenceScriptWindowTransientOwner};
use protocol::{Mutation, OpBinary, OpText};

#[semio_framework_async_macros::async_test]
async fn sequence_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).unwrap();
    let base_config: SequenceMainWindowConfig = semio_framework_pack_json::from_json_str(&fixture["baseConfig"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let base_transient: SequenceScriptWindowTransient = semio_framework_pack_json::from_json_str(&fixture["baseTransient"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture["configMutations"].as_array().unwrap() {
        let mutation: SequenceMainWindowConfigMutation = semio_framework_pack_json::from_json_str(&row["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(serde_json::Value::from(&semio_framework_value::ToValue::to_value(&mutation)), row["mutation"], "set-mutation wire stays byte-identical to the committed neutral fixture");
        let after = protocol::apply_diff(mutation.diff(&base_config).diff(), &base_config).unwrap();
        let restored = mutation.inverse(&base_config).expect("valid retained mutation inverse fixture").into_iter().rev().fold(after, |state, inverse| protocol::apply_diff(inverse.diff(&state).diff(), &state).unwrap());
        assert_eq!(restored, base_config);
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base_config).await;
        assert_eq!(SequenceMainWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceMainWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for row in fixture["transientMutations"].as_array().unwrap() {
        let mutation: SequenceScriptWindowTransientMutation = semio_framework_pack_json::from_json_str(&row["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let after = protocol::apply_diff(mutation.diff(&base_transient).diff(), &base_transient).unwrap();
        let restored = mutation.inverse(&base_transient).expect("valid retained mutation inverse fixture").into_iter().rev().fold(after, |state, inverse| protocol::apply_diff(inverse.diff(&state).diff(), &state).unwrap());
        assert_eq!(restored, base_transient);
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base_transient).await;
        assert_eq!(SequenceScriptWindowTransientMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceScriptWindowTransientMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
}
