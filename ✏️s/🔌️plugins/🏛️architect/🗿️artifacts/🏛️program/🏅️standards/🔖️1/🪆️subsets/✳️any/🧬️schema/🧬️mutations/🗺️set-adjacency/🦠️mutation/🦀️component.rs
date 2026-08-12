//! 🦠️ ProgramSnapshot mutation — `set_adjacency` leaf: `ConnectAdjacency`. `Adjacency` is a
//! relationship/edge row (`element_a_id`/`element_b_id` + header) per
//! `📓️derivation-rules.md` rule 4 (relationship/edge collection): `connect`/`disconnect`, not
//! create/delete/rename/replace. Supersedes the old `SetAdjacency`/`ClearAdjacency` naming — "set"
//! is only approved for a narrow addressed single-field setter and "clear" means emptying a whole
//! collection, neither of which matches an edge upsert-or-create. Directory keeps its
//! pre-migration name (`📦️glue.rs` — outside this facet's boundary — `#[path]`-wires it by this
//! exact name); only the Rust type/variant/kind names change, tracked as a `sharedFileRequests`
//! entry for the eventual directory rename.

use crate::artifacts::program::registers::Adjacency;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️ConnectAdjacency
/// 🔌️ Upserts an adjacency edge between two elements: normalizes the endpoint pair, replaces the
/// existing edge for that pair if present (keeping its id), otherwise adds a new edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectAdjacency {
    pub adjacency: Adjacency,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ConnectAdjacency {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "adjacency", kind: "connect-adjacency", record: "ConnectedAdjacency" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_connect(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_connect(self, base)
    }
    fn label(&self) -> String {
        format!("Connect adjacency between \"{}\" and \"{}\"", self.adjacency.element_a_id.0, self.adjacency.element_b_id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.adjacency.header.id.0.clone()]
    }
}
//#endregion 🔖️ConnectAdjacency
