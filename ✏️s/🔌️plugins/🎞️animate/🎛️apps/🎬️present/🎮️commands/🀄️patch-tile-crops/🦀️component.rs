//! 🀄️ 🀄️ Animate present app commands command — `patch-tile-crops`.

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
#[dsl(keyword = "patch-tile-crops")]
pub struct PatchTileCrops {
    pub ids: Vec<String>,
    pub field: String,
    pub value: f64,
}

pub fn handle(payload: &PatchTileCrops, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (_, deck_tiles) = crate::artifacts::present::present_working_scene(deck);
    let targets: HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
    let operations: Vec<PresentMutation> = deck_tiles
        .iter()
        .filter(|tile| targets.contains(tile.id.as_str()))
        .map(|tile| {
            let mut crop = tile.crop.clone();
            match payload.field.as_str() {
                "x" => crop.x = payload.value,
                "y" => crop.y = payload.value,
                "width" => crop.width = payload.value,
                "height" => crop.height = payload.value,
                _ => {}
            }
            PresentMutation::ResizeTileCrop(ResizeTileCrop { id: tile.id.clone(), new_crop: clamp_tile_crop(&crop) })
        })
        .collect();
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}
