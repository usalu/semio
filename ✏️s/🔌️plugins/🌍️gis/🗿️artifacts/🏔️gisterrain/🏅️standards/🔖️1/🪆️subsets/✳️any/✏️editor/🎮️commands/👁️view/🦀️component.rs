//! 👁️ GIS 3D play app command — the free/live viewport camera. Config-only: it emits
//! `config_mutations`, never document operations.

use crate::editor::gis3d::config::{Gis3dConfig, Gis3dConfigMutation};
use crate::artifacts::gisterrain::op::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub camera_json: String,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis3dConfigMutation::SetCamera { camera_json: payload.camera_json.clone() }]))
    }
}
//#endregion 🔖️SetCamera

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis3d::testkit::{app, dispatch};
    use crate::editor::gis3d::Gis3dCommand;
    use serde_json::json;

    #[test]
    fn camera_is_config_state_and_emits_no_operations() {
        let mut app = app();
        let camera = dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: json!({ "position": [1.0, 1.0, 1.0] }).to_string() }));
        assert!(camera.mutations.is_empty(), "camera is ephemeral config state");
    }

    #[test]
    fn the_camera_reaches_the_rendered_scene() {
        let mut app = app();
        dispatch(&mut app, Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: json!({ "position": [123.0, 1.0, 1.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string() }));
        assert!(crate::editor::gis3d::testkit::render(&mut app, crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE).contains("123"));
    }
}
//#endregion 🧪️Tests
