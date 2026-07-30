//! ⚡ ISO 16757 app — operation alias + norm family + laws (constitutional: op).
//!
//! 🧬 `SetDocumentOperation<Document>` (whole-document replace) already implements both
//! `store::Operation<Document>` and, now that `Document` derives `dsl::DslDocument` (i.e.
//! `store::DocumentDsl`), `store::OpText` too — see `norm_core`'s generic `impl<D: DocumentDsl + ...>
//! OpText for SetDocumentOperation<D>`. A coarse, whole-value-replace operation is the legitimate,
//! sufficient choice per the migration cheat sheet: this reference/lookup-table document has no
//! existing interactive editor driving fine-grained field-level edits, so reusing this generic
//! pair (rather than hand-deriving a redundant one-variant `#[derive(dsl::DslOps)]` enum that would
//! duplicate exactly this shape) keeps every norm family crate's Operation layer DRY.
//!
//! ⚠️ Circular-dependency check (per the migration recipe): `apply_X_operation` would need to live
//! here (not in `engine`) if it matched on a locally-derived operation ENUM. It does not — `Operation`
//! is a type alias to `norm_core::SetDocumentOperation<Document>`, whose single `apply`-equivalent
//! behavior is already blanket-implemented generically in `norm_core` itself. There is nothing to
//! apply here, so this crate is free to depend on `engine` (to call `evaluate()` from the
//! `NormFamily` impl) without creating a cycle.

use iso16757::Document;
use norm_core::{CheckReport, NormFamily, NormFamilyId, NormHost, SetDocumentOperation};

/// 🧬 See module doc comment.
pub type Operation = SetDocumentOperation<Document>;
pub type Host = NormHost<Iso16757Family>;

/// 📦 VCS envelope/store aliases for the ISO 16757 document, now that `Document`/`Operation` both
/// satisfy `store::DocumentDsl`/`store::OpText`.
pub type Iso16757Envelope = store::DocumentEnvelope<Document, Operation>;
pub type Iso16757Store = store::DocumentStore<Document, Operation>;

pub struct Iso16757Family;

impl NormFamily for Iso16757Family {
    type Document = Document;
    type Operation = Operation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::Iso16757
    }

    fn evaluate(document: &Document) -> CheckReport {
        iso16757_engine::evaluate(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_family_evaluate_matches_host() {
        let doc = Document::default();
        let host = Host::from_document(doc);
        assert!(!host.report().checks.is_empty());
        assert_eq!(Iso16757Family::family_id(), NormFamilyId::Iso16757);
    }

    #[test]
    fn set_document_operation_op_text_round_trips_for_iso16757() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: Document::reference_fixture() });
    }
}
