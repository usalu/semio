//! 🧬️ Semantic `Wfc2dConfig` mutation vocabulary and codecs.

use super::Wfc2dConfig;

#[path = "🔄️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "🎥️change-camera/🦀️.rs"]
mod change_camera;
pub use change_camera::ChangeCamera;
#[path = "🀄️change-active-tile/🦀️.rs"]
mod change_active_tile;
pub use change_active_tile::ChangeActiveTile;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", content = "payload", rename_all = "camelCase"))]
#[mutations(snapshot = Wfc2dConfig, diff = Wfc2dConfigDiff, schema = "wfc.wfc2d.config")]
pub enum Wfc2dConfigMutation {
    ReplaceConfig(ReplaceConfig),
    ChangeCamera(ChangeCamera),
    ChangeActiveTile(ChangeActiveTile),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
