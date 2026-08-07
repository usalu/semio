//! ⚖️ Mathematical artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! `protocol::OpBinary for MathOperation` is implemented directly in `crate::artifacts::mathematical::op`
//! (see that module's doc comment). This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law and a whole-store round trip.
//!
//! The app's typed `MathCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `🎛️apps/➗️mathematical/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::mathematical::op::MathOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `MathOperation` to its binary command form.
pub fn encode_op(operation: &MathOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `MathOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<MathOperation, protocol::ProtocolError> {
    MathOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mathematical::MathProjection;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = MathOperation::SetGraph { graph: crate::artifacts::mathematical::MathGraph::default() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn math_document_text_round_trips_through_store() {
        let initial = MathProjection::default();
        let envelope = store::create_document_envelope(crate::artifacts::mathematical::MATH_DOCUMENT_SCHEMA, "math-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        let graph = crate::artifacts::mathematical::MathGraph { algorithm: "components".into(), ..crate::artifacts::mathematical::MathGraph::default() };
        store.dispatch(store::DocumentCommand::Apply { operations: vec![MathOperation::SetGraph { graph }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
