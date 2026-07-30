//! ⚡ EN 1997 app — operation type + family + laws (constitutional: op).
//!
//! 🧩 EN family apps carry no bespoke operation enum: the sole mutation is a whole-document replace,
//! already generically implemented as `norm_core::SetDocumentOperation<D>` (its `OpText`/`OpBinary`
//! impls are blanket ones bounded on `D: DocumentDsl`/`DocumentPack`, satisfied for free by `en1997`'s
//! `#[derive(dsl::DslDocument)]`). This crate's own content is therefore the `NormFamily` binding that
//! ties `Document` (rs) to `evaluate` (engine) for the headless `NormHost` session.

use en1997::Document;
use norm_core::{CheckReport, NormFamily, NormFamilyId, NormHost, SetDocumentOperation};

//#region 🔖Types
pub type Operation = SetDocumentOperation<Document>;

pub struct En1997Family;

impl NormFamily for En1997Family {
    type Document = Document;
    type Operation = Operation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1997
    }

    fn evaluate(document: &Document) -> CheckReport {
        en1997_engine::evaluate(document)
    }
}

pub type Host = NormHost<En1997Family>;
//#endregion 🔖Types

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
