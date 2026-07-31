//! ⚖️ Procedural 2D app — binary command protocol surface + laws (constitutional: protocol).

use procedural_2d_op::Procedural2dOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `Procedural2dOperation` to its binary command form.
pub fn encode_op(operation: &Procedural2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural2dOperation, protocol::ProtocolError> {
    Procedural2dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use procedural_2d::PROCEDURAL_2D_SCHEMA;
    use procedural_2d_engine::empty_procedural2d_projection;
    use procedural_2d_op::Procedural2dStore;
    use store::{create_document_envelope, test_support, DocumentCommand};

    //#region 🔖️DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = Procedural2dStore::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Procedural2dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DocumentTextTests
}
//#endregion 🧪️Tests
