//! 📜️ S Home launcher app — textual document grammar surface + laws (constitutional: dsl).

use home::SHomeDocument;

/// 📖️ Parses `.shome` DSL text into an `SHomeDocument`.
pub fn parse_dsl(text: &str) -> Result<SHomeDocument, store::TextError> {
    <SHomeDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `SHomeDocument` back to `.shome` DSL text.
pub fn print_dsl(document: &SHomeDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_dsl_round_trips_default_and_populated_documents() {
        store::test_support::assert_dsl_round_trip(&SHomeDocument { schema: "s.home".into(), catalog_generation: 0 });
        store::test_support::assert_dsl_round_trip(&SHomeDocument { schema: "s.home".into(), catalog_generation: 42 });
    }
}
//#endregion 🧪️Tests
