//! 📜 EN 1992 design of concrete structures — textual document grammar surface + laws (constitutional: dsl).
//!
//! 📄 No handcrafted `.en1992` DSL fixture exists for this app — the original monolith's own DSL law
//! test exercised only `Document::default()`, so that is the representative document here too.

use en1992::Document;

/// 📖 Parses `.en1992` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1992` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&Document::default());
    }

    #[test]
    fn dsl_round_trip_agrees_with_print_parse_wrappers() {
        let document = Document::default();
        let printed = print_dsl(&document);
        assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
    }

    #[test]
    fn document_dsl_parse_error_reports_the_real_line_of_the_bad_field() {
        // The engine's per-token spans are a concrete improvement over the old `dsl_kv` printer,
        // whose errors always reported `TextSpan::at(1, 1)` regardless of which line actually
        // failed. `fire-rating` (kebab-cased from `fire_rating`) is the 16th `key value` line in
        // `print_dsl`'s fixed field order.
        let printed = print_dsl(&Document::default());
        let bad = printed.replacen("fire-rating=r60", "fire-rating=not-a-rating", 1);
        assert_ne!(bad, printed, "fire_rating's printed line must match the literal replaced above");
        let bad_line = bad.lines().position(|l| l.contains("not-a-rating")).expect("bad line present") as u32 + 1;
        let error = parse_dsl(&bad).expect_err("an unknown fire_rating tag must fail to parse");
        assert_eq!(error.span.line, bad_line, "error span must point at the actual malformed line, not (1, 1)");
    }
}
