use super::*;

#[semio_framework_async_macros::async_test]
async fn component_grammar_semio_is_grammar_dialect() {
    let g = semio_framework_dsl::parse_grammar(MUTATION_GRAMMAR_SEMIO).expect("parse grammar.semio");
    assert_eq!(g.dialect, semio_framework_dsl::SemioDialect::Grammar);
    assert!(!MUTATION_GRAMMAR_SEMIO.is_empty());
    let _ = MUTATION_GRAMMAR_PATH;
}
