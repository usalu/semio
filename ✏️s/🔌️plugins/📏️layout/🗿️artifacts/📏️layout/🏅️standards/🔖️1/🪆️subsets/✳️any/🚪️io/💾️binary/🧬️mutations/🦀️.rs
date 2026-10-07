//! ⚖️ Layout artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpText`/`protocol::OpBinary for LayoutMutation` are implemented directly in
//! `../../📝️text/🧬️mutations/🦀️.rs` (`serde_json`-based, no DSL mirror needed now that every variant wraps
//! a plain local payload struct — see that file's doc comment for why the pre-migration
//! `LayoutMutationDsl`/`FramePatchDsl`/`ColorPatch` mirrors were retired). This component only adds
//! the thin artifact-facing `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `LayoutCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::LayoutMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `LayoutMutation` to its binary state-patch form.
pub fn encode_op(operation: &LayoutMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LayoutMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<LayoutMutation, protocol::ProtocolError> {
    LayoutMutation::decode_op(bytes)
}

//#region 🔖️Store














//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
pub use crate::mutations::LayoutMutation;
pub use mutations_wire_codec::*;
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
impl protocol::OpBinary for LayoutMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Key, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Key, bytes)
    }
}
}
