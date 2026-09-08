//! ⌨️ ⌨️ Animate presentation app commands command — `engagement-submit`.

#![allow(clippy::result_large_err)]

use crate::mutations::create_tile::CreateTile;
use crate::mutations::replace_tiles::ReplaceTiles;
use crate::op::PresentationMutation;
use crate::standards::v1::subsets::any::schema::{parse_grid_engagement, populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::{FigureTileDraft, FigureTileFrame, PresentationSnapshot};
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, new_tile_id, tile_morph_prompt_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: String,
}

pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let trimmed = payload.value.trim();
    let (deck_source, deck_tiles) = crate::presentation_working_scene(deck);
    if let Some((rows, columns)) = parse_grid_engagement(trimmed) {
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck_source, rows, columns, gap: 0.0, key_prefix: "tile" });
        let selected: Vec<String> = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
        return Ok(Emit {
            artifact_mutations: vec![PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })],
            config_mutations: vec![PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: String::new() })],
            effects: vec![interaction_select_effect(&selected, "replace")],
            ..Default::default()
        });
    }
    match trimmed.to_lowercase().as_str() {
        "add" => {
            let id = new_tile_id("tile");
            let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
            Ok(Emit {
                artifact_mutations: vec![PresentationMutation::CreateTile(CreateTile { index: deck_tiles.len(), tile })],
                config_mutations: vec![PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: String::new() })],
                effects: vec![interaction_select_effect(&[id], "replace")],
                ..Default::default()
            })
        }
        "clear" => Ok(Emit {
            artifact_mutations: vec![PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: Vec::new() })],
            config_mutations: vec![PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: String::new() })],
            effects: vec![interaction_select_effect(&[], "replace")],
            ..Default::default()
        }),
        "copy" | "copy prompt" => Ok(Emit { config_mutations: vec![PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: String::new() })], effects: vec![tile_morph_prompt_effect(deck)], ..Default::default() }),
        _ => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
