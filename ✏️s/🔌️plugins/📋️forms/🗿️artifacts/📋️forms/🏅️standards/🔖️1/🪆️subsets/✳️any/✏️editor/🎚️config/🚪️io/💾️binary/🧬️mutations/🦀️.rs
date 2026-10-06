//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::forms::config::component::mutations::*;
use crate::editor::forms::config::component::*;
use replace_config::ReplaceConfig;
use set_contributions::SetContributions;

impl protocol::OpBinary for FormsConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        protocol::OpText::parse_op(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}
}
pub use mutations_codec::*;
