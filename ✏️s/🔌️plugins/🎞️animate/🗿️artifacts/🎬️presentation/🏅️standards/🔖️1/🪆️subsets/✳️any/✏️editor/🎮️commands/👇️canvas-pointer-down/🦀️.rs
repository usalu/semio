//! 👁️ 👁️ Animate presentation app commands command — `canvas-pointer-down`.

#![allow(clippy::result_large_err)]

use crate::op::PresentationMutation;
use crate::PresentationSnapshot;
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
    let (_, deck_tiles) = crate::presentation_working_scene(deck);
    let ids: Vec<String> = match &payload.layer_id {
        Some(id) if deck_tiles.iter().any(|tile| &tile.id == id) => vec![id.clone()],
        _ => Vec::new(),
    };
    Ok(Emit { effects: vec![interaction_select_effect(&ids, "replace")], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
