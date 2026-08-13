//! 🀄️ 🀄️ Animate present app commands command — `delete-tile`.

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
#[dsl(keyword = "delete-tile")]
pub struct DeleteTile {
    pub id: String,
}

pub fn handle(payload: &DeleteTile, doc: &ArtifactView<'_, PresentSnapshot>, cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let config = cfg.snapshot;
    let targets = valid_tile_ids(deck, vec![payload.id.clone()]);
    if targets.is_empty() {
        return Ok(Emit::default());
    }
    let remaining: Vec<String> = config.selected_ids.iter().filter(|selected| !targets.contains(selected)).cloned().collect();
    Ok(Emit {
        artifact_mutations: targets.into_iter().map(|id| PresentMutation::DeleteTile(DeleteTileMutation { id })).collect(),
        config_mutations: vec![PresentConfigMutation::SetSelectedIds { ids: remaining }],
        ..Default::default()
    })
}
