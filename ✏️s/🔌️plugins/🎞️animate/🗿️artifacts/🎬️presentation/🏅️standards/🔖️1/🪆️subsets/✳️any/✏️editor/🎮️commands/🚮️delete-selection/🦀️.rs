//! 🀄️ 🀄️ Animate presentation app commands command — `delete-selection`.

#![allow(clippy::result_large_err)]

use crate::mutations::delete_tiles::DeleteTiles;
use crate::op::PresentationMutation;
use crate::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{valid_tile_ids, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ Reads the `tiles` domain's current selection (`ctx.selected_ids`, resolved once by
/// `ArtifactApp::handle` from `InteractionView`) instead of a deleted config field — no config
/// mutation needed afterwards: `tiles` is declared `HierarchyProvider::Flat`, so the framework never
/// auto-prunes a Flat domain's selection (see the plugin SDK's `validate_state` doc); a deleted tile's
/// stale id simply stays selected until the next real pick, a documented, accepted gap matching
/// `🖍️draw`'s `delete-layer`.
pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let targets = valid_tile_ids(deck, ctx.selected_ids.clone());
    if targets.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![PresentationMutation::DeleteTiles(DeleteTiles { ids: targets })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
