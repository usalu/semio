//! 🧬️ Semantic `Wfc3dConfig` mutation vocabulary and codecs.

use crate::editor::wfc3d::config::{Wfc3dConfig, Wfc3dConfigDiff};

#[path = "🔄️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "🎥️change-camera/🦀️.rs"]
mod change_camera;
pub use change_camera::ChangeCamera;
#[path = "🀄️change-active-tile/🦀️.rs"]
mod change_active_tile;
pub use change_active_tile::ChangeActiveTile;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_dsl_record_derive::DslReceiving, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", content = "payload", rename_all = "camelCase"))]
#[mutations(snapshot = Wfc3dConfig, diff = Wfc3dConfigDiff, schema = "wfc.wfc3d.config", reference = Wfc3dConfig, reference_diff = Wfc3dConfigDiff)]
pub enum Wfc3dConfigMutation {
    ReplaceConfig(ReplaceConfig),
    ChangeCamera(ChangeCamera),
    ChangeActiveTile(ChangeActiveTile),
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<Wfc3dConfig> for Wfc3dConfigMutation {
    fn exchange(self, post: &mut Wfc3dConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::ReplaceConfig(ReplaceConfig { config }) => Self::ReplaceConfig(ReplaceConfig { config: std::mem::replace(post, config) }),
            Self::ChangeCamera(ChangeCamera { x, y, zoom }) => Self::ChangeCamera(ChangeCamera {
                x: std::mem::replace(&mut post.camera_x, x),
                y: std::mem::replace(&mut post.camera_y, y),
                zoom: std::mem::replace(&mut post.camera_zoom, zoom),
            }),
            Self::ChangeActiveTile(ChangeActiveTile { tile_id }) => Self::ChangeActiveTile(ChangeActiveTile { tile_id: std::mem::replace(&mut post.active_tile_id, tile_id) }),
        })
    }

    fn payload_bytes(&self) -> usize {
        match self {
            Self::ReplaceConfig(ReplaceConfig { config }) => config.active_tile_id.len(),
            Self::ChangeActiveTile(ChangeActiveTile { tile_id }) => tile_id.len(),
            Self::ChangeCamera(_) => 0,
        }
    }
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
