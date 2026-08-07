//! ⚡️ EN 1992 design of concrete structures — operation enum + laws (constitutional: op).
//!
//! 🧩️ Every norm family document shares the same one-shot whole-document replacement operation
//! (`crate::core::SetDocumentOperation<D>`) so this slot only needs to bind that generic operation to
//! `crate::artifacts::en1992::Document`.

use crate::artifacts::en1992::Document;

pub type Operation = crate::core::SetDocumentOperation<Document>;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }
}
