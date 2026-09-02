//! 🐫 `change-shell-r-mm` payload — changes the En1999 document's `shell_r_mm` (shell radius r [mm]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_shell_r_mm::ChangeShellRMm;

//#region 🔖️ChangeShellRMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeShellRMm {
    pub new_shell_r_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeShellRMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "shell-r-mm", kind: "change-shell-r-mm", record: "ChangedShellRMm" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change shell radius r [mm] to {}", self.new_shell_r_mm)
    }
}
//#endregion 🔖️ChangeShellRMm
