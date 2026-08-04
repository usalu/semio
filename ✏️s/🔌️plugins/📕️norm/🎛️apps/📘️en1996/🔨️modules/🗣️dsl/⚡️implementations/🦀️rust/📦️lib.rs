//! 📜️ EN 1996 app — textual document grammar surface + laws (constitutional: dsl).

use en1996::Document;

/// 🗄️ The load-bearing-wall example fixture, handcrafted in `en1996`'s DSL (`store::DocumentDsl`):
/// an EN-annex masonry class 2 wall check under a transient design situation, distinct from
/// `Document::default()`'s DE-annex/persistent values so the grammar's non-default branches
/// (annex, masonry class, design situation, exposure, mortar) are exercised too.
pub const EN1996_LOADBEARING_WALL_EXAMPLE_TEXT: &str = include_str!("../../../../⚡️implementations/🦀️rust/📚️examples/📕️loadbearing-wall.en1996");

/// 📖️ Parses `.en1996` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1996` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use en1996::part_2;
    use norm_core::{AnnexChoice, DesignSituation};

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&Document::default());
    }

    #[test]
    fn loadbearing_wall_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1996_LOADBEARING_WALL_EXAMPLE_TEXT).expect("parse loadbearing wall example");
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.masonry_class, en1996::MasonryClass::Class2);
        assert_eq!(document.design_situation, DesignSituation::Transient);
        assert_eq!(document.exposure, part_2::ExposureClass::Mx3);
        assert_eq!(document.mortar, part_2::MortarClass::M10);
        assert_eq!(document.storeys, 4);
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
