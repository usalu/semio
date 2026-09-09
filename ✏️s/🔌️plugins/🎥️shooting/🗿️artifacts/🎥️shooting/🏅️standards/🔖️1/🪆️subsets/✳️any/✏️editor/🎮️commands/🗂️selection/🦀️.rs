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
use crate::op::ShootingMutation;
use crate::ShootingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetShotSelection
pub mod set_shot_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-shot-selection")]
    pub struct SetShotSelection {
        pub shot_ids: Vec<String>,
    }

    pub fn handle(payload: &SetShotSelection, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::config(vec![ShootingConfigMutation::SetShotSelection(crate::editor::shooting::config::SetShotSelection { shot_ids: payload.shot_ids.clone() })]))
    }
}
//#endregion 🔖️SetShotSelection

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {}

    pub fn handle(_payload: &WorldPointerDown, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️WorldPointerDown

//#region 🔖️WorldPointerMove
pub mod world_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-move")]
    pub struct WorldPointerMove {}

    pub fn handle(_payload: &WorldPointerMove, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️WorldPointerMove

//#region 🔖️SetCenterModel
pub mod set_center_model {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "center-model")]
    pub struct SetCenterModel {
        pub pressed: Option<bool>,
    }

    pub fn handle(payload: &SetCenterModel, _doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let next = payload.pressed.unwrap_or(!config.center_model);
        let mut config_mutations = vec![ShootingConfigMutation::SetCenterModel(crate::editor::shooting::config::SetCenterModel { value: next })];
        if next && !config.center_model {
            config_mutations.push(ShootingConfigMutation::SetFitRevision(crate::editor::shooting::config::SetFitRevision { value: config.fit_revision + 1 }));
        }
        Ok(Emit::config(config_mutations))
    }
}
//#endregion 🔖️SetCenterModel

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
