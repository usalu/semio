//! 🎥️ Shooting play app commands — the saved-camera catalogue and the free/live viewport camera.
//!
//! `SetCamera`/`SetCameraDraftLabel`/`LoadSavedCamera` are config-only: the free/live viewport camera is
//! session-only runtime state, never a document field (see `ShootingConfig::camera`). `SetShotCamera` and
//! `SaveCamera` ARE real document mutations.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigOperation};
use crate::artifacts::shooting::op::ShootingOperation;
use crate::artifacts::shooting::{ShootingCamera, ShootingFixture, ShootingSavedCamera};
use protocol::CollectionOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetShotCamera
pub mod set_shot_camera {
    use super::*;

    /// 🎥️ Deliberately overwrites `shot_id`'s *saved* camera with the given pose — a real, undoable
    /// document edit. A no-op when that shot has no saved camera (the free/live camera is `SetCamera`'s
    /// job, and never reaches this operation).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "shot-camera")]
    pub struct SetShotCamera {
        pub shot_id: String,
        #[dsl(block)]
        pub camera: ShootingCamera,
    }

    pub fn handle(payload: &SetShotCamera, _doc: &DocumentView<'_, ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        Ok(Emit::operations(vec![ShootingOperation::SetShotCamera { shot_id: payload.shot_id.clone(), camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetShotCamera

//#region 🔖️SaveCamera
pub mod save_camera {
    use super::*;
    use crate::artifacts::shooting::engine::next_shooting_id;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "save-camera")]
    pub struct SaveCamera {}

    pub fn handle(_payload: &SaveCamera, doc: &DocumentView<'_, ShootingFixture>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        let draft = config.camera_draft_label.trim().to_string();
        let label = if draft.is_empty() { format!("Camera {}", fixture.saved_cameras.len() + 1) } else { draft };
        let saved_camera = ShootingSavedCamera { id: next_shooting_id("camera"), label, camera: config.camera.clone() };
        Ok(Emit {
            document_operations: vec![ShootingOperation::SavedCameras(CollectionOperation::Add { index: fixture.saved_cameras.len(), item: saved_camera })],
            config_operations: vec![ShootingConfigOperation::SetCameraDraftLabel { value: String::new() }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️SaveCamera

//#region 🔖️LoadSavedCamera
pub mod load_saved_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "load-saved-camera")]
    pub struct LoadSavedCamera {
        pub id: String,
    }

    pub fn handle(payload: &LoadSavedCamera, doc: &DocumentView<'_, ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        match doc.projection.saved_cameras.iter().find(|entry| entry.id == payload.id) {
            Some(saved) => Ok(Emit::config(vec![ShootingConfigOperation::SetCamera { camera: saved.camera.clone() }])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️LoadSavedCamera

//#region 🔖️SetCameraDraftLabel
pub mod set_camera_draft_label {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera-draft-label")]
    pub struct SetCameraDraftLabel {
        pub value: String,
    }

    pub fn handle(payload: &SetCameraDraftLabel, _doc: &DocumentView<'_, ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigOperation::SetCameraDraftLabel { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetCameraDraftLabel

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: ShootingCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &DocumentView<'_, ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigOperation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn save_and_load_camera_round_trip() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetCameraDraftLabel(set_camera_draft_label::SetCameraDraftLabel { value: "Hero".into() }));
        let result = dispatch(&mut app, ShootingCommand::SaveCamera(save_camera::SaveCamera {}));
        assert_eq!(result.operations.len(), 1);
    }

    #[test]
    fn set_camera_never_touches_the_document() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::SetCamera(set_camera::SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } }));
        assert!(result.operations.is_empty(), "the free/live camera is config-only");
    }
}
//#endregion 🧪️Tests
