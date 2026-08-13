//! ⌨️ ⌨️ Animate present app commands command — `engagement-input`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::{new_tile_id, tile_morph_prompt_effect};
use crate::artifacts::present::schema::{parse_grid_engagement, populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{FigureTileDraft, FigureTileFrame, PresentSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-input")]
pub struct EngagementInput {
    pub value: String,
}

pub fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::config(vec![PresentConfigMutation::SetEngagementInput { value: payload.value.clone() }]))
}
