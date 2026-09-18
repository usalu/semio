//! ⚖️ `wfc3d` artifact — state-patch-representation wire codec + laws.
//!
//! The binary TAG of an operation is its position in `Wfc3dOperationDsl`'s own variant table, which
//! `dsl::variants_binary` derives from the same `DslVariants` roster the text keywords come from —
//! so a kind can never carry one tag on the wire and another in the grammar.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::text::{operation_from_dsl, operation_to_dsl, Wfc3dOperationDsl};
use crate::schema::mutations::Wfc3dMutation;
use protocol::OpBinary;

//#region 🔖️HandcraftedOpCodecs
impl OpBinary for Wfc3dOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge in the sibling `📝️text` facet.
impl OpBinary for Wfc3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Wfc3dOperationDsl::decode_op(bytes)?;
        operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "wfc3d mutation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📦️ Encodes a `Wfc3dMutation` to its binary state-patch form.
pub fn encode_op(operation: &Wfc3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Wfc3dMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Wfc3dMutation, protocol::ProtocolError> {
    Wfc3dMutation::decode_op(bytes)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode_op`/`decode_op` speak.
pub type Wfc3dMutationBinary = Vec<u8>;
//#endregion 🚚️Carrier
