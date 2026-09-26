//! 💾️ EN 1992 mutation binary wire.

pub use crate::artifact_schema::mutations::En1992Mutation;
use protocol::OpBinary;

pub const COMPONENT_PROTOCOL_SEMIO: &str = "";
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::protocol");

impl OpBinary for En1992Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))?;
        pack::json::from_json_str(text).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}

pub fn encode_op(mutation: &En1992Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}
pub fn decode_op(bytes: &[u8]) -> Result<En1992Mutation, protocol::ProtocolError> {
    En1992Mutation::decode_op(bytes)
}
