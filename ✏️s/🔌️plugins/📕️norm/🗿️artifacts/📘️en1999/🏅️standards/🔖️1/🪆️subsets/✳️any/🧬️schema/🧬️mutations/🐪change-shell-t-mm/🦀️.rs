//! 🐪 `change-shell-t-mm` payload — changes the En1999 document's `shell_t_mm` (shell thickness t [mm]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_shell_t_mm::ChangeShellTMm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeShellTMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeShellTMm {
    pub new_shell_t_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeShellTMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "shell-t-mm", kind: "change-shell-t-mm", record: "ChangedShellTMm" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change shell thickness t [mm] to {}", self.new_shell_t_mm)
    }
}
//#endregion 🔖️ChangeShellTMm
