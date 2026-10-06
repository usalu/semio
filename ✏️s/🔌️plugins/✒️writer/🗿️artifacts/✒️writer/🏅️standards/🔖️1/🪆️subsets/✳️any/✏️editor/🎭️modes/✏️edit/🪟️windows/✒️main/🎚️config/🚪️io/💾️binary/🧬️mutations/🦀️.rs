//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::writer::modes::edit::windows::main::component::config::mutations::*;
use crate::editor::writer::modes::edit::windows::main::component::config::WriterMainWindowConfig;
use set_camera::SetCamera;
use set_editor_settings::SetEditorSettings;

impl protocol::OpBinary for WriterMainWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
