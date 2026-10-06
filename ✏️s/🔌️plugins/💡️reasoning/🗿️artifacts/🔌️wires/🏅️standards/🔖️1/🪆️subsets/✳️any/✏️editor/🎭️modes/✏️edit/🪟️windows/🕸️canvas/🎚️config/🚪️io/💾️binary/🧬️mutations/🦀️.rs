//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::wires::modes::edit::windows::canvas::config::mutations::*;
use crate::editor::wires::modes::edit::windows::canvas::config::{WiresCanvasCamera, WiresCanvasWindowConfig};
use set_camera::SetCamera;

impl protocol::OpBinary for WiresCanvasWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
