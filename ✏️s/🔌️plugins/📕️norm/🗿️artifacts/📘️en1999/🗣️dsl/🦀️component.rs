//! 📜️ EN 1999 app — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1999::En1999Snapshot;

/// 🗄️ The aluminium-roof-purlin example fixture, handcrafted in `en1999`'s DSL (`store::En1999SnapshotDsl`):
/// a welded AW-6082-T6 aluminium roof purlin under the EN annex, exercising the higher-strength alloy's
/// cross-section, buckling, bending, fatigue, welded-joint, cold-formed-sheeting, and shell-buckling
/// checks together, distinct from `En1999Snapshot::default()`'s AW-6060-T6/DE-annex values so the grammar's
/// non-default branches (alloy, annex) are exercised too.
pub const EN1999_ALUMINIUM_ROOF_PURLIN_EXAMPLE_TEXT: &str = include_str!("../📚️examples/📕️aluminium-roof-purlin/🖼️assets/🗣️aluminium-roof-purlin.dsl.semio");

/// 📖️ Parses `.en1999` DSL text into a `En1999Snapshot`.
pub fn parse_dsl(text: &str) -> Result<En1999Snapshot, store::TextError> {
    <En1999Snapshot as store::En1999SnapshotDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `En1999Snapshot` back to `.en1999` DSL text.
pub fn print_dsl(document: &En1999Snapshot) -> String {
    store::En1999SnapshotDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::AnnexChoice;

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&En1999Snapshot::default());
    }

    #[test]
    fn aluminium_roof_purlin_example_fixture_parses_and_round_trips() {
        let document = parse_dsl(EN1999_ALUMINIUM_ROOF_PURLIN_EXAMPLE_TEXT).expect("parse aluminium roof purlin example");
        assert_eq!(document.alloy, "aw6082t6");
        assert_eq!(document.annex, AnnexChoice::En);
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
