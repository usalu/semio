//! ⚖️ Lowpoly artifact — binary command wire codec (constitutional: spr; renamed from the old
//! `📡️protocol` module — no `📡️protocol` path segment may survive under a migrated plugin).

use crate::artifacts::lowpoly::op::LowpolyOperation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `LowpolyOperation` to its binary command form.
pub fn encode_op(operation: &LowpolyOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LowpolyOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<LowpolyOperation, protocol::ProtocolError> {
    LowpolyOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::engine::default_projection;

    #[test]
    fn handcrafted_spr_protocol_enumerates_operation_tags() {
        for tag in [
            "record ObjectsAdd tag 1",
            "record ObjectsRemove tag 2",
            "record ObjectsMove tag 3",
            "record ObjectsPatch tag 4",
            "record AddPaintLayer tag 5",
            "record RemovePaintLayer tag 6",
            "record PatchPaintLayer tag 7",
            "record PaintStroke tag 8",
            "record SetProjection tag 9",
        ] {
            assert!(COMPONENT_PROTOCOL_SEMIO.contains(tag), "missing {tag}");
        }
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("field format u8"));
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("field ordinal varint"));
    }
    use crate::artifacts::lowpoly::op::LowpolyPaintLayerPatch;
    use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = LowpolyOperation::ObjectsMove { id: "obj-1".into(), to_index: 2 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_after_applying_an_operation() {
        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let envelope = store::create_document_envelope::<crate::artifacts::lowpoly::LowpolyProjection, LowpolyOperation>(LOWPOLY_DOCUMENT_SCHEMA, "test-doc", projection, None);
        let mut doc_store = store::DocumentStore::new(envelope);
        let operation = LowpolyOperation::PatchPaintLayer { object_id, index: 0, patch: LowpolyPaintLayerPatch { name: Some("Renamed Layer".into()), visible: None, opacity: None, blend_mode: None } };
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![operation], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
