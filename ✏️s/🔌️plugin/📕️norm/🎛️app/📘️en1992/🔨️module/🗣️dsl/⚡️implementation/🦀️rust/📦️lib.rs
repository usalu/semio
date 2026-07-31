//! 📜️ EN 1992 design of concrete structures — textual document grammar surface + laws (constitutional: dsl).

use en1992::Document;

/// 💧️ The liquid-retaining-fem-anchor example fixture, handcrafted in `en1992`'s DSL
/// (`store::DocumentDsl`): a liquid-retaining structure (EN 1992-3 tightness class TC2) section
/// checked with a FEM-based analysis, an R90 fire rating, and a post-installed anchor in cracked
/// concrete, under the EN annex — distinct from `Document::default()`'s DE-annex/TC1/R60/uncracked
/// values so the grammar's non-default branches (annex, fire rating, tightness class, `use_fem`,
/// `anchor_cracked`) are exercised too.
pub const EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/📕️norm/📚️example/📘️en1992/📕️liquid-retaining-fem-anchor.en1992");

/// 📖️ Parses `.en1992` DSL text into a `Document`.
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

    #[test]
    fn liquid_retaining_fem_anchor_example_fixture_parses_and_round_trips() {
        use en1992::part_1_2::FireRating;
        use en1992::part_3::TightnessClass;
        use norm_core::AnnexChoice;
        let document = parse_dsl(EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT).expect("parse liquid retaining fem anchor example");
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.fire_rating, FireRating::R90);
        assert_eq!(document.tightness_class, TightnessClass::Tc2);
        assert!(document.use_fem);
        assert!(document.anchor_cracked);
        store::test_support::assert_dsl_round_trip(&document);
    }
}
