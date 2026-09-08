
use super::*;

#[test]
fn component_grammar_semio_is_grammar_dialect() {
    let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
    assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
    assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
    let _ = COMPONENT_GRAMMAR_PATH;
}
