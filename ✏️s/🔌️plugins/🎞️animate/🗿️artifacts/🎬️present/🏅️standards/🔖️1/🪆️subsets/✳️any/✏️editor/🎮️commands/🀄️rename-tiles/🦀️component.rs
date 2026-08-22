//! 🀄️ 🀄️ Animate present app commands command — `rename-tiles`.

#![allow(clippy::result_large_err)]

use crate::artifacts::present::mutations::rename_tile::mutation::RenameTile;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::{valid_tile_ids, PresentDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "rename-tiles")]
pub struct RenameTiles {
    pub ids: Vec<String>,
    pub value: String,
}

pub fn handle(payload: &RenameTiles, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
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
