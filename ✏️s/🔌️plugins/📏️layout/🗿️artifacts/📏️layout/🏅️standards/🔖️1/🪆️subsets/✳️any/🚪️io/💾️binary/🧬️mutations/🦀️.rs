//! ⚖️ Layout mutation binary codec and original receiving use the authored operation tags.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::LayoutMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `LayoutMutation` to its binary state-patch form.
pub fn encode_op(operation: &LayoutMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LayoutMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<LayoutMutation, protocol::ProtocolError> {
    LayoutMutation::decode_op(bytes)
}

impl OpBinary for LayoutMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Key, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Key, bytes)
    }
}

dsl::original_pack_value_operation_receiving!(LayoutMutation, tagged_key = COMPONENT_PROTOCOL_SEMIO);
dsl::artifact_persisted_binary_operation!(LayoutMutation);
