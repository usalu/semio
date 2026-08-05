//! 🗺️ CAD play app commands — which model definition the document is focused on, and which bundled example is loaded.

use crate::apps::cad::config::{CadConfig, CadConfigOperation};
use crate::apps::cad::CadDispatchCtx;
use crate::artifacts::cad::op::CadOperation;
use crate::artifacts::cad::CadScene;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::apps::cad::{runtime_of, snapshot_of, CadPlayRuntime};
use crate::artifacts::cad::engine::{default_document, forest_play_camera, forest_play_scene, CAD_EXAMPLE_FOREST_LEFT};


//#region 🔖️FocusModelDefinition
pub mod focus_model_definition {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "focus-model-definition")]
    pub struct FocusModelDefinition {
        pub model_definition_id: String,
    }

    pub fn handle(payload: &FocusModelDefinition, _doc: &DocumentView<'_, CadScene>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        Ok(Emit::operations(vec![CadOperation::SetActiveModelDefinition { model_definition_id: payload.model_definition_id.clone() }]))
    }
}
//#endregion 🔖️FocusModelDefinition

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        let _ = runtime_of(cfg);
        let (scene, runtime) = if payload.example_id.is_empty() {
            (default_document(), CadPlayRuntime::default())
        } else if payload.example_id == CAD_EXAMPLE_FOREST_LEFT || payload.example_id == "forest-left" {
            let forest_camera = forest_play_camera();
            (
                forest_play_scene(),
                CadPlayRuntime {
                    active_example_id: Some(CAD_EXAMPLE_FOREST_LEFT.into()),
                    camera: forest_camera.clone(),
                    camera_building: forest_camera.clone(),
                    camera_energy: forest_camera.clone(),
                    camera_structure_classic: forest_camera,
                    ..CadPlayRuntime::default()
                },
            )
        } else {
            return Ok(Emit::default());
        };
        let mut emit = Emit::operations(vec![CadOperation::SetScene { scene: Box::new(scene) }]);
        emit.config_operations = vec![snapshot_of(&runtime, cfg.projection)];
        Ok(emit)
    }
}
//#endregion 🔖️SetActiveExample
