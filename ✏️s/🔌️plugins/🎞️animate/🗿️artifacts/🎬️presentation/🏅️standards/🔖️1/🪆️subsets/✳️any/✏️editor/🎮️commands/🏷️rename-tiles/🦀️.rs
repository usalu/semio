//! 🀄️ 🀄️ Animate presentation app commands command — `rename-tiles`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::mutations::rename_tile::mutation::RenameTile;
use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{valid_tile_ids, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "rename-tiles")]
pub struct RenameTiles {
    pub ids: Vec<String>,
    pub value: String,
}

pub fn handle(payload: &RenameTiles, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let name = payload.value.trim();
    if name.is_empty() {
        return Ok(Emit::default());
    }
    let valid = valid_tile_ids(deck, payload.ids.clone());
    if valid.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(valid.into_iter().map(|id| PresentationMutation::RenameTile(RenameTile { id, new_name: name.to_string() })).collect()))
}
