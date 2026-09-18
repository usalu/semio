//! ⚖️ `s.wfc.grid3d` — state-patch-representation wire codec + laws.
//!
//! The binary TAG of an operation is its position in `Grid3dOperationDsl`'s own variant table, which
//! `dsl::variants_binary` derives from the same `DslVariants` roster the text keywords come from —
//! so a kind can never carry one tag on the wire and another in the grammar.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::text::{operation_from_dsl, operation_to_dsl, Grid3dOperationDsl};
use crate::schema::mutations::Grid3dMutation;
use protocol::OpBinary;

//#region 🔖️HandcraftedOpCodecs
impl OpBinary for Grid3dOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge in the sibling `📝️text` facet.
impl OpBinary for Grid3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(operation_from_dsl(Grid3dOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📦️ Encodes a `Grid3dMutation` to its binary state-patch form.
pub fn encode_op(operation: &Grid3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Grid3dMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Grid3dMutation, protocol::ProtocolError> {
    Grid3dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode_op`/`decode_op` speak.
pub type Grid3dMutationBinary = Vec<u8>;
//#endregion 🚚️Carrier
