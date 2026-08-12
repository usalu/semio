//! 🦠️ ProgramSnapshot mutation — `update_governance` leaf: `RenameGovernance`/`ReplaceGovernance`.
//! `Governance` is a document-level scalar facet (`program.governance`) per
//! `📓️derivation-rules.md` rule 1 — same shape/rationale as `🏷️update-meta`, identity-like field
//! is `framework`. Supersedes the banned raw-Patch-payload `UpdateGovernance { patch: GovernancePatch }`.

use crate::artifacts::program::registers::Governance;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️RenameGovernance
/// ✏️ Sets `program.governance.framework`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameGovernance {
    pub new_framework: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameGovernance {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "governance", kind: "rename-governance", record: "RenamedGovernance" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename governance framework to \"{}\"", self.new_framework)
    }
}
//#endregion 🔖️RenameGovernance

//#region 🔖️ReplaceGovernance
/// 🔁️ Whole-value swap of `program.governance`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceGovernance {
    pub new_governance: Governance,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceGovernance {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "governance", kind: "replace-governance", record: "ReplacedGovernance" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace governance \"{}\"", self.new_governance.framework)
    }
}
//#endregion 🔖️ReplaceGovernance
