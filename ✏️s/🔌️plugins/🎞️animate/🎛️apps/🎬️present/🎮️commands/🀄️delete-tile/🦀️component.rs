//! 🀄️ 🀄️ Animate present app commands command — `delete-tile`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::{valid_tile_ids, PresentDispatchCtx};
use crate::artifacts::present::mutations::delete_tile::mutation::DeleteTile as DeleteTileMutation;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-tile")]
pub struct DeleteTile {
    pub id: String,
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM);
/// `tiles` is declared `HierarchyProvider::Flat` and Flat domains are deliberately never auto-pruned
/// on document change, so a deleted tile's stale id simply stays selected until the next real pick —
/// a documented, accepted gap, not routed around here (matches `🖍️draw`'s `delete-layer`).
pub fn handle(payload: &DeleteTile, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let targets = valid_tile_ids(deck, vec![payload.id.clone()]);
    if targets.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(targets.into_iter().map(|id| PresentMutation::DeleteTile(DeleteTileMutation { id })).collect()))
}
