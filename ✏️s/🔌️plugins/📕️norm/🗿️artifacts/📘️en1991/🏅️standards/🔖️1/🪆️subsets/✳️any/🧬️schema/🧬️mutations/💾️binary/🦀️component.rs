//! ⚖️ EN 1991 actions on structures — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::en1991::schema::mutations::text::En1991Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub async fn encode_op(mutation: &En1991Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub async fn decode_op(bytes: &[u8]) -> Result<En1991Mutation, protocol::ProtocolError> {
    En1991Mutation::decode_op(bytes)
}
