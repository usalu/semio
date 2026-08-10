//! 📜️ EN 1992 design of concrete structures — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::en1992::En1992Snapshot;

/// 💧️ The liquid-retaining-fem-anchor example fixture, handcrafted in `en1992`'s DSL
/// (`store::DocumentDsl`): a liquid-retaining structure (EN 1992-3 tightness class TC2) section
/// checked with a FEM-based analysis, an R90 fire rating, and a post-installed anchor in cracked
/// concrete, under the EN annex — distinct from `En1992Snapshot::default()`'s DE-annex/TC1/R60/uncracked
/// values so the grammar's non-default branches (annex, fire rating, tightness class, `use_fem`,
/// `anchor_cracked`) are exercised too.
pub const EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio");

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


/// 📖️ Parses `.en1992` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1992Snapshot, store::TextError> {
    <En1992Snapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1992` DSL text.
pub fn print_dsl(document: &En1992Snapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&En1992Snapshot::default());
    }

    #[test]
    fn dsl_round_trip_agrees_with_print_parse_wrappers() {
        let document = En1992Snapshot::default();
        let printed = print_dsl(&document);
        assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
    }

    #[test]
    fn document_dsl_parse_error_reports_the_real_line_of_the_bad_field() {
        // The engine's per-token spans are a concrete improvement over the old `dsl_kv` printer,
        // whose errors always reported `TextSpan::at(1, 1)` regardless of which line actually
        // failed. `fire-rating` (kebab-cased from `fire_rating`) is the 16th `key value` line in
        // `print_dsl`'s fixed field order.
        let printed = print_dsl(&En1992Snapshot::default());
        let bad = printed.replacen("fire-rating=r60", "fire-rating=not-a-rating", 1);
        assert_ne!(bad, printed, "fire_rating's printed line must match the literal replaced above");
        // Spans are relative to the document body after preamble strip.
        let body = bad.split_once('\n').map(|(_, rest)| rest).unwrap_or(bad.as_str());
        let bad_line = body.lines().position(|l| l.contains("not-a-rating")).expect("bad line present") as u32 + 1;
        let error = parse_dsl(&bad).expect_err("an unknown fire_rating tag must fail to parse");
        assert_eq!(error.span.line, bad_line, "error span must point at the actual malformed line, not (1, 1)");
    }

    #[test]
    fn liquid_retaining_fem_anchor_example_fixture_parses_and_round_trips() {
        use crate::artifacts::en1992::part_1_2::FireRating;
        use crate::artifacts::en1992::part_3::TightnessClass;
        use crate::document::AnnexChoice;
        let document = parse_dsl(EN1992_LIQUID_RETAINING_FEM_ANCHOR_EXAMPLE_TEXT).expect("parse liquid retaining fem anchor example");
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.fire_rating, FireRating::R90);
        assert_eq!(document.tightness_class, TightnessClass::Tc2);
        assert!(document.use_fem);
        assert!(document.anchor_cracked);
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}
