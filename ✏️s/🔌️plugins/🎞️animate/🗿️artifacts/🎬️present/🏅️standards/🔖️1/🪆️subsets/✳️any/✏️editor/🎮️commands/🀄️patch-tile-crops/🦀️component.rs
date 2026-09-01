//! 🀄️ 🀄️ Animate present app commands command — `patch-tile-crops`.

#![allow(clippy::result_large_err)]

use crate::artifacts::present::mutations::resize_tile_crop::mutation::ResizeTileCrop;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::schema::clamp_tile_crop;
use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::PresentDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use std::collections::HashSet;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-tile-crops")]
pub struct PatchTileCrops {
    pub ids: Vec<String>,
    pub field: String,
    pub value: f64,
}

pub fn handle(payload: &PatchTileCrops, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
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
