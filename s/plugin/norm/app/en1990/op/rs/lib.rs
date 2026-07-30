//! ⚡ EN 1990 basis of structural design — operation enum + laws (constitutional: op).
//!
//! 🧩 Every norm family document shares the same one-shot whole-document replacement operation
//! (`norm_core::SetDocumentOperation<D>`, with its `Operation`/`OperationDiff`/`OpText`/`OpBinary`
//! impls already generic over any `D: DocumentDsl + DocumentPack` — see `norm_core`'s `🔖OpText`
//! region) so this slot only needs to bind that generic operation to `en1990::Document`.

use en1990::Document;

pub type Operation = norm_core::SetDocumentOperation<Document>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }
}
