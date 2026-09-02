//! 🦠️ ProgramSnapshot mutation — `create-equipment` leaf (create). Split from the
//! pre-migration `🛠️equipment` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Equipment;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new equipment row into existence in `program.equipment`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateEquipment {
    pub equipment: Equipment,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateEquipment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "equipment", kind: "create-equipment", record: "CreatedEquipment" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create equipment \"{}\"", self.equipment.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.equipment.header.id.0.clone()]
    }
}
