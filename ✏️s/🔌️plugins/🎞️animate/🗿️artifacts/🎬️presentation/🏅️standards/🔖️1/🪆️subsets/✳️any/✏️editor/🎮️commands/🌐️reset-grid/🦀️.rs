//! 🌐️ 🌐️ Animate presentation app commands command — `reset-grid`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎛️ App-scope command addressed locally as `"resetGrid"`; the owner is carried separately.
/// Its manifest command id (`resetGrid`) diverges from what the wire keyword (`reset-grid`)
/// would suggest, which is exactly what `app_commands!`'s `"id" as "wire-key"` two-literal row
/// exists for — see `crate::editor::animate`'s invocation.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "reset-grid")]
pub struct ResetGrid {}

pub fn handle(_payload: &ResetGrid, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (deck_source, _) = crate::artifacts::presentation::presentation_working_scene(deck);
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck_source, rows: 3, columns: 5, gap: 0.0, key_prefix: "tile" });
    let selected: Vec<String> = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
    let mut emit = Emit::mutations(vec![PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })]);
    emit.effects.push(interaction_select_effect(&selected, "replace"));
    Ok(emit)
}
