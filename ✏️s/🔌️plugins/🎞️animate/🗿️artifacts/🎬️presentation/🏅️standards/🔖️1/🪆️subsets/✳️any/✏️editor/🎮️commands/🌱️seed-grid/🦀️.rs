//! 🌐️ 🌐️ Animate presentation app commands command — `seed-grid`.

#![allow(clippy::result_large_err)]

use crate::mutations::replace_tiles::ReplaceTiles;
use crate::op::PresentationMutation;
use crate::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "seed-grid")]
pub struct SeedGrid {
    pub rows: u32,
    pub columns: u32,
}

pub fn handle(payload: &SeedGrid, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (deck_source, _) = crate::presentation_working_scene(deck);
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck_source, rows: payload.rows, columns: payload.columns, gap: 0.0, key_prefix: "tile" });
    let selected: Vec<String> = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
    let mut emit = Emit::mutations(vec![PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })]);
    emit.effects.push(interaction_select_effect(&selected, "replace"));
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
