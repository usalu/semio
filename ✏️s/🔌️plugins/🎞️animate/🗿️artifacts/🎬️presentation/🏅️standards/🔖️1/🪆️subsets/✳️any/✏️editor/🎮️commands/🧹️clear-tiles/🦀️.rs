//! 🌐️ 🌐️ Animate presentation app commands command — `clear-tiles`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, PresentationDispatchCtx};
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
