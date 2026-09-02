//! 🎥️ CAD play app commands — the per-pane camera pose and its classical-projection configuration. All three are config-only: a camera move never records a VCS edit.

use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{cad_camera_distance, cad_camera_projection_config, cad_camera_set_projection_config};
use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::{CadCamera, CadPaneId};
use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{cad_pane_camera_runtime, cad_pane_camera_runtime_mut, cad_pane_id_from_surface_id, cad_pane_suffix, runtime_of, snapshot_of};
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use protocol::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetCamera
pub mod set_camera {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        pub pane: Option<String>,
        #[dsl(block)]
        pub camera: CadCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        // 🎥️ `pane` carries the FULL `surfaceId` (`"cad.play.scene3d/building"`), not a bare
        // pane suffix — mirrors the pre-B1 `args.get("surfaceId")` resolution exactly.
        let mut runtime = runtime_of(cfg);
        let pane = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_surface_id);
        *cad_pane_camera_runtime_mut(&mut runtime, pane) = payload.camera.clone();
        Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)?], format!("camera:{}", cad_pane_suffix(pane))))
    }
}
//#endregion 🔖️SetCamera

//#region 🔖️SetProjection
pub mod set_projection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "projection")]
    pub struct SetProjection {
        pub pane: Option<String>,
        pub field: Option<String>,
        pub value_str: Option<String>,
        pub value_num: Option<f64>,
        pub param: Option<String>,
    }

    pub fn handle(payload: &SetProjection, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        // 🎥️ `pane` carries the full `surfaceId` — see `SetCamera`'s doc comment.
        let mut runtime = runtime_of(cfg);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_surface_id);
        let mut camera = cad_pane_camera_runtime(&runtime, pane_id).clone();
        let mut projection_config = cad_camera_projection_config(&camera);
        // 🌉️ `world3d_projection_action_moves_pose`/`apply_world3d_projection_action` (framework
        // `🔌️plugin/🦀️.rs`) still take `Option<&serde_json::Value>` — a genuine framework
        // boundary, bridged once here.
        let value = payload.value_str.clone().map(DslValue::String).or_else(|| payload.value_num.map(DslValue::float)).unwrap_or(DslValue::Null);
        let dsl_args = DslValue::object([
            ("field".to_string(), payload.field.clone().map(DslValue::String).unwrap_or(DslValue::Null)),
            ("value".to_string(), value),
            ("param".to_string(), payload.param.clone().map(DslValue::String).unwrap_or(DslValue::Null)),
        ]);
        let args_value = serde_json::Value::from(&dsl_args);
        let args = Some(&args_value);
        let moves_pose = world3d_projection_action_moves_pose("setProjection", args);
        apply_world3d_projection_action(&mut projection_config, "setProjection", args);
        if moves_pose {
            let (position, _up) = world3d_projection_pose(&projection_config, camera.target, cad_camera_distance(&camera));
            camera.position = position;
        }
        cad_camera_set_projection_config(&mut camera, &projection_config);
        *cad_pane_camera_runtime_mut(&mut runtime, pane_id) = camera;
        Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)?], format!("projection:{}", cad_pane_suffix(pane_id))))
    }
}
//#endregion 🔖️SetProjection

//#region 🔖️SetProjectionParam
pub mod set_projection_param {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "projection-param")]
    pub struct SetProjectionParam {
        pub pane: Option<String>,
        pub field: Option<String>,
        pub value_str: Option<String>,
        pub value_num: Option<f64>,
        pub param: Option<String>,
    }

    pub fn handle(payload: &SetProjectionParam, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        // 🎥️ `pane` carries the full `surfaceId` — see `SetCamera`'s doc comment.
        let mut runtime = runtime_of(cfg);
        let pane_id = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_surface_id);
        let mut camera = cad_pane_camera_runtime(&runtime, pane_id).clone();
        let mut projection_config = cad_camera_projection_config(&camera);
        // 🌉️ `world3d_projection_action_moves_pose`/`apply_world3d_projection_action` (framework
        // `🔌️plugin/🦀️.rs`) still take `Option<&serde_json::Value>` — a genuine framework
        // boundary, bridged once here.
        let value = payload.value_str.clone().map(DslValue::String).or_else(|| payload.value_num.map(DslValue::float)).unwrap_or(DslValue::Null);
        let dsl_args = DslValue::object([
            ("field".to_string(), payload.field.clone().map(DslValue::String).unwrap_or(DslValue::Null)),
            ("value".to_string(), value),
            ("param".to_string(), payload.param.clone().map(DslValue::String).unwrap_or(DslValue::Null)),
        ]);
        let args_value = serde_json::Value::from(&dsl_args);
        let args = Some(&args_value);
        let moves_pose = world3d_projection_action_moves_pose("setProjectionParam", args);
        apply_world3d_projection_action(&mut projection_config, "setProjectionParam", args);
        if moves_pose {
            let (position, _up) = world3d_projection_pose(&projection_config, camera.target, cad_camera_distance(&camera));
            camera.position = position;
        }
        cad_camera_set_projection_config(&mut camera, &projection_config);
        *cad_pane_camera_runtime_mut(&mut runtime, pane_id) = camera;
        Ok(Emit::amend_config(vec![snapshot_of(&runtime, cfg.snapshot)?], format!("projection:{}", cad_pane_suffix(pane_id))))
    }
}
//#endregion 🔖️SetProjectionParam
