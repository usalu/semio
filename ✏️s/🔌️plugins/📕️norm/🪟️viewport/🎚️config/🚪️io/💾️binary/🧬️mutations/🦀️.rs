//! 🎚️ Norm configuration mutation 💾️binary codec.

use crate::mutations::NormViewportWindowConfigMutation;

/// 🎯️ Uses the shared canonical terminal-operation frame for this schema-owned aggregate.
impl protocol::OpBinary for NormViewportWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
