//! 🦠️ ProgramSnapshot mutation — `create-approval-record` leaf (create). Split from the
//! pre-migration `👍approvals` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ApprovalRecord;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new approval record row into existence in `program.approvals`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalRecord {
    pub approval_record: ApprovalRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateApprovalRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "approval-record", kind: "create-approval-record", record: "CreatedApprovalRecord" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create approval record \"{}\"", self.approval_record.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.approval_record.header.id.0.clone()]
    }
}
