//! ⚡️ EN 1992 design of concrete structures — OpText/OpBinary via shared `SetDocumentMutation`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::en1992::schema::mutations::En1992Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1992::En1992Snapshot;

    #[test]
    fn set_document_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&En1992Mutation::SetSnapshot { snapshot: En1992Snapshot::default() });
    }
}
//#endregion 🧪️Tests
