//! 🦠️ ProgramSnapshot mutation — `replace-validation-record` leaf (replace). Split from the
//! pre-migration `✔️validations` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::ValidationRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one validation record row's non-identity content, addressed by
/// `validation_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceValidationRecord {
    pub validation_record: ValidationRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceValidationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "validation-record", kind: "replace-validation-record", record: "ReplacedValidationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace validation record \"{}\"", self.validation_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.validation_record.header.id.0.clone()]
    }
}
