//! ⚖️ Binary op codec of `ModelMutation`: `format`, wire tag (the leaf `binaryTag`, declared in `📡️.protocol.semio`) and the payload value.

/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::ModelMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `ModelMutation` to its binary op form.
pub fn encode_op(operation: &ModelMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `ModelMutation` from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<ModelMutation, protocol::ProtocolError> {
    ModelMutation::decode_op(bytes)
}

impl OpBinary for ModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}
