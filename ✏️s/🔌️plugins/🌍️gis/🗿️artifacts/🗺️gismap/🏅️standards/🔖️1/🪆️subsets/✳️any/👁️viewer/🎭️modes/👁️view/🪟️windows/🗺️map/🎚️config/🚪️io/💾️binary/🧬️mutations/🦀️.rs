//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::viewer::gismap::modes::view::windows::map::config::mutations::*;
use crate::viewer::gismap::modes::view::windows::map::config::{GisMapViewerCamera, GisMapViewerWindowConfig};


impl protocol::OpBinary for GisMapViewerWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
