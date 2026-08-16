//! 🦠️ ProgramSnapshot mutation — `replace-meta` leaf (replace). Split from the
//! pre-migration `🏷️update-meta` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ProgramMeta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of `program.meta`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceMeta {
    pub new_meta: ProgramMeta,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceMeta {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "meta", kind: "replace-meta", record: "ReplacedMeta" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace document metadata \"{}\"", self.new_meta.title)
    }
}
