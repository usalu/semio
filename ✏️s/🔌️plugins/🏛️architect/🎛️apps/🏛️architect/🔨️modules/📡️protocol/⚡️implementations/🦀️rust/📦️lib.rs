//! 📡️ Architect app operation protocol surface (constitutional: protocol).

use architect_op::ProgramOperation;
use protocol::OpBinary;

//#region 🔖️OperationProtocol
/// 📡️ Encodes an Architect operation for transport or persistence.
pub fn encode(operation: &ProgramOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📥️ Decodes an Architect operation from its transport representation.
pub fn decode(bytes: &[u8]) -> Result<ProgramOperation, protocol::ProtocolError> {
    ProgramOperation::decode_op(bytes)
}
//#endregion 🔖️OperationProtocol
