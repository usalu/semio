//! ⚡ EN 1991 actions on structures — operation enum + laws (constitutional: op).
//!
//! 🧩 Every norm family document shares the same one-shot whole-document replacement operation
//! (`norm_core::SetDocumentOperation<D>`) so this slot only needs to bind that generic operation to
//! `en1991::Document`.

use en1991::Document;

pub type Operation = norm_core::SetDocumentOperation<Document>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }
}
