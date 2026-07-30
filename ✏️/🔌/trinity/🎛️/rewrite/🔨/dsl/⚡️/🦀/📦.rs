//! 📜 Trinity Rewrite app — textual document grammar surface + laws (constitutional: dsl).

use rewrite::{LayoutPoint, RewriteRuleState};
use store::DocumentDsl;

/// 📖 Parses `.rewrite` DSL text into a `RewriteRuleState`.
pub fn parse_dsl(text: &str) -> Result<RewriteRuleState, store::TextError> {
    <RewriteRuleState as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RewriteRuleState` back to `.rewrite` DSL text.
pub fn print_dsl(document: &RewriteRuleState) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use store::test_support::{assert_dsl_pack_equivalence, assert_dsl_round_trip};
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
    fn dsl_round_trip_rewrite_rule_state() {
        assert_dsl_round_trip(&sample_rule_state());
    }

    //#region 🔖DslErrorTests
    #[test]
    fn rewrite_rule_state_parse_dsl_errors_on_unknown_keyword() {
        // 🔀 The `dsl::` derive engine parses `RewriteRuleState` as a structured `key=value` record
        // (see its `#[derive(dsl::DslDocument)]` in `rewrite`), not a line-by-line bare-keyword
        // dispatch like the OLD hand-rolled grammar — so garbage input now fails with a field-shape
        // mismatch instead of an "unknown dsl line keyword" message. Still asserts the same
        // underlying contract: malformed text is rejected, not silently accepted.
        let err = RewriteRuleState::parse_dsl("bogus line").unwrap_err();
        assert!(err.message.contains("expected"));
    }

    #[test]
    fn rewrite_rule_state_parse_dsl_errors_on_malformed_binding() {
        assert!(RewriteRuleState::parse_dsl("binding onlykey").is_err());
    }

    #[test]
    fn rewrite_rule_state_parse_dsl_errors_on_malformed_layout() {
        assert!(RewriteRuleState::parse_dsl("layout a").is_err());
        assert!(RewriteRuleState::parse_dsl("layout a notanumber 2").is_err());
        assert!(RewriteRuleState::parse_dsl("layout a 1 notanumber").is_err());
    }

    #[test]
    fn rewrite_rule_state_parse_dsl_valid_binding_and_layout_lines() {
        let mut original = RewriteRuleState { before_fixture_json: "{}".into(), lhs_json: "{}".into(), rhs_json: "{}".into(), ..Default::default() };
        original.parameter_bindings.insert("label".to_string(), PropertyValue::String("hi".into()));
        original.rule_layout.insert("a".to_string(), LayoutPoint { x: 1.0, y: 2.0 });
        let state = RewriteRuleState::parse_dsl(&original.print_dsl()).unwrap();
        assert_eq!(state.parameter_bindings.get("label"), Some(&PropertyValue::String("hi".into())));
        assert_eq!(state.rule_layout.get("a"), Some(&LayoutPoint { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn rewrite_rule_state_parse_dsl_errors_on_malformed_quoted_blob() {
        assert!(RewriteRuleState::parse_dsl("before nope").is_err());
        assert!(RewriteRuleState::parse_dsl("before \"abc").is_err());
        assert!(RewriteRuleState::parse_dsl("before \"ok\" trailing").is_err());
        assert!(RewriteRuleState::parse_dsl(r#"before "a\"#).is_err());
    }

    #[test]
    fn quote_blob_round_trips_backslashes_and_quotes() {
        let mut state = sample_rule_state();
        state.before_fixture_json = "a\\b\"c\nd".to_string();
        assert_dsl_round_trip(&state);
        assert_dsl_pack_equivalence(&state);
    }
    //#endregion 🔖DslErrorTests
}
//#endregion 🧪Tests
