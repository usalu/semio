//! 🌐️ 🌐️ Animate presentation app commands command — `clear-tiles`.

#![allow(clippy::result_large_err)]

use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentationDispatchCtx};
use crate::mutations::replace_tiles::ReplaceTiles;
use crate::op::PresentationMutation;
use crate::PresentationSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "clear-tiles")]
pub struct ClearTiles {}

pub fn handle(_payload: &ClearTiles, _doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let mut emit = Emit::mutations(vec![PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: Vec::new() })]);
    emit.effects.push(interaction_select_effect(&[], "replace"));
    Ok(emit)
}
