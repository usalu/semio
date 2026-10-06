//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::generation3d::config::component::mutations::*;
use crate::editor::generation3d::config::component::{CameraJson, Generation3dConfig, Generation3dPreviewCamera};
use set_camera::SetCamera;
use set_lod_mode::SetLodMode;
use set_preview_camera::SetPreviewCamera;
use set_selected_generation::SetSelectedGeneration;
use set_show_mode::SetShowMode;
use set_snapshot::SetSnapshot;
use set_sun::SetSun;

impl protocol::OpBinary for Generation3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
