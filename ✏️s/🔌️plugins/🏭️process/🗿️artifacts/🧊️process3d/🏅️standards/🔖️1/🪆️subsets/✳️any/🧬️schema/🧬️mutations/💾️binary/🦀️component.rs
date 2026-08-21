//! ⚖️ Process3d artifact — binary operation wire codec surface + laws (constitutional: spr, renamed
//! from the old `📡️protocol` module — no `📡️protocol` path segment may survive under `✏️s/🔌️plugins/`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::process3d::schema::mutations::text::Process3dMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `Process3dMutation` to its binary command form.
pub async fn encode_op(operation: &Process3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Process3dMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<Process3dMutation, protocol::ProtocolError> {
    Process3dMutation::decode_op(bytes)
}
