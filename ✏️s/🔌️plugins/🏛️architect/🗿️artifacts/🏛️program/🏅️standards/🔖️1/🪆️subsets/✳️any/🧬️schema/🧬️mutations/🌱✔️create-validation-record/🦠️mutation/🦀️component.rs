//! 🦠️ ProgramSnapshot mutation — `create-validation-record` leaf (create). Split from the
//! pre-migration `✔️validations` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::ValidationRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new validation record row into existence in `program.validations`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateValidationRecord {
    pub validation_record: ValidationRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateValidationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "validation-record", kind: "create-validation-record", record: "CreatedValidationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create validation record \"{}\"", self.validation_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.validation_record.header.id.0.clone()]
    }
}
