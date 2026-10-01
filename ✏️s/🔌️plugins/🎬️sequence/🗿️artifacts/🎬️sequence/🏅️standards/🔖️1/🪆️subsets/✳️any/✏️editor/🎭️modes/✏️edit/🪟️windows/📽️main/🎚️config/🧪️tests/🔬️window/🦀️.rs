use super::*;
use crate::editor::sequence::modes::edit::windows::script::transient::{SequenceScriptWindowTransient, SequenceScriptWindowTransientMutation, SequenceScriptWindowTransientOwner};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn sequence_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window/🔣️.json")).unwrap();
    let base_config: SequenceMainWindowConfig = dsl::json::from_json_str(&fixture["baseConfig"].to_string()).unwrap();
    let base_transient: SequenceScriptWindowTransient = dsl::json::from_json_str(&fixture["baseTransient"].to_string()).unwrap();
    for row in fixture["configMutations"].as_array().unwrap() {
        let mutation: SequenceMainWindowConfigMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_config).diff().apply(&base_config).unwrap();
        let restored = mutation.inverse(&base_config).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_config);
        assert_eq!(SequenceMainWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceMainWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for row in fixture["transientMutations"].as_array().unwrap() {
        let mutation: SequenceScriptWindowTransientMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
        let after = mutation.diff(&base_transient).diff().apply(&base_transient).unwrap();
        let restored = mutation.inverse(&base_transient).into_iter().fold(after, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, base_transient);
        assert_eq!(SequenceScriptWindowTransientMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(SequenceScriptWindowTransientMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
}
