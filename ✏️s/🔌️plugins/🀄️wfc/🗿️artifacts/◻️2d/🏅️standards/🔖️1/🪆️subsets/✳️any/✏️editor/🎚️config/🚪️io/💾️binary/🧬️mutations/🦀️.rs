//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::wfc2d::config::mutations::*;
use crate::editor::wfc2d::config::Wfc2dConfig;
use crate::editor::wfc2d::config::mutations::ReplaceConfig;
use crate::editor::wfc2d::config::mutations::ChangeCamera;
use crate::editor::wfc2d::config::mutations::ChangeActiveTile;

impl protocol::OpBinary for Wfc2dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
