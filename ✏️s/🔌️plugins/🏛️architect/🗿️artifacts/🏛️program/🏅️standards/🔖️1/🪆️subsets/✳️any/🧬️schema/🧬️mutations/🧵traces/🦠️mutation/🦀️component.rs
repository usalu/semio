//! 🦠️ ProgramSnapshot mutation — `traces` leaf: `ConnectTrace`/`DisconnectTrace`. `TraceLink`
//! (`kernel/🦀️component.rs`) is a directed audit edge (`from_id`/`to_id`/`kind`, no
//! `EntityHeader`/name) — a relationship/edge row per `📓️derivation-rules.md` rule 4, so it gets
//! `connect`/`disconnect` rather than the header-shaped registers' create/delete/rename/replace
//! quad. Supersedes the generic `Traces(CollectionMutation<EntityId, TraceLink, TraceLinkPatch>)`.

use crate::artifacts::program::kernel::{EntityId, TraceLink};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️ConnectTrace
/// 🔌️ Upserts a trace edge by its own id: adds it if new, replaces its full content if present.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectTrace {
    pub trace: TraceLink,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ConnectTrace {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "trace", kind: "connect-trace", record: "ConnectedTrace" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_connect(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_connect(self, base)
    }
    fn label(&self) -> String {
        format!("Connect trace \"{}\" -> \"{}\"", self.trace.from_id.0, self.trace.to_id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.trace.id.0.clone()]
    }
}
//#endregion 🔖️ConnectTrace

//#region 🔖️DisconnectTrace
/// ✂️ Removes one trace edge by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectTrace {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DisconnectTrace {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "trace", kind: "disconnect-trace", record: "DisconnectedTrace" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_disconnect(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_disconnect(self, base)
    }
    fn label(&self) -> String {
        format!("Disconnect trace \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DisconnectTrace
