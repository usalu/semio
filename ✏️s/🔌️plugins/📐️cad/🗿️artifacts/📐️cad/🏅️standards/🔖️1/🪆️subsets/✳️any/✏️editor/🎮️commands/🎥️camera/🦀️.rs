//! 🎥️ CAD world-window camera and projection commands owned by the exact invoking window.

use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::modes::edit::windows::config as window_config;
use crate::editor::cad::cad_pane_id_from_surface_id;
use crate::op::CadMutation;
use crate::standards::v1::subsets::any::schema::inferences::{cad_camera_distance, cad_camera_projection_config, cad_camera_set_projection_config};
use crate::CadSnapshot;
use crate::{CadCamera, CadPaneId};
use protocol::DslValue;
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let _surface = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_surface_id);
        let mut config = window_config::current(cfg);
        config.camera = payload.camera.clone();
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
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

    pub fn handle(payload: &SetProjection, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let _surface = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_surface_id);
        let mut config = window_config::current(cfg);
        let mut camera = config.camera.clone();
        let mut projection_config = cad_camera_projection_config(&camera);
        // 🌉️ `world3d_projection_action_moves_pose`/`apply_world3d_projection_action` (framework
        // `🔌️plugin/🦀️.rs`) take `Option<&dsl::os_pack::json::Value>` — a genuine framework
        // boundary, bridged once here from a `DslValue` built the normal way.
        let value = payload.value_str.clone().map(DslValue::String).or_else(|| payload.value_num.map(DslValue::float)).unwrap_or(DslValue::Null);
        let dsl_args = DslValue::object([("field".to_string(), payload.field.clone().map_or(DslValue::Null, DslValue::String)), ("value".to_string(), value), ("param".to_string(), payload.param.clone().map_or(DslValue::Null, DslValue::String))]);
        let args_value = protocol::json::from_dsl_value(&dsl_args);
        let args = Some(&args_value);
        let moves_pose = world3d_projection_action_moves_pose("setProjection", args);
        apply_world3d_projection_action(&mut projection_config, "setProjection", args);
        if moves_pose {
            let (position, _up) = world3d_projection_pose(&projection_config, camera.target, cad_camera_distance(&camera));
            camera.position = position;
        }
        cad_camera_set_projection_config(&mut camera, &projection_config);
        config.camera = camera;
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
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

    pub fn handle(payload: &SetProjectionParam, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let _surface = payload.pane.as_deref().map_or(CadPaneId::Shape, cad_pane_id_from_surface_id);
        let mut config = window_config::current(cfg);
        let mut camera = config.camera.clone();
        let mut projection_config = cad_camera_projection_config(&camera);
        // 🌉️ `world3d_projection_action_moves_pose`/`apply_world3d_projection_action` (framework
        // `🔌️plugin/🦀️.rs`) take `Option<&dsl::os_pack::json::Value>` — a genuine framework
        // boundary, bridged once here from a `DslValue` built the normal way.
        let value = payload.value_str.clone().map(DslValue::String).or_else(|| payload.value_num.map(DslValue::float)).unwrap_or(DslValue::Null);
        let dsl_args = DslValue::object([("field".to_string(), payload.field.clone().map_or(DslValue::Null, DslValue::String)), ("value".to_string(), value), ("param".to_string(), payload.param.clone().map_or(DslValue::Null, DslValue::String))]);
        let args_value = protocol::json::from_dsl_value(&dsl_args);
        let args = Some(&args_value);
        let moves_pose = world3d_projection_action_moves_pose("setProjectionParam", args);
        apply_world3d_projection_action(&mut projection_config, "setProjectionParam", args);
        if moves_pose {
            let (position, _up) = world3d_projection_pose(&projection_config, camera.target, cad_camera_distance(&camera));
            camera.position = position;
        }
        cad_camera_set_projection_config(&mut camera, &projection_config);
        config.camera = camera;
        Ok(Emit { window_config_mutations: vec![window_config::addressed_from_context(ctx, config)?], ..Default::default() })
    }
}
//#endregion 🔖️SetProjectionParam
