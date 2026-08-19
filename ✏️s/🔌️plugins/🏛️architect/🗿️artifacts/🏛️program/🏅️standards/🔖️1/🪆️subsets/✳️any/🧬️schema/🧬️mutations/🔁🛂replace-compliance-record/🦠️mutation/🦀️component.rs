//! 🦠️ ProgramSnapshot mutation — `replace-compliance-record` leaf (replace). Split from the
//! pre-migration `🛂compliance-records` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ComplianceRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one compliance record row's non-identity content, addressed by
/// `compliance_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceComplianceRecord {
    pub compliance_record: ComplianceRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceComplianceRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "compliance-record", kind: "replace-compliance-record", record: "ReplacedComplianceRecord" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace compliance record \"{}\"", self.compliance_record.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.compliance_record.header.id.0.clone()]
    }
}
