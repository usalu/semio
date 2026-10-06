//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::playbook::config::component::mutations::*;
use crate::editor::playbook::config::component::PlaybookConfig;
use replace_config::ReplaceConfig;
use set_contributions::SetContributions;

impl protocol::OpBinary for PlaybookConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
