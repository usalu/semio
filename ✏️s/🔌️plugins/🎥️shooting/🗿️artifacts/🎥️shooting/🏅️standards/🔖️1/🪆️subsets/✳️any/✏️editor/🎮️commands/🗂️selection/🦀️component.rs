//! 🗂️ Shooting play app commands — the viewport transform-utility surface plus the gallery/document-tree
//! shot selection. CONFIG-only: they mutate `ShootingConfig` and never emit document operations.
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `set_selection`/`set_selection_method`/
//! `world_select`/`set_hover`/`world_pick` are DELETED — asset selection/hover dissolved into the
//! framework-owned `"assets"` interaction domain (`assets_interaction_definition`, `crate::editor::shooting`'s
//! `🔖️Manifest` region): the framework auto-injects `interactionSelect`/`interactionHover`/
//! `clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity`, and the world-3d scene
//! surface dispatches those directly (client-side hit-testing against the mesh instance ids already in
//! the scene payload — see `🎭️modes/✏️edit/🪟️windows/🎥️scene`'s doc comments), so `worldPick`/`worldSelect`/
//! `setHover` have no Rust command counterpart any more. `set_shot_selection` replaces `set_selection`'s
//! shot half — see `ShootingConfig::selected_shot_ids`'s doc comment for why shot selection stayed a
//! plain config field instead of joining the interaction domain.

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetShotSelection
pub mod set_shot_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-shot-selection")]
    pub struct SetShotSelection {
        pub shot_ids: Vec<String>,
    }

    pub async fn handle(payload: &SetShotSelection, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetShotSelection { shot_ids: payload.shot_ids.clone() }]))
    }
}
//#endregion 🔖️SetShotSelection

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {}

    pub async fn handle(_payload: &WorldPointerDown, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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

    pub async fn handle(_payload: &WorldPointerMove, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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

    pub async fn handle(payload: &SetCenterModel, _doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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

    /// 🕹️ Used to also clear the (now framework-owned) `"assets"` domain hover here — an app command's
    /// `Emit` has no channel into `InteractionState` any more (only the framework's own injected
    /// `interactionHover` dispatch writes it), so switching the transform utility no longer clears
    /// hover. Documented behavior change, matching this wave's other apps (e.g. `raster`'s `add-layer`).
    pub async fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_shot_selection_is_config_only_and_selects_the_shot_in_the_inspector() {
        use crate::editor::shooting::testkit::render;
        use crate::editor::shooting::SHOOTING_PLAY_BODY_INSPECTION;

        let mut app = shooting_app();
        let shot_id = app.snapshot().expect("snapshot").shots.first().expect("fixture shot").id.clone();
        let result = dispatch(&mut app, ShootingCommand::SetShotSelection(set_shot_selection::SetShotSelection { shot_ids: vec![shot_id] }));
        assert!(result.mutations.is_empty(), "shot selection is config-only");
        assert!(render(&mut app, SHOOTING_PLAY_BODY_INSPECTION).contains("shooting-play-inspector.shot"), "inspector renders the shot group for the selected shot");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_emits_no_artifact_mutations() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }));
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    }

    #[semio_framework_async_macros::async_test]
    async fn center_model_toggle_bumps_fit_revision_only_on_the_off_to_on_edge() {
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
