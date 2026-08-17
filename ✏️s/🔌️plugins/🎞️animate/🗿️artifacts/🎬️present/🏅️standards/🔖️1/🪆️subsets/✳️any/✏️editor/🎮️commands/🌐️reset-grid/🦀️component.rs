//! 🌐️ 🌐️ Animate present app commands command — `reset-grid`.

use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentDispatchCtx};
use crate::artifacts::present::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::present::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🎛️ App-scope command addressed locally as `"resetGrid"`; the owner is carried separately.
/// Its manifest command id (`resetGrid`) diverges from what the wire keyword (`reset-grid`)
/// would suggest, which is exactly what `app_commands!`'s `"id" as "wire-key"` two-literal row
/// exists for — see `crate::editor::animate`'s invocation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "reset-grid")]
pub struct ResetGrid {}

pub fn handle(_payload: &ResetGrid, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (deck_source, _) = crate::artifacts::present::present_working_scene(deck);
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck_source, rows: 3, columns: 5, gap: 0.0, key_prefix: "tile" });
    let selected: Vec<String> = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
    let mut emit = Emit::mutations(vec![PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })]);
    emit.effects.push(interaction_select_effect(&selected, "replace"));
    Ok(emit)
}
