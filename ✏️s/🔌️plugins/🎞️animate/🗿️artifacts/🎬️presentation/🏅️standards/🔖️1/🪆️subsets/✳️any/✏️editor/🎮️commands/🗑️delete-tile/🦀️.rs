//! 🀄️ 🀄️ Animate presentation app commands command — `delete-tile`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::mutations::delete_tile::mutation::DeleteTile as DeleteTileMutation;
use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{valid_tile_ids, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-tile")]
pub struct DeleteTile {
    pub id: String,
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM);
/// `tiles` is declared `HierarchyProvider::Flat` and Flat domains are deliberately never auto-pruned
/// on document change, so a deleted tile's stale id simply stays selected until the next real pick —
/// a documented, accepted gap, not routed around here (matches `🖍️draw`'s `delete-layer`).
pub fn handle(payload: &DeleteTile, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let targets = valid_tile_ids(deck, vec![payload.id.clone()]);
    if targets.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(targets.into_iter().map(|id| PresentationMutation::DeleteTile(DeleteTileMutation { id })).collect()))
}
