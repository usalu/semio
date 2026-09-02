//! 🀄️ 🀄️ Animate presentation app commands command — `patch-tile-crops`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::mutations::resize_tile_crop::mutation::ResizeTileCrop;
use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::schema::clamp_tile_crop;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::PresentationDispatchCtx;
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

pub fn handle(payload: &PatchTileCrops, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (_, deck_tiles) = crate::artifacts::presentation::presentation_working_scene(deck);
    let targets: HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
    let operations: Vec<PresentationMutation> = deck_tiles
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
            PresentationMutation::ResizeTileCrop(ResizeTileCrop { id: tile.id.clone(), new_crop: clamp_tile_crop(&crop) })
        })
        .collect();
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}
