//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::shooting::config::component::mutations::*;
use crate::editor::shooting::config::component::*;
use replace_config::ReplaceConfig;
use set_shot_selection::SetShotSelection;
use set_center_model::SetCenterModel;
use set_fit_revision::SetFitRevision;
use set_camera::SetCamera;
use set_defaults::SetDefaults;

impl protocol::OpBinary for ShootingConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
