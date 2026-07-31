//! 📜️ Trinity Jack app — textual document grammar surface + laws (constitutional: dsl).

use trinity_ram::GraphFixture;

/// 📄️ The Nakagin Capsule Tower example fixture, handcrafted in the `.trinity` DSL.
pub const NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🔱️trinity/📚️example/🔱️nakagin-capsule-tower.trinity");

/// 📖️ Parses `.trinity` DSL text into a `GraphFixture`.
pub fn parse_dsl(text: &str) -> Result<GraphFixture, store::TextError> {
    <GraphFixture as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `GraphFixture` back to `.trinity` DSL text.
pub fn print_dsl(document: &GraphFixture) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nakagin_example_dsl_round_trips() {
        let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn empty_document_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&trinity_jack_engine::empty_jack_document());
    }
}
//#endregion 🧪️Tests
