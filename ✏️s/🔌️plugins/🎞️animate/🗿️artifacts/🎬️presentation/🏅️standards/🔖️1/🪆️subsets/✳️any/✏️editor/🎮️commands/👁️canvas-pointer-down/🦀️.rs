//! 👁️ 👁️ Animate presentation app commands command — `canvas-pointer-down`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub layer_id: Option<String>,
}

/// 🕹️ Hit-testing stays here (the canvas surface is the only thing that knows which layer a click
/// landed on); the resulting selection is applied through the framework's `interactionSelect` verb,
/// never a `PresentationConfigMutation`, now that selection is framework-owned state (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (_, deck_tiles) = crate::artifacts::presentation::presentation_working_scene(deck);
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
    use crate::editor::animate::testkit::{dispatch, presentation_app_with_registry};
    use crate::editor::animate::{commands::add_tile, PresentationCommand};
    use semio_framework_plugin::Effect;

    #[semio_framework_async_macros::async_test]
    async fn canvas_pointer_down_emits_interaction_select_for_a_hit_and_clears_on_miss() {
        let mut app = presentation_app_with_registry().await;
        dispatch(&mut app, PresentationCommand::AddTile(add_tile::AddTile { crop: None })).await;
        let tile_id = crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection")).1[0].id.clone();

        let hit = dispatch(&mut app, PresentationCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some(tile_id) })).await;
        assert!(matches!(hit.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));

        let miss = dispatch(&mut app, PresentationCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some("source-frame".into()) })).await;
        assert!(matches!(miss.requested_effects.as_slice(), [Effect::ReplayShellCommand { action_id, .. }] if action_id == semio_framework::INTERACTION_SELECT_ACTION_ID));
    }
}
//#endregion 🧪️Tests
