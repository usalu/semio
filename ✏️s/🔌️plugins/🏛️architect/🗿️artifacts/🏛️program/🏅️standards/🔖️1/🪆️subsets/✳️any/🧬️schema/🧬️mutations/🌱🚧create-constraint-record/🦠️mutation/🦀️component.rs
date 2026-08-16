//! 🦠️ ProgramSnapshot mutation — `create-constraint-record` leaf (create). Split from the
//! pre-migration `🚧constraints` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ConstraintRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new constraint record row into existence in `program.constraints`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConstraintRecord {
    pub constraint_record: ConstraintRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateConstraintRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "constraint-record", kind: "create-constraint-record", record: "CreatedConstraintRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create constraint record \"{}\"", self.constraint_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.constraint_record.header.id.0.clone()]
    }
}
