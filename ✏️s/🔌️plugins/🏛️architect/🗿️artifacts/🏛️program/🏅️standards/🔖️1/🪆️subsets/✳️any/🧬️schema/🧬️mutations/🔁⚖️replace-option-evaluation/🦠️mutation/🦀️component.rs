//! 🦠️ ProgramSnapshot mutation — `replace-option-evaluation` leaf (replace). Split from the
//! pre-migration `⚖️options` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::OptionEvaluation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one option evaluation row's non-identity content, addressed by
/// `option_evaluation.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOptionEvaluation {
    pub option_evaluation: OptionEvaluation,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceOptionEvaluation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "option-evaluation", kind: "replace-option-evaluation", record: "ReplacedOptionEvaluation" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace option evaluation \"{}\"", self.option_evaluation.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.option_evaluation.header.id.0.clone()]
    }
}
