//! 🦠️ ProgramSnapshot mutation — `create-user-profile` leaf (create). Split from the
//! pre-migration `🧑users` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::UserProfile;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new user profile row into existence in `program.users`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserProfile {
    pub user_profile: UserProfile,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateUserProfile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "user-profile", kind: "create-user-profile", record: "CreatedUserProfile" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create user profile \"{}\"", self.user_profile.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.user_profile.header.id.0.clone()]
    }
}
