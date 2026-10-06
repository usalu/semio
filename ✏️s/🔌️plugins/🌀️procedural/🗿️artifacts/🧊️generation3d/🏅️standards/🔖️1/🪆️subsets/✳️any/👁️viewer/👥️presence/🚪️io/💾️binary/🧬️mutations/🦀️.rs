//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::viewer::generation3d::presence::component::mutations::*;
use crate::viewer::generation3d::presence::component::Generation3dViewPresence;
use crate::viewer::generation3d::config::Generation3dViewCamera;
use set_preview_camera::SetPreviewCamera;
use set_show_mode::SetShowMode;

impl protocol::OpBinary for Generation3dViewPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
