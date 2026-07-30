//! ⚡ DIN V 18599 app — operation type + laws (constitutional: op).
//!
//! 📌 DIN V 18599 has no bespoke operation enum: every session mutation is a whole-document
//! replace, so `Operation` is a re-export of `norm_core`'s generic `SetDocumentOperation<Document>`,
//! which already carries its own `Operation`/`OpText`/`OpBinary` impls — nothing to implement here.

use din18599::Document;

pub type Operation = norm_core::SetDocumentOperation<Document>;

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }
}
//#endregion 🧪Tests
