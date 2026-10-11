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

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = ShootingConfig, diff = ShootingConfigDiff, schema = "shooting.config")]
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

impl store::snapshot_clone_preparation::ConfigApplyMutation<ShootingConfig> for ShootingConfigMutation {
    fn exchange(self, post: &mut ShootingConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::ReplaceConfig(ReplaceConfig { config }) => Self::ReplaceConfig(ReplaceConfig { config: std::mem::replace(post, config) }),
            Self::SetShotSelection(SetShotSelection { shot_ids }) => Self::SetShotSelection(SetShotSelection { shot_ids: std::mem::replace(&mut post.selected_shot_ids, shot_ids) }),
            Self::SetCenterModel(SetCenterModel { value }) => Self::SetCenterModel(SetCenterModel { value: std::mem::replace(&mut post.center_model, value) }),
            Self::SetFitRevision(SetFitRevision { value }) => Self::SetFitRevision(SetFitRevision { value: std::mem::replace(&mut post.fit_revision, value) }),
            Self::SetCamera(SetCamera { camera }) => Self::SetCamera(SetCamera { camera: std::mem::replace(&mut post.camera, camera) }),
            Self::SetDefaults(SetDefaults { shot_format, shot_shape, asset_format }) => Self::SetDefaults(SetDefaults {
                shot_format: std::mem::replace(&mut post.default_shot_format, shot_format),
                shot_shape: std::mem::replace(&mut post.default_shot_shape, shot_shape),
                asset_format: std::mem::replace(&mut post.default_asset_format, asset_format),
            }),
        })
    }

    fn payload_bytes(&self) -> usize {
        let config_bytes = |config: &ShootingConfig| config.default_shot_format.len() + config.default_shot_shape.len() + config.default_asset_format.len() + config.selected_shot_ids.iter().map(String::len).sum::<usize>();
        match self {
            Self::ReplaceConfig(ReplaceConfig { config }) => config_bytes(config),
            Self::SetShotSelection(SetShotSelection { shot_ids }) => shot_ids.iter().map(String::len).sum(),
            Self::SetDefaults(SetDefaults { shot_format, shot_shape, asset_format }) => shot_format.len() + shot_shape.len() + asset_format.len(),
            Self::SetCenterModel(_) | Self::SetFitRevision(_) | Self::SetCamera(_) => 0,
        }
    }
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
