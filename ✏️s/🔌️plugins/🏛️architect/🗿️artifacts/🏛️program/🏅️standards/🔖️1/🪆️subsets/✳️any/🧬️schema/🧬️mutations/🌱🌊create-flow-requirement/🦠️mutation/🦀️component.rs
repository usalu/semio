//! 🦠️ ProgramSnapshot mutation — `create-flow-requirement` leaf (create). Split from the
//! pre-migration `🌊flows` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::FlowRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new flow requirement row into existence in `program.flows`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFlowRequirement {
    pub flow_requirement: FlowRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateFlowRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "flow-requirement", kind: "create-flow-requirement", record: "CreatedFlowRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create flow requirement \"{}\"", self.flow_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.flow_requirement.header.id.0.clone()]
    }
}
