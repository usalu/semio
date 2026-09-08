//! 📡️ CAD artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `CadMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! cad host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

use crate::op::CadMutation;
use crate::CadSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `CadMutation` to its binary command form.
pub fn encode_op(operation: &CadMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `CadMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<CadMutation, protocol::ProtocolError> {
    CadMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type CadEnvelope = ArtifactEnvelope<CadSnapshot, CadMutation>;
pub type CadStore = ArtifactStore<CadSnapshot, CadMutation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
