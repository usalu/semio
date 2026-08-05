//! ⚡️ EN 1996 artifact — the operation type + its laws.
//!
//! 🧩️ EN family artifacts carry no bespoke operation enum: the sole mutation is a whole-document
//! replace, already generically implemented as `crate::core::SetDocumentOperation<D>` (its
//! `OpText`/`OpBinary` impls are blanket ones bounded on `D: DocumentDsl`/`DocumentPack`, satisfied
//! for free by this artifact's `#[derive(dsl::DslDocument)]`). The `NormFamily` binding that ties
//! `Document` to `evaluate` lives in `⚙️engine`, next to the compute it names.

use crate::artifacts::en1996::Document;
use crate::core::SetDocumentOperation;

//#region 🔖️Types
pub type Operation = SetDocumentOperation<Document>;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_document_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::default() });
    }
}
//#endregion 🧪️Tests
