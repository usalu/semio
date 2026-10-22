//! 🦠️ ProgramSnapshot mutation — `replace-governance` leaf (replace). Split from the
//! pre-migration `🏛️update-governance` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::standards::v1::subsets::any::schema::registers::Governance;
use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, snapshot::ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔁️ Whole-value swap of `program.governance`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceGovernance {
    pub new_governance: Governance,
}
impl<Op> MutationKind<ProgramSnapshot, Op> for ReplaceGovernance
where
    Op: protocol::Mutation<ProgramSnapshot, Diff = ProgramDiff> + From<ReplaceGovernance>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "governance", kind: "replace-governance", record: "ReplacedGovernance" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace governance \"{}\"", self.new_governance.framework), &format!("Governance \"{}\" ersetzen", self.new_governance.framework))
    }
}

impl<Op> protocol::ColdMutationKind<ProgramSnapshot, Op> for ReplaceGovernance
where
    Op: protocol::ColdMutation<ProgramSnapshot, ReferenceDiff = ProgramDiff> + From<ReplaceGovernance>,
{
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
