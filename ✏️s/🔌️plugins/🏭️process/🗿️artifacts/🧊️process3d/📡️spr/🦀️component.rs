//! ⚖️ Process3d artifact — binary operation wire codec surface + laws (constitutional: spr, renamed
//! from the old `📡️protocol` module — no `📡️protocol` path segment may survive under `✏️s/🔌️plugins/`).

use crate::artifacts::process3d::op::Process3dOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `Process3dOperation` to its binary command form.
pub fn encode_op(operation: &Process3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Process3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Process3dOperation, protocol::ProtocolError> {
    Process3dOperation::decode_op(bytes)
}
