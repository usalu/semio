//! 🔬️ The combination-terms text syntax: every accepted form reads to the same pairs and unreadable or empty text is refused.

use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::combination_terms::{parse_combination_terms, CombinationTermsError};

#[test]
fn combination_terms_read_every_accepted_syntax_and_refuse_the_rest() {
    let expected = vec![("dead".to_owned(), 1.35), ("live".to_owned(), 1.5)];
    for text in ["dead:1.35, live:1.5", "dead=1.35; live=1.5", "1.35 dead + 1.5 live", "1.35*dead\n1.5*live", "[[\"dead\", 1.35], [\"live\", 1.5]]", "[{\"caseId\":\"dead\",\"factor\":1.35},{\"caseId\":\"live\",\"factor\":1.5}]"] {
        assert_eq!(parse_combination_terms(text), Ok(expected.clone()), "{text}");
    }
    assert_eq!(parse_combination_terms("dead live wind"), Err(CombinationTermsError::Unreadable));
    assert_eq!(parse_combination_terms("[1, 2"), Err(CombinationTermsError::Unreadable));
    assert_eq!(parse_combination_terms("  ,; "), Err(CombinationTermsError::Empty));
    assert_eq!(parse_combination_terms("[]"), Err(CombinationTermsError::Empty));
}
