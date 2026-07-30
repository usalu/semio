//! ⚖️ Forms app — binary command protocol surface + laws (constitutional: protocol).

use forms_op::FormOperation;
use protocol::OpBinary;

//#region 🔖Types
pub type FormsEnvelope = playbook::PlaybookEnvelope;
pub type FormsStore = playbook::PlaybookStore;
//#endregion 🔖Types

/// 📦 Encodes a `FormOperation` to its binary command form.
pub fn encode_op(operation: &FormOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `FormOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<FormOperation, protocol::ProtocolError> {
    FormOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use forms::{FormStep, FORMS_DOCUMENT_SCHEMA};
    use forms_engine::empty_forms_projection;
    use store::create_document_envelope;

    #[test]
    fn forms_document_vcs_materializes() {
        let store = FormsStore::new(create_document_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let projection = store.projection().expect("projection");
        assert_eq!(projection.schema, FORMS_DOCUMENT_SCHEMA);
    }

    #[test]
    fn add_step_op_replays() {
        let mut store = FormsStore::new(create_document_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let step = FormStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        store.dispatch(store::DocumentCommand::Apply { operations: vec![FormOperation::AddStep { step, index: None }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 2);
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = FormOperation::UpdatePlaybook { title: Some("Renamed".into()) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪Tests
