//! 🦠️ ProgramSnapshot mutation — `create-security-requirement` leaf (create). Split from the
//! pre-migration `🛡️security` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::SecurityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new security requirement row into existence in `program.security`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSecurityRequirement {
    pub security_requirement: SecurityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSecurityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "security-requirement", kind: "create-security-requirement", record: "CreatedSecurityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create security requirement \"{}\"", self.security_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.security_requirement.header.id.0.clone()]
    }
}
