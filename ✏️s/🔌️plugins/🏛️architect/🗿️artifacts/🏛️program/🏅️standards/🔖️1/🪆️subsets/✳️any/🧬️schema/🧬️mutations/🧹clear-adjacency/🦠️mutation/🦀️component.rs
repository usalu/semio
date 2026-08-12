//! 🦠️ ProgramSnapshot mutation — `clear_adjacency` leaf: `DisconnectAdjacency`. Removes one
//! adjacency edge by its own id (the pre-migration `ClearAdjacency` also cascaded to any edge
//! touching a given *element* id — that cascade belongs to a future `delete-element` mutation's
//! own severed-link capture per `📓️taxonomy.md`'s `delete` row, not to a plain edge disconnect;
//! not replicated here, noted as a `sharedFileRequests` gap). Directory keeps its pre-migration
//! name (`📦️glue.rs` `#[path]`-wires it); see `🗺️set-adjacency`'s doc comment for the naming note.

use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️DisconnectAdjacency
/// ✂️ Removes one adjacency edge by its own id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectAdjacency {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DisconnectAdjacency {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "adjacency", kind: "disconnect-adjacency", record: "DisconnectedAdjacency" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_disconnect(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_disconnect(self, base)
    }
    fn label(&self) -> String {
        format!("Disconnect adjacency \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DisconnectAdjacency
