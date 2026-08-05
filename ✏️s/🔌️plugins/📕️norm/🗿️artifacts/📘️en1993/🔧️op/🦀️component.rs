//! ⚡️ EN 1993 design of steel structures — operation enum + laws (constitutional: op).
//!
//! 🧩️ Every norm family document shares the same one-shot whole-document replacement operation
//! (`crate::core::SetDocumentOperation<D>`) so this slot only needs to bind that generic operation to
//! `crate::artifacts::en1993::Document`.

use crate::artifacts::en1993::Document;

pub type Operation = crate::core::SetDocumentOperation<Document>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }
}
