//! 🧬️ Shooting configuration mutation collection.

use super::*;
#[path = "📸️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "☑️set-shot-selection/🦀️.rs"]
mod set_shot_selection;
pub use set_shot_selection::SetShotSelection;
#[path = "🎯️set-center-model/🦀️.rs"]
mod set_center_model;
pub use set_center_model::SetCenterModel;
#[path = "🔢️set-fit-revision/🦀️.rs"]
mod set_fit_revision;
pub use set_fit_revision::SetFitRevision;
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "🔧️set-defaults/🦀️.rs"]
mod set_defaults;
pub use set_defaults::SetDefaults;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ShootingConfig, diff = ShootingConfig, schema = "shooting.config")]
pub enum ShootingConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-shot-selection")]
    SetShotSelection(SetShotSelection),
    #[dsl(key = "set-center-model")]
    SetCenterModel(SetCenterModel),
    #[dsl(key = "set-fit-revision")]
    SetFitRevision(SetFitRevision),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-defaults")]
    SetDefaults(SetDefaults),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
