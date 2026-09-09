//! 🀄️ 🀄️ Animate presentation app commands command — `add-tile`.

#![allow(clippy::result_large_err)]

use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{interaction_select_effect, new_tile_id, PresentationDispatchCtx};
use crate::mutations::create_tile::CreateTile;
use crate::op::PresentationMutation;
use crate::{FigureTileDraft, FigureTileFrame, PresentationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-tile")]
pub struct AddTile {
    #[dsl(block)]
    pub crop: Option<FigureTileFrame>,
}

pub fn handle(payload: &AddTile, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let id = new_tile_id("tile");
    let crop = payload.crop.clone().unwrap_or(FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 });
    let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop };
    let tile_count = crate::presentation_working_scene(deck).1.len();
    let mut emit = Emit::mutations(vec![PresentationMutation::CreateTile(CreateTile { index: tile_count, tile })]);
    emit.effects.push(interaction_select_effect(&[id], "replace"));
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
