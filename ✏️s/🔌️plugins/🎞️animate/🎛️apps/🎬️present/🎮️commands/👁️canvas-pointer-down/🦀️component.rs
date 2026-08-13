//! 👁️ 👁️ Animate present app commands command — `canvas-pointer-down`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::valid_tile_ids;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub layer_id: Option<String>,
}

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let (_, deck_tiles) = crate::artifacts::present::present_working_scene(deck);
    match &payload.layer_id {
        Some(id) if deck_tiles.iter().any(|tile| &tile.id == id) => Ok(Emit::config(vec![PresentConfigMutation::SetSelectedIds { ids: vec![id.clone()] }])),
        _ => Ok(Emit::config(vec![PresentConfigMutation::SetSelectedIds { ids: Vec::new() }])),
    }
}
