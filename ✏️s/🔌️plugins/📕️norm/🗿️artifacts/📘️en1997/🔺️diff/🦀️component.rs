//! 🔺️ EN 1997 artifact — the operation diff and its `OperationDiff` law.
//!
//! 📌️ Every norm artifact's sole mutation is a whole-document replace, so its diff is
//! `crate::core::DocumentDiff<Document>` — the one generic diff `crate::core::SetDocumentOperation<D>`
//! names as its `Operation::Diff`, with the `OperationDiff` impl (apply = "take the replacement,
//! otherwise keep the projection"; absorb = "the later replacement wins") living beside it in
//! `🫀️core` because all fifteen artifacts share exactly one copy of it. This node states the concrete
//! binding for this artifact and proves the law against this artifact's own `Document`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1997::Document;

//#region 🔖️Types
/// 🔺️ This artifact's concrete operation diff.
pub type Diff = crate::core::DocumentDiff<Document>;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1997::op::Operation;
    use protocol::{Operation as _, OperationDiff};

    #[test]
    fn set_document_diff_replaces_the_whole_projection() {
        let base = Document::default();
        let operation = Operation::SetDocument { document: Document::default() };
        let diff: Diff = operation.diff(&base);
        assert_eq!(diff.apply(&base), Document::default());
    }

    #[test]
    fn an_empty_diff_keeps_the_projection() {
        let base = Document::default();
        assert_eq!(Diff::default().apply(&base), base);
    }

    #[test]
    fn absorb_keeps_the_later_replacement() {
        let base = Document::default();
        let mut diff = Diff::default();
        diff.absorb(Diff { document: Some(Document::default()) });
        assert_eq!(diff.apply(&base), Document::default());
    }
}
//#endregion 🧪️Tests
