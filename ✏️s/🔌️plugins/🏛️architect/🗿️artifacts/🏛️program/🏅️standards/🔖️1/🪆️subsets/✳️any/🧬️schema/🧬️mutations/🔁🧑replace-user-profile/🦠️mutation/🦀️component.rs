//! 🦠️ ProgramSnapshot mutation — `replace-user-profile` leaf (replace). Split from the
//! pre-migration `🧑users` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::UserProfile;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one user profile row's non-identity content, addressed by
/// `user_profile.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceUserProfile {
    pub user_profile: UserProfile,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceUserProfile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "user-profile", kind: "replace-user-profile", record: "ReplacedUserProfile" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace user profile \"{}\"", self.user_profile.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.user_profile.header.id.0.clone()]
    }
}
