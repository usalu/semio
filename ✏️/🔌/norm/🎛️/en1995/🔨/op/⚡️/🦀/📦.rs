//! ⚡ EN 1995 app — operation type + family + laws (constitutional: op).
//!
//! 🧩 EN family apps carry no bespoke operation enum: the sole mutation is a whole-document replace,
//! already generically implemented as `norm_core::SetDocumentOperation<D>` (its `OpText`/`OpBinary`
//! impls are blanket ones bounded on `D: DocumentDsl`/`DocumentPack`, satisfied for free by `en1995`'s
//! `#[derive(dsl::DslDocument)]`). This crate's own content is therefore the `NormFamily` binding that
//! ties `Document` (rs) to `evaluate` (engine) for the headless `NormHost` session.

use en1995::Document;
use norm_core::{CheckReport, NormFamily, NormFamilyId, NormHost, SetDocumentOperation};

//#region 🔖Types
pub type Operation = SetDocumentOperation<Document>;

pub struct En1995Family;

impl NormFamily for En1995Family {
    type Document = Document;
    type Operation = Operation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1995
    }

    fn evaluate(document: &Document) -> CheckReport {
        en1995_engine::evaluate(document)
    }
}

pub type Host = NormHost<En1995Family>;
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
