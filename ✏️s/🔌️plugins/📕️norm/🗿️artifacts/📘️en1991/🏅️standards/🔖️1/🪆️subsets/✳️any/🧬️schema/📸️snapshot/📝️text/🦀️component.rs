//! 📜️ EN 1991 actions on structures — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1991::En1991Snapshot;

/// 🏬️ The retail-hydrocarbon-fire example fixture, handcrafted in `en1991`'s DSL
/// (`store::DocumentDsl`): a retail unit (imposed category D) evaluated under the EN annex with a
/// hydrocarbon fire curve and a full set of the other action sub-scenarios (snow, wind, thermal,
/// construction, accidental impact, bridge, crane, silo) at plausible non-zero values — distinct
/// from `En1991Snapshot::default()`'s category-B/DE-annex/standard-fire-curve values so the grammar's
/// non-default branches (category, annex, fire curve) are exercised too.
pub const EN1991_RETAIL_HYDROCARBON_FIRE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio");

/// 📖️ Parses `.en1991` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<En1991Snapshot, store::TextError> {
    <En1991Snapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1991` DSL text.
pub fn print_dsl(document: &En1991Snapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1991::part_1_2::FireCurve;
    use crate::document::{AnnexChoice, ImposedCategory};

    #[test]
    fn document_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&En1991Snapshot::default());
    }

    #[test]
    fn dsl_round_trip_agrees_with_print_parse_wrappers() {
        let document = En1991Snapshot::default();
        let printed = print_dsl(&document);
        assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
    }

    #[test]
    fn retail_hydrocarbon_fire_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1991_RETAIL_HYDROCARBON_FIRE_EXAMPLE_TEXT).expect("parse retail hydrocarbon fire example");
        assert_eq!(document.category, ImposedCategory::D);
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.fire_curve, FireCurve::Hydrocarbon);
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
