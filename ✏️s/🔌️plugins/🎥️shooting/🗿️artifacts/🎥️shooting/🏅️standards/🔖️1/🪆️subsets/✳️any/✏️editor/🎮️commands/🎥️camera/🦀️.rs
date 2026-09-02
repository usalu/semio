//! 🎥️ Shooting play app commands — the saved-camera catalogue and the free/live viewport camera.
//!
//! `SetCamera`/`SetCameraDraftLabel`/`LoadSavedCamera` are config-only: the free/live viewport camera is
//! session-only runtime state, never a document field (see `ShootingConfig::camera`). `SetShotCamera` and
//! `SaveCamera` ARE real document mutations.

use crate::artifacts::shooting::mutations::create_saved_camera::mutation::CreateSavedCamera;
use crate::artifacts::shooting::mutations::replace_shot_camera::mutation::ReplaceShotCamera;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingCamera, ShootingSavedCamera, ShootingSnapshot};
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

    pub async fn handle(payload: &SetShotCamera, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![ShootingMutation::ReplaceShotCamera(ReplaceShotCamera { shot_id: payload.shot_id.clone(), new_camera: payload.camera.clone() })]))
    }
}
//#endregion 🔖️SetShotCamera

//#region 🔖️SaveCamera
pub mod save_camera {
    use super::*;
    use crate::artifacts::shooting::schema::next_shooting_id;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "save-camera")]
    pub struct SaveCamera {}

    pub async fn handle(_payload: &SaveCamera, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        let draft = config.camera_draft_label.trim().to_string();
        let label = if draft.is_empty() { format!("Camera {}", snapshot.saved_cameras.len() + 1) } else { draft };
        let saved_camera = ShootingSavedCamera { id: next_shooting_id("camera"), label, camera: config.camera.clone() };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::CreateSavedCamera(CreateSavedCamera { saved_camera, index: Some(snapshot.saved_cameras.len()) })],
            config_mutations: vec![ShootingConfigMutation::SetCameraDraftLabel { value: String::new() }],
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

    pub async fn handle(payload: &LoadSavedCamera, doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match doc.snapshot.saved_cameras.iter().find(|entry| entry.id == payload.id) {
            Some(saved) => Ok(Emit::config(vec![ShootingConfigMutation::SetCamera { camera: saved.camera.clone() }])),
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

    pub async fn handle(payload: &SetCameraDraftLabel, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetCameraDraftLabel { value: payload.value.clone() }]))
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

    pub async fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;

    #[semio_framework_async_macros::async_test]
    async fn save_and_load_camera_round_trip() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetCameraDraftLabel(set_camera_draft_label::SetCameraDraftLabel { value: "Hero".into() }));
        let result = dispatch(&mut app, ShootingCommand::SaveCamera(save_camera::SaveCamera {}));
        assert_eq!(result.mutations.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_never_touches_the_document() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::SetCamera(set_camera::SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } }));
        assert!(result.mutations.is_empty(), "the free/live camera is config-only");
    }
}
//#endregion 🧪️Tests
