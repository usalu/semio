//! ⚖️ Trinity Rewrite app — binary command protocol surface + laws (constitutional: protocol).

use rewrite_op::RewriteRuleOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `RewriteRuleOperation` to its binary command form.
pub fn encode_op(operation: &RewriteRuleOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RewriteRuleOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RewriteRuleOperation, protocol::ProtocolError> {
    RewriteRuleOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use rewrite::{LayoutPoint, RewriteRuleState};
    use rewrite_op::{create_rewrite_rule_envelope, dispatch_rewrite_rule_state, RewriteRuleStore};
    use std::collections::BTreeMap;
    use protocol::OpText;
    use store::test_support::{assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};
    use trinity_ram::PropertyValue;

    fn sample_rule_state() -> RewriteRuleState {
        let mut parameter_bindings = BTreeMap::new();
        parameter_bindings.insert("label".to_string(), PropertyValue::String("nakagin-core".into()));
        parameter_bindings.insert("count".to_string(), PropertyValue::Number(3.0));
        let mut rule_layout = BTreeMap::new();
        rule_layout.insert("a".to_string(), LayoutPoint::from((10.5, -20.25)));
        RewriteRuleState {
            before_fixture_json: "{\"schema\":\"trinity.graph\",\"name\":\"x \\\"quoted\\\"\\nline\"}".to_string(),
            lhs_json: r#"{"pattern":{"leftVar":"a","leftKind":"Piece"}}"#.to_string(),
            rhs_json: r#"{"set":[{"var":"a","prop":"label","value":"$label"}]}"#.to_string(),
            parameter_bindings,
            rule_layout,
        }
    }

    #[test]
    fn op_text_round_trip_set_state() {
        assert_op_line_round_trip(&RewriteRuleOperation::SetState { state: sample_rule_state() });
    }

    #[test]
    fn document_text_round_trip_rewrite_rule_store() {
        let mut store = RewriteRuleStore::new(create_rewrite_rule_envelope("test", sample_rule_state()));
        let mut next = sample_rule_state();
        next.lhs_json = "{}".into();
        dispatch_rewrite_rule_state(&mut store, next).unwrap();
        assert_document_text_round_trip(&store);
        assert_document_pack_round_trip(&store);
    }

    #[test]
    fn op_text_parse_op_errors_on_unknown_keyword() {
        let err = RewriteRuleOperation::parse_op("bogus xyz").unwrap_err();
        assert!(err.message.contains("unknown operation line"));
    }
}
//#endregion 🧪️Tests
