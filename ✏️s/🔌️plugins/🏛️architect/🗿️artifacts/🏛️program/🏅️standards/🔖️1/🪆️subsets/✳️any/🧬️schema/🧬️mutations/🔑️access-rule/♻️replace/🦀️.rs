//! 🦠️ ProgramSnapshot mutation — `replace-access-rule` leaf (replace). Split from the
//! pre-migration `🔑access-rules` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::registers::AccessRule;
use crate::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔁️ Whole-value swap of one access rule row's non-identity content, addressed by
/// `access_rule.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceAccessRule {
    pub access_rule: AccessRule,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAccessRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "access-rule", kind: "replace-access-rule", record: "ReplacedAccessRule" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace access rule \"{}\"", self.access_rule.header.name), &format!("Zugangsregel \"{}\" ersetzen", self.access_rule.header.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.access_rule.header.id.0.clone()]
    }
}
