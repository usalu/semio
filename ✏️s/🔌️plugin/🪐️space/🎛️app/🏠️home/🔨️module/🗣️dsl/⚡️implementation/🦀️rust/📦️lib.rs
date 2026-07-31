//! 📜️ S Home launcher app — textual document grammar surface + laws (constitutional: dsl).

use home::SHomeDocument;

/// 📦️ The `home` app's "default" example, embedded at compile time as handcrafted `.shome` DSL text —
/// exercised by the round-trip test below. Not yet wired into a `.example(...)` manifest registration
/// (the `home` UI manifest has none today).
pub const HOME_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🪐️space/📚️example/🏠️home/🪐️default.shome");

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

    #[test]
    fn home_dsl_round_trips_bundled_default_example() {
        let document = parse_dsl(HOME_EXAMPLE_TEXT).expect("parse default example");
        store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
