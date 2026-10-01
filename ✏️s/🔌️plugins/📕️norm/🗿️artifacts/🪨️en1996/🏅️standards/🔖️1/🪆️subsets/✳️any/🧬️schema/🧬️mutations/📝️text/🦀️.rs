//! ⚡️ En1996 mutations — OpText via JSON tokens, OpBinary via the norm-wide protocol-tagged payload frame.

pub use crate::artifact_schema::mutations::En1996Mutation;

use protocol::{OpBinary, OpText};

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

impl OpText for En1996Mutation {
    fn print_op(&self) -> String {
        pack::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        pack::json::from_json_str(line).map_err(|e| store::TextError::new(e.to_string(), store::TextSpan::at(1, 1)))
    }
}

/// 💾️ The norm-wide payload op frame (`semio_s_artifact_norm_contract::payload_op_binary`), tagged by this subset's
/// `📡️.protocol.semio` records.
impl OpBinary for En1996Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::encode::<crate::En1996Snapshot, _>(include_str!("../💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::decode::<crate::En1996Snapshot, _>(include_str!("../💾️binary/📡️.protocol.semio"), bytes)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
