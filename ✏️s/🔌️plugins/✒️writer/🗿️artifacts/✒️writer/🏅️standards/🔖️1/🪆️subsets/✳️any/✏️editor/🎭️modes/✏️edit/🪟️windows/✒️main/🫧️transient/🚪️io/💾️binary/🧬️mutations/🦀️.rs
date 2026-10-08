//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::writer::modes::edit::windows::main::component::transient::mutations::*;
use crate::editor::writer::modes::edit::windows::main::component::transient::WriterMainWindowTransient;
use crate::editor::writer::modes::edit::windows::main::transient::SetEditorSelection;
use crate::editor::writer::modes::edit::windows::main::transient::SetLintGeneration;
use crate::editor::writer::modes::edit::windows::main::transient::SetEngagementInput;

impl protocol::OpBinary for WriterMainWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
