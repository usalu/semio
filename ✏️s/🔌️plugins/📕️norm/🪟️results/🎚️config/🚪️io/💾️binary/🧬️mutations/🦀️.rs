//! 🎚️ Norm configuration mutation 💾️binary codec.

use crate::results_window_config::NormResultsWindowConfigMutation;

/// 🎯️ Uses the shared canonical terminal-operation frame for this schema-owned aggregate.
impl protocol::OpBinary for NormResultsWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
