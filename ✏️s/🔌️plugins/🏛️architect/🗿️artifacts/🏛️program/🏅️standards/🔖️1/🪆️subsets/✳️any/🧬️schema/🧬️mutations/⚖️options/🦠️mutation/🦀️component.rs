//! 🦠️ ProgramSnapshot mutation — `options` leaf: create/delete/rename/replace option evaluation rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `OptionEvaluation` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::OptionEvaluation;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateOptionEvaluation
/// 🌱️ Brings a new option evaluation row into existence in `program.options`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOptionEvaluation {
    pub option_evaluation: OptionEvaluation,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateOptionEvaluation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "option-evaluation", kind: "create-option-evaluation", record: "CreatedOptionEvaluation" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create option evaluation \"{}\"", self.option_evaluation.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.option_evaluation.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateOptionEvaluation

//#region 🔖️DeleteOptionEvaluation
/// 🗑️ Removes a option evaluation row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOptionEvaluation {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteOptionEvaluation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "option-evaluation", kind: "delete-option-evaluation", record: "DeletedOptionEvaluation" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete option evaluation \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteOptionEvaluation

//#region 🔖️RenameOptionEvaluation
/// ✏️ Sets the identity `name` field of one option evaluation row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameOptionEvaluation {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameOptionEvaluation {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "option-evaluation", kind: "rename-option-evaluation", record: "RenamedOptionEvaluation" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename option evaluation to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameOptionEvaluation

//#region 🔖️ReplaceOptionEvaluation
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
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace option evaluation \"{}\"", self.option_evaluation.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.option_evaluation.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceOptionEvaluation
