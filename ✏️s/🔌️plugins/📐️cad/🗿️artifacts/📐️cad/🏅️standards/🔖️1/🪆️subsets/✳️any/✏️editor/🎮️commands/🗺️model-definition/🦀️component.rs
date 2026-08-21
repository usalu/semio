//! 🗺️ CAD play app commands — which model definition the document is focused on, and which bundled example is loaded.

use crate::artifacts::cad::mutations::change_active_model_definition::mutation::ChangeActiveModelDefinition;
use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_camera, forest_play_scene, CAD_EXAMPLE_FOREST_LEFT};
use crate::artifacts::cad::CadSnapshot;
use crate::editor::cad::config::{CadConfig, CadConfigMutation};
use crate::editor::cad::CadDispatchCtx;
use crate::editor::cad::{reset_document_effect, runtime_of, snapshot_of, CadPlayRuntime};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️FocusModelDefinition
pub mod focus_model_definition {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "focus-model-definition")]
    pub struct FocusModelDefinition {
        pub model_definition_id: String,
    }

    pub async fn handle(payload: &FocusModelDefinition, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![CadMutation::ChangeActiveModelDefinition(ChangeActiveModelDefinition { new_model_definition_id: payload.model_definition_id.clone() })]))
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

    pub async fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx<'_>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
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
        let mut emit = Emit { effects: vec![reset_document_effect(&scene)], ..Default::default() };
        emit.config_mutations = vec![snapshot_of(&runtime, cfg.snapshot)];
        Ok(emit)
    }
}
//#endregion 🔖️SetActiveExample
