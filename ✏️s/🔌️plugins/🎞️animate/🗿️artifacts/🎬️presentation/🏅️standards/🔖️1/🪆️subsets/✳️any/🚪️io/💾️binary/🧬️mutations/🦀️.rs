//! 📡️ Animate presentation artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! Also hosts the `PresentationEnvelope`/`PresentationStore` type aliases and the VCS envelope helpers — both need
//! `PresentationMutation` (from `crate::standards::v1::subsets::any::io::text::mutations`) alongside `PresentationSnapshot` (from the artifact's
//! own component file), so this is the natural home for them.
//!
//! The app's typed `PresentationCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`. Its
//! WASM bridge moved to `✏️editor/🌉️wasm/🦀️component.rs`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::empty_presentation_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use protocol::{Mutation as _, MutationDiff as _, OpBinary};
use store::ArtifactOwnedValueRetirementFactory as _;
use store::{create_document_envelope, ArtifactEnvelope};

//#region 🧬️OwnedEnvelopeCatalog


















































/// 🎯️ Nonblocking publication target for one completed Presentation snapshot owner.
pub trait PresentationProjectionAdoptionTarget {
    #[expect(clippy::result_large_err, reason = "Returns the exact unadopted snapshot owner for retry or incremental retirement without allocating on refusal.")]
    fn try_adopt(&mut self, value: PresentationSnapshot) -> Result<(), PresentationSnapshot>;
}
















































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
use crate::standards::v1::subsets::any::schema::mutations::{apply_presentation_mutation,inverse_presentation_mutation,PresentationMutation};

impl protocol::OpBinary for PresentationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
