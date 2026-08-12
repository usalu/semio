//! 🦠️ ProgramSnapshot mutation — `replace-equipment` leaf (replace). Split from the
//! pre-migration `🛠️equipment` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::Equipment;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one equipment row's non-identity content, addressed by
/// `equipment.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceEquipment {
    pub equipment: Equipment,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceEquipment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "equipment", kind: "replace-equipment", record: "ReplacedEquipment" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace equipment \"{}\"", self.equipment.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.equipment.header.id.0.clone()]
    }
}
