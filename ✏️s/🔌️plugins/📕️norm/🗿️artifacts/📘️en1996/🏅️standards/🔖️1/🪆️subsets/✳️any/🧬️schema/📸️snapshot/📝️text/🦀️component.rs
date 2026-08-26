//! 📜️ EN 1996 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::en1996::En1996Snapshot;

/// 🗄️ The load-bearing-wall example fixture, handcrafted in `en1996`'s DSL (`store::ArtifactDsl`):
/// an EN-annex masonry class 2 wall check under a transient design situation, distinct from
/// `En1996Snapshot::default()`'s DE-annex/persistent values so the grammar's non-default branches
/// (annex, masonry class, design situation, exposure, mortar) are exercised too.
pub const EN1996_LOADBEARING_WALL_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/📕️loadbearing-wall/🖼️assets/🗣️loadbearing-wall.dsl.semio");

/// 📖️ Parses `.en1996` DSL text into a `En1996Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1996Snapshot, store::TextError> {
    <En1996Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1996Snapshot` back to `.en1996` DSL text.
pub fn print_dsl(document: &En1996Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1996::part_2;
    use crate::document::{AnnexChoice, DesignSituation};

    #[semio_framework_async_macros::async_test]
    fn document_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&En1996Snapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    fn loadbearing_wall_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1996_LOADBEARING_WALL_EXAMPLE_TEXT).expect("parse loadbearing wall example");
        assert_eq!(document.annex, AnnexChoice::En);
        assert_eq!(document.masonry_class, crate::artifacts::en1996::MasonryClass::Class2);
        assert_eq!(document.design_situation, DesignSituation::Transient);
        assert_eq!(document.exposure, part_2::ExposureClass::Mx3);
        assert_eq!(document.mortar, part_2::MortarClass::M10);
        assert_eq!(document.storeys, 4);
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
