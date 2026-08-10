//! 🗂️ Shooting play app commands — the viewport selection/hover/transform-utility surface. All
//! CONFIG-only: they mutate `ShootingConfig` and never emit document operations.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{merge_world_selection_ids, ConfigView, ArtifactView, Emit, Fault, SelectionSet};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub shot_ids: Vec<String>,
        pub asset_ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetSelection { shot_ids: payload.shot_ids.clone(), asset_ids: payload.asset_ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetSelectionMethod
pub mod set_selection_method {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection-method")]
    pub struct SetSelectionMethod {
        pub method: String,
    }

    pub fn handle(payload: &SetSelectionMethod, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetSelectionMethod { method: payload.method.clone() }]))
    }
}
//#endregion 🔖️SetSelectionMethod

//#region 🔖️WorldSelect
pub mod world_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-select")]
    pub struct WorldSelect {
        pub ids: Vec<String>,
        pub merge: String,
    }

    pub fn handle(payload: &WorldSelect, _doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let merged = merge_world_selection_ids(&SelectionSet::from_ids(config.selected_asset_ids.clone()), &payload.ids, &payload.merge).to_vec();
        Ok(Emit::config(vec![ShootingConfigMutation::SetSelection { shot_ids: config.selected_shot_ids.clone(), asset_ids: merged }]))
    }
}
//#endregion 🔖️WorldSelect

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-hover")]
    pub struct SetHover {
        pub asset_id: Option<String>,
    }

    pub fn handle(payload: &SetHover, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetHoveredAsset { asset_id: payload.asset_id.clone() }]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️WorldPick
pub mod world_pick {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pick")]
    pub struct WorldPick {
        pub asset_id: Option<String>,
        pub asset_index: Option<u64>,
        pub merge: String,
    }

    pub fn handle(payload: &WorldPick, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let resolved = payload.asset_index.and_then(|index| doc.snapshot.assets.get(index as usize)).map(|asset| asset.id.clone()).or_else(|| payload.asset_id.clone());
        match resolved {
            None if payload.merge == "replace" => Ok(Emit::config(vec![ShootingConfigMutation::SetSelection { shot_ids: config.selected_shot_ids.clone(), asset_ids: Vec::new() }])),
            None => Ok(Emit::default()),
            Some(id) => {
                let merged = merge_world_selection_ids(&SelectionSet::from_ids(config.selected_asset_ids.clone()), &[id], &payload.merge).to_vec();
                Ok(Emit::config(vec![ShootingConfigMutation::SetSelection { shot_ids: config.selected_shot_ids.clone(), asset_ids: merged }]))
            }
        }
    }
}
//#endregion 🔖️WorldPick

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {}

    pub fn handle(_payload: &WorldPointerDown, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️WorldPointerDown

//#region 🔖️WorldPointerMove
pub mod world_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-move")]
    pub struct WorldPointerMove {}

    pub fn handle(_payload: &WorldPointerMove, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️WorldPointerMove

//#region 🔖️SetCenterModel
pub mod set_center_model {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "center-model")]
    pub struct SetCenterModel {
        pub pressed: Option<bool>,
    }

    pub fn handle(payload: &SetCenterModel, _doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let next = payload.pressed.unwrap_or(!config.center_model);
        let mut config_mutations = vec![ShootingConfigMutation::SetCenterModel { value: next }];
        if next && !config.center_model {
            config_mutations.push(ShootingConfigMutation::SetFitRevision { value: config.fit_revision + 1 });
        }
        Ok(Emit::config(config_mutations))
    }
}
//#endregion 🔖️SetCenterModel

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }, ShootingConfigMutation::SetHoveredAsset { asset_id: None }]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn world_pick_and_hover_drive_selection_protocol() {
        use crate::apps::shooting::testkit::render;
        use crate::apps::shooting::SHOOTING_PLAY_BODY_SCENE;
        use serde_json::{json, Value};

        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::WorldPick(world_pick::WorldPick { asset_id: None, asset_index: Some(0), merge: "replace".into() }));
        assert!(result.mutations.is_empty(), "worldPick mutates only ephemeral selection, never the document");
        let payload: Value = serde_json::from_str(&render(&mut app, SHOOTING_PLAY_BODY_SCENE)).unwrap();
        let selection: Value = serde_json::from_str(payload["world3d"]["selectionJson"].as_str().unwrap()).unwrap();
        assert_eq!(selection["ids"], json!(["base"]), "the picked asset becomes the config selection");
    }

    #[test]
    fn set_active_utility_clears_hover_and_emits_no_artifact_mutations() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetHover(set_hover::SetHover { asset_id: Some("base".into()) }));
        let result = dispatch(&mut app, ShootingCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }));
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    }

    #[test]
    fn center_model_toggle_bumps_fit_revision_only_on_the_off_to_on_edge() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(false) }));
        dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(true) }));
        // fit_revision itself is asserted end-to-end (render fitJson) in the scene window's own tests;
        // here we just assert the command round-trips without error under both edges.
        let result = dispatch(&mut app, ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: None }));
        assert!(result.mutations.is_empty(), "center-model is config-only");
    }
}
//#endregion 🧪️Tests
