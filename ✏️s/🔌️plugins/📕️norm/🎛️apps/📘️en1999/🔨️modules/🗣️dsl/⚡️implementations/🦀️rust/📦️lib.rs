//! 📜️ EN 1999 app — textual document grammar surface + laws (constitutional: dsl).

use en1999::Document;

/// 🗄️ The aluminium-roof-purlin example fixture, handcrafted in `en1999`'s DSL (`store::DocumentDsl`):
/// a welded AW-6082-T6 aluminium roof purlin under the EN annex, exercising the higher-strength alloy's
/// cross-section, buckling, bending, fatigue, welded-joint, cold-formed-sheeting, and shell-buckling
/// checks together, distinct from `Document::default()`'s AW-6060-T6/DE-annex values so the grammar's
/// non-default branches (alloy, annex) are exercised too.
pub const EN1999_ALUMINIUM_ROOF_PURLIN_EXAMPLE_TEXT: &str = include_str!("../../../../⚡️implementations/🦀️rust/📚️examples/📕️aluminium-roof-purlin.en1999");

/// 📖️ Parses `.en1999` DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Document, store::TextError> {
    <Document as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.en1999` DSL text.
pub fn print_dsl(document: &Document) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use norm_core::AnnexChoice;

    #[test]
    fn document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&Document::default());
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
