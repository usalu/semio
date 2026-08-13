//! 🀄️ 🀄️ Animate present app commands command — `rename-tiles`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::{new_tile_id, valid_tile_ids};
use crate::artifacts::present::schema::clamp_tile_crop;
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::mutations::delete_tile::mutation::DeleteTile as DeleteTileMutation;
use crate::artifacts::present::mutations::delete_tiles::mutation::DeleteTiles;
use crate::artifacts::present::mutations::rename_tile::mutation::RenameTile;
use crate::artifacts::present::mutations::resize_tile_crop::mutation::ResizeTileCrop;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{FigureTileDraft, FigureTileFrame, PresentSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "rename-tiles")]
pub struct RenameTiles {
    pub ids: Vec<String>,
    pub value: String,
}

pub fn handle(payload: &RenameTiles, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let name = payload.value.trim();
    if name.is_empty() {
        return Ok(Emit::default());
    }
    let valid = valid_tile_ids(deck, payload.ids.clone());
    if valid.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(valid.into_iter().map(|id| PresentMutation::RenameTile(RenameTile { id, new_name: name.to_string() })).collect()))
}
