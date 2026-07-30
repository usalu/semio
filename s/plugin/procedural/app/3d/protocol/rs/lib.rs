//! ⚖️ Procedural 3D app — binary command protocol surface + laws (constitutional: protocol).

use procedural_3d_op::Procedural3dOperation;
use protocol::OpBinary;

/// 📦 Encodes a `Procedural3dOperation` to its binary command form.
pub fn encode_op(operation: &Procedural3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `Procedural3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural3dOperation, protocol::ProtocolError> {
    Procedural3dOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use procedural_3d::PROCEDURAL_3D_SCHEMA;
    use procedural_3d_engine::empty_procedural3d_projection;
    use procedural_3d_op::Procedural3dStore;
    use store::{create_document_envelope, test_support, DocumentCommand};

    //#region 🔖DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests
