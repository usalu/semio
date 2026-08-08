//! ⚡️ EN 1997 artifact — the operation type + its laws.
//!
//! 🧩️ EN family artifacts carry no bespoke operation enum: the sole mutation is a whole-document
//! replace, already generically implemented as `crate::document::SetDocumentMutation<D>` (its
//! `OpText`/`OpBinary` impls are blanket ones bounded on `D: DocumentDsl`/`DocumentPack`, satisfied
//! for free by this artifact's `#[derive(dsl::DslRecord)]`). The `NormFamily` binding that ties
//! `Document` to `evaluate` lives in `⚙️engine`, next to the compute it names.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1997::Document;

pub use crate::artifacts::en1997::mutations::En1997Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&En1997Mutation::SetDocument { document: Document::default() });
    }
}
//#endregion 🧪️Tests
