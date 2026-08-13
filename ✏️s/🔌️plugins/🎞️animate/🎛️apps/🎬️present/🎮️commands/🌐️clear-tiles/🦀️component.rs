//! 🌐️ 🌐️ Animate present app commands command — `clear-tiles`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::artifacts::present::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::present::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "clear-tiles")]
pub struct ClearTiles {}

pub fn handle(_payload: &ClearTiles, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit { artifact_mutations: vec![PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: Vec::new() })], config_mutations: vec![PresentConfigMutation::SetSelectedIds { ids: Vec::new() }], ..Default::default() })
}
