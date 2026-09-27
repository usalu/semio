//! 💾️ Direct compact JSON snapshot-patch binary codec.

use super::*;
use protocol::OpBinary;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "patch-snapshot");

pub fn encode(value: &JsonMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JsonMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(patch.encode_op())
}

pub fn decode(bytes: &[u8]) -> Result<JsonMutation, protocol::ProtocolError> {
    editing::SnapshotPatch::decode_op(bytes).map(|patch| JsonMutation::PatchSnapshot(PatchSnapshot { patch }))
}
