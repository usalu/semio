//! ⚖️ EN 1994 design of composite steel and concrete structures — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::En1994Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1994Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1994Mutation, protocol::ProtocolError> {
    En1994Mutation::decode_op(bytes)
}

mod native_codec {
use super::*;
use crate::artifact_schema::mutations::En1994Mutation;
use protocol::OpText;

impl protocol::OpBinary for En1994Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(text).map_err(|e| protocol::ProtocolError::Malformed { what: "json", offset: 0, detail: e.to_string() })
    }
}
}
