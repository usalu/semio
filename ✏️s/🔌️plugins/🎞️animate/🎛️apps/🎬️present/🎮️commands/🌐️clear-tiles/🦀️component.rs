//! 🌐️ 🌐️ Animate present app commands command — `clear-tiles`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::{interaction_select_effect, PresentDispatchCtx};
use crate::artifacts::present::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "clear-tiles")]
pub struct ClearTiles {}

pub fn handle(_payload: &ClearTiles, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let mut emit = Emit::mutations(vec![PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: Vec::new() })]);
    emit.effects.push(interaction_select_effect(&[], "replace"));
    Ok(emit)
}
