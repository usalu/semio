//! 📡️ Presentation mutation binary representation.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use protocol::OpBinary;

//#region 🧬️OwnedEnvelopeCatalog

//#endregion 🧬️OwnedEnvelopeCatalog

/// 📦️ Encodes a `PresentationMutation` to its binary state-patch form.
pub fn encode_op(operation: &PresentationMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `PresentationMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<PresentationMutation, protocol::ProtocolError> {
    PresentationMutation::decode_op(bytes)
}

//#region 🔖️Store

//#endregion 🔖️Store

//#region 🔖️VcsEnvelope

//#endregion 🔖️VcsEnvelope

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;

impl protocol::OpBinary for PresentationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
