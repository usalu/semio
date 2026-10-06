//! 🎥️ Shooting play app commands — the saved-camera catalogue and the free/live viewport camera.
//!
//! `SetCamera`/`LoadSavedCamera` are config-only: the free/live viewport camera is session-only runtime state, never a
//! document field (see `ShootingConfig::camera`). `SetShotCamera` and `SaveCamera` ARE real document mutations; the
//! label `SaveCamera` stores is the text the scene window's camera field submits — typing it publishes nothing.

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use crate::mutations::create_saved_camera::CreateSavedCamera;
use crate::mutations::replace_shot_camera::ReplaceShotCamera;
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
use crate::{ShootingCamera, ShootingSavedCamera, ShootingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetShotCamera
pub mod set_shot_camera {
    use super::*;

    /// 🎥️ Deliberately overwrites `shot_id`'s *saved* camera with the given pose — a real, undoable
    /// document edit. A no-op when that shot has no saved camera (the free/live camera is `SetCamera`'s
    /// job, and never reaches this operation).
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
    use crate::standards::v1::subsets::any::schema::next_shooting_id;

    /// 💾️ Saves the live viewport camera under `label` (the submitted camera field; `Camera N` when blank).
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "save-camera")]
    pub struct SaveCamera {
        pub label: String,
    }

    pub fn handle(payload: &SaveCamera, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let typed = payload.label.trim();
        let label = if typed.is_empty() { format!("Camera {}", snapshot.saved_cameras.len() + 1) } else { typed.to_string() };
        let saved_camera = ShootingSavedCamera { id: next_shooting_id("camera"), label, camera: cfg.snapshot.camera.clone() };
        Ok(Emit::mutations(vec![ShootingMutation::CreateSavedCamera(CreateSavedCamera { saved_camera, index: Some(snapshot.saved_cameras.len()) })]))
    }
}
//#endregion 🔖️SaveCamera

//#region 🔖️LoadSavedCamera
pub mod load_saved_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: ShootingCamera,
    }

    /// 🎥️ ONE config edit per dispatch: both World3d hosts dispatch `setCamera` once, when a navigation gesture
    /// settles (React debounced at its end, wgpu on camera settle), so a camera orbit is one edit, never one per
    /// pointer move (design §20.1: no amend on any lane).
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
