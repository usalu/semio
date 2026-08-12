//! 🦠️ ProgramSnapshot mutation — `create-compliance-record` leaf (create). Split from the
//! pre-migration `🛂compliance-records` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::ComplianceRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new compliance record row into existence in `program.compliance_records`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComplianceRecord {
    pub compliance_record: ComplianceRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateComplianceRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "compliance-record", kind: "create-compliance-record", record: "CreatedComplianceRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create compliance record \"{}\"", self.compliance_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.compliance_record.header.id.0.clone()]
    }
}
