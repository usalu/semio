//! 🧬️ Semantic `Wfc3dConfig` mutation vocabulary and codecs.

use super::Wfc3dConfig;

#[path = "🔄️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "🎥️change-camera/🦀️.rs"]
mod change_camera;
pub use change_camera::ChangeCamera;
#[path = "🀄️change-active-tile/🦀️.rs"]
mod change_active_tile;
pub use change_active_tile::ChangeActiveTile;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = Wfc3dConfig, diff = Wfc3dConfig, schema = "wfc.wfc3d.config")]
pub enum Wfc3dConfigMutation {
    ReplaceConfig(ReplaceConfig),
    ChangeCamera(ChangeCamera),
    ChangeActiveTile(ChangeActiveTile),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
