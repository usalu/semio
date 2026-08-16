//! 🦠️ ProgramSnapshot mutation — `replace-site-context` leaf (replace). Split from the
//! pre-migration `📍site-context` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::SiteContext;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one site context row's non-identity content, addressed by
/// `site_context.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSiteContext {
    pub site_context: SiteContext,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSiteContext {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "site-context", kind: "replace-site-context", record: "ReplacedSiteContext" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace site context \"{}\"", self.site_context.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.site_context.header.id.0.clone()]
    }
}
