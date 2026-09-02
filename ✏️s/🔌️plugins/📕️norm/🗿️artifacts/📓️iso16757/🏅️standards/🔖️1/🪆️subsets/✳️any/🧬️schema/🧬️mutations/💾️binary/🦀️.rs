//! ⚖️ ISO 16757 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::iso16757::schema::mutations::text::Iso16757Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Iso16757Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Iso16757Mutation, protocol::ProtocolError> {
    Iso16757Mutation::decode_op(bytes)
}
