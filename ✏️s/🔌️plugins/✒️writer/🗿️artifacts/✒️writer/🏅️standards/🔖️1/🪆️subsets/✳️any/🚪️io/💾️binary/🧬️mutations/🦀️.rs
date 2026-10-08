//! ⚖️ Writer artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! This component only carries the artifact-facing `encode_op`/`decode_op` wrappers plus the op
//! text↔binary equivalence law and a whole-store round trip. The app's typed `WriterCommand` enum —
//! which used to share the old `📡️protocol` crate with this codec — is an EDITOR-surface concern, not
//! an artifact one: it now lives in the subset's `✏️editor/🦀️.rs`, assembled from the
//! `🎮️commands/*` payload
//! modules by `semio_framework_plugin::app_commands!`.

use crate::op::WriterMutation;
use crate::schema;
use crate::WriterSnapshot;
use protocol::{Mutation, OpBinary};
use store::ArtifactEnvelopeMutationFieldTarget;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `WriterMutation` to its binary state-patch form.
pub fn encode_op(operation: &WriterMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `WriterMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<WriterMutation, protocol::ProtocolError> {
    WriterMutation::decode_op(bytes)
}

//#region 🔖️OwnedEnvelopeCatalog








































































//#region 🔖️Store














//#endregion 🔖️Store












//#endregion 🔖️OwnedEnvelopeCatalog

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;


pub const BINARY_TAG_REGISTRY: &[(&str, u8)] =
    &[("rename-writer", crate::standards::v1::subsets::any::io::binary::mutations::rename_writer::BINARY_TAG), ("change-uri", crate::standards::v1::subsets::any::io::binary::mutations::change_uri::BINARY_TAG), ("change-language", crate::standards::v1::subsets::any::io::binary::mutations::change_language::BINARY_TAG), ("edit-text", crate::standards::v1::subsets::any::io::binary::mutations::edit_text::BINARY_TAG), ("splice-text", crate::standards::v1::subsets::any::io::binary::mutations::splice_text::BINARY_TAG)];

#[path = "🌐change-language/🦀️.rs"]
pub mod change_language;

#[path = "✏️edit-text/🦀️.rs"]
pub mod edit_text;

#[path = "✂️splice-text/🦀️.rs"]
pub mod splice_text;

#[path = "🔗change-uri/🦀️.rs"]
pub mod change_uri;

#[path = "🏷️rename-writer/🦀️.rs"]
pub mod rename_writer;

mod native_codec {
use super::*;
use crate::schema::mutations::{change_language, change_uri, edit_text, inverse_writer_mutation, rename_writer, splice_text, ChangeLanguage, ChangeUri, EditText, RenameWriter, SpliceText, WriterMutation};
use crate::central_apply::{apply_writer_mutation};

impl protocol::OpBinary for WriterMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
