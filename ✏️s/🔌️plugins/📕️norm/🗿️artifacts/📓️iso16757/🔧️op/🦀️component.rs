//! ⚡️ ISO 16757 artifact — the operation alias, its store aliases, and its laws.
//!
//! 🧬️ `SetDocumentMutation<Document>` (whole-document replace) already implements both
//! `store::Mutation<Document>` and, now that `Document` derives `dsl::DslDocument` (i.e.
//! `store::DocumentDsl`), `store::OpText` too — see `crate::core`'s generic `impl<D: DocumentDsl + ...>
//! OpText for SetDocumentMutation<D>`. A coarse, whole-value-replace operation is the legitimate,
//! sufficient choice: this reference/lookup-table document has no interactive editor driving
//! fine-grained field-level edits, so reusing the generic pair (rather than hand-deriving a redundant
//! one-variant `#[derive(dsl::DslEnum)]` enum that would duplicate exactly this shape) keeps every norm
//! artifact's operation layer DRY. The `NormFamily` binding lives in `⚙️engine`, next to `evaluate`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::iso16757::Document;

pub use crate::artifacts::iso16757::mutations::Iso16757Mutation;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_operation_op_text_round_trips_for_iso16757() {
        store::test_support::assert_op_line_round_trip(&Iso16757Mutation::SetDocument { document: Document::reference_fixture() });
    }
}
//#endregion 🧪️Tests
