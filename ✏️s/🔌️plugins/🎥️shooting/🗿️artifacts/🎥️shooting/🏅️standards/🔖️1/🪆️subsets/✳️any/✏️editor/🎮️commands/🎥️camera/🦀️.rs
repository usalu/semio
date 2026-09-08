//! 🎥️ Shooting play app commands — the saved-camera catalogue and the free/live viewport camera.
//!
//! `SetCamera`/`SetCameraDraftLabel`/`LoadSavedCamera` are config-only: the free/live viewport camera is
//! session-only runtime state, never a document field (see `ShootingConfig::camera`). `SetShotCamera` and
//! `SaveCamera` ARE real document mutations.

use crate::mutations::create_saved_camera::CreateSavedCamera;
use crate::mutations::replace_shot_camera::ReplaceShotCamera;
use crate::op::ShootingMutation;
use crate::{ShootingCamera, ShootingSavedCamera, ShootingSnapshot};
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetShotCamera
pub mod set_shot_camera {
    use super::*;

    /// 🎥️ Deliberately overwrites `shot_id`'s *saved* camera with the given pose — a real, undoable
    /// document edit. A no-op when that shot has no saved camera (the free/live camera is `SetCamera`'s
    /// job, and never reaches this operation).
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "shot-camera")]
    pub struct SetShotCamera {
        pub shot_id: String,
        #[dsl(block)]
        pub camera: ShootingCamera,
    }

    pub fn handle(payload: &SetShotCamera, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::ReplaceShotCamera(ReplaceShotCamera { shot_id: payload.shot_id.clone(), new_camera: payload.camera.clone() })]))
    }
}
//#endregion 🔖️SetShotCamera

//#region 🔖️SaveCamera
pub mod save_camera {
    use super::*;
    use crate::schema::next_shooting_id;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "save-camera")]
    pub struct SaveCamera {}

    pub fn handle(_payload: &SaveCamera, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        let draft = config.camera_draft_label.trim().to_string();
        let label = if draft.is_empty() { format!("Camera {}", snapshot.saved_cameras.len() + 1) } else { draft };
        let saved_camera = ShootingSavedCamera { id: next_shooting_id("camera"), label, camera: config.camera.clone() };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::CreateSavedCamera(CreateSavedCamera { saved_camera, index: Some(snapshot.saved_cameras.len()) })],
            config_mutations: vec![ShootingConfigMutation::SetCameraDraftLabel(crate::editor::shooting::config::SetCameraDraftLabel { value: String::new() })],
            ..Default::default()
        })
    }
}
//#endregion 🔖️SaveCamera

//#region 🔖️LoadSavedCamera
pub mod load_saved_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "load-saved-camera")]
    pub struct LoadSavedCamera {
        pub id: String,
    }

    pub fn handle(payload: &LoadSavedCamera, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match doc.snapshot.saved_cameras.iter().find(|entry| entry.id == payload.id) {
            Some(saved) => Ok(Emit::config(vec![ShootingConfigMutation::SetCamera(crate::editor::shooting::config::SetCamera { camera: saved.camera.clone() })])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️LoadSavedCamera

//#region 🔖️SetCameraDraftLabel
pub mod set_camera_draft_label {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "camera-draft-label")]
    pub struct SetCameraDraftLabel {
        pub value: String,
    }

    pub fn handle(payload: &SetCameraDraftLabel, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetCameraDraftLabel(crate::editor::shooting::config::SetCameraDraftLabel { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetCameraDraftLabel

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: ShootingCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetCamera(crate::editor::shooting::config::SetCamera { camera: payload.camera.clone() })]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
