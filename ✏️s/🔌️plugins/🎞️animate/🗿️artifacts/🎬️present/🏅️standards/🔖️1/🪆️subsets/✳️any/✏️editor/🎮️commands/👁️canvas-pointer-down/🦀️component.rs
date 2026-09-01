//! 👁️ 👁️ Animate present app commands command — `canvas-pointer-down`.

#![allow(clippy::result_large_err)]

use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub layer_id: Option<String>,
}

/// 🕹️ Hit-testing stays here (the canvas surface is the only thing that knows which layer a click
/// landed on); the resulting selection is applied through the framework's `interactionSelect` verb,
/// never a `PresentConfigMutation`, now that selection is framework-owned state (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (_, deck_tiles) = crate::artifacts::present::present_working_scene(deck);
    let ids: Vec<String> = match &payload.layer_id {
        Some(id) if deck_tiles.iter().any(|tile| &tile.id == id) => vec![id.clone()],
        _ => Vec::new(),
    };
    Ok(Emit { effects: vec![interaction_select_effect(&ids, "replace")], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{dispatch, present_app_with_registry};
    use crate::editor::animate::{commands::add_tile, PresentCommand};
    use semio_framework_plugin::Effect;

    #[semio_framework_async_macros::async_test]
    async fn canvas_pointer_down_emits_interaction_select_for_a_hit_and_clears_on_miss() {
        let mut app = present_app_with_registry().await;
        dispatch(&mut app, PresentCommand::AddTile(add_tile::AddTile { crop: None })).await;
        let tile_id = crate::artifacts::present::present_working_scene(&app.snapshot().await.expect("projection")).1[0].id.clone();

        let hit = dispatch(&mut app, PresentCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some(tile_id) })).await;
        assert!(matches!(hit.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));

        let miss = dispatch(&mut app, PresentCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some("source-frame".into()) })).await;
        assert!(matches!(miss.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));
    }
}
//#endregion 🧪️Tests
